#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700086ca54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086ca54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086ca58); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086ca5e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086ca65); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700086ca6c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086ca70); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086ca74); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086ca78); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ca85); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086ca8c); };
            c.cmp(t7, 0x83du64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ca99); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086caa0); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086caa7); };
            c.w32(12, t12);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x97u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x700086caba);
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0x263)) else { return RegfileExit::fault(0x700086cabb); };
            c.w32(12, t13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086cac2); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x700086cacc); };
            c.w64(9, t15);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086cad6); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086cae0); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086cae7); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086caee); };
            c.w64(12, t20);
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086caf1); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086cafd); };
            c.w64(2, t23);
            let Some(t24) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086cb04); };
            c.cmp(t24, c.r64(15), 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700086cb0d); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cb15); };
            c.w32(12, t25);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x700086cb25);
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700086cb26); };
            c.w32(12, t26);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086cb2d); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086cb37); }
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cb3b); };
            c.w32(12, t27);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086cb4b);
        }
        12 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086cb4c); };
            c.w32(12, t28);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086cb50); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086cb54); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086cb58); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086cb5c); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086cb63); };
            c.w64(15, t30);
            c.w64(2, c.r64(15).wrapping_add(0x1c));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086cb6b); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086cb72); };
            c.cmp(t32, c.r64(2), 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x700086cb7b); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cb83); };
            c.w32(15, t33);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xbeu64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x700086cb96);
        }
        16 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086cb97); };
            c.w32(15, t34);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086cb9e); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086cba3); };
            c.w64(15, t35);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086cbaa); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086cbaf); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086cbb4); };
            c.w64(15, t36);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x700086cbbb); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1060)) else { return RegfileExit::fault(0x700086cbc0); };
            c.w64(15, t37);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700086cbc7); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086cbcc); }
            let Some(t38) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cbd1); };
            c.w32(15, t38);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t39) = c.ld64(c.r64(2).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x700086cbdf); };
            c.w64(15, t39);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x700086cbe3); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086cbe8); };
            c.w64(15, t40);
            let Some(t41) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086cbef); };
            c.w64(2, t41);
            c.w64(1, c.r64(2).wrapping_add(0x1c));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086cbf6); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086cbfd); };
            c.cmp(t43, c.r64(1), 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x700086cc06); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 18; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cc0d); };
            c.w32(2, t44);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x700086cc20);
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086cc21); };
            c.w32(2, t45);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700086cc28); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086cc2c); };
            c.w64(2, t46);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700086cc33); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700086cc37); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086cc3b); };
            c.w64(2, t47);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700086cc42); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1058)) else { return RegfileExit::fault(0x700086cc46); };
            c.w64(2, t48);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086cc4d); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086cc51); }
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cc55); };
            c.w32(11, t49);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t50) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x700086cc63); };
            c.w64(11, t50);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x700086cc67); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086cc6b); };
            c.w64(11, t51);
            let Some(t52) = c.ld32(c.r64(11)) else { return RegfileExit::fault(0x700086cc72); };
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
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086cc89); };
            c.w64(8, t53);
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(1, c.r64(15));
            c.w64(2, c.r64(8));
            c.w64(6, c.r64(9));
            return c.call(0x70008b2ef4, 0x700086cca4, 0x700086cc9f);
        }
        24 => {
            blk = 59; continue;
        }
        25 => {
            let Some(t54) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086cca9); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086ccb0); };
            c.w32(9, t56);
            let r57 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ccb7); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086ccbe); };
            c.w64(12, t59);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ccc6); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086cccd); };
            c.cmp(t61, c.r64(15), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700086ccd6); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 27; continue;
        }
        27 => {
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086ccde); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x700086ccee);
        }
        29 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700086ccef); };
            c.w32(12, t63);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086ccf6); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086cd00); }
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cd04); };
            c.w32(12, t64);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x700086cd14);
        }
        31 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086cd15); };
            c.w32(12, t65);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086cd19); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086cd1d); }
            c.w64(12, c.r64(0));
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086cd24); }
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cd28); };
            c.w32(15, t66);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x700086cd3b);
        }
        33 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086cd3c); };
            c.w32(15, t67);
            let r68 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086cd46); };
            c.w64(0, t69);
            let Some(t70) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086cd4d); };
            c.w64(2, t70);
            let Some(t71) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086cd50); };
            c.w32(15, t71);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086cd58); };
            c.w64(7, t72);
            let Some(t73) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086cd5f); };
            c.cmp(t73, c.r64(1), 64);
            if c.cc_be() { blk = 101; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700086cd68); }
            let r74 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r74);
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086cd6f); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086cd73); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086cd7a); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086cd7e); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700086cd85); }
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x700086cd89); }
            let Some(t76) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086cd8d); };
            c.w64(15, t76);
            c.w64(1, c.r64(15).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x700086cd94); }
            let Some(t77) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086cd98); };
            c.cmp(t77, c.r64(1), 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700086cda1); }
            let r78 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r78);
            blk = 37; continue;
        }
        37 => {
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cda8); };
            c.w32(0, t79);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086cdb2); }
            c.cmp(c.r64(0), 0xbeu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700086cdbe);
        }
        39 => {
            let Some(t80) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086cdbf); };
            c.w32(0, t80);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700086cdc6); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086cdca); };
            c.w64(0, t81);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700086cdd1); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700086cdd5); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086cdd9); };
            c.w64(0, t82);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x700086cde0); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1060)) else { return RegfileExit::fault(0x700086cde4); };
            c.w64(0, t83);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700086cdeb); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086cdef); }
            let Some(t84) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cdf3); };
            c.w32(0, t84);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(13).wrapping_add(0x63f0));
            let Some(t85) = c.ld64(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(4))) else { return RegfileExit::fault(0x700086ce00); };
            c.w64(0, t85);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700086ce04); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ce08); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086ce0f); };
            c.w64(1, t87);
            c.w64(7, c.r64(1).wrapping_add(0x1c));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ce16); };
            c.w64(12, t88);
            let Some(t89) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086ce1d); };
            c.cmp(t89, c.r64(7), 64);
            if c.cc_be() { blk = 105; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x700086ce27); }
            c.w64(12, c.r64(1).wrapping_add(0x1));
            blk = 41; continue;
        }
        41 => {
            let Some(t90) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086ce2e); };
            c.w32(0, t90);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700086ce38); }
            c.cmp(c.r64(0), 0xbeu64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::fault(0x700086ce44);
        }
        43 => {
            let Some(t91) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086ce45); };
            c.w32(9, t91);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700086ce4c); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086ce51); };
            c.w64(9, t92);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700086ce58); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700086ce5d); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086ce62); };
            c.w64(9, t93);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x700086ce69); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x1058)) else { return RegfileExit::fault(0x700086ce6e); };
            c.w64(9, t94);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086ce75); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086ce7a); }
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086ce7f); };
            c.w32(9, t95);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(13).wrapping_add(0x63f0));
            let Some(t96) = c.ld64(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700086ce8d); };
            c.w64(9, t96);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x700086ce91); }
            let Some(t97) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ce96); };
            c.w32(9, t97);
            let r98 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r98);
            let Some(t99) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086ce9d); };
            c.w32(9, t99);
            let r100 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r100);
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cea4); };
            c.w32(11, t101);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x700086ceb7);
        }
        45 => {
            let Some(t102) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086ceb8); };
            c.w32(7, t102);
            let r103 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r103);
            let Some(t104) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086cec2); };
            c.w32(9, t104);
            let r105 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r105);
            let Some(t106) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086cec9); };
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
            let Some(t107) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ced6); };
            c.w32(9, t107);
            let r108 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r108);
            let Some(t109) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086cedd); };
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
            return RegfileExit::fault(0x700086cefc);
        }
        51 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086cefd); };
            c.w64(9, t110);
            blk = 52; continue;
        }
        52 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086cf04); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086cf06); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086cf07); };
            c.w64(9, t111);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086cf0e); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x700086cf1a, 0x700086cf15);
        }
        53 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086cf1a); };
            c.w64(2, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086cf1e); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086cf22); };
            c.w64(12, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086cf26); };
            c.w64(6, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086cf2a); };
            c.w64(3, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086cf2e); };
            c.w64(8, t117);
            blk = 54; continue;
        }
        54 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086cf32); };
            c.w64(9, t118);
            let Some(t119) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700086cf39); };
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
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086cf50); };
            c.w64(8, t120);
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086cf57); }
            c.w64(0, c.r64(2));
            return c.call(0x70008ba734, 0x700086cf63, 0x700086cf5e);
        }
        58 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086cf63); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086cf67); };
            c.w64(3, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086cf6b); };
            c.w64(1, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086cf6f); };
            c.w64(2, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086cf73); };
            c.w64(6, t125);
            return c.call(0x70008b2ef4, 0x700086cf7c, 0x700086cf77);
        }
        59 => {
            c.w32(8, 0x2u64);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086cf82); };
            c.w64(9, t126);
            if !c.st32(c.r64(9).wrapping_add(0x2b), c.r32(8)) { return RegfileExit::fault(0x700086cf86); }
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086cf8a); };
            c.w64(0, t127);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086cf94); }
            return c.ret(0x0, 0x700086cf95);
        }
        60 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700086cf96); }
            c.w64(0, c.r64(3));
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x700086cf9d); };
            c.w64(3, t128);
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x700086cfac, 0x700086cfa7);
        }
        61 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086cfac); };
            c.w64(11, t129);
            let Some(t130) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086cfb0); };
            c.w32(8, t130);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x700086cfc3);
        }
        63 => {
            let Some(t131) = c.ld32(c.r64(11).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086cfc4); };
            c.cmp(t131, c.r32(0), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086cfcd); };
            c.w64(0, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086cfd1); };
            c.w64(6, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086cfd5); };
            c.w64(3, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086cfd9); };
            c.w64(8, t135);
            blk = 25; continue;
        }
        65 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086cfe2); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086cfe6); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086cfea); };
            c.w64(3, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086cfee); };
            c.w64(8, t139);
            blk = 6; continue;
        }
        66 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086cffc); }
            return c.call(0x70007febf4, 0x700086d005, 0x700086d000);
        }
        67 => {
            c.w64(11, c.r64(0));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086d008); };
            c.w64(0, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d00c); };
            c.w64(6, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d010); };
            c.w64(3, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d014); };
            c.w64(8, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d018); };
            c.w64(9, t144);
            blk = 8; continue;
        }
        68 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086d026); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086d02a); }
            return c.call(0x70007febf4, 0x700086d033, 0x700086d02e);
        }
        69 => {
            c.w64(12, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086d036); };
            c.w64(0, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d03a); };
            c.w64(6, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d03e); };
            c.w64(3, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d042); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d046); };
            c.w64(9, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d04a); };
            c.w64(11, t150);
            blk = 14; continue;
        }
        70 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086d058); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086d05c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086d060); }
            return c.call(0x70007febf4, 0x700086d069, 0x700086d064);
        }
        71 => {
            c.w64(15, c.r64(0));
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086d06c); };
            c.w64(0, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d070); };
            c.w64(6, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d074); };
            c.w64(3, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d078); };
            c.w64(8, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d07c); };
            c.w64(12, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d080); };
            c.w64(9, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d084); };
            c.w64(11, t157);
            blk = 18; continue;
        }
        72 => {
            let Some(t158) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d08d); };
            c.w32(8, t158);
            let r159 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r159);
            let Some(t160) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086d094); };
            c.w32(8, t160);
            let r161 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r161);
            let Some(t162) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d09b); };
            c.w32(11, t162);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086d0a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086d0a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086d0ad); }
            c.cmp(c.r64(11), 0xeau64, 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            return RegfileExit::fault(0x700086d0ba);
        }
        74 => {
            let Some(t163) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086d0bb); };
            c.w32(8, t163);
            let r164 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r164);
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086d0c5); };
            c.w64(11, t165);
            let Some(t166) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086d0cc); };
            c.w64(2, t166);
            let Some(t167) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086d0cf); };
            c.w32(8, t167);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086d0d7); }
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086d0db); };
            c.w64(7, t168);
            let Some(t169) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086d0e2); };
            c.cmp(t169, c.r64(1), 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700086d0f1, 0x700086d0ec);
        }
        76 => {
            blk = 78; continue;
        }
        77 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700086d0f3); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 78; continue;
        }
        78 => {
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d0fd); };
            c.w64(8, t170);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086d101); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086d105); };
            c.w64(8, t171);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086d10c); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086d110); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086d117); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086d11b); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086d11f); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086d123); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086d127); };
            c.w64(8, t172);
            let Some(t173) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086d12e); };
            c.w32(8, t173);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700086d131); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086d135); }
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
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d149); };
            c.w64(6, t174);
            blk = 93; continue;
        }
        81 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d152); };
            c.w64(6, t175);
            let Some(t176) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d156); };
            c.w32(9, t176);
            let r177 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r177);
            let Some(t178) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086d15d); };
            c.w32(9, t178);
            let r179 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r179);
            let Some(t180) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d164); };
            c.w32(11, t180);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            return RegfileExit::fault(0x700086d177);
        }
        83 => {
            let Some(t181) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086d178); };
            c.w32(7, t181);
            let r182 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r182);
            let Some(t183) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d182); };
            c.w32(9, t183);
            let r184 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r184);
            let Some(t185) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086d189); };
            c.logic(t185 & 0x2u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d190); };
            c.w64(3, t186);
            let Some(t187) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d194); };
            c.w32(9, t187);
            let r188 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r188);
            let Some(t189) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086d19b); };
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
            return RegfileExit::fault(0x700086d1ba);
        }
        88 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086d1bb); };
            c.w64(9, t190);
            blk = 89; continue;
        }
        89 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086d1c2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d1c4); }
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086d1c5); };
            c.w64(9, t191);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086d1cc); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700086d1db, 0x700086d1d6);
        }
        90 => {
            blk = 92; continue;
        }
        91 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d1dd); }
            c.w32(0, 0x1u64);
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086d1e3); };
            c.w64(3, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d1ea); };
            c.w64(6, t193);
            return c.call(0x700086ed74, 0x700086d1f3, 0x700086d1ee);
        }
        92 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d1f3); };
            c.w64(6, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d1f7); };
            c.w64(0, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d1fb); };
            c.w64(8, t196);
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d205); }
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d206); };
            if !c.push64(t197) { return RegfileExit::fault(0x700086d206); }
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086d209); };
            c.w64(3, t198);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086d21d, 0x700086d218);
        }
        95 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d21d); };
            c.w64(0, t199);
            blk = 96; continue;
        }
        96 => {
            c.w64(8, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d224); };
            c.w64(3, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d228); };
            c.w64(12, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086d22c); };
            c.w64(15, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d230); };
            c.w64(9, t203);
            blk = 23; continue;
        }
        97 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086d239); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086d23a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086d23b); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700086d23d); }
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700086d23f); };
            c.w64(8, t204);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086d246); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086d248); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086d24c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086d250); }
            c.w32(0, 0x5u64);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x25a0)) else { return RegfileExit::fault(0x700086d259); };
            c.w64(3, t205);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700086d26b, 0x700086d266);
        }
        98 => {
            c.w64(8, c.r64(0));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d26e); };
            c.w64(3, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d272); };
            c.w64(12, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086d276); };
            c.w64(15, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d27a); };
            c.w64(9, t209);
            blk = 23; continue;
        }
        99 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086d288); }
            return c.call(0x70007febf4, 0x700086d291, 0x700086d28c);
        }
        100 => {
            c.w64(11, c.r64(0));
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086d294); };
            c.w64(0, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d298); };
            c.w64(6, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d29c); };
            c.w64(3, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d2a0); };
            c.w64(8, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086d2a4); };
            c.w64(9, t214);
            blk = 27; continue;
        }
        101 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086d2b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086d2b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086d2ba); }
            return c.call(0x70007febf4, 0x700086d2c3, 0x700086d2be);
        }
        102 => {
            c.w64(2, c.r64(0));
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086d2c6); };
            c.w64(12, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d2ca); };
            c.w64(6, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d2ce); };
            c.w64(3, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d2d2); };
            c.w64(8, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d2d6); };
            c.w64(11, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086d2da); };
            c.w64(9, t220);
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086d2de); };
            c.w64(0, t221);
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086d2e5); };
            c.w64(7, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086d2ec); };
            c.w64(15, t223);
            blk = 35; continue;
        }
        103 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086d2fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086d2fe); }
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x700086d30a, 0x700086d305);
        }
        104 => {
            c.w64(15, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086d30d); };
            c.w64(12, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d311); };
            c.w64(2, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d315); };
            c.w64(6, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d319); };
            c.w64(3, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d31d); };
            c.w64(8, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d321); };
            c.w64(11, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086d325); };
            c.w64(9, t230);
            blk = 37; continue;
        }
        105 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086d333); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086d337); }
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x700086d343, 0x700086d33e);
        }
        106 => {
            c.w64(12, c.r64(0));
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d346); };
            c.w64(2, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086d34a); };
            c.w64(15, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d34e); };
            c.w64(6, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d352); };
            c.w64(3, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d356); };
            c.w64(8, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d35a); };
            c.w64(11, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086d35e); };
            c.w64(9, t237);
            blk = 41; continue;
        }
        107 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d367); }
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086d368); };
            c.w64(3, t238);
            c.w32(0, 0x1u64);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d374); };
            c.w64(6, t239);
            return c.call(0x700086ed74, 0x700086d37d, 0x700086d378);
        }
        108 => {
            blk = 53; continue;
        }
        109 => {
            let Some(t240) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d382); };
            c.w32(8, t240);
            let r241 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r241);
            let Some(t242) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086d389); };
            c.w32(8, t242);
            let r243 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r243);
            let Some(t244) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d390); };
            c.w32(9, t244);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return RegfileExit::fault(0x700086d3a3);
        }
        111 => {
            let Some(t245) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086d3a4); };
            c.w32(8, t245);
            let r246 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r246);
            let Some(t247) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086d3ae); };
            c.w32(8, t247);
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086d3b2); };
            c.w64(9, t248);
            let Some(t249) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086d3b9); };
            c.w64(11, t249);
            c.w64(0, c.r64(11).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086d3c0); }
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086d3c4); };
            c.w64(1, t250);
            let Some(t251) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086d3cb); };
            c.cmp(t251, c.r64(0), 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(2, 0x14u64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700086d3dd, 0x700086d3d8);
        }
        113 => {
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d3dd); };
            c.w64(8, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d3e1); };
            c.w64(2, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086d3e5); };
            c.w64(15, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d3e9); };
            c.w64(12, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d3ed); };
            c.w64(6, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d3f1); };
            c.w64(3, t257);
            blk = 115; continue;
        }
        114 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x700086d3f7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 115; continue;
        }
        115 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086d401); }
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086d405); };
            c.w64(8, t258);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086d40c); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086d410); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086d417); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086d41b); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086d41f); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086d423); }
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086d427); };
            c.w64(8, t259);
            let Some(t260) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086d42e); };
            c.w32(8, t260);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700086d431); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086d435); }
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
            let Some(t261) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d44d); };
            c.w32(9, t261);
            let r262 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r262);
            let Some(t263) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086d454); };
            c.w32(9, t263);
            let r264 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r264);
            let Some(t265) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d45b); };
            c.w32(11, t265);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            return RegfileExit::fault(0x700086d46e);
        }
        119 => {
            let Some(t266) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086d46f); };
            c.w32(7, t266);
            let r267 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r267);
            let Some(t268) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d479); };
            c.w32(9, t268);
            let r269 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r269);
            let Some(t270) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086d480); };
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
            let Some(t271) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d493); };
            c.w32(11, t271);
            let r272 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r272);
            let Some(t273) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086d49a); };
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
            return RegfileExit::fault(0x700086d4b9);
        }
        125 => {
            blk = 127; continue;
        }
        126 => {
            c.w64(9, c.r64(2));
            blk = 127; continue;
        }
        127 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086d4bf); };
            c.w64(11, t274);
            blk = 128; continue;
        }
        128 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086d4c6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d4c8); }
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086d4c9); };
            c.w64(11, t275);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086d4d0); }
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x70007f2134, 0x700086d4df, 0x700086d4da);
        }
        129 => {
            blk = 131; continue;
        }
        130 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d4e1); }
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086d4e2); };
            c.w64(3, t276);
            c.w32(0, 0x1u64);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d4ee); };
            c.w64(6, t277);
            return c.call(0x700086ed74, 0x700086d4f7, 0x700086d4f2);
        }
        131 => {
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d4f7); };
            c.w64(2, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086d4fb); };
            c.w64(15, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d4ff); };
            c.w64(12, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d503); };
            c.w64(6, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d507); };
            c.w64(3, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d50b); };
            c.w64(0, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086d50f); };
            c.w64(8, t284);
            blk = 132; continue;
        }
        132 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 133; continue;
        }
        133 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d519); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086d51a); }
            c.w32(0, 0x2u64);
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086d520); };
            c.w64(3, t285);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700086d532, 0x700086d52d);
        }
        134 => {
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d532); };
            c.w64(2, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086d536); };
            c.w64(15, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d53a); };
            c.w64(12, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d53e); };
            c.w64(6, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d542); };
            c.w64(3, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086d546); };
            c.w64(0, t291);
            blk = 135; continue;
        }
        135 => {
            c.w64(8, c.r64(0));
            blk = 57; continue;
        }
        136 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086d552); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086d553); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086d554); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700086d556); }
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700086d558); };
            c.w64(8, t292);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086d55f); }
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x25a8)) else { return RegfileExit::fault(0x700086d561); };
            c.w64(3, t293);
            c.w32(0, 0x5u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700086d575, 0x700086d570);
        }
        137 => {
            c.w64(8, c.r64(0));
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086d578); };
            c.w64(2, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086d57c); };
            c.w64(6, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086d580); };
            c.w64(3, t296);
            blk = 57; continue;
        }
        138 => {
            return RegfileExit::jump(0x700086d58c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086d594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086d594); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086d598); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086d599); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d59a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086d59f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086d5a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d5a9); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700086d5b8);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086d5b9); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0x2b), c.r32(1)) { return RegfileExit::fault(0x700086d5c1); }
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x700086d5c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700086d5c7); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d5cb); };
            c.w64(0, t5);
            return c.call(0x70007fa8f4, 0x700086d5d4, 0x700086d5cf);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086d5d4); };
            c.w64(5, t6);
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086d5d8); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700086d5dd); };
            c.w64(6, t8);
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086d5e1); };
            c.w64(10, t9);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700086d5e6); }
            let r10 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(0x700086b0f4);
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086d5f4); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700086d602, 0x700086d5fd);
        }
        6 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d602); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x700086d60c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086d614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086d614); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086d618); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086d619); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d61a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086d61f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086d623); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d629); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700086d638);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086d639); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0x2b), c.r32(1)) { return RegfileExit::fault(0x700086d641); }
            c.w32(1, 0x4u64);
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x700086d649); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700086d64c); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d650); };
            c.w64(0, t5);
            return c.call(0x70007fa8f4, 0x700086d659, 0x700086d654);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086d659); };
            c.w64(5, t6);
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086d65d); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700086d662); };
            c.w64(6, t8);
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086d666); };
            c.w64(10, t9);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700086d66b); }
            let r10 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(0x700086b0f4);
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086d679); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700086d687, 0x700086d682);
        }
        6 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d687); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x700086d690);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086d694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086d694); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086d698); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086d699); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d69a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086d69f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086d6a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d6a9); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700086d6b8);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086d6b9); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x2b), c.r32(2)) { return RegfileExit::fault(0x700086d6c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086d6c4); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700086d6c8); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d6cf); };
            c.w64(3, t6);
            return c.call(0x7000869834, 0x700086d6d8, 0x700086d6d3);
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086d6d8); };
            c.w64(5, t7);
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086d6dc); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700086d6e1); };
            c.w64(6, t9);
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086d6e5); };
            c.w64(10, t10);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700086d6ea); }
            let r11 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(0x700086b0f4);
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086d6f8); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700086d706, 0x700086d701);
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d706); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x700086d710);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086d714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086d714); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086d718); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086d719); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d71a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086d71f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086d723); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d729); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700086d738);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086d739); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x2b), c.r32(2)) { return RegfileExit::fault(0x700086d741); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086d744); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086d748); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d74f); };
            c.w64(3, t6);
            return c.call(0x7000869834, 0x700086d758, 0x700086d753);
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086d758); };
            c.w64(5, t7);
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086d75c); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700086d761); };
            c.w64(6, t9);
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086d765); };
            c.w64(10, t10);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700086d76a); }
            let r11 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(0x700086b0f4);
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086d778); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700086d786, 0x700086d781);
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d786); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x700086d790);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086d794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086d794); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086d798); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086d799); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d79a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086d79f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086d7a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d7a9); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700086d7b8);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086d7b9); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x2b), c.r32(2)) { return RegfileExit::fault(0x700086d7c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086d7c4); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d7c8); };
            c.w64(3, t5);
            return c.call(0x7000869974, 0x700086d7d1, 0x700086d7cc);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086d7d1); };
            c.w64(5, t6);
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086d7d5); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700086d7da); };
            c.w64(6, t8);
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086d7de); };
            c.w64(10, t9);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700086d7e3); }
            let r10 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(0x700086b0f4);
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086d7f1); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700086d7ff, 0x700086d7fa);
        }
        6 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d7ff); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x700086d808);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086d814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086d814); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086d818); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086d819); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d81a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086d81f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086d823); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d829); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700086d838);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086d839); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0x2b), c.r32(1)) { return RegfileExit::fault(0x700086d841); }
            c.w32(1, 0x2u64);
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x700086d849); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700086d84c); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d850); };
            c.w64(0, t5);
            return c.call(0x70007fa8f4, 0x700086d859, 0x700086d854);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086d859); };
            c.w64(5, t6);
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086d85d); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700086d862); };
            c.w64(6, t8);
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086d866); };
            c.w64(10, t9);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700086d86b); }
            let r10 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(0x700086b0f4);
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086d879); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700086d887, 0x700086d882);
        }
        6 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086d887); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x700086d890);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086d894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086d894); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086d898); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086d899); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086d89a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086d89f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086d8a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d8b7); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700086d8bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086d8c3); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086d8cd); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086d8d3); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d8da); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086d8e1); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d8e9); };
            c.w32(15, t9);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086d8f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700086d8f8); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700086d905);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086d906); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086d911); };
            c.w64(0, t12);
            let Some(t13) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086d918); };
            c.w64(3, t13);
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086d91b); };
            c.w32(15, t14);
            c.w64(2, c.r64(3).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086d923); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086d92a); };
            c.cmp(t16, c.r64(2), 64);
            if c.cc_be() { blk = 88; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700086d933); }
            let r17 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r17);
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086d93a); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086d93e); };
            c.w64(15, t18);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086d945); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086d949); }
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086d94f); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700086d952); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086d955); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700086d958); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086d95b); };
            c.w64(7, t19);
            let Some(t20) = c.ld32(c.r64(7)) else { return RegfileExit::fault(0x700086d962); };
            c.w32(7, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700086d964); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700086d968); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d97f); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086d986); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086d98d); };
            c.w32(9, t25);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x8eu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086d9a0);
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086d9a1); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d9ab); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700086d9b2); }
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086d9b6); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086d9bd); };
            c.w64(9, t31);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086d9ca); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086d9d1); };
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
            return RegfileExit::fault(0x700086d9eb);
        }
        17 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086d9ec); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086d9ee); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086d9ef); }
            c.w32(0, 0x2u64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086d9f6); };
            c.w64(7, t35);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x700086da02, 0x700086d9fd);
        }
        18 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086da02); };
            c.w64(3, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086da06); };
            c.w64(11, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086da0a); };
            c.w64(6, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086da0e); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086da15); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086da1c); };
            c.w64(1, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086da23); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086da27); };
            c.w64(7, t43);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        19 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086da2d); };
            c.w64(9, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086da31); };
            c.w64(8, t45);
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(7) & 0x6u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086da3b); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086da3c); };
            c.w64(7, t46);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086da43); }
            c.w32(0, 0x2u64);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086da49); };
            c.w64(3, t47);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700086da58, 0x700086da53);
        }
        22 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086da58); };
            c.w64(3, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086da5c); };
            c.w64(11, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086da60); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086da64); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086da68); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086da6f); };
            c.w64(0, t53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086da76); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086da7d); };
            c.w64(12, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086da81); };
            c.w64(8, t56);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086da91); };
            c.w32(7, t57);
            let r58 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r58);
            let Some(t59) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086da98); };
            c.cmp(t59, 0x837u64, 16);
            if c.cc_ne() { blk = 98; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t60) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086daa4); };
            c.w32(7, t60);
            let r61 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r61);
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086daab); };
            c.w32(9, t62);
            let r63 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r63);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_g() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086dab8); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(0));
            return c.call(0x70007f21d4, 0x700086dac6, 0x700086dac1);
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086dac8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086daca); }
            c.w64(7, c.r64(9));
            c.w32(0, 0x1u64);
            return c.call(0x70007f21d4, 0x700086dad8, 0x700086dad3);
        }
        29 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086dad8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t64) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086dae4); };
            c.w32(8, t64);
            let r65 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r65);
            let Some(t66) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086daeb); };
            c.cmp(t66, 0xbau64, 16);
            if c.cc_b() { blk = 96; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086daf8); };
            c.w64(6, t67);
            let Some(t68) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086dafc); };
            c.w32(9, t68);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x700086db0c);
        }
        33 => {
            let Some(t69) = c.ld32(c.r64(6).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x700086db0d); };
            c.cmp(t69, c.r32(8), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x700086db16); };
            c.w64(3, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086db1d); };
            c.w64(6, t71);
            return c.call(0x7000870914, 0x700086db26, 0x700086db21);
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700086db26); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x700086db2a); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086db31); };
            c.w64(0, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086db35); };
            c.w64(6, t74);
            return c.call(0x7000870914, 0x700086db3e, 0x700086db39);
        }
        36 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086db3e); };
            c.w64(8, t75);
            blk = 38; continue;
        }
        37 => {
            let Some(t76) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086db44); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086db4b); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086db51); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t80) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086db5b); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700086db5f); };
            c.cmp(t81, c.r32(9), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a16f4, 0x700086db70, 0x700086db6b);
        }
        41 => {
            c.w64(8, c.r64(0));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086db73); };
            c.w64(0, t82);
            blk = 42; continue;
        }
        42 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086db77); };
            c.w64(6, t83);
            let Some(t84) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086db7b); };
            c.w32(9, t84);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700086db85); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x700086db92);
        }
        44 => {
            let Some(t85) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086db93); };
            c.w32(9, t85);
            let r86 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r86);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(9));
            return c.call(0x70008ba474, 0x700086dba8, 0x700086dba3);
        }
        45 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086dba8); };
            c.w64(8, t87);
            let Some(t88) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086dbac); };
            c.w32(9, t88);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_a() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            return RegfileExit::fault(0x700086dbbf);
        }
        47 => {
            let Some(t89) = c.ld32(c.r64(8).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086dbc0); };
            c.w32(11, t89);
            c.cmp(c.r64(9), 0x79u64, 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::fault(0x700086dbcd);
        }
        49 => {
            let Some(t90) = c.ld32(c.r64(8).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x700086dbce); };
            c.w32(8, t90);
            c.w32(9, 0x8u64);
            c.w64(12, c.r64(9));
            let r91 = c.add(c.r64(12), 0xcu64, 64);
            c.w64(12, r91);
            if c.cc_o() { blk = 85; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086dbe8); };
            c.w64(15, t92);
            let Some(t93) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086dbef); };
            c.w64(3, t93);
            c.w64(2, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086dbf6); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086dbfd); };
            c.cmp(t95, c.r64(2), 64);
            if c.cc_be() { blk = 92; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 92; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700086dc13); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086dc1a); }
            c.w32(8, 0x6u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086dc25); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 55; continue;
        }
        54 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086dc2f); }
            let r96 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r96);
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(9), 0x14u64, 64);
            if c.cc_ne() { blk = 54; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086dc3e); };
            c.w32(8, t97);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            return RegfileExit::fault(0x700086dc4f);
        }
        58 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x700086dc50); };
            c.w64(9, t98);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700086dc57); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x700086dc62);
        }
        60 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086dc63); };
            c.w64(8, t99);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086dc6a); }
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086dc6f); };
            c.w64(9, t100);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700086dc73); }
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x1088)) else { return RegfileExit::fault(0x700086dc78); };
            c.w64(9, t101);
            let Some(t102) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086dc7f); };
            c.w32(3, t102);
            let r103 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r103);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t104) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x700086dc95); };
            c.w64(3, t104);
            blk = 81; continue;
        }
        62 => {
            let Some(t105) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086dc9e); };
            c.w32(2, t105);
            let r106 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r106);
            let Some(t107) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086dca4); };
            c.w32(2, t107);
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_l() { blk = 69; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_l() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(2), 0x9du64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(2), 0x9eu64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(2), 0x5eu64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_l() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(2), 0x53u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(2), 0x48u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(2), 0x49u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086dcf0); };
            c.w32(3, t108);
            blk = 81; continue;
        }
        75 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x700086dcf5); };
            c.w64(3, t109);
            blk = 81; continue;
        }
        76 => {
            let Some(t110) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086dcfe); };
            c.w32(3, t110);
            blk = 81; continue;
        }
        77 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x700086dd03); };
            c.w64(3, t111);
            blk = 81; continue;
        }
        78 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x700086dd0c); };
            c.w64(3, t112);
            blk = 81; continue;
        }
        79 => {
            let Some(t113) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086dd15); };
            c.w32(3, t113);
            blk = 81; continue;
        }
        80 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x700086dd1a); };
            c.w64(3, t114);
            blk = 81; continue;
        }
        81 => {
            let Some(t115) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086dd21); };
            c.w64(2, t115);
            c.w64(6, c.r64(2).wrapping_add(0x1c));
            let Some(t116) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086dd28); };
            c.cmp(t116, c.r64(6), 64);
            if c.cc_be() { blk = 94; continue; }
            blk = 82; continue;
        }
        82 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x700086dd31); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 83; continue;
        }
        83 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700086dd38); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086dd3c); };
            c.w64(11, t117);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700086dd43); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700086dd47); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086dd4b); };
            c.w64(11, t118);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x700086dd52); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086dd56); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086dd5a); }
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x700086dd5e); }
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086dd62); };
            c.w64(6, t119);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(8));
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086dd6c); };
            c.w64(2, t120);
            return c.call(0x70008b2ef4, 0x700086dd75, 0x700086dd70);
        }
        84 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086dd75); };
            c.w64(8, t121);
            let r122 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r122);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086dd80); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086dd81); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086dd87); }
            return c.ret(0x0, 0x700086dd89);
        }
        85 => {
            return RegfileExit::fault(0x700086dd8a);
        }
        86 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086dd8b); }
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086dd8f); };
            c.w64(3, t123);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086dda0, 0x700086dd9b);
        }
        87 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086dda0); };
            c.w64(6, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086dda4); };
            c.w64(0, t125);
            blk = 1; continue;
        }
        88 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700086ddb2); }
            return c.call(0x70007febf4, 0x700086ddbb, 0x700086ddb6);
        }
        89 => {
            c.w64(3, c.r64(0));
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ddbe); };
            c.w64(11, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ddc2); };
            c.w64(6, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ddc6); };
            c.w64(9, t128);
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ddca); };
            c.w64(0, t129);
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ddd1); };
            c.w64(1, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086ddd8); };
            c.w64(12, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086dddc); };
            c.w64(8, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086dde0); };
            c.w64(15, t133);
            blk = 8; continue;
        }
        90 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086dde9); }
            c.w32(0, 0x1u64);
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086ddef); };
            c.w64(3, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ddf6); };
            c.w64(6, t135);
            return c.call(0x700086ed74, 0x700086ddff, 0x700086ddfa);
        }
        91 => {
            blk = 18; continue;
        }
        92 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086de04); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086de08); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086de0c); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700086de18, 0x700086de13);
        }
        93 => {
            c.w64(12, c.r64(0));
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086de1b); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086de1f); };
            c.w64(11, t137);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086de23); };
            c.w64(15, t138);
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086de2a); };
            c.w64(1, t139);
            c.w32(9, 0x8u64);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086de37); };
            c.w64(8, t140);
            blk = 53; continue;
        }
        94 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086de45); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700086de49); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086de4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086de51); }
            return c.call(0x70007febf4, 0x700086de5a, 0x700086de55);
        }
        95 => {
            c.w64(15, c.r64(0));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086de5d); };
            c.w64(0, t141);
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086de61); };
            c.w64(8, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086de68); };
            c.w64(3, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086de6c); };
            c.w64(12, t144);
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x1088)) else { return RegfileExit::fault(0x700086de70); };
            c.w64(9, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086de77); };
            c.w64(11, t146);
            blk = 83; continue;
        }
        96 => {
            c.w32(8, 0x8eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086de86); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086de88); }
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086de89); };
            c.w64(3, t147);
            c.w32(0, 0x2u64);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086de95); };
            c.w64(6, t148);
            return c.call(0x700086ed74, 0x700086de9e, 0x700086de99);
        }
        97 => {
            blk = 99; continue;
        }
        98 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086dea6); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086dea8); };
            c.w64(8, t149);
            let Some(t150) = c.ld32(c.r64(8).wrapping_add(0xff)) else { return RegfileExit::fault(0x700086deaf); };
            c.w32(8, t150);
            let r151 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r151);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086deb9); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086debb); }
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086debd); };
            c.w64(3, t152);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700086ded1, 0x700086decc);
        }
        99 => {
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086ded1); };
            c.w64(1, t153);
            c.w64(3, c.r64(0));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086dedb); };
            c.w64(0, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086dedf); };
            c.w64(6, t155);
            return c.call(0x70008b1fb4, 0x700086dee8, 0x700086dee3);
        }
        100 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086dee8); };
            c.w64(8, t156);
            let r157 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086def0); };
            c.w64(0, t158);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086def7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086def8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086defe); }
            return c.ret(0x0, 0x700086df00);
        }
        101 => {
            return RegfileExit::jump(0x700086df04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086df14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086df14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086df18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086df19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086df1a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086df1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086df23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086df37); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700086df3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700086df43); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086df4d); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086df53); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086df5a); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086df61); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086df69); };
            c.w32(15, t9);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086df74); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086df78); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700086df85);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086df86); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086df91); };
            c.w64(0, t12);
            let Some(t13) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086df98); };
            c.w64(3, t13);
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086df9b); };
            c.w32(15, t14);
            c.w64(2, c.r64(3).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086dfa3); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086dfaa); };
            c.cmp(t16, c.r64(2), 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700086dfb3); }
            let r17 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r17);
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086dfba); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086dfbe); };
            c.w64(15, t18);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086dfc5); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086dfc9); }
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086dfcf); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700086dfd2); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086dfd5); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700086dfd8); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086dfdb); };
            c.w64(7, t19);
            let Some(t20) = c.ld32(c.r64(7)) else { return RegfileExit::fault(0x700086dfe2); };
            c.w32(7, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700086dfe4); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700086dfe8); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086dfff); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086e006); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e00d); };
            c.w32(9, t25);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x8eu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086e020);
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086e021); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e02b); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700086e032); }
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086e036); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e03d); };
            c.w64(9, t31);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e04a); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e051); };
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
            return RegfileExit::fault(0x700086e06b);
        }
        17 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086e06c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086e06e); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086e06f); }
            c.w32(0, 0x2u64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086e076); };
            c.w64(7, t35);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x700086e082, 0x700086e07d);
        }
        18 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e082); };
            c.w64(3, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e086); };
            c.w64(11, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e08a); };
            c.w64(6, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086e08e); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e095); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e09c); };
            c.w64(1, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e0a3); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086e0a7); };
            c.w64(7, t43);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        19 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e0ad); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e0b1); };
            c.w64(9, t45);
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(7) & 0x6u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086e0bb); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e0bc); };
            c.w64(7, t46);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086e0c3); }
            c.w32(0, 0x2u64);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086e0c9); };
            c.w64(3, t47);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700086e0d8, 0x700086e0d3);
        }
        22 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e0d8); };
            c.w64(3, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e0dc); };
            c.w64(11, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e0e0); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e0e4); };
            c.w64(8, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e0e8); };
            c.w64(9, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086e0ec); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e0f3); };
            c.w64(0, t54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e0fa); };
            c.w64(1, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e101); };
            c.w64(12, t56);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 106; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e111); };
            c.w32(7, t57);
            let r58 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r58);
            let Some(t59) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e118); };
            c.cmp(t59, 0x837u64, 16);
            if c.cc_ne() { blk = 106; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t60) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086e124); };
            c.w32(9, t60);
            let r61 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700086e12b); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0xad8)) else { return RegfileExit::fault(0x700086e12f); };
            c.w64(3, t62);
            c.w64(0, c.r64(9));
            c.w64(7, c.r64(3));
            return c.call(0x7000870914, 0x700086e141, 0x700086e13c);
        }
        26 => {
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700086e141); };
            c.cmp(t63, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e150); };
            c.cmp(t64, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r65 = c.r32(8) | c.r32(9);
            c.w32(8, r65);
            c.logic(r65, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e168); };
            c.cmp(t66, 0x0u64, 64);
            if c.cc_g() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e16f); };
            if !c.push64(t67) { return RegfileExit::fault(0x700086e16f); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e172); };
            c.w64(6, t68);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f21d4, 0x700086e183, 0x700086e17e);
        }
        29 => {
            blk = 31; continue;
        }
        30 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e185); };
            if !c.push64(t69) { return RegfileExit::fault(0x700086e185); }
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e188); };
            if !c.push64(t70) { return RegfileExit::fault(0x700086e188); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e193); };
            c.w64(6, t71);
            return c.call(0x70007f21d4, 0x700086e19c, 0x700086e197);
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086e19c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t72) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e1a8); };
            c.w32(8, t72);
            let r73 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r73);
            let Some(t74) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e1af); };
            c.cmp(t74, 0xbau64, 16);
            if c.cc_b() { blk = 104; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e1bc); };
            c.w64(6, t75);
            let Some(t76) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e1c0); };
            c.w32(9, t76);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::fault(0x700086e1d0);
        }
        35 => {
            let Some(t77) = c.ld32(c.r64(6).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x700086e1d1); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x700086e1da); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e1e1); };
            c.w64(6, t79);
            return c.call(0x7000870914, 0x700086e1ea, 0x700086e1e5);
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700086e1ea); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x700086e1ee); };
            c.w64(3, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e1f5); };
            c.w64(0, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e1f9); };
            c.w64(6, t82);
            return c.call(0x7000870914, 0x700086e202, 0x700086e1fd);
        }
        38 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086e202); };
            c.w64(8, t83);
            blk = 40; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086e208); };
            c.w32(8, t84);
            let r85 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r85);
            let Some(t86) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086e20f); };
            c.w32(0, t86);
            let r87 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r87);
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086e215); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t88) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e21f); };
            c.w32(9, t88);
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700086e223); };
            c.cmp(t89, c.r32(9), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a16f4, 0x700086e234, 0x700086e22f);
        }
        43 => {
            c.w64(8, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e237); };
            c.w64(0, t90);
            blk = 44; continue;
        }
        44 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e23b); };
            c.w64(6, t91);
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e23f); };
            c.w32(9, t92);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700086e249); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700086e256);
        }
        46 => {
            let Some(t93) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086e257); };
            c.w32(9, t93);
            let r94 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r94);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(9));
            return c.call(0x70008ba474, 0x700086e26c, 0x700086e267);
        }
        47 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e26c); };
            c.w64(8, t95);
            let Some(t96) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e270); };
            c.w32(9, t96);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::fault(0x700086e283);
        }
        49 => {
            let Some(t97) = c.ld32(c.r64(8).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086e284); };
            c.w32(11, t97);
            c.cmp(c.r64(9), 0x79u64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x700086e291);
        }
        51 => {
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x700086e292); };
            c.w32(8, t98);
            c.w32(9, 0x8u64);
            c.w64(12, c.r64(9));
            let r99 = c.add(c.r64(12), 0xcu64, 64);
            c.w64(12, r99);
            if c.cc_o() { blk = 87; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e2ac); };
            c.w64(15, t100);
            let Some(t101) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086e2b3); };
            c.w64(3, t101);
            c.w64(2, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e2ba); };
            c.w64(1, t102);
            let Some(t103) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086e2c1); };
            c.cmp(t103, c.r64(2), 64);
            if c.cc_be() { blk = 100; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 100; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700086e2d7); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086e2de); }
            c.w32(8, 0x6u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086e2e9); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 57; continue;
        }
        56 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086e2f3); }
            let r104 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r104);
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(9), 0x14u64, 64);
            if c.cc_ne() { blk = 56; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e302); };
            c.w32(8, t105);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x700086e313);
        }
        60 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x700086e314); };
            c.w64(9, t106);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700086e31b); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x700086e326);
        }
        62 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e327); };
            c.w64(8, t107);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086e32e); }
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086e333); };
            c.w64(9, t108);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700086e337); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x1088)) else { return RegfileExit::fault(0x700086e33c); };
            c.w64(9, t109);
            let Some(t110) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e343); };
            c.w32(3, t110);
            let r111 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r111);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t112) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x700086e359); };
            c.w64(3, t112);
            blk = 83; continue;
        }
        64 => {
            let Some(t113) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e362); };
            c.w32(2, t113);
            let r114 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r114);
            let Some(t115) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e368); };
            c.w32(2, t115);
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_l() { blk = 71; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_l() { blk = 69; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(2), 0x9du64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(2), 0x9eu64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(2), 0x5eu64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_l() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(2), 0x53u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(2), 0x48u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(2), 0x49u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t116) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e3b4); };
            c.w32(3, t116);
            blk = 83; continue;
        }
        77 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x700086e3b9); };
            c.w64(3, t117);
            blk = 83; continue;
        }
        78 => {
            let Some(t118) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e3c2); };
            c.w32(3, t118);
            blk = 83; continue;
        }
        79 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x700086e3c7); };
            c.w64(3, t119);
            blk = 83; continue;
        }
        80 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x700086e3d0); };
            c.w64(3, t120);
            blk = 83; continue;
        }
        81 => {
            let Some(t121) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086e3d9); };
            c.w32(3, t121);
            blk = 83; continue;
        }
        82 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x700086e3de); };
            c.w64(3, t122);
            blk = 83; continue;
        }
        83 => {
            let Some(t123) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086e3e5); };
            c.w64(2, t123);
            c.w64(6, c.r64(2).wrapping_add(0x1c));
            let Some(t124) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086e3ec); };
            c.cmp(t124, c.r64(6), 64);
            if c.cc_be() { blk = 102; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x700086e3f5); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 85; continue;
        }
        85 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700086e3fc); }
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086e400); };
            c.w64(11, t125);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700086e407); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700086e40b); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086e40f); };
            c.w64(11, t126);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x700086e416); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086e41a); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086e41e); }
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x700086e422); }
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e426); };
            c.w64(6, t127);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(8));
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e430); };
            c.w64(2, t128);
            return c.call(0x70008b2ef4, 0x700086e439, 0x700086e434);
        }
        86 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e439); };
            c.w64(8, t129);
            let r130 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r130);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086e444); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086e445); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086e44b); }
            return c.ret(0x0, 0x700086e44d);
        }
        87 => {
            return RegfileExit::fault(0x700086e44e);
        }
        88 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e44f); };
            c.w64(3, t131);
            blk = 89; continue;
        }
        89 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086e453); };
            c.w64(1, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e45a); };
            c.w64(0, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e45e); };
            c.w64(6, t134);
            return c.call(0x70008b1fb4, 0x700086e467, 0x700086e462);
        }
        90 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e467); };
            c.w64(8, t135);
            let r136 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e46f); };
            c.w64(0, t137);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086e476); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086e477); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086e47d); }
            return c.ret(0x0, 0x700086e47f);
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086e480); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086e484); };
            c.w64(3, t138);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086e495, 0x700086e490);
        }
        92 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e495); };
            c.w64(6, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e499); };
            c.w64(0, t140);
            blk = 1; continue;
        }
        93 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700086e4a7); }
            return c.call(0x70007febf4, 0x700086e4b0, 0x700086e4ab);
        }
        94 => {
            c.w64(3, c.r64(0));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e4b3); };
            c.w64(11, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e4b7); };
            c.w64(6, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e4bb); };
            c.w64(8, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e4bf); };
            c.w64(9, t144);
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e4c3); };
            c.w64(0, t145);
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e4ca); };
            c.w64(1, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e4d1); };
            c.w64(12, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086e4d5); };
            c.w64(15, t148);
            blk = 8; continue;
        }
        95 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086e4de); }
            c.w32(0, 0x1u64);
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086e4e4); };
            c.w64(3, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e4eb); };
            c.w64(6, t150);
            return c.call(0x700086ed74, 0x700086e4f4, 0x700086e4ef);
        }
        96 => {
            blk = 18; continue;
        }
        97 => {
            c.w64(3, c.r64(0));
            blk = 89; continue;
        }
        98 => {
            c.w64(3, c.r64(0));
            blk = 89; continue;
        }
        99 => {
            c.w64(3, c.r64(0));
            blk = 89; continue;
        }
        100 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086e511); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086e515); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086e519); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700086e525, 0x700086e520);
        }
        101 => {
            c.w64(12, c.r64(0));
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e528); };
            c.w64(0, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086e52c); };
            c.w64(11, t152);
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e530); };
            c.w64(15, t153);
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e537); };
            c.w64(1, t154);
            c.w32(9, 0x8u64);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e544); };
            c.w64(8, t155);
            blk = 55; continue;
        }
        102 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086e552); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700086e556); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086e55a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086e55e); }
            return c.call(0x70007febf4, 0x700086e567, 0x700086e562);
        }
        103 => {
            c.w64(15, c.r64(0));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e56a); };
            c.w64(0, t156);
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e56e); };
            c.w64(8, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e575); };
            c.w64(3, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e579); };
            c.w64(12, t159);
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x1088)) else { return RegfileExit::fault(0x700086e57d); };
            c.w64(9, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086e584); };
            c.w64(11, t161);
            blk = 85; continue;
        }
        104 => {
            c.w32(8, 0x8eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086e593); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086e595); }
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086e596); };
            c.w64(3, t162);
            c.w32(0, 0x2u64);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e5a2); };
            c.w64(6, t163);
            return c.call(0x700086ed74, 0x700086e5ab, 0x700086e5a6);
        }
        105 => {
            c.w64(3, c.r64(0));
            blk = 89; continue;
        }
        106 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086e5b9); }
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086e5bb); };
            c.w64(8, t164);
            let Some(t165) = c.ld32(c.r64(8).wrapping_add(0x103)) else { return RegfileExit::fault(0x700086e5c2); };
            c.w32(8, t165);
            let r166 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r166);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086e5cc); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086e5ce); }
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086e5d0); };
            c.w64(3, t167);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700086e5e4, 0x700086e5df);
        }
        107 => {
            c.w64(3, c.r64(0));
            blk = 89; continue;
        }
        108 => {
            return RegfileExit::jump(0x700086e5f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086e5f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086e5f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086e5f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086e5f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086e5fa); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086e5ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086e603); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086e617); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700086e61f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086e623); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086e62d); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e633); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e63a); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086e641); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e649); };
            c.w32(15, t9);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086e654); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086e658); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700086e665);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086e666); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e671); };
            c.w64(0, t12);
            let Some(t13) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086e678); };
            c.w64(3, t13);
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086e67b); };
            c.w32(15, t14);
            c.w64(2, c.r64(3).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e683); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086e68a); };
            c.cmp(t16, c.r64(2), 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700086e693); }
            let r17 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r17);
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086e69a); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086e69e); };
            c.w64(15, t18);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086e6a5); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086e6a9); }
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086e6af); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700086e6b2); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086e6b5); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700086e6b8); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086e6bb); };
            c.w64(7, t19);
            let Some(t20) = c.ld32(c.r64(7)) else { return RegfileExit::fault(0x700086e6c2); };
            c.w32(7, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700086e6c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700086e6c8); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e6df); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086e6e6); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e6ed); };
            c.w32(9, t25);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x8eu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086e700);
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086e701); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e70b); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700086e712); }
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086e716); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e71d); };
            c.w64(9, t31);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e72a); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e731); };
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
            return RegfileExit::fault(0x700086e74b);
        }
        17 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086e74c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086e74e); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086e74f); }
            c.w32(0, 0x2u64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086e756); };
            c.w64(7, t35);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x700086e762, 0x700086e75d);
        }
        18 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e762); };
            c.w64(3, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e766); };
            c.w64(11, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e76a); };
            c.w64(6, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086e76e); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e775); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e77c); };
            c.w64(1, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e783); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086e787); };
            c.w64(7, t43);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        19 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e78d); };
            c.w64(9, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e791); };
            c.w64(8, t45);
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(7) & 0x6u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086e79b); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e79c); };
            c.w64(7, t46);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086e7a3); }
            c.w32(0, 0x2u64);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086e7a9); };
            c.w64(3, t47);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700086e7b8, 0x700086e7b3);
        }
        22 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086e7b8); };
            c.w64(3, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e7bc); };
            c.w64(11, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e7c0); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e7c4); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e7c8); };
            c.w64(8, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086e7cc); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e7d3); };
            c.w64(0, t54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e7da); };
            c.w64(1, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e7e1); };
            c.w64(12, t56);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e7f1); };
            c.w32(7, t57);
            let r58 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r58);
            let Some(t59) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e7f8); };
            c.cmp(t59, 0x837u64, 16);
            if c.cc_ne() { blk = 103; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t60) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086e804); };
            c.w32(9, t60);
            let r61 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086e80b); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x700086e80f); };
            c.w64(3, t62);
            c.w64(0, c.r64(9));
            c.w64(7, c.r64(3));
            return c.call(0x7000870914, 0x700086e821, 0x700086e81c);
        }
        26 => {
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700086e821); };
            c.cmp(t63, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086e830); };
            c.cmp(t64, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r65 = c.r32(8) | c.r32(9);
            c.w32(8, r65);
            c.logic(r65, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086e848); };
            c.cmp(t66, 0x0u64, 64);
            if c.cc_g() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e84f); };
            if !c.push64(t67) { return RegfileExit::fault(0x700086e84f); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e852); };
            c.w64(6, t68);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f21d4, 0x700086e863, 0x700086e85e);
        }
        29 => {
            blk = 31; continue;
        }
        30 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086e865); };
            if !c.push64(t69) { return RegfileExit::fault(0x700086e865); }
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e868); };
            if !c.push64(t70) { return RegfileExit::fault(0x700086e868); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e873); };
            c.w64(6, t71);
            return c.call(0x70007f21d4, 0x700086e87c, 0x700086e877);
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086e87c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 101; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t72) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e888); };
            c.w32(8, t72);
            let r73 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r73);
            let Some(t74) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086e88f); };
            c.cmp(t74, 0xbau64, 16);
            if c.cc_b() { blk = 101; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e89c); };
            c.w64(6, t75);
            let Some(t76) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e8a0); };
            c.w32(9, t76);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::fault(0x700086e8b0);
        }
        35 => {
            let Some(t77) = c.ld32(c.r64(6).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x700086e8b1); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x700086e8ba); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e8c1); };
            c.w64(6, t79);
            return c.call(0x7000870914, 0x700086e8ca, 0x700086e8c5);
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700086e8ca); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x700086e8ce); };
            c.w64(3, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e8d5); };
            c.w64(0, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086e8d9); };
            c.w64(6, t82);
            return c.call(0x7000870914, 0x700086e8e2, 0x700086e8dd);
        }
        38 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086e8e2); };
            c.w64(8, t83);
            blk = 40; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086e8e8); };
            c.w32(8, t84);
            let r85 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r85);
            let Some(t86) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086e8ef); };
            c.w32(0, t86);
            let r87 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r87);
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086e8f5); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t88) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086e8ff); };
            c.w32(9, t88);
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700086e903); };
            c.cmp(t89, c.r32(9), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a16f4, 0x700086e914, 0x700086e90f);
        }
        43 => {
            c.w64(8, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086e917); };
            c.w64(0, t90);
            blk = 44; continue;
        }
        44 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e91b); };
            c.w64(6, t91);
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e91f); };
            c.w32(9, t92);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700086e929); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700086e936);
        }
        46 => {
            let Some(t93) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086e937); };
            c.w32(9, t93);
            let r94 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r94);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(9));
            return c.call(0x70008ba474, 0x700086e94c, 0x700086e947);
        }
        47 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086e94c); };
            c.w64(8, t95);
            let Some(t96) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e950); };
            c.w32(9, t96);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::fault(0x700086e963);
        }
        49 => {
            let Some(t97) = c.ld32(c.r64(8).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086e964); };
            c.w32(11, t97);
            c.cmp(c.r64(9), 0x79u64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x700086e971);
        }
        51 => {
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x700086e972); };
            c.w32(8, t98);
            c.w32(9, 0x8u64);
            c.w64(12, c.r64(9));
            let r99 = c.add(c.r64(12), 0xcu64, 64);
            c.w64(12, r99);
            if c.cc_o() { blk = 87; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086e98c); };
            c.w64(15, t100);
            let Some(t101) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086e993); };
            c.w64(3, t101);
            c.w64(2, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086e99a); };
            c.w64(1, t102);
            let Some(t103) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086e9a1); };
            c.cmp(t103, c.r64(2), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 97; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700086e9b7); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086e9be); }
            c.w32(8, 0x6u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086e9c9); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 57; continue;
        }
        56 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086e9d3); }
            let r104 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r104);
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(9), 0x14u64, 64);
            if c.cc_ne() { blk = 56; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086e9e2); };
            c.w32(8, t105);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x700086e9f3);
        }
        60 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x700086e9f4); };
            c.w64(9, t106);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700086e9fb); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x700086ea06);
        }
        62 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086ea07); };
            c.w64(8, t107);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086ea0e); }
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086ea13); };
            c.w64(9, t108);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700086ea17); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x1088)) else { return RegfileExit::fault(0x700086ea1c); };
            c.w64(9, t109);
            let Some(t110) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086ea23); };
            c.w32(3, t110);
            let r111 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r111);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t112) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x700086ea39); };
            c.w64(3, t112);
            blk = 83; continue;
        }
        64 => {
            let Some(t113) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ea42); };
            c.w32(2, t113);
            let r114 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r114);
            let Some(t115) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086ea48); };
            c.w32(2, t115);
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_l() { blk = 71; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_l() { blk = 69; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(2), 0x9du64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(2), 0x9eu64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(2), 0x5eu64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_l() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(2), 0x53u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(2), 0x48u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(2), 0x49u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t116) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086ea94); };
            c.w32(3, t116);
            blk = 83; continue;
        }
        77 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x700086ea99); };
            c.w64(3, t117);
            blk = 83; continue;
        }
        78 => {
            let Some(t118) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086eaa2); };
            c.w32(3, t118);
            blk = 83; continue;
        }
        79 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x700086eaa7); };
            c.w64(3, t119);
            blk = 83; continue;
        }
        80 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x700086eab0); };
            c.w64(3, t120);
            blk = 83; continue;
        }
        81 => {
            let Some(t121) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086eab9); };
            c.w32(3, t121);
            blk = 83; continue;
        }
        82 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x700086eabe); };
            c.w64(3, t122);
            blk = 83; continue;
        }
        83 => {
            let Some(t123) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086eac5); };
            c.w64(2, t123);
            c.w64(6, c.r64(2).wrapping_add(0x1c));
            let Some(t124) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086eacc); };
            c.cmp(t124, c.r64(6), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x700086ead5); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 85; continue;
        }
        85 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700086eadc); }
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086eae0); };
            c.w64(11, t125);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700086eae7); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700086eaeb); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086eaef); };
            c.w64(11, t126);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x700086eaf6); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086eafa); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086eafe); }
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x700086eb02); }
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086eb06); };
            c.w64(6, t127);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(8));
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086eb10); };
            c.w64(2, t128);
            return c.call(0x70008b2ef4, 0x700086eb19, 0x700086eb14);
        }
        86 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086eb19); };
            c.w64(8, t129);
            let r130 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r130);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086eb24); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086eb25); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086eb2b); }
            return c.ret(0x0, 0x700086eb2d);
        }
        87 => {
            return RegfileExit::fault(0x700086eb2e);
        }
        88 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086eb2f); };
            c.w64(3, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086eb36); };
            c.w64(0, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086eb3a); };
            c.w64(6, t133);
            return c.call(0x700082faf4, 0x700086eb43, 0x700086eb3e);
        }
        89 => {
            c.w64(3, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086eb46); };
            c.w64(0, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086eb4a); };
            c.w64(6, t135);
            return c.call(0x70008ba734, 0x700086eb53, 0x700086eb4e);
        }
        90 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086eb53); };
            c.w64(8, t136);
            let r137 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086eb5b); };
            c.w64(0, t138);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086eb62); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086eb63); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086eb69); }
            return c.ret(0x0, 0x700086eb6b);
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086eb6c); }
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086eb70); };
            c.w64(3, t139);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086eb81, 0x700086eb7c);
        }
        92 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086eb81); };
            c.w64(6, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086eb85); };
            c.w64(0, t141);
            blk = 1; continue;
        }
        93 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700086eb93); }
            return c.call(0x70007febf4, 0x700086eb9c, 0x700086eb97);
        }
        94 => {
            c.w64(3, c.r64(0));
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086eb9f); };
            c.w64(11, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086eba3); };
            c.w64(6, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086eba7); };
            c.w64(9, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086ebab); };
            c.w64(8, t145);
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ebaf); };
            c.w64(0, t146);
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ebb6); };
            c.w64(1, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086ebbd); };
            c.w64(12, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ebc1); };
            c.w64(15, t149);
            blk = 8; continue;
        }
        95 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086ebca); }
            c.w32(0, 0x1u64);
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086ebd0); };
            c.w64(3, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ebd7); };
            c.w64(6, t151);
            return c.call(0x700086ed74, 0x700086ebe0, 0x700086ebdb);
        }
        96 => {
            blk = 18; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086ebe5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086ebe9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086ebed); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700086ebf9, 0x700086ebf4);
        }
        98 => {
            c.w64(12, c.r64(0));
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ebfc); };
            c.w64(0, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ec00); };
            c.w64(11, t153);
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ec04); };
            c.w64(15, t154);
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ec0b); };
            c.w64(1, t155);
            c.w32(9, 0x8u64);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086ec18); };
            c.w64(8, t156);
            blk = 55; continue;
        }
        99 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700086ec26); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700086ec2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086ec2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086ec32); }
            return c.call(0x70007febf4, 0x700086ec3b, 0x700086ec36);
        }
        100 => {
            c.w64(15, c.r64(0));
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ec3e); };
            c.w64(0, t157);
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086ec42); };
            c.w64(8, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086ec49); };
            c.w64(3, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086ec4d); };
            c.w64(12, t160);
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x1088)) else { return RegfileExit::fault(0x700086ec51); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ec58); };
            c.w64(11, t162);
            blk = 85; continue;
        }
        101 => {
            c.w32(8, 0x8eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ec67); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086ec69); }
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086ec6a); };
            c.w64(3, t163);
            c.w32(0, 0x2u64);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ec76); };
            c.w64(6, t164);
            return c.call(0x700086ed74, 0x700086ec7f, 0x700086ec7a);
        }
        102 => {
            blk = 104; continue;
        }
        103 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ec87); }
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086ec89); };
            c.w64(8, t165);
            let Some(t166) = c.ld32(c.r64(8).wrapping_add(0x107)) else { return RegfileExit::fault(0x700086ec90); };
            c.w32(8, t166);
            let r167 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r167);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ec9a); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086ec9c); }
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086ec9e); };
            c.w64(3, t168);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700086ecb2, 0x700086ecad);
        }
        104 => {
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086ecb2); };
            c.w64(1, t169);
            c.w64(3, c.r64(0));
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ecbc); };
            c.w64(0, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ecc0); };
            c.w64(6, t171);
            return c.call(0x70008b1fb4, 0x700086ecc9, 0x700086ecc4);
        }
        105 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ecc9); };
            c.w64(8, t172);
            let r173 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ecd1); };
            c.w64(0, t174);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086ecd8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086ecd9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086ecdf); }
            return c.ret(0x0, 0x700086ece1);
        }
        106 => {
            return RegfileExit::jump(0x700086ece4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086ecf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086ecf4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086ecf8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086ecf9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086ecfa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086ecff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086ed05); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700086ed14);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086ed15); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086ed1b); };
            c.w64(0, t5);
            return c.call(0x700082faf4, 0x700086ed24, 0x700086ed1f);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086ed24); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086ed2b); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x10, 0x700086ed32);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086ed35); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086ed3c); }
            return c.ret(0x0, 0x700086ed3e);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700086ed3f); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086ed43); };
            c.w64(3, t7);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700086ed51, 0x700086ed4c);
        }
        8 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ed51); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        9 => {
            return RegfileExit::jump(0x700086ed58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086ed74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086ed74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x700086ed78); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086ed7a); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086ed7f); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086ed86); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086ed8d); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086eda1); }
            c.w64(1, c.r64(12));
            c.w64(2, c.r64(15));
            c.w64(8, c.r64(13));
            return c.call(c.r64(3), 0x700086edb0, 0x700086edae);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086edb0); };
            c.cmp(c.r32(0), t2, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086edbd); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086edc1); };
            c.w64(5, t4);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086edc9); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086edca); };
            c.w64(6, t5);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086edd1); }
            return c.ret(0x0, 0x700086eddc);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086edef); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086edf4); };
            c.w64(0, t8);
            c.w64(10, 0x700086ee02u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086ee02); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086ee06); }
            return c.call(c.r64(0), 0x700086ee0c, 0x700086ee0a);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086ee0c); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086ee14); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086ee19); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086ee20); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086ee27); };
            c.w64(5, t12);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086ee33); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086ee3e); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086ee49); };
            c.w64(7, t13);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086ee52);
        }
        8 => {
            return RegfileExit::fault(0x700086ee53);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086ee74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086ee74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x2au64) { return RegfileExit::fault(0x700086ee78); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086ee7a); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086ee7f); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086ee86); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086ee8d); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086eea1); }
            c.w64(1, c.r64(12));
            c.w64(2, c.r64(15));
            c.w64(8, c.r64(13));
            return c.call(c.r64(3), 0x700086eeb0, 0x700086eeae);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086eeb0); };
            c.cmp(c.r32(0), t2, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086eebd); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086eec1); };
            c.w64(5, t4);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086eec9); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086eeca); };
            c.w64(6, t5);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086eed1); }
            return c.ret(0x0, 0x700086eedc);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086eeef); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086eef4); };
            c.w64(0, t8);
            c.w64(10, 0x700086ef02u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086ef02); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086ef06); }
            return c.call(c.r64(0), 0x700086ef0c, 0x700086ef0a);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086ef0c); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086ef14); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086ef19); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086ef20); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086ef27); };
            c.w64(5, t12);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086ef33); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086ef3e); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086ef49); };
            c.w64(7, t13);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086ef52);
        }
        8 => {
            return RegfileExit::fault(0x700086ef53);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086ef74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086ef74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x700086ef78); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086ef7a); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086ef7c); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086ef83); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086ef8a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086ef99); }
            c.w64(12, c.r64(0));
            c.w64(1, c.r64(12));
            c.w64(2, c.r64(15));
            c.w64(8, c.r64(13));
            return c.call(c.r64(3), 0x700086efab, 0x700086efa9);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086efab); };
            c.cmp(c.r32(0), t2, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.leave() { return RegfileExit::fault(0x700086efb8); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086efb9); };
            c.w64(6, t3);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086efc0); }
            return c.ret(0x0, 0x700086efcb);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086efde); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086efe3); };
            c.w64(0, t6);
            c.w64(10, 0x700086eff1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086eff1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086eff5); }
            return c.call(c.r64(0), 0x700086effb, 0x700086eff9);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086effb); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f003); };
            c.w64(4, t7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f008); };
            c.w64(6, t8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f00f); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f016); };
            c.w64(5, t10);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f022); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f02d); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f038); };
            c.w64(7, t11);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086f041);
        }
        8 => {
            return RegfileExit::fault(0x700086f042);
        }
        9 => {
            return RegfileExit::fault(0x700086f043);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086f054); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x700086f058); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086f05a); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086f05f); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086f066); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086f06d); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0xb0u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x700086f080); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700086f085); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 2) { return RegfileExit::fault(0x700086f08a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 3) { return RegfileExit::fault(0x700086f08f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 4) { return RegfileExit::fault(0x700086f094); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 5) { return RegfileExit::fault(0x700086f099); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 6) { return RegfileExit::fault(0x700086f09e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 7) { return RegfileExit::fault(0x700086f0a3); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 8) { return RegfileExit::fault(0x700086f0a8); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 9) { return RegfileExit::fault(0x700086f0ae); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x68), 10) { return RegfileExit::fault(0x700086f0b4); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x70), 11) { return RegfileExit::fault(0x700086f0ba); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x78), 12) { return RegfileExit::fault(0x700086f0c0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 13) { return RegfileExit::fault(0x700086f0c6); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x88), 14) { return RegfileExit::fault(0x700086f0cc); }
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086f0d9); }
            c.w64(1, c.r64(12));
            c.w64(2, c.r64(15));
            c.w64(8, c.r64(13));
            return c.call(c.r64(3), 0x700086f0e8, 0x700086f0e6);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086f0e8); };
            c.cmp(c.r32(0), t2, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x700086f0f5); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700086f0fa); }
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700086f0ff); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700086f104); }
            if !c.movsd_rm(4, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700086f109); }
            if !c.movsd_rm(5, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700086f10e); }
            if !c.movsd_rm(6, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700086f113); }
            if !c.movsd_rm(7, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x700086f118); }
            if !c.movsd_rm(8, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700086f11d); }
            if !c.movsd_rm(9, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700086f123); }
            if !c.movsd_rm(10, c.r64(5).wrapping_sub(0x68)) { return RegfileExit::fault(0x700086f129); }
            if !c.movsd_rm(11, c.r64(5).wrapping_sub(0x70)) { return RegfileExit::fault(0x700086f12f); }
            if !c.movsd_rm(12, c.r64(5).wrapping_sub(0x78)) { return RegfileExit::fault(0x700086f135); }
            if !c.movsd_rm(13, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x700086f13b); }
            if !c.movsd_rm(14, c.r64(5).wrapping_sub(0x88)) { return RegfileExit::fault(0x700086f141); }
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086f14a); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086f14e); };
            c.w64(5, t4);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086f156); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086f157); };
            c.w64(6, t5);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f15e); }
            return c.ret(0x0, 0x700086f169);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086f17c); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086f181); };
            c.w64(0, t8);
            c.w64(10, 0x700086f18fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086f18f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086f193); }
            return c.call(c.r64(0), 0x700086f199, 0x700086f197);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086f199); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f1a1); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f1a6); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f1ad); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f1b4); };
            c.w64(5, t12);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f1c0); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f1cb); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f1d6); };
            c.w64(7, t13);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086f1df);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f1f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086f1f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x2au64) { return RegfileExit::fault(0x700086f1f8); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086f1fa); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086f1ff); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086f206); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086f20d); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0xb0u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x700086f220); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700086f225); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 2) { return RegfileExit::fault(0x700086f22a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 3) { return RegfileExit::fault(0x700086f22f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 4) { return RegfileExit::fault(0x700086f234); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 5) { return RegfileExit::fault(0x700086f239); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 6) { return RegfileExit::fault(0x700086f23e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 7) { return RegfileExit::fault(0x700086f243); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 8) { return RegfileExit::fault(0x700086f248); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 9) { return RegfileExit::fault(0x700086f24e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x68), 10) { return RegfileExit::fault(0x700086f254); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x70), 11) { return RegfileExit::fault(0x700086f25a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x78), 12) { return RegfileExit::fault(0x700086f260); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 13) { return RegfileExit::fault(0x700086f266); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x88), 14) { return RegfileExit::fault(0x700086f26c); }
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086f279); }
            c.w64(1, c.r64(12));
            c.w64(2, c.r64(15));
            c.w64(8, c.r64(13));
            return c.call(c.r64(3), 0x700086f288, 0x700086f286);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086f288); };
            c.cmp(c.r32(0), t2, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x700086f295); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700086f29a); }
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700086f29f); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700086f2a4); }
            if !c.movsd_rm(4, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700086f2a9); }
            if !c.movsd_rm(5, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700086f2ae); }
            if !c.movsd_rm(6, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700086f2b3); }
            if !c.movsd_rm(7, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x700086f2b8); }
            if !c.movsd_rm(8, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700086f2bd); }
            if !c.movsd_rm(9, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700086f2c3); }
            if !c.movsd_rm(10, c.r64(5).wrapping_sub(0x68)) { return RegfileExit::fault(0x700086f2c9); }
            if !c.movsd_rm(11, c.r64(5).wrapping_sub(0x70)) { return RegfileExit::fault(0x700086f2cf); }
            if !c.movsd_rm(12, c.r64(5).wrapping_sub(0x78)) { return RegfileExit::fault(0x700086f2d5); }
            if !c.movsd_rm(13, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x700086f2db); }
            if !c.movsd_rm(14, c.r64(5).wrapping_sub(0x88)) { return RegfileExit::fault(0x700086f2e1); }
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086f2ea); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086f2ee); };
            c.w64(5, t4);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086f2f6); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086f2f7); };
            c.w64(6, t5);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f2fe); }
            return c.ret(0x0, 0x700086f309);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086f31c); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086f321); };
            c.w64(0, t8);
            c.w64(10, 0x700086f32fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086f32f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086f333); }
            return c.call(c.r64(0), 0x700086f339, 0x700086f337);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086f339); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f341); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f346); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f34d); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f354); };
            c.w64(5, t12);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f360); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f36b); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f376); };
            c.w64(7, t13);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086f37f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086f394); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x700086f398); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086f39a); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086f39f); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086f3a6); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086f3ad); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086f3c1); }
            c.w64(1, c.r64(4).wrapping_add(0x30));
            c.w64(2, c.r64(12));
            c.w64(8, c.r64(15));
            c.w64(9, c.r64(13));
            return c.call(c.r64(3), 0x700086f3d5, 0x700086f3d3);
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086f3d5); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700086f3da); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086f3df); };
            c.cmp(c.r32(0), t4, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086f3ec); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086f3f0); };
            c.w64(5, t6);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086f3f8); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086f3f9); };
            c.w64(6, t7);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f400); }
            return c.ret(0x0, 0x700086f40b);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r8 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r8);
            let r9 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r9);
            c.logic(r9, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086f41e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086f423); };
            c.w64(0, t10);
            c.w64(10, 0x700086f431u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086f431); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086f435); }
            return c.call(c.r64(0), 0x700086f43b, 0x700086f439);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086f43b); }
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f443); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f448); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f44f); };
            c.w64(4, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f456); };
            c.w64(5, t14);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f462); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f46d); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f478); };
            c.w64(7, t15);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086f481);
        }
        8 => {
            return RegfileExit::fault(0x700086f482);
        }
        9 => {
            return RegfileExit::fault(0x700086f483);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086f494); }
            c.w64(5, c.r64(4));
            if !c.push64(0x2au64) { return RegfileExit::fault(0x700086f498); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086f49a); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086f49f); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086f4a6); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086f4ad); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086f4c1); }
            c.w64(1, c.r64(4).wrapping_add(0x30));
            c.w64(2, c.r64(12));
            c.w64(8, c.r64(15));
            c.w64(9, c.r64(13));
            return c.call(c.r64(3), 0x700086f4d5, 0x700086f4d3);
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086f4d5); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700086f4da); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086f4df); };
            c.cmp(c.r32(0), t4, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086f4ec); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086f4f0); };
            c.w64(5, t6);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086f4f8); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086f4f9); };
            c.w64(6, t7);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f500); }
            return c.ret(0x0, 0x700086f50b);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r8 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r8);
            let r9 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r9);
            c.logic(r9, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086f51e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086f523); };
            c.w64(0, t10);
            c.w64(10, 0x700086f531u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086f531); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086f535); }
            return c.call(c.r64(0), 0x700086f53b, 0x700086f539);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086f53b); }
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f543); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f548); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f54f); };
            c.w64(4, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f556); };
            c.w64(5, t14);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f562); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f56d); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f578); };
            c.w64(7, t15);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086f581);
        }
        8 => {
            return RegfileExit::fault(0x700086f582);
        }
        9 => {
            return RegfileExit::fault(0x700086f583);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086f594); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x700086f598); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086f59a); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086f59c); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086f5a3); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086f5aa); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086f5b9); }
            c.w64(12, c.r64(0));
            c.w64(1, c.r64(4).wrapping_add(0x30));
            c.w64(2, c.r64(12));
            c.w64(8, c.r64(15));
            c.w64(9, c.r64(13));
            return c.call(c.r64(3), 0x700086f5d0, 0x700086f5ce);
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086f5d0); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700086f5d5); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086f5da); };
            c.cmp(c.r32(0), t4, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.leave() { return RegfileExit::fault(0x700086f5e7); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086f5e8); };
            c.w64(6, t5);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f5ef); }
            return c.ret(0x0, 0x700086f5fa);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086f60d); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086f612); };
            c.w64(0, t8);
            c.w64(10, 0x700086f620u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086f620); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086f624); }
            return c.call(c.r64(0), 0x700086f62a, 0x700086f628);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086f62a); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f632); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f637); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f63e); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f645); };
            c.w64(5, t12);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f651); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f65c); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f667); };
            c.w64(7, t13);
            return RegfileExit::jump(c.r64(7));
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086f674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x700086f678); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086f67a); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086f67f); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086f686); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086f68d); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0xc0u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x700086f6a0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700086f6a5); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 2) { return RegfileExit::fault(0x700086f6aa); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 3) { return RegfileExit::fault(0x700086f6af); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 4) { return RegfileExit::fault(0x700086f6b4); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 5) { return RegfileExit::fault(0x700086f6b9); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 6) { return RegfileExit::fault(0x700086f6be); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 7) { return RegfileExit::fault(0x700086f6c3); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 8) { return RegfileExit::fault(0x700086f6c8); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 9) { return RegfileExit::fault(0x700086f6ce); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x68), 10) { return RegfileExit::fault(0x700086f6d4); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x70), 11) { return RegfileExit::fault(0x700086f6da); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x78), 12) { return RegfileExit::fault(0x700086f6e0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 13) { return RegfileExit::fault(0x700086f6e6); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x88), 14) { return RegfileExit::fault(0x700086f6ec); }
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086f6f9); }
            c.w64(1, c.r64(4).wrapping_add(0x30));
            c.w64(2, c.r64(12));
            c.w64(8, c.r64(15));
            c.w64(9, c.r64(13));
            return c.call(c.r64(3), 0x700086f70d, 0x700086f70b);
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086f70d); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700086f712); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086f717); };
            c.cmp(c.r32(0), t4, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x700086f724); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700086f729); }
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700086f72e); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700086f733); }
            if !c.movsd_rm(4, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700086f738); }
            if !c.movsd_rm(5, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700086f73d); }
            if !c.movsd_rm(6, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700086f742); }
            if !c.movsd_rm(7, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x700086f747); }
            if !c.movsd_rm(8, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700086f74c); }
            if !c.movsd_rm(9, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700086f752); }
            if !c.movsd_rm(10, c.r64(5).wrapping_sub(0x68)) { return RegfileExit::fault(0x700086f758); }
            if !c.movsd_rm(11, c.r64(5).wrapping_sub(0x70)) { return RegfileExit::fault(0x700086f75e); }
            if !c.movsd_rm(12, c.r64(5).wrapping_sub(0x78)) { return RegfileExit::fault(0x700086f764); }
            if !c.movsd_rm(13, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x700086f76a); }
            if !c.movsd_rm(14, c.r64(5).wrapping_sub(0x88)) { return RegfileExit::fault(0x700086f770); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086f779); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086f77d); };
            c.w64(5, t6);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086f785); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086f786); };
            c.w64(6, t7);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f78d); }
            return c.ret(0x0, 0x700086f798);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r8 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r8);
            let r9 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r9);
            c.logic(r9, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086f7ab); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086f7b0); };
            c.w64(0, t10);
            c.w64(10, 0x700086f7beu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086f7be); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086f7c2); }
            return c.call(c.r64(0), 0x700086f7c8, 0x700086f7c6);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086f7c8); }
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f7d0); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f7d5); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f7dc); };
            c.w64(4, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f7e3); };
            c.w64(5, t14);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f7ef); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f7fa); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f805); };
            c.w64(7, t15);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086f80e);
        }
        8 => {
            return RegfileExit::fault(0x700086f80f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086f814); }
            c.w64(5, c.r64(4));
            if !c.push64(0x2au64) { return RegfileExit::fault(0x700086f818); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x700086f81a); }
            c.w64(12, c.r64(0));
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x700086f81f); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700086f826); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x700086f82d); }
            c.w64(15, c.r64(5).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x8));
            let r0 = c.sub(c.r64(4), 0xc0u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x700086f840); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700086f845); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 2) { return RegfileExit::fault(0x700086f84a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 3) { return RegfileExit::fault(0x700086f84f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 4) { return RegfileExit::fault(0x700086f854); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 5) { return RegfileExit::fault(0x700086f859); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 6) { return RegfileExit::fault(0x700086f85e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 7) { return RegfileExit::fault(0x700086f863); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 8) { return RegfileExit::fault(0x700086f868); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 9) { return RegfileExit::fault(0x700086f86e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x68), 10) { return RegfileExit::fault(0x700086f874); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x70), 11) { return RegfileExit::fault(0x700086f87a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x78), 12) { return RegfileExit::fault(0x700086f880); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 13) { return RegfileExit::fault(0x700086f886); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x88), 14) { return RegfileExit::fault(0x700086f88c); }
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x700086f899); }
            c.w64(1, c.r64(4).wrapping_add(0x30));
            c.w64(2, c.r64(12));
            c.w64(8, c.r64(15));
            c.w64(9, c.r64(13));
            return c.call(c.r64(3), 0x700086f8ad, 0x700086f8ab);
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700086f8ad); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700086f8b2); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x700086f8b7); };
            c.cmp(c.r32(0), t4, 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x700086f8c4); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700086f8c9); }
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700086f8ce); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700086f8d3); }
            if !c.movsd_rm(4, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700086f8d8); }
            if !c.movsd_rm(5, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700086f8dd); }
            if !c.movsd_rm(6, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700086f8e2); }
            if !c.movsd_rm(7, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x700086f8e7); }
            if !c.movsd_rm(8, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700086f8ec); }
            if !c.movsd_rm(9, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700086f8f2); }
            if !c.movsd_rm(10, c.r64(5).wrapping_sub(0x68)) { return RegfileExit::fault(0x700086f8f8); }
            if !c.movsd_rm(11, c.r64(5).wrapping_sub(0x70)) { return RegfileExit::fault(0x700086f8fe); }
            if !c.movsd_rm(12, c.r64(5).wrapping_sub(0x78)) { return RegfileExit::fault(0x700086f904); }
            if !c.movsd_rm(13, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x700086f90a); }
            if !c.movsd_rm(14, c.r64(5).wrapping_sub(0x88)) { return RegfileExit::fault(0x700086f910); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086f919); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086f91d); };
            c.w64(5, t6);
            c.w64(4, c.r64(15).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086f925); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700086f926); };
            c.w64(6, t7);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f92d); }
            return c.ret(0x0, 0x700086f938);
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(13));
            c.w64(10, c.r64(4));
            let r8 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r8);
            let r9 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r9);
            c.logic(r9, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700086f94b); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2268)) else { return RegfileExit::fault(0x700086f950); };
            c.w64(0, t10);
            c.w64(10, 0x700086f95eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086f95e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700086f962); }
            return c.call(c.r64(0), 0x700086f968, 0x700086f966);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700086f968); }
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086f970); };
            c.w64(4, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x3000)) else { return RegfileExit::fault(0x700086f975); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x3020)) else { return RegfileExit::fault(0x700086f97c); };
            c.w64(4, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x3018)) else { return RegfileExit::fault(0x700086f983); };
            c.w64(5, t14);
            c.logic(c.r64(6) & c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x8), c.r64(6)) { return RegfileExit::fault(0x700086f98f); }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0xffffffffffffffffu64);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x700086f99a); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x3008)) else { return RegfileExit::fault(0x700086f9a5); };
            c.w64(7, t15);
            return RegfileExit::jump(c.r64(7));
        }
        7 => {
            return RegfileExit::fault(0x700086f9ae);
        }
        8 => {
            return RegfileExit::fault(0x700086f9af);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return RegfileExit::fault(0x700086f9b4);
        }
        1 => {
            return RegfileExit::fault(0x700086f9b5);
        }
        2 => {
            return RegfileExit::fault(0x700086f9b6);
        }
        3 => {
            return RegfileExit::fault(0x700086f9b7);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086f9d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086f9d4); };
            c.w32(1, t0);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086f9e0); };
            c.w32(7, t1);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(8, c.r64(1).wrapping_add(c.r64(7)));
            c.cmp(c.r32(8), 0x1fffffe8u64, 32);
            if c.cc_a() { blk = 77; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), 0xdu64, 32);
            if c.cc_b() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086fa03); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086fa09); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086fa0f); };
            c.w32(1, t6);
            let Some(t7) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086fa13); };
            c.w32(7, t7);
            let r8 = c.r32(7) & c.r32(1);
            c.w32(7, r8);
            c.logic(r8, 32);
            c.logic(c.r8(7) & 0x8u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x300)) else { return RegfileExit::fault(0x700086fa1f); };
            c.w64(1, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2f8)) else { return RegfileExit::fault(0x700086fa28); };
            c.w64(1, t10);
            blk = 7; continue;
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086fa2f); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086fa36); };
            c.w64(9, t12);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086fa3d); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086fa44); };
            c.cmp(t14, c.r64(11), 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700086fa4e); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700086fa55); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086fa58); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700086fa5c); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700086fa63); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x700086fa66); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700086fa6c);
        }
        10 => {
            c.w64(9, c.r64(3));
            c.w64(11, c.r64(0));
            blk = 11; continue;
        }
        11 => {
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086fa73); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086fa7a); };
            c.w32(15, t17);
            let r18 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r18);
            let Some(t19) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086fa81); };
            c.w32(12, t19);
            let Some(t20) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086fa87); };
            c.w32(15, t20);
            c.w64(2, c.r64(12));
            let r21 = c.r32(2) ^ c.r32(15);
            c.w32(2, r21);
            c.logic(r21, 32);
            c.logic(c.r8(2) & 0x8u64, 8);
            if c.cc_ne() { blk = 76; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(2, c.r64(12));
            let r22 = c.r32(2) | c.r32(15);
            c.w32(2, r22);
            c.logic(r22, 32);
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.logic(c.r8(2) & 0x8u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r23 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r23);
            if c.cc_o() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086fad3); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086fada); };
            c.w64(15, t25);
            let r26 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r26);
            let r27 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r27);
            c.logic(r27, 64);
            c.w64(0, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086fae9); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700086faf0); };
            c.cmp(t29, c.r64(0), 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 67; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086fb06); };
            c.w64(12, t30);
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700086fb0d); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700086fb15); };
            c.w64(15, t31);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086fb1c); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700086fb21); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086fb2a); }
            c.w64(0, c.r64(12));
            blk = 22; continue;
        }
        20 => {
            return RegfileExit::fault(0x700086fb34);
        }
        21 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700086fb35); };
            c.w64(0, t32);
            blk = 22; continue;
        }
        22 => {
            let r33 = c.add(c.r64(1), 0xbu64, 64);
            c.w64(1, r33);
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(8, 0xbu64);
            c.w64(12, c.r64(8));
            blk = 24; continue;
        }
        24 => {
            let Some(t34) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700086fb4f); };
            c.w32(15, t34);
            if !c.st8(c.r64(0).wrapping_add(c.r64(12)), c.r8(15)) { return RegfileExit::fault(0x700086fb54); }
            let r35 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r35);
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_ne() { blk = 24; continue; }
            blk = 25; continue;
        }
        25 => {
            blk = 27; continue;
        }
        26 => {
            c.w32(8, 0xbu64);
            blk = 27; continue;
        }
        27 => {
            let r36 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r36);
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 30; continue;
        }
        29 => {
            let r37 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r37);
            c.w64(8, c.r64(11));
            blk = 30; continue;
        }
        30 => {
            c.w64(11, c.r64(8).wrapping_add(0x1));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700086fb80); };
            c.w32(8, t38);
            if !c.st8(c.r64(0).wrapping_add(c.r64(1)), c.r8(8)) { return RegfileExit::fault(0x700086fb85); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 29; continue; }
            blk = 31; continue;
        }
        31 => {
            return c.ret(0x0, 0x700086fb8e);
        }
        32 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(12, c.r64(8));
            let r39 = c.add(c.r32(12), c.r32(8), 32);
            c.w32(12, r39);
            if c.cc_o() { blk = 40; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r40 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r40);
            if c.cc_o() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086fbaf); };
            c.w64(15, t41);
            let Some(t42) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086fbb6); };
            c.w64(15, t42);
            let r43 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r43);
            let r44 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r44);
            c.logic(r44, 64);
            c.w64(0, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086fbc5); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700086fbcc); };
            c.cmp(t46, c.r64(0), 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 71; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086fbe2); };
            c.w64(12, t47);
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700086fbe9); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 38; continue;
        }
        38 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700086fbf1); };
            c.w64(15, t48);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086fbf8); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700086fbfd); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086fc06); }
            c.w64(0, c.r64(12));
            blk = 42; continue;
        }
        39 => {
            return RegfileExit::fault(0x700086fc10);
        }
        40 => {
            return RegfileExit::fault(0x700086fc11);
        }
        41 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700086fc12); };
            c.w64(0, t49);
            blk = 42; continue;
        }
        42 => {
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(8, 0xbu64);
            c.w64(12, c.r64(8));
            blk = 44; continue;
        }
        44 => {
            let Some(t50) = c.ld16(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700086fc30); };
            c.w32(15, t50);
            if !c.st16(c.r64(0).wrapping_add(c.r64(12)), c.r16(15)) { return RegfileExit::fault(0x700086fc35); }
            let r51 = c.add(c.r64(12), 0x2u64, 64);
            c.w64(12, r51);
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_ne() { blk = 44; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 47; continue;
        }
        46 => {
            c.w32(8, 0xbu64);
            blk = 47; continue;
        }
        47 => {
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 31; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 50; continue;
        }
        49 => {
            let r52 = c.add(c.r64(1), 0x2u64, 64);
            c.w64(1, r52);
            c.w64(8, c.r64(11));
            blk = 50; continue;
        }
        50 => {
            c.w64(11, c.r64(8).wrapping_add(0x2));
            let Some(t53) = c.ld16(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700086fc6a); };
            c.w32(8, t53);
            if !c.st16(c.r64(0).wrapping_add(c.r64(1)), c.r16(8)) { return RegfileExit::fault(0x700086fc6f); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 49; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 31; continue;
        }
        52 => {
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x700086fc81);
        }
        53 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fc82); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fc86); }
            let r54 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r54);
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700086fc91); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700086fc95); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086fc99); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700086fc9d); }
            return c.call(0x70007febf4, 0x700086fca6, 0x700086fca1);
        }
        54 => {
            c.w64(7, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086fca9); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086fcad); };
            c.w64(0, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086fcb1); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086fcb5); };
            c.w64(1, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086fcbc); }
            blk = 9; continue;
        }
        55 => {
            let r59 = c.r32(15) & 0x7u64;
            c.w32(15, r59);
            c.logic(r59, 32);
            let r60 = c.r32(12) & 0x7u64;
            c.w32(12, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(15), 0x5u64, 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t61) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086fce0); };
            c.w32(12, t61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700086fce4); };
            c.cmp(t62, c.r32(12), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086fcfc); };
            c.w32(12, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700086fd00); };
            c.cmp(t64, c.r32(12), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086fd0d); };
            c.w32(9, t65);
            let r66 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r66);
            blk = 11; continue;
        }
        62 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086fd19); };
            c.w32(11, t67);
            let r68 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r68);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086fd2f); };
            c.w32(12, t69);
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700086fd33); };
            c.cmp(t70, c.r32(12), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086fd40); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            blk = 11; continue;
        }
        66 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fd4c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fd50); }
            let r73 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r73);
            blk = 68; continue;
        }
        67 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fd58); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fd5c); }
            let r74 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r74);
            blk = 68; continue;
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700086fd62); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700086fd66); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700086fd6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700086fd6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086fd72); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700086fd7e, 0x700086fd79);
        }
        69 => {
            c.w64(12, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086fd81); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086fd85); };
            c.w64(7, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086fd89); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086fd8d); };
            c.w64(1, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086fd91); };
            c.w64(8, t79);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086fd98); }
            blk = 19; continue;
        }
        70 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fd9e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fda2); }
            let r80 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r80);
            blk = 72; continue;
        }
        71 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fdaa); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fdae); }
            let r81 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r81);
            blk = 72; continue;
        }
        72 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700086fdb4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700086fdb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700086fdbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700086fdc0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086fdc4); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700086fdd0, 0x700086fdcb);
        }
        73 => {
            c.w64(12, c.r64(0));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086fdd3); };
            c.w64(9, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086fdd7); };
            c.w64(7, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086fddb); };
            c.w64(11, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086fddf); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086fde3); };
            c.w64(8, t86);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086fdea); }
            blk = 38; continue;
        }
        74 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fdf0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fdf4); }
            let r87 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r87);
            blk = 78; continue;
        }
        75 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fdfc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fe00); }
            let r88 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r88);
            blk = 78; continue;
        }
        76 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fe08); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fe0c); }
            let r89 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r89);
            blk = 78; continue;
        }
        77 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086fe14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086fe18); }
            let r90 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r90);
            blk = 78; continue;
        }
        78 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086fe1e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086fe1f); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2760)) else { return RegfileExit::fault(0x700086fe20); };
            c.w64(3, t91);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700086fe31, 0x700086fe2c);
        }
        79 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086fe34); }
            return c.ret(0x0, 0x700086fe35);
        }
        80 => {
            return RegfileExit::jump(0x700086fe38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086fe54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086fe5a); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086fe67); };
            c.w32(9, t2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086fe6b); };
            c.w32(8, t3);
            c.w64(11, c.r64(1));
            let r4 = c.sub(c.r64(11), c.r64(7), 64);
            c.w64(11, r4);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 117; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 116; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(8) & 0x7u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r64(11), 0x1u64, 64);
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
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(0));
            blk = 5; continue;
        }
        5 => {
            c.w64(12, c.r64(9));
            let r5 = c.r32(12) & 0x7u64;
            c.w32(12, r5);
            c.logic(r5, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 17; continue;
        }
        10 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 17; continue;
        }
        13 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086fee7); };
            c.w32(9, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700086feeb); };
            c.cmp(t7, c.r32(9), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086fef4); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086fefb); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086ff02); };
            c.w32(9, t12);
            blk = 5; continue;
        }
        15 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086ff09); };
            c.w32(9, t13);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r14 = c.add(c.r64(6), c.r64(9), 64);
            c.w64(6, r14);
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086ff16); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ff1d); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086ff24); };
            c.w32(9, t19);
            blk = 5; continue;
        }
        16 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086ff2b); };
            c.w32(8, t20);
            let r21 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r21);
            let Some(t22) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ff32); };
            c.w32(9, t22);
            let r23 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r23);
            let Some(t24) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086ff39); };
            c.w32(9, t24);
            blk = 5; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086ff43); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086ff47); }
            let r25 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            blk = 119; continue;
        }
        18 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 22; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086ff60); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086ff64); }
            let r26 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            blk = 119; continue;
        }
        20 => {
            c.w32(12, 0x1u64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(11), 0xdu64, 64);
            if c.cc_l() { blk = 30; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(6)));
            let r27 = c.r32(8) & 0x8u64;
            c.w32(8, r27);
            c.logic(r27, 32);
            let r28 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r28);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ffa2); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086ffa9); };
            c.w64(8, t30);
            c.w64(12, c.r64(8).wrapping_add(0x14));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ffb0); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086ffb7); };
            c.cmp(t32, c.r64(12), 64);
            if c.cc_be() { blk = 121; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x700086ffc0); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x700086ffc7); };
            c.w64(8, t33);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086ffce); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700086ffd2); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700086ffd9); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086ffdd); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x700086ffe1); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700086ffe7);
        }
        27 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ffe8); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086ffef); };
            c.w64(8, t35);
            c.w64(15, c.r64(8).wrapping_add(0x14));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086fff6); };
            c.w64(12, t36);
            let Some(t37) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086fffd); };
            c.cmp(t37, c.r64(15), 64);
            if c.cc_be() { blk = 123; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x7000870007); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x700087000e); };
            c.w64(8, t38);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000870015); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000870019); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000870020); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000870024); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x7000870028); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700087002e);
        }
        30 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(1, c.r32(11));
            let r39 = c.r32(8) & 0x8u64;
            c.w32(8, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(1, c.r32(11));
            c.w64(8, c.r64(1));
            let r40 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r40);
            if c.cc_o() { blk = 40; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(8, 0xcu64);
            let r41 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r41);
            if c.cc_o() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000870075); };
            c.w64(8, t42);
            let Some(t43) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087007c); };
            c.w64(8, t43);
            let r44 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r44);
            let r45 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r45);
            c.logic(r45, 64);
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087008b); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000870092); };
            c.cmp(t47, c.r64(12), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 126; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008700a8); };
            c.w64(1, t48);
            if !c.st64(c.r64(1), c.r64(12)) { return RegfileExit::fault(0x70008700af); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 38; continue;
        }
        38 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008700b6); };
            c.w64(8, t49);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008700bd); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008700c1); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008700c8); }
            c.w64(0, c.r64(1));
            blk = 42; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008700d1);
        }
        40 => {
            return RegfileExit::fault(0x70008700d2);
        }
        41 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008700d3); };
            c.w64(0, t50);
            blk = 42; continue;
        }
        42 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(6)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(11).wrapping_mul(2)));
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
            let Some(t52) = c.ld16(c.r64(9).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x7000870102); };
            c.w32(1, t52);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x7000870107); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            return c.ret(0x0, 0x7000870111);
        }
        47 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(1, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(8, 0xcu64);
            let r53 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r53);
            if c.cc_o() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000870125); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087012c); };
            c.w64(8, t55);
            let r56 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r56);
            let r57 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r57);
            c.logic(r57, 64);
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087013b); };
            c.w64(15, t58);
            let Some(t59) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000870142); };
            c.cmp(t59, c.r64(12), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 130; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000870158); };
            c.w64(1, t60);
            if !c.st64(c.r64(1), c.r64(12)) { return RegfileExit::fault(0x700087015f); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 52; continue;
        }
        52 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000870166); };
            c.w64(8, t61);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087016d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000870171); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000870178); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        53 => {
            return RegfileExit::fault(0x7000870181);
        }
        54 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000870182); };
            c.w64(0, t62);
            blk = 55; continue;
        }
        55 => {
            c.w64(1, c.r64(6).wrapping_add(c.r64(7)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(11)));
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
            let Some(t64) = c.ld8(c.r64(9).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008701ae); };
            c.w32(1, t64);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70008701b3); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 57; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 46; continue;
        }
        60 => {
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 90; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w32(1, c.r32(11));
            let Some(t65) = c.ld64(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008701ce); };
            c.w64(9, t65);
            let r66 = c.r32(8) & 0x8u64;
            c.w32(8, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(1, c.r32(11));
            c.w64(8, c.r64(1));
            let r67 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r67);
            if c.cc_o() { blk = 70; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(8, 0xcu64);
            let r68 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r68);
            if c.cc_o() { blk = 69; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087020b); };
            c.w64(8, t69);
            let Some(t70) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000870212); };
            c.w64(8, t70);
            let r71 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r71);
            let r72 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r72);
            c.logic(r72, 64);
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000870221); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000870228); };
            c.cmp(t74, c.r64(12), 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 134; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087023e); };
            c.w64(1, t75);
            if !c.st64(c.r64(1), c.r64(12)) { return RegfileExit::fault(0x7000870245); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 68; continue;
        }
        68 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700087024c); };
            c.w64(8, t76);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000870253); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000870257); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700087025e); }
            c.w64(0, c.r64(1));
            blk = 72; continue;
        }
        69 => {
            return RegfileExit::fault(0x7000870267);
        }
        70 => {
            return RegfileExit::fault(0x7000870268);
        }
        71 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000870269); };
            c.w64(0, t77);
            blk = 72; continue;
        }
        72 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(6)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(11).wrapping_mul(2)));
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
            let Some(t79) = c.ld16(c.r64(1).wrapping_add(c.r64(9)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x700087029c); };
            c.w32(1, t79);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70008702a2); }
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
            c.w64(1, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(8, 0xcu64);
            let r80 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r80);
            if c.cc_o() { blk = 83; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008702c4); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008702cb); };
            c.w64(8, t82);
            let r83 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r83);
            let r84 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r84);
            c.logic(r84, 64);
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008702da); };
            c.w64(15, t85);
            let Some(t86) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008702e1); };
            c.cmp(t86, c.r64(12), 64);
            if c.cc_be() { blk = 137; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 138; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008702f7); };
            c.w64(1, t87);
            if !c.st64(c.r64(1), c.r64(12)) { return RegfileExit::fault(0x70008702fe); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 82; continue;
        }
        82 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000870305); };
            c.w64(8, t88);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087030c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000870310); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000870317); }
            c.w64(0, c.r64(1));
            blk = 85; continue;
        }
        83 => {
            return RegfileExit::fault(0x7000870320);
        }
        84 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000870321); };
            c.w64(0, t89);
            blk = 85; continue;
        }
        85 => {
            c.w64(1, c.r64(6).wrapping_add(c.r64(7)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(11)));
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
            let Some(t91) = c.ld8(c.r64(1).wrapping_add(c.r64(9)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x700087034d); };
            c.w32(1, t91);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x7000870353); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 87; continue; }
            blk = 89; continue;
        }
        89 => {
            blk = 46; continue;
        }
        90 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870361); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870365); }
            let r92 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r92);
            blk = 119; continue;
        }
        91 => {
            c.w32(1, c.r16(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(0));
            blk = 93; continue;
        }
        93 => {
            c.w64(9, c.r64(8));
            let r93 = c.r32(9) & 0x7u64;
            c.w32(9, r93);
            c.logic(r93, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 98; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 97; continue;
        }
        97 => {
            blk = 141; continue;
        }
        98 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            blk = 141; continue;
        }
        101 => {
            let Some(t94) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008703bf); };
            c.w32(8, t94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008703c3); };
            c.cmp(t95, c.r32(8), 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t96) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008703d0); };
            c.w32(1, t96);
            let r97 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r97);
            let Some(t98) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008703d6); };
            c.w32(8, t98);
            let r99 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r99);
            let Some(t100) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008703dd); };
            c.w32(8, t100);
            blk = 93; continue;
        }
        103 => {
            let Some(t101) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008703e4); };
            c.w32(8, t101);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r102 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r102);
            let Some(t103) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008703f1); };
            c.w32(1, t103);
            let r104 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r104);
            let Some(t105) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008703f7); };
            c.w32(8, t105);
            let r106 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r106);
            let Some(t107) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008703fe); };
            c.w32(8, t107);
            blk = 93; continue;
        }
        104 => {
            let Some(t108) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870405); };
            c.w32(1, t108);
            let r109 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r109);
            let Some(t110) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087040b); };
            c.w32(8, t110);
            let r111 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r111);
            let Some(t112) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870412); };
            c.w32(8, t112);
            blk = 93; continue;
        }
        105 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 108; continue;
        }
        106 => {
            c.w32(12, 0x1u64);
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 108; continue;
        }
        107 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 109; continue;
        }
        109 => {
            let r113 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r113);
            blk = 110; continue;
        }
        110 => {
            let r114 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r114);
            let r115 = c.r32(1) & 0x8u64;
            c.w32(1, r115);
            c.logic(r115, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t116) = c.ld16(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x700087045c); };
            c.w32(1, t116);
            blk = 113; continue;
        }
        112 => {
            let Some(t117) = c.ld8(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000870463); };
            c.w32(1, t117);
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 158; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700087047c); };
            c.w64(8, t118);
            let Some(t119) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870483); };
            c.w32(0, t119);
            let r120 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087048b); };
            c.cmp(t121, c.r32(0), 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 46; continue;
        }
        116 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087049d); };
            c.w64(0, t122);
            return c.ret(0x0, 0x70008704a4);
        }
        117 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 118; continue;
        }
        118 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008704af); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008704b3); }
            let r123 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r123);
            blk = 119; continue;
        }
        119 => {
            let r124 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r124);
            let r125 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r125);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008704bd); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008704be); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008704bf); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008704c0); };
            c.w64(3, t126);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008704d3, 0x70008704ce);
        }
        120 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008704d6); }
            return c.ret(0x0, 0x70008704d7);
        }
        121 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008704d8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008704dc); }
            let r127 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r127);
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008704e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008704eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008704ef); }
            return c.call(0x70007febf4, 0x70008704f8, 0x70008704f3);
        }
        122 => {
            c.w64(7, c.r64(0));
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008704fb); };
            c.w64(11, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008704ff); };
            c.w64(9, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000870503); };
            c.w64(1, t130);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087050a); }
            blk = 26; continue;
        }
        123 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870510); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870514); }
            let r131 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r131);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700087051a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700087051e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000870522); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x7000870530, 0x700087052b);
        }
        124 => {
            c.w64(7, c.r64(0));
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000870533); };
            c.w64(11, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000870537); };
            c.w64(9, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087053b); };
            c.w64(1, t134);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000870542); }
            blk = 29; continue;
        }
        125 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870548); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087054c); }
            let r135 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r135);
            blk = 127; continue;
        }
        126 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870554); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870558); }
            let r136 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r136);
            blk = 127; continue;
        }
        127 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700087055e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x7000870562); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000870566); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700087056a); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000870576, 0x7000870571);
        }
        128 => {
            c.w64(1, c.r64(0));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000870579); };
            c.w64(7, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087057d); };
            c.w64(11, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000870581); };
            c.w64(6, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000870585); };
            c.w64(9, t140);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087058c); }
            blk = 38; continue;
        }
        129 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870592); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870596); }
            let r141 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r141);
            blk = 131; continue;
        }
        130 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087059e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008705a2); }
            let r142 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r142);
            blk = 131; continue;
        }
        131 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008705a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008705ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008705b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008705b4); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008705c0, 0x70008705bb);
        }
        132 => {
            c.w64(1, c.r64(0));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008705c3); };
            c.w64(7, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008705c7); };
            c.w64(11, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008705cb); };
            c.w64(6, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008705cf); };
            c.w64(9, t146);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008705d6); }
            blk = 52; continue;
        }
        133 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008705dc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008705e0); }
            let r147 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r147);
            blk = 135; continue;
        }
        134 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008705e8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008705ec); }
            let r148 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r148);
            blk = 135; continue;
        }
        135 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008705f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008705f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008705fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008705fe); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x700087060a, 0x7000870605);
        }
        136 => {
            c.w64(1, c.r64(0));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087060d); };
            c.w64(7, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000870611); };
            c.w64(9, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000870615); };
            c.w64(11, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000870619); };
            c.w64(6, t152);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000870620); }
            blk = 68; continue;
        }
        137 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870626); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087062a); }
            let r153 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r153);
            blk = 139; continue;
        }
        138 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870632); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870636); }
            let r154 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r154);
            blk = 139; continue;
        }
        139 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700087063c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000870640); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x7000870644); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000870648); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000870654, 0x700087064f);
        }
        140 => {
            c.w64(1, c.r64(0));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000870657); };
            c.w64(7, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087065b); };
            c.w64(9, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087065f); };
            c.w64(11, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000870663); };
            c.w64(6, t158);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087066a); }
            blk = 82; continue;
        }
        141 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870670); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870674); }
            let r159 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r159);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700087067a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700087067e); }
            blk = 145; continue;
        }
        142 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870684); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870688); }
            let r160 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r160);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700087068e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000870692); }
            blk = 145; continue;
        }
        143 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870698); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087069c); }
            let r161 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r161);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008706a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008706a6); }
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t162) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008706af); };
            c.w64(8, t162);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008706b6); }
            blk = 110; continue;
        }
        145 => {
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w32(1, c.r64(7).wrapping_add(c.r64(7)));
            blk = 156; continue;
        }
        147 => {
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008706ca); };
            c.w64(1, t163);
            let Some(t164) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008706d1); };
            c.w64(8, t164);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008706d8); };
            c.w64(11, t165);
            let Some(t166) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008706df); };
            c.cmp(t166, c.r64(9), 64);
            if c.cc_a() { blk = 150; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008706ee, 0x70008706e9);
        }
        149 => {
            c.w64(1, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008706f1); };
            c.w64(0, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008706f5); };
            c.w64(7, t168);
            blk = 151; continue;
        }
        150 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008706fb); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 151; continue;
        }
        151 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 155; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let r169 = c.r64(10) | 0x1u64;
            c.w64(10, r169);
            c.logic(r169, 64);
            blk = 154; continue;
        }
        154 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 155; continue;
        }
        155 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000870729); };
            c.w64(7, t170);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000870730); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000870733); }
            blk = 156; continue;
        }
        156 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000870738); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000870739); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700087073a); };
            c.w64(3, t171);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087074d, 0x7000870748);
        }
        157 => {
            let r172 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r172);
            c.w64(1, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000870755); }
            blk = 113; continue;
        }
        158 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087075b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087075f); }
            let r173 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r173);
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000870765); };
            c.w64(7, t174);
            let Some(t175) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087076c); };
            c.w64(8, t175);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000870773); }
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000870777); };
            c.w64(11, t176);
            let Some(t177) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700087077e); };
            c.cmp(t177, c.r64(9), 64);
            if c.cc_a() { blk = 161; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700087078d, 0x7000870788);
        }
        160 => {
            blk = 162; continue;
        }
        161 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700087078f); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 162; continue;
        }
        162 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000870799); };
            c.w64(1, t178);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008707a0); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008707a3); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008707aa); }
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008707b1); };
            c.w64(1, t179);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70008707b5); }
            blk = 120; continue;
        }
        163 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008707be); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008707c2); }
            let r180 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r180);
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008707c8); };
            c.w64(9, t181);
            let Some(t182) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008707cf); };
            c.w64(11, t182);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008707d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008707da); }
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008707de); };
            c.w64(15, t183);
            let Some(t184) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008707e5); };
            c.cmp(t184, c.r64(12), 64);
            if c.cc_a() { blk = 166; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008707f4, 0x70008707ef);
        }
        165 => {
            blk = 167; continue;
        }
        166 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008707f6); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 167; continue;
        }
        167 => {
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000870800); };
            c.w64(1, t185);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000870807); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700087080a); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000870811); }
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000870818); };
            c.w64(1, t186);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x700087081c); }
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700087081f); };
            c.w64(8, t187);
            let Some(t188) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870826); };
            c.w32(7, t188);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            return RegfileExit::fault(0x7000870835);
        }
        169 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000870836); };
            c.w64(7, t189);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700087083a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r190 = c.r64(1) & c.r64(8);
            c.w64(1, r190);
            c.logic(r190, 64);
            let Some(t191) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087084c); };
            c.logic(t191 & 0x4u64, 8);
            if c.cc_ne() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            blk = 120; continue;
        }
        172 => {
            c.w32(0, c.r32(0));
            let r192 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r192);
            c.w64(1, 0xfffffffffffc0000u64);
            let r193 = c.r64(1) & c.r64(0);
            c.w64(1, r193);
            c.logic(r193, 64);
            let Some(t194) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000870866); };
            c.logic(t194 & 0x2u64, 8);
            if c.cc_e() { blk = 171; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000870870); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000870871); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700087087e, 0x7000870879);
        }
        174 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087087e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000870880); }
            blk = 171; continue;
        }
        175 => {
            return RegfileExit::jump(0x7000870884);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000870894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000870896); };
            c.w64(3, t0);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::fault(0x70008708a2);
        }
        2 => {
            return RegfileExit::fault(0x70008708a3);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008708b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008708b4); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008708b5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008708b6); }
            let Some(t0) = c.ld32(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008708b7); };
            c.w32(3, t0);
            if !c.movsd_rm(15, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70008708bb); }
            let Some(t1) = c.ld32(c.r64(4).wrapping_add(0x24)) else { return RegfileExit::fault(0x70008708c2); };
            c.w32(1, t1);
            let r2 = c.r32(1) & 0x7ff00000u64;
            c.w32(1, r2);
            c.logic(r2, 32);
            let r3 = c.shr(c.r32(1), 0x14u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r3);
            c.w32(0, c.r64(1).wrapping_sub(0x3ff));
            c.cmp(c.r32(0), 0x34u64, 32);
            if c.cc_b() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let r4 = c.sub(c.r32(1), 0x433u64, 32);
            c.w32(1, r4);
            c.w32(0, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), 0x1fu64, 32);
            if c.cc_a() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let r5 = c.shl(c.r32(3), c.r8(1), 32).unwrap_or(c.r32(3));
            c.w32(3, r5);
            blk = 4; continue;
        }
        3 => {
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(0, cv);
            blk = 5; continue;
        }
        4 => {
            c.w32(0, c.r32(3));
            let r6 = c.neg(c.r32(0), 32);
            c.w32(0, r6);
            let Some(t7) = c.ld32(c.r64(4).wrapping_add(0x24)) else { return RegfileExit::fault(0x70008708f6); };
            c.cmp(t7, 0x0u64, 32);
            if c.cc_g() { c.w32(0, c.r32(3)); } else { let cur = c.r32(0); c.w32(0, cur); }
            blk = 5; continue;
        }
        5 => {
            if !c.st32(c.r64(4).wrapping_add(0x20), c.r32(0)) { return RegfileExit::fault(0x70008708fe); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x7000870902); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000870903); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x7000870904); }
            return c.ret(0x0, 0x7000870905);
        }
        6 => {
            return RegfileExit::fault(0x7000870906);
        }
        7 => {
            return RegfileExit::fault(0x7000870907);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000870914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000870914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000870918); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700087091e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000870922); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000870926); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870932); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870939); };
            c.w32(9, t3);
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_b() { blk = 151; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(9), 0xbcu64, 16);
            if c.cc_e() { blk = 169; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087095f); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870966); };
            c.w32(11, t6);
            c.cmp(c.r16(11), 0x42u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000870972); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(12) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(15, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(15), c.r64(12), 64);
            c.w64(15, r7);
            c.w64(12, 0x3ffffffffffffeu64);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 151; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_ae() { blk = 153; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008709bf); };
            c.w32(12, t8);
            c.logic(c.r32(12) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(11), 0x25u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(11), 0x2du64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(11) & 0x20u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, c.r64(3));
            blk = 18; continue;
        }
        16 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008709f7); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            blk = 18; continue;
        }
        17 => {
            c.w64(11, c.r64(3));
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000870a03); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(0));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000870a0d); };
            c.w64(2, t11);
            blk = 20; continue;
        }
        19 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870a16); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870a1d); };
            c.w32(12, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000870a22); }
            blk = 20; continue;
        }
        20 => {
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000870a26); };
            c.cmp(t15, 0x410u64, 32);
            if c.cc_le() { blk = 49; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870a33); };
            c.w32(0, t16);
            c.logic(c.r32(0) & 0x200000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000870a42); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            c.logic(c.r32(0) & 0xffc00u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 23; continue;
        }
        23 => {
            let r19 = c.shr(c.r32(0), 0xau64, 32).unwrap_or(c.r32(0));
            c.w32(0, r19);
            let r20 = c.r32(0) & 0x3ffu64;
            c.w32(0, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(0), 0x20u64, 32);
            if c.cc_be() { blk = 38; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(3).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000870a65); };
            c.w32(6, (t21 as u16 as i16 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870a69); };
            c.w32(7, t22);
            let r23 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r23);
            let r24 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000870a73); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000870a77); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000870a80); }
            blk = 26; continue;
        }
        26 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000870a84); };
            c.w64(9, t25);
            let r26 = c.sub(c.r32(9), c.r32(15), 32);
            c.w32(9, r26);
            let r27 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r27);
            let r28 = c.add(c.r32(9), c.r32(15), 32);
            c.w32(9, r28);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000870a98); };
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
            let Some(t33) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870ab2); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870aba); };
            c.w32(12, t35);
            let r36 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r36);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_be() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(15, c.r64(9).wrapping_add(0x1));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000870acc); };
            c.w64(9, t37);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 29; continue;
        }
        29 => {
            blk = 31; continue;
        }
        30 => {
            let r38 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r38);
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_l() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(9, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000870ae7); };
            c.w32(9, t39);
            let r40 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r40);
            let r41 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r41);
            let r42 = c.r32(9) & 0x3ffu64;
            c.w32(9, r42);
            c.logic(r42, 32);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870b01); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000870b09); }
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870b0d); };
            c.w32(12, t45);
            let r46 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r46);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_a() { blk = 93; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000870b28); };
            c.w64(9, t47);
            blk = 60; continue;
        }
        36 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000870b31); };
            c.w64(9, t48);
            blk = 60; continue;
        }
        37 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000870b3a); };
            c.w64(9, t49);
            blk = 60; continue;
        }
        38 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            let r50 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r50);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 39; continue;
        }
        39 => {
            let r51 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r51);
            let Some(t52) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870b58); };
            c.cmp(t52, c.r32(11), 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 39; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 60; continue;
        }
        42 => {
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870b6e); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(12, c.r64(2));
            blk = 44; continue;
        }
        44 => {
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870b7e); };
            c.w32(15, t55);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r56 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r56);
            let Some(t57) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870b8d); };
            c.w32(0, t57);
            let r58 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            let r59 = c.r64(0) & c.r64(15);
            c.w64(0, r59);
            c.logic(r59, 64);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        45 => {
            let r60 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r60);
            let r61 = c.add(c.r64(0), c.r64(1), 64);
            c.w64(0, r61);
            c.w64(3, c.r64(15));
            let r62 = c.r64(3) & c.r64(0);
            c.w64(3, r62);
            c.logic(r62, 64);
            c.w64(0, c.r64(3));
            blk = 46; continue;
        }
        46 => {
            c.w64(3, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(3).wrapping_mul(4));
            let Some(t63) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870bb8); };
            c.w32(7, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000870bbd); };
            c.cmp(t64, c.r32(7), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 45; continue;
        }
        49 => {
            let Some(t65) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000870bd5); };
            c.cmp(t65, 0xbdu64, 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t66) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000870be2); };
            c.logic(t66 & 0x24u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870bed); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(12, c.r64(2));
            blk = 53; continue;
        }
        53 => {
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870bfd); };
            c.w32(15, t69);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r70 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r70);
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870c0c); };
            c.w32(0, t71);
            let r72 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r72);
            let r73 = c.r64(0) & c.r64(15);
            c.w64(0, r73);
            c.logic(r73, 64);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 55; continue;
        }
        54 => {
            let r74 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r74);
            let r75 = c.add(c.r64(0), c.r64(1), 64);
            c.w64(0, r75);
            c.w64(3, c.r64(15));
            let r76 = c.r64(3) & c.r64(0);
            c.w64(3, r76);
            c.logic(r76, 64);
            c.w64(0, c.r64(3));
            blk = 55; continue;
        }
        55 => {
            c.w64(3, c.r64(0).wrapping_add(0x5));
            let Some(t77) = c.ld32(c.r64(12).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870c2e); };
            c.w32(6, t77);
            let r78 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r78);
            let Some(t79) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000870c36); };
            c.cmp(t79, c.r32(6), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000870c3f); };
            c.cmp(t80, c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t81) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870c48); };
            c.cmp(t81, c.r32(11), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870c4e); };
            c.w32(12, t82);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_ae() { blk = 165; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t83) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870c62); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000870c69); };
            c.cmp(t85, c.r32(12), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t86) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000870c76); };
            c.cmp(t86, 0x82eu64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t87) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870c83); };
            c.w32(8, t87);
            let r88 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r88);
            let Some(t89) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870c8a); };
            c.w32(8, t89);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 91; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t90) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870c9a); };
            c.w32(12, t90);
            let r91 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r91);
            c.w32(15, c.r32(12));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(12), 0x30u64, 32);
            if c.cc_g() { blk = 91; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(12, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(8, c.r64(11));
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 66; continue;
        }
        66 => {
            c.w64(15, c.r64(12));
            let r92 = c.r32(15) & 0x7u64;
            c.w32(15, r92);
            c.logic(r92, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 155; continue;
        }
        71 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            blk = 155; continue;
        }
        74 => {
            let Some(t93) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870d04); };
            c.w32(12, t93);
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000870d08); };
            c.cmp(t94, c.r32(12), 32);
            if c.cc_ne() { blk = 156; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t95) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870d15); };
            c.w32(8, t95);
            let r96 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r96);
            let Some(t97) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870d1c); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r98);
            let Some(t99) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870d23); };
            c.w32(12, t99);
            blk = 66; continue;
        }
        76 => {
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870d2b); };
            c.w32(12, t100);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r101 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r101);
            let Some(t102) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870d38); };
            c.w32(8, t102);
            let r103 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r103);
            let Some(t104) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870d3f); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870d46); };
            c.w32(12, t106);
            blk = 66; continue;
        }
        77 => {
            let Some(t107) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870d51); };
            c.w32(8, t107);
            let r108 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r108);
            let Some(t109) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870d58); };
            c.w32(12, t109);
            let r110 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r110);
            let Some(t111) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870d5f); };
            c.w32(12, t111);
            blk = 66; continue;
        }
        78 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 81; continue;
        }
        79 => {
            c.w32(15, 0x1u64);
            blk = 81; continue;
        }
        80 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(11));
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 82; continue;
        }
        82 => {
            let r112 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r112);
            blk = 83; continue;
        }
        83 => {
            let r113 = c.r32(12) & 0x8u64;
            c.w32(12, r113);
            c.logic(r113, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t114) = c.ld16(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x7000870d97); };
            c.w32(8, t114);
            blk = 86; continue;
        }
        85 => {
            let Some(t115) = c.ld8(c.r64(8).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x7000870d9e); };
            c.w32(8, t115);
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 151; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870dd1); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000870dd8); };
            c.cmp(t118, c.r32(8), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 19; continue;
        }
        93 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000870dea); };
            c.w64(12, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000870dee); };
            c.w64(9, t120);
            blk = 94; continue;
        }
        94 => {
            if let Some(sr) = c.shl(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let Some(t121) = c.ld32(c.r64(3).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000870df6); };
            c.w32(15, t121);
            c.w32(0, c.r32(15));
            c.w64(2, c.r64(15));
            let r122 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r122);
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t123) = c.ld32(c.r64(3).wrapping_add(c.r64(12)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000870e09); };
            c.w32(12, t123);
            let r124 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r124);
            blk = 110; continue;
        }
        96 => {
            c.w64(12, c.r64(2));
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t125) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000870e1d); };
            c.w32(15, t125);
            let r126 = c.r64(12) & 0x3ffu64;
            c.w64(12, r126);
            c.logic(r126, 64);
            let r127 = c.r32(0) & 0x380u64;
            c.w32(0, r127);
            c.logic(r127, 32);
            let r128 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r128);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            let Some(t129) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870e3e); };
            c.w32(9, t129);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 101; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870e48); };
            c.w32(0, t130);
            let r131 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r131);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000870e53); };
            c.w64(0, t132);
            blk = 99; continue;
        }
        99 => {
            let r133 = c.sub(c.r64(12), c.r64(9), 64);
            c.w64(12, r133);
            let Some(t134) = c.ld32(c.r64(0).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870e5d); };
            c.w32(12, t134);
            let r135 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r135);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000870e6f); }
            blk = 103; continue;
        }
        101 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870e7e); };
            c.w32(9, t136);
            let r137 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r137);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000870e86); }
            blk = 103; continue;
        }
        103 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000870e8c); };
            c.w64(9, t138);
            let Some(t139) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000870e93); };
            c.w64(12, t139);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000870e9b); };
            c.w64(0, t140);
            let Some(t141) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000870ea2); };
            c.cmp(t141, c.r64(15), 64);
            if c.cc_be() { blk = 161; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000870eab); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 105; continue;
        }
        105 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000870eb3); };
            c.w64(12, t142);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000870eba); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000870ebe); }
            c.w64(12, c.r64(9));
            blk = 110; continue;
        }
        106 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870ec9); };
            c.w32(12, t143);
            let r144 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r144);
            blk = 110; continue;
        }
        107 => {
            let Some(t145) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870ed3); };
            c.w32(9, t145);
            let Some(t146) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870ed8); };
            c.w32(15, t146);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_ae() { blk = 163; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t147) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870ef0); };
            c.w32(12, t147);
            let r148 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r148);
            let r149 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r149);
            c.w64(2, c.r64(15));
            blk = 110; continue;
        }
        109 => {
            let Some(t150) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870f00); };
            c.w32(9, t150);
            let r151 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r151);
            c.w64(2, c.r64(9));
            blk = 110; continue;
        }
        110 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 149; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t152) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870f13); };
            c.w32(9, t152);
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000870f18); };
            c.cmp(t153, c.r32(9), 32);
            if c.cc_ne() { blk = 132; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t154) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000870f25); };
            c.w32(2, t154);
            let r155 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r155);
            let Some(t156) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870f2d); };
            c.w32(9, t156);
            let r157 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r157);
            let Some(t158) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000870f34); };
            c.logic(t158 & 0x2u64, 8);
            if c.cc_ne() { blk = 130; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000870f3f); };
            c.cmp(t159, c.r32(9), 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000870f48); };
            c.w64(0, t160);
            blk = 115; continue;
        }
        115 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000870f52); }
            return c.ret(0x0, 0x7000870f53);
        }
        116 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000870f54); };
            c.w64(9, t161);
            let Some(t162) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x7000870f5b); };
            c.cmp(t162, 0x20u64, 8);
            if c.cc_e() { blk = 167; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t163) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000870f65); };
            c.w32(9, t163);
            let Some(t164) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000870f69); };
            c.cmp(t164, c.r32(9), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t165) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870f76); };
            c.w32(9, t165);
            let r166 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r166);
            let Some(t167) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870f7d); };
            c.w32(11, t167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000870f81); };
            c.cmp(t168, c.r32(11), 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t169) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000870f8a); };
            c.w32(9, t169);
            let r170 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r170);
            let Some(t171) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870f91); };
            c.w32(11, t171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000870f95); };
            c.cmp(t172, c.r32(11), 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 120; continue;
        }
        120 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t173) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870fa8); };
            c.w32(11, t173);
            let r174 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r174);
            let Some(t175) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870faf); };
            c.w32(12, t175);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 126; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000870fcf); };
            c.cmp(t176, c.r32(11), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 127; continue;
        }
        125 => {
            let Some(t177) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000870fde); };
            c.w32(8, t177);
            let r178 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r178);
            blk = 127; continue;
        }
        126 => {
            c.w64(8, c.r64(9));
            blk = 127; continue;
        }
        127 => {
            let Some(t179) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000870fea); };
            c.w32(8, t179);
            let r180 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r180);
            let Some(t181) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000870ff1); };
            c.w32(8, t181);
            let r182 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r182);
            let Some(t183) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000870ff8); };
            c.w32(9, t183);
            let r184 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r184);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 151; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087100e); };
            c.w32(6, t185);
            let r186 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871015); };
            if !c.push64(t187) { return RegfileExit::fault(0x7000871015); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700087101f, 0x700087101a);
        }
        129 => {
            blk = 115; continue;
        }
        130 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871024); };
            if !c.push64(t188) { return RegfileExit::fault(0x7000871024); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087102c); };
            c.w64(6, t189);
            return c.call(0x70007f2274, 0x7000871035, 0x7000871030);
        }
        131 => {
            blk = 115; continue;
        }
        132 => {
            let Some(t190) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087103a); };
            c.w32(9, t190);
            let r191 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r191);
            let Some(t192) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871041); };
            c.w32(11, t192);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 147; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w32(15, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 139; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 151; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t193) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087106b); };
            c.w32(9, t193);
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000871070); };
            c.cmp(t194, c.r32(9), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087107d); };
            c.w32(8, t195);
            let r196 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r196);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t197) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087108e); };
            c.w32(9, t197);
            let r198 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r198);
            let Some(t199) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871095); };
            c.cmp(t199, 0x40u64, 16);
            if c.cc_ae() { blk = 151; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t200) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008710a1); };
            c.w32(8, t200);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 115; continue;
        }
        139 => {
            let Some(t201) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008710ae); };
            c.w32(11, t201);
            let Some(t202) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008710b3); };
            c.cmp(t202, c.r32(11), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t203) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008710c0); };
            c.w32(9, t203);
            c.w64(11, c.r64(9));
            let r204 = c.r32(11) & 0xc0u64;
            c.w32(11, r204);
            c.logic(r204, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t205) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008710d8); };
            c.w32(11, t205);
            let r206 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r206);
            let Some(t207) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008710df); };
            c.w32(11, t207);
            let r208 = c.r32(11) & 0x1fu64;
            c.w32(11, r208);
            c.logic(r208, 32);
            let r209 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r209);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 151; continue; }
            blk = 142; continue;
        }
        142 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t210) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008710f7); };
            c.w32(0, t210);
            let r211 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r211);
            let Some(t212) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008710fe); };
            c.cmp(t212, c.r32(0), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t213) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871107); };
            c.w32(8, t213);
            let Some(t214) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700087110b); };
            c.cmp(t214, c.r32(8), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            blk = 115; continue;
        }
        146 => {
            let Some(t215) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871119); };
            c.w32(0, t215);
            let r216 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r216);
            blk = 115; continue;
        }
        147 => {
            let Some(t217) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871124); };
            c.w32(9, t217);
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000871129); };
            c.cmp(t218, c.r32(9), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t219) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871132); };
            c.w32(0, t219);
            let r220 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r220);
            blk = 115; continue;
        }
        149 => {
            c.w64(0, c.r64(12));
            blk = 115; continue;
        }
        150 => {
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000871146); };
            c.w64(0, t221);
            blk = 115; continue;
        }
        151 => {
            let Some(t222) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000871152); };
            c.w64(5, t222);
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000871156); };
            c.w64(3, t223);
            c.w32(0, 0x2u64);
            let Some(t224) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x7000871162); };
            c.w64(10, t224);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x7000871167); }
            let Some(t225) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700087116c); };
            c.w64(10, t225);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x7000871171); }
            let Some(t226) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000871176); };
            c.w64(10, t226);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x700087117b); }
            let Some(t227) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000871180); };
            c.w64(6, t227);
            let r228 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r228);
            return RegfileExit::jump(0x700086ed74);
        }
        152 => {
            blk = 151; continue;
        }
        153 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 151; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t229) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871197); };
            c.w32(11, t229);
            let r230 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r230);
            blk = 18; continue;
        }
        155 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008711a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008711a7); }
            blk = 159; continue;
        }
        156 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008711ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008711b1); }
            blk = 159; continue;
        }
        157 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008711b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008711bb); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t231) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008711c5); };
            c.w64(8, t231);
            blk = 83; continue;
        }
        159 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008711ce); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008711d2); }
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008711d3); };
            c.w64(3, t232);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008711e4, 0x70008711df);
        }
        160 => {
            let r233 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r233);
            c.w64(8, c.r64(0));
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008711e9); };
            c.w64(11, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008711ed); };
            c.w64(9, t235);
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008711f1); };
            c.w64(2, t236);
            blk = 86; continue;
        }
        161 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008711fd); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000871206); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700087120a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700087120e); }
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700087121b, 0x7000871216);
        }
        162 => {
            c.w64(9, c.r64(0));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087121e); };
            c.w64(8, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000871222); };
            c.w64(11, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000871226); };
            c.w64(2, t239);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700087122a); }
            blk = 105; continue;
        }
        163 => {
            return RegfileExit::fault(0x7000871234);
        }
        164 => {
            blk = 151; continue;
        }
        165 => {
            return RegfileExit::fault(0x700087123a);
        }
        166 => {
            blk = 151; continue;
        }
        167 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000871240); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000871242); }
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871244); };
            if !c.push64(t240) { return RegfileExit::fault(0x7000871244); }
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000871247); };
            c.w64(3, t241);
            c.w32(0, 0x3u64);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000871253); };
            c.w64(6, t242);
            return c.call(0x700086ed74, 0x700087125c, 0x7000871257);
        }
        168 => {
            blk = 115; continue;
        }
        169 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a1af4, 0x7000871269, 0x7000871264);
        }
        170 => {
            let Some(t243) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000871269); };
            c.w64(5, t243);
            c.w32(2, 0x2u64);
            c.w64(3, c.r64(0));
            let Some(t244) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x7000871275); };
            c.w64(0, t244);
            let Some(t245) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087127a); };
            c.w64(6, t245);
            c.w64(1, c.r64(0));
            let r246 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r246);
            return RegfileExit::jump(0x70008bc174);
        }
        171 => {
            blk = 151; continue;
        }
        172 => {
            return RegfileExit::jump(0x7000871294);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008712b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008712b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008712b8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008712be); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008712c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008712c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008712ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008712ce); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008712db); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008712e2); };
            c.w32(9, t3);
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_b() { blk = 154; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(9), 0xbcu64, 16);
            if c.cc_e() { blk = 172; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871308); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087130f); };
            c.w32(11, t6);
            c.cmp(c.r16(11), 0x42u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700087131b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(12) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(15, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(15), c.r64(12), 64);
            c.w64(15, r7);
            c.w64(12, 0x3ffffffffffffeu64);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 154; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_ae() { blk = 156; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871368); };
            c.w32(12, t8);
            c.logic(c.r32(12) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(11), 0x25u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(11), 0x2du64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(11) & 0x20u64, 8);
            if c.cc_ne() { blk = 154; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, c.r64(3));
            blk = 18; continue;
        }
        16 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008713a0); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            blk = 18; continue;
        }
        17 => {
            c.w64(11, c.r64(3));
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008713ac); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(0));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008713b6); };
            c.w64(7, t11);
            blk = 20; continue;
        }
        19 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008713bf); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008713c6); };
            c.w32(12, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008713cb); }
            blk = 20; continue;
        }
        20 => {
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008713cf); };
            c.cmp(t15, 0x410u64, 32);
            if c.cc_le() { blk = 49; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008713dc); };
            c.w32(3, t16);
            c.logic(c.r32(3) & 0x200000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008713ec); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            c.logic(c.r32(3) & 0xffc00u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 23; continue;
        }
        23 => {
            let r19 = c.shr(c.r32(3), 0xau64, 32).unwrap_or(c.r32(3));
            c.w32(3, r19);
            let r20 = c.r32(3) & 0x3ffu64;
            c.w32(3, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_be() { blk = 38; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000871411); };
            c.w32(6, (t21 as u16 as i16 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871415); };
            c.w32(2, t22);
            let r23 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r23);
            let r24 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700087141f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000871423); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700087142c); }
            blk = 26; continue;
        }
        26 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000871430); };
            c.w64(9, t25);
            let r26 = c.sub(c.r32(9), c.r32(15), 32);
            c.w32(9, r26);
            let r27 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r27);
            let r28 = c.add(c.r32(9), c.r32(15), 32);
            c.w32(9, r28);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000871444); };
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
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087145e); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871466); };
            c.w32(12, t35);
            let r36 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r36);
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_be() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(15, c.r64(9).wrapping_add(0x1));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000871478); };
            c.w64(9, t37);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 29; continue;
        }
        29 => {
            blk = 31; continue;
        }
        30 => {
            let r38 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r38);
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_l() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(9, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000871493); };
            c.w32(9, t39);
            let r40 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r40);
            let r41 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r41);
            let r42 = c.r32(9) & 0x3ffu64;
            c.w32(9, r42);
            c.logic(r42, 32);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008714ad); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008714b5); }
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008714b9); };
            c.w32(12, t45);
            let r46 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r46);
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_a() { blk = 93; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008714d4); };
            c.w64(9, t47);
            blk = 60; continue;
        }
        36 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008714dd); };
            c.w64(9, t48);
            blk = 60; continue;
        }
        37 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008714e6); };
            c.w64(9, t49);
            blk = 60; continue;
        }
        38 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            let r50 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r50);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 39; continue;
        }
        39 => {
            let r51 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r51);
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871504); };
            c.cmp(t52, c.r32(11), 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 39; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 60; continue;
        }
        42 => {
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087151a); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(12, c.r64(7));
            blk = 44; continue;
        }
        44 => {
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087152a); };
            c.w32(15, t55);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r56 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r56);
            let Some(t57) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871539); };
            c.w32(0, t57);
            let r58 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            let r59 = c.r64(0) & c.r64(15);
            c.w64(0, r59);
            c.logic(r59, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        45 => {
            let r60 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r60);
            let r61 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r61);
            c.w64(2, c.r64(15));
            let r62 = c.r64(2) & c.r64(0);
            c.w64(2, r62);
            c.logic(r62, 64);
            c.w64(0, c.r64(2));
            blk = 46; continue;
        }
        46 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(2).wrapping_mul(4));
            let Some(t63) = c.ld32(c.r64(12).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871564); };
            c.w32(6, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000871569); };
            c.cmp(t64, c.r32(6), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(6), c.r32(11), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 45; continue;
        }
        49 => {
            let Some(t65) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000871581); };
            c.cmp(t65, 0xbdu64, 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t66) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087158e); };
            c.logic(t66 & 0x24u64, 8);
            if c.cc_ne() { blk = 154; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871599); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(12, c.r64(7));
            blk = 53; continue;
        }
        53 => {
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008715a9); };
            c.w32(15, t69);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r70 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r70);
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008715b8); };
            c.w32(0, t71);
            let r72 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r72);
            let r73 = c.r64(0) & c.r64(15);
            c.w64(0, r73);
            c.logic(r73, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 55; continue;
        }
        54 => {
            let r74 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r74);
            let r75 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r75);
            c.w64(2, c.r64(15));
            let r76 = c.r64(2) & c.r64(0);
            c.w64(2, r76);
            c.logic(r76, 64);
            c.w64(0, c.r64(2));
            blk = 55; continue;
        }
        55 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t77) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008715da); };
            c.w32(1, t77);
            let r78 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r78);
            let Some(t79) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008715e2); };
            c.cmp(t79, c.r32(1), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008715eb); };
            c.cmp(t80, c.r32(1), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t81) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008715f4); };
            c.cmp(t81, c.r32(11), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008715fa); };
            c.w32(12, t82);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_ae() { blk = 168; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t83) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087160e); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000871615); };
            c.cmp(t85, c.r32(12), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t86) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000871622); };
            c.cmp(t86, 0x82eu64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t87) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087162f); };
            c.w32(8, t87);
            let r88 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r88);
            let Some(t89) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871636); };
            c.w32(8, t89);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 91; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t90) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871646); };
            c.w32(12, t90);
            let r91 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r91);
            c.w32(15, c.r32(12));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(12), 0x30u64, 32);
            if c.cc_g() { blk = 91; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(12, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(8, c.r64(11));
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 66; continue;
        }
        66 => {
            c.w64(15, c.r64(12));
            let r92 = c.r32(15) & 0x7u64;
            c.w32(15, r92);
            c.logic(r92, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 158; continue;
        }
        71 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            blk = 158; continue;
        }
        74 => {
            let Some(t93) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008716b0); };
            c.w32(12, t93);
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008716b4); };
            c.cmp(t94, c.r32(12), 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t95) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008716c1); };
            c.w32(8, t95);
            let r96 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r96);
            let Some(t97) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008716c8); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r98);
            let Some(t99) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008716cf); };
            c.w32(12, t99);
            blk = 66; continue;
        }
        76 => {
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008716d7); };
            c.w32(12, t100);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r101 = c.add(c.r64(0), c.r64(12), 64);
            c.w64(0, r101);
            let Some(t102) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008716e4); };
            c.w32(8, t102);
            let r103 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r103);
            let Some(t104) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008716eb); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008716f2); };
            c.w32(12, t106);
            blk = 66; continue;
        }
        77 => {
            let Some(t107) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008716fd); };
            c.w32(8, t107);
            let r108 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r108);
            let Some(t109) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871704); };
            c.w32(12, t109);
            let r110 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r110);
            let Some(t111) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087170b); };
            c.w32(12, t111);
            blk = 66; continue;
        }
        78 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 81; continue;
        }
        79 => {
            c.w32(15, 0x1u64);
            blk = 81; continue;
        }
        80 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(11));
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 160; continue; }
            blk = 82; continue;
        }
        82 => {
            let r112 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r112);
            blk = 83; continue;
        }
        83 => {
            let r113 = c.r32(12) & 0x8u64;
            c.w32(12, r113);
            c.logic(r113, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t114) = c.ld16(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(2))) else { return RegfileExit::fault(0x7000871743); };
            c.w32(8, t114);
            blk = 86; continue;
        }
        85 => {
            let Some(t115) = c.ld8(c.r64(8).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700087174a); };
            c.w32(8, t115);
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 154; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087177d); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000871784); };
            c.cmp(t118, c.r32(8), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 19; continue;
        }
        93 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000871796); };
            c.w64(12, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087179a); };
            c.w64(9, t120);
            blk = 94; continue;
        }
        94 => {
            if let Some(sr) = c.shl(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let Some(t121) = c.ld32(c.r64(1).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008717a2); };
            c.w32(15, t121);
            c.w32(0, c.r32(15));
            c.w64(3, c.r64(15));
            let r122 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r122);
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t123) = c.ld32(c.r64(1).wrapping_add(c.r64(12)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008717b5); };
            c.w32(12, t123);
            let r124 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r124);
            blk = 110; continue;
        }
        96 => {
            c.w64(12, c.r64(3));
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t125) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008717c9); };
            c.w32(15, t125);
            let r126 = c.r64(12) & 0x3ffu64;
            c.w64(12, r126);
            c.logic(r126, 64);
            let r127 = c.r32(0) & 0x380u64;
            c.w32(0, r127);
            c.logic(r127, 32);
            let r128 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r128);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            let Some(t129) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008717ea); };
            c.w32(9, t129);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 101; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008717f4); };
            c.w32(0, t130);
            let r131 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r131);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008717ff); };
            c.w64(0, t132);
            blk = 99; continue;
        }
        99 => {
            let r133 = c.sub(c.r64(12), c.r64(9), 64);
            c.w64(12, r133);
            let Some(t134) = c.ld32(c.r64(0).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871809); };
            c.w32(12, t134);
            let r135 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r135);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700087181b); }
            blk = 103; continue;
        }
        101 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087182a); };
            c.w32(9, t136);
            let r137 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r137);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000871832); }
            blk = 103; continue;
        }
        103 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000871838); };
            c.w64(9, t138);
            let Some(t139) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700087183f); };
            c.w64(12, t139);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000871847); };
            c.w64(0, t140);
            let Some(t141) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700087184e); };
            c.cmp(t141, c.r64(15), 64);
            if c.cc_be() { blk = 164; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000871857); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 105; continue;
        }
        105 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087185f); };
            c.w64(12, t142);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000871866); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087186a); }
            c.w64(12, c.r64(9));
            blk = 110; continue;
        }
        106 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871875); };
            c.w32(12, t143);
            let r144 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r144);
            blk = 110; continue;
        }
        107 => {
            let Some(t145) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087187f); };
            c.w32(9, t145);
            let Some(t146) = c.ld32(c.r64(12).wrapping_add(c.r64(1)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871884); };
            c.w32(15, t146);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_ae() { blk = 166; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t147) = c.ld32(c.r64(12).wrapping_add(c.r64(1)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087189c); };
            c.w32(12, t147);
            let r148 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r148);
            let r149 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r149);
            c.w64(3, c.r64(15));
            blk = 110; continue;
        }
        109 => {
            let Some(t150) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008718ac); };
            c.w32(9, t150);
            let r151 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r151);
            c.w64(3, c.r64(9));
            blk = 110; continue;
        }
        110 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 149; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t152) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008718bf); };
            c.w32(9, t152);
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008718c4); };
            c.cmp(t153, c.r32(9), 32);
            if c.cc_ne() { blk = 132; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t154) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008718d1); };
            c.w32(2, t154);
            let r155 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r155);
            let Some(t156) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008718d9); };
            c.w32(9, t156);
            let r157 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r157);
            let Some(t158) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008718e0); };
            c.logic(t158 & 0x2u64, 8);
            if c.cc_ne() { blk = 130; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008718eb); };
            c.cmp(t159, c.r32(9), 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008718f4); };
            c.w64(0, t160);
            blk = 115; continue;
        }
        115 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008718fe); }
            return c.ret(0x0, 0x70008718ff);
        }
        116 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000871900); };
            c.w64(9, t161);
            let Some(t162) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x7000871907); };
            c.cmp(t162, 0x20u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t163) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000871911); };
            c.w32(9, t163);
            let Some(t164) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000871915); };
            c.cmp(t164, c.r32(9), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t165) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871922); };
            c.w32(9, t165);
            let r166 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r166);
            let Some(t167) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871929); };
            c.w32(11, t167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700087192d); };
            c.cmp(t168, c.r32(11), 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t169) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000871936); };
            c.w32(9, t169);
            let r170 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r170);
            let Some(t171) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087193d); };
            c.w32(11, t171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000871941); };
            c.cmp(t172, c.r32(11), 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 120; continue;
        }
        120 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t173) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871954); };
            c.w32(11, t173);
            let r174 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r174);
            let Some(t175) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087195b); };
            c.w32(12, t175);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 126; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700087197b); };
            c.cmp(t176, c.r32(11), 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 127; continue;
        }
        125 => {
            let Some(t177) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087198a); };
            c.w32(8, t177);
            let r178 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r178);
            blk = 127; continue;
        }
        126 => {
            c.w64(8, c.r64(9));
            blk = 127; continue;
        }
        127 => {
            let Some(t179) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871996); };
            c.w32(8, t179);
            let r180 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r180);
            let Some(t181) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087199d); };
            c.w32(8, t181);
            let r182 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r182);
            let Some(t183) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008719a4); };
            c.w32(9, t183);
            let r184 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r184);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 154; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008719ba); };
            c.w32(6, t185);
            let r186 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008719c1); };
            if !c.push64(t187) { return RegfileExit::fault(0x70008719c1); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008719cb, 0x70008719c6);
        }
        129 => {
            blk = 115; continue;
        }
        130 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008719d0); };
            if !c.push64(t188) { return RegfileExit::fault(0x70008719d0); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008719d8); };
            c.w64(6, t189);
            return c.call(0x70007f2274, 0x70008719e1, 0x70008719dc);
        }
        131 => {
            blk = 115; continue;
        }
        132 => {
            let Some(t190) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008719e6); };
            c.w32(9, t190);
            let r191 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r191);
            let Some(t192) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008719ed); };
            c.w32(11, t192);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 147; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w32(15, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 139; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 154; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t193) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871a17); };
            c.w32(9, t193);
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000871a1c); };
            c.cmp(t194, c.r32(9), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871a29); };
            c.w32(8, t195);
            let r196 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r196);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t197) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871a3a); };
            c.w32(9, t197);
            let r198 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r198);
            let Some(t199) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871a41); };
            c.cmp(t199, 0x40u64, 16);
            if c.cc_ae() { blk = 154; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t200) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871a4d); };
            c.w32(8, t200);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 115; continue;
        }
        139 => {
            let Some(t201) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871a5a); };
            c.w32(11, t201);
            let Some(t202) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000871a5f); };
            c.cmp(t202, c.r32(11), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t203) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000871a6c); };
            c.w32(9, t203);
            c.w64(11, c.r64(9));
            let r204 = c.r32(11) & 0xc0u64;
            c.w32(11, r204);
            c.logic(r204, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t205) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871a84); };
            c.w32(11, t205);
            let r206 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r206);
            let Some(t207) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000871a8b); };
            c.w32(11, t207);
            let r208 = c.r32(11) & 0x1fu64;
            c.w32(11, r208);
            c.logic(r208, 32);
            let r209 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r209);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 154; continue; }
            blk = 142; continue;
        }
        142 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 154; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t210) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000871aab); };
            c.w32(0, t210);
            let r211 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r211);
            let Some(t212) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000871ab2); };
            c.cmp(t212, c.r32(0), 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t213) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871abb); };
            c.w32(8, t213);
            let Some(t214) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000871abf); };
            c.cmp(t214, c.r32(8), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            blk = 115; continue;
        }
        146 => {
            let Some(t215) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871acd); };
            c.w32(0, t215);
            let r216 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r216);
            blk = 115; continue;
        }
        147 => {
            let Some(t217) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871ad8); };
            c.w32(9, t217);
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000871add); };
            c.cmp(t218, c.r32(9), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t219) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871ae6); };
            c.w32(0, t219);
            let r220 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r220);
            blk = 115; continue;
        }
        149 => {
            c.w64(0, c.r64(12));
            blk = 115; continue;
        }
        150 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t221) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000871afd); };
            c.cmp(c.r32(8), t221, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000871b03); };
            c.w64(0, t222);
            blk = 115; continue;
        }
        152 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000871b0f); };
            if !c.push64(t223) { return RegfileExit::fault(0x7000871b0f); }
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000871b12); };
            c.w64(3, t224);
            c.w32(0, 0x1u64);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000871b1e); };
            c.w64(6, t225);
            return c.call(0x700086ed74, 0x7000871b27, 0x7000871b22);
        }
        153 => {
            blk = 115; continue;
        }
        154 => {
            let Some(t226) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000871b2c); };
            c.w64(5, t226);
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0x2610)) else { return RegfileExit::fault(0x7000871b30); };
            c.w64(3, t227);
            c.w32(0, 0x4u64);
            let Some(t228) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x7000871b3c); };
            c.w64(10, t228);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000871b41); }
            let Some(t229) = c.ld64(c.r64(4).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000871b46); };
            c.w64(10, t229);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x7000871b4b); }
            let Some(t230) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x7000871b50); };
            c.w64(10, t230);
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(10)) { return RegfileExit::fault(0x7000871b55); }
            let Some(t231) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x7000871b5a); };
            c.w64(10, t231);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x7000871b5f); }
            let Some(t232) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000871b64); };
            c.w64(10, t232);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x7000871b69); }
            let Some(t233) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000871b6e); };
            c.w64(6, t233);
            let r234 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r234);
            return RegfileExit::jump(0x700086ed74);
        }
        155 => {
            blk = 154; continue;
        }
        156 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 154; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t235) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871b85); };
            c.w32(11, t235);
            let r236 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r236);
            blk = 18; continue;
        }
        158 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000871b91); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000871b95); }
            blk = 162; continue;
        }
        159 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000871b9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000871b9f); }
            blk = 162; continue;
        }
        160 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000871ba5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000871ba9); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t237) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000871bb3); };
            c.w64(8, t237);
            blk = 83; continue;
        }
        162 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000871bbc); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000871bc0); }
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x7000871bc1); };
            c.w64(3, t238);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000871bd2, 0x7000871bcd);
        }
        163 => {
            let r239 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r239);
            c.w64(8, c.r64(0));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000871bd7); };
            c.w64(11, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000871bdb); };
            c.w64(9, t241);
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000871bdf); };
            c.w64(7, t242);
            blk = 86; continue;
        }
        164 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000871bf0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000871bf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000871bf8); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x7000871bfc); }
            return c.call(0x70007febf4, 0x7000871c06, 0x7000871c01);
        }
        165 => {
            c.w64(9, c.r64(0));
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000871c09); };
            c.w64(8, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000871c0d); };
            c.w64(11, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000871c11); };
            c.w64(3, t245);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x7000871c15); }
            blk = 105; continue;
        }
        166 => {
            return RegfileExit::fault(0x7000871c1f);
        }
        167 => {
            blk = 154; continue;
        }
        168 => {
            return RegfileExit::fault(0x7000871c25);
        }
        169 => {
            blk = 154; continue;
        }
        170 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000871c2b); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000871c2d); }
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000871c2f); };
            if !c.push64(t246) { return RegfileExit::fault(0x7000871c2f); }
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000871c32); };
            c.w64(3, t247);
            c.w32(0, 0x3u64);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000871c3e); };
            c.w64(6, t248);
            return c.call(0x700086ed74, 0x7000871c47, 0x7000871c42);
        }
        171 => {
            blk = 115; continue;
        }
        172 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a1af4, 0x7000871c54, 0x7000871c4f);
        }
        173 => {
            let Some(t249) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871c54); };
            c.w32(8, t249);
            let Some(t250) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000871c58); };
            c.cmp(t250, c.r32(8), 32);
            if c.cc_ne() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t251) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871c61); };
            c.logic(t251 & 0x1u64, 8);
            if c.cc_ne() { blk = 154; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t252) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000871c6b); };
            c.w64(5, t252);
            c.w64(3, c.r64(0));
            let Some(t253) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x7000871c72); };
            c.w64(0, t253);
            let Some(t254) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x7000871c77); };
            c.w64(1, t254);
            let Some(t255) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000871c7c); };
            c.w64(2, t255);
            let Some(t256) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000871c81); };
            c.w64(6, t256);
            let r257 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r257);
            return RegfileExit::jump(0x70008bc174);
        }
        176 => {
            blk = 154; continue;
        }
        177 => {
            return RegfileExit::jump(0x7000871c98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000871cb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000871cb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000871cb8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000871cbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000871cc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000871cc6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000871cca); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 932; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871cd6); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871cdd); };
            c.w32(9, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000871ce2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000871ce6); }
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_be() { blk = 932; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 475; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000871cff); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000871d06); };
            c.w32(11, t6);
            c.cmp(c.r16(11), 0x42u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000871d12); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(12) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(15, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(15), c.r64(12), 64);
            c.w64(15, r7);
            c.w64(2, 0x3ffffffffffffeu64);
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_be() { blk = 476; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_e() { blk = 25; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_ae() { blk = 933; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871d63); };
            c.w32(12, t8);
            c.logic(c.r32(12) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 932; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r16(11), 0x25u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(11), 0x2du64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(11) & 0x20u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(9, c.r64(3));
            blk = 26; continue;
        }
        15 => {
            c.w64(10, c.r64(4));
            let r9 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r9);
            let r10 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r10);
            c.logic(r10, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000871dad); }
            c.w64(2, c.r64(3));
            c.w64(1, c.r64(13));
            c.w64(11, c.r64(1));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x7000871dbb); };
            c.w64(0, t11);
            c.w64(10, 0x7000871dc9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000871dc9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000871dcd); }
            return c.call(c.r64(0), 0x7000871dd3, 0x7000871dd1);
        }
        16 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000871dd3); }
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000871ddb); };
            c.w64(4, t12);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(8), 0xfffffffffffffffeu64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(12, c.r64(8));
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871df9); };
            c.w64(0, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000871dfd); };
            c.w64(3, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000871e01); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000871e05); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000871e09); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000871e0d); };
            c.w64(9, t18);
            blk = 476; continue;
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871e16); };
            c.w64(0, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000871e1a); };
            c.w64(1, t20);
            blk = 932; continue;
        }
        21 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871e23); };
            c.w64(0, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000871e27); };
            c.w64(1, t22);
            blk = 932; continue;
        }
        22 => {
            c.w64(9, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871e33); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000871e37); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000871e3b); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000871e3f); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000871e43); };
            c.w64(8, t27);
            blk = 26; continue;
        }
        23 => {
            c.w64(9, c.r64(3));
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871e4c); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(11));
            blk = 26; continue;
        }
        24 => {
            if let Some(sr) = c.shr(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let r30 = c.r64(12) & 0xffffffu64;
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 476; continue;
        }
        25 => {
            c.w64(9, c.r64(3));
            blk = 26; continue;
        }
        26 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000871e6e); };
            c.w32(11, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000871e72); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000871e76); }
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_ne() { blk = 299; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000871e87); };
            c.w32(12, t32);
            let r33 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r33);
            c.logic(c.r32(11) & 0xffc00u64, 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 28; continue;
        }
        28 => {
            let r34 = c.shr(c.r32(11), 0xau64, 32).unwrap_or(c.r32(11));
            c.w32(11, r34);
            let r35 = c.r32(11) & 0x3ffu64;
            c.w32(11, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_be() { blk = 41; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t36) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000871eb0); };
            c.w32(15, (t36 as u16 as i16 as i64 as u64));
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871eb6); };
            c.w32(2, t37);
            let r38 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            let r39 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r39);
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000871ec3); }
            c.w64(7, c.r64(3));
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000871ecc); }
            blk = 31; continue;
        }
        31 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000871ed0); };
            c.w64(6, t40);
            let r41 = c.sub(c.r32(6), c.r32(3), 32);
            c.w32(6, r41);
            let r42 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r42);
            let r43 = c.add(c.r32(6), c.r32(3), 32);
            c.w32(6, r43);
            c.w32(7, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000871ee0); };
            c.w32(7, t44);
            let r45 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r45);
            let r46 = c.shr(c.r32(7), 0x9u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r46);
            let r47 = c.r32(7) & 0x3ffu64;
            c.w32(7, r47);
            c.logic(r47, 32);
            c.w32(7, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t48) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871ef6); };
            c.w32(7, t48);
            let r49 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r49);
            let Some(t50) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871efe); };
            c.w32(7, t50);
            let r51 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r51);
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_be() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(3, c.r64(6).wrapping_add(0x1));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000871f0b); };
            c.w64(6, t52);
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            let r53 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r53);
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_l() { blk = 167; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(6, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000871f27); };
            c.w32(6, t54);
            let r55 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r55);
            let r56 = c.shr(c.r32(6), 0x9u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r56);
            let r57 = c.r32(6) & 0x3ffu64;
            c.w32(6, r57);
            c.logic(r57, 32);
            c.w32(7, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t58) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871f3d); };
            c.w32(1, t58);
            let r59 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r59);
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871f45); };
            c.w32(0, t60);
            let r61 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 167; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(11), c.r32(6), 32);
            if c.cc_be() { blk = 167; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000871f61); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000871f65); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000871f69); };
            c.w64(3, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000871f6d); };
            c.w64(1, t65);
            blk = 45; continue;
        }
        41 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r66 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(7, r66);
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 167; continue; }
            blk = 42; continue;
        }
        42 => {
            let r67 = c.sub(c.r64(7), 0x3u64, 64);
            c.w64(7, r67);
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000871f88); };
            c.cmp(t68, c.r32(9), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 42; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 167; continue;
        }
        45 => {
            c.w64(11, c.r64(7).wrapping_mul(4));
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000871fa2); };
            c.w32(15, t69);
            c.w32(2, c.r32(15));
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 47; continue;
        }
        47 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000871fc4); };
            c.w32(8, t70);
            let r71 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r71);
            blk = 412; continue;
        }
        49 => {
            let r72 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r72);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t73) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000871fd8); };
            c.w32(11, t73);
            let r74 = c.r64(15) & 0x3ffu64;
            c.w64(15, r74);
            c.logic(r74, 64);
            let r75 = c.r32(2) & 0x380u64;
            c.w32(2, r75);
            c.logic(r75, 32);
            let r76 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r76);
            c.cmp(c.r32(2), 0x100u64, 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let Some(t77) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000871ffb); };
            c.w32(8, t77);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872005); };
            c.w32(15, t78);
            let r79 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r79);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000872012); };
            c.w64(15, t80);
            blk = 52; continue;
        }
        52 => {
            let r81 = c.sub(c.r64(11), c.r64(8), 64);
            c.w64(11, r81);
            let Some(t82) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087201c); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 412; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087202e); }
            blk = 56; continue;
        }
        54 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087203c); };
            c.w32(8, t84);
            let r85 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r85);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000872044); }
            blk = 56; continue;
        }
        56 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087204a); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000872051); };
            c.w64(11, t87);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000872058); };
            c.w64(15, t88);
            let Some(t89) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700087205f); };
            c.cmp(t89, c.r64(12), 64);
            if c.cc_be() { blk = 935; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000872068); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 58; continue;
        }
        58 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087206f); };
            c.w64(11, t90);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000872076); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087207a); }
            blk = 412; continue;
        }
        59 => {
            let Some(t91) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872085); };
            c.w32(8, t91);
            let r92 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r92);
            blk = 412; continue;
        }
        60 => {
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000872092); };
            c.cmp(t93, c.r32(9), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x700087209f); };
            c.cmp(t94, c.r32(9), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008720ac); };
            c.cmp(t95, c.r32(9), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008720b9); };
            c.cmp(t96, c.r32(9), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t97) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x70008720c6); };
            c.cmp(t97, c.r32(9), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008720d3); };
            c.cmp(t98, c.r32(9), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008720e0); };
            c.cmp(t99, c.r32(9), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 67; continue;
        }
        67 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008720f3); };
            c.cmp(t100, c.r32(1), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 165; continue;
        }
        70 => {
            c.w64(9, c.r64(2));
            let r101 = c.r32(9) & 0x380u64;
            c.w32(9, r101);
            c.logic(r101, 32);
            c.cmp(c.r32(9), 0x80u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(9), 0x100u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(9), 0x180u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 85; continue;
        }
        75 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 932; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000872141); };
            c.w32(11, t102);
            let r103 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r103);
            c.w32(12, 0x4u64);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(12, 0x2u64);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 79; continue;
        }
        79 => {
            let r104 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872171); };
            c.cmp(t105, c.r32(11), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 932; continue;
        }
        81 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t106) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872181); };
            c.w32(11, t106);
            let Some(t107) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000872185); };
            c.cmp(t107, c.r32(11), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 85; continue;
        }
        84 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 932; continue; }
            blk = 85; continue;
        }
        85 => {
            let r108 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r108);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t109) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008721a4); };
            c.w32(11, t109);
            let Some(t110) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008721a9); };
            c.w32(8, t110);
            let r111 = c.r32(2) & 0x8u64;
            c.w32(2, r111);
            c.logic(r111, 32);
            let r112 = c.r64(15) & 0x3ffu64;
            c.w64(15, r112);
            c.logic(r112, 64);
            c.cmp(c.r32(9), 0x100u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(2), 0x8u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r113 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r113);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 127; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t114) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008721de); };
            c.w32(15, t114);
            let r115 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r115);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008721eb); };
            c.w64(15, t116);
            blk = 88; continue;
        }
        88 => {
            let r117 = c.sub(c.r64(11), c.r64(8), 64);
            c.w64(11, r117);
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 113; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700087220d); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r118 = c.r64(9) & c.r64(15);
            c.w64(9, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872220); };
            c.logic(t119 & 0x4u64, 8);
            if c.cc_ne() { blk = 1122; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 165; continue;
        }
        93 => {
            let Some(t120) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000872230); };
            c.w32(8, t120);
            let r121 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r121);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 94; continue;
        }
        94 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 101; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t122) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872258); };
            c.w32(9, t122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087225c); };
            c.cmp(t123, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t124) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872269); };
            c.w32(9, t124);
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087226d); };
            c.cmp(t125, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 98; continue;
        }
        98 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087227a); }
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000872280); }
            blk = 104; continue;
        }
        99 => {
            let Some(t126) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872287); };
            c.w32(9, t126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087228b); };
            c.cmp(t127, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(9, c.r64(1));
            let r128 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r128);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087229e); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 104; continue;
        }
        101 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t129) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008722b9); };
            c.w32(9, t129);
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008722bd); };
            c.cmp(t130, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 103; continue;
        }
        103 => {
            let r131 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r131);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008722cd); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 104; continue;
        }
        104 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            if c.cc_e() { blk = 111; continue; }
            blk = 106; continue;
        }
        106 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 108; continue;
        }
        108 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 165; continue; }
            blk = 109; continue;
        }
        109 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 110; continue;
        }
        110 => {
            blk = 165; continue;
        }
        111 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 112; continue;
        }
        112 => {
            blk = 932; continue;
        }
        113 => {
            let Some(t132) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000872324); };
            c.w32(8, t132);
            let r133 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r133);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 114; continue;
        }
        114 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000872331); }
            blk = 116; continue;
        }
        115 => {
            c.w64(9, c.r64(1));
            let r134 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r134);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087234d); }
            blk = 165; continue;
        }
        118 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000872358); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            if c.cc_e() { blk = 125; continue; }
            blk = 120; continue;
        }
        120 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 122; continue;
        }
        122 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 165; continue; }
            blk = 123; continue;
        }
        123 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 165; continue;
        }
        125 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 932; continue;
        }
        127 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 132; continue; }
            blk = 129; continue;
        }
        129 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008723bf); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r135 = c.r64(9) & c.r64(0);
            c.w64(9, r135);
            c.logic(r135, 64);
            let Some(t136) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008723d2); };
            c.logic(t136 & 0x4u64, 8);
            if c.cc_ne() { blk = 1119; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 165; continue;
        }
        132 => {
            let Some(t137) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008723e2); };
            c.w32(8, t137);
            let r138 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r138);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 133; continue;
        }
        133 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 138; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t139) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087240a); };
            c.w32(9, t139);
            let Some(t140) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087240e); };
            c.cmp(t140, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t141) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087241b); };
            c.w32(9, t141);
            let Some(t142) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087241f); };
            c.cmp(t142, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 137; continue;
        }
        137 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087242c); }
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000872432); }
            blk = 143; continue;
        }
        138 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872439); };
            c.w32(9, t143);
            let Some(t144) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087243d); };
            c.cmp(t144, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(9, c.r64(1));
            let r145 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r145);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000872450); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 143; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t146) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087246b); };
            c.w32(9, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087246f); };
            c.cmp(t147, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 142; continue;
        }
        142 => {
            let r148 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r148);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700087247f); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 143; continue;
        }
        143 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            if c.cc_e() { blk = 150; continue; }
            blk = 145; continue;
        }
        145 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 147; continue;
        }
        147 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 165; continue; }
            blk = 148; continue;
        }
        148 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 149; continue;
        }
        149 => {
            blk = 165; continue;
        }
        150 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 932; continue;
        }
        152 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008724db); }
            blk = 155; continue;
        }
        154 => {
            c.w64(8, c.r64(1));
            let r149 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r149);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 155; continue;
        }
        155 => {
            let Some(t150) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008724f1); };
            c.w32(8, t150);
            let r151 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r151);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 164; continue; }
            blk = 156; continue;
        }
        156 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008724ff); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            if c.cc_e() { blk = 163; continue; }
            blk = 158; continue;
        }
        158 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 160; continue;
        }
        160 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 164; continue; }
            blk = 161; continue;
        }
        161 => {
            if c.cc_e() { blk = 932; continue; }
            blk = 162; continue;
        }
        162 => {
            blk = 164; continue;
        }
        163 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 164; continue;
        }
        164 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000872542); }
            blk = 165; continue;
        }
        165 => {
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        166 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087254e); }
            return c.ret(0x0, 0x700087254f);
        }
        167 => {
            let Some(t152) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000872550); };
            c.cmp(t152, c.r32(9), 32);
            if c.cc_e() { blk = 298; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x700087255d); };
            c.cmp(t153, c.r32(9), 32);
            if c.cc_e() { blk = 297; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x700087256a); };
            c.cmp(t154, c.r32(9), 32);
            if c.cc_e() { blk = 296; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000872577); };
            c.cmp(t155, c.r32(9), 32);
            if c.cc_e() { blk = 295; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t156) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000872584); };
            c.cmp(t156, c.r32(9), 32);
            if c.cc_e() { blk = 294; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t157) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000872591); };
            c.cmp(t157, c.r32(9), 32);
            if c.cc_e() { blk = 293; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x700087259e); };
            c.cmp(t158, c.r32(9), 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008725ab); };
            c.w32(8, t159);
            let r160 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r160);
            c.w32(11, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 290; continue; }
            blk = 176; continue;
        }
        176 => {
            let r161 = c.r32(11) & 0x3u64;
            c.w32(11, r161);
            c.logic(r161, 32);
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            let r162 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r162);
            c.logic(r162, 64);
            blk = 208; continue;
        }
        178 => {
            let Some(t163) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008725dc); };
            c.w32(11, t163);
            let Some(t164) = c.ld32(c.r64(13).wrapping_add(0x148)) else { return RegfileExit::fault(0x70008725e0); };
            c.cmp(t164, c.r32(11), 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008725e9); };
            c.w64(0, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008725ed); };
            c.w64(1, t166);
            blk = 932; continue;
        }
        180 => {
            let Some(t167) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008725f6); };
            c.w32(11, t167);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            c.w8(11, (c.cc_b() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t168) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000872612); };
            c.w32(12, t168);
            let r169 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r169);
            c.w64(7, c.r64(12));
            blk = 183; continue;
        }
        182 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 289; continue; }
            blk = 184; continue;
        }
        184 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_be() { blk = 203; continue; }
            blk = 185; continue;
        }
        185 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t170) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000872638); };
            c.w32(11, t170);
            let r171 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r171);
            blk = 188; continue;
        }
        187 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 188; continue;
        }
        188 => {
            let Some(t172) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872644); };
            c.w32(12, t172);
            let r173 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r173);
            let r174 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r174);
            c.w64(0, c.r64(11));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 190; continue;
        }
        189 => {
            c.w64(0, c.r64(3));
            blk = 190; continue;
        }
        190 => {
            c.w64(3, c.r64(0));
            let r175 = c.sub(c.r32(3), c.r32(15), 32);
            c.w32(3, r175);
            let r176 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r176);
            let r177 = c.add(c.r32(3), c.r32(15), 32);
            c.w32(3, r177);
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087266c); };
            c.w32(2, t178);
            let r179 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r179);
            let Some(t180) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872674); };
            c.w32(2, t180);
            let r181 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r181);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_be() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            c.w32(15, c.r64(3).wrapping_add(0x1));
            c.w64(3, c.r64(0));
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ne() { blk = 189; continue; }
            blk = 193; continue;
        }
        193 => {
            blk = 195; continue;
        }
        194 => {
            let r182 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r182);
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_l() { blk = 202; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w32(0, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t183) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087269d); };
            c.w32(3, t183);
            let r184 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r184);
            let Some(t185) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008726a5); };
            c.w32(2, t185);
            let r186 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r186);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(7), c.r32(15), 32);
            if c.cc_be() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(11, c.r64(0).wrapping_add(0x2));
            blk = 207; continue;
        }
        200 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008726c0); };
            c.w64(0, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008726c4); };
            c.w64(1, t188);
            blk = 932; continue;
        }
        201 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008726cd); };
            c.w64(0, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008726d1); };
            c.w64(1, t190);
            blk = 932; continue;
        }
        202 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008726da); };
            c.w64(0, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008726de); };
            c.w64(1, t192);
            blk = 932; continue;
        }
        203 => {
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x2));
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 288; continue; }
            blk = 204; continue;
        }
        204 => {
            let r193 = c.sub(c.r64(11), 0x2u64, 64);
            c.w64(11, r193);
            let Some(t194) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000872700); };
            c.cmp(t194, c.r32(9), 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_ne() { blk = 204; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 288; continue;
        }
        207 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000872712); };
            c.w32(8, t195);
            let r196 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r196);
            let r197 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r197);
            c.logic(r197, 64);
            blk = 208; continue;
        }
        208 => {
            let Some(t198) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700087271e); };
            c.w32(11, t198);
            let r199 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r199);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000872725); }
            c.w32(12, 0);
            c.logic(0, 32);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t200) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872731); };
            c.cmp(t200, c.r32(12), 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872737); };
            c.w64(0, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087273b); };
            c.w64(1, t202);
            blk = 932; continue;
        }
        211 => {
            let Some(t203) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000872744); };
            c.w32(11, t203);
            c.logic(c.r32(11) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 287; continue; }
            blk = 212; continue;
        }
        212 => {
            if let Some(sr) = c.shr(c.r64(11), 0xau64, 64) { c.w64(11, sr); }
            let r204 = c.r64(11) & 0x3ffu64;
            c.w64(11, r204);
            c.logic(r204, 64);
            c.w64(11, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t205) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000872769); };
            c.w32(12, t205);
            let r206 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r206);
            let Some(t207) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000872770); };
            c.cmp(t207, c.r32(9), 32);
            if c.cc_ne() { blk = 286; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t208) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087277b); };
            c.w32(15, t208);
            c.w32(0, c.r32(15));
            c.logic(c.r8(15) & 0x52u64, 8);
            if c.cc_ne() { blk = 285; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t209) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087278d); };
            c.w32(3, t209);
            let Some(t210) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000872791); };
            c.cmp(t210, c.r32(3), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t211) = c.ld8(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087279a); };
            c.logic(t211 & 0x1u64, 8);
            if c.cc_e() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w32(3, 0x1u64);
            c.w64(7, c.r64(3));
            blk = 218; continue;
        }
        217 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 218; continue;
        }
        218 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008727be); };
            c.w64(0, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008727c2); };
            c.w64(1, t213);
            blk = 932; continue;
        }
        220 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008727d1); };
            c.w64(2, t214);
            let Some(t215) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008727d5); };
            c.cmp(t215, c.r32(2), 32);
            if c.cc_ne() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008727dc); };
            c.w64(1, t216);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008727e0); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r217 = c.r64(9) & c.r64(1);
            c.w64(9, r217);
            c.logic(r217, 64);
            let Some(t218) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008727ee); };
            c.logic(t218 & 0x4u64, 8);
            if c.cc_ne() { blk = 1116; continue; }
            blk = 223; continue;
        }
        223 => {
            blk = 284; continue;
        }
        224 => {
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008727fe); };
            c.w64(0, t219);
            c.w64(1, c.r64(2));
            blk = 932; continue;
        }
        225 => {
            let r220 = c.r32(0) & 0x380u64;
            c.w32(0, r220);
            c.logic(r220, 32);
            c.cmp(c.r32(0), 0x80u64, 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r32(0), 0x100u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(0), 0x180u64, 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 244; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872835); };
            c.w64(0, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872839); };
            c.w64(1, t222);
            blk = 932; continue;
        }
        230 => {
            let Some(t223) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872842); };
            c.logic(t223 & 0x1u64, 8);
            if c.cc_e() { blk = 238; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t224) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000872848); };
            c.w32(9, t224);
            let r225 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r225);
            c.w32(11, 0x4u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 232; continue;
        }
        232 => {
            c.w32(11, 0x2u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            let r226 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r226);
            c.logic(r226, 64);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872874); };
            c.w64(12, t227);
            let Some(t228) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872878); };
            c.cmp(t228, c.r32(9), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087287f); };
            c.w64(0, t229);
            c.w64(1, c.r64(12));
            blk = 932; continue;
        }
        236 => {
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087288b); };
            c.w64(0, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087288f); };
            c.w64(1, t231);
            blk = 932; continue;
        }
        237 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872898); };
            c.w64(0, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087289c); };
            c.w64(1, t233);
            blk = 932; continue;
        }
        238 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008728a5); };
            c.w64(0, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008728a9); };
            c.w64(1, t235);
            blk = 932; continue;
        }
        239 => {
            let Some(t236) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008728b2); };
            c.logic(t236 & 0x1u64, 8);
            if c.cc_e() { blk = 244; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008728b8); };
            c.w64(12, t237);
            let Some(t238) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008728bc); };
            c.w32(9, t238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008728c1); };
            c.cmp(t239, c.r32(9), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008728ca); };
            c.w64(0, t240);
            c.w64(1, c.r64(12));
            blk = 932; continue;
        }
        242 => {
            let Some(t241) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008728d6); };
            c.logic(t241 & 0x1u64, 8);
            if c.cc_e() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008728dc); };
            c.w64(0, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008728e0); };
            c.w64(1, t243);
            blk = 932; continue;
        }
        244 => {
            let r244 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r244);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t245) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008728f0); };
            c.w32(9, t245);
            let Some(t246) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008728f5); };
            c.w32(11, t246);
            let r247 = c.r64(15) & 0x3ffu64;
            c.w64(15, r247);
            c.logic(r247, 64);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r248 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r248);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 271; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t249) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872920); };
            c.logic(t249 & 0x1u64, 8);
            if c.cc_e() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872926); };
            c.w64(0, t250);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087292a); }
            blk = 249; continue;
        }
        248 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872931); };
            c.w64(0, t251);
            let r252 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r252);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 249; continue;
        }
        249 => {
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087293f); };
            c.w64(12, t253);
            let Some(t254) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000872946); };
            c.w64(15, t254);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087294e); };
            c.w64(3, t255);
            let Some(t256) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000872955); };
            c.cmp(t256, c.r64(0), 64);
            if c.cc_be() { blk = 937; continue; }
            blk = 250; continue;
        }
        250 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700087295e); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 251; continue;
        }
        251 => {
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000872966); };
            c.w64(15, t257);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700087296d); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000872972); }
            c.w64(0, c.r64(12));
            blk = 253; continue;
        }
        252 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087297e); };
            c.w64(0, t258);
            blk = 253; continue;
        }
        253 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872982); };
            c.w64(15, t259);
            let Some(t260) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872986); };
            c.w32(12, t260);
            let r261 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r261);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700087298d); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t262) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872997); };
            c.w32(3, t262);
            let r263 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r263);
            c.w64(2, c.r64(3));
            let r264 = c.r32(2) & 0x3ffu64;
            c.w32(2, r264);
            c.logic(r264, 32);
            let r265 = c.r32(3) & 0x3ffffc00u64;
            c.w32(3, r265);
            c.logic(r265, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 256; continue;
        }
        255 => {
            let r266 = c.shl(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r266);
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008729bf); };
            c.w64(3, t267);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 256; continue;
        }
        256 => {
            let r268 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r268);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 267; continue; }
            blk = 257; continue;
        }
        257 => {
            c.w64(11, c.r64(2).wrapping_add(0x3));
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008729d8); };
            c.w64(1, t269);
            let Some(t270) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008729df); };
            c.w64(6, t270);
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(7).wrapping_add(c.r64(6)));
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008729ee); };
            c.w64(0, t271);
            let Some(t272) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008729f5); };
            c.cmp(t272, c.r64(8), 64);
            if c.cc_be() { blk = 939; continue; }
            blk = 258; continue;
        }
        258 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008729fe); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 259; continue;
        }
        259 => {
            c.w32(0, c.r32(11));
            c.w32(1, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x7000872a0c); };
            c.w64(6, t273);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000872a13); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000872a17); }
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_e() { blk = 262; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000872a30); };
            c.w64(1, t274);
            blk = 261; continue;
        }
        261 => {
            let r275 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r275);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000872a3b); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ne() { blk = 261; continue; }
            blk = 262; continue;
        }
        262 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            let r276 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r276);
            let Some(t277) = c.ld32(c.r64(2).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000872a4e); };
            c.w32(11, t277);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000872a52); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 263; continue; }
            blk = 264; continue;
        }
        264 => {
            let r278 = c.r32(0) | c.r32(12);
            c.w32(0, r278);
            c.logic(r278, 32);
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000872a63); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000872a67); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 265; continue;
        }
        265 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r279 = c.r64(11) & c.r64(15);
            c.w64(11, r279);
            c.logic(r279, 64);
            let Some(t280) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872a7b); };
            c.logic(t280 & 0x4u64, 8);
            if c.cc_ne() { blk = 1113; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w64(3, c.r64(8));
            blk = 267; continue;
        }
        267 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000872a91); };
            c.w64(0, t281);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000872a95); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r282 = c.r64(9) & c.r64(3);
            c.w64(9, r282);
            c.logic(r282, 64);
            let Some(t283) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872aa7); };
            c.logic(t283 & 0x4u64, 8);
            if c.cc_ne() { blk = 1110; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000872ab2); };
            c.w64(8, t284);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000872ab6); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r285 = c.r64(9) & c.r64(15);
            c.w64(9, r285);
            c.logic(r285, 64);
            let Some(t286) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872ac4); };
            c.logic(t286 & 0x4u64, 8);
            if c.cc_ne() { blk = 1107; continue; }
            blk = 270; continue;
        }
        270 => {
            blk = 284; continue;
        }
        271 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 276; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872ae2); };
            c.w64(12, t287);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000872ae6); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r288 = c.r64(11) & c.r64(12);
            c.w64(11, r288);
            c.logic(r288, 64);
            let Some(t289) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872af5); };
            c.logic(t289 & 0x4u64, 8);
            if c.cc_ne() { blk = 1104; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872b00); };
            c.w64(0, t290);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000872b04); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 275; continue; }
            blk = 274; continue;
        }
        274 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r291 = c.r64(8) & c.r64(12);
            c.w64(8, r291);
            c.logic(r291, 64);
            let Some(t292) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872b16); };
            c.logic(t292 & 0x4u64, 8);
            if c.cc_ne() { blk = 1101; continue; }
            blk = 275; continue;
        }
        275 => {
            blk = 284; continue;
        }
        276 => {
            let Some(t293) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872b26); };
            c.logic(t293 & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872b2c); };
            c.w64(0, t294);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000872b30); }
            blk = 279; continue;
        }
        278 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872b37); };
            c.w64(0, t295);
            let r296 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r296);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 279; continue;
        }
        279 => {
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000872b45); };
            c.w64(11, t297);
            let Some(t298) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000872b4c); };
            c.w64(12, t298);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000872b54); };
            c.w64(0, t299);
            let Some(t300) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000872b5b); };
            c.cmp(t300, c.r64(15), 64);
            if c.cc_be() { blk = 941; continue; }
            blk = 280; continue;
        }
        280 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000872b64); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 281; continue;
        }
        281 => {
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000872b6c); };
            c.w64(12, t301);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000872b73); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000872b77); }
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872b7d); };
            c.w64(12, t302);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000872b81); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r303 = c.r64(15) & c.r64(12);
            c.w64(15, r303);
            c.logic(r303, 64);
            let Some(t304) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872b90); };
            c.logic(t304 & 0x4u64, 8);
            if c.cc_ne() { blk = 1098; continue; }
            blk = 282; continue;
        }
        282 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x7000872b9b); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r305 = c.r64(8) & c.r64(12);
            c.w64(8, r305);
            c.logic(r305, 64);
            let Some(t306) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872baf); };
            c.logic(t306 & 0x4u64, 8);
            if c.cc_ne() { blk = 1095; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872bba); };
            c.w64(0, t307);
            blk = 166; continue;
        }
        285 => {
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872bc3); };
            c.w64(0, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872bc7); };
            c.w64(1, t309);
            blk = 932; continue;
        }
        286 => {
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872bd0); };
            c.w64(0, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872bd4); };
            c.w64(1, t311);
            blk = 932; continue;
        }
        287 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872bdd); };
            c.w64(0, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872be1); };
            c.w64(1, t313);
            blk = 932; continue;
        }
        288 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872bea); };
            c.w64(0, t314);
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872bee); };
            c.w64(1, t315);
            blk = 932; continue;
        }
        289 => {
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872bf7); };
            c.w64(0, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872bfb); };
            c.w64(1, t317);
            blk = 932; continue;
        }
        290 => {
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c04); };
            c.w64(0, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c08); };
            c.w64(1, t319);
            blk = 932; continue;
        }
        291 => {
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c11); };
            c.w64(0, t320);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c15); };
            c.w64(1, t321);
            blk = 932; continue;
        }
        292 => {
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c1e); };
            c.w64(0, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c22); };
            c.w64(1, t323);
            blk = 932; continue;
        }
        293 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c2b); };
            c.w64(0, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c2f); };
            c.w64(1, t325);
            blk = 932; continue;
        }
        294 => {
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c38); };
            c.w64(0, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c3c); };
            c.w64(1, t327);
            blk = 932; continue;
        }
        295 => {
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c45); };
            c.w64(0, t328);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c49); };
            c.w64(1, t329);
            blk = 932; continue;
        }
        296 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c52); };
            c.w64(0, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c56); };
            c.w64(1, t331);
            blk = 932; continue;
        }
        297 => {
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c5f); };
            c.w64(0, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c63); };
            c.w64(1, t333);
            blk = 932; continue;
        }
        298 => {
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872c6c); };
            c.w64(0, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000872c70); };
            c.w64(1, t335);
            blk = 932; continue;
        }
        299 => {
            let Some(t336) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872c79); };
            c.w32(12, t336);
            let r337 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r337);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t338) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000872c86); };
            c.w64(12, t338);
            blk = 301; continue;
        }
        301 => {
            let Some(t339) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000872c8d); };
            c.w32(15, t339);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t340) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872c95); };
            c.w32(2, t340);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r341 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r341);
            let r342 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r342);
            c.w64(7, c.r64(2));
            let r343 = c.r64(7) & c.r64(15);
            c.w64(7, r343);
            c.logic(r343, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000872ca9); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(2)) { return RegfileExit::fault(0x7000872cad); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 303; continue;
        }
        302 => {
            let r344 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r344);
            c.w64(0, c.r64(3).wrapping_add(c.r64(7)));
            c.w64(2, c.r64(15));
            let r345 = c.r64(2) & c.r64(0);
            c.w64(2, r345);
            c.logic(r345, 64);
            c.w64(7, c.r64(2));
            blk = 303; continue;
        }
        303 => {
            c.w64(6, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(0, c.r64(6).wrapping_mul(4));
            let Some(t346) = c.ld32(c.r64(12).wrapping_add(c.r64(0)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000872cd3); };
            c.w32(2, t346);
            let Some(t347) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000872cd8); };
            c.cmp(t347, c.r32(2), 32);
            if c.cc_e() { blk = 327; continue; }
            blk = 304; continue;
        }
        304 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ne() { blk = 302; continue; }
            blk = 305; continue;
        }
        305 => {
            let Some(t348) = c.ld32(c.r64(12).wrapping_add(c.r64(0)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000872cea); };
            c.w32(8, t348);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 306; continue;
        }
        306 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 309; continue; }
            blk = 307; continue;
        }
        307 => {
            c.w64(8, c.r64(6).wrapping_add(0x1));
            let Some(t349) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872d03); };
            c.w32(11, t349);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 943; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t350) = c.ld32(c.r64(12).wrapping_add(c.r64(0)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000872d17); };
            c.w32(8, t350);
            let r351 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r351);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872d1f); };
            c.w64(0, t352);
            blk = 412; continue;
        }
        309 => {
            let Some(t353) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000872d28); };
            c.cmp(t353, c.r32(9), 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t354) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000872d35); };
            c.cmp(t354, c.r32(9), 32);
            if c.cc_e() { blk = 325; continue; }
            blk = 311; continue;
        }
        311 => {
            let Some(t355) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000872d42); };
            c.cmp(t355, c.r32(9), 32);
            if c.cc_e() { blk = 324; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000872d4f); };
            c.cmp(t356, c.r32(9), 32);
            if c.cc_e() { blk = 323; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000872d5c); };
            c.cmp(t357, c.r32(9), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t358) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000872d65); };
            c.cmp(t358, c.r32(9), 32);
            if c.cc_e() { blk = 321; continue; }
            blk = 315; continue;
        }
        315 => {
            let Some(t359) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000872d6e); };
            c.cmp(t359, c.r32(9), 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t360) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872d77); };
            c.w32(8, t360);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(6).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 944; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(8, c.r64(6).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000872d97); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r361 = c.r64(9) & c.r64(12);
            c.w64(9, r361);
            c.logic(r361, 64);
            let Some(t362) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000872daa); };
            c.logic(t362 & 0x4u64, 8);
            if c.cc_ne() { blk = 1092; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        320 => {
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872dbd); };
            c.w64(0, t363);
            blk = 932; continue;
        }
        321 => {
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872dc6); };
            c.w64(0, t364);
            blk = 932; continue;
        }
        322 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872dcf); };
            c.w64(0, t365);
            blk = 932; continue;
        }
        323 => {
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872dd8); };
            c.w64(0, t366);
            blk = 932; continue;
        }
        324 => {
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872de1); };
            c.w64(0, t367);
            blk = 932; continue;
        }
        325 => {
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872dea); };
            c.w64(0, t368);
            blk = 932; continue;
        }
        326 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872df3); };
            c.w64(0, t369);
            blk = 932; continue;
        }
        327 => {
            let Some(t370) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000872dfc); };
            c.cmp(t370, 0x82eu64, 16);
            if c.cc_e() { blk = 474; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t371) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000872e09); };
            c.cmp(t371, c.r32(9), 32);
            if c.cc_e() { blk = 473; continue; }
            blk = 329; continue;
        }
        329 => {
            let Some(t372) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000872e16); };
            c.cmp(t372, c.r32(9), 32);
            if c.cc_e() { blk = 472; continue; }
            blk = 330; continue;
        }
        330 => {
            let Some(t373) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000872e23); };
            c.cmp(t373, c.r32(9), 32);
            if c.cc_e() { blk = 471; continue; }
            blk = 331; continue;
        }
        331 => {
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000872e30); };
            c.cmp(t374, c.r32(9), 32);
            if c.cc_e() { blk = 470; continue; }
            blk = 332; continue;
        }
        332 => {
            let Some(t375) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000872e3d); };
            c.cmp(t375, c.r32(9), 32);
            if c.cc_e() { blk = 469; continue; }
            blk = 333; continue;
        }
        333 => {
            let Some(t376) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000872e4a); };
            c.cmp(t376, c.r32(9), 32);
            if c.cc_e() { blk = 468; continue; }
            blk = 334; continue;
        }
        334 => {
            let Some(t377) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000872e57); };
            c.cmp(t377, c.r32(9), 32);
            if c.cc_e() { blk = 467; continue; }
            blk = 335; continue;
        }
        335 => {
            let Some(t378) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872e64); };
            c.w32(15, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000872e68); };
            c.cmp(t379, c.r32(15), 32);
            if c.cc_ne() { blk = 339; continue; }
            blk = 336; continue;
        }
        336 => {
            let Some(t380) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000872e71); };
            c.w32(15, t380);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 339; continue; }
            blk = 337; continue;
        }
        337 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_e() { blk = 341; continue; }
            blk = 338; continue;
        }
        338 => {
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872e81); };
            c.w64(0, t381);
            blk = 932; continue;
        }
        339 => {
            c.logic(c.r32(11) & 0x8000000u64, 32);
            if c.cc_ne() { blk = 341; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000872e93); };
            c.w64(0, t382);
            blk = 932; continue;
        }
        341 => {
            let Some(t383) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000872e9c); };
            c.w32(15, t383);
            let r384 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r384);
            let Some(t385) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872ea3); };
            c.w32(0, t385);
            let r386 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000872eaa); };
            c.w64(3, t387);
            blk = 343; continue;
        }
        342 => {
            let Some(t388) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000872eb0); };
            c.w32(0, t388);
            let r389 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r389);
            blk = 343; continue;
        }
        343 => {
            let Some(t390) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000872eb7); };
            c.cmp(t390, c.r32(15), 32);
            if c.cc_e() { blk = 439; continue; }
            blk = 344; continue;
        }
        344 => {
            let Some(t391) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000872ec4); };
            c.w32(2, t391);
            c.cmp(c.r16(2), 0x410u64, 16);
            if c.cc_be() { blk = 373; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t392) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000872ed3); };
            c.w32(6, t392);
            c.logic(c.r32(6) & 0x200000u64, 32);
            if c.cc_ne() { blk = 366; continue; }
            blk = 346; continue;
        }
        346 => {
            let Some(t393) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000872ee2); };
            c.w32(7, t393);
            let r394 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r394);
            c.logic(c.r32(6) & 0xffc00u64, 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 347; continue;
        }
        347 => {
            let r395 = c.shr(c.r32(6), 0xau64, 32).unwrap_or(c.r32(6));
            c.w32(6, r395);
            let r396 = c.r32(6) & 0x3ffu64;
            c.w32(6, r396);
            c.logic(r396, 32);
            c.cmp(c.r32(6), 0x20u64, 32);
            if c.cc_be() { blk = 362; continue; }
            blk = 348; continue;
        }
        348 => {
            let Some(t397) = c.ld16(c.r64(7).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000872f06); };
            c.w32(12, (t397 as u16 as i16 as i64 as u64));
            let r398 = c.sub(c.r32(12), 0x1u64, 32);
            c.w32(12, r398);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000872f0f); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 350; continue;
        }
        349 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000872f18); }
            blk = 350; continue;
        }
        350 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000872f1c); };
            c.w64(11, t399);
            let r400 = c.sub(c.r32(11), c.r32(8), 32);
            c.w32(11, r400);
            let r401 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r401);
            let r402 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r402);
            c.w32(1, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t403) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000872f30); };
            c.w32(1, t403);
            let r404 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r404);
            let r405 = c.shr(c.r32(1), 0x9u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r405);
            let r406 = c.r32(1) & 0x3ffu64;
            c.w32(1, r406);
            c.logic(r406, 32);
            c.w32(1, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t407) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000872f45); };
            c.w32(1, t407);
            let r408 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r408);
            let Some(t409) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872f4c); };
            c.w32(1, t409);
            let r410 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r410);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_be() { blk = 352; continue; }
            blk = 351; continue;
        }
        351 => {
            c.w32(8, c.r64(11).wrapping_add(0x1));
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000872f5a); };
            c.w64(11, t411);
            blk = 352; continue;
        }
        352 => {
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_ne() { blk = 349; continue; }
            blk = 353; continue;
        }
        353 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000872f63); }
            blk = 355; continue;
        }
        354 => {
            let r412 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r412);
            blk = 355; continue;
        }
        355 => {
            c.cmp(c.r32(12), c.r32(8), 32);
            if c.cc_l() { blk = 361; continue; }
            blk = 356; continue;
        }
        356 => {
            c.w32(11, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t413) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000872f79); };
            c.w32(11, t413);
            let r414 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r414);
            let r415 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r415);
            let r416 = c.r32(11) & 0x3ffu64;
            c.w32(11, r416);
            c.logic(r416, 32);
            c.w32(1, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t417) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000872f93); };
            c.w32(0, t417);
            let r418 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r418);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x7000872f9a); }
            let Some(t419) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872f9e); };
            c.w32(1, t419);
            let r420 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r420);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_ne() { blk = 360; continue; }
            blk = 357; continue;
        }
        357 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 354; continue; }
            blk = 358; continue;
        }
        358 => {
            c.cmp(c.r32(6), c.r32(11), 32);
            if c.cc_a() { blk = 387; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000872fb6); };
            c.w64(0, t421);
            blk = 384; continue;
        }
        360 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000872fbf); };
            c.w64(0, t422);
            blk = 384; continue;
        }
        361 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000872fc8); };
            c.w64(0, t423);
            blk = 384; continue;
        }
        362 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            let r424 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(1, r424);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 384; continue; }
            blk = 363; continue;
        }
        363 => {
            let r425 = c.sub(c.r64(1), 0x3u64, 64);
            c.w64(1, r425);
            let Some(t426) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000872fe6); };
            c.cmp(t426, c.r32(9), 32);
            if c.cc_e() { blk = 388; continue; }
            blk = 364; continue;
        }
        364 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ne() { blk = 363; continue; }
            blk = 365; continue;
        }
        365 => {
            blk = 384; continue;
        }
        366 => {
            let Some(t427) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000872ffc); };
            c.w32(8, t427);
            let r428 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r428);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 368; continue; }
            blk = 367; continue;
        }
        367 => {
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000873009); };
            c.w64(8, t429);
            blk = 368; continue;
        }
        368 => {
            let Some(t430) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873010); };
            c.w32(11, t430);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r431 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r431);
            c.w64(12, c.r64(3));
            let r432 = c.r64(12) & c.r64(11);
            c.w64(12, r432);
            c.logic(r432, 64);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 370; continue;
        }
        369 => {
            let r433 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r433);
            let r434 = c.add(c.r64(12), c.r64(1), 64);
            c.w64(12, r434);
            c.w64(6, c.r64(11));
            let r435 = c.r64(6) & c.r64(12);
            c.w64(6, r435);
            c.logic(r435, 64);
            c.w64(12, c.r64(6));
            blk = 370; continue;
        }
        370 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(7, c.r64(6).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x7000873045); }
            let Some(t436) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000873049); };
            c.w32(6, t436);
            let Some(t437) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087304e); };
            c.cmp(t437, c.r32(6), 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 371; continue;
        }
        371 => {
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_e() { blk = 404; continue; }
            blk = 372; continue;
        }
        372 => {
            blk = 369; continue;
        }
        373 => {
            c.cmp(c.r16(2), 0xbdu64, 16);
            if c.cc_ne() { blk = 408; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t438) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000873071); };
            c.logic(t438 & 0x24u64, 8);
            if c.cc_ne() { blk = 438; continue; }
            blk = 375; continue;
        }
        375 => {
            let Some(t439) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087307b); };
            c.w32(8, t439);
            let r440 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r440);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000873082); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 377; continue; }
            blk = 376; continue;
        }
        376 => {
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700087308c); };
            c.w64(8, t441);
            blk = 377; continue;
        }
        377 => {
            let Some(t442) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873093); };
            c.w32(11, t442);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r443 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r443);
            c.w64(12, c.r64(3));
            let r444 = c.r64(12) & c.r64(11);
            c.w64(12, r444);
            c.logic(r444, 64);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 379; continue;
        }
        378 => {
            let r445 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r445);
            let r446 = c.add(c.r64(12), c.r64(1), 64);
            c.w64(12, r446);
            c.w64(15, c.r64(11));
            let r447 = c.r64(15) & c.r64(12);
            c.w64(15, r447);
            c.logic(r447, 64);
            c.w64(12, c.r64(15));
            blk = 379; continue;
        }
        379 => {
            c.w64(6, c.r64(12).wrapping_add(0x5));
            c.w64(7, c.r64(6).wrapping_mul(4));
            let Some(t448) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008730c8); };
            c.w32(15, t448);
            let r449 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r449);
            let Some(t450) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008730d0); };
            c.cmp(t450, c.r32(15), 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 380; continue;
        }
        380 => {
            let Some(t451) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008730d9); };
            c.cmp(t451, c.r32(15), 32);
            if c.cc_e() { blk = 378; continue; }
            blk = 381; continue;
        }
        381 => {
            let Some(t452) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008730e2); };
            c.cmp(t452, c.r32(9), 32);
            if c.cc_ne() { blk = 378; continue; }
            blk = 382; continue;
        }
        382 => {
            let Some(t453) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008730e8); };
            c.w32(11, t453);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(6).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 948; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t454) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008730ff); };
            c.w32(8, t454);
            let r455 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r455);
            let Some(t456) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000873107); };
            c.w32(11, t456);
            let r457 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r457);
            let Some(t458) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087310e); };
            c.cmp(t458, c.r32(11), 32);
            if c.cc_ne() { blk = 409; continue; }
            blk = 384; continue;
        }
        384 => {
            c.cmp(c.r16(2), 0x82eu64, 16);
            if c.cc_e() { blk = 437; continue; }
            blk = 385; continue;
        }
        385 => {
            let Some(t459) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873126); };
            c.w32(15, t459);
            let r460 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r460);
            let Some(t461) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087312d); };
            c.cmp(t461, c.r32(15), 32);
            if c.cc_e() { blk = 439; continue; }
            blk = 386; continue;
        }
        386 => {
            blk = 342; continue;
        }
        387 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087313f); };
            c.w64(1, t462);
            let Some(t463) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000873143); };
            c.w64(0, t463);
            blk = 388; continue;
        }
        388 => {
            c.w64(8, c.r64(1).wrapping_mul(4));
            let Some(t464) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087314f); };
            c.w32(11, t464);
            c.w32(12, c.r32(11));
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 389; continue;
        }
        389 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 439; continue; }
            blk = 390; continue;
        }
        390 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 392; continue; }
            blk = 391; continue;
        }
        391 => {
            let Some(t465) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000873171); };
            c.w32(8, t465);
            let r466 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r466);
            blk = 403; continue;
        }
        392 => {
            let r467 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r467);
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t468) = c.ld8(c.r64(0).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000873185); };
            c.w32(8, t468);
            let r469 = c.r64(11) & 0x3ffu64;
            c.w64(11, r469);
            c.logic(r469, 64);
            let r470 = c.r32(12) & 0x380u64;
            c.w32(12, r470);
            c.logic(r470, 32);
            let r471 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r471);
            c.cmp(c.r32(12), 0x100u64, 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t472) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008731aa); };
            c.w32(12, t472);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 397; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t473) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008731b4); };
            c.w32(0, t473);
            let r474 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r474);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 395; continue; }
            blk = 394; continue;
        }
        394 => {
            let Some(t475) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008731bf); };
            c.w64(0, t475);
            blk = 395; continue;
        }
        395 => {
            let r476 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r476);
            let Some(t477) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008731c9); };
            c.w32(8, t477);
            let r478 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r478);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 403; continue; }
            blk = 396; continue;
        }
        396 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008731d7); }
            blk = 399; continue;
        }
        397 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 402; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t479) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008731e5); };
            c.w32(8, t479);
            let r480 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r480);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008731ed); }
            blk = 399; continue;
        }
        399 => {
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008731f3); };
            c.w64(8, t481);
            let Some(t482) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008731fa); };
            c.w64(11, t482);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t483) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000873201); };
            c.w64(0, t483);
            let Some(t484) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000873208); };
            c.cmp(t484, c.r64(12), 64);
            if c.cc_be() { blk = 945; continue; }
            blk = 400; continue;
        }
        400 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000873211); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 401; continue;
        }
        401 => {
            let Some(t485) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000873218); };
            c.w64(11, t485);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700087321f); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000873223); }
            blk = 403; continue;
        }
        402 => {
            let Some(t486) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087322b); };
            c.w32(8, t486);
            let r487 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r487);
            blk = 403; continue;
        }
        403 => {
            c.w64(0, c.r64(15));
            blk = 412; continue;
        }
        404 => {
            let Some(t488) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873238); };
            c.w32(11, t488);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 405; continue;
        }
        405 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 439; continue; }
            blk = 406; continue;
        }
        406 => {
            let Some(t489) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873251); };
            c.w32(11, t489);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087325b); };
            c.w64(12, t490);
            let r491 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r491);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 947; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t492) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087326c); };
            c.w32(8, t492);
            let r493 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r493);
            c.w64(0, c.r64(15));
            blk = 412; continue;
        }
        408 => {
            let Some(t494) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873279); };
            c.w64(0, t494);
            let Some(t495) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087327d); };
            c.w64(1, t495);
            blk = 932; continue;
        }
        409 => {
            let Some(t496) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000873286); };
            c.w32(8, t496);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 410; continue;
        }
        410 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 439; continue; }
            blk = 411; continue;
        }
        411 => {
            let Some(t497) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087329e); };
            c.w64(0, t497);
            c.w64(8, c.r64(11));
            blk = 412; continue;
        }
        412 => {
            let Some(t498) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008732a5); };
            c.w32(11, t498);
            let Some(t499) = c.ld32(c.r64(13).wrapping_add(0xd60)) else { return RegfileExit::fault(0x70008732a9); };
            c.cmp(t499, c.r32(11), 32);
            if c.cc_ne() { blk = 414; continue; }
            blk = 413; continue;
        }
        413 => {
            let Some(t500) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008732b2); };
            c.w64(0, t500);
            let Some(t501) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008732b6); };
            c.w64(1, t501);
            blk = 932; continue;
        }
        414 => {
            let Some(t502) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008732bf); };
            c.w32(7, t502);
            let r503 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r503);
            let Some(t504) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008732c6); };
            c.w32(8, t504);
            let r505 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r505);
            let Some(t506) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008732cd); };
            c.cmp(t506, c.r32(8), 32);
            if c.cc_e() { blk = 419; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t507) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008732d6); };
            c.logic(t507 & 0x2u64, 8);
            if c.cc_ne() { blk = 417; continue; }
            blk = 416; continue;
        }
        416 => {
            let Some(t508) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008732dd); };
            c.w64(5, t508);
            let Some(t509) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008732e1); };
            c.w64(3, t509);
            let Some(t510) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x70008732e8); };
            c.w64(10, t510);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x70008732ed); }
            let Some(t511) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70008732f2); };
            c.w64(6, t511);
            if !c.st64(c.r64(4).wrapping_add(0x70), 0xbcu64) { return RegfileExit::fault(0x70008732f7); }
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(9)) { return RegfileExit::fault(0x7000873300); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(0)) { return RegfileExit::fault(0x7000873305); }
            c.w32(0, 0x3u64);
            let r512 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r512);
            return RegfileExit::jump(0x700086ed74);
        }
        417 => {
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000873318); };
            if !c.push64(t513) { return RegfileExit::fault(0x7000873318); }
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087331b); };
            if !c.push64(t514) { return RegfileExit::fault(0x700087331b); }
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087331e); };
            c.w64(6, t515);
            c.w32(0, 0x1u64);
            return c.call(0x70007f21d4, 0x700087332c, 0x7000873327);
        }
        418 => {
            let Some(t516) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087332c); };
            c.w64(0, t516);
            blk = 166; continue;
        }
        419 => {
            let Some(t517) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873335); };
            c.w64(0, t517);
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000873339); };
            c.w64(1, t518);
            blk = 932; continue;
        }
        420 => {
            let Some(t519) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873342); };
            c.w64(8, t519);
            let Some(t520) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000873346); };
            c.w32(11, t520);
            let r521 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r521);
            let Some(t522) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087334d); };
            c.cmp(t522, c.r32(11), 32);
            if c.cc_e() { blk = 949; continue; }
            blk = 421; continue;
        }
        421 => {
            let Some(t523) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087335a); };
            c.w32(12, t523);
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 433; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t524) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087336a); };
            c.w32(11, t524);
            let r525 = c.r32(11) & 0x12u64;
            c.w32(11, r525);
            c.logic(r525, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 432; continue; }
            blk = 423; continue;
        }
        423 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 425; continue; }
            blk = 424; continue;
        }
        424 => {
            let Some(t526) = c.ld64(c.r64(13).wrapping_add(0xb30)) else { return RegfileExit::fault(0x700087337f); };
            c.w64(11, t526);
            blk = 434; continue;
        }
        425 => {
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 431; continue; }
            blk = 426; continue;
        }
        426 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 430; continue; }
            blk = 427; continue;
        }
        427 => {
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 429; continue; }
            blk = 428; continue;
        }
        428 => {
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0xaa8)) else { return RegfileExit::fault(0x700087339e); };
            c.w64(11, t527);
            blk = 434; continue;
        }
        429 => {
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x550)) else { return RegfileExit::fault(0x70008733a7); };
            c.w64(11, t528);
            blk = 434; continue;
        }
        430 => {
            let Some(t529) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x70008733b0); };
            c.w64(11, t529);
            blk = 434; continue;
        }
        431 => {
            let Some(t530) = c.ld64(c.r64(13).wrapping_add(0x918)) else { return RegfileExit::fault(0x70008733b9); };
            c.w64(11, t530);
            blk = 434; continue;
        }
        432 => {
            let Some(t531) = c.ld64(c.r64(13).wrapping_add(0x730)) else { return RegfileExit::fault(0x70008733c2); };
            c.w64(11, t531);
            blk = 434; continue;
        }
        433 => {
            let Some(t532) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008733cb); };
            c.w32(11, t532);
            let r533 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r533);
            blk = 434; continue;
        }
        434 => {
            c.w32(12, 0x15au64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008733d8); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008733da); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008733dc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008733de); }
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008733e0); };
            c.w64(3, t534);
            c.w32(0, 0x4u64);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008733ec); };
            c.w64(6, t535);
            return c.call(0x700086ed74, 0x70008733f5, 0x70008733f0);
        }
        435 => {
            return RegfileExit::fault(0x70008733f5);
        }
        436 => {
            blk = 439; continue;
        }
        437 => {
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008733f8); };
            c.w64(0, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008733fc); };
            c.w64(1, t537);
            blk = 932; continue;
        }
        438 => {
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873405); };
            c.w64(0, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000873409); };
            c.w64(1, t539);
            blk = 932; continue;
        }
        439 => {
            let Some(t540) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000873412); };
            c.logic(t540 & 0x100000u64, 32);
            if c.cc_e() { blk = 443; continue; }
            blk = 440; continue;
        }
        440 => {
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087341b); };
            c.w64(1, t541);
            let Some(t542) = c.ld32(c.r64(1).wrapping_add(0x23)) else { return RegfileExit::fault(0x700087341f); };
            c.w32(8, t542);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 443; continue; }
            blk = 441; continue;
        }
        441 => {
            c.w64(10, c.r64(4));
            let r543 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r543);
            let r544 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r544);
            c.logic(r544, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000873434); }
            let Some(t545) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x7000873439); };
            c.w64(0, t545);
            c.w64(10, 0x7000873447u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000873447); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700087344b); }
            return c.call(c.r64(0), 0x7000873451, 0x700087344f);
        }
        442 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000873451); }
            let Some(t546) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000873459); };
            c.w64(4, t546);
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087345e); };
            c.w64(9, t547);
            blk = 443; continue;
        }
        443 => {
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000873462); };
            c.w64(12, t548);
            let Some(t549) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873466); };
            c.w32(8, t549);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t550) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087346e); };
            c.w32(11, t550);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 950; continue; }
            blk = 444; continue;
        }
        444 => {
            let Some(t551) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000873480); };
            c.w32(15, t551);
            let r552 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r552);
            c.w64(0, c.r64(15));
            let r553 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r553);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r554 = c.r64(0) & 0xfffffffffffffffeu64;
            c.w64(0, r554);
            c.logic(r554, 64);
            let r555 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r555);
            c.w32(3, c.r32(11));
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_b() { blk = 466; continue; }
            blk = 445; continue;
        }
        445 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 951; continue; }
            blk = 446; continue;
        }
        446 => {
            let r556 = c.sub(c.r32(3), c.r32(15), 32);
            c.w32(3, r556);
            let r557 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r557);
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t558) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008734b6); };
            c.w32(0, t558);
            let r559 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r559);
            c.logic(r559, 64);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_b() { blk = 466; continue; }
            blk = 447; continue;
        }
        447 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 952; continue; }
            blk = 448; continue;
        }
        448 => {
            let Some(t560) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008734d2); };
            c.w32(8, t560);
            let r561 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r561);
            c.w32(11, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(11), 0xfffffeu64, 32);
            if c.cc_a() { blk = 466; continue; }
            blk = 449; continue;
        }
        449 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70008734ee); }
            let Some(t562) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008734f3); };
            c.w32(11, t562);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 953; continue; }
            blk = 450; continue;
        }
        450 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700087350b); }
            let Some(t563) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873510); };
            c.w32(11, t563);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r564 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r564);
            let Some(t565) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087351f); };
            c.w32(15, t565);
            let r566 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r566);
            let r567 = c.r64(15) & c.r64(11);
            c.w64(15, r567);
            c.logic(r567, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 452; continue;
        }
        451 => {
            let r568 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r568);
            let r569 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r569);
            c.w64(3, c.r64(11));
            let r570 = c.r64(3) & c.r64(15);
            c.w64(3, r570);
            c.logic(r570, 64);
            c.w64(15, c.r64(3));
            blk = 452; continue;
        }
        452 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            c.w64(1, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t571) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000873553); };
            c.w32(6, t571);
            let Some(t572) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000873558); };
            c.cmp(t572, c.r32(6), 32);
            if c.cc_e() { blk = 454; continue; }
            blk = 453; continue;
        }
        453 => {
            let Some(t573) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000873561); };
            c.cmp(t573, c.r32(6), 32);
            if c.cc_ne() { blk = 451; continue; }
            blk = 454; continue;
        }
        454 => {
            let Some(t574) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087356a); };
            c.w32(11, t574);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 954; continue; }
            blk = 455; continue;
        }
        455 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x700087357e); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 457; continue; }
            blk = 456; continue;
        }
        456 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r575 = c.r64(11) & c.r64(12);
            c.w64(11, r575);
            c.logic(r575, 64);
            let Some(t576) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873592); };
            c.logic(t576 & 0x4u64, 8);
            if c.cc_ne() { blk = 1089; continue; }
            blk = 457; continue;
        }
        457 => {
            let Some(t577) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087359d); };
            c.w32(11, t577);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ae() { blk = 955; continue; }
            blk = 458; continue;
        }
        458 => {
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            c.w32(11, c.r32(8));
            c.w64(15, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008735c4); };
            c.w64(0, t578);
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008735c8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 460; continue; }
            blk = 459; continue;
        }
        459 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r579 = c.r64(1) & c.r64(12);
            c.w64(1, r579);
            c.logic(r579, 64);
            let Some(t580) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008735da); };
            c.logic(t580 & 0x4u64, 8);
            if c.cc_ne() { blk = 1086; continue; }
            blk = 460; continue;
        }
        460 => {
            let Some(t581) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008735e4); };
            c.w32(15, t581);
            let Some(t582) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008735e8); };
            c.cmp(t582, c.r32(15), 32);
            if c.cc_ne() { blk = 463; continue; }
            blk = 461; continue;
        }
        461 => {
            let Some(t583) = c.ld8(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008735f1); };
            c.logic(t583 & 0x1u64, 8);
            if c.cc_e() { blk = 463; continue; }
            blk = 462; continue;
        }
        462 => {
            let r584 = c.r32(11) | 0x20u64;
            c.w32(11, r584);
            c.logic(r584, 32);
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 464; continue;
        }
        463 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 464; continue;
        }
        464 => {
            let Some(t585) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873604); };
            c.w32(9, t585);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(3).wrapping_add(0x2));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 956; continue; }
            blk = 465; continue;
        }
        465 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700087361c); }
            blk = 166; continue;
        }
        466 => {
            let Some(t586) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000873626); };
            c.w64(5, t586);
            let Some(t587) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x700087362a); };
            c.w64(3, t587);
            c.w32(0, 0x3u64);
            let Some(t588) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x7000873636); };
            c.w64(10, t588);
            let Some(t589) = c.ld64(c.r64(4).wrapping_add(0x58)) else { return RegfileExit::fault(0x700087363b); };
            if !c.push64(t589) { return RegfileExit::fault(0x700087363b); }
            let sp590 = c.r64(4);
            let Some(pv591) = c.ld64(sp590) else { return RegfileExit::fault(0x700087363f); };
            c.set_rsp(sp590.wrapping_add(8));
            let pea592 = c.r64(4).wrapping_add(0x70);
            if !c.st64(pea592, pv591) { c.set_rsp(sp590); return RegfileExit::fault(0x700087363f); }
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000873643); }
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(9)) { return RegfileExit::fault(0x7000873648); }
            let Some(t593) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x700087364d); };
            c.w64(10, t593);
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(10)) { return RegfileExit::fault(0x7000873652); }
            let Some(t594) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x7000873657); };
            c.w64(6, t594);
            let r595 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r595);
            return RegfileExit::jump(0x700086ed74);
        }
        467 => {
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873665); };
            c.w64(0, t596);
            blk = 932; continue;
        }
        468 => {
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087366e); };
            c.w64(0, t597);
            blk = 932; continue;
        }
        469 => {
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873677); };
            c.w64(0, t598);
            blk = 932; continue;
        }
        470 => {
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873680); };
            c.w64(0, t599);
            blk = 932; continue;
        }
        471 => {
            let Some(t600) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873689); };
            c.w64(0, t600);
            blk = 932; continue;
        }
        472 => {
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000873692); };
            c.w64(0, t601);
            blk = 932; continue;
        }
        473 => {
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087369b); };
            c.w64(0, t602);
            blk = 932; continue;
        }
        474 => {
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008736a4); };
            c.w64(0, t603);
            blk = 932; continue;
        }
        475 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11));
            blk = 476; continue;
        }
        476 => {
            let Some(t604) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008736b6); };
            c.w32(11, t604);
            c.w64(15, c.r64(11));
            let r605 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r605);
            let Some(t606) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008736c2); };
            c.w32(2, t606);
            let r607 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r607);
            let r608 = c.r32(15) & 0x3fu64;
            c.w32(15, r608);
            c.logic(r608, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008736cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008736d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008736d4); }
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_be() { blk = 479; continue; }
            blk = 477; continue;
        }
        477 => {
            c.cmp(c.r32(15), 0x11u64, 32);
            if c.cc_ge() { blk = 932; continue; }
            blk = 478; continue;
        }
        478 => {
            let r609 = c.r32(11) & 0xfcu64;
            c.w32(11, r609);
            c.logic(r609, 32);
            c.cmp(c.r32(11), 0x30u64, 32);
            blk = 932; continue;
        }
        479 => {
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 482; continue; }
            blk = 480; continue;
        }
        480 => {
            let Some(t610) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873700); };
            c.w32(9, t610);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 806; continue; }
            blk = 481; continue;
        }
        481 => {
            blk = 932; continue;
        }
        482 => {
            let Some(t611) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000873718); };
            c.w32(9, t611);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 806; continue; }
            blk = 483; continue;
        }
        483 => {
            let Some(t612) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087372b); };
            c.w32(7, t612);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(7));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000873737); }
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_be() { blk = 932; continue; }
            blk = 484; continue;
        }
        484 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 674; continue; }
            blk = 485; continue;
        }
        485 => {
            let Some(t613) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087374d); };
            c.logic(t613 & 0x200000u64, 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 486; continue;
        }
        486 => {
            let Some(t614) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087375b); };
            c.w32(9, t614);
            let r615 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r615);
            let Some(t616) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000873762); };
            c.w32(9, t616);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 932; continue; }
            blk = 487; continue;
        }
        487 => {
            let Some(t617) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000873770); };
            c.w32(9, t617);
            let Some(t618) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000873774); };
            c.cmp(t618, c.r32(9), 32);
            if c.cc_ne() { blk = 604; continue; }
            blk = 488; continue;
        }
        488 => {
            c.w64(9, c.r64(8));
            blk = 489; continue;
        }
        489 => {
            let Some(t619) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873784); };
            c.w32(9, t619);
            let r620 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r620);
            let Some(t621) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087378b); };
            c.cmp(t621, c.r32(9), 32);
            if c.cc_e() { blk = 494; continue; }
            blk = 490; continue;
        }
        490 => {
            let Some(t622) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000873794); };
            c.w32(9, t622);
            let r623 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r623);
            let Some(t624) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087379b); };
            c.cmp(t624, 0x411u64, 16);
            if c.cc_be() { blk = 932; continue; }
            blk = 491; continue;
        }
        491 => {
            let Some(t625) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008737a8); };
            c.w32(11, t625);
            c.w64(7, c.r64(11));
            let r626 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r626);
            let r627 = c.r32(7) & 0x3fu64;
            c.w32(7, r627);
            c.logic(r627, 32);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_be() { blk = 489; continue; }
            blk = 492; continue;
        }
        492 => {
            let r628 = c.r32(11) & 0xfcu64;
            c.w32(11, r628);
            c.logic(r628, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 489; continue; }
            blk = 493; continue;
        }
        493 => {
            blk = 932; continue;
        }
        494 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 574; continue; }
            blk = 495; continue;
        }
        495 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_le() { blk = 513; continue; }
            blk = 496; continue;
        }
        496 => {
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008737e3); };
            c.w64(11, t629);
            let r630 = c.r32(11) & 0xfcu64;
            c.w32(11, r630);
            c.logic(r630, 32);
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_e() { blk = 510; continue; }
            blk = 497; continue;
        }
        497 => {
            let Some(t631) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008737f8); };
            c.w32(9, t631);
            let r632 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r632);
            let Some(t633) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008737ff); };
            c.w32(9, t633);
            let r634 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r634);
            let Some(t635) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873806); };
            c.w32(11, t635);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 499; continue; }
            blk = 498; continue;
        }
        498 => {
            return RegfileExit::fault(0x7000873816);
        }
        499 => {
            let Some(t636) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000873817); };
            c.cmp(t636, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 500; continue;
        }
        500 => {
            c.w64(8, c.r64(0));
            let r637 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r637);
            c.logic(r637, 64);
            let Some(t638) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087382e); };
            c.w64(15, t638);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 507; continue; }
            blk = 501; continue;
        }
        501 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 507; continue; }
            blk = 502; continue;
        }
        502 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t639) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873842); };
            c.w64(3, t639);
            let Some(t640) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000873849); };
            c.w64(3, t640);
            c.w64(7, c.r64(15));
            let r641 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r641);
            c.logic(r641, 64);
            c.w64(6, c.r64(3));
            let r642 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r642);
            c.logic(r642, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 505; continue; }
            blk = 503; continue;
        }
        503 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 506; continue; }
            blk = 504; continue;
        }
        504 => {
            blk = 507; continue;
        }
        505 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_be() { blk = 507; continue; }
            blk = 506; continue;
        }
        506 => {
            let Some(t643) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873871); };
            c.w32(8, t643);
            let Some(t644) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000873875); };
            c.cmp(t644, c.r32(8), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 507; continue;
        }
        507 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 509; continue; }
            blk = 508; continue;
        }
        508 => {
            return RegfileExit::fault(0x7000873888);
        }
        509 => {
            let Some(t645) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000873889); };
            c.w32(8, t645);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000873890); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r646 = c.r64(9) & c.r64(0);
            c.w64(9, r646);
            c.logic(r646, 64);
            let Some(t647) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087389e); };
            c.logic(t647 & 0x4u64, 8);
            if c.cc_ne() { blk = 1083; continue; }
            blk = 510; continue;
        }
        510 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(9, c.r64(12).wrapping_add(0x1));
            let r648 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r648);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008738b9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 512; continue; }
            blk = 511; continue;
        }
        511 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r649 = c.r64(11) & c.r64(2);
            c.w64(11, r649);
            c.logic(r649, 64);
            let Some(t650) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008738cc); };
            c.logic(t650 & 0x4u64, 8);
            if c.cc_ne() { blk = 1080; continue; }
            blk = 512; continue;
        }
        512 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008738d7); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        513 => {
            let Some(t651) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008738e3); };
            c.w32(9, t651);
            let r652 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r652);
            let Some(t653) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008738ea); };
            c.w32(11, t653);
            let Some(t654) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008738ee); };
            c.w32(9, t654);
            let r655 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r655);
            let Some(t656) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008738f5); };
            c.cmp(t656, c.r32(11), 32);
            if c.cc_ne() { blk = 551; continue; }
            blk = 514; continue;
        }
        514 => {
            c.w64(11, c.r64(0));
            let r657 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r657);
            c.logic(r657, 64);
            let Some(t658) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087390c); };
            c.w64(15, t658);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 522; continue; }
            blk = 515; continue;
        }
        515 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 522; continue; }
            blk = 516; continue;
        }
        516 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t659) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873920); };
            c.w64(7, t659);
            let Some(t660) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000873927); };
            c.w64(7, t660);
            c.w64(6, c.r64(15));
            let r661 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r661);
            c.logic(r661, 64);
            c.w64(1, c.r64(7));
            let r662 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r662);
            c.logic(r662, 64);
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_e() { blk = 519; continue; }
            blk = 517; continue;
        }
        517 => {
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_e() { blk = 520; continue; }
            blk = 518; continue;
        }
        518 => {
            blk = 522; continue;
        }
        519 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 522; continue; }
            blk = 520; continue;
        }
        520 => {
            let Some(t663) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087394f); };
            c.w32(11, t663);
            let Some(t664) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000873953); };
            c.cmp(t664, c.r32(11), 32);
            if c.cc_ne() { blk = 522; continue; }
            blk = 521; continue;
        }
        521 => {
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087395c); };
            c.w64(1, t665);
            blk = 932; continue;
        }
        522 => {
            let Some(t666) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873965); };
            c.w32(11, t666);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 524; continue; }
            blk = 523; continue;
        }
        523 => {
            return RegfileExit::fault(0x7000873975);
        }
        524 => {
            let Some(t667) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000873976); };
            c.cmp(t667, c.r32(8), 32);
            if c.cc_ne() { blk = 528; continue; }
            blk = 525; continue;
        }
        525 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 527; continue; }
            blk = 526; continue;
        }
        526 => {
            return RegfileExit::fault(0x7000873985);
        }
        527 => {
            let Some(t668) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000873986); };
            c.w32(8, t668);
            blk = 534; continue;
        }
        528 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 530; continue; }
            blk = 529; continue;
        }
        529 => {
            return RegfileExit::fault(0x7000873995);
        }
        530 => {
            let Some(t669) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000873996); };
            c.cmp(t669, c.r32(8), 32);
            if c.cc_ne() { blk = 550; continue; }
            blk = 531; continue;
        }
        531 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 533; continue; }
            blk = 532; continue;
        }
        532 => {
            return RegfileExit::fault(0x70008739a9);
        }
        533 => {
            let Some(t670) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008739aa); };
            c.w32(8, t670);
            blk = 534; continue;
        }
        534 => {
            c.cmp(c.r64(3), 0x3fffu64, 64);
            if c.cc_ae() { blk = 549; continue; }
            blk = 535; continue;
        }
        535 => {
            c.cmp(c.r64(3), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 961; continue; }
            blk = 536; continue;
        }
        536 => {
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008739cb); };
            c.w64(11, t671);
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 957; continue; }
            blk = 537; continue;
        }
        537 => {
            c.w32(15, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t672) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008739e8); };
            c.w64(3, t672);
            let Some(t673) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008739ef); };
            c.w64(1, t673);
            c.w64(6, c.r64(9).wrapping_add(c.r64(1)));
            c.w64(7, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            let Some(t674) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008739fe); };
            c.cmp(t674, c.r64(6), 64);
            if c.cc_be() { blk = 959; continue; }
            blk = 538; continue;
        }
        538 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000873a08); }
            c.w64(9, c.r64(1).wrapping_add(0x1));
            blk = 539; continue;
        }
        539 => {
            let Some(t675) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000873a0f); };
            c.w64(3, t675);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000873a16); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000873a1a); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 545; continue; }
            blk = 540; continue;
        }
        540 => {
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            blk = 541; continue;
        }
        541 => {
            let r676 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r676);
            let Some(t677) = c.ld32(c.r64(7).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000873a30); };
            c.w32(15, t677);
            let r678 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r678);
            let Some(t679) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000873a38); };
            c.cmp(t679, c.r32(15), 32);
            if c.cc_e() { blk = 543; continue; }
            blk = 542; continue;
        }
        542 => {
            let r680 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r680);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(c.r64(9)), 0) { return RegfileExit::fault(0x7000873a4d); }
            blk = 544; continue;
        }
        543 => {
            c.w64(15, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(11).wrapping_add(c.r64(9)), c.r64(15)) { return RegfileExit::fault(0x7000873a5f); }
            blk = 544; continue;
        }
        544 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 541; continue; }
            blk = 545; continue;
        }
        545 => {
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r681 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r681);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000873a71); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 547; continue; }
            blk = 546; continue;
        }
        546 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r682 = c.r64(15) & c.r64(0);
            c.w64(15, r682);
            c.logic(r682, 64);
            let Some(t683) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873a85); };
            c.logic(t683 & 0x4u64, 8);
            if c.cc_ne() { blk = 1077; continue; }
            blk = 547; continue;
        }
        547 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000873a90); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r684 = c.r64(9) & c.r64(0);
            c.w64(9, r684);
            c.logic(r684, 64);
            let Some(t685) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873a9e); };
            c.logic(t685 & 0x4u64, 8);
            if c.cc_ne() { blk = 1074; continue; }
            blk = 548; continue;
        }
        548 => {
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000873aa9); };
            c.w64(1, t686);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000873aad); }
            let Some(t687) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000873ab2); };
            c.w32(8, t687);
            let r688 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r688);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000873ac3); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000873aca); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        549 => {
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000873ad6); };
            c.w64(1, t689);
            blk = 932; continue;
        }
        550 => {
            let Some(t690) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000873adf); };
            c.w64(1, t690);
            blk = 932; continue;
        }
        551 => {
            c.w64(11, c.r64(0));
            let r691 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r691);
            c.logic(r691, 64);
            let Some(t692) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873af2); };
            c.w64(15, t692);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 558; continue; }
            blk = 552; continue;
        }
        552 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 558; continue; }
            blk = 553; continue;
        }
        553 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t693) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873b06); };
            c.w64(3, t693);
            let Some(t694) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000873b0d); };
            c.w64(3, t694);
            c.w64(7, c.r64(15));
            let r695 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r695);
            c.logic(r695, 64);
            c.w64(6, c.r64(3));
            let r696 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r696);
            c.logic(r696, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 556; continue; }
            blk = 554; continue;
        }
        554 => {
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 557; continue; }
            blk = 555; continue;
        }
        555 => {
            blk = 558; continue;
        }
        556 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_be() { blk = 558; continue; }
            blk = 557; continue;
        }
        557 => {
            let Some(t697) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873b35); };
            c.w32(11, t697);
            let Some(t698) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000873b39); };
            c.cmp(t698, c.r32(11), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 558; continue;
        }
        558 => {
            let Some(t699) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873b46); };
            c.w32(11, t699);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 560; continue; }
            blk = 559; continue;
        }
        559 => {
            return RegfileExit::fault(0x7000873b56);
        }
        560 => {
            let Some(t700) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000873b57); };
            c.cmp(t700, c.r32(8), 32);
            if c.cc_ne() { blk = 564; continue; }
            blk = 561; continue;
        }
        561 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 563; continue; }
            blk = 562; continue;
        }
        562 => {
            return RegfileExit::fault(0x7000873b66);
        }
        563 => {
            let Some(t701) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000873b67); };
            c.w32(8, t701);
            blk = 570; continue;
        }
        564 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 566; continue; }
            blk = 565; continue;
        }
        565 => {
            return RegfileExit::fault(0x7000873b76);
        }
        566 => {
            let Some(t702) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000873b77); };
            c.cmp(t702, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 567; continue;
        }
        567 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 569; continue; }
            blk = 568; continue;
        }
        568 => {
            return RegfileExit::fault(0x7000873b8a);
        }
        569 => {
            let Some(t703) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000873b8b); };
            c.w32(8, t703);
            blk = 570; continue;
        }
        570 => {
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r704 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r704);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000873ba2); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r705 = c.r64(12) & c.r64(0);
            c.w64(12, r705);
            c.logic(r705, 64);
            let Some(t706) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873bb0); };
            c.logic(t706 & 0x4u64, 8);
            if c.cc_ne() { blk = 1071; continue; }
            blk = 571; continue;
        }
        571 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x7000873bbc); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 573; continue; }
            blk = 572; continue;
        }
        572 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r707 = c.r64(8) & c.r64(2);
            c.w64(8, r707);
            c.logic(r707, 64);
            let Some(t708) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873bcf); };
            c.logic(t708 & 0x4u64, 8);
            if c.cc_ne() { blk = 1068; continue; }
            blk = 573; continue;
        }
        573 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000873bda); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        574 => {
            let Some(t709) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000873be6); };
            c.w64(11, t709);
            let r710 = c.r32(11) & 0xfcu64;
            c.w32(11, r710);
            c.logic(r710, 32);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 603; continue; }
            blk = 575; continue;
        }
        575 => {
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_e() { blk = 603; continue; }
            blk = 576; continue;
        }
        576 => {
            let Some(t711) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000873c05); };
            c.w32(9, t711);
            let r712 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r712);
            let Some(t713) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000873c0c); };
            c.w32(9, t713);
            let r714 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r714);
            let Some(t715) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873c13); };
            c.w32(11, t715);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 578; continue; }
            blk = 577; continue;
        }
        577 => {
            return RegfileExit::fault(0x7000873c23);
        }
        578 => {
            let Some(t716) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000873c24); };
            c.cmp(t716, c.r32(8), 32);
            if c.cc_ne() { blk = 590; continue; }
            blk = 579; continue;
        }
        579 => {
            c.w64(8, c.r64(0));
            let r717 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r717);
            c.logic(r717, 64);
            let Some(t718) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873c3b); };
            c.w64(15, t718);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 586; continue; }
            blk = 580; continue;
        }
        580 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 586; continue; }
            blk = 581; continue;
        }
        581 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t719) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873c4f); };
            c.w64(3, t719);
            let Some(t720) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000873c56); };
            c.w64(3, t720);
            c.w64(7, c.r64(15));
            let r721 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r721);
            c.logic(r721, 64);
            c.w64(6, c.r64(3));
            let r722 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r722);
            c.logic(r722, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 584; continue; }
            blk = 582; continue;
        }
        582 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 585; continue; }
            blk = 583; continue;
        }
        583 => {
            blk = 586; continue;
        }
        584 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_be() { blk = 586; continue; }
            blk = 585; continue;
        }
        585 => {
            let Some(t723) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873c7e); };
            c.w32(8, t723);
            let Some(t724) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000873c82); };
            c.cmp(t724, c.r32(8), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 586; continue;
        }
        586 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 588; continue; }
            blk = 587; continue;
        }
        587 => {
            return RegfileExit::fault(0x7000873c95);
        }
        588 => {
            let Some(t725) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000873c96); };
            c.w32(8, t725);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000873c9d); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r726 = c.r64(9) & c.r64(0);
            c.w64(9, r726);
            c.logic(r726, 64);
            let Some(t727) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873cab); };
            c.logic(t727 & 0x4u64, 8);
            if c.cc_ne() { blk = 1065; continue; }
            blk = 589; continue;
        }
        589 => {
            blk = 603; continue;
        }
        590 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 592; continue; }
            blk = 591; continue;
        }
        591 => {
            return RegfileExit::fault(0x7000873cc1);
        }
        592 => {
            let Some(t728) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000873cc2); };
            c.cmp(t728, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 593; continue;
        }
        593 => {
            c.w64(8, c.r64(0));
            let r729 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r729);
            c.logic(r729, 64);
            let Some(t730) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873cd9); };
            c.w64(15, t730);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 600; continue; }
            blk = 594; continue;
        }
        594 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 600; continue; }
            blk = 595; continue;
        }
        595 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t731) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873ced); };
            c.w64(3, t731);
            let Some(t732) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000873cf4); };
            c.w64(3, t732);
            c.w64(7, c.r64(15));
            let r733 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r733);
            c.logic(r733, 64);
            c.w64(6, c.r64(3));
            let r734 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r734);
            c.logic(r734, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 598; continue; }
            blk = 596; continue;
        }
        596 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 599; continue; }
            blk = 597; continue;
        }
        597 => {
            blk = 600; continue;
        }
        598 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_be() { blk = 600; continue; }
            blk = 599; continue;
        }
        599 => {
            let Some(t735) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873d1c); };
            c.w32(8, t735);
            let Some(t736) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000873d20); };
            c.cmp(t736, c.r32(8), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 600; continue;
        }
        600 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 602; continue; }
            blk = 601; continue;
        }
        601 => {
            return RegfileExit::fault(0x7000873d33);
        }
        602 => {
            let Some(t737) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000873d34); };
            c.w32(8, t737);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000873d3b); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r738 = c.r64(9) & c.r64(0);
            c.w64(9, r738);
            c.logic(r738, 64);
            let Some(t739) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873d49); };
            c.logic(t739 & 0x4u64, 8);
            if c.cc_ne() { blk = 1062; continue; }
            blk = 603; continue;
        }
        603 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            let r740 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r740);
            if !c.st32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000873d5c); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000873d61); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        604 => {
            let Some(t741) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000873d6d); };
            c.cmp(t741, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 605; continue;
        }
        605 => {
            c.w64(9, c.r64(8));
            blk = 606; continue;
        }
        606 => {
            let Some(t742) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873d7d); };
            c.w32(9, t742);
            let r743 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r743);
            let Some(t744) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000873d84); };
            c.cmp(t744, c.r32(9), 32);
            if c.cc_e() { blk = 611; continue; }
            blk = 607; continue;
        }
        607 => {
            let Some(t745) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000873d8d); };
            c.w32(9, t745);
            let r746 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r746);
            let Some(t747) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000873d94); };
            c.cmp(t747, 0x411u64, 16);
            if c.cc_be() { blk = 932; continue; }
            blk = 608; continue;
        }
        608 => {
            let Some(t748) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000873da1); };
            c.w32(15, t748);
            c.w64(11, c.r64(15));
            let r749 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r749);
            let r750 = c.r32(11) & 0x3fu64;
            c.w32(11, r750);
            c.logic(r750, 32);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_be() { blk = 606; continue; }
            blk = 609; continue;
        }
        609 => {
            let r751 = c.r32(15) & 0xfcu64;
            c.w32(15, r751);
            c.logic(r751, 32);
            c.cmp(c.r32(15), 0xa0u64, 32);
            if c.cc_e() { blk = 606; continue; }
            blk = 610; continue;
        }
        610 => {
            blk = 932; continue;
        }
        611 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 658; continue; }
            blk = 612; continue;
        }
        612 => {
            let Some(t752) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000873dd5); };
            c.w32(9, t752);
            let Some(t753) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000873dd9); };
            c.cmp(t753, c.r32(9), 32);
            if c.cc_ne() { blk = 614; continue; }
            blk = 613; continue;
        }
        613 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000873de2); }
            blk = 659; continue;
        }
        614 => {
            let Some(t754) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000873dec); };
            c.w32(9, t754);
            let r755 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r755);
            c.w64(11, c.r64(0));
            let r756 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r756);
            c.logic(r756, 64);
            let Some(t757) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000873dfd); };
            c.w32(9, t757);
            let r758 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r758);
            let Some(t759) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873e04); };
            c.w64(15, t759);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 622; continue; }
            blk = 615; continue;
        }
        615 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 622; continue; }
            blk = 616; continue;
        }
        616 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t760) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873e18); };
            c.w64(6, t760);
            let Some(t761) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000873e1f); };
            c.w64(6, t761);
            c.w64(1, c.r64(15));
            let r762 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r762);
            c.logic(r762, 64);
            c.w64(12, c.r64(6));
            let r763 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r763);
            c.logic(r763, 64);
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_e() { blk = 619; continue; }
            blk = 617; continue;
        }
        617 => {
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_e() { blk = 620; continue; }
            blk = 618; continue;
        }
        618 => {
            blk = 622; continue;
        }
        619 => {
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_be() { blk = 622; continue; }
            blk = 620; continue;
        }
        620 => {
            let Some(t764) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000873e47); };
            c.w32(11, t764);
            let Some(t765) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000873e4b); };
            c.cmp(t765, c.r32(11), 32);
            if c.cc_ne() { blk = 622; continue; }
            blk = 621; continue;
        }
        621 => {
            let Some(t766) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000873e54); };
            c.w64(1, t766);
            blk = 932; continue;
        }
        622 => {
            let Some(t767) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000873e5d); };
            c.w32(11, t767);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 624; continue; }
            blk = 623; continue;
        }
        623 => {
            return RegfileExit::fault(0x7000873e6d);
        }
        624 => {
            let Some(t768) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000873e6e); };
            c.cmp(t768, c.r32(8), 32);
            if c.cc_ne() { blk = 628; continue; }
            blk = 625; continue;
        }
        625 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 627; continue; }
            blk = 626; continue;
        }
        626 => {
            return RegfileExit::fault(0x7000873e7d);
        }
        627 => {
            let Some(t769) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000873e7e); };
            c.w32(8, t769);
            blk = 634; continue;
        }
        628 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 630; continue; }
            blk = 629; continue;
        }
        629 => {
            return RegfileExit::fault(0x7000873e8d);
        }
        630 => {
            let Some(t770) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000873e8e); };
            c.cmp(t770, c.r32(8), 32);
            if c.cc_ne() { blk = 657; continue; }
            blk = 631; continue;
        }
        631 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 633; continue; }
            blk = 632; continue;
        }
        632 => {
            return RegfileExit::fault(0x7000873ea1);
        }
        633 => {
            let Some(t771) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000873ea2); };
            c.w32(8, t771);
            blk = 634; continue;
        }
        634 => {
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 656; continue; }
            blk = 635; continue;
        }
        635 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 967; continue; }
            blk = 636; continue;
        }
        636 => {
            c.w64(9, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            let Some(t772) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873ecb); };
            c.w64(11, t772);
            let Some(t773) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000873ed2); };
            c.w64(12, t773);
            c.w64(15, c.r64(9).wrapping_add(c.r64(12)));
            let Some(t774) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000873ed9); };
            c.w64(1, t774);
            let Some(t775) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000873ee0); };
            c.cmp(t775, c.r64(15), 64);
            if c.cc_be() { blk = 963; continue; }
            blk = 637; continue;
        }
        637 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000873ee9); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 638; continue;
        }
        638 => {
            c.w32(11, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t776) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000873ef5); };
            c.w64(12, t776);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000873efc); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000873f00); }
            c.w64(11, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 641; continue; }
            blk = 639; continue;
        }
        639 => {
            let Some(t777) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000873f12); };
            c.w64(12, t777);
            blk = 640; continue;
        }
        640 => {
            let r778 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r778);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x7000873f1d); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 640; continue; }
            blk = 641; continue;
        }
        641 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 650; continue; }
            blk = 642; continue;
        }
        642 => {
            c.w64(12, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t779) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x7000873f41); };
            c.w64(15, t779);
            blk = 644; continue;
        }
        643 => {
            let r780 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r780);
            let Some(t781) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000873f4e); };
            c.w64(2, t781);
            blk = 644; continue;
        }
        644 => {
            let r782 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r782);
            let Some(t783) = c.ld64(c.r64(11).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000873f56); };
            c.cmp(t783, c.r64(15), 64);
            if c.cc_e() { blk = 649; continue; }
            blk = 645; continue;
        }
        645 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(2))) { return RegfileExit::fault(0x7000873f5c); }
            let Some(t784) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000873f62); };
            c.w64(3, t784);
            let Some(t785) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000873f69); };
            c.w64(1, t785);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            c.w64(7, c.r64(12).wrapping_sub(0x4));
            let Some(t786) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000873f75); };
            c.w64(2, t786);
            let Some(t787) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000873f7c); };
            c.cmp(t787, c.r64(6), 64);
            if c.cc_be() { blk = 965; continue; }
            blk = 646; continue;
        }
        646 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000873f85); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 647; continue;
        }
        647 => {
            let Some(t788) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000873f8c); };
            c.w64(2, t788);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000873f93); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000873f96); }
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(3)) { return RegfileExit::fault(0x7000873f9b); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 649; continue; }
            blk = 648; continue;
        }
        648 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r789 = c.r64(2) & c.r64(9);
            c.w64(2, r789);
            c.logic(r789, 64);
            let Some(t790) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873fae); };
            c.logic(t790 & 0x4u64, 8);
            if c.cc_ne() { blk = 1059; continue; }
            blk = 649; continue;
        }
        649 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 643; continue; }
            blk = 650; continue;
        }
        650 => {
            let Some(t791) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000873fbe); };
            c.w64(12, t791);
            c.w64(11, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let r792 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r792);
            let r793 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r793);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000873fd1); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 652; continue; }
            blk = 651; continue;
        }
        651 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r794 = c.r64(15) & c.r64(0);
            c.w64(15, r794);
            c.logic(r794, 64);
            let Some(t795) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873fe5); };
            c.logic(t795 & 0x4u64, 8);
            if c.cc_ne() { blk = 1056; continue; }
            blk = 652; continue;
        }
        652 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000873ff0); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r796 = c.r64(9) & c.r64(0);
            c.w64(9, r796);
            c.logic(r796, 64);
            let Some(t797) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000873ffe); };
            c.logic(t797 & 0x4u64, 8);
            if c.cc_ne() { blk = 1053; continue; }
            blk = 653; continue;
        }
        653 => {
            let Some(t798) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000874009); };
            c.w32(8, t798);
            let r799 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r799);
            let Some(t800) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874010); };
            c.w64(9, t800);
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)), c.r32(9)) { return RegfileExit::fault(0x7000874014); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 655; continue; }
            blk = 654; continue;
        }
        654 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r801 = c.r64(15) & c.r64(8);
            c.w64(15, r801);
            c.logic(r801, 64);
            let Some(t802) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874028); };
            c.logic(t802 & 0x4u64, 8);
            if c.cc_ne() { blk = 1050; continue; }
            blk = 655; continue;
        }
        655 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x7000874033); }
            c.w64(0, c.r64(9));
            blk = 166; continue;
        }
        656 => {
            let Some(t803) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087403f); };
            c.w64(1, t803);
            blk = 932; continue;
        }
        657 => {
            let Some(t804) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874048); };
            c.w64(1, t804);
            blk = 932; continue;
        }
        658 => {
            c.w64(9, c.r64(1));
            let r805 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r805);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 659; continue;
        }
        659 => {
            let Some(t806) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000874060); };
            c.w64(11, t806);
            let r807 = c.r32(11) & 0xfcu64;
            c.w32(11, r807);
            c.logic(r807, 32);
            c.cmp(c.r32(11), 0x14u64, 32);
            if c.cc_e() { blk = 673; continue; }
            blk = 660; continue;
        }
        660 => {
            let Some(t808) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874075); };
            c.w32(9, t808);
            let r809 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r809);
            let Some(t810) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087407c); };
            c.w32(9, t810);
            let r811 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r811);
            let Some(t812) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000874083); };
            c.w32(11, t812);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 662; continue; }
            blk = 661; continue;
        }
        661 => {
            return RegfileExit::fault(0x7000874093);
        }
        662 => {
            let Some(t813) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000874094); };
            c.cmp(t813, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 663; continue;
        }
        663 => {
            c.w64(8, c.r64(0));
            let r814 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r814);
            c.logic(r814, 64);
            let Some(t815) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008740ab); };
            c.w64(15, t815);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 670; continue; }
            blk = 664; continue;
        }
        664 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 670; continue; }
            blk = 665; continue;
        }
        665 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t816) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008740bf); };
            c.w64(3, t816);
            let Some(t817) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008740c6); };
            c.w64(3, t817);
            c.w64(7, c.r64(15));
            let r818 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r818);
            c.logic(r818, 64);
            c.w64(6, c.r64(3));
            let r819 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r819);
            c.logic(r819, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 668; continue; }
            blk = 666; continue;
        }
        666 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 669; continue; }
            blk = 667; continue;
        }
        667 => {
            blk = 670; continue;
        }
        668 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_be() { blk = 670; continue; }
            blk = 669; continue;
        }
        669 => {
            let Some(t820) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008740ee); };
            c.w32(8, t820);
            let Some(t821) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008740f2); };
            c.cmp(t821, c.r32(8), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 670; continue;
        }
        670 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 672; continue; }
            blk = 671; continue;
        }
        671 => {
            return RegfileExit::fault(0x7000874105);
        }
        672 => {
            let Some(t822) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000874106); };
            c.w32(8, t822);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087410d); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r823 = c.r64(9) & c.r64(0);
            c.w64(9, r823);
            c.logic(r823, 64);
            let Some(t824) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087411b); };
            c.logic(t824 & 0x4u64, 8);
            if c.cc_ne() { blk = 1047; continue; }
            blk = 673; continue;
        }
        673 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let r825 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r825);
            c.w64(9, c.r64(2));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700087413b); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000874142); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        674 => {
            let Some(t826) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087414e); };
            c.logic(t826 & 0x200000u64, 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 675; continue;
        }
        675 => {
            let Some(t827) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087415c); };
            c.w32(9, t827);
            let r828 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r828);
            let Some(t829) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000874163); };
            c.w32(9, t829);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 932; continue; }
            blk = 676; continue;
        }
        676 => {
            let Some(t830) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874171); };
            c.w32(9, t830);
            let Some(t831) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000874175); };
            c.cmp(t831, c.r32(9), 32);
            if c.cc_ne() { blk = 750; continue; }
            blk = 677; continue;
        }
        677 => {
            c.w64(9, c.r64(8));
            blk = 678; continue;
        }
        678 => {
            let Some(t832) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874185); };
            c.w32(9, t832);
            let r833 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r833);
            let Some(t834) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087418c); };
            c.cmp(t834, c.r32(9), 32);
            if c.cc_e() { blk = 683; continue; }
            blk = 679; continue;
        }
        679 => {
            let Some(t835) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874195); };
            c.w32(9, t835);
            let r836 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r836);
            let Some(t837) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087419c); };
            c.cmp(t837, 0x411u64, 16);
            if c.cc_be() { blk = 932; continue; }
            blk = 680; continue;
        }
        680 => {
            let Some(t838) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008741a9); };
            c.w32(11, t838);
            c.w64(7, c.r64(11));
            let r839 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r839);
            let r840 = c.r32(7) & 0x3fu64;
            c.w32(7, r840);
            c.logic(r840, 32);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_be() { blk = 678; continue; }
            blk = 681; continue;
        }
        681 => {
            let r841 = c.r32(11) & 0xfcu64;
            c.w32(11, r841);
            c.logic(r841, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 678; continue; }
            blk = 682; continue;
        }
        682 => {
            blk = 932; continue;
        }
        683 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 749; continue; }
            blk = 684; continue;
        }
        684 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_le() { blk = 688; continue; }
            blk = 685; continue;
        }
        685 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(9, c.r64(12).wrapping_add(0x1));
            let r842 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r842);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008741f0); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 687; continue; }
            blk = 686; continue;
        }
        686 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r843 = c.r64(11) & c.r64(2);
            c.w64(11, r843);
            c.logic(r843, 64);
            let Some(t844) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874203); };
            c.logic(t844 & 0x4u64, 8);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 687; continue;
        }
        687 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700087420e); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        688 => {
            let Some(t845) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087421a); };
            c.w32(9, t845);
            let r846 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r846);
            let Some(t847) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874221); };
            c.w32(11, t847);
            let Some(t848) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000874225); };
            c.w32(9, t848);
            let r849 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r849);
            let Some(t850) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087422c); };
            c.cmp(t850, c.r32(11), 32);
            if c.cc_ne() { blk = 726; continue; }
            blk = 689; continue;
        }
        689 => {
            c.w64(11, c.r64(0));
            let r851 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r851);
            c.logic(r851, 64);
            let Some(t852) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874243); };
            c.w64(15, t852);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 697; continue; }
            blk = 690; continue;
        }
        690 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 697; continue; }
            blk = 691; continue;
        }
        691 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t853) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874257); };
            c.w64(7, t853);
            let Some(t854) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087425e); };
            c.w64(7, t854);
            c.w64(6, c.r64(15));
            let r855 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r855);
            c.logic(r855, 64);
            c.w64(1, c.r64(7));
            let r856 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r856);
            c.logic(r856, 64);
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_e() { blk = 694; continue; }
            blk = 692; continue;
        }
        692 => {
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_e() { blk = 695; continue; }
            blk = 693; continue;
        }
        693 => {
            blk = 697; continue;
        }
        694 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 697; continue; }
            blk = 695; continue;
        }
        695 => {
            let Some(t857) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874286); };
            c.w32(11, t857);
            let Some(t858) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700087428a); };
            c.cmp(t858, c.r32(11), 32);
            if c.cc_ne() { blk = 697; continue; }
            blk = 696; continue;
        }
        696 => {
            let Some(t859) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874293); };
            c.w64(1, t859);
            blk = 932; continue;
        }
        697 => {
            let Some(t860) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087429c); };
            c.w32(11, t860);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 699; continue; }
            blk = 698; continue;
        }
        698 => {
            return RegfileExit::fault(0x70008742ac);
        }
        699 => {
            let Some(t861) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008742ad); };
            c.cmp(t861, c.r32(8), 32);
            if c.cc_ne() { blk = 703; continue; }
            blk = 700; continue;
        }
        700 => {
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 702; continue; }
            blk = 701; continue;
        }
        701 => {
            return RegfileExit::fault(0x70008742bc);
        }
        702 => {
            let Some(t862) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008742bd); };
            c.w32(8, t862);
            blk = 709; continue;
        }
        703 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 705; continue; }
            blk = 704; continue;
        }
        704 => {
            return RegfileExit::fault(0x70008742cc);
        }
        705 => {
            let Some(t863) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x70008742cd); };
            c.cmp(t863, c.r32(8), 32);
            if c.cc_ne() { blk = 725; continue; }
            blk = 706; continue;
        }
        706 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 708; continue; }
            blk = 707; continue;
        }
        707 => {
            return RegfileExit::fault(0x70008742e0);
        }
        708 => {
            let Some(t864) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008742e1); };
            c.w32(8, t864);
            blk = 709; continue;
        }
        709 => {
            c.cmp(c.r64(3), 0x3fffu64, 64);
            if c.cc_ae() { blk = 724; continue; }
            blk = 710; continue;
        }
        710 => {
            c.cmp(c.r64(3), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 973; continue; }
            blk = 711; continue;
        }
        711 => {
            let Some(t865) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000874302); };
            c.w64(11, t865);
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 969; continue; }
            blk = 712; continue;
        }
        712 => {
            let Some(t866) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087431b); };
            c.w64(15, t866);
            let Some(t867) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000874322); };
            c.w64(1, t867);
            c.w64(6, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t868) = c.ld64(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874329); };
            c.cmp(t868, c.r64(6), 64);
            if c.cc_be() { blk = 971; continue; }
            blk = 713; continue;
        }
        713 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x7000874333); }
            c.w64(9, c.r64(1).wrapping_add(0x1));
            blk = 714; continue;
        }
        714 => {
            c.w32(15, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t869) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700087433e); };
            c.w64(3, t869);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000874345); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000874349); }
            c.w64(15, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 720; continue; }
            blk = 715; continue;
        }
        715 => {
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            blk = 716; continue;
        }
        716 => {
            let r870 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r870);
            let Some(t871) = c.ld32(c.r64(15).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000874367); };
            c.w32(3, t871);
            let r872 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r872);
            let Some(t873) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087436f); };
            c.cmp(t873, c.r32(3), 32);
            if c.cc_e() { blk = 718; continue; }
            blk = 717; continue;
        }
        717 => {
            let r874 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r874);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(c.r64(9)), 0) { return RegfileExit::fault(0x7000874382); }
            blk = 719; continue;
        }
        718 => {
            let Some(t875) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x700087438a); };
            c.w64(3, t875);
            if !c.st64(c.r64(11).wrapping_add(c.r64(9)), c.r64(3)) { return RegfileExit::fault(0x7000874391); }
            blk = 719; continue;
        }
        719 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 716; continue; }
            blk = 720; continue;
        }
        720 => {
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r876 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r876);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008743a3); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 722; continue; }
            blk = 721; continue;
        }
        721 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r877 = c.r64(15) & c.r64(0);
            c.w64(15, r877);
            c.logic(r877, 64);
            let Some(t878) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008743b7); };
            c.logic(t878 & 0x4u64, 8);
            if c.cc_ne() { blk = 1041; continue; }
            blk = 722; continue;
        }
        722 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008743c2); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r879 = c.r64(9) & c.r64(0);
            c.w64(9, r879);
            c.logic(r879, 64);
            let Some(t880) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008743d0); };
            c.logic(t880 & 0x4u64, 8);
            if c.cc_ne() { blk = 1038; continue; }
            blk = 723; continue;
        }
        723 => {
            let Some(t881) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008743db); };
            c.w64(1, t881);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008743df); }
            let Some(t882) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008743e4); };
            c.w32(8, t882);
            let r883 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r883);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x70008743f5); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008743fc); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        724 => {
            let Some(t884) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874408); };
            c.w64(1, t884);
            blk = 932; continue;
        }
        725 => {
            let Some(t885) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874411); };
            c.w64(1, t885);
            blk = 932; continue;
        }
        726 => {
            c.w64(11, c.r64(0));
            let r886 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r886);
            c.logic(r886, 64);
            let Some(t887) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874424); };
            c.w64(15, t887);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 733; continue; }
            blk = 727; continue;
        }
        727 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 733; continue; }
            blk = 728; continue;
        }
        728 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t888) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874438); };
            c.w64(3, t888);
            let Some(t889) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700087443f); };
            c.w64(3, t889);
            c.w64(7, c.r64(15));
            let r890 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r890);
            c.logic(r890, 64);
            c.w64(6, c.r64(3));
            let r891 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r891);
            c.logic(r891, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 731; continue; }
            blk = 729; continue;
        }
        729 => {
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 732; continue; }
            blk = 730; continue;
        }
        730 => {
            blk = 733; continue;
        }
        731 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_be() { blk = 733; continue; }
            blk = 732; continue;
        }
        732 => {
            let Some(t892) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874467); };
            c.w32(11, t892);
            let Some(t893) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700087446b); };
            c.cmp(t893, c.r32(11), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 733; continue;
        }
        733 => {
            let Some(t894) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000874478); };
            c.w32(11, t894);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 735; continue; }
            blk = 734; continue;
        }
        734 => {
            return RegfileExit::fault(0x7000874488);
        }
        735 => {
            let Some(t895) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000874489); };
            c.cmp(t895, c.r32(8), 32);
            if c.cc_ne() { blk = 739; continue; }
            blk = 736; continue;
        }
        736 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 738; continue; }
            blk = 737; continue;
        }
        737 => {
            return RegfileExit::fault(0x7000874498);
        }
        738 => {
            let Some(t896) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000874499); };
            c.w32(8, t896);
            blk = 745; continue;
        }
        739 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 741; continue; }
            blk = 740; continue;
        }
        740 => {
            return RegfileExit::fault(0x70008744a8);
        }
        741 => {
            let Some(t897) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x70008744a9); };
            c.cmp(t897, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 742; continue;
        }
        742 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 744; continue; }
            blk = 743; continue;
        }
        743 => {
            return RegfileExit::fault(0x70008744bc);
        }
        744 => {
            let Some(t898) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x70008744bd); };
            c.w32(8, t898);
            blk = 745; continue;
        }
        745 => {
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r899 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r899);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008744d4); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r900 = c.r64(12) & c.r64(0);
            c.w64(12, r900);
            c.logic(r900, 64);
            let Some(t901) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008744e2); };
            c.logic(t901 & 0x4u64, 8);
            if c.cc_ne() { blk = 1035; continue; }
            blk = 746; continue;
        }
        746 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x70008744ee); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 748; continue; }
            blk = 747; continue;
        }
        747 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r902 = c.r64(8) & c.r64(2);
            c.w64(8, r902);
            c.logic(r902, 64);
            let Some(t903) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874501); };
            c.logic(t903 & 0x4u64, 8);
            if c.cc_ne() { blk = 1032; continue; }
            blk = 748; continue;
        }
        748 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700087450c); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        749 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            let r904 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r904);
            if !c.st32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000874520); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000874525); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        750 => {
            let Some(t905) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000874531); };
            c.cmp(t905, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 751; continue;
        }
        751 => {
            c.w64(9, c.r64(8));
            blk = 752; continue;
        }
        752 => {
            let Some(t906) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874541); };
            c.w32(9, t906);
            let r907 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r907);
            let Some(t908) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000874548); };
            c.cmp(t908, c.r32(9), 32);
            if c.cc_e() { blk = 757; continue; }
            blk = 753; continue;
        }
        753 => {
            let Some(t909) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874551); };
            c.w32(9, t909);
            let r910 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r910);
            let Some(t911) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000874558); };
            c.cmp(t911, 0x411u64, 16);
            if c.cc_be() { blk = 932; continue; }
            blk = 754; continue;
        }
        754 => {
            let Some(t912) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000874565); };
            c.w32(11, t912);
            c.w64(15, c.r64(11));
            let r913 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r913);
            let r914 = c.r32(15) & 0x3fu64;
            c.w32(15, r914);
            c.logic(r914, 32);
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_be() { blk = 752; continue; }
            blk = 755; continue;
        }
        755 => {
            let r915 = c.r32(11) & 0xfcu64;
            c.w32(11, r915);
            c.logic(r915, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 752; continue; }
            blk = 756; continue;
        }
        756 => {
            blk = 932; continue;
        }
        757 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 804; continue; }
            blk = 758; continue;
        }
        758 => {
            let Some(t916) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874599); };
            c.w32(9, t916);
            let Some(t917) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087459d); };
            c.cmp(t917, c.r32(9), 32);
            if c.cc_ne() { blk = 760; continue; }
            blk = 759; continue;
        }
        759 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008745a6); }
            blk = 805; continue;
        }
        760 => {
            let Some(t918) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008745b0); };
            c.w32(9, t918);
            let r919 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r919);
            c.w64(11, c.r64(0));
            let r920 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r920);
            c.logic(r920, 64);
            let Some(t921) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008745c1); };
            c.w32(9, t921);
            let r922 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r922);
            let Some(t923) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008745c8); };
            c.w64(15, t923);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 768; continue; }
            blk = 761; continue;
        }
        761 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 768; continue; }
            blk = 762; continue;
        }
        762 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t924) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008745dc); };
            c.w64(6, t924);
            let Some(t925) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008745e3); };
            c.w64(6, t925);
            c.w64(1, c.r64(15));
            let r926 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r926);
            c.logic(r926, 64);
            c.w64(12, c.r64(6));
            let r927 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r927);
            c.logic(r927, 64);
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_e() { blk = 765; continue; }
            blk = 763; continue;
        }
        763 => {
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_e() { blk = 766; continue; }
            blk = 764; continue;
        }
        764 => {
            blk = 768; continue;
        }
        765 => {
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_be() { blk = 768; continue; }
            blk = 766; continue;
        }
        766 => {
            let Some(t928) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087460b); };
            c.w32(11, t928);
            let Some(t929) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700087460f); };
            c.cmp(t929, c.r32(11), 32);
            if c.cc_ne() { blk = 768; continue; }
            blk = 767; continue;
        }
        767 => {
            let Some(t930) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874618); };
            c.w64(1, t930);
            blk = 932; continue;
        }
        768 => {
            let Some(t931) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000874621); };
            c.w32(11, t931);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 770; continue; }
            blk = 769; continue;
        }
        769 => {
            return RegfileExit::fault(0x7000874631);
        }
        770 => {
            let Some(t932) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000874632); };
            c.cmp(t932, c.r32(8), 32);
            if c.cc_ne() { blk = 774; continue; }
            blk = 771; continue;
        }
        771 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 773; continue; }
            blk = 772; continue;
        }
        772 => {
            return RegfileExit::fault(0x7000874641);
        }
        773 => {
            let Some(t933) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000874642); };
            c.w32(8, t933);
            blk = 780; continue;
        }
        774 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 776; continue; }
            blk = 775; continue;
        }
        775 => {
            return RegfileExit::fault(0x7000874651);
        }
        776 => {
            let Some(t934) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000874652); };
            c.cmp(t934, c.r32(8), 32);
            if c.cc_ne() { blk = 803; continue; }
            blk = 777; continue;
        }
        777 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 779; continue; }
            blk = 778; continue;
        }
        778 => {
            return RegfileExit::fault(0x7000874665);
        }
        779 => {
            let Some(t935) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000874666); };
            c.w32(8, t935);
            blk = 780; continue;
        }
        780 => {
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 802; continue; }
            blk = 781; continue;
        }
        781 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 979; continue; }
            blk = 782; continue;
        }
        782 => {
            c.w32(9, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t936) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087468b); };
            c.w64(11, t936);
            let Some(t937) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000874692); };
            c.w64(12, t937);
            c.w64(15, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(15).wrapping_add(c.r64(12)));
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t938) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008746b3); };
            c.w64(6, t938);
            let Some(t939) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008746ba); };
            c.cmp(t939, c.r64(3), 64);
            if c.cc_be() { blk = 975; continue; }
            blk = 783; continue;
        }
        783 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008746c3); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 784; continue;
        }
        784 => {
            let Some(t940) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008746cb); };
            c.w64(12, t940);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008746d2); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008746d6); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 787; continue; }
            blk = 785; continue;
        }
        785 => {
            c.w64(9, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t941) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008746e7); };
            c.w64(12, t941);
            blk = 786; continue;
        }
        786 => {
            let r942 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r942);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008746f2); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 786; continue; }
            blk = 787; continue;
        }
        787 => {
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 796; continue; }
            blk = 788; continue;
        }
        788 => {
            c.w64(12, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t943) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x7000874716); };
            c.w64(15, t943);
            blk = 790; continue;
        }
        789 => {
            let r944 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r944);
            let Some(t945) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000874723); };
            c.w64(2, t945);
            blk = 790; continue;
        }
        790 => {
            let r946 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r946);
            let Some(t947) = c.ld64(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700087472b); };
            c.cmp(t947, c.r64(15), 64);
            if c.cc_e() { blk = 795; continue; }
            blk = 791; continue;
        }
        791 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(2))) { return RegfileExit::fault(0x7000874731); }
            let Some(t948) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874737); };
            c.w64(3, t948);
            let Some(t949) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700087473e); };
            c.w64(1, t949);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            c.w64(7, c.r64(12).wrapping_sub(0x4));
            let Some(t950) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087474a); };
            c.w64(2, t950);
            let Some(t951) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000874751); };
            c.cmp(t951, c.r64(6), 64);
            if c.cc_be() { blk = 977; continue; }
            blk = 792; continue;
        }
        792 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x700087475a); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 793; continue;
        }
        793 => {
            let Some(t952) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000874761); };
            c.w64(2, t952);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000874768); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087476b); }
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(3)) { return RegfileExit::fault(0x7000874770); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 795; continue; }
            blk = 794; continue;
        }
        794 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r953 = c.r64(2) & c.r64(11);
            c.w64(2, r953);
            c.logic(r953, 64);
            let Some(t954) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874783); };
            c.logic(t954 & 0x4u64, 8);
            if c.cc_ne() { blk = 1029; continue; }
            blk = 795; continue;
        }
        795 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 789; continue; }
            blk = 796; continue;
        }
        796 => {
            let Some(t955) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000874793); };
            c.w64(12, t955);
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let r956 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r956);
            let r957 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r957);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008747a6); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 798; continue; }
            blk = 797; continue;
        }
        797 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r958 = c.r64(15) & c.r64(0);
            c.w64(15, r958);
            c.logic(r958, 64);
            let Some(t959) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008747ba); };
            c.logic(t959 & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 798; continue;
        }
        798 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008747c5); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r960 = c.r64(11) & c.r64(0);
            c.w64(11, r960);
            c.logic(r960, 64);
            let Some(t961) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008747d3); };
            c.logic(t961 & 0x4u64, 8);
            if c.cc_ne() { blk = 1023; continue; }
            blk = 799; continue;
        }
        799 => {
            let Some(t962) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008747de); };
            c.w32(8, t962);
            let r963 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r963);
            let Some(t964) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008747e5); };
            c.w64(11, t964);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x70008747e9); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 801; continue; }
            blk = 800; continue;
        }
        800 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r965 = c.r64(15) & c.r64(8);
            c.w64(15, r965);
            c.logic(r965, 64);
            let Some(t966) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008747fd); };
            c.logic(t966 & 0x4u64, 8);
            if c.cc_ne() { blk = 1020; continue; }
            blk = 801; continue;
        }
        801 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x7000874808); }
            c.w64(0, c.r64(11));
            blk = 166; continue;
        }
        802 => {
            let Some(t967) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874814); };
            c.w64(1, t967);
            blk = 932; continue;
        }
        803 => {
            let Some(t968) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087481d); };
            c.w64(1, t968);
            blk = 932; continue;
        }
        804 => {
            c.w64(8, c.r64(1));
            let r969 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r969);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 805; continue;
        }
        805 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let r970 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r970);
            c.w64(9, c.r64(2));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700087484a); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000874851); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        806 => {
            let Some(t971) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087485d); };
            c.w32(9, t971);
            let Some(t972) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000874861); };
            c.cmp(t972, c.r32(9), 32);
            if c.cc_ne() { blk = 878; continue; }
            blk = 807; continue;
        }
        807 => {
            let Some(t973) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087486e); };
            c.w32(9, t973);
            let Some(t974) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000874873); };
            c.cmp(t974, c.r32(9), 32);
            if c.cc_ne() { blk = 814; continue; }
            blk = 808; continue;
        }
        808 => {
            c.w64(9, c.r64(8));
            blk = 809; continue;
        }
        809 => {
            let Some(t975) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087487f); };
            c.w32(9, t975);
            let r976 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r976);
            let Some(t977) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000874886); };
            c.cmp(t977, c.r32(9), 32);
            if c.cc_e() { blk = 814; continue; }
            blk = 810; continue;
        }
        810 => {
            let Some(t978) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087488f); };
            c.w32(9, t978);
            let r979 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r979);
            let Some(t980) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000874896); };
            c.cmp(t980, 0x411u64, 16);
            if c.cc_be() { blk = 932; continue; }
            blk = 811; continue;
        }
        811 => {
            let Some(t981) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008748a3); };
            c.w32(11, t981);
            c.w64(7, c.r64(11));
            let r982 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r982);
            let r983 = c.r32(7) & 0x3fu64;
            c.w32(7, r983);
            c.logic(r983, 32);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_be() { blk = 809; continue; }
            blk = 812; continue;
        }
        812 => {
            let r984 = c.r32(11) & 0xfcu64;
            c.w32(11, r984);
            c.logic(r984, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 809; continue; }
            blk = 813; continue;
        }
        813 => {
            blk = 932; continue;
        }
        814 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 877; continue; }
            blk = 815; continue;
        }
        815 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_le() { blk = 819; continue; }
            blk = 816; continue;
        }
        816 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008748e2); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 818; continue; }
            blk = 817; continue;
        }
        817 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r985 = c.r64(9) & c.r64(2);
            c.w64(9, r985);
            c.logic(r985, 64);
            let Some(t986) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008748f5); };
            c.logic(t986 & 0x4u64, 8);
            if c.cc_ne() { blk = 1017; continue; }
            blk = 818; continue;
        }
        818 => {
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        819 => {
            let Some(t987) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874908); };
            c.w32(9, t987);
            let r988 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r988);
            let Some(t989) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087490f); };
            c.w32(11, t989);
            let Some(t990) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000874913); };
            c.w32(9, t990);
            let r991 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r991);
            let Some(t992) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087491a); };
            c.cmp(t992, c.r32(11), 32);
            if c.cc_ne() { blk = 854; continue; }
            blk = 820; continue;
        }
        820 => {
            c.w64(11, c.r64(0));
            let r993 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r993);
            c.logic(r993, 64);
            let Some(t994) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874931); };
            c.w64(15, t994);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 827; continue; }
            blk = 821; continue;
        }
        821 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 827; continue; }
            blk = 822; continue;
        }
        822 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t995) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874945); };
            c.w64(7, t995);
            let Some(t996) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087494c); };
            c.w64(7, t996);
            c.w64(3, c.r64(15));
            let r997 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r997);
            c.logic(r997, 64);
            c.w64(6, c.r64(7));
            let r998 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r998);
            c.logic(r998, 64);
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_e() { blk = 825; continue; }
            blk = 823; continue;
        }
        823 => {
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_e() { blk = 826; continue; }
            blk = 824; continue;
        }
        824 => {
            blk = 827; continue;
        }
        825 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 827; continue; }
            blk = 826; continue;
        }
        826 => {
            let Some(t999) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874974); };
            c.w32(11, t999);
            let Some(t1000) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000874978); };
            c.cmp(t1000, c.r32(11), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 827; continue;
        }
        827 => {
            let Some(t1001) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000874985); };
            c.w32(11, t1001);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 829; continue; }
            blk = 828; continue;
        }
        828 => {
            return RegfileExit::fault(0x7000874995);
        }
        829 => {
            let Some(t1002) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000874996); };
            c.cmp(t1002, c.r32(8), 32);
            if c.cc_ne() { blk = 833; continue; }
            blk = 830; continue;
        }
        830 => {
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 832; continue; }
            blk = 831; continue;
        }
        831 => {
            return RegfileExit::fault(0x70008749a5);
        }
        832 => {
            let Some(t1003) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008749a6); };
            c.w32(8, t1003);
            blk = 839; continue;
        }
        833 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 835; continue; }
            blk = 834; continue;
        }
        834 => {
            return RegfileExit::fault(0x70008749b5);
        }
        835 => {
            let Some(t1004) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x70008749b6); };
            c.cmp(t1004, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 836; continue;
        }
        836 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 838; continue; }
            blk = 837; continue;
        }
        837 => {
            return RegfileExit::fault(0x70008749c9);
        }
        838 => {
            let Some(t1005) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008749ca); };
            c.w32(8, t1005);
            blk = 839; continue;
        }
        839 => {
            let Some(t1006) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008749d1); };
            c.w32(9, t1006);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3fffu64, 64);
            if c.cc_ae() { blk = 932; continue; }
            blk = 840; continue;
        }
        840 => {
            c.cmp(c.r64(11), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 985; continue; }
            blk = 841; continue;
        }
        841 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 981; continue; }
            blk = 842; continue;
        }
        842 => {
            let Some(t1007) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874a0d); };
            c.w64(3, t1007);
            let Some(t1008) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000874a14); };
            c.w64(6, t1008);
            c.w64(7, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t1009) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874a1b); };
            c.cmp(t1009, c.r64(7), 64);
            if c.cc_be() { blk = 983; continue; }
            blk = 843; continue;
        }
        843 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x7000874a25); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 844; continue;
        }
        844 => {
            let r1010 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r1010);
            let Some(t1011) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000874a2f); };
            c.w64(3, t1011);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000874a36); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000874a3a); }
            c.w64(11, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 850; continue; }
            blk = 845; continue;
        }
        845 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            blk = 846; continue;
        }
        846 => {
            let r1012 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r1012);
            let Some(t1013) = c.ld32(c.r64(11).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000874a58); };
            c.w32(3, t1013);
            let r1014 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r1014);
            let Some(t1015) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000874a60); };
            c.cmp(t1015, c.r32(3), 32);
            if c.cc_e() { blk = 848; continue; }
            blk = 847; continue;
        }
        847 => {
            let r1016 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r1016);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x7000874a73); }
            blk = 849; continue;
        }
        848 => {
            let Some(t1017) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x7000874a7b); };
            c.w64(3, t1017);
            if !c.st64(c.r64(9).wrapping_add(c.r64(15)), c.r64(3)) { return RegfileExit::fault(0x7000874a82); }
            blk = 849; continue;
        }
        849 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 846; continue; }
            blk = 850; continue;
        }
        850 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000874a8c); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 852; continue; }
            blk = 851; continue;
        }
        851 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r1018 = c.r64(9) & c.r64(0);
            c.w64(9, r1018);
            c.logic(r1018, 64);
            let Some(t1019) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874aa0); };
            c.logic(t1019 & 0x4u64, 8);
            if c.cc_ne() { blk = 1014; continue; }
            blk = 852; continue;
        }
        852 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000874aab); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r1020 = c.r64(9) & c.r64(0);
            c.w64(9, r1020);
            c.logic(r1020, 64);
            let Some(t1021) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874ab9); };
            c.logic(t1021 & 0x4u64, 8);
            if c.cc_ne() { blk = 1011; continue; }
            blk = 853; continue;
        }
        853 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000874ac4); }
            let Some(t1022) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000874ac9); };
            c.w32(8, t1022);
            let r1023 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1023);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000874ada); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        854 => {
            c.w64(11, c.r64(0));
            let r1024 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r1024);
            c.logic(r1024, 64);
            let Some(t1025) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874af3); };
            c.w64(15, t1025);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 861; continue; }
            blk = 855; continue;
        }
        855 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 861; continue; }
            blk = 856; continue;
        }
        856 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t1026) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874b07); };
            c.w64(7, t1026);
            let Some(t1027) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000874b0e); };
            c.w64(7, t1027);
            c.w64(3, c.r64(15));
            let r1028 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r1028);
            c.logic(r1028, 64);
            c.w64(6, c.r64(7));
            let r1029 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r1029);
            c.logic(r1029, 64);
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_e() { blk = 859; continue; }
            blk = 857; continue;
        }
        857 => {
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_e() { blk = 860; continue; }
            blk = 858; continue;
        }
        858 => {
            blk = 861; continue;
        }
        859 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 861; continue; }
            blk = 860; continue;
        }
        860 => {
            let Some(t1030) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874b36); };
            c.w32(11, t1030);
            let Some(t1031) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000874b3a); };
            c.cmp(t1031, c.r32(11), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 861; continue;
        }
        861 => {
            let Some(t1032) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000874b47); };
            c.w32(11, t1032);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 863; continue; }
            blk = 862; continue;
        }
        862 => {
            return RegfileExit::fault(0x7000874b57);
        }
        863 => {
            let Some(t1033) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000874b58); };
            c.cmp(t1033, c.r32(8), 32);
            if c.cc_ne() { blk = 867; continue; }
            blk = 864; continue;
        }
        864 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 866; continue; }
            blk = 865; continue;
        }
        865 => {
            return RegfileExit::fault(0x7000874b67);
        }
        866 => {
            let Some(t1034) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000874b68); };
            c.w32(8, t1034);
            blk = 873; continue;
        }
        867 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 869; continue; }
            blk = 868; continue;
        }
        868 => {
            return RegfileExit::fault(0x7000874b77);
        }
        869 => {
            let Some(t1035) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000874b78); };
            c.cmp(t1035, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 870; continue;
        }
        870 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 872; continue; }
            blk = 871; continue;
        }
        871 => {
            return RegfileExit::fault(0x7000874b8b);
        }
        872 => {
            let Some(t1036) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000874b8c); };
            c.w32(8, t1036);
            blk = 873; continue;
        }
        873 => {
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000874b9b); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r1037 = c.r64(11) & c.r64(0);
            c.w64(11, r1037);
            c.logic(r1037, 64);
            let Some(t1038) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874ba9); };
            c.logic(t1038 & 0x4u64, 8);
            if c.cc_ne() { blk = 1008; continue; }
            blk = 874; continue;
        }
        874 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x7000874bb4); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 876; continue; }
            blk = 875; continue;
        }
        875 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r1039 = c.r64(8) & c.r64(2);
            c.w64(8, r1039);
            c.logic(r1039, 64);
            let Some(t1040) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874bc7); };
            c.logic(t1040 & 0x4u64, 8);
            if c.cc_ne() { blk = 1005; continue; }
            blk = 876; continue;
        }
        876 => {
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        877 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000874bda); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        878 => {
            let Some(t1041) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000874be7); };
            c.cmp(t1041, c.r32(9), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 879; continue;
        }
        879 => {
            let Some(t1042) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x7000874bf4); };
            c.w64(9, t1042);
            let Some(t1043) = c.ld64(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000874bfb); };
            c.cmp(t1043, c.r64(9), 64);
            if c.cc_ne() { blk = 886; continue; }
            blk = 880; continue;
        }
        880 => {
            c.w64(9, c.r64(8));
            blk = 881; continue;
        }
        881 => {
            let Some(t1044) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874c05); };
            c.w32(9, t1044);
            let r1045 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1045);
            let Some(t1046) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000874c0c); };
            c.cmp(t1046, c.r32(9), 32);
            if c.cc_e() { blk = 886; continue; }
            blk = 882; continue;
        }
        882 => {
            let Some(t1047) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874c15); };
            c.w32(9, t1047);
            let r1048 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1048);
            let Some(t1049) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000874c1c); };
            c.cmp(t1049, 0x411u64, 16);
            if c.cc_be() { blk = 932; continue; }
            blk = 883; continue;
        }
        883 => {
            let Some(t1050) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000874c29); };
            c.w32(11, t1050);
            c.w64(15, c.r64(11));
            let r1051 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r1051);
            let r1052 = c.r32(15) & 0x3fu64;
            c.w32(15, r1052);
            c.logic(r1052, 32);
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_be() { blk = 881; continue; }
            blk = 884; continue;
        }
        884 => {
            let r1053 = c.r32(11) & 0xfcu64;
            c.w32(11, r1053);
            c.logic(r1053, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 881; continue; }
            blk = 885; continue;
        }
        885 => {
            blk = 932; continue;
        }
        886 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 930; continue; }
            blk = 887; continue;
        }
        887 => {
            let Some(t1054) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874c5d); };
            c.w32(9, t1054);
            let Some(t1055) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000874c61); };
            c.cmp(t1055, c.r32(9), 32);
            if c.cc_ne() { blk = 889; continue; }
            blk = 888; continue;
        }
        888 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000874c6a); }
            blk = 931; continue;
        }
        889 => {
            let Some(t1056) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000874c74); };
            c.w32(9, t1056);
            let r1057 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1057);
            c.w64(11, c.r64(0));
            let r1058 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r1058);
            c.logic(r1058, 64);
            let Some(t1059) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000874c85); };
            c.w32(9, t1059);
            let r1060 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1060);
            let Some(t1061) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874c8c); };
            c.w64(15, t1061);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 896; continue; }
            blk = 890; continue;
        }
        890 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 896; continue; }
            blk = 891; continue;
        }
        891 => {
            c.w64(15, c.r64(0).wrapping_add(0x13));
            let Some(t1062) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874ca0); };
            c.w64(7, t1062);
            let Some(t1063) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000874ca7); };
            c.w64(7, t1063);
            c.w64(3, c.r64(15));
            let r1064 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r1064);
            c.logic(r1064, 64);
            c.w64(6, c.r64(7));
            let r1065 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r1065);
            c.logic(r1065, 64);
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_e() { blk = 894; continue; }
            blk = 892; continue;
        }
        892 => {
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_e() { blk = 895; continue; }
            blk = 893; continue;
        }
        893 => {
            blk = 896; continue;
        }
        894 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 896; continue; }
            blk = 895; continue;
        }
        895 => {
            let Some(t1066) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000874ccf); };
            c.w32(11, t1066);
            let Some(t1067) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000874cd3); };
            c.cmp(t1067, c.r32(11), 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 896; continue;
        }
        896 => {
            let Some(t1068) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000874ce0); };
            c.w32(11, t1068);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 898; continue; }
            blk = 897; continue;
        }
        897 => {
            return RegfileExit::fault(0x7000874cf0);
        }
        898 => {
            let Some(t1069) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000874cf1); };
            c.cmp(t1069, c.r32(8), 32);
            if c.cc_ne() { blk = 902; continue; }
            blk = 899; continue;
        }
        899 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 901; continue; }
            blk = 900; continue;
        }
        900 => {
            return RegfileExit::fault(0x7000874d00);
        }
        901 => {
            let Some(t1070) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000874d01); };
            c.w32(8, t1070);
            blk = 908; continue;
        }
        902 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 904; continue; }
            blk = 903; continue;
        }
        903 => {
            return RegfileExit::fault(0x7000874d10);
        }
        904 => {
            let Some(t1071) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000874d11); };
            c.cmp(t1071, c.r32(8), 32);
            if c.cc_ne() { blk = 932; continue; }
            blk = 905; continue;
        }
        905 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 907; continue; }
            blk = 906; continue;
        }
        906 => {
            return RegfileExit::fault(0x7000874d24);
        }
        907 => {
            let Some(t1072) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000874d25); };
            c.w32(8, t1072);
            blk = 908; continue;
        }
        908 => {
            let Some(t1073) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000874d2c); };
            c.w32(9, t1073);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000874d39); }
            c.cmp(c.r64(11), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 932; continue; }
            blk = 909; continue;
        }
        909 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 991; continue; }
            blk = 910; continue;
        }
        910 => {
            let Some(t1074) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874d57); };
            c.w64(15, t1074);
            let Some(t1075) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000874d5e); };
            c.w64(3, t1075);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t1076) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000874d6d); };
            c.w64(1, t1076);
            let Some(t1077) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000874d74); };
            c.cmp(t1077, c.r64(7), 64);
            if c.cc_be() { blk = 987; continue; }
            blk = 911; continue;
        }
        911 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x7000874d7d); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 912; continue;
        }
        912 => {
            let r1078 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r1078);
            let Some(t1079) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000874d87); };
            c.w64(3, t1079);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000874d8e); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000874d92); }
            c.w64(11, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 915; continue; }
            blk = 913; continue;
        }
        913 => {
            c.w64(3, c.r64(11));
            let Some(t1080) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000874da7); };
            c.w64(1, t1080);
            blk = 914; continue;
        }
        914 => {
            let r1081 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r1081);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)), c.r32(1)) { return RegfileExit::fault(0x7000874db2); }
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_ne() { blk = 914; continue; }
            blk = 915; continue;
        }
        915 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 924; continue; }
            blk = 916; continue;
        }
        916 => {
            let Some(t1082) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x7000874dce); };
            c.w64(3, t1082);
            blk = 918; continue;
        }
        917 => {
            let r1083 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r1083);
            let Some(t1084) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000874ddb); };
            c.w64(2, t1084);
            blk = 918; continue;
        }
        918 => {
            let r1085 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r1085);
            let Some(t1086) = c.ld64(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000874de3); };
            c.cmp(t1086, c.r64(3), 64);
            if c.cc_e() { blk = 923; continue; }
            blk = 919; continue;
        }
        919 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(2))) { return RegfileExit::fault(0x7000874de9); }
            let Some(t1087) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000874def); };
            c.w64(1, t1087);
            let Some(t1088) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000874df6); };
            c.w64(6, t1088);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            c.w64(2, c.r64(11).wrapping_sub(0x4));
            let Some(t1089) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000874e01); };
            c.w64(8, t1089);
            let Some(t1090) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000874e08); };
            c.cmp(t1090, c.r64(7), 64);
            if c.cc_be() { blk = 989; continue; }
            blk = 920; continue;
        }
        920 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000874e11); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 921; continue;
        }
        921 => {
            let Some(t1091) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000874e18); };
            c.w64(1, t1091);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000874e1f); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000874e23); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(2)), c.r32(8)) { return RegfileExit::fault(0x7000874e29); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 923; continue; }
            blk = 922; continue;
        }
        922 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r1092 = c.r64(1) & c.r64(15);
            c.w64(1, r1092);
            c.logic(r1092, 64);
            let Some(t1093) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874e3d); };
            c.logic(t1093 & 0x4u64, 8);
            if c.cc_ne() { blk = 1002; continue; }
            blk = 923; continue;
        }
        923 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 917; continue; }
            blk = 924; continue;
        }
        924 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000874e55); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 926; continue; }
            blk = 925; continue;
        }
        925 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r1094 = c.r64(9) & c.r64(0);
            c.w64(9, r1094);
            c.logic(r1094, 64);
            let Some(t1095) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874e69); };
            c.logic(t1095 & 0x4u64, 8);
            if c.cc_ne() { blk = 999; continue; }
            blk = 926; continue;
        }
        926 => {
            let Some(t1096) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000874e74); };
            c.w64(9, t1096);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000874e78); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r1097 = c.r64(11) & c.r64(0);
            c.w64(11, r1097);
            c.logic(r1097, 64);
            let Some(t1098) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874e86); };
            c.logic(t1098 & 0x4u64, 8);
            if c.cc_ne() { blk = 996; continue; }
            blk = 927; continue;
        }
        927 => {
            let Some(t1099) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000874e91); };
            c.w32(9, t1099);
            let r1100 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1100);
            let Some(t1101) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874e98); };
            c.w64(11, t1101);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000874e9c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 929; continue; }
            blk = 928; continue;
        }
        928 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r1102 = c.r64(12) & c.r64(9);
            c.w64(12, r1102);
            c.logic(r1102, 64);
            let Some(t1103) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000874eb0); };
            c.logic(t1103 & 0x4u64, 8);
            if c.cc_ne() { blk = 993; continue; }
            blk = 929; continue;
        }
        929 => {
            c.w64(0, c.r64(11));
            blk = 166; continue;
        }
        930 => {
            c.w64(8, c.r64(1));
            let r1104 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r1104);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 931; continue;
        }
        931 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            c.w64(8, c.r64(2));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000874ee0); }
            c.w64(0, c.r64(1));
            blk = 166; continue;
        }
        932 => {
            let Some(t1105) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000874eef); };
            c.w64(5, t1105);
            let Some(t1106) = c.ld64(c.r64(13).wrapping_add(0x2470)) else { return RegfileExit::fault(0x7000874ef3); };
            c.w64(3, t1106);
            let Some(t1107) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x7000874efa); };
            c.w64(10, t1107);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000874eff); }
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(0)) { return RegfileExit::fault(0x7000874f04); }
            let Some(t1108) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x7000874f09); };
            c.w64(10, t1108);
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(10)) { return RegfileExit::fault(0x7000874f0e); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(1)) { return RegfileExit::fault(0x7000874f13); }
            c.w32(0, 0x3u64);
            let Some(t1109) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x7000874f1d); };
            c.w64(6, t1109);
            let r1110 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r1110);
            return RegfileExit::jump(0x700086ed74);
        }
        933 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 932; continue; }
            blk = 934; continue;
        }
        934 => {
            let Some(t1111) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000874f32); };
            c.w32(9, t1111);
            let r1112 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1112);
            blk = 26; continue;
        }
        935 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000874f43); }
            return c.call(0x70007febf4, 0x7000874f4d, 0x7000874f48);
        }
        936 => {
            c.w64(8, c.r64(0));
            let Some(t1113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000874f50); };
            c.w64(0, t1113);
            let Some(t1114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000874f54); };
            c.w64(3, t1114);
            let Some(t1115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000874f58); };
            c.w64(1, t1115);
            let Some(t1116) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000874f5c); };
            c.w64(6, t1116);
            let Some(t1117) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000874f60); };
            c.w64(9, t1117);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000874f64); }
            blk = 58; continue;
        }
        937 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000874f73); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000874f77); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000874f7b); }
            return c.call(0x70007febf4, 0x7000874f85, 0x7000874f80);
        }
        938 => {
            c.w64(12, c.r64(0));
            let Some(t1118) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000874f88); };
            c.w64(8, t1118);
            let Some(t1119) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000874f8c); };
            c.w64(9, t1119);
            let Some(t1120) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000874f90); };
            c.w64(11, t1120);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000874f94); }
            blk = 251; continue;
        }
        939 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000874f9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000874fa2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000874fa6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000874faa); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000874fae); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febf4, 0x7000874fba, 0x7000874fb5);
        }
        940 => {
            c.w64(8, c.r64(0));
            let Some(t1121) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000874fbd); };
            c.w64(15, t1121);
            let Some(t1122) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000874fc1); };
            c.w64(9, t1122);
            let Some(t1123) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000874fc5); };
            c.w64(3, t1123);
            let Some(t1124) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000874fc9); };
            c.w64(12, t1124);
            let Some(t1125) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000874fcd); };
            c.w64(11, t1125);
            let Some(t1126) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000874fd1); };
            c.w64(2, t1126);
            blk = 259; continue;
        }
        941 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000874fdf); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000874fe3); }
            return c.call(0x70007febf4, 0x7000874fed, 0x7000874fe8);
        }
        942 => {
            c.w64(11, c.r64(0));
            let Some(t1127) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000874ff0); };
            c.w64(9, t1127);
            let Some(t1128) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000874ff4); };
            c.w64(8, t1128);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000874ff8); }
            blk = 281; continue;
        }
        943 => {
            return RegfileExit::fault(0x7000875002);
        }
        944 => {
            return RegfileExit::fault(0x7000875003);
        }
        945 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000875009); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700087500e); }
            return c.call(0x70007febf4, 0x7000875017, 0x7000875012);
        }
        946 => {
            c.w64(8, c.r64(0));
            let Some(t1129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087501a); };
            c.w64(9, t1129);
            let Some(t1130) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087501e); };
            c.w64(15, t1130);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000875022); }
            blk = 401; continue;
        }
        947 => {
            return RegfileExit::fault(0x700087502c);
        }
        948 => {
            return RegfileExit::fault(0x700087502d);
        }
        949 => {
            let Some(t1131) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x700087502e); };
            c.w64(11, t1131);
            blk = 434; continue;
        }
        950 => {
            return RegfileExit::fault(0x700087503a);
        }
        951 => {
            return RegfileExit::fault(0x700087503b);
        }
        952 => {
            return RegfileExit::fault(0x700087503c);
        }
        953 => {
            return RegfileExit::fault(0x700087503d);
        }
        954 => {
            return RegfileExit::fault(0x700087503e);
        }
        955 => {
            return RegfileExit::fault(0x700087503f);
        }
        956 => {
            return RegfileExit::fault(0x7000875040);
        }
        957 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1132) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000875043); };
            c.w64(3, t1132);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000875051, 0x700087504c);
        }
        958 => {
            return RegfileExit::fault(0x7000875051);
        }
        959 => {
            let r1133 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r1133);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000875055); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000875059); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700087505a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700087505e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x7000875062); }
            let Some(t1134) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000875066); };
            c.w64(3, t1134);
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700087507a, 0x7000875075);
        }
        960 => {
            c.w64(9, c.r64(0));
            let Some(t1135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087507d); };
            c.w64(0, t1135);
            let Some(t1136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000875081); };
            c.w64(2, t1136);
            let Some(t1137) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000875085); };
            c.w64(12, t1137);
            let Some(t1138) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000875089); };
            c.w64(11, t1138);
            let Some(t1139) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087508d); };
            c.w64(8, t1139);
            let Some(t1140) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000875091); };
            c.w64(7, t1140);
            let Some(t1141) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000875095); };
            c.w64(15, t1141);
            blk = 539; continue;
        }
        961 => {
            let Some(t1142) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700087509e); };
            c.w64(3, t1142);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008750ae, 0x70008750a9);
        }
        962 => {
            return RegfileExit::fault(0x70008750ae);
        }
        963 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008750af); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008750b3); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febf4, 0x70008750bf, 0x70008750ba);
        }
        964 => {
            c.w64(9, c.r64(0));
            let Some(t1143) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008750c2); };
            c.w64(0, t1143);
            let Some(t1144) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008750c6); };
            c.w64(2, t1144);
            let Some(t1145) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008750ca); };
            c.w64(7, t1145);
            let Some(t1146) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008750ce); };
            c.w64(3, t1146);
            let Some(t1147) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008750d2); };
            c.w64(8, t1147);
            blk = 638; continue;
        }
        965 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008750e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008750e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008750e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008750ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008750f0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70008750f4); }
            return c.call(0x70007febf4, 0x70008750fe, 0x70008750f9);
        }
        966 => {
            c.w64(3, c.r64(0));
            let Some(t1148) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875101); };
            c.w64(0, t1148);
            let Some(t1149) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000875105); };
            c.w64(8, t1149);
            let Some(t1150) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000875109); };
            c.w64(9, t1150);
            let Some(t1151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087510d); };
            c.w64(12, t1151);
            let Some(t1152) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000875111); };
            c.w64(11, t1152);
            let Some(t1153) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000875115); };
            c.w64(7, t1153);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000875119); }
            let Some(t1154) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x700087511e); };
            c.w64(15, t1154);
            blk = 647; continue;
        }
        967 => {
            let Some(t1155) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700087512a); };
            c.w64(3, t1155);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087513a, 0x7000875135);
        }
        968 => {
            return RegfileExit::fault(0x700087513a);
        }
        969 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1156) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700087513d); };
            c.w64(3, t1156);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087514b, 0x7000875146);
        }
        970 => {
            return RegfileExit::fault(0x700087514b);
        }
        971 => {
            let r1157 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r1157);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700087514f); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000875153); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000875154); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000875158); }
            c.w32(0, 0x2u64);
            let Some(t1158) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000875161); };
            c.w64(3, t1158);
            c.w64(9, c.r64(3));
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x7000875173, 0x700087516e);
        }
        972 => {
            c.w64(9, c.r64(0));
            let Some(t1159) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875176); };
            c.w64(0, t1159);
            let Some(t1160) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087517a); };
            c.w64(2, t1160);
            let Some(t1161) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087517e); };
            c.w64(12, t1161);
            let Some(t1162) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000875182); };
            c.w64(11, t1162);
            let Some(t1163) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000875186); };
            c.w64(3, t1163);
            let Some(t1164) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087518a); };
            c.w64(8, t1164);
            blk = 714; continue;
        }
        973 => {
            let Some(t1165) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000875193); };
            c.w64(3, t1165);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008751a3, 0x700087519e);
        }
        974 => {
            return RegfileExit::fault(0x70008751a3);
        }
        975 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008751a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008751a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008751ac); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008751b8, 0x70008751b3);
        }
        976 => {
            c.w64(11, c.r64(0));
            let Some(t1166) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008751bb); };
            c.w64(0, t1166);
            let Some(t1167) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008751bf); };
            c.w64(2, t1167);
            let Some(t1168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008751c3); };
            c.w64(8, t1168);
            let Some(t1169) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008751c7); };
            c.w64(7, t1169);
            let Some(t1170) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008751cb); };
            c.w64(1, t1170);
            let Some(t1171) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008751cf); };
            c.w64(9, t1171);
            blk = 784; continue;
        }
        977 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008751dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008751e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008751e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008751e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008751ed); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70008751f1); }
            return c.call(0x70007febf4, 0x70008751fb, 0x70008751f6);
        }
        978 => {
            c.w64(3, c.r64(0));
            let Some(t1172) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008751fe); };
            c.w64(0, t1172);
            let Some(t1173) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000875202); };
            c.w64(8, t1173);
            let Some(t1174) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000875206); };
            c.w64(11, t1174);
            let Some(t1175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087520a); };
            c.w64(12, t1175);
            let Some(t1176) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087520e); };
            c.w64(9, t1176);
            let Some(t1177) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000875212); };
            c.w64(7, t1177);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000875216); }
            let Some(t1178) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x700087521b); };
            c.w64(15, t1178);
            blk = 793; continue;
        }
        979 => {
            let Some(t1179) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000875227); };
            c.w64(3, t1179);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000875237, 0x7000875232);
        }
        980 => {
            return RegfileExit::fault(0x7000875237);
        }
        981 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1180) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700087523a); };
            c.w64(3, t1180);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000875248, 0x7000875243);
        }
        982 => {
            return RegfileExit::fault(0x7000875248);
        }
        983 => {
            let r1181 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r1181);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700087524c); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000875250); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000875251); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000875255); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000875259); }
            let Some(t1182) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700087525d); };
            c.w64(3, t1182);
            c.w32(0, 0x2u64);
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x7000875271, 0x700087526c);
        }
        984 => {
            c.w64(15, c.r64(0));
            let Some(t1183) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875274); };
            c.w64(0, t1183);
            let Some(t1184) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000875278); };
            c.w64(1, t1184);
            let Some(t1185) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087527c); };
            c.w64(2, t1185);
            let Some(t1186) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000875280); };
            c.w64(12, t1186);
            let Some(t1187) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000875284); };
            c.w64(8, t1187);
            let Some(t1188) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000875288); };
            c.w64(9, t1188);
            let Some(t1189) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087528c); };
            c.w64(11, t1189);
            blk = 844; continue;
        }
        985 => {
            let Some(t1190) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000875295); };
            c.w64(3, t1190);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008752a5, 0x70008752a0);
        }
        986 => {
            return RegfileExit::fault(0x70008752a5);
        }
        987 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008752a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008752aa); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x70008752b6, 0x70008752b1);
        }
        988 => {
            c.w64(15, c.r64(0));
            let Some(t1191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008752b9); };
            c.w64(0, t1191);
            let Some(t1192) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008752bd); };
            c.w64(2, t1192);
            let Some(t1193) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008752c1); };
            c.w64(12, t1193);
            let Some(t1194) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008752c5); };
            c.w64(8, t1194);
            let Some(t1195) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008752c9); };
            c.w64(9, t1195);
            let Some(t1196) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008752cd); };
            c.w64(11, t1196);
            blk = 912; continue;
        }
        989 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008752d6); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008752df); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008752e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008752e7); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70008752eb); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008752f8, 0x70008752f3);
        }
        990 => {
            c.w64(8, c.r64(0));
            let Some(t1197) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008752fb); };
            c.w64(0, t1197);
            let Some(t1198) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008752ff); };
            c.w64(12, t1198);
            let Some(t1199) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000875303); };
            c.w64(15, t1199);
            let Some(t1200) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000875307); };
            c.w64(11, t1200);
            let Some(t1201) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087530b); };
            c.w64(9, t1201);
            let Some(t1202) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087530f); };
            c.w64(2, t1202);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000875313); }
            let Some(t1203) = c.ld64(0x7000873a57u64) else { return RegfileExit::fault(0x7000875318); };
            c.w64(3, t1203);
            blk = 921; continue;
        }
        991 => {
            let Some(t1204) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000875324); };
            c.w64(3, t1204);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000875334, 0x700087532f);
        }
        992 => {
            return RegfileExit::fault(0x7000875334);
        }
        993 => {
            c.w32(11, c.r32(11));
            let r1205 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1205);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1206 = c.r64(12) & c.r64(11);
            c.w64(12, r1206);
            c.logic(r1206, 64);
            let Some(t1207) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875345); };
            c.logic(t1207 & 0x2u64, 8);
            if c.cc_e() { blk = 929; continue; }
            blk = 994; continue;
        }
        994 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875355); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875356); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000875363, 0x700087535e);
        }
        995 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875363); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875365); }
            blk = 929; continue;
        }
        996 => {
            c.w32(9, c.r32(9));
            let r1208 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1208);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1209 = c.r64(11) & c.r64(9);
            c.w64(11, r1209);
            c.logic(r1209, 64);
            let Some(t1210) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087537b); };
            c.logic(t1210 & 0x2u64, 8);
            if c.cc_e() { blk = 927; continue; }
            blk = 997; continue;
        }
        997 => {
            c.w64(12, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087538a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087538b); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000875398, 0x7000875393);
        }
        998 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875398); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087539a); }
            blk = 927; continue;
        }
        999 => {
            c.w32(15, c.r32(15));
            let r1211 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1211);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1212 = c.r64(9) & c.r64(15);
            c.w64(9, r1212);
            c.logic(r1212, 64);
            let Some(t1213) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008753b0); };
            c.logic(t1213 & 0x2u64, 8);
            if c.cc_e() { blk = 926; continue; }
            blk = 1000; continue;
        }
        1000 => {
            c.w64(11, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008753bf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008753c0); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008753cd, 0x70008753c8);
        }
        1001 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008753cd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008753cf); }
            blk = 926; continue;
        }
        1002 => {
            c.w32(8, c.r32(8));
            let r1214 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1214);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1215 = c.r64(1) & c.r64(8);
            c.w64(1, r1215);
            c.logic(r1215, 64);
            let Some(t1216) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008753e5); };
            c.logic(t1216 & 0x2u64, 8);
            if c.cc_e() { blk = 923; continue; }
            blk = 1003; continue;
        }
        1003 => {
            c.w64(6, c.r64(15).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008753f3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008753f4); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x7000875401, 0x70008753fc);
        }
        1004 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875401); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875403); }
            blk = 923; continue;
        }
        1005 => {
            c.w32(1, c.r32(1));
            let r1217 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1217);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1218 = c.r64(8) & c.r64(1);
            c.w64(8, r1218);
            c.logic(r1218, 64);
            let Some(t1219) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875418); };
            c.logic(t1219 & 0x2u64, 8);
            if c.cc_e() { blk = 876; continue; }
            blk = 1006; continue;
        }
        1006 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875427); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875428); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000875435, 0x7000875430);
        }
        1007 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875435); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875437); }
            blk = 876; continue;
        }
        1008 => {
            c.w32(8, c.r32(8));
            let r1220 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1220);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1221 = c.r64(11) & c.r64(8);
            c.w64(11, r1221);
            c.logic(r1221, 64);
            let Some(t1222) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087544d); };
            c.logic(t1222 & 0x2u64, 8);
            if c.cc_e() { blk = 874; continue; }
            blk = 1009; continue;
        }
        1009 => {
            c.w64(12, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087545c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087545d); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x700087546a, 0x7000875465);
        }
        1010 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087546a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087546c); }
            blk = 874; continue;
        }
        1011 => {
            c.w32(8, c.r32(8));
            let r1223 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1223);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1224 = c.r64(9) & c.r64(8);
            c.w64(9, r1224);
            c.logic(r1224, 64);
            let Some(t1225) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875482); };
            c.logic(t1225 & 0x2u64, 8);
            if c.cc_e() { blk = 853; continue; }
            blk = 1012; continue;
        }
        1012 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875491); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875492); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x700087549f, 0x700087549a);
        }
        1013 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087549f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008754a1); }
            blk = 853; continue;
        }
        1014 => {
            c.w32(15, c.r32(15));
            let r1226 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1226);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1227 = c.r64(9) & c.r64(15);
            c.w64(9, r1227);
            c.logic(r1227, 64);
            let Some(t1228) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008754b7); };
            c.logic(t1228 & 0x2u64, 8);
            if c.cc_e() { blk = 852; continue; }
            blk = 1015; continue;
        }
        1015 => {
            c.w64(11, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008754c6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008754c7); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008754d4, 0x70008754cf);
        }
        1016 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008754d4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008754d6); }
            blk = 852; continue;
        }
        1017 => {
            c.w32(1, c.r32(1));
            let r1229 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1229);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1230 = c.r64(9) & c.r64(1);
            c.w64(9, r1230);
            c.logic(r1230, 64);
            let Some(t1231) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008754eb); };
            c.logic(t1231 & 0x2u64, 8);
            if c.cc_e() { blk = 818; continue; }
            blk = 1018; continue;
        }
        1018 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008754fa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008754fb); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000875508, 0x7000875503);
        }
        1019 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875508); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087550a); }
            blk = 818; continue;
        }
        1020 => {
            c.w32(11, c.r32(11));
            let r1232 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1232);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1233 = c.r64(15) & c.r64(11);
            c.w64(15, r1233);
            c.logic(r1233, 64);
            let Some(t1234) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875520); };
            c.logic(t1234 & 0x2u64, 8);
            if c.cc_e() { blk = 801; continue; }
            blk = 1021; continue;
        }
        1021 => {
            c.w64(3, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087552f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875530); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700087553d, 0x7000875538);
        }
        1022 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087553d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087553f); }
            blk = 801; continue;
        }
        1023 => {
            c.w32(8, c.r32(8));
            let r1235 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1235);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1236 = c.r64(11) & c.r64(8);
            c.w64(11, r1236);
            c.logic(r1236, 64);
            let Some(t1237) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875555); };
            c.logic(t1237 & 0x2u64, 8);
            if c.cc_e() { blk = 799; continue; }
            blk = 1024; continue;
        }
        1024 => {
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875564); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875565); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000875572, 0x700087556d);
        }
        1025 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875572); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875574); }
            blk = 799; continue;
        }
        1026 => {
            c.w32(11, c.r32(11));
            let r1238 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1238);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1239 = c.r64(15) & c.r64(11);
            c.w64(15, r1239);
            c.logic(r1239, 64);
            let Some(t1240) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087558a); };
            c.logic(t1240 & 0x2u64, 8);
            if c.cc_e() { blk = 798; continue; }
            blk = 1027; continue;
        }
        1027 => {
            c.w64(3, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875599); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087559a); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008755a7, 0x70008755a2);
        }
        1028 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008755a7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008755a9); }
            blk = 798; continue;
        }
        1029 => {
            c.w32(3, c.r32(3));
            let r1241 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1241);
            c.w64(2, 0xfffffffffffc0000u64);
            let r1242 = c.r64(2) & c.r64(3);
            c.w64(2, r1242);
            c.logic(r1242, 64);
            let Some(t1243) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008755be); };
            c.logic(t1243 & 0x2u64, 8);
            if c.cc_e() { blk = 795; continue; }
            blk = 1030; continue;
        }
        1030 => {
            c.w64(1, c.r64(11).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008755cc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008755cd); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008755da, 0x70008755d5);
        }
        1031 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008755da); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008755dc); }
            blk = 795; continue;
        }
        1032 => {
            c.w32(1, c.r32(1));
            let r1244 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1244);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1245 = c.r64(8) & c.r64(1);
            c.w64(8, r1245);
            c.logic(r1245, 64);
            let Some(t1246) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008755f1); };
            c.logic(t1246 & 0x2u64, 8);
            if c.cc_e() { blk = 748; continue; }
            blk = 1033; continue;
        }
        1033 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875600); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875601); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700087560e, 0x7000875609);
        }
        1034 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087560e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875610); }
            blk = 748; continue;
        }
        1035 => {
            c.w32(8, c.r32(8));
            let r1247 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1247);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1248 = c.r64(12) & c.r64(8);
            c.w64(12, r1248);
            c.logic(r1248, 64);
            let Some(t1249) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875626); };
            c.logic(t1249 & 0x2u64, 8);
            if c.cc_e() { blk = 746; continue; }
            blk = 1036; continue;
        }
        1036 => {
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875636); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875637); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000875644, 0x700087563f);
        }
        1037 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875644); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875646); }
            blk = 746; continue;
        }
        1038 => {
            c.w32(8, c.r32(8));
            let r1250 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1250);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1251 = c.r64(9) & c.r64(8);
            c.w64(9, r1251);
            c.logic(r1251, 64);
            let Some(t1252) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087565c); };
            c.logic(t1252 & 0x2u64, 8);
            if c.cc_e() { blk = 723; continue; }
            blk = 1039; continue;
        }
        1039 => {
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087566b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087566c); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000875679, 0x7000875674);
        }
        1040 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875679); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087567b); }
            blk = 723; continue;
        }
        1041 => {
            c.w32(9, c.r32(9));
            let r1253 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1253);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1254 = c.r64(15) & c.r64(9);
            c.w64(15, r1254);
            c.logic(r1254, 64);
            let Some(t1255) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875691); };
            c.logic(t1255 & 0x2u64, 8);
            if c.cc_e() { blk = 722; continue; }
            blk = 1042; continue;
        }
        1042 => {
            c.w64(3, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008756a0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008756a1); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008756ae, 0x70008756a9);
        }
        1043 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008756ae); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008756b0); }
            blk = 722; continue;
        }
        1044 => {
            c.w32(1, c.r32(1));
            let r1256 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1256);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1257 = c.r64(11) & c.r64(1);
            c.w64(11, r1257);
            c.logic(r1257, 64);
            let Some(t1258) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008756c5); };
            c.logic(t1258 & 0x2u64, 8);
            if c.cc_e() { blk = 687; continue; }
            blk = 1045; continue;
        }
        1045 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008756d4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008756d5); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008756e2, 0x70008756dd);
        }
        1046 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008756e2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008756e4); }
            blk = 687; continue;
        }
        1047 => {
            c.w32(8, c.r32(8));
            let r1259 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1259);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1260 = c.r64(9) & c.r64(8);
            c.w64(9, r1260);
            c.logic(r1260, 64);
            let Some(t1261) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008756fa); };
            c.logic(t1261 & 0x2u64, 8);
            if c.cc_e() { blk = 673; continue; }
            blk = 1048; continue;
        }
        1048 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875709); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087570a); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000875717, 0x7000875712);
        }
        1049 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875717); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875719); }
            blk = 673; continue;
        }
        1050 => {
            c.w32(9, c.r32(9));
            let r1262 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1262);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1263 = c.r64(15) & c.r64(9);
            c.w64(15, r1263);
            c.logic(r1263, 64);
            let Some(t1264) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087572f); };
            c.logic(t1264 & 0x2u64, 8);
            if c.cc_e() { blk = 655; continue; }
            blk = 1051; continue;
        }
        1051 => {
            c.w64(3, c.r64(8).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087573e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087573f); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700087574c, 0x7000875747);
        }
        1052 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087574c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087574e); }
            blk = 655; continue;
        }
        1053 => {
            c.w32(8, c.r32(8));
            let r1265 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1265);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1266 = c.r64(9) & c.r64(8);
            c.w64(9, r1266);
            c.logic(r1266, 64);
            let Some(t1267) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875764); };
            c.logic(t1267 & 0x2u64, 8);
            if c.cc_e() { blk = 653; continue; }
            blk = 1054; continue;
        }
        1054 => {
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875773); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875774); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000875781, 0x700087577c);
        }
        1055 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875781); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875783); }
            blk = 653; continue;
        }
        1056 => {
            c.w32(9, c.r32(9));
            let r1268 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1268);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1269 = c.r64(15) & c.r64(9);
            c.w64(15, r1269);
            c.logic(r1269, 64);
            let Some(t1270) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875799); };
            c.logic(t1270 & 0x2u64, 8);
            if c.cc_e() { blk = 652; continue; }
            blk = 1057; continue;
        }
        1057 => {
            c.w64(3, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008757a8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008757a9); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008757b6, 0x70008757b1);
        }
        1058 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008757b6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008757b8); }
            blk = 652; continue;
        }
        1059 => {
            c.w32(3, c.r32(3));
            let r1271 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1271);
            c.w64(2, 0xfffffffffffc0000u64);
            let r1272 = c.r64(2) & c.r64(3);
            c.w64(2, r1272);
            c.logic(r1272, 64);
            let Some(t1273) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008757cd); };
            c.logic(t1273 & 0x2u64, 8);
            if c.cc_e() { blk = 649; continue; }
            blk = 1060; continue;
        }
        1060 => {
            c.w64(1, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008757db); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008757dc); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008757e9, 0x70008757e4);
        }
        1061 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008757e9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008757eb); }
            blk = 649; continue;
        }
        1062 => {
            c.w32(8, c.r32(8));
            let r1274 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1274);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1275 = c.r64(9) & c.r64(8);
            c.w64(9, r1275);
            c.logic(r1275, 64);
            let Some(t1276) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875801); };
            c.logic(t1276 & 0x2u64, 8);
            if c.cc_e() { blk = 603; continue; }
            blk = 1063; continue;
        }
        1063 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875810); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875811); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x700087581e, 0x7000875819);
        }
        1064 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087581e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875820); }
            blk = 603; continue;
        }
        1065 => {
            c.w32(8, c.r32(8));
            let r1277 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1277);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1278 = c.r64(9) & c.r64(8);
            c.w64(9, r1278);
            c.logic(r1278, 64);
            let Some(t1279) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875836); };
            c.logic(t1279 & 0x2u64, 8);
            if c.cc_e() { blk = 589; continue; }
            blk = 1066; continue;
        }
        1066 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875845); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875846); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000875853, 0x700087584e);
        }
        1067 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875853); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875855); }
            blk = 589; continue;
        }
        1068 => {
            c.w32(1, c.r32(1));
            let r1280 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1280);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1281 = c.r64(8) & c.r64(1);
            c.w64(8, r1281);
            c.logic(r1281, 64);
            let Some(t1282) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087586a); };
            c.logic(t1282 & 0x2u64, 8);
            if c.cc_e() { blk = 573; continue; }
            blk = 1069; continue;
        }
        1069 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875879); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087587a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000875887, 0x7000875882);
        }
        1070 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875887); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875889); }
            blk = 573; continue;
        }
        1071 => {
            c.w32(8, c.r32(8));
            let r1283 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1283);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1284 = c.r64(12) & c.r64(8);
            c.w64(12, r1284);
            c.logic(r1284, 64);
            let Some(t1285) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087589f); };
            c.logic(t1285 & 0x2u64, 8);
            if c.cc_e() { blk = 571; continue; }
            blk = 1072; continue;
        }
        1072 => {
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008758af); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008758b0); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x70008758bd, 0x70008758b8);
        }
        1073 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008758bd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008758bf); }
            blk = 571; continue;
        }
        1074 => {
            c.w32(8, c.r32(8));
            let r1286 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1286);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1287 = c.r64(9) & c.r64(8);
            c.w64(9, r1287);
            c.logic(r1287, 64);
            let Some(t1288) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008758d5); };
            c.logic(t1288 & 0x2u64, 8);
            if c.cc_e() { blk = 548; continue; }
            blk = 1075; continue;
        }
        1075 => {
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008758e4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008758e5); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x70008758f2, 0x70008758ed);
        }
        1076 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008758f2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008758f4); }
            blk = 548; continue;
        }
        1077 => {
            c.w32(9, c.r32(9));
            let r1289 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1289);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1290 = c.r64(15) & c.r64(9);
            c.w64(15, r1290);
            c.logic(r1290, 64);
            let Some(t1291) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087590a); };
            c.logic(t1291 & 0x2u64, 8);
            if c.cc_e() { blk = 547; continue; }
            blk = 1078; continue;
        }
        1078 => {
            c.w64(3, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875919); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087591a); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000875927, 0x7000875922);
        }
        1079 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875927); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875929); }
            blk = 547; continue;
        }
        1080 => {
            c.w32(1, c.r32(1));
            let r1292 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1292);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1293 = c.r64(11) & c.r64(1);
            c.w64(11, r1293);
            c.logic(r1293, 64);
            let Some(t1294) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087593e); };
            c.logic(t1294 & 0x2u64, 8);
            if c.cc_e() { blk = 512; continue; }
            blk = 1081; continue;
        }
        1081 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087594d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087594e); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700087595b, 0x7000875956);
        }
        1082 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087595b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087595d); }
            blk = 512; continue;
        }
        1083 => {
            c.w32(8, c.r32(8));
            let r1295 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1295);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1296 = c.r64(9) & c.r64(8);
            c.w64(9, r1296);
            c.logic(r1296, 64);
            let Some(t1297) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875973); };
            c.logic(t1297 & 0x2u64, 8);
            if c.cc_e() { blk = 510; continue; }
            blk = 1084; continue;
        }
        1084 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875982); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875983); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000875990, 0x700087598b);
        }
        1085 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875990); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875992); }
            blk = 510; continue;
        }
        1086 => {
            c.w32(0, c.r32(0));
            let r1298 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1298);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1299 = c.r64(1) & c.r64(0);
            c.w64(1, r1299);
            c.logic(r1299, 64);
            let Some(t1300) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008759a7); };
            c.logic(t1300 & 0x2u64, 8);
            if c.cc_e() { blk = 460; continue; }
            blk = 1087; continue;
        }
        1087 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008759b5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008759b6); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008759c3, 0x70008759be);
        }
        1088 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008759c3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008759c5); }
            blk = 460; continue;
        }
        1089 => {
            c.w32(9, c.r32(9));
            let r1301 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1301);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1302 = c.r64(11) & c.r64(9);
            c.w64(11, r1302);
            c.logic(r1302, 64);
            let Some(t1303) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008759db); };
            c.logic(t1303 & 0x2u64, 8);
            if c.cc_e() { blk = 457; continue; }
            blk = 1090; continue;
        }
        1090 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008759ea); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008759eb); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008759f8, 0x70008759f3);
        }
        1091 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008759f8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008759fa); }
            blk = 457; continue;
        }
        1092 => {
            c.w32(1, c.r32(1));
            let r1304 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1304);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1305 = c.r64(9) & c.r64(1);
            c.w64(9, r1305);
            c.logic(r1305, 64);
            let Some(t1306) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875a0f); };
            c.logic(t1306 & 0x2u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 1093; continue;
        }
        1093 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875a1e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875a1f); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000875a2c, 0x7000875a27);
        }
        1094 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875a2c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875a2e); }
            blk = 319; continue;
        }
        1095 => {
            c.w32(11, c.r32(11));
            let r1307 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1307);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1308 = c.r64(8) & c.r64(11);
            c.w64(8, r1308);
            c.logic(r1308, 64);
            let Some(t1309) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875a44); };
            c.logic(t1309 & 0x2u64, 8);
            if c.cc_e() { blk = 284; continue; }
            blk = 1096; continue;
        }
        1096 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875a53); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875a54); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000875a61, 0x7000875a5c);
        }
        1097 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875a61); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875a63); }
            blk = 284; continue;
        }
        1098 => {
            c.w32(8, c.r32(8));
            let r1310 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1310);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1311 = c.r64(15) & c.r64(8);
            c.w64(15, r1311);
            c.logic(r1311, 64);
            let Some(t1312) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875a79); };
            c.logic(t1312 & 0x2u64, 8);
            if c.cc_e() { blk = 282; continue; }
            blk = 1099; continue;
        }
        1099 => {
            c.w64(0, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875a89); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875a8a); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x7000875a97, 0x7000875a92);
        }
        1100 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875a97); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875a99); }
            blk = 282; continue;
        }
        1101 => {
            c.w32(0, c.r32(0));
            let r1313 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1313);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1314 = c.r64(8) & c.r64(0);
            c.w64(8, r1314);
            c.logic(r1314, 64);
            let Some(t1315) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875aae); };
            c.logic(t1315 & 0x2u64, 8);
            if c.cc_e() { blk = 275; continue; }
            blk = 1102; continue;
        }
        1102 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875abd); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875abe); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000875acb, 0x7000875ac6);
        }
        1103 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875acb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875acd); }
            blk = 275; continue;
        }
        1104 => {
            c.w32(8, c.r32(8));
            let r1316 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1316);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1317 = c.r64(11) & c.r64(8);
            c.w64(11, r1317);
            c.logic(r1317, 64);
            let Some(t1318) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875ae3); };
            c.logic(t1318 & 0x2u64, 8);
            if c.cc_e() { blk = 273; continue; }
            blk = 1105; continue;
        }
        1105 => {
            c.w64(15, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875af3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875af4); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000875b01, 0x7000875afc);
        }
        1106 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875b01); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875b03); }
            blk = 273; continue;
        }
        1107 => {
            c.w32(8, c.r32(8));
            let r1319 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1319);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1320 = c.r64(9) & c.r64(8);
            c.w64(9, r1320);
            c.logic(r1320, 64);
            let Some(t1321) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875b19); };
            c.logic(t1321 & 0x2u64, 8);
            if c.cc_e() { blk = 270; continue; }
            blk = 1108; continue;
        }
        1108 => {
            c.w64(11, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875b28); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875b29); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000875b36, 0x7000875b31);
        }
        1109 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875b36); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875b38); }
            blk = 270; continue;
        }
        1110 => {
            c.w32(0, c.r32(0));
            let r1322 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1322);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1323 = c.r64(9) & c.r64(0);
            c.w64(9, r1323);
            c.logic(r1323, 64);
            let Some(t1324) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875b4d); };
            c.logic(t1324 & 0x2u64, 8);
            if c.cc_e() { blk = 269; continue; }
            blk = 1111; continue;
        }
        1111 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875b5c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875b5d); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000875b6a, 0x7000875b65);
        }
        1112 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875b6a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875b6c); }
            blk = 269; continue;
        }
        1113 => {
            c.w32(8, c.r32(8));
            let r1325 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1325);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1326 = c.r64(11) & c.r64(8);
            c.w64(11, r1326);
            c.logic(r1326, 64);
            let Some(t1327) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875b82); };
            c.logic(t1327 & 0x2u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 1114; continue;
        }
        1114 => {
            c.w64(12, c.r64(15).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875b91); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875b92); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000875b9f, 0x7000875b9a);
        }
        1115 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875b9f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875ba1); }
            blk = 266; continue;
        }
        1116 => {
            c.w32(8, c.r32(8));
            let r1328 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1328);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1329 = c.r64(9) & c.r64(8);
            c.w64(9, r1329);
            c.logic(r1329, 64);
            let Some(t1330) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875bb7); };
            c.logic(t1330 & 0x2u64, 8);
            if c.cc_e() { blk = 223; continue; }
            blk = 1117; continue;
        }
        1117 => {
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875bc6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875bc7); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000875bd4, 0x7000875bcf);
        }
        1118 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875bd4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875bd6); }
            blk = 223; continue;
        }
        1119 => {
            c.w32(1, c.r32(1));
            let r1331 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1331);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1332 = c.r64(9) & c.r64(1);
            c.w64(9, r1332);
            c.logic(r1332, 64);
            let Some(t1333) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875beb); };
            c.logic(t1333 & 0x2u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 1120; continue;
        }
        1120 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875bfa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875bfb); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000875c08, 0x7000875c03);
        }
        1121 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875c08); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875c0a); }
            blk = 131; continue;
        }
        1122 => {
            c.w32(1, c.r32(1));
            let r1334 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1334);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1335 = c.r64(9) & c.r64(1);
            c.w64(9, r1335);
            c.logic(r1335, 64);
            let Some(t1336) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000875c1f); };
            c.logic(t1336 & 0x2u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 1123; continue;
        }
        1123 => {
            c.w64(11, c.r64(15).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000875c2e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000875c2f); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000875c3c, 0x7000875c37);
        }
        1124 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000875c3c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000875c3e); }
            blk = 92; continue;
        }
        1125 => {
            return RegfileExit::jump(0x7000875c48);
        }
            _ => unreachable!(),
        }
    }
}
