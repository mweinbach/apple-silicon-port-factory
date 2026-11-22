#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008fdb74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fdb74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fdb78); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fdb7e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fdb82); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fdb86); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fdb8d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fdb94); };
            c.w32(11, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fdb9a); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fdba0); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fdba4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fdba8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fdbac); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fdbb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fdbb4); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fdbb8); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fdbc1); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fdbc8); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fdbcc); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fdbd9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fdbdd); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fdbe3); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fdbeb); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fdbf2); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fdbff); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008fdc17); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70008fdc23, 0x70008fdc1e);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fdc23); };
            c.w64(3, t18);
            c.w32(6, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fdc2c); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fdc30); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fdc34); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fdc38); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fdc3c); };
            c.w64(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fdc40); };
            c.w64(2, t24);
            blk = 3; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fdc46); };
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
            let Some(t29) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fdc66); };
            c.cmp(t29, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fdc6e); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fdc81); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fdc86); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fdc8e); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fdc97); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008fdc98); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008fdcac, 0x70008fdca7);
        }
        15 => {
            return RegfileExit::fault(0x70008fdcac);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fdcad); }
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
            let r37 = c.r32(11) & c.r32(0);
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
            let Some(t39) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fdcf1); };
            c.cmp(t39, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fdcf7); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fdd0d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fdd12); }
            blk = 26; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fdd1a); };
            c.w64(7, t43);
            let r44 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fdd22); };
            c.w32(8, t45);
            let Some(t46) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fdd27); };
            c.w64(1, t46);
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fdd2b); };
            c.w64(5, t47);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r48 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fdd3b); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fdd42); };
            c.w64(1, t50);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fdd49); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fdd4d); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fdd51); };
            c.w64(12, t51);
            let Some(t52) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fdd58); };
            c.cmp(t52, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008fdd6b, 0x70008fdd66);
        }
        29 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fdd6b); };
            c.w64(3, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fdd6f); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fdd73); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fdd77); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fdd7b); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fdd7f); };
            c.w64(2, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fdd83); };
            c.w64(11, t59);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008fdd89); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fdd90); };
            c.w64(1, t60);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008fdd97); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fdda3); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r61 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fddb4); }
            return c.call(0x70008708b4, 0x70008fddbe, 0x70008fddb9);
        }
        33 => {
            let Some(t62) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fddbe); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x70008fddcc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fddd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fddd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fddd8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fddde); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fdde2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fdde6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fdded); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fddf4); };
            c.w32(11, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fddfa); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fde00); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fde04); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fde08); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fde0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fde10); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fde14); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fde18); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fde21); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fde28); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fde2c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fde39); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fde3d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fde43); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fde4b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fde52); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fde5f); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008fde77); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70008fde83, 0x70008fde7e);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fde83); };
            c.w64(3, t18);
            c.w32(6, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fde8c); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fde90); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fde94); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fde98); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fde9c); };
            c.w64(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fdea0); };
            c.w64(2, t24);
            blk = 3; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fdea6); };
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
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t29) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fdec3); };
            c.cmp(t29, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fdecb); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fdede); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fdee3); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fdeeb); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fdef4); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008fdef5); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008fdf09, 0x70008fdf04);
        }
        15 => {
            return RegfileExit::fault(0x70008fdf09);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fdf0a); }
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
            c.w64(1, c.r64(11));
            let r37 = c.shl(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r37);
            c.w64(11, c.r64(0));
            let r38 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r38);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
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
            let Some(t39) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fdf50); };
            c.cmp(t39, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fdf56); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fdf6c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fdf71); }
            blk = 26; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fdf79); };
            c.w64(7, t43);
            let r44 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fdf81); };
            c.w32(8, t45);
            let Some(t46) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fdf86); };
            c.w64(1, t46);
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fdf8a); };
            c.w64(5, t47);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r48 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fdf9a); };
            c.w64(11, t49);
            let Some(t50) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fdfa1); };
            c.w64(1, t50);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fdfa8); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008fdfac); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fdfb0); };
            c.w64(12, t51);
            let Some(t52) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fdfb7); };
            c.cmp(t52, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fdfca, 0x70008fdfc5);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fdfcd); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fdfd1); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fdfd5); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fdfd9); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fdfdd); };
            c.w64(2, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fdfe1); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fdfe5); };
            c.w64(3, t59);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x70008fdfeb); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fdff2); };
            c.w64(1, t60);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008fdff9); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fe005); }
            blk = 20; continue;
        }
        32 => {
            let r61 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fe014); }
            return c.call(0x70008708b4, 0x70008fe01e, 0x70008fe019);
        }
        33 => {
            let Some(t62) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fe01e); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x70008fe02c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fe034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe034); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe038); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fe03e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe042); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fe046); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fe04d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fe054); };
            c.w32(11, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fe05a); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe060); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fe064); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe068); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fe06c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fe070); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fe074); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fe078); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe081); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fe088); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe08c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe099); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fe09d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fe0a3); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe0ab); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe0b2); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fe0bf); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008fe0d7); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70008fe0e3, 0x70008fe0de);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fe0e3); };
            c.w64(3, t18);
            c.w32(6, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe0ec); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe0f0); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe0f4); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe0f8); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fe0fc); };
            c.w64(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe100); };
            c.w64(2, t24);
            blk = 3; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fe106); };
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
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t29) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe123); };
            c.cmp(t29, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fe12b); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fe13e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fe143); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fe14b); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fe154); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008fe155); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008fe169, 0x70008fe164);
        }
        15 => {
            return RegfileExit::fault(0x70008fe169);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fe16a); }
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
            c.w64(1, c.r64(11));
            let r37 = c.sar(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r37);
            c.w64(11, c.r64(0));
            let r38 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r38);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
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
            let Some(t39) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe1b0); };
            c.cmp(t39, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fe1b6); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fe1cc); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fe1d1); }
            blk = 26; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe1d9); };
            c.w64(7, t43);
            let r44 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fe1e1); };
            c.w32(8, t45);
            let Some(t46) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fe1e6); };
            c.w64(1, t46);
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fe1ea); };
            c.w64(5, t47);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r48 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fe1fa); };
            c.w64(11, t49);
            let Some(t50) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fe201); };
            c.w64(1, t50);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fe208); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008fe20c); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fe210); };
            c.w64(12, t51);
            let Some(t52) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fe217); };
            c.cmp(t52, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fe22a, 0x70008fe225);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe22d); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe231); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe235); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe239); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe23d); };
            c.w64(2, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fe241); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fe245); };
            c.w64(3, t59);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x70008fe24b); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe252); };
            c.w64(1, t60);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008fe259); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fe265); }
            blk = 20; continue;
        }
        32 => {
            let r61 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fe274); }
            return c.call(0x70008708b4, 0x70008fe27e, 0x70008fe279);
        }
        33 => {
            let Some(t62) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fe27e); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x70008fe28c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fe294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe294); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe298); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fe29e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe2a2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fe2a6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fe2ad); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fe2b4); };
            c.w32(11, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fe2ba); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe2c0); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fe2c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe2c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fe2cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fe2d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fe2d4); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fe2d8); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe2e1); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fe2e8); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe2ec); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe2f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fe2fd); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fe303); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe30b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe312); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fe31f); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008fe337); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70008fe343, 0x70008fe33e);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fe343); };
            c.w64(3, t18);
            c.w32(6, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe34c); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe350); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe354); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe358); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fe35c); };
            c.w64(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe360); };
            c.w64(2, t24);
            blk = 3; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fe366); };
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
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t29) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe383); };
            c.cmp(t29, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fe38b); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fe39e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fe3a3); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fe3ab); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fe3b4); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008fe3b5); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008fe3c9, 0x70008fe3c4);
        }
        15 => {
            return RegfileExit::fault(0x70008fe3c9);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fe3ca); }
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
            c.w64(1, c.r64(11));
            let r37 = c.shr(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r37);
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
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
            let Some(t38) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe410); };
            c.cmp(t38, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t39) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fe416); };
            c.w32(1, t39);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r40 = c.r32(2) | c.r32(0);
            c.w32(2, r40);
            c.logic(r40, 32);
            let r41 = c.r32(2) | c.r32(6);
            c.w32(2, r41);
            c.logic(r41, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fe42c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fe431); }
            blk = 26; continue;
        }
        26 => {
            let Some(t42) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe439); };
            c.w64(7, t42);
            let r43 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r43);
            let Some(t44) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fe441); };
            c.w32(8, t44);
            let Some(t45) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fe446); };
            c.w64(1, t45);
            let Some(t46) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fe44a); };
            c.w64(5, t46);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r47 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r47);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fe45a); };
            c.w64(11, t48);
            let Some(t49) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fe461); };
            c.w64(1, t49);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fe468); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008fe46c); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fe470); };
            c.w64(12, t50);
            let Some(t51) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fe477); };
            c.cmp(t51, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fe48a, 0x70008fe485);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe48d); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe491); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe495); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe499); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe49d); };
            c.w64(2, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fe4a1); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fe4a5); };
            c.w64(3, t58);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x70008fe4ab); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe4b2); };
            c.w64(1, t59);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008fe4b9); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fe4c9); }
            blk = 20; continue;
        }
        32 => {
            let r60 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r60);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fe4d8); }
            return c.call(0x70008708b4, 0x70008fe4e2, 0x70008fe4dd);
        }
        33 => {
            let Some(t61) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fe4e2); };
            c.w32(0, t61);
            let r62 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x70008fe4f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fe4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe4f7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fe4fb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fe502); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fe506); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fe50d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe513); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fe517); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe520); };
            c.w64(8, t9);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe52f); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe535); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fe53e); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fe556); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fe563); }
            blk = 12; continue;
        }
        9 => {
            let r16 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r16);
            if c.cc_o() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(11, c.r32(2));
            let r17 = c.r32(11) | 0x2u64;
            c.w32(11, r17);
            c.logic(r17, 32);
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        11 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fe588); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fe58f); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fe598); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008fe59f); };
            c.cmp(t22, c.r64(1), 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x70008fe5a8); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe5c1); };
            c.w64(0, t23);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008fe5c8); }
            let r24 = c.r32(2) | 0xeu64;
            c.w32(2, r24);
            c.logic(r24, 32);
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008fe5cf); }
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe5db); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fe5e4); };
            c.w32(11, t26);
            c.w32(2, c.r32(2));
            c.w32(1, c.r32(11));
            let r27 = c.r32(2) | c.r32(1);
            c.w32(2, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fe5f8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fe5fd); }
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe605); };
            c.w64(8, t28);
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x2));
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008fe610); };
            c.w32(11, t29);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fe626); };
            c.w64(1, t30);
            c.w64(12, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        20 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008fe62f); }
            let r31 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r31);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008fe63b); };
            c.w32(7, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fe640); };
            c.w64(1, t33);
            c.w64(12, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe649); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe64d); }
            let r34 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r34);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008fe657); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe65b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fe65f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fe663); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe667); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fe66b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008fe66f); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x70008fe67b, 0x70008fe676);
        }
        22 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe67b); };
            c.w64(11, t35);
            c.w32(2, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe684); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe688); };
            c.w64(7, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe68c); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe690); };
            c.w64(8, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe694); };
            c.w64(3, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fe69b); }
            blk = 3; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe6a1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe6a5); }
            let r41 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r41);
            c.w32(2, c.r32(2));
            let r42 = c.r32(2) | 0x40u64;
            c.w32(2, r42);
            c.logic(r42, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fe6b7); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fe6bb); }
            c.w32(0, 0x1cu64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fe6c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe6c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fe6c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fe6ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe6ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fe6d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008fe6d6); }
            c.w64(6, c.r64(11));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008fe6dd); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008fe6f1, 0x70008fe6ec);
        }
        24 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fe6f1); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe6f5); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe6f9); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe6fd); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe701); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe705); };
            c.w64(8, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fe70c); }
            blk = 15; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe712); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe716); }
            let r50 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008fe71c); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe725); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fe729); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fe72d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe731); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fe735); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008fe739); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fe746, 0x70008fe741);
        }
        26 => {
            c.w64(11, c.r64(0));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe749); };
            c.w64(15, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe74d); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe751); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe755); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe759); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe75d); };
            c.w64(2, t56);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008fe761); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fe769); }
            blk = 14; continue;
        }
        27 => {
            return RegfileExit::jump(0x70008fe770);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fe774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe777); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fe77b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fe782); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fe786); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fe78d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe793); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fe797); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe7a0); };
            c.w64(8, t9);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fe7af); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe7b5); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fe7be); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fe7d6); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fe7e3); }
            blk = 12; continue;
        }
        9 => {
            let r16 = c.add(c.r32(1), 0xfffffffffffffffeu64, 32);
            c.w32(1, r16);
            if c.cc_o() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(11, c.r32(2));
            let r17 = c.r32(11) | 0x2u64;
            c.w32(11, r17);
            c.logic(r17, 32);
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        11 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fe808); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fe80f); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fe818); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008fe81f); };
            c.cmp(t22, c.r64(1), 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x70008fe828); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.w64(10, 0xbff0000000000000u64);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fe842); };
            c.w64(0, t23);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008fe849); }
            let r24 = c.r32(2) | 0xeu64;
            c.w32(2, r24);
            c.logic(r24, 32);
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008fe850); }
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fe85c); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fe865); };
            c.w32(11, t26);
            c.w32(2, c.r32(2));
            c.w32(1, c.r32(11));
            let r27 = c.r32(2) | c.r32(1);
            c.w32(2, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fe879); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fe87e); }
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe886); };
            c.w64(8, t28);
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x2));
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008fe891); };
            c.w32(11, t29);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fe8a7); };
            c.w64(1, t30);
            c.w64(12, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        20 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008fe8b0); }
            let r31 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r31);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008fe8bc); };
            c.w32(7, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fe8c1); };
            c.w64(1, t33);
            c.w64(12, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe8ca); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe8ce); }
            let r34 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r34);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008fe8d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe8dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fe8e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fe8e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe8e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fe8ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008fe8f0); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x70008fe8fc, 0x70008fe8f7);
        }
        22 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe8fc); };
            c.w64(11, t35);
            c.w32(2, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe905); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe909); };
            c.w64(7, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe90d); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe911); };
            c.w64(8, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe915); };
            c.w64(3, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fe91c); }
            blk = 3; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe922); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe926); }
            let r41 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r41);
            c.w32(2, c.r32(2));
            let r42 = c.r32(2) | 0x40u64;
            c.w32(2, r42);
            c.logic(r42, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fe938); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fe93c); }
            c.w32(0, 0x1eu64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fe942); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe943); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fe947); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fe94b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe94f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fe953); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008fe957); }
            c.w64(6, c.r64(11));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008fe95e); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008fe972, 0x70008fe96d);
        }
        24 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fe972); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe976); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe97a); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe97e); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe982); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe986); };
            c.w64(8, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fe98d); }
            blk = 15; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fe993); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fe997); }
            let r50 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008fe99d); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fe9a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fe9aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fe9ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fe9b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fe9b6); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008fe9ba); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fe9c7, 0x70008fe9c2);
        }
        26 => {
            c.w64(11, c.r64(0));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fe9ca); };
            c.w64(15, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fe9ce); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fe9d2); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fe9d6); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fe9da); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fe9de); };
            c.w64(2, t56);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008fe9e2); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fe9ea); }
            blk = 14; continue;
        }
        27 => {
            return RegfileExit::jump(0x70008fe9f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fea14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fea17); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fea1b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fea22); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fea26); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fea2d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fea33); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fea37); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fea40); };
            c.w64(8, t9);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fea4f); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fea55); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fea5e); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fea76); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fea83); }
            blk = 13; continue;
        }
        9 => {
            c.w32(11, c.r32(2));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(6, 0x80000000u64);
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let r16 = c.neg(c.r32(1), 32);
            c.w32(1, r16);
            let r17 = c.r32(11) | 0x2u64;
            c.w32(11, r17);
            c.logic(r17, 32);
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 17; continue;
        }
        12 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008feab3); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008feaba); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let r21 = c.r32(2) | 0xeu64;
            c.w32(2, r21);
            c.logic(r21, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008feac9); };
            c.w64(6, t22);
            let Some(t23) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008fead0); };
            c.cmp(t23, c.r64(1), 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x70008fead9); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008feae0); };
            c.w64(10, t24);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008feae7); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008feaeb); };
            c.w64(0, t25);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008feaf2); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008feaf6); }
            c.w64(0, c.r64(11));
            blk = 17; continue;
        }
        16 => {
            let r26 = c.r32(11) | 0xeu64;
            c.w32(11, r26);
            c.logic(r26, 32);
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008feb08); };
            c.w64(0, t27);
            blk = 17; continue;
        }
        17 => {
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008feb0f); };
            c.cmp(t28, c.r32(8), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008feb18); };
            c.w32(11, t29);
            c.w32(2, c.r32(2));
            c.w32(1, c.r32(11));
            let r30 = c.r32(2) | c.r32(1);
            c.w32(2, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008feb2c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008feb31); }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008feb39); };
            c.w64(7, t31);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x2));
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008feb44); };
            c.w32(8, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008feb49); };
            c.w64(1, t33);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008feb52); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008feb56); }
            let r34 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r34);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008feb60); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008feb64); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008feb68); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008feb6c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008feb70); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008feb74); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008feb78); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x70008feb84, 0x70008feb7f);
        }
        22 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008feb84); };
            c.w64(11, t35);
            c.w32(2, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008feb8d); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008feb91); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008feb95); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008feb99); };
            c.w64(8, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008feb9d); };
            c.w64(3, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008feba4); }
            blk = 3; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008febaa); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008febae); }
            let r41 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r41);
            c.w32(2, c.r32(2));
            let r42 = c.r32(2) | 0x40u64;
            c.w32(2, r42);
            c.logic(r42, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008febc0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008febc4); }
            c.w32(0, 0x1au64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008febca); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008febcb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008febcf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008febd3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008febd7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008febdb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008febdf); }
            c.w64(6, c.r64(11));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008febe6); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008febfa, 0x70008febf5);
        }
        24 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008febfa); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008febfe); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fec02); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fec06); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fec0a); };
            c.w64(7, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fec0e); };
            c.w64(8, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fec15); }
            blk = 17; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fec1b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fec1f); }
            let r50 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008fec25); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fec2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fec32); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fec36); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fec3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fec3e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008fec42); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fec4f, 0x70008fec4a);
        }
        26 => {
            c.w64(11, c.r64(0));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fec52); };
            c.w64(15, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fec56); };
            c.w64(9, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fec5a); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fec5e); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fec62); };
            c.w64(3, t55);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008fec66); }
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fec6b); };
            c.w64(2, t56);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fec72); }
            blk = 15; continue;
        }
        27 => {
            return RegfileExit::jump(0x70008fec7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fec94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fec94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fec98); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fec9e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008feca2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008feca6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fecad); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fecb1); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fecb8); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fecbe); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fecc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fecc6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fecca); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fecce); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008fecd2); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fecd6); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fecdf); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fece6); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fecf3); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008fecf9); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fed01); };
            c.w32(6, t13);
            let r14 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fed07); };
            c.cmp(t15, c.r32(6), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fed10); };
            c.w32(1, t16);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fed28); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x70008fed34, 0x70008fed2f);
        }
        8 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fed34); };
            c.w64(3, t17);
            c.w32(1, 0xfeu64);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fed3d); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fed41); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fed45); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fed49); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fed4d); };
            c.w64(11, t22);
            blk = 3; continue;
        }
        9 => {
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fed53); };
            c.w32(0, t23);
            let r24 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r24);
            c.w32(1, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(11, c.r32(1));
            let r25 = c.r32(11) | 0x2u64;
            c.w32(11, r25);
            c.logic(r25, 32);
            let r26 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r26);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fed6e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(0));
            c.w32(11, c.r32(1));
            let r27 = c.r32(11) | 0xeu64;
            c.w32(11, r27);
            c.logic(r27, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        13 => {
            let r28 = !c.r32(0);
            c.w32(0, r28);
            c.w64(2, c.r64(0));
            let r29 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r29);
            if c.cc_o() { blk = 27; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(2, 0xeu64);
            blk = 18; continue;
        }
        17 => {
            c.w32(2, 0x2u64);
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fedae); };
            c.cmp(t30, 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fedb4); };
            c.w32(1, t31);
            c.w32(11, c.r32(11));
            c.w32(2, c.r32(2));
            c.w32(6, c.r32(1));
            let r32 = c.r32(11) | c.r32(2);
            c.w32(11, r32);
            c.logic(r32, 32);
            let r33 = c.r32(11) | c.r32(6);
            c.w32(11, r33);
            c.logic(r33, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fedce); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fedd3); }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008feddb); };
            c.w64(7, t34);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x2));
            let Some(t35) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fede6); };
            c.w32(8, t35);
            let Some(t36) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fedeb); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fedef); };
            c.w64(5, t37);
            c.w64(12, c.r64(7));
            let r38 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r38);
            return RegfileExit::jump(c.r64(1));
        }
        22 => {
            c.w64(2, c.r64(9));
            c.w32(1, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t39) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fee02); };
            c.cmp(t39, 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fee08); };
            c.w32(6, t40);
            c.w32(9, c.r32(6));
            let r41 = c.r32(9) | 0x40u64;
            c.w32(9, r41);
            c.logic(r41, 32);
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(6, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x70008fee1c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fee21); }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fee29); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fee2d); }
            c.w32(8, 0x18u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fee34); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008fee36); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008fee4a, 0x70008fee45);
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fee4a); };
            c.w64(7, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fee4e); };
            c.w64(15, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fee52); };
            c.w64(9, t45);
            blk = 21; continue;
        }
        27 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fee58); };
            c.w64(2, t46);
            let Some(t47) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008fee5f); };
            c.w64(1, t47);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008fee66); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008fee6a); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fee6e); };
            c.w64(12, t48);
            let Some(t49) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fee75); };
            c.cmp(t49, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fee85, 0x70008fee80);
        }
        29 => {
            c.w64(2, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fee88); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fee8c); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fee90); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fee94); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fee98); };
            c.w64(11, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fee9c); };
            c.w64(0, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008feea0); };
            c.w64(3, t56);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x70008feea6); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008feead); };
            c.w64(1, t57);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008feeb4); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008feebf); }
            c.w64(0, c.r64(2));
            blk = 15; continue;
        }
        32 => {
            let r58 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r58);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008feed0); }
            return c.call(0x70008708b4, 0x70008feeda, 0x70008feed5);
        }
        33 => {
            let Some(t59) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008feeda); };
            c.w32(0, t59);
            let r60 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r60);
            blk = 12; continue;
        }
        34 => {
            return RegfileExit::jump(0x70008feee8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008feef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008feef4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008fef03); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fef0c); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008fef12); };
            c.logic(t4 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fef18); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fef21); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008fef2f); };
            c.w64(3, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008fef38); };
            c.w64(3, t8);
            blk = 10; continue;
        }
        10 => {
            let r9 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008fef43); };
            c.w32(2, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fef48); };
            c.w64(1, t11);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fef51); };
            c.logic(t12 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 8; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fef5c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 9; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008fef70);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fef74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008fef74); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008fef7d); };
            c.w64(3, t1);
            blk = 3; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008fef86); };
            c.w64(3, t2);
            blk = 3; continue;
        }
        3 => {
            let r3 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008fef91); };
            c.w32(2, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fef96); };
            c.w64(1, t5);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008fefa0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fefb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fefbc); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fefc2); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fefcf); };
            c.w32(2, t3);
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008fefdd); };
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
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb30)) else { return RegfileExit::fault(0x70008fefee); };
            c.w64(3, t6);
            blk = 15; continue;
        }
        6 => {
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_ae() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_b() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xaa8)) else { return RegfileExit::fault(0x70008ff00a); };
            c.w64(3, t7);
            blk = 15; continue;
        }
        10 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x550)) else { return RegfileExit::fault(0x70008ff013); };
            c.w64(3, t8);
            blk = 15; continue;
        }
        11 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x70008ff01c); };
            c.w64(3, t9);
            blk = 15; continue;
        }
        12 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x918)) else { return RegfileExit::fault(0x70008ff025); };
            c.w64(3, t10);
            blk = 15; continue;
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x730)) else { return RegfileExit::fault(0x70008ff02e); };
            c.w64(3, t11);
            blk = 15; continue;
        }
        14 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff037); };
            c.w32(3, t12);
            let r13 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r13);
            blk = 15; continue;
        }
        15 => {
            let r14 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r14);
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008ff041); };
            c.w32(2, t15);
            c.w64(6, c.r64(2).wrapping_mul(8));
            c.cmp(c.r32(2), 0xb4u64, 32);
            if c.cc_ae() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff056); };
            c.w64(1, t16);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        17 => {
            c.w64(2, c.r64(5));
            if !c.st64(c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x648), c.r64(3)) { return RegfileExit::fault(0x70008ff062); }
            let r17 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r17);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008ff06e); };
            c.w32(2, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff073); };
            c.w64(1, t19);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x70008ff07c); };
            c.w64(3, t20);
            blk = 15; continue;
        }
        19 => {
            return RegfileExit::jump(0x70008ff088);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ff094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ff094); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ff098); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008ff09e); };
            c.w64(2, (t1 as u8 as i8 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ff0a4); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff0a8); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008ff0ac); };
            c.w64(8, t4);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008ff0b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008ff0b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008ff0bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008ff0c0); }
            c.w32(1, 0x2u64);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(8));
            return c.call(0x7000820e54, 0x70008ff0d7, 0x70008ff0d2);
        }
        1 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff0d7); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff0db); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff0df); };
            c.w64(7, t7);
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008ff0e7); };
            c.w32(7, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ff0ec); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff0f0); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ff0f4); };
            c.w64(5, t11);
            c.w64(12, c.r64(2));
            let r12 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r12);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008ff104);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ff114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ff114); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ff118); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008ff11e); };
            c.w64(2, (t1 as u8 as i8 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ff124); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff128); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008ff12c); };
            c.w64(8, t4);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008ff134); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008ff138); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008ff13c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008ff140); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(8));
            return c.call(0x7000820e54, 0x70008ff154, 0x70008ff14f);
        }
        1 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff154); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff158); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff15c); };
            c.w64(7, t7);
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008ff164); };
            c.w32(7, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ff169); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff16d); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ff171); };
            c.w64(5, t11);
            c.w64(12, c.r64(2));
            let r12 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r12);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008ff180);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ff194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff194); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008ff19a); };
            c.w64(2, (t2 as u8 as i8 as i64 as u64));
            c.w64(6, c.r64(5));
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ff1a3); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            if !c.st64(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(8)), c.r64(3)) { return RegfileExit::fault(0x70008ff1a9); }
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008ff1b1); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff1b6); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008ff1c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ff1d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ff1d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ff1d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ff1e2); };
            c.w64(1, t1);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008ff1e6); }
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ff1ea); };
            c.w64(2, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008ff1ee); };
            c.w64(8, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff1f4); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff1fa); };
            c.w64(8, t6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff1fe); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ff204); };
            c.w32(11, (t9 as u8 as i8 as i64 as u64));
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff20a); };
            c.w32(15, t10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008ff210); };
            c.w32(0, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff216); };
            c.w32(3, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008ff219); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008ff21d); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008ff221); };
            c.cmp(t13, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ff22a); };
            c.w64(2, t14);
            blk = 2; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008ff231); };
            c.w64(3, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff235); };
            c.w32(6, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008ff238); };
            c.cmp(t17, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff255); };
            c.w32(7, t18);
            let r19 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r19);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x70008ff25f); }
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ff263); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            c.w32(15, c.r32(7));
            let r22 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r22);
            c.logic(r22, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008ff271); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008ff27e); };
            c.cmp(t23, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008ff28b); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff2a1); };
            c.w32(15, t25);
            c.w64(11, c.r64(7));
            let r26 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r26);
            c.logic(r26, 64);
            c.w32(0, c.r32(15));
            let r27 = c.r32(0) & 0x3u64;
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff2d6); };
            c.w32(15, t28);
            let r29 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r29);
            let Some(t30) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff2dd); };
            c.w32(15, t30);
            let r31 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r31);
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff2e4); };
            c.w32(0, t32);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008ff2f4);
        }
        12 => {
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008ff2f5); };
            c.cmp(t33, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t34) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ff2ff); };
            c.w64(9, (t34 as u8 as i8 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t35) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff308); };
            c.w64(9, t35);
            let r36 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff319); };
            c.w32(9, t37);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r38 = c.r32(11) & 0x1u64;
            c.w32(11, r38);
            c.logic(r38, 32);
            let r39 = c.r32(12) & 0x3u64;
            c.w32(12, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r40 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r40);
            c.logic(r40, 64);
            let r41 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r41);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70008ff345); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff34a); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            let Some(t42) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ff360); };
            c.w32(9, t42);
            let r43 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r43);
            blk = 20; continue;
        }
        20 => {
            let Some(t44) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff367); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ff36e); };
            c.w32(11, t46);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r47 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r47);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ff390); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            let Some(t50) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff397); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff39e); };
            c.w32(9, t52);
            let r53 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r53);
            let Some(t54) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff3a5); };
            c.w32(11, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff3a9); };
            c.cmp(t55, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r56 = c.r64(9) | 0x3u64;
            c.w64(9, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff3ba); };
            c.w64(15, t57);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x70008ff3be); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r58 = c.r64(11) & c.r64(2);
            c.w64(11, r58);
            c.logic(r58, 64);
            let Some(t59) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff3d2); };
            c.logic(t59 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff3dd); }
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff3e4); };
            c.w64(11, t60);
            blk = 56; continue;
        }
        26 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff3ed); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff400); };
            c.w32(9, t62);
            let r63 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r63);
            let Some(t64) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ff407); };
            c.w32(9, t64);
            let r65 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r65);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff41d); };
            c.w32(9, t66);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff426); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            let Some(t69) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ff42d); };
            c.w32(12, t69);
            let r70 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r70);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff444); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r73 = c.r64(11) | 0x3u64;
            c.w64(11, r73);
            c.logic(r73, 64);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff458); };
            c.w64(15, t74);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x70008ff45c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r75 = c.r64(9) & c.r64(2);
            c.w64(9, r75);
            c.logic(r75, 64);
            let Some(t76) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff470); };
            c.logic(t76 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff47b); }
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff482); };
            c.w64(11, t77);
            blk = 56; continue;
        }
        35 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff48b); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        36 => {
            let Some(t79) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff494); };
            c.w32(15, t79);
            c.w32(7, c.r32(15));
            let r80 = c.r32(7) & 0x3u64;
            c.w32(7, r80);
            c.logic(r80, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r81 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r81);
            c.logic(r81, 64);
            let r82 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r82);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70008ff4b7); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff4bc); }
            blk = 39; continue;
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff4c3); };
            c.w32(15, t83);
            let r84 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r84);
            let Some(t85) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff4ca); };
            c.w32(15, t85);
            let r86 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r86);
            let Some(t87) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff4d1); };
            c.w32(7, t87);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x70008ff4e1);
        }
        41 => {
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008ff4e2); };
            c.cmp(t88, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t89) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ff4ed); };
            c.w64(9, (t89 as u8 as i8 as i64 as u64));
            let Some(t90) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff4f3); };
            c.w32(12, t90);
            let Some(t91) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff4f8); };
            c.w64(9, t91);
            c.w32(15, c.r32(12));
            let r92 = c.r32(15) & 0x3u64;
            c.w32(15, r92);
            c.logic(r92, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r93 = c.r64(12) | 0x2u64;
            c.w64(12, r93);
            c.logic(r93, 64);
            let r94 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r94);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70008ff51c); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff521); }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(12, c.r64(9));
            blk = 49; continue;
        }
        48 => {
            let Some(t95) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ff537); };
            c.w32(12, t95);
            let r96 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r96);
            blk = 49; continue;
        }
        49 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff53f); };
            c.w32(15, t97);
            let r98 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r98);
            let Some(t99) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ff547); };
            c.w32(15, t99);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r100 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r100);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t101) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ff565); };
            c.w32(12, t101);
            let r102 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r102);
            let Some(t103) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff56d); };
            c.w32(15, t103);
            let r104 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r104);
            let Some(t105) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff574); };
            c.w32(12, t105);
            let r106 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r106);
            let Some(t107) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff57c); };
            c.w32(15, t107);
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff580); };
            c.cmp(t108, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r109 = c.r64(9) | 0x3u64;
            c.w64(9, r109);
            c.logic(r109, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x70008ff58b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r110 = c.r64(12) & c.r64(2);
            c.w64(12, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff59f); };
            c.logic(t111 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff5a7); }
            blk = 56; continue;
        }
        55 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008ff5b0); };
            c.w64(9, t112);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x70008ff5b7); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff5bc); }
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff5c3); };
            c.w64(11, t113);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ff5ce); };
            c.w64(1, t114);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t115) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ff5d5); };
            c.w64(5, t115);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r116 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(0x70007fcbb4);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r117 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r117);
            c.w64(12, 0xfffffffffffc0000u64);
            let r118 = c.r64(12) & c.r64(9);
            c.w64(12, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff5fb); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ff607); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ff608); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x70008ff615, 0x70008ff610);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ff615); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ff617); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            c.w64(9, 0xfffffffffffc0000u64);
            let r121 = c.r64(9) & c.r64(11);
            c.w64(9, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff62a); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ff639); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ff63a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70008ff647, 0x70008ff642);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ff647); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ff649); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r123 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r123);
            c.w64(11, 0xfffffffffffc0000u64);
            let r124 = c.r64(11) & c.r64(9);
            c.w64(11, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff65f); };
            c.logic(t125 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ff66e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ff66f); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70008ff67c, 0x70008ff677);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ff67c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ff67e); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x70008ff688);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ff694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ff694); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ff698); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ff6a2); };
            c.w64(1, t1);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008ff6a6); }
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ff6aa); };
            c.w64(2, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008ff6ae); };
            c.w64(8, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff6b4); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff6ba); };
            c.w64(8, t6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff6be); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ff6c4); };
            c.w32(11, (t9 as u8 as i8 as i64 as u64));
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff6ca); };
            c.w32(15, t10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008ff6d0); };
            c.w32(0, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff6d6); };
            c.w32(3, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008ff6d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008ff6dd); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008ff6e1); };
            c.cmp(t13, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ff6ea); };
            c.w64(2, t14);
            blk = 2; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008ff6f1); };
            c.w64(3, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff6f5); };
            c.w32(6, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008ff6f8); };
            c.cmp(t17, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff715); };
            c.w32(7, t18);
            let r19 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r19);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x70008ff71f); }
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ff723); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            c.w32(15, c.r32(7));
            let r22 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r22);
            c.logic(r22, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008ff731); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008ff73e); };
            c.cmp(t23, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008ff74b); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff761); };
            c.w32(15, t25);
            c.w64(11, c.r64(7));
            let r26 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r26);
            c.logic(r26, 64);
            c.w32(0, c.r32(15));
            let r27 = c.r32(0) & 0x3u64;
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff796); };
            c.w32(15, t28);
            let r29 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r29);
            let Some(t30) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff79d); };
            c.w32(15, t30);
            let r31 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r31);
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff7a4); };
            c.w32(0, t32);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008ff7b4);
        }
        12 => {
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008ff7b5); };
            c.cmp(t33, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t34) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ff7bf); };
            c.w64(9, (t34 as u8 as i8 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t35) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff7c8); };
            c.w64(9, t35);
            let r36 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff7d9); };
            c.w32(9, t37);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r38 = c.r32(11) & 0x1u64;
            c.w32(11, r38);
            c.logic(r38, 32);
            let r39 = c.r32(12) & 0x3u64;
            c.w32(12, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r40 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r40);
            c.logic(r40, 64);
            let r41 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r41);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70008ff805); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff80a); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            let Some(t42) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ff820); };
            c.w32(9, t42);
            let r43 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r43);
            blk = 20; continue;
        }
        20 => {
            let Some(t44) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff827); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ff82e); };
            c.w32(11, t46);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r47 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r47);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ff850); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            let Some(t50) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff857); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff85e); };
            c.w32(9, t52);
            let r53 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r53);
            let Some(t54) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff865); };
            c.w32(11, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff869); };
            c.cmp(t55, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r56 = c.r64(9) | 0x3u64;
            c.w64(9, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff87a); };
            c.w64(15, t57);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x70008ff87e); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r58 = c.r64(11) & c.r64(2);
            c.w64(11, r58);
            c.logic(r58, 64);
            let Some(t59) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff892); };
            c.logic(t59 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff89d); }
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff8a4); };
            c.w64(11, t60);
            blk = 56; continue;
        }
        26 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff8ad); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff8c0); };
            c.w32(9, t62);
            let r63 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r63);
            let Some(t64) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ff8c7); };
            c.w32(9, t64);
            let r65 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r65);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff8dd); };
            c.w32(9, t66);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff8e6); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            let Some(t69) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ff8ed); };
            c.w32(12, t69);
            let r70 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r70);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff904); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r73 = c.r64(11) | 0x3u64;
            c.w64(11, r73);
            c.logic(r73, 64);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ff918); };
            c.w64(15, t74);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x70008ff91c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r75 = c.r64(9) & c.r64(2);
            c.w64(9, r75);
            c.logic(r75, 64);
            let Some(t76) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ff930); };
            c.logic(t76 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff93b); }
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff942); };
            c.w64(11, t77);
            blk = 56; continue;
        }
        35 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ff94b); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        36 => {
            let Some(t79) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff954); };
            c.w32(15, t79);
            c.w32(7, c.r32(15));
            let r80 = c.r32(7) & 0x3u64;
            c.w32(7, r80);
            c.logic(r80, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r81 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r81);
            c.logic(r81, 64);
            let r82 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r82);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70008ff977); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff97c); }
            blk = 39; continue;
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff983); };
            c.w32(15, t83);
            let r84 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r84);
            let Some(t85) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ff98a); };
            c.w32(15, t85);
            let r86 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r86);
            let Some(t87) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ff991); };
            c.w32(7, t87);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x70008ff9a1);
        }
        41 => {
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008ff9a2); };
            c.cmp(t88, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t89) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ff9ad); };
            c.w64(9, (t89 as u8 as i8 as i64 as u64));
            let Some(t90) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ff9b3); };
            c.w32(12, t90);
            let Some(t91) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ff9b8); };
            c.w64(9, t91);
            c.w32(15, c.r32(12));
            let r92 = c.r32(15) & 0x3u64;
            c.w32(15, r92);
            c.logic(r92, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r93 = c.r64(12) | 0x2u64;
            c.w64(12, r93);
            c.logic(r93, 64);
            let r94 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r94);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70008ff9dc); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ff9e1); }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(12, c.r64(9));
            blk = 49; continue;
        }
        48 => {
            let Some(t95) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ff9f7); };
            c.w32(12, t95);
            let r96 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r96);
            blk = 49; continue;
        }
        49 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ff9ff); };
            c.w32(15, t97);
            let r98 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r98);
            let Some(t99) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ffa07); };
            c.w32(15, t99);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r100 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r100);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t101) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ffa25); };
            c.w32(12, t101);
            let r102 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r102);
            let Some(t103) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffa2d); };
            c.w32(15, t103);
            let r104 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r104);
            let Some(t105) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffa34); };
            c.w32(12, t105);
            let r106 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r106);
            let Some(t107) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffa3c); };
            c.w32(15, t107);
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffa40); };
            c.cmp(t108, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r109 = c.r64(9) | 0x3u64;
            c.w64(9, r109);
            c.logic(r109, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x70008ffa4b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r110 = c.r64(12) & c.r64(2);
            c.w64(12, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ffa5f); };
            c.logic(t111 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffa67); }
            blk = 56; continue;
        }
        55 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008ffa70); };
            c.w64(9, t112);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x70008ffa77); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffa7c); }
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ffa83); };
            c.w64(11, t113);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ffa8e); };
            c.w64(1, t114);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t115) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ffa95); };
            c.w64(5, t115);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r116 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(0x70007fcbb4);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r117 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r117);
            c.w64(12, 0xfffffffffffc0000u64);
            let r118 = c.r64(12) & c.r64(9);
            c.w64(12, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ffabb); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ffac7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ffac8); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x70008ffad5, 0x70008ffad0);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ffad5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ffad7); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            c.w64(9, 0xfffffffffffc0000u64);
            let r121 = c.r64(9) & c.r64(11);
            c.w64(9, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ffaea); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ffaf9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ffafa); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70008ffb07, 0x70008ffb02);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ffb07); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ffb09); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r123 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r123);
            c.w64(11, 0xfffffffffffc0000u64);
            let r124 = c.r64(11) & c.r64(9);
            c.w64(11, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ffb1f); };
            c.logic(t125 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ffb2e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ffb2f); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70008ffb3c, 0x70008ffb37);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ffb3c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ffb3e); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x70008ffb48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ffb54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ffb54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ffb58); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ffb62); };
            c.w64(2, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008ffb66); }
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ffb6a); };
            c.w64(3, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008ffb6e); };
            c.w64(1, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffb74); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ffb7a); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ffb7e); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ffb84); };
            c.w32(8, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffb8a); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008ffb8e); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ffb97); };
            c.w64(3, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008ffb9e); };
            c.w64(11, t13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffba2); };
            c.w32(15, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008ffba6); };
            c.cmp(t15, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ffbc3); };
            c.w32(0, t16);
            let r17 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r17);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70008ffbce); }
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ffbd3); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            c.w32(6, c.r32(0));
            let r20 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r20);
            c.logic(r20, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008ffbe0); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008ffbec); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008ffbf9); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ffc0f); };
            c.w32(6, t23);
            c.w64(7, c.r64(0));
            let r24 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r24);
            c.logic(r24, 64);
            c.w32(8, c.r32(6));
            let r25 = c.r32(8) & 0x3u64;
            c.w32(8, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffc46); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffc4d); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ffc54); };
            c.w32(6, t30);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008ffc64);
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008ffc65); };
            c.cmp(t31, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ffc6f); };
            c.w64(8, (t32 as u8 as i8 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t33) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ffc77); };
            c.w64(8, t33);
            let r34 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ffc87); };
            c.w32(8, t35);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r36 = c.r32(0) & 0x1u64;
            c.w32(0, r36);
            c.logic(r36, 32);
            let r37 = c.r32(6) & 0x3u64;
            c.w32(6, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r38 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r38);
            c.logic(r38, 64);
            let r39 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x70008ffcaf); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffcb4); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ffcc9); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            blk = 20; continue;
        }
        20 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffcd0); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            let Some(t44) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ffcd7); };
            c.w32(0, t44);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r45 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r45);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ffcf2); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffcf9); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffd00); };
            c.w32(8, t50);
            let r51 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffd07); };
            c.w32(0, t52);
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffd0a); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r54 = c.r64(8) | 0x3u64;
            c.w64(8, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ffd1b); };
            c.w64(0, t55);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x70008ffd1f); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r56 = c.r64(15) & c.r64(3);
            c.w64(15, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ffd33); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffd3e); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t58) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffd53); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            let Some(t60) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ffd5a); };
            c.w32(8, t60);
            let r61 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r61);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffd70); };
            c.w32(8, t62);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffd7d); };
            c.w32(0, t63);
            let r64 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r64);
            let Some(t65) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ffd83); };
            c.w32(0, t65);
            let r66 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r66);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffd95); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r68);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r69 = c.r64(0) | 0x3u64;
            c.w64(0, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ffda8); };
            c.w64(8, t70);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008ffdac); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r71 = c.r64(15) & c.r64(3);
            c.w64(15, r71);
            c.logic(r71, 64);
            let Some(t72) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ffdbe); };
            c.logic(t72 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffdc9); }
            blk = 54; continue;
        }
        34 => {
            let Some(t73) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ffdd5); };
            c.w32(0, t73);
            c.w32(6, c.r32(0));
            let r74 = c.r32(6) & 0x3u64;
            c.w32(6, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r75 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r75);
            c.logic(r75, 64);
            let r76 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r76);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70008ffdf4); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffdf9); }
            blk = 37; continue;
        }
        37 => {
            let Some(t77) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffe00); };
            c.w32(0, t77);
            let r78 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r78);
            let Some(t79) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffe07); };
            c.w32(0, t79);
            let r80 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r80);
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ffe0d); };
            c.w32(6, t81);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x70008ffe1c);
        }
        39 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008ffe1d); };
            c.cmp(t82, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t83) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ffe27); };
            c.w64(0, (t83 as u8 as i8 as i64 as u64));
            let Some(t84) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ffe2d); };
            c.w32(6, t84);
            let Some(t85) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ffe32); };
            c.w64(0, t85);
            c.w32(7, c.r32(6));
            let r86 = c.r32(7) & 0x3u64;
            c.w32(7, r86);
            c.logic(r86, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r87 = c.r64(6) | 0x2u64;
            c.w64(6, r87);
            c.logic(r87, 64);
            let r88 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r88);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x70008ffe52); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffe57); }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(6, c.r64(0));
            blk = 47; continue;
        }
        46 => {
            let Some(t89) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ffe6b); };
            c.w32(6, t89);
            let r90 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r90);
            blk = 47; continue;
        }
        47 => {
            let Some(t91) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffe71); };
            c.w32(7, t91);
            let r92 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r92);
            let Some(t93) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ffe77); };
            c.w32(7, t93);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r94 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r94);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t95) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ffe90); };
            c.w32(6, t95);
            let r96 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r96);
            let Some(t97) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffe96); };
            c.w32(7, t97);
            let r98 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r98);
            let Some(t99) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ffe9d); };
            c.w32(6, t99);
            let r100 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r100);
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffea3); };
            c.w32(7, t101);
            let Some(t102) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ffea6); };
            c.cmp(t102, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r103 = c.r64(0) | 0x3u64;
            c.w64(0, r103);
            c.logic(r103, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008ffeaf); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r104 = c.r64(15) & c.r64(3);
            c.w64(15, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ffec1); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffec8); }
            blk = 54; continue;
        }
        53 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008ffed1); };
            c.w64(8, t106);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008ffed8); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008ffedd); }
            blk = 54; continue;
        }
        54 => {
            let Some(t107) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008ffee4); };
            c.w64(3, (t107 as u8 as i8 as i64 as u64));
            let Some(t108) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ffeea); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ffeee); };
            c.w64(5, t109);
            let r110 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r110);
            let Some(t111) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008ffef6); };
            if !c.push64(t111) { return RegfileExit::fault(0x70008ffef6); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x70008ffefe); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r112 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r112);
            c.w64(15, 0xfffffffffffc0000u64);
            let r113 = c.r64(15) & c.r64(0);
            c.w64(15, r113);
            c.logic(r113, 64);
            let Some(t114) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008fff1a); };
            c.logic(t114 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fff25); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fff26); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70008fff33, 0x70008fff2e);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008fff33); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008fff35); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r115 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r115);
            c.w64(15, 0xfffffffffffc0000u64);
            let r116 = c.r64(15) & c.r64(0);
            c.w64(15, r116);
            c.logic(r116, 64);
            let Some(t117) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008fff47); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fff56); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fff57); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70008fff64, 0x70008fff5f);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008fff64); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008fff66); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r118 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r118);
            c.w64(15, 0xfffffffffffc0000u64);
            let r119 = c.r64(15) & c.r64(8);
            c.w64(15, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008fff7c); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fff8b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fff8c); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70008fff99, 0x70008fff94);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008fff99); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008fff9b); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x70008fffa4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fffb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fffb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fffb8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fffc2); };
            c.w64(2, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008fffc6); }
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fffca); };
            c.w64(3, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fffce); };
            c.w64(1, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fffd4); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fffda); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fffde); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008fffe4); };
            c.w32(8, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fffea); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fffee); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ffff7); };
            c.w64(3, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008ffffe); };
            c.w64(11, t13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900002); };
            c.w32(15, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000900006); };
            c.cmp(t15, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900023); };
            c.w32(0, t16);
            let r17 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r17);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700090002e); }
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000900033); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            c.w32(6, c.r32(0));
            let r20 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r20);
            c.logic(r20, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000900040); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700090004c); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000900059); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090006f); };
            c.w32(6, t23);
            c.w64(7, c.r64(0));
            let r24 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r24);
            c.logic(r24, 64);
            c.w32(8, c.r32(6));
            let r25 = c.r32(8) & 0x3u64;
            c.w32(8, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009000a6); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009000ad); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009000b4); };
            c.w32(6, t30);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009000c4);
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009000c5); };
            c.cmp(t31, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009000cf); };
            c.w64(8, (t32 as u8 as i8 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t33) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009000d7); };
            c.w64(8, t33);
            let r34 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009000e7); };
            c.w32(8, t35);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r36 = c.r32(0) & 0x1u64;
            c.w32(0, r36);
            c.logic(r36, 32);
            let r37 = c.r32(6) & 0x3u64;
            c.w32(6, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r38 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r38);
            c.logic(r38, 64);
            let r39 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x700090010f); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900114); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000900129); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            blk = 20; continue;
        }
        20 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900130); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            let Some(t44) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900137); };
            c.w32(0, t44);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r45 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r45);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000900152); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900159); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900160); };
            c.w32(8, t50);
            let r51 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900167); };
            c.w32(0, t52);
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090016a); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r54 = c.r64(8) | 0x3u64;
            c.w64(8, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090017b); };
            c.w64(0, t55);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x700090017f); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r56 = c.r64(15) & c.r64(3);
            c.w64(15, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900193); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090019e); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t58) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009001b3); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            let Some(t60) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009001ba); };
            c.w32(8, t60);
            let r61 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r61);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009001d0); };
            c.w32(8, t62);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009001dd); };
            c.w32(0, t63);
            let r64 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r64);
            let Some(t65) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009001e3); };
            c.w32(0, t65);
            let r66 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r66);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009001f5); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r68);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r69 = c.r64(0) | 0x3u64;
            c.w64(0, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000900208); };
            c.w64(8, t70);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700090020c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r71 = c.r64(15) & c.r64(3);
            c.w64(15, r71);
            c.logic(r71, 64);
            let Some(t72) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090021e); };
            c.logic(t72 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900229); }
            blk = 54; continue;
        }
        34 => {
            let Some(t73) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900235); };
            c.w32(0, t73);
            c.w32(6, c.r32(0));
            let r74 = c.r32(6) & 0x3u64;
            c.w32(6, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r75 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r75);
            c.logic(r75, 64);
            let r76 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r76);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000900254); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900259); }
            blk = 37; continue;
        }
        37 => {
            let Some(t77) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900260); };
            c.w32(0, t77);
            let r78 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r78);
            let Some(t79) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900267); };
            c.w32(0, t79);
            let r80 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r80);
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090026d); };
            c.w32(6, t81);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700090027c);
        }
        39 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700090027d); };
            c.cmp(t82, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t83) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000900287); };
            c.w64(0, (t83 as u8 as i8 as i64 as u64));
            let Some(t84) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090028d); };
            c.w32(6, t84);
            let Some(t85) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000900292); };
            c.w64(0, t85);
            c.w32(7, c.r32(6));
            let r86 = c.r32(7) & 0x3u64;
            c.w32(7, r86);
            c.logic(r86, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r87 = c.r64(6) | 0x2u64;
            c.w64(6, r87);
            c.logic(r87, 64);
            let r88 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r88);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x70009002b2); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009002b7); }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(6, c.r64(0));
            blk = 47; continue;
        }
        46 => {
            let Some(t89) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009002cb); };
            c.w32(6, t89);
            let r90 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r90);
            blk = 47; continue;
        }
        47 => {
            let Some(t91) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009002d1); };
            c.w32(7, t91);
            let r92 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r92);
            let Some(t93) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009002d7); };
            c.w32(7, t93);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r94 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r94);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t95) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009002f0); };
            c.w32(6, t95);
            let r96 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r96);
            let Some(t97) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009002f6); };
            c.w32(7, t97);
            let r98 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r98);
            let Some(t99) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009002fd); };
            c.w32(6, t99);
            let r100 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r100);
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900303); };
            c.w32(7, t101);
            let Some(t102) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900306); };
            c.cmp(t102, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r103 = c.r64(0) | 0x3u64;
            c.w64(0, r103);
            c.logic(r103, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700090030f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r104 = c.r64(15) & c.r64(3);
            c.w64(15, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900321); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900328); }
            blk = 54; continue;
        }
        53 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000900331); };
            c.w64(8, t106);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000900338); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090033d); }
            blk = 54; continue;
        }
        54 => {
            let Some(t107) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000900344); };
            c.w64(3, (t107 as u8 as i8 as i64 as u64));
            let Some(t108) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090034a); };
            c.w64(8, (t108 as u8 as i8 as i64 as u64));
            let Some(t109) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000900350); };
            c.w64(3, t109);
            let Some(t110) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000900354); };
            c.w64(2, t110);
            let Some(t111) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000900358); };
            c.w64(5, t111);
            let r112 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r112);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000900360); }
            let Some(t113) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900361); };
            if !c.push64(t113) { return RegfileExit::fault(0x7000900361); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x700090036d); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r114 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r114);
            c.w64(15, 0xfffffffffffc0000u64);
            let r115 = c.r64(15) & c.r64(0);
            c.w64(15, r115);
            c.logic(r115, 64);
            let Some(t116) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900389); };
            c.logic(t116 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000900394); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000900395); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70009003a2, 0x700090039d);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009003a2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009003a4); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(15, 0xfffffffffffc0000u64);
            let r118 = c.r64(15) & c.r64(0);
            c.w64(15, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009003b6); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009003c5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009003c6); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70009003d3, 0x70009003ce);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009003d3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009003d5); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r120 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r120);
            c.w64(15, 0xfffffffffffc0000u64);
            let r121 = c.r64(15) & c.r64(8);
            c.w64(15, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009003eb); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009003fa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009003fb); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000900408, 0x7000900403);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000900408); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700090040a); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x7000900414);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000900434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000900434); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000900438); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000900442); };
            c.w64(2, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000900446); }
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090044a); };
            c.w64(3, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090044e); };
            c.w64(1, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900454); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700090045a); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090045e); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000900464); };
            c.w32(8, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090046a); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090046e); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000900477); };
            c.w64(3, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090047e); };
            c.w64(11, t13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900482); };
            c.w32(15, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000900486); };
            c.cmp(t15, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009004a3); };
            c.w32(0, t16);
            let r17 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r17);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009004ae); }
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009004b3); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            c.w32(6, c.r32(0));
            let r20 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r20);
            c.logic(r20, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70009004c0); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009004cc); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70009004d9); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009004ef); };
            c.w32(6, t23);
            c.w64(7, c.r64(0));
            let r24 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r24);
            c.logic(r24, 64);
            c.w32(8, c.r32(6));
            let r25 = c.r32(8) & 0x3u64;
            c.w32(8, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900526); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090052d); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000900534); };
            c.w32(6, t30);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000900544);
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000900545); };
            c.cmp(t31, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x700090054f); };
            c.w64(8, (t32 as u8 as i8 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t33) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000900557); };
            c.w64(8, t33);
            let r34 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900567); };
            c.w32(8, t35);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r36 = c.r32(0) & 0x1u64;
            c.w32(0, r36);
            c.logic(r36, 32);
            let r37 = c.r32(6) & 0x3u64;
            c.w32(6, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r38 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r38);
            c.logic(r38, 64);
            let r39 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x700090058f); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900594); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009005a9); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            blk = 20; continue;
        }
        20 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009005b0); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            let Some(t44) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009005b7); };
            c.w32(0, t44);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r45 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r45);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009005d2); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009005d9); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009005e0); };
            c.w32(8, t50);
            let r51 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009005e7); };
            c.w32(0, t52);
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009005ea); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r54 = c.r64(8) | 0x3u64;
            c.w64(8, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009005fb); };
            c.w64(0, t55);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x70009005ff); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r56 = c.r64(15) & c.r64(3);
            c.w64(15, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900613); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090061e); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t58) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900633); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            let Some(t60) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090063a); };
            c.w32(8, t60);
            let r61 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r61);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900650); };
            c.w32(8, t62);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090065d); };
            c.w32(0, t63);
            let r64 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r64);
            let Some(t65) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900663); };
            c.w32(0, t65);
            let r66 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r66);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900675); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r68);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r69 = c.r64(0) | 0x3u64;
            c.w64(0, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000900688); };
            c.w64(8, t70);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700090068c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r71 = c.r64(15) & c.r64(3);
            c.w64(15, r71);
            c.logic(r71, 64);
            let Some(t72) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090069e); };
            c.logic(t72 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009006a9); }
            blk = 54; continue;
        }
        34 => {
            let Some(t73) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009006b5); };
            c.w32(0, t73);
            c.w32(6, c.r32(0));
            let r74 = c.r32(6) & 0x3u64;
            c.w32(6, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r75 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r75);
            c.logic(r75, 64);
            let r76 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r76);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009006d4); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009006d9); }
            blk = 37; continue;
        }
        37 => {
            let Some(t77) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009006e0); };
            c.w32(0, t77);
            let r78 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r78);
            let Some(t79) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009006e7); };
            c.w32(0, t79);
            let r80 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r80);
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009006ed); };
            c.w32(6, t81);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x70009006fc);
        }
        39 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009006fd); };
            c.cmp(t82, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t83) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000900707); };
            c.w64(0, (t83 as u8 as i8 as i64 as u64));
            let Some(t84) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090070d); };
            c.w32(6, t84);
            let Some(t85) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000900712); };
            c.w64(0, t85);
            c.w32(7, c.r32(6));
            let r86 = c.r32(7) & 0x3u64;
            c.w32(7, r86);
            c.logic(r86, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r87 = c.r64(6) | 0x2u64;
            c.w64(6, r87);
            c.logic(r87, 64);
            let r88 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r88);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x7000900732); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900737); }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(6, c.r64(0));
            blk = 47; continue;
        }
        46 => {
            let Some(t89) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090074b); };
            c.w32(6, t89);
            let r90 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r90);
            blk = 47; continue;
        }
        47 => {
            let Some(t91) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900751); };
            c.w32(7, t91);
            let r92 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r92);
            let Some(t93) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900757); };
            c.w32(7, t93);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r94 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r94);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t95) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000900770); };
            c.w32(6, t95);
            let r96 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r96);
            let Some(t97) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900776); };
            c.w32(7, t97);
            let r98 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r98);
            let Some(t99) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090077d); };
            c.w32(6, t99);
            let r100 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r100);
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900783); };
            c.w32(7, t101);
            let Some(t102) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900786); };
            c.cmp(t102, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r103 = c.r64(0) | 0x3u64;
            c.w64(0, r103);
            c.logic(r103, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700090078f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r104 = c.r64(15) & c.r64(3);
            c.w64(15, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009007a1); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009007a8); }
            blk = 54; continue;
        }
        53 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009007b1); };
            c.w64(8, t106);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70009007b8); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009007bd); }
            blk = 54; continue;
        }
        54 => {
            let Some(t107) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009007c4); };
            c.w64(3, (t107 as u8 as i8 as i64 as u64));
            let Some(t108) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009007ca); };
            c.w64(8, (t108 as u8 as i8 as i64 as u64));
            let Some(t109) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70009007d0); };
            c.w64(9, (t109 as u8 as i8 as i64 as u64));
            let Some(t110) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009007d6); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009007da); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70009007de); };
            c.w64(2, t112);
            let Some(t113) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009007e2); };
            c.w64(5, t113);
            let r114 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r114);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009007ea); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009007ec); }
            let Some(t115) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70009007ed); };
            if !c.push64(t115) { return RegfileExit::fault(0x70009007ed); }
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x70009007f9); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r116 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r116);
            c.w64(15, 0xfffffffffffc0000u64);
            let r117 = c.r64(15) & c.r64(0);
            c.w64(15, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900815); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000900820); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000900821); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700090082e, 0x7000900829);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700090082e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000900830); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r119 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r119);
            c.w64(15, 0xfffffffffffc0000u64);
            let r120 = c.r64(15) & c.r64(0);
            c.w64(15, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900845); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000900854); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000900855); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000900862, 0x700090085d);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000900862); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000900864); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r122 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r122);
            c.w64(15, 0xfffffffffffc0000u64);
            let r123 = c.r64(15) & c.r64(8);
            c.w64(15, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090087a); };
            c.logic(t124 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000900889); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090088a); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000900897, 0x7000900892);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000900897); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000900899); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x70009008a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009008b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009008b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009008b8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009008c2); };
            c.w64(1, t1);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009008c6); }
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009008ca); };
            c.w64(2, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009008ce); };
            c.w64(8, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009008d4); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009008da); };
            c.w64(8, t6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009008de); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009008e4); };
            c.w32(11, (t9 as u8 as i8 as i64 as u64));
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009008ea); };
            c.w32(0, t10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70009008f0); };
            c.w32(9, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009008f6); };
            c.w32(12, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70009008fa); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009008fe); };
            c.cmp(t13, c.r32(12), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000900907); };
            c.w64(2, t14);
            blk = 2; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090090e); };
            c.w64(12, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900912); };
            c.w32(15, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000900916); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(9).wrapping_mul(4));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900933); };
            c.w32(3, t18);
            let r19 = c.add(c.r32(3), 0x8u64, 32);
            c.w32(3, r19);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x700090093e); }
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000900943); };
            c.w32(3, t20);
            let r21 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r21);
            c.w32(6, c.r32(3));
            let r22 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(8), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000900959); };
            c.cmp(t23, c.r32(3), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000900966); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090097c); };
            c.w32(6, t25);
            c.w64(7, c.r64(3));
            let r26 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r26);
            c.logic(r26, 64);
            c.w32(0, c.r32(6));
            let r27 = c.r32(0) & 0x3u64;
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009009ae); };
            c.w32(0, t28);
            let r29 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r29);
            let Some(t30) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009009b6); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            let Some(t32) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009009bc); };
            c.w32(6, t32);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009009cb);
        }
        12 => {
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009009cc); };
            c.cmp(t33, c.r32(8), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(0, c.r32(3));
            let r34 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r34);
            c.logic(r34, 32);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009009db); };
            c.w64(3, t35);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009009ea); };
            c.w32(0, t36);
            c.w32(3, c.r32(8));
            c.w32(6, c.r32(0));
            let r37 = c.r32(3) & 0x1u64;
            c.w32(3, r37);
            c.logic(r37, 32);
            let r38 = c.r32(6) & 0x3u64;
            c.w32(6, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r39 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r39);
            c.logic(r39, 64);
            let r40 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r40);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000900a0f); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900a14); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000900a29); };
            c.w32(0, t41);
            let r42 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r42);
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900a2f); };
            c.w32(3, t43);
            let r44 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r44);
            let Some(t45) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900a35); };
            c.w32(3, t45);
            c.cmp(c.r16(3), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(3, c.r16(3));
            let r46 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r46);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000900a52); };
            c.w32(0, t47);
            let r48 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r48);
            let Some(t49) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900a58); };
            c.w32(3, t49);
            let r50 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r50);
            let Some(t51) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900a60); };
            c.w32(0, t51);
            let r52 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r52);
            let Some(t53) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900a66); };
            c.w32(3, t53);
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900a69); };
            c.cmp(t54, c.r32(3), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(15, c.r64(8));
            let r55 = c.r64(15) | 0x3u64;
            c.w64(15, r55);
            c.logic(r55, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x7000900a79); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r56 = c.r64(0) & c.r64(2);
            c.w64(0, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900a8d); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900a97); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t58) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900aad); };
            c.w32(0, t58);
            let r59 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r59);
            let Some(t60) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900ab4); };
            c.w32(0, t60);
            let r61 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r61);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900ac6); };
            c.w32(0, t62);
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900ad2); };
            c.w32(3, t63);
            let r64 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r64);
            let Some(t65) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900ad8); };
            c.w32(3, t65);
            let r66 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r66);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900aeb); };
            c.w32(3, t67);
            let r68 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r68);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r69 = c.r64(3) | 0x3u64;
            c.w64(3, r69);
            c.logic(r69, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(3)) { return RegfileExit::fault(0x7000900afd); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r70 = c.r64(15) & c.r64(2);
            c.w64(15, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900b10); };
            c.logic(t71 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900b1b); }
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000900b22); };
            c.w64(0, t72);
            blk = 55; continue;
        }
        34 => {
            let Some(t73) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900b2b); };
            c.w32(3, t73);
            c.w32(6, c.r32(3));
            let r74 = c.r32(6) & 0x3u64;
            c.w32(6, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r75 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r75);
            c.logic(r75, 64);
            let r76 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r76);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x7000900b4b); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900b50); }
            blk = 37; continue;
        }
        37 => {
            let Some(t77) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900b57); };
            c.w32(3, t77);
            let r78 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r78);
            let Some(t79) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900b5f); };
            c.w32(3, t79);
            let r80 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r80);
            let Some(t81) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000900b65); };
            c.w32(6, t81);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x7000900b74);
        }
        39 => {
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000900b75); };
            c.cmp(t82, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(3, c.r64(8));
            blk = 45; continue;
        }
        41 => {
            let Some(t83) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900b80); };
            c.w32(3, t83);
            c.w32(6, c.r32(3));
            let r84 = c.r32(6) & 0x3u64;
            c.w32(6, r84);
            c.logic(r84, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r85 = c.r64(3) | 0x2u64;
            c.w64(3, r85);
            c.logic(r85, 64);
            let r86 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r86);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x7000900ba0); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900ba5); }
            blk = 44; continue;
        }
        44 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000900bac); };
            c.w64(3, t87);
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(6, c.r64(3));
            blk = 48; continue;
        }
        47 => {
            let Some(t88) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000900bc1); };
            c.w32(6, t88);
            let r89 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r89);
            blk = 48; continue;
        }
        48 => {
            let Some(t90) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900bc7); };
            c.w32(7, t90);
            let r91 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r91);
            let Some(t92) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900bcd); };
            c.w32(7, t92);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r16(7));
            let r93 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r93);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t94) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000900be6); };
            c.w32(6, t94);
            let r95 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r95);
            let Some(t96) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900bec); };
            c.w32(7, t96);
            let r97 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r97);
            let Some(t98) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900bf4); };
            c.w32(6, t98);
            let r99 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r99);
            let Some(t100) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900bfa); };
            c.w32(7, t100);
            let Some(t101) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900bfd); };
            c.cmp(t101, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r102 = c.r64(3) | 0x3u64;
            c.w64(3, r102);
            c.logic(r102, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(3)) { return RegfileExit::fault(0x7000900c06); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r103 = c.r64(15) & c.r64(2);
            c.w64(15, r103);
            c.logic(r103, 64);
            let Some(t104) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900c19); };
            c.logic(t104 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900c20); }
            blk = 55; continue;
        }
        54 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000900c29); };
            c.w64(9, t105);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x7000900c30); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900c35); }
            blk = 55; continue;
        }
        55 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t106) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000900c43); };
            c.w64(5, t106);
            let Some(t107) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000900c47); };
            c.w64(0, t107);
            c.w64(6, c.r64(12));
            c.w64(7, c.r64(8));
            let r108 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r108);
            return RegfileExit::jump(0x70007fcc14);
        }
        56 => {
            c.w32(3, c.r32(3));
            let r109 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r109);
            c.w64(15, 0xfffffffffffc0000u64);
            let r110 = c.r64(15) & c.r64(3);
            c.w64(15, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900c69); };
            c.logic(t111 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000900c74); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000900c75); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000900c82, 0x7000900c7d);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000900c82); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000900c84); }
            blk = 53; continue;
        }
        59 => {
            c.w32(3, c.r32(3));
            let r112 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r112);
            c.w64(15, 0xfffffffffffc0000u64);
            let r113 = c.r64(15) & c.r64(3);
            c.w64(15, r113);
            c.logic(r113, 64);
            let Some(t114) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900c96); };
            c.logic(t114 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000900ca5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000900ca6); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x7000900cb3, 0x7000900cae);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000900cb3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000900cb5); }
            blk = 33; continue;
        }
        62 => {
            c.w32(15, c.r32(15));
            let r115 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r115);
            c.w64(0, 0xfffffffffffc0000u64);
            let r116 = c.r64(0) & c.r64(15);
            c.w64(0, r116);
            c.logic(r116, 64);
            let Some(t117) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900ccb); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(3, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000900cd9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000900cda); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(3));
            return c.call(0x70007f1594, 0x7000900ce7, 0x7000900ce2);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000900ce7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000900ce9); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000900cf0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000900cf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(2, c.r64(5));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000900cfb); }
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000900cff); };
            c.w64(3, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000900d03); };
            c.w64(1, (t1 as u8 as i8 as i64 as u64));
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900d09); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x7000900d0f); };
            c.w64(1, t4);
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000900d13); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000900d19); };
            c.w32(8, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900d1f); };
            c.w32(9, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000900d23); };
            c.cmp(t9, c.r32(9), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000900d2c); };
            c.w64(3, t10);
            blk = 2; continue;
        }
        2 => {
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000900d33); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900d37); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000900d3b); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900d58); };
            c.w32(11, t14);
            let r15 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r15);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000900d64); }
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000900d69); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            c.w32(12, c.r32(11));
            let r18 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000900d81); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000900d8e); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900da5); };
            c.w32(12, t21);
            c.w64(15, c.r64(11));
            let r22 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r22);
            c.logic(r22, 64);
            c.w32(0, c.r32(12));
            let r23 = c.r32(0) & 0x3u64;
            c.w32(0, r23);
            c.logic(r23, 32);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t24) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900dda); };
            c.w32(12, t24);
            let r25 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r25);
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900de1); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000900de9); };
            c.w32(0, t28);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000900dfa);
        }
        12 => {
            let Some(t29) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000900dfb); };
            c.cmp(t29, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(11, c.r32(11));
            let r30 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r30);
            c.logic(r30, 32);
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000900e0d); };
            c.w64(12, t31);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900e1d); };
            c.w32(11, t32);
            c.w32(12, c.r32(1));
            c.w32(15, c.r32(11));
            let r33 = c.r32(12) & 0x1u64;
            c.w32(12, r33);
            c.logic(r33, 32);
            let r34 = c.r32(15) & 0x3u64;
            c.w32(15, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r35 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r35);
            c.logic(r35, 64);
            let r36 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r36);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000900e49); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900e4e); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(11, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t37) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000900e64); };
            c.w32(11, t37);
            let r38 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r38);
            blk = 20; continue;
        }
        20 => {
            let Some(t39) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900e6b); };
            c.w32(12, t39);
            let r40 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r40);
            let Some(t41) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900e72); };
            c.w32(12, t41);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r42 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r42);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000900e95); };
            c.w32(11, t43);
            let r44 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900e9c); };
            c.w32(12, t45);
            let r46 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r46);
            let Some(t47) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900ea3); };
            c.w32(11, t47);
            let r48 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r48);
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900eaa); };
            c.w32(12, t49);
            let Some(t50) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900eaf); };
            c.cmp(t50, c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(1));
            let r51 = c.r64(9) | 0x3u64;
            c.w64(9, r51);
            c.logic(r51, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x7000900ec0); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r52 = c.r64(11) & c.r64(3);
            c.w64(11, r52);
            c.logic(r52, 64);
            let Some(t53) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900ed4); };
            c.logic(t53 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900edf); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t54) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900ef4); };
            c.w32(11, t54);
            let r55 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r55);
            let Some(t56) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900efb); };
            c.w32(11, t56);
            let r57 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r57);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t58) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900f11); };
            c.w32(11, t58);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t59) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900f1e); };
            c.w32(12, t59);
            let r60 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r60);
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000900f25); };
            c.w32(12, t61);
            let r62 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r62);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t63) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900f3c); };
            c.w32(12, t63);
            let r64 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r64);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r65 = c.r64(12) | 0x3u64;
            c.w64(12, r65);
            c.logic(r65, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x7000900f50); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r66 = c.r64(9) & c.r64(3);
            c.w64(9, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000900f64); };
            c.logic(t67 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900f6f); }
            blk = 55; continue;
        }
        34 => {
            let Some(t68) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900f7b); };
            c.w32(11, t68);
            c.w32(12, c.r32(11));
            let r69 = c.r32(12) & 0x3u64;
            c.w32(12, r69);
            c.logic(r69, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r70 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r70);
            c.logic(r70, 64);
            let r71 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r71);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000900fa0); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000900fa5); }
            blk = 37; continue;
        }
        37 => {
            let Some(t72) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000900fac); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            let Some(t74) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000900fb3); };
            c.w32(11, t74);
            let r75 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r75);
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000900fba); };
            c.w32(12, t76);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x7000900fca);
        }
        39 => {
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000900fcb); };
            c.cmp(t77, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(11, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t78) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000900fd6); };
            c.w32(11, t78);
            c.w32(12, c.r32(11));
            let r79 = c.r32(12) & 0x3u64;
            c.w32(12, r79);
            c.logic(r79, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r80 = c.r64(11) | 0x2u64;
            c.w64(11, r80);
            c.logic(r80, 64);
            let r81 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r81);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000900ffb); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901000); }
            blk = 44; continue;
        }
        44 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000901007); };
            c.w64(11, t82);
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(12, c.r64(11));
            blk = 48; continue;
        }
        47 => {
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090101d); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            blk = 48; continue;
        }
        48 => {
            let Some(t85) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901025); };
            c.w32(15, t85);
            let r86 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r86);
            let Some(t87) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090102d); };
            c.w32(15, t87);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(15, c.r16(15));
            let r88 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r88);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t89) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090104b); };
            c.w32(12, t89);
            let r90 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r90);
            let Some(t91) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901053); };
            c.w32(15, t91);
            let r92 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r92);
            let Some(t93) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090105a); };
            c.w32(12, t93);
            let r94 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r94);
            let Some(t95) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901062); };
            c.w32(15, t95);
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901066); };
            c.cmp(t96, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r97 = c.r64(11) | 0x3u64;
            c.w64(11, r97);
            c.logic(r97, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000901071); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r98 = c.r64(9) & c.r64(3);
            c.w64(9, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901085); };
            c.logic(t99 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090108c); }
            blk = 55; continue;
        }
        54 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000901095); };
            c.w64(8, t100);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700090109c); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009010a1); }
            blk = 55; continue;
        }
        55 => {
            let Some(t101) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70009010a8); };
            if !c.push64(t101) { return RegfileExit::fault(0x70009010a8); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(2));
            c.w64(7, c.r64(1));
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009010b3); };
            c.w64(10, t102);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x70009010ba); }
            return RegfileExit::jump(0x70007f2274);
        }
        56 => {
            c.w32(11, c.r32(11));
            let r103 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r103);
            c.w64(9, 0xfffffffffffc0000u64);
            let r104 = c.r64(9) & c.r64(11);
            c.w64(9, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009010d4); };
            c.logic(t105 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009010df); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009010e0); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70009010ed, 0x70009010e8);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009010ed); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009010ef); }
            blk = 53; continue;
        }
        59 => {
            c.w32(12, c.r32(12));
            let r106 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r106);
            c.w64(9, 0xfffffffffffc0000u64);
            let r107 = c.r64(9) & c.r64(12);
            c.w64(9, r107);
            c.logic(r107, 64);
            let Some(t108) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901102); };
            c.logic(t108 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901111); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901112); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x700090111f, 0x700090111a);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700090111f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901121); }
            blk = 33; continue;
        }
        62 => {
            c.w32(9, c.r32(9));
            let r109 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r109);
            c.w64(11, 0xfffffffffffc0000u64);
            let r110 = c.r64(11) & c.r64(9);
            c.w64(11, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901137); };
            c.logic(t111 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901146); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901147); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000901154, 0x700090114f);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000901154); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901156); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000901160);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000901174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000901174); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000901178); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000901182); };
            c.w64(2, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000901186); }
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090118a); };
            c.w64(3, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090118e); };
            c.w64(1, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901194); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700090119a); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090119e); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009011a4); };
            c.w32(8, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009011aa); };
            c.w32(11, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70009011ae); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009011b2); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009011bb); };
            c.w64(3, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009011c2); };
            c.w64(11, t13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009011c6); };
            c.w32(15, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009011ca); };
            c.cmp(t15, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009011e7); };
            c.w32(0, t16);
            let r17 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r17);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009011f2); }
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009011f7); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            c.w32(6, c.r32(0));
            let r20 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700090120c); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000901219); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090122f); };
            c.w32(6, t23);
            c.w64(7, c.r64(0));
            let r24 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r24);
            c.logic(r24, 64);
            c.w32(2, c.r32(6));
            let r25 = c.r32(2) & 0x3u64;
            c.w32(2, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901262); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            let Some(t28) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901269); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090126f); };
            c.w32(6, t30);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700090127e);
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x43)) else { return RegfileExit::fault(0x700090127f); };
            c.cmp(t31, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            let r32 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r32);
            c.logic(r32, 32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090128d); };
            c.w64(0, t33);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090129c); };
            c.w32(2, t34);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(2));
            let r35 = c.r32(0) & 0x1u64;
            c.w32(0, r35);
            c.logic(r35, 32);
            let r36 = c.r32(6) & 0x3u64;
            c.w32(6, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r37 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r37);
            c.logic(r37, 64);
            let r38 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x70009012c1); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009012c6); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(2, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009012db); };
            c.w32(2, t39);
            let r40 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r40);
            blk = 20; continue;
        }
        20 => {
            let Some(t41) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009012e1); };
            c.w32(0, t41);
            let r42 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r42);
            let Some(t43) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009012e7); };
            c.w32(0, t43);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r44 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r44);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000901302); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            let Some(t47) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901308); };
            c.w32(0, t47);
            let r48 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r48);
            let Some(t49) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090130f); };
            c.w32(2, t49);
            let r50 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r50);
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901315); };
            c.w32(0, t51);
            let Some(t52) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901318); };
            c.cmp(t52, c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(2, c.r64(1));
            let r53 = c.r64(2) | 0x3u64;
            c.w64(2, r53);
            c.logic(r53, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x7000901328); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r54 = c.r64(15) & c.r64(3);
            c.w64(15, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090133b); };
            c.logic(t55 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901346); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t56) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090135b); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            let Some(t58) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901361); };
            c.w32(2, t58);
            let r59 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r59);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901374); };
            c.w32(2, t60);
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t61) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090137f); };
            c.w32(0, t61);
            let r62 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r62);
            let Some(t63) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901385); };
            c.w32(0, t63);
            let r64 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r64);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901397); };
            c.w32(0, t65);
            let r66 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r66);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r67 = c.r64(0) | 0x3u64;
            c.w64(0, r67);
            c.logic(r67, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70009013a9); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r68 = c.r64(2) & c.r64(3);
            c.w64(2, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009013bb); };
            c.logic(t69 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009013c5); }
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009013cc); };
            c.w64(2, t70);
            blk = 55; continue;
        }
        34 => {
            let Some(t71) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009013d5); };
            c.w32(0, t71);
            c.w32(6, c.r32(0));
            let r72 = c.r32(6) & 0x3u64;
            c.w32(6, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r73 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r73);
            c.logic(r73, 64);
            let r74 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r74);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009013f4); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009013f9); }
            blk = 37; continue;
        }
        37 => {
            let Some(t75) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901400); };
            c.w32(0, t75);
            let r76 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r76);
            let Some(t77) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901407); };
            c.w32(0, t77);
            let r78 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r78);
            let Some(t79) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090140d); };
            c.w32(6, t79);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700090141c);
        }
        39 => {
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700090141d); };
            c.cmp(t80, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t81) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901427); };
            c.w32(0, t81);
            c.w32(6, c.r32(0));
            let r82 = c.r32(6) & 0x3u64;
            c.w32(6, r82);
            c.logic(r82, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r83 = c.r64(0) | 0x2u64;
            c.w64(0, r83);
            c.logic(r83, 64);
            let r84 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r84);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000901446); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090144b); }
            blk = 44; continue;
        }
        44 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000901452); };
            c.w64(0, t85);
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(6, c.r64(0));
            blk = 48; continue;
        }
        47 => {
            let Some(t86) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000901466); };
            c.w32(6, t86);
            let r87 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r87);
            blk = 48; continue;
        }
        48 => {
            let Some(t88) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090146c); };
            c.w32(7, t88);
            let r89 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r89);
            let Some(t90) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901472); };
            c.w32(7, t90);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r16(7));
            let r91 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r91);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090148b); };
            c.w32(6, t92);
            let r93 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r93);
            let Some(t94) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901491); };
            c.w32(7, t94);
            let r95 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r95);
            let Some(t96) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901498); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090149e); };
            c.w32(7, t98);
            let Some(t99) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009014a1); };
            c.cmp(t99, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r100 = c.r64(0) | 0x3u64;
            c.w64(0, r100);
            c.logic(r100, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70009014aa); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r101 = c.r64(15) & c.r64(3);
            c.w64(15, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009014bc); };
            c.logic(t102 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009014c3); }
            blk = 55; continue;
        }
        54 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009014cc); };
            c.w64(2, t103);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70009014d3); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009014d8); }
            blk = 55; continue;
        }
        55 => {
            let Some(t104) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009014df); };
            c.w64(3, (t104 as u8 as i8 as i64 as u64));
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009014e5); };
            c.w64(2, t105);
            let Some(t106) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009014e9); };
            c.w64(3, t106);
            let Some(t107) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009014ed); };
            c.w64(5, t107);
            let r108 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r108);
            let Some(t109) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009014f5); };
            if !c.push64(t109) { return RegfileExit::fault(0x70009014f5); }
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x7000901504); }
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000901509); };
            c.w64(10, t110);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x7000901510); }
            return RegfileExit::jump(0x70007f2274);
        }
        56 => {
            c.w32(0, c.r32(0));
            let r111 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r111);
            c.w64(15, 0xfffffffffffc0000u64);
            let r112 = c.r64(15) & c.r64(0);
            c.w64(15, r112);
            c.logic(r112, 64);
            let Some(t113) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901529); };
            c.logic(t113 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901534); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901535); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000901542, 0x700090153d);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000901542); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901544); }
            blk = 53; continue;
        }
        59 => {
            c.w32(0, c.r32(0));
            let r114 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r114);
            c.w64(2, 0xfffffffffffc0000u64);
            let r115 = c.r64(2) & c.r64(0);
            c.w64(2, r115);
            c.logic(r115, 64);
            let Some(t116) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901559); };
            c.logic(t116 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901567); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901568); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x7000901575, 0x7000901570);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000901575); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901577); }
            blk = 33; continue;
        }
        62 => {
            c.w32(2, c.r32(2));
            let r117 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r117);
            c.w64(15, 0xfffffffffffc0000u64);
            let r118 = c.r64(15) & c.r64(2);
            c.w64(15, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090158c); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700090159b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090159c); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x70009015a9, 0x70009015a4);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009015a9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009015ab); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x70009015b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009015d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009015d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009015d8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009015e2); };
            c.w64(2, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009015e6); }
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009015ea); };
            c.w64(3, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009015ee); };
            c.w64(1, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009015f4); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70009015fa); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009015fe); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000901604); };
            c.w32(8, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090160a); };
            c.w32(11, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700090160e); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000901612); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090161b); };
            c.w64(3, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000901622); };
            c.w64(11, t13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901626); };
            c.w32(15, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090162a); };
            c.cmp(t15, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901647); };
            c.w32(0, t16);
            let r17 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r17);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000901652); }
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000901657); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            c.w32(6, c.r32(0));
            let r20 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700090166c); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000901679); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090168f); };
            c.w32(6, t23);
            c.w64(7, c.r64(0));
            let r24 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r24);
            c.logic(r24, 64);
            c.w32(2, c.r32(6));
            let r25 = c.r32(2) & 0x3u64;
            c.w32(2, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009016c2); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            let Some(t28) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009016c9); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009016cf); };
            c.w32(6, t30);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009016de);
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009016df); };
            c.cmp(t31, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            let r32 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r32);
            c.logic(r32, 32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009016ed); };
            c.w64(0, t33);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009016fc); };
            c.w32(2, t34);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(2));
            let r35 = c.r32(0) & 0x1u64;
            c.w32(0, r35);
            c.logic(r35, 32);
            let r36 = c.r32(6) & 0x3u64;
            c.w32(6, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r37 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r37);
            c.logic(r37, 64);
            let r38 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x7000901721); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901726); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(2, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090173b); };
            c.w32(2, t39);
            let r40 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r40);
            blk = 20; continue;
        }
        20 => {
            let Some(t41) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901741); };
            c.w32(0, t41);
            let r42 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r42);
            let Some(t43) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901747); };
            c.w32(0, t43);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r44 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r44);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000901762); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            let Some(t47) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901768); };
            c.w32(0, t47);
            let r48 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r48);
            let Some(t49) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090176f); };
            c.w32(2, t49);
            let r50 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r50);
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901775); };
            c.w32(0, t51);
            let Some(t52) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901778); };
            c.cmp(t52, c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(2, c.r64(1));
            let r53 = c.r64(2) | 0x3u64;
            c.w64(2, r53);
            c.logic(r53, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x7000901788); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r54 = c.r64(15) & c.r64(3);
            c.w64(15, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090179b); };
            c.logic(t55 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009017a6); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t56) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009017bb); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            let Some(t58) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009017c1); };
            c.w32(2, t58);
            let r59 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r59);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009017d4); };
            c.w32(2, t60);
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t61) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009017df); };
            c.w32(0, t61);
            let r62 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r62);
            let Some(t63) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009017e5); };
            c.w32(0, t63);
            let r64 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r64);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009017f7); };
            c.w32(0, t65);
            let r66 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r66);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r67 = c.r64(0) | 0x3u64;
            c.w64(0, r67);
            c.logic(r67, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000901809); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r68 = c.r64(2) & c.r64(3);
            c.w64(2, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090181b); };
            c.logic(t69 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901825); }
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090182c); };
            c.w64(2, t70);
            blk = 55; continue;
        }
        34 => {
            let Some(t71) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901835); };
            c.w32(0, t71);
            c.w32(6, c.r32(0));
            let r72 = c.r32(6) & 0x3u64;
            c.w32(6, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r73 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r73);
            c.logic(r73, 64);
            let r74 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r74);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000901854); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901859); }
            blk = 37; continue;
        }
        37 => {
            let Some(t75) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901860); };
            c.w32(0, t75);
            let r76 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r76);
            let Some(t77) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901867); };
            c.w32(0, t77);
            let r78 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r78);
            let Some(t79) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090186d); };
            c.w32(6, t79);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700090187c);
        }
        39 => {
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700090187d); };
            c.cmp(t80, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t81) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901887); };
            c.w32(0, t81);
            c.w32(6, c.r32(0));
            let r82 = c.r32(6) & 0x3u64;
            c.w32(6, r82);
            c.logic(r82, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r83 = c.r64(0) | 0x2u64;
            c.w64(0, r83);
            c.logic(r83, 64);
            let r84 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r84);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009018a6); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009018ab); }
            blk = 44; continue;
        }
        44 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009018b2); };
            c.w64(0, t85);
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(6, c.r64(0));
            blk = 48; continue;
        }
        47 => {
            let Some(t86) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009018c6); };
            c.w32(6, t86);
            let r87 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r87);
            blk = 48; continue;
        }
        48 => {
            let Some(t88) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009018cc); };
            c.w32(7, t88);
            let r89 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r89);
            let Some(t90) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009018d2); };
            c.w32(7, t90);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r16(7));
            let r91 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r91);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009018eb); };
            c.w32(6, t92);
            let r93 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r93);
            let Some(t94) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009018f1); };
            c.w32(7, t94);
            let r95 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r95);
            let Some(t96) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009018f8); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009018fe); };
            c.w32(7, t98);
            let Some(t99) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901901); };
            c.cmp(t99, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r100 = c.r64(0) | 0x3u64;
            c.w64(0, r100);
            c.logic(r100, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700090190a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r101 = c.r64(15) & c.r64(3);
            c.w64(15, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090191c); };
            c.logic(t102 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901923); }
            blk = 55; continue;
        }
        54 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700090192c); };
            c.w64(2, t103);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000901933); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901938); }
            blk = 55; continue;
        }
        55 => {
            let Some(t104) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x700090193f); };
            c.w64(3, (t104 as u8 as i8 as i64 as u64));
            let Some(t105) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000901945); };
            c.w64(2, (t105 as u8 as i8 as i64 as u64));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090194b); };
            c.w64(8, t106);
            let Some(t107) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090194f); };
            c.w64(3, t107);
            let Some(t108) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000901953); };
            c.w64(2, t108);
            let Some(t109) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000901957); };
            c.w64(5, t109);
            let r110 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r110);
            let Some(t111) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700090195f); };
            if !c.push64(t111) { return RegfileExit::fault(0x700090195f); }
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700090196e); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x7000901973); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000901978); };
            c.w64(10, t112);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x700090197f); }
            return RegfileExit::jump(0x70007f2274);
        }
        56 => {
            c.w32(0, c.r32(0));
            let r113 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r113);
            c.w64(15, 0xfffffffffffc0000u64);
            let r114 = c.r64(15) & c.r64(0);
            c.w64(15, r114);
            c.logic(r114, 64);
            let Some(t115) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901998); };
            c.logic(t115 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009019a3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009019a4); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70009019b1, 0x70009019ac);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009019b1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009019b3); }
            blk = 53; continue;
        }
        59 => {
            c.w32(0, c.r32(0));
            let r116 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r116);
            c.w64(2, 0xfffffffffffc0000u64);
            let r117 = c.r64(2) & c.r64(0);
            c.w64(2, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009019c8); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009019d6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009019d7); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x70009019e4, 0x70009019df);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009019e4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009019e6); }
            blk = 33; continue;
        }
        62 => {
            c.w32(2, c.r32(2));
            let r119 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r119);
            c.w64(15, 0xfffffffffffc0000u64);
            let r120 = c.r64(15) & c.r64(2);
            c.w64(15, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009019fb); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901a0a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901a0b); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x7000901a18, 0x7000901a13);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000901a18); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901a1a); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000901a24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000901a34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000901a34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000901a38); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000901a42); };
            c.w64(1, t1);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000901a46); }
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000901a4a); };
            c.w64(2, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000901a4e); };
            c.w64(8, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901a54); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000901a5a); };
            c.w64(8, t6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000901a5e); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000901a64); };
            c.w32(11, (t9 as u8 as i8 as i64 as u64));
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000901a6a); };
            c.w32(15, t10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000901a70); };
            c.w32(0, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901a76); };
            c.w32(3, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x7000901a79); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000901a7d); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000901a81); };
            c.cmp(t13, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000901a8a); };
            c.w64(2, t14);
            blk = 2; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000901a91); };
            c.w64(3, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901a95); };
            c.w32(6, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000901a98); };
            c.cmp(t17, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901ab5); };
            c.w32(7, t18);
            let r19 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r19);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x7000901abf); }
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000901ac3); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            c.w32(15, c.r32(7));
            let r22 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r22);
            c.logic(r22, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000901ad1); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000901ade); };
            c.cmp(t23, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000901aeb); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901b01); };
            c.w32(15, t25);
            c.w64(11, c.r64(7));
            let r26 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r26);
            c.logic(r26, 64);
            c.w32(0, c.r32(15));
            let r27 = c.r32(0) & 0x3u64;
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901b36); };
            c.w32(15, t28);
            let r29 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r29);
            let Some(t30) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901b3d); };
            c.w32(15, t30);
            let r31 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r31);
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000901b44); };
            c.w32(0, t32);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000901b54);
        }
        12 => {
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000901b55); };
            c.cmp(t33, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t34) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000901b5f); };
            c.w64(9, (t34 as u8 as i8 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t35) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x7000901b68); };
            c.w64(9, t35);
            let r36 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901b79); };
            c.w32(9, t37);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r38 = c.r32(11) & 0x1u64;
            c.w32(11, r38);
            c.logic(r38, 32);
            let r39 = c.r32(12) & 0x3u64;
            c.w32(12, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r40 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r40);
            c.logic(r40, 64);
            let r41 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r41);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x7000901ba5); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901baa); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            let Some(t42) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000901bc0); };
            c.w32(9, t42);
            let r43 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r43);
            blk = 20; continue;
        }
        20 => {
            let Some(t44) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901bc7); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901bce); };
            c.w32(11, t46);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r47 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r47);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000901bf0); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            let Some(t50) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901bf7); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901bfe); };
            c.w32(9, t52);
            let r53 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r53);
            let Some(t54) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901c05); };
            c.w32(11, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901c09); };
            c.cmp(t55, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r56 = c.r64(9) | 0x3u64;
            c.w64(9, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000901c1a); };
            c.w64(15, t57);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x7000901c1e); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r58 = c.r64(11) & c.r64(2);
            c.w64(11, r58);
            c.logic(r58, 64);
            let Some(t59) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901c32); };
            c.logic(t59 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901c3d); }
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000901c44); };
            c.w64(11, t60);
            blk = 56; continue;
        }
        26 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000901c4d); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901c60); };
            c.w32(9, t62);
            let r63 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r63);
            let Some(t64) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901c67); };
            c.w32(9, t64);
            let r65 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r65);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901c7d); };
            c.w32(9, t66);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901c86); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            let Some(t69) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901c8d); };
            c.w32(12, t69);
            let r70 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r70);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901ca4); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r73 = c.r64(11) | 0x3u64;
            c.w64(11, r73);
            c.logic(r73, 64);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000901cb8); };
            c.w64(15, t74);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x7000901cbc); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r75 = c.r64(9) & c.r64(2);
            c.w64(9, r75);
            c.logic(r75, 64);
            let Some(t76) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901cd0); };
            c.logic(t76 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901cdb); }
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000901ce2); };
            c.w64(11, t77);
            blk = 56; continue;
        }
        35 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000901ceb); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        36 => {
            let Some(t79) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901cf4); };
            c.w32(15, t79);
            c.w32(7, c.r32(15));
            let r80 = c.r32(7) & 0x3u64;
            c.w32(7, r80);
            c.logic(r80, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r81 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r81);
            c.logic(r81, 64);
            let r82 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r82);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x7000901d17); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901d1c); }
            blk = 39; continue;
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901d23); };
            c.w32(15, t83);
            let r84 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r84);
            let Some(t85) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901d2a); };
            c.w32(15, t85);
            let r86 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r86);
            let Some(t87) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000901d31); };
            c.w32(7, t87);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x7000901d41);
        }
        41 => {
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000901d42); };
            c.cmp(t88, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t89) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000901d4d); };
            c.w64(9, (t89 as u8 as i8 as i64 as u64));
            let Some(t90) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000901d53); };
            c.w32(12, t90);
            let Some(t91) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x7000901d58); };
            c.w64(9, t91);
            c.w32(15, c.r32(12));
            let r92 = c.r32(15) & 0x3u64;
            c.w32(15, r92);
            c.logic(r92, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r93 = c.r64(12) | 0x2u64;
            c.w64(12, r93);
            c.logic(r93, 64);
            let r94 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r94);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x7000901d7c); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901d81); }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(12, c.r64(9));
            blk = 49; continue;
        }
        48 => {
            let Some(t95) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000901d97); };
            c.w32(12, t95);
            let r96 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r96);
            blk = 49; continue;
        }
        49 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901d9f); };
            c.w32(15, t97);
            let r98 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r98);
            let Some(t99) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000901da7); };
            c.w32(15, t99);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r100 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r100);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t101) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000901dc5); };
            c.w32(12, t101);
            let r102 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r102);
            let Some(t103) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901dcd); };
            c.w32(15, t103);
            let r104 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r104);
            let Some(t105) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000901dd4); };
            c.w32(12, t105);
            let r106 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r106);
            let Some(t107) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901ddc); };
            c.w32(15, t107);
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000901de0); };
            c.cmp(t108, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r109 = c.r64(9) | 0x3u64;
            c.w64(9, r109);
            c.logic(r109, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x7000901deb); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r110 = c.r64(12) & c.r64(2);
            c.w64(12, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901dff); };
            c.logic(t111 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901e07); }
            blk = 56; continue;
        }
        55 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000901e10); };
            c.w64(9, t112);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x7000901e17); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000901e1c); }
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000901e23); };
            c.w64(11, t113);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000901e2e); };
            c.w64(1, t114);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t115) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000901e35); };
            c.w64(5, t115);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r116 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(0x70007fcc74);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r117 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r117);
            c.w64(12, 0xfffffffffffc0000u64);
            let r118 = c.r64(12) & c.r64(9);
            c.w64(12, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901e5b); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901e67); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901e68); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x7000901e75, 0x7000901e70);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000901e75); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901e77); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            c.w64(9, 0xfffffffffffc0000u64);
            let r121 = c.r64(9) & c.r64(11);
            c.w64(9, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901e8a); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901e99); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901e9a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000901ea7, 0x7000901ea2);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000901ea7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901ea9); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r123 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r123);
            c.w64(11, 0xfffffffffffc0000u64);
            let r124 = c.r64(11) & c.r64(9);
            c.w64(11, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000901ebf); };
            c.logic(t125 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000901ece); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000901ecf); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000901edc, 0x7000901ed7);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000901edc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000901ede); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x7000901ee8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000901ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000901ef4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000901ef8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000901f02); };
            c.w64(7, t1);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000901f06); }
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000901f0a); };
            c.w32(2, t2);
            let r3 = c.shl(c.r32(2), 0x5u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000901f13); };
            c.w32(0, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000901f19); };
            c.w64(8, t5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000901f1d); };
            c.w64(11, (t6 as u8 as i8 as i64 as u64));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2e30)) else { return RegfileExit::fault(0x7000901f23); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000901f2a); };
            c.w64(3, t8);
            c.w64(15, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000901f33); }
            c.w64(6, c.r64(8));
            return c.call(0x700086ef74, 0x7000901f3f, 0x7000901f3a);
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000901f3f); };
            c.w64(2, t9);
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000901f43); };
            c.w64(7, t10);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000901f4a); };
            c.w64(2, t11);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000901f55); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000901f5a); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000901f61); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000901f65); };
            c.w64(5, t15);
            c.w64(12, c.r64(2));
            let r16 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r16);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000901f74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000901f94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000901f94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000901f98); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000901fa2); };
            c.w64(7, t1);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000901fa6); }
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000901faa); };
            c.w32(2, t2);
            let r3 = c.shl(c.r32(2), 0x5u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000901fb3); };
            c.w32(8, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000901fb9); };
            c.w64(11, t5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000901fbd); };
            c.w64(3, (t6 as u8 as i8 as i64 as u64));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2e30)) else { return RegfileExit::fault(0x7000901fc3); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_add(c.r64(1)).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000901fca); };
            c.w64(2, t8);
            c.w64(15, c.r64(7).wrapping_add(c.r64(3).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000901fd3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000901fd7); }
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x700086f594, 0x7000901fe9, 0x7000901fe4);
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000901fe9); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000901fed); };
            c.w64(8, t10);
            let Some(t11) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000901ff1); };
            c.w64(11, t11);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t12) = c.ld8(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000901ffb); };
            c.w64(3, (t12 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x7000902001); }
            c.w64(0, c.r64(3).wrapping_sub(0x1));
            if !c.st64(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x7000902009); }
            c.w64(9, c.r64(8).wrapping_add(0x6));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000902011); };
            c.w32(2, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000902016); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700090201d); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000902021); };
            c.w64(5, t16);
            c.w64(12, c.r64(11));
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000902028); };
            c.w64(0, t17);
            let r18 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r18);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000902034);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000902054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(1, c.r64(5));
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090205b); }
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090205f); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902063); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090206a); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000902071); };
            c.w32(11, t5);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000902078); };
            c.w32(15, t6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x700090207e); };
            c.w32(0, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000902084); };
            c.w32(9, t8);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000902092);
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902093); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(2));
            return RegfileExit::jump(0x70007fcc14);
        }
        3 => {
            return RegfileExit::jump(0x70009020b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009020b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009020b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009020b8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009020c2); };
            c.w64(8, t1);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70009020c6); }
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009020ca); };
            c.w64(7, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009020ce); };
            c.w32(11, t3);
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009020d4); };
            c.w32(0, (t4 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70009020da); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70009020de); }
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_l() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_l() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0xfu64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), 0x10u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(11), 0xdu64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(11), 0xbu64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009021ae); };
            c.w64(7, t5);
            c.w32(11, 0x58u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009021b8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x70009021ba); };
            c.w64(3, t6);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70009021ce, 0x70009021c9);
        }
        25 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009021ce); };
            c.w64(0, t7);
            c.w64(7, c.r64(0));
            blk = 79; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t8) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009021e4); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009021e8); };
            c.w64(11, t9);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c774, 0x70009021f9, 0x70009021f4);
        }
        27 => {
            blk = 79; continue;
        }
        28 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t10) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902205); };
            c.w64(0, t10);
            let Some(t11) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000902209); };
            c.w64(11, t11);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082d2d4, 0x700090221a, 0x7000902215);
        }
        29 => {
            blk = 79; continue;
        }
        30 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t12) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902226); };
            c.w64(0, t12);
            let Some(t13) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090222a); };
            c.w64(11, t13);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c2b4, 0x700090223b, 0x7000902236);
        }
        31 => {
            blk = 79; continue;
        }
        32 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t14) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000902247); };
            c.w64(0, t14);
            let Some(t15) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090224b); };
            c.w64(3, t15);
            let Some(t16) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090224f); };
            c.w64(11, t16);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c614, 0x7000902260, 0x700090225b);
        }
        33 => {
            blk = 79; continue;
        }
        34 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t17) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090226c); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902270); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000902274); };
            c.w64(11, t19);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c6b4, 0x7000902285, 0x7000902280);
        }
        35 => {
            blk = 79; continue;
        }
        36 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t20) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902291); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000902295); };
            c.w64(11, t21);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700086bf14, 0x70009022a6, 0x70009022a1);
        }
        37 => {
            blk = 79; continue;
        }
        38 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t22) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009022b2); };
            c.w64(0, t22);
            let Some(t23) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009022b6); };
            c.w64(11, t23);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700086ca54, 0x70009022c7, 0x70009022c2);
        }
        39 => {
            blk = 79; continue;
        }
        40 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t24) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009022d3); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009022d7); };
            c.w64(11, t25);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000869974, 0x70009022e8, 0x70009022e3);
        }
        41 => {
            blk = 79; continue;
        }
        42 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t26) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009022f4); };
            c.w64(0, t26);
            let Some(t27) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009022f8); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009022fc); };
            c.w64(11, t28);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000869834, 0x700090230d, 0x7000902308);
        }
        43 => {
            blk = 79; continue;
        }
        44 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t29) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000902319); };
            c.w64(0, t29);
            let Some(t30) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090231d); };
            c.w64(3, t30);
            let Some(t31) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000902321); };
            c.w64(11, t31);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x70008699f4, 0x7000902332, 0x700090232d);
        }
        45 => {
            blk = 79; continue;
        }
        46 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t32) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090233e); };
            c.w64(0, t32);
            let Some(t33) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000902342); };
            c.w64(11, t33);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082fbd4, 0x7000902353, 0x700090234e);
        }
        47 => {
            blk = 79; continue;
        }
        48 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t34) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090235b); };
            c.w64(7, t34);
            let Some(t35) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090235f); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            c.w64(0, c.r64(7));
            blk = 79; continue;
        }
        49 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t37) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902370); };
            c.w64(7, t37);
            c.w64(11, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700090237b); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090237f); };
            c.w64(0, t38);
            c.w64(7, c.r64(0));
            blk = 79; continue;
        }
        50 => {
            let Some(t39) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090238e); };
            c.w32(11, t39);
            let r40 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000902395); };
            c.w32(11, t41);
            let r42 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r42);
            let Some(t43) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090239c); };
            c.w32(0, t43);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x7au64, 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x70009023ac);
        }
        52 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x1ef)) else { return RegfileExit::fault(0x70009023ad); };
            c.w32(11, t44);
            c.w64(0, c.r64(7));
            blk = 54; continue;
        }
        53 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009023b9); };
            c.w32(0, t45);
            let r46 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r46);
            blk = 54; continue;
        }
        54 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009023bf); };
            c.cmp(t47, c.r32(11), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009023c5); };
            c.w32(3, t48);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_a() { blk = 53; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x70009023d4);
        }
        57 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009023d5); };
            c.w32(11, t49);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x70009023e5);
        }
        59 => {
            let Some(t50) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009023e6); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            let Some(t52) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70009023ed); };
            c.w32(11, t52);
            let r53 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r53);
            let Some(t54) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009023f4); };
            c.cmp(t54, c.r32(11), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, c.r64(11));
            blk = 79; continue;
        }
        61 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x22a0)) else { return RegfileExit::fault(0x7000902405); };
            c.w64(3, t55);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000902416, 0x7000902411);
        }
        62 => {
            blk = 79; continue;
        }
        63 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t56) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902422); };
            c.w64(0, t56);
            let Some(t57) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000902426); };
            c.w64(11, t57);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000821334, 0x7000902437, 0x7000902432);
        }
        64 => {
            blk = 79; continue;
        }
        65 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t58) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902443); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000902447); };
            c.w64(11, t59);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082faf4, 0x7000902458, 0x7000902453);
        }
        66 => {
            blk = 79; continue;
        }
        67 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t60) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902460); };
            c.w64(11, t60);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000902464); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t61) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902472); };
            c.w32(0, t61);
            let r62 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r62);
            let Some(t63) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902479); };
            c.cmp(t63, 0xbau64, 16);
            if c.cc_b() { blk = 83; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000902485); };
            c.w64(3, t64);
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000870914, 0x7000902497, 0x7000902492);
        }
        70 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000902497); };
            c.w64(6, t65);
            let Some(t66) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090249b); };
            c.w32(7, t66);
            let r67 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r67);
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009024a1); };
            c.w32(7, t68);
            let r69 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009024a7); };
            c.w32(8, t70);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2du64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::fault(0x70009024b7);
        }
        72 => {
            let Some(t71) = c.ld32(c.r64(7).wrapping_add(0xbb)) else { return RegfileExit::fault(0x70009024b8); };
            c.w32(7, t71);
            let r72 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r72);
            let Some(t73) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009024c1); };
            c.w32(8, t73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009024c6); };
            c.w64(11, t74);
            let Some(t75) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009024cd); };
            c.w64(3, t75);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009024dc); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70009024e3); };
            c.cmp(t77, c.r64(8), 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x70009024ec); }
            c.w64(8, c.r64(3).wrapping_add(0x1));
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70009024f3); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009024f7); };
            c.w64(7, t78);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70009024fe); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000902502); }
            c.w64(7, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000902510); };
            c.w64(11, t79);
            blk = 76; continue;
        }
        76 => {
            let r80 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r80);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x700090251b); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 76; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000902525); };
            c.w64(7, t81);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000902529); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700090252d); }
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(8));
            blk = 79; continue;
        }
        79 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000902534); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000902538); };
            c.w64(8, t83);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t84) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090253f); };
            c.w64(7, t84);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_add(0x4));
            let Some(t85) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700090254a); };
            c.w32(8, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700090254f); };
            c.w64(15, t86);
            let Some(t87) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902556); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090255a); };
            c.w64(5, t88);
            c.w64(12, c.r64(7));
            let r89 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r89);
            return RegfileExit::jump(c.r64(1));
        }
        80 => {
            blk = 83; continue;
        }
        81 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000902569); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x700090256d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000902571); }
            return c.call(0x70007febf4, 0x700090257a, 0x7000902575);
        }
        82 => {
            c.w64(8, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090257d); };
            c.w64(0, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000902581); };
            c.w64(2, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000902585); };
            c.w64(7, t92);
            blk = 74; continue;
        }
        83 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x700090258e); };
            c.w64(3, t93);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700090259f, 0x700090259a);
        }
        84 => {
            c.w64(8, c.r64(0));
            blk = 78; continue;
        }
        85 => {
            return RegfileExit::jump(0x70009025a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009025b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009025b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009025b8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009025c2); };
            c.w64(11, t1);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009025c6); }
            let Some(t2) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009025ca); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009025ce); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009025d4); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009025db); };
            c.w64(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009025df); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009025e6); };
            c.w32(2, (t9 as u8 as i8 as i64 as u64));
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009025ec); };
            c.w32(1, t10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70009025f2); };
            c.w32(6, t11);
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009025f8); };
            c.w32(7, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009025fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x7000902600); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000902604); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000902608); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000902611); };
            c.w64(8, t14);
            blk = 2; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902618); };
            c.w64(7, t15);
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090261c); };
            c.w32(9, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000902620); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, c.r64(6).wrapping_mul(4));
            let Some(t18) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000902635); };
            c.w32(12, t18);
            let r19 = c.add(c.r32(12), 0x8u64, 32);
            c.w32(12, r19);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x7000902641); }
            c.w64(6, c.r64(6).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090264e); };
            c.w32(12, t20);
            let r21 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r21);
            c.w32(15, c.r32(12));
            let r22 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000902666); };
            c.cmp(t23, c.r32(12), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000902673); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(15, c.r32(12));
            let r25 = c.r32(15) & 0x3u64;
            c.w32(15, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090268f); };
            c.w32(15, t26);
            let r27 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r27);
            let Some(t28) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902697); };
            c.cmp(t28, 0x5au64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009026a7); };
            c.w32(6, t29);
            let r30 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r30);
            let Some(t31) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009026ad); };
            c.w32(6, t31);
            let r32 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r32);
            let Some(t33) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009026b3); };
            c.w32(15, t33);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009026c3);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(6).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70009026c4); };
            c.cmp(t34, c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 28; continue;
        }
        14 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t35) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009026e3); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009026ea); };
            c.w32(12, t37);
            let r38 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r38);
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009026f2); };
            c.w32(15, t39);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x7000902703);
        }
        18 => {
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0xb3)) else { return RegfileExit::fault(0x7000902704); };
            c.cmp(t40, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(12, c.r64(0));
            blk = 22; continue;
        }
        21 => {
            let Some(t41) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090271f); };
            c.w32(12, t41);
            let r42 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r42);
            blk = 22; continue;
        }
        22 => {
            let Some(t43) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902727); };
            c.w32(15, t43);
            let r44 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r44);
            let Some(t45) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090272f); };
            c.w32(15, t45);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(15, c.r16(15));
            let r46 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r46);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t47) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090274d); };
            c.w32(12, t47);
            let r48 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r48);
            let Some(t49) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902755); };
            c.w32(15, t49);
            let r50 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r50);
            let Some(t51) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090275c); };
            c.w32(12, t51);
            let r52 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r52);
            let Some(t53) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000902764); };
            c.w32(15, t53);
            let Some(t54) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000902768); };
            c.cmp(t54, c.r32(15), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(0));
            let r55 = c.r64(9) | 0x3u64;
            c.w64(9, r55);
            c.logic(r55, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(9)) { return RegfileExit::fault(0x7000902776); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r56 = c.r64(12) & c.r64(8);
            c.w64(12, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090278a); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000902796); }
            blk = 38; continue;
        }
        28 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009027a3); };
            c.w64(6, t58);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x70009027aa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009027af); }
            blk = 38; continue;
        }
        29 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70009027bc); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70009027c3); };
            c.w64(12, t60);
            c.w64(15, c.r64(12).wrapping_add(0x1c));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70009027cb); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70009027d2); };
            c.cmp(t62, c.r64(15), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70009027db); }
            c.w64(0, c.r64(12).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70009027e3); };
            c.w64(9, t63);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70009027ea); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70009027f1); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70009027f5); }
            if !c.st32(c.r64(0).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70009027f9); }
            if !c.st32(c.r64(0).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x7000902800); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x7000902807); };
            c.w64(9, t64);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700090280e); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x7000902812); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000902819); };
            c.w64(12, t66);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x700090281c); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r67 = c.r64(15) & c.r64(0);
            c.w64(15, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000902830); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 47; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x700090283b); }
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x700090283e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r69 = c.r64(3) & c.r64(8);
            c.w64(3, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000902850); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090285a); }
            c.w64(12, c.r64(0));
            blk = 36; continue;
        }
        36 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(6, c.r64(7));
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000902875); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000902879); };
            c.w64(2, t72);
            c.w64(3, c.r64(12));
            return c.call(0x70007fcd34, 0x7000902885, 0x7000902880);
        }
        37 => {
            blk = 39; continue;
        }
        38 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(6, c.r64(7));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(7, c.r64(3));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090289d); };
            c.w64(3, t73);
            return c.call(0x70007fccd4, 0x70009028a9, 0x70009028a4);
        }
        39 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009028a9); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009028ad); };
            c.w64(11, t75);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t76) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009028b4); };
            c.w64(3, t76);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t77) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70009028bf); };
            c.w32(11, t77);
            c.w64(2, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70009028d5); };
            c.w64(15, t78);
            let Some(t79) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009028dc); };
            c.w64(1, t79);
            let Some(t80) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009028e0); };
            c.w64(5, t80);
            c.w64(12, c.r64(3));
            let r81 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r81);
            return RegfileExit::jump(c.r64(1));
        }
        41 => {
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70009028ed); }
            let r82 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r82);
            let Some(t83) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70009028f9); };
            c.w32(8, t83);
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70009028fe); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902905); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000902909); };
            c.w64(5, t86);
            c.w64(12, c.r64(3));
            let r87 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r87);
            return RegfileExit::jump(c.r64(1));
        }
        42 => {
            c.w32(0, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700090291b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700090291f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000902923); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000902927); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700090292b); }
            c.w64(2, c.r64(0));
            return c.call(0x70007fec74, 0x7000902937, 0x7000902932);
        }
        43 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000902937); };
            c.w64(6, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090293b); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090293f); };
            c.w64(1, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000902943); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000902947); };
            c.w64(2, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090294b); };
            c.w64(8, t93);
            blk = 31; continue;
        }
        44 => {
            c.w32(0, c.r32(0));
            let r94 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r94);
            c.w64(3, 0xfffffffffffc0000u64);
            let r95 = c.r64(3) & c.r64(0);
            c.w64(3, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000902963); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000902971); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000902972); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700090297f, 0x700090297a);
        }
        46 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700090297f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000902981); }
            blk = 35; continue;
        }
        47 => {
            c.w32(12, c.r32(12));
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            c.w64(15, 0xfffffffffffc0000u64);
            let r98 = c.r64(15) & c.r64(12);
            c.w64(15, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000902997); };
            c.logic(t99 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(3, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009029a6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009029a7); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70009029b4, 0x70009029af);
        }
        49 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009029b4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009029b6); }
            blk = 33; continue;
        }
        50 => {
            c.w32(9, c.r32(9));
            let r100 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r100);
            c.w64(12, 0xfffffffffffc0000u64);
            let r101 = c.r64(12) & c.r64(9);
            c.w64(12, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009029cc); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009029dc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009029dd); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70009029ea, 0x70009029e5);
        }
        52 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009029ea); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009029ec); }
            blk = 27; continue;
        }
        53 => {
            return RegfileExit::jump(0x70009029f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000902a14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000902a14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000902a18); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000902a22); };
            c.w64(11, t1);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000902a26); }
            let Some(t2) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000902a2a); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000902a2e); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000902a34); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902a3b); };
            c.w64(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000902a3f); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000902a46); };
            c.w32(2, (t9 as u8 as i8 as i64 as u64));
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000902a4c); };
            c.w32(1, t10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000902a52); };
            c.w32(6, t11);
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902a58); };
            c.w32(7, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x7000902a5c); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000902a60); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(8));
            blk = 3; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000902a6e); };
            c.w64(7, t14);
            blk = 3; continue;
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902a75); };
            c.w64(9, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000902a79); };
            c.w64(8, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000902a80); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(6).wrapping_mul(4));
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000902a91); };
            c.w32(15, t18);
            let r19 = c.add(c.r32(15), 0x8u64, 32);
            c.w32(15, r19);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x7000902a9d); }
            c.w64(6, c.r64(6).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000902aaa); };
            c.w32(15, t20);
            c.w32(8, c.r32(15));
            let r21 = c.r32(8) & 0xfffffffffffffffdu64;
            c.w32(8, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(7, c.r64(3));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000902ad2); };
            c.w64(3, t22);
            c.w64(6, c.r64(9));
            return c.call(0x70007fcd94, 0x7000902ae1, 0x7000902adc);
        }
        6 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000902ae1); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000902ae5); };
            c.w64(11, t24);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t25) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000902aec); };
            c.w64(3, t25);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t26) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000902af7); };
            c.w32(11, t26);
            c.w64(2, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000902b0d); };
            c.w64(15, t27);
            let Some(t28) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902b14); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000902b18); };
            c.w64(5, t29);
            c.w64(12, c.r64(3));
            let r30 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r30);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x7000902b25); }
            let r31 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r31);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000902b31); };
            c.w32(8, t32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000902b36); };
            c.w64(15, t33);
            let Some(t34) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902b3d); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000902b41); };
            c.w64(5, t35);
            c.w64(12, c.r64(3));
            let r36 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r36);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000902b4e); };
            c.cmp(t37, c.r32(15), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(8, c.r32(15));
            let r38 = c.r32(8) & 0x3u64;
            c.w32(8, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000902b68); };
            c.cmp(t39, c.r32(15), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 23; continue;
        }
        13 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(0));
            blk = 16; continue;
        }
        16 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902b8b); };
            c.w32(15, t40);
            let r41 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r41);
            let Some(t42) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902b92); };
            c.w32(15, t42);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000902b9f); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            blk = 16; continue;
        }
        18 => {
            let r45 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r45);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000902bb5); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902bbc); };
            c.w32(8, t48);
            let r49 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r49);
            let Some(t50) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902bc3); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000902bca); };
            c.w32(8, t52);
            let Some(t53) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000902bce); };
            c.cmp(t53, c.r32(8), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, c.r64(0));
            let r54 = c.r64(8) | 0x3u64;
            c.w64(8, r54);
            c.logic(r54, 64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(6)), c.r32(8)) { return RegfileExit::fault(0x7000902bdb); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r55 = c.r64(12) & c.r64(7);
            c.w64(12, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000902bef); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000902bf7); }
            blk = 5; continue;
        }
        23 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000902c03); };
            c.w64(8, t57);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000902c0a); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000902c0f); }
            blk = 5; continue;
        }
        24 => {
            c.w32(8, c.r32(8));
            let r58 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r58);
            c.w64(12, 0xfffffffffffc0000u64);
            let r59 = c.r64(12) & c.r64(8);
            c.w64(12, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000902c2b); };
            c.logic(t60 & 0x2u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(15, c.r64(7).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000902c37); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000902c41, 0x7000902c3c);
        }
        26 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000902c41); }
            blk = 22; continue;
        }
        27 => {
            return RegfileExit::jump(0x7000902c48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000902c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000902c54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000902c58); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000902c5e); };
            c.w64(7, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000902c62); };
            c.w64(8, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000902c68); };
            c.w64(11, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000902c6c); };
            c.w64(8, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000902c70); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000902c74); }
            c.w32(2, 0x60u64);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x20u64);
            c.w64(6, c.r64(3));
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
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
            let Some(t5) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902cba); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902cc1); };
            c.w32(15, t7);
            let r8 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902cc8); };
            c.w32(11, t9);
            let Some(t10) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902cce); };
            c.w32(6, t10);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 98; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(11), 0x42u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 90; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(6), 0xbau64, 16);
            if c.cc_ae() { blk = 58; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000902d0d); };
            c.logic(t11 & 0x10u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 88; continue;
        }
        12 => {
            c.w32(11, c.r32(3));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000902d20); };
            c.cmp(t12, c.r32(12), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 13; continue;
        }
        13 => {
            let r13 = c.r32(11) | 0x8u64;
            c.w32(11, r13);
            c.logic(r13, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000902d3d); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            blk = 1; continue;
        }
        15 => {
            c.w32(3, c.r32(3));
            let r16 = c.r32(3) | 0x6u64;
            c.w32(3, r16);
            c.logic(r16, 32);
            c.cmp(c.r16(11), c.r16(6), 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(6), 0x40u64, 16);
            if c.cc_b() { blk = 97; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r16(6), 0xbau64, 16);
            if c.cc_b() { blk = 69; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 26; continue;
        }
        21 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 26; continue;
        }
        22 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000902d83); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 23; continue;
        }
        23 => {
            let r18 = c.r32(3) | 0x8u64;
            c.w32(3, r18);
            c.logic(r18, 32);
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000902d93); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        24 => {
            c.cmp(c.r16(6), 0xbau64, 16);
            if c.cc_b() { blk = 71; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(3, 0x3feu64);
            blk = 26; continue;
        }
        26 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 1; continue;
        }
        27 => {
            c.w32(11, c.r32(3));
            let r21 = c.r32(11) | 0x2u64;
            c.w32(11, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902dd0); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000902dd6); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t25) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902de3); };
            c.w32(6, t25);
            c.cmp(c.r16(6), 0x40u64, 16);
            if c.cc_b() { blk = 102; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 122; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r16(6), 0xbau64, 16);
            if c.cc_ae() { blk = 100; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 74; continue;
        }
        34 => {
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000902e11); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 35; continue;
        }
        35 => {
            let r27 = c.r32(11) | 0x8u64;
            c.w32(11, r27);
            c.logic(r27, 32);
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000902e22); };
            c.w32(0, t28);
            let r29 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r29);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        36 => {
            c.w32(11, c.r32(3));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000902e3d); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000902e44); };
            c.cmp(t32, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t33) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000902e51); };
            c.w32(8, t33);
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
            let r34 = c.r32(11) | 0x80u64;
            c.w32(11, r34);
            c.logic(r34, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        43 => {
            let r35 = c.r32(11) | 0x100u64;
            c.w32(11, r35);
            c.logic(r35, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        44 => {
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000902e91); };
            c.cmp(t36, c.r32(0), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let r37 = c.r32(11) | 0x210u64;
            c.w32(11, r37);
            c.logic(r37, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        46 => {
            let r38 = c.r32(11) | 0x8u64;
            c.w32(11, r38);
            c.logic(r38, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        47 => {
            let r39 = c.r32(11) | 0x200u64;
            c.w32(11, r39);
            c.logic(r39, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        48 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(1, c.r64(2));
            blk = 50; continue;
        }
        50 => {
            c.w32(8, c.r32(1));
            let r40 = c.r32(8) | c.r32(11);
            c.w32(8, r40);
            c.logic(r40, 32);
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        51 => {
            let r41 = c.r32(11) | 0x6u64;
            c.w32(11, r41);
            c.logic(r41, 32);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000902edf); }
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
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
            let r42 = c.r32(11) | 0x2u64;
            c.w32(11, r42);
            c.logic(r42, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        55 => {
            let Some(t43) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000902f09); };
            c.logic(t43 & 0x10u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(3, 0x210u64);
            blk = 81; continue;
        }
        57 => {
            c.w32(3, 0x210u64);
            blk = 79; continue;
        }
        58 => {
            c.w32(8, c.r32(3));
            let r44 = c.r32(8) | 0x200u64;
            c.w32(8, r44);
            c.logic(r44, 32);
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        59 => {
            let r45 = c.r32(11) | 0x10u64;
            c.w32(11, r45);
            c.logic(r45, 32);
            let Some(t46) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000902f3b); };
            c.logic(t46 & 0x10u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r16(6), 0x42u64, 16);
            if c.cc_e() { blk = 66; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let r47 = c.r32(11) | 0x210u64;
            c.w32(11, r47);
            c.logic(r47, 32);
            c.cmp(c.r16(6), 0xbau64, 16);
            if c.cc_ae() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let r48 = c.r32(11) | 0x3feu64;
            c.w32(11, r48);
            c.logic(r48, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        64 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        65 => {
            let r49 = c.r32(11) | 0x18u64;
            c.w32(11, r49);
            c.logic(r49, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        66 => {
            let r50 = c.r32(11) | 0x6u64;
            c.w32(11, r50);
            c.logic(r50, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        67 => {
            let r51 = c.r32(11) | 0x210u64;
            c.w32(11, r51);
            c.logic(r51, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        68 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000902f9d); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000902fa3); }
            blk = 77; continue;
        }
        69 => {
            let r52 = c.r32(3) | 0x3feu64;
            c.w32(3, r52);
            c.logic(r52, 32);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        70 => {
            let r53 = c.r32(3) | 0x18u64;
            c.w32(3, r53);
            c.logic(r53, 32);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        71 => {
            c.cmp(c.r16(6), 0x40u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w32(3, 0x3feu64);
            blk = 81; continue;
        }
        73 => {
            c.w32(8, c.r32(3));
            let r54 = c.r32(8) | 0x80u64;
            c.w32(8, r54);
            c.logic(r54, 32);
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        74 => {
            let r55 = c.r32(11) | 0x3feu64;
            c.w32(11, r55);
            c.logic(r55, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        75 => {
            let r56 = c.r32(11) | 0x18u64;
            c.w32(11, r56);
            c.logic(r56, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        76 => {
            let r57 = c.r32(11) | 0x6u64;
            c.w32(11, r57);
            c.logic(r57, 32);
            let r58 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r58);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000902ff8); }
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
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
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000903011); };
            c.w64(0, t59);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903018); }
            blk = 82; continue;
        }
        80 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        81 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000903021); };
            c.w64(0, t60);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903028); }
            blk = 82; continue;
        }
        82 => {
            let Some(t61) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090302c); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903030); };
            c.w64(7, t62);
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000903034); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090303b); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            c.w64(11, c.r64(9));
            let Some(t67) = c.ld8(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000903045); };
            c.w32(3, t67);
            let Some(t68) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090304b); };
            c.w32(2, t68);
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090304f); };
            c.cmp(t69, c.r32(2), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000903058); };
            c.w64(8, t70);
            blk = 84; continue;
        }
        84 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090305f); };
            c.cmp(t71, c.r32(8), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t72) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000903068); };
            c.w32(2, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090306d); };
            c.w64(1, t73);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r74 = c.r32(1) | c.r32(6);
            c.w32(1, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700090307e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000903083); }
            blk = 87; continue;
        }
        87 => {
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700090308f); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000903094); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000903098); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090309c); };
            c.w64(5, t78);
            c.w64(12, c.r64(7));
            let r79 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r79);
            return RegfileExit::jump(c.r64(1));
        }
        88 => {
            c.w32(11, c.r32(3));
            let r80 = c.r32(11) | 0x3feu64;
            c.w32(11, r80);
            c.logic(r80, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009030ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009030be); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009030c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009030c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009030ca); }
            c.w64(0, c.r64(8));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009030d1); };
            c.w64(6, t81);
            return c.call(0x70008a2374, 0x70009030da, 0x70009030d5);
        }
        89 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009030da); };
            c.w64(3, t82);
            c.w64(8, c.r64(0));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009030e1); };
            c.w64(0, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009030e5); };
            c.w64(9, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009030e9); };
            c.w64(7, t85);
            c.w32(1, 0x20u64);
            c.w32(2, 0x60u64);
            blk = 1; continue;
        }
        90 => {
            c.w32(11, c.r32(3));
            let r86 = c.r32(11) | 0x100u64;
            c.w32(11, r86);
            c.logic(r86, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000903106); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090310a); }
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090310e); };
            c.cmp(t87, c.r32(15), 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 110; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r16(6), 0x40u64, 16);
            if c.cc_b() { blk = 108; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t88) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700090312f); };
            c.cmp(t88, c.r32(15), 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r16(6), 0xbau64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 95; continue;
        }
        95 => {
            blk = 81; continue;
        }
        96 => {
            let r89 = c.r32(11) | 0x8u64;
            c.w32(11, r89);
            c.logic(r89, 32);
            let Some(t90) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090314f); };
            c.w32(0, t90);
            let r91 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r91);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        97 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 103; continue;
        }
        98 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000903162); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000903166); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700090316a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700090316e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000903172); }
            c.cmp(c.r16(6), 0x40u64, 16);
            if c.cc_b() { blk = 114; continue; }
            blk = 99; continue;
        }
        99 => {
            blk = 26; continue;
        }
        100 => {
            let r92 = c.r32(11) | 0x200u64;
            c.w32(11, r92);
            c.logic(r92, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903193); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000903197); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090319b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700090319f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009031a3); }
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009031a7); };
            c.w64(6, t93);
            return c.call(0x70008a2374, 0x70009031b0, 0x70009031ab);
        }
        101 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009031b0); };
            c.w64(3, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009031b4); };
            c.w64(8, t95);
            c.w32(2, 0x60u64);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009031bd); };
            c.w64(9, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009031c1); };
            c.w64(7, t97);
            c.w32(1, 0x20u64);
            blk = 1; continue;
        }
        102 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 103; continue;
        }
        103 => {
            let Some(t98) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009031d2); };
            c.w32(3, t98);
            let r99 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r99);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009031d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009031dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70009031e0); }
            let Some(t100) = c.ld8(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009031e4); };
            c.logic(t100 & 0x20u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(3, c.r64(2));
            blk = 106; continue;
        }
        105 => {
            c.w64(3, c.r64(1));
            blk = 106; continue;
        }
        106 => {
            c.w32(3, c.r32(3));
            c.w32(11, c.r32(11));
            let r101 = c.r32(3) | c.r32(11);
            c.w32(3, r101);
            c.logic(r101, 32);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w32(6, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000903204); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903208); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090320c); };
            c.w64(6, t102);
            return c.call(0x70008a0fd4, 0x7000903215, 0x7000903210);
        }
        107 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000903215); };
            c.w64(3, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000903219); };
            c.w64(8, t104);
            c.w32(2, 0x60u64);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000903222); };
            c.w64(9, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903226); };
            c.w64(7, t106);
            c.w32(1, 0x20u64);
            blk = 1; continue;
        }
        108 => {
            let r107 = c.r32(11) | 0x60u64;
            c.w32(11, r107);
            c.logic(r107, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000903242); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903246); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903248); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903249); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1d50)) else { return RegfileExit::fault(0x7000903254); };
            c.w64(3, t108);
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000903266, 0x7000903261);
        }
        109 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903266); };
            c.w64(7, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090326a); };
            c.w64(9, t110);
            blk = 82; continue;
        }
        110 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090327d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903281); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903283); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903284); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700090328a); };
            c.w64(3, t111);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700090329e, 0x7000903299);
        }
        111 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090329e); };
            c.w64(7, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009032a2); };
            c.w64(9, t113);
            blk = 82; continue;
        }
        112 => {
            let r114 = c.r32(11) | 0x6u64;
            c.w32(11, r114);
            c.logic(r114, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009032b9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009032bd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009032bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009032c0); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x70009032c6); };
            c.w64(3, t115);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70009032da, 0x70009032d5);
        }
        113 => {
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009032da); };
            c.w64(7, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009032de); };
            c.w64(9, t117);
            blk = 82; continue;
        }
        114 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009032eb); }
            c.w64(2, c.r64(8));
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009032f2); };
            c.w64(6, t118);
            return c.call(0x70007fb534, 0x70009032fb, 0x70009032f6);
        }
        115 => {
            let Some(t119) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009032fb); };
            c.logic(t119 & 0x20u64, 8);
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
            let Some(t120) = c.ld8(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090330d); };
            c.logic(t120 & 0x20u64, 8);
            if c.cc_e() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w32(7, 0x60u64);
            blk = 121; continue;
        }
        120 => {
            c.w32(7, 0x20u64);
            blk = 121; continue;
        }
        121 => {
            c.w32(8, c.r32(1));
            c.w32(7, c.r32(7));
            let Some(t121) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000903324); };
            c.w32(11, t121);
            let r122 = c.r32(8) | c.r32(7);
            c.w32(8, r122);
            c.logic(r122, 32);
            let r123 = c.r32(8) | c.r32(11);
            c.w32(8, r123);
            c.logic(r123, 32);
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903331); }
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903335); };
            c.w64(7, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000903339); };
            c.w64(9, t125);
            blk = 82; continue;
        }
        122 => {
            let r126 = c.r32(11) | 0x100u64;
            c.w32(11, r126);
            c.logic(r126, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000903353); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903357); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903358); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090335a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090335e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903362); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x7000903368); };
            c.w64(3, t127);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700090337c, 0x7000903377);
        }
        123 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090337c); };
            c.w64(7, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000903380); };
            c.w64(9, t129);
            blk = 82; continue;
        }
        124 => {
            return RegfileExit::jump(0x700090338c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000903394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000903397); };
            c.w64(8, (t0 as u8 as i8 as i64 as u64));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090339d); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009033c8); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009033cf); };
            c.cmp(t4, c.r32(11), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009033e5); };
            c.w32(3, t5);
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_b() { blk = 73; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(3), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009033fe); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903405); };
            c.w32(0, t8);
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_ae() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000903411); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(3), 0x43u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r16(0), 0x40u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(8, 0x80u64);
            blk = 61; continue;
        }
        12 => {
            c.cmp(c.r16(0), 0x43u64, 16);
            if c.cc_b() { blk = 35; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090344d); };
            c.cmp(t10, c.r32(8), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(8, 0x210u64);
            blk = 61; continue;
        }
        15 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000903465); };
            c.cmp(t11, c.r32(8), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(0), 0xbau64, 16);
            if c.cc_ae() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(0), 0x43u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(8, 0x210u64);
            blk = 61; continue;
        }
        19 => {
            c.w32(8, 0x200u64);
            blk = 61; continue;
        }
        20 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903498); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090349f); };
            c.cmp(t14, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 35; continue;
        }
        22 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009034b5); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009034b9); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70009034c6); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009034cc); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(8, 0x6u64);
            blk = 61; continue;
        }
        27 => {
            c.w32(8, 0x6u64);
            blk = 64; continue;
        }
        28 => {
            let r17 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70009034f1); }
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
            c.w32(8, 0x6u64);
            blk = 61; continue;
        }
        31 => {
            c.w32(8, 0x6u64);
            blk = 64; continue;
        }
        32 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903522); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903529); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903532); };
            c.cmp(t21, 0x43u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(8, 0x3feu64);
            blk = 61; continue;
        }
        36 => {
            c.w32(8, 0x1eu64);
            blk = 61; continue;
        }
        37 => {
            let r22 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r22);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000903553); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(8, 0x6u64);
            blk = 61; continue;
        }
        40 => {
            c.w32(8, 0x6u64);
            blk = 64; continue;
        }
        41 => {
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            blk = 61; continue;
        }
        42 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903596); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090359d); };
            c.cmp(t25, c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t26) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009035aa); };
            c.w32(8, t26);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 54; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_ae() { blk = 53; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(8, 0x80u64);
            blk = 64; continue;
        }
        49 => {
            c.w32(8, 0x100u64);
            blk = 64; continue;
        }
        50 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70009035e2); };
            c.cmp(t27, c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(8, 0x210u64);
            blk = 64; continue;
        }
        52 => {
            c.w32(8, 0x8u64);
            blk = 64; continue;
        }
        53 => {
            c.w32(8, 0x200u64);
            blk = 64; continue;
        }
        54 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(8, 0x60u64);
            blk = 57; continue;
        }
        56 => {
            c.w32(8, 0x20u64);
            blk = 57; continue;
        }
        57 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 64; continue;
        }
        58 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700090361c); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            if c.cc_e() { blk = 62; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(8, 0x6u64);
            blk = 61; continue;
        }
        61 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000903630); };
            c.w64(0, t28);
            blk = 65; continue;
        }
        62 => {
            c.w32(8, 0x6u64);
            blk = 64; continue;
        }
        63 => {
            c.w32(8, 0x2u64);
            blk = 64; continue;
        }
        64 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000903647); };
            c.w64(0, t29);
            blk = 65; continue;
        }
        65 => {
            let Some(t30) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090364e); };
            c.w64(11, t30);
            let Some(t31) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903652); };
            c.w64(7, t31);
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000903656); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090365d); };
            c.w32(11, t34);
            let r35 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r35);
            c.w64(3, c.r64(9));
            let Some(t36) = c.ld8(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000903667); };
            c.w32(2, t36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090366c); };
            c.w32(1, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000903670); };
            c.cmp(t38, c.r32(1), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000903679); };
            c.w64(11, t39);
            blk = 67; continue;
        }
        67 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000903680); };
            c.cmp(t40, c.r32(11), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000903689); };
            c.w32(1, t41);
            c.w32(8, c.r32(8));
            c.w32(6, c.r32(1));
            let r42 = c.r32(8) | c.r32(6);
            c.w32(8, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700090369e); }
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009036a3); }
            blk = 70; continue;
        }
        70 => {
            c.w64(9, c.r64(3).wrapping_add(0x3));
            let Some(t43) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009036af); };
            c.w32(8, t43);
            let Some(t44) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009036b4); };
            c.w64(1, t44);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        71 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009036bd); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009036c1); }
            let r45 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r45);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009036cb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009036cf); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009036d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009036d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009036d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009036da); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70009036de); };
            c.w64(3, t46);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70009036f1, 0x70009036ec);
        }
        72 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009036f1); };
            c.w64(7, t47);
            c.w32(8, 0x100u64);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009036fb); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009036ff); };
            c.w64(9, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000903706); }
            blk = 65; continue;
        }
        73 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090370c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000903710); }
            let r50 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r50);
            let Some(t51) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903716); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090371d); };
            c.w32(11, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000903722); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000903726); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090372a); }
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000903738); }
            blk = 35; continue;
        }
        75 => {
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            c.logic(c.r8(3) & 0x20u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(3, 0x60u64);
            blk = 78; continue;
        }
        77 => {
            c.w32(3, 0x20u64);
            blk = 78; continue;
        }
        78 => {
            c.logic(c.r8(11) & 0x20u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w32(11, 0x60u64);
            blk = 81; continue;
        }
        80 => {
            c.w32(11, 0x20u64);
            blk = 81; continue;
        }
        81 => {
            c.w32(11, c.r32(11));
            c.w32(3, c.r32(3));
            let r54 = c.r32(11) | c.r32(3);
            c.w32(11, r54);
            c.logic(r54, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000903772); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903776); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007fb534, 0x7000903784, 0x700090377f);
        }
        82 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000903784); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903788); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090378c); };
            c.w64(15, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000903790); };
            c.w64(9, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000903797); }
            blk = 65; continue;
        }
        83 => {
            return RegfileExit::jump(0x70009037a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009037b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009037b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009037b8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009037be); };
            c.w64(7, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009037c2); };
            c.w64(8, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009037c8); };
            c.w64(8, t3);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009037d0); };
            c.w64(3, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009037d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009037d8); }
            c.w64(2, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(1));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009037e7); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009037eb); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009037ef); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009037f3); };
            c.w64(3, t8);
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(1, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090380d); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903814); };
            c.cmp(t11, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903829); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 52; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 55; continue;
        }
        9 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903855); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090385c); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700090386a); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700090386f); }
            blk = 15; continue;
        }
        13 => {
            let r17 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000903878); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r18 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r18);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000903889); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(8, c.r32(8));
            let r19 = c.r32(8) | 0x6u64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(8, c.r32(8));
            let r20 = c.r32(8) | 0x2u64;
            c.w32(8, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70009038b6); };
            c.w64(0, t21);
            blk = 20; continue;
        }
        19 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70009038bf); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009038c6); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009038ca); };
            c.w64(7, t24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009038ce); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009038d5); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009038dc); };
            c.w64(11, t29);
            let Some(t30) = c.ld8(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009038e0); };
            c.w32(3, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009038e6); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009038ea); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009038f3); };
            c.w64(8, t33);
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009038fa); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000903903); };
            c.w32(2, t35);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r36 = c.r32(1) | c.r32(6);
            c.w32(1, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000903915); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090391a); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t37) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000903926); };
            c.w32(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090392b); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090392f); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000903933); };
            c.w64(5, t40);
            c.w64(12, c.r64(7));
            let r41 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r41);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t42) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903940); };
            c.w32(9, t42);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000903945); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000903949); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700090394d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903951); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000903955); }
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 39; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903976); };
            c.cmp(t43, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t44) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090397f); };
            c.cmp(t44, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(8, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(8, c.r32(8));
            let r45 = c.r32(8) | 0x1eu64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000903997); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009039a3); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x70009039af, 0x70009039aa);
        }
        35 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009039af); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009039b3); };
            c.w64(7, t47);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009039b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009039bb); }
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009039bf); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009039c3); };
            c.w64(6, t49);
            return c.call(0x70008a10d4, 0x70009039cc, 0x70009039c7);
        }
        36 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009039cc); };
            c.w64(1, t50);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009039d0); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009039d4); };
            c.w64(0, t51);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009039dd); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x70009039ec, 0x70009039e7);
        }
        38 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009039ec); };
            c.w64(1, t52);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009039f0); }
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009039f4); };
            c.w64(0, t53);
            blk = 1; continue;
        }
        39 => {
            let Some(t54) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009039fd); };
            c.w32(9, t54);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 63; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903a24); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000903a33, 0x7000903a2e);
        }
        43 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000903a33); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903a37); };
            c.w64(7, t56);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903a3b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000903a3f); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000903a43); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000903a47); };
            c.w64(6, t58);
            return c.call(0x70008a11d4, 0x7000903a50, 0x7000903a4b);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903a5d); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000903a69, 0x7000903a64);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903a73); };
            c.cmp(t59, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t60) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903a80); };
            c.w32(9, t60);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 65; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903a9f); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000903aab, 0x7000903aa6);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000903ab5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000903ab9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903abd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000903ac1); }
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903ac5); };
            c.cmp(t61, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903ad5); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000903ae1, 0x7000903adc);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000903aeb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000903aef); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903af3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000903af7); }
            let Some(t62) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903afb); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903b0a); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000903b19, 0x7000903b14);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000903b1e); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000903b22); };
            c.w64(0, t63);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000903b2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000903b2f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903b33); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000903b37); }
            let Some(t64) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903b3b); };
            c.cmp(t64, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903b4a); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000903b56, 0x7000903b51);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903b60); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903b6a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903b6c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000903b6d); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000903b73); };
            c.w64(3, t65);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000903b82, 0x7000903b7d);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903b87); };
            c.w64(7, t66);
            blk = 20; continue;
        }
        63 => {
            c.w32(8, c.r32(8));
            let r67 = c.r32(8) | 0x60u64;
            c.w32(8, r67);
            c.logic(r67, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903b9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000903b9e); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb9f4, 0x7000903baa, 0x7000903ba5);
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000903baa); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903bae); };
            c.w64(7, t69);
            blk = 20; continue;
        }
        65 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903bb7); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903bc1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000903bc3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903bc4); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000903bc5); };
            c.w64(3, t70);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000903bd6, 0x7000903bd1);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903bdb); };
            c.w64(7, t71);
            blk = 20; continue;
        }
        67 => {
            c.w32(8, c.r32(8));
            let r72 = c.r32(8) | 0x100u64;
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903bf1); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903bfb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000903bfd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903bfe); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000903bff); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000903c03); };
            c.w64(3, t73);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000903c14, 0x7000903c0f);
        }
        68 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000903c14); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903c18); };
            c.w64(7, t75);
            blk = 20; continue;
        }
        69 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903c21); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903c2b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000903c2d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903c2e); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000903c2f); };
            c.w64(3, t76);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000903c40, 0x7000903c3b);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903c45); };
            c.w64(7, t77);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903c4e); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903c58); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903c5a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000903c5b); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000903c5c); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000903c6d, 0x7000903c68);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903c72); };
            c.w64(7, t79);
            blk = 20; continue;
        }
        73 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903c7b); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903c85); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000903c87); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903c88); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000903c89); };
            c.w64(3, t80);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000903c9a, 0x7000903c95);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903c9f); };
            c.w64(7, t81);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903ca8); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000903cb2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000903cb4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000903cb5); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000903cb6); };
            c.w64(3, t82);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000903cc7, 0x7000903cc2);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903ccc); };
            c.w64(7, t83);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x7000903cd8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000903cf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000903cf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000903cf8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000903cfe); };
            c.w64(7, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000903d02); };
            c.w64(8, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000903d08); };
            c.w64(8, t3);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000903d10); };
            c.w64(3, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000903d14); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000903d18); }
            c.w64(2, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(1));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000903d27); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000903d2b); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903d2f); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000903d33); };
            c.w64(3, t8);
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(1, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903d4d); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903d54); };
            c.cmp(t11, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903d69); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 52; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 55; continue;
        }
        9 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903d95); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903d9c); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000903daa); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000903daf); }
            blk = 15; continue;
        }
        13 => {
            let r17 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000903db8); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r18 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r18);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000903dc9); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(8, c.r32(8));
            let r19 = c.r32(8) | 0x6u64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(8, c.r32(8));
            let r20 = c.r32(8) | 0x2u64;
            c.w32(8, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000903df6); };
            c.w64(0, t21);
            blk = 20; continue;
        }
        19 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000903dff); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000903e06); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903e0a); };
            c.w64(7, t24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000903e0e); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000903e15); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000903e1c); };
            c.w64(11, t29);
            let Some(t30) = c.ld8(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000903e20); };
            c.w32(3, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000903e26); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000903e2a); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000903e33); };
            c.w64(8, t33);
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000903e3a); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000903e43); };
            c.w32(2, t35);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r36 = c.r32(1) | c.r32(6);
            c.w32(1, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000903e55); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000903e5a); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t37) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000903e66); };
            c.w32(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000903e6b); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000903e6f); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000903e73); };
            c.w64(5, t40);
            c.w64(12, c.r64(7));
            let r41 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r41);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t42) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903e80); };
            c.w32(9, t42);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000903e85); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000903e89); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000903e8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903e91); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000903e95); }
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 39; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903eb6); };
            c.cmp(t43, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t44) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903ebf); };
            c.cmp(t44, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(8, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(8, c.r32(8));
            let r45 = c.r32(8) | 0x1eu64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000903ed7); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903ee3); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x7000903eef, 0x7000903eea);
        }
        35 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000903eef); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903ef3); };
            c.w64(7, t47);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903ef7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000903efb); }
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000903eff); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000903f03); };
            c.w64(6, t49);
            return c.call(0x70008a10d4, 0x7000903f0c, 0x7000903f07);
        }
        36 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000903f0c); };
            c.w64(1, t50);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000903f10); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000903f14); };
            c.w64(0, t51);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000903f1d); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000903f2c, 0x7000903f27);
        }
        38 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000903f2c); };
            c.w64(1, t52);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000903f30); }
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000903f34); };
            c.w64(0, t53);
            blk = 1; continue;
        }
        39 => {
            let Some(t54) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903f3d); };
            c.w32(9, t54);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 63; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903f64); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000903f73, 0x7000903f6e);
        }
        43 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000903f73); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000903f77); };
            c.w64(7, t56);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903f7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000903f7f); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000903f83); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000903f87); };
            c.w64(6, t58);
            return c.call(0x70008a11d4, 0x7000903f90, 0x7000903f8b);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903f9d); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000903fa9, 0x7000903fa4);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000903fb3); };
            c.cmp(t59, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t60) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000903fc0); };
            c.w32(9, t60);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 65; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000903fdf); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000903feb, 0x7000903fe6);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000903ff5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000903ff9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000903ffd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904001); }
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904005); };
            c.cmp(t61, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904015); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904021, 0x700090401c);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090402b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090402f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904033); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904037); }
            let Some(t62) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090403b); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700090404a); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904059, 0x7000904054);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090405e); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000904062); };
            c.w64(0, t63);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090406b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700090406f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904073); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904077); }
            let Some(t64) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090407b); };
            c.cmp(t64, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700090408a); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904096, 0x7000904091);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009040a0); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009040aa); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009040ac); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009040ad); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70009040b3); };
            c.w64(3, t65);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70009040c2, 0x70009040bd);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009040c7); };
            c.w64(7, t66);
            blk = 20; continue;
        }
        63 => {
            c.w32(8, c.r32(8));
            let r67 = c.r32(8) | 0x60u64;
            c.w32(8, r67);
            c.logic(r67, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009040da); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70009040de); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb774, 0x70009040ea, 0x70009040e5);
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009040ea); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009040ee); };
            c.w64(7, t69);
            blk = 20; continue;
        }
        65 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009040f7); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904101); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904103); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904104); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000904105); };
            c.w64(3, t70);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904116, 0x7000904111);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090411b); };
            c.w64(7, t71);
            blk = 20; continue;
        }
        67 => {
            c.w32(8, c.r32(8));
            let r72 = c.r32(8) | 0x100u64;
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904131); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090413b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090413d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090413e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700090413f); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000904143); };
            c.w64(3, t73);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904154, 0x700090414f);
        }
        68 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904154); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904158); };
            c.w64(7, t75);
            blk = 20; continue;
        }
        69 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904161); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090416b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090416d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090416e); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700090416f); };
            c.w64(3, t76);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904180, 0x700090417b);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904185); };
            c.w64(7, t77);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090418e); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904198); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090419a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090419b); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700090419c); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009041ad, 0x70009041a8);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009041b2); };
            c.w64(7, t79);
            blk = 20; continue;
        }
        73 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009041bb); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009041c5); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009041c7); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009041c8); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009041c9); };
            c.w64(3, t80);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009041da, 0x70009041d5);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009041df); };
            c.w64(7, t81);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009041e8); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009041f2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009041f4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009041f5); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009041f6); };
            c.w64(3, t82);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904207, 0x7000904202);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090420c); };
            c.w64(7, t83);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x7000904218);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000904234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000904234); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000904238); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090423e); };
            c.w64(7, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000904242); };
            c.w64(8, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000904248); };
            c.w64(8, t3);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000904250); };
            c.w64(3, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000904254); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000904258); }
            c.w64(2, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(1));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000904267); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090426b); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090426f); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000904273); };
            c.w64(3, t8);
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(1, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090428d); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000904294); };
            c.cmp(t11, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009042a9); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 52; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 55; continue;
        }
        9 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009042d5); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009042dc); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70009042ea); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009042ef); }
            blk = 15; continue;
        }
        13 => {
            let r17 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70009042f8); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r18 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r18);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000904309); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(8, c.r32(8));
            let r19 = c.r32(8) | 0x6u64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(8, c.r32(8));
            let r20 = c.r32(8) | 0x2u64;
            c.w32(8, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000904336); };
            c.w64(0, t21);
            blk = 20; continue;
        }
        19 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700090433f); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000904346); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090434a); };
            c.w64(7, t24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090434e); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000904355); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090435c); };
            c.w64(11, t29);
            let Some(t30) = c.ld8(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000904360); };
            c.w32(3, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904366); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090436a); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000904373); };
            c.w64(8, t33);
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090437a); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000904383); };
            c.w32(2, t35);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r36 = c.r32(1) | c.r32(6);
            c.w32(1, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000904395); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700090439a); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t37) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009043a6); };
            c.w32(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009043ab); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009043af); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009043b3); };
            c.w64(5, t40);
            c.w64(12, c.r64(7));
            let r41 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r41);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t42) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009043c0); };
            c.w32(9, t42);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009043c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009043c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009043cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009043d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009043d5); }
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 39; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009043f6); };
            c.cmp(t43, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t44) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009043ff); };
            c.cmp(t44, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(8, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(8, c.r32(8));
            let r45 = c.r32(8) | 0x1eu64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000904417); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904423); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x700090442f, 0x700090442a);
        }
        35 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090442f); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904433); };
            c.w64(7, t47);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904437); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090443b); }
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090443f); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000904443); };
            c.w64(6, t49);
            return c.call(0x70008a10d4, 0x700090444c, 0x7000904447);
        }
        36 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090444c); };
            c.w64(1, t50);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000904450); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000904454); };
            c.w64(0, t51);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090445d); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700090446c, 0x7000904467);
        }
        38 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090446c); };
            c.w64(1, t52);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000904470); }
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000904474); };
            c.w64(0, t53);
            blk = 1; continue;
        }
        39 => {
            let Some(t54) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090447d); };
            c.w32(9, t54);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 63; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009044a4); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009044b3, 0x70009044ae);
        }
        43 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009044b3); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009044b7); };
            c.w64(7, t56);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009044bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009044bf); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009044c3); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009044c7); };
            c.w64(6, t58);
            return c.call(0x70008a11d4, 0x70009044d0, 0x70009044cb);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009044dd); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x70009044e9, 0x70009044e4);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009044f3); };
            c.cmp(t59, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t60) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904500); };
            c.w32(9, t60);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 65; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700090451f); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700090452b, 0x7000904526);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000904535); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000904539); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700090453d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904541); }
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904545); };
            c.cmp(t61, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904555); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904561, 0x700090455c);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090456b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090456f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904573); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904577); }
            let Some(t62) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090457b); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700090458a); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904599, 0x7000904594);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090459e); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009045a2); };
            c.w64(0, t63);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009045ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009045af); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009045b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009045b7); }
            let Some(t64) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009045bb); };
            c.cmp(t64, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009045ca); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009045d6, 0x70009045d1);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009045e0); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009045ea); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009045ec); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009045ed); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70009045f3); };
            c.w64(3, t65);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000904602, 0x70009045fd);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904607); };
            c.w64(7, t66);
            blk = 20; continue;
        }
        63 => {
            c.w32(8, c.r32(8));
            let r67 = c.r32(8) | 0x60u64;
            c.w32(8, r67);
            c.logic(r67, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090461a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700090461e); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fbb34, 0x700090462a, 0x7000904625);
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090462a); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090462e); };
            c.w64(7, t69);
            blk = 20; continue;
        }
        65 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904637); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904641); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904643); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904644); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000904645); };
            c.w64(3, t70);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904656, 0x7000904651);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090465b); };
            c.w64(7, t71);
            blk = 20; continue;
        }
        67 => {
            c.w32(8, c.r32(8));
            let r72 = c.r32(8) | 0x100u64;
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904671); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090467b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090467d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090467e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700090467f); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000904683); };
            c.w64(3, t73);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904694, 0x700090468f);
        }
        68 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904694); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904698); };
            c.w64(7, t75);
            blk = 20; continue;
        }
        69 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009046a1); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009046ab); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009046ad); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009046ae); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009046af); };
            c.w64(3, t76);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009046c0, 0x70009046bb);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009046c5); };
            c.w64(7, t77);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009046ce); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009046d8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009046da); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009046db); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009046dc); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009046ed, 0x70009046e8);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009046f2); };
            c.w64(7, t79);
            blk = 20; continue;
        }
        73 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009046fb); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904705); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904707); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904708); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000904709); };
            c.w64(3, t80);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700090471a, 0x7000904715);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090471f); };
            c.w64(7, t81);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904728); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904732); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904734); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904735); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000904736); };
            c.w64(3, t82);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904747, 0x7000904742);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090474c); };
            c.w64(7, t83);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x7000904758);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000904774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000904774); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000904778); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090477e); };
            c.w64(7, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000904782); };
            c.w64(8, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000904788); };
            c.w64(8, t3);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000904790); };
            c.w64(3, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000904794); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000904798); }
            c.w64(2, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(1));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009047a7); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009047ab); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009047af); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009047b3); };
            c.w64(3, t8);
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(1, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009047cd); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009047d4); };
            c.cmp(t11, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009047e9); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 52; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 55; continue;
        }
        9 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904815); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090481c); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700090482a); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700090482f); }
            blk = 15; continue;
        }
        13 => {
            let r17 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000904838); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r18 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r18);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000904849); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(8, c.r32(8));
            let r19 = c.r32(8) | 0x6u64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(8, c.r32(8));
            let r20 = c.r32(8) | 0x2u64;
            c.w32(8, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000904876); };
            c.w64(0, t21);
            blk = 20; continue;
        }
        19 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700090487f); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000904886); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090488a); };
            c.w64(7, t24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090488e); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000904895); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090489c); };
            c.w64(11, t29);
            let Some(t30) = c.ld8(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009048a0); };
            c.w32(3, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009048a6); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009048aa); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009048b3); };
            c.w64(8, t33);
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009048ba); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009048c3); };
            c.w32(2, t35);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r36 = c.r32(1) | c.r32(6);
            c.w32(1, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70009048d5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009048da); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t37) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009048e6); };
            c.w32(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009048eb); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009048ef); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009048f3); };
            c.w64(5, t40);
            c.w64(12, c.r64(7));
            let r41 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r41);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t42) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904900); };
            c.w32(9, t42);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000904905); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000904909); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700090490d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904911); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904915); }
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 39; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000904936); };
            c.cmp(t43, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t44) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090493f); };
            c.cmp(t44, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(8, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(8, c.r32(8));
            let r45 = c.r32(8) | 0x1eu64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000904957); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904963); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x700090496f, 0x700090496a);
        }
        35 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090496f); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904973); };
            c.w64(7, t47);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904977); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090497b); }
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090497f); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000904983); };
            c.w64(6, t49);
            return c.call(0x70008a10d4, 0x700090498c, 0x7000904987);
        }
        36 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090498c); };
            c.w64(1, t50);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000904990); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000904994); };
            c.w64(0, t51);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090499d); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x70009049ac, 0x70009049a7);
        }
        38 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009049ac); };
            c.w64(1, t52);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009049b0); }
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009049b4); };
            c.w64(0, t53);
            blk = 1; continue;
        }
        39 => {
            let Some(t54) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009049bd); };
            c.w32(9, t54);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 63; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009049e4); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009049f3, 0x70009049ee);
        }
        43 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009049f3); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009049f7); };
            c.w64(7, t56);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009049fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009049ff); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000904a03); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000904a07); };
            c.w64(6, t58);
            return c.call(0x70008a11d4, 0x7000904a10, 0x7000904a0b);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904a1d); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000904a29, 0x7000904a24);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000904a33); };
            c.cmp(t59, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t60) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904a40); };
            c.w32(9, t60);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 65; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904a5f); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904a6b, 0x7000904a66);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000904a75); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000904a79); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904a7d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904a81); }
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904a85); };
            c.cmp(t61, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904a95); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904aa1, 0x7000904a9c);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000904aab); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000904aaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904ab3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904ab7); }
            let Some(t62) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904abb); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904aca); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904ad9, 0x7000904ad4);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000904ade); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000904ae2); };
            c.w64(0, t63);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000904aeb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000904aef); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904af3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000904af7); }
            let Some(t64) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904afb); };
            c.cmp(t64, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904b0a); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000904b16, 0x7000904b11);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904b20); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904b2a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904b2c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904b2d); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000904b33); };
            c.w64(3, t65);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000904b42, 0x7000904b3d);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904b47); };
            c.w64(7, t66);
            blk = 20; continue;
        }
        63 => {
            c.w32(8, c.r32(8));
            let r67 = c.r32(8) | 0x60u64;
            c.w32(8, r67);
            c.logic(r67, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904b5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000904b5e); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb8b4, 0x7000904b6a, 0x7000904b65);
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904b6a); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904b6e); };
            c.w64(7, t69);
            blk = 20; continue;
        }
        65 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904b77); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904b81); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904b83); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904b84); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000904b85); };
            c.w64(3, t70);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904b96, 0x7000904b91);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904b9b); };
            c.w64(7, t71);
            blk = 20; continue;
        }
        67 => {
            c.w32(8, c.r32(8));
            let r72 = c.r32(8) | 0x100u64;
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904bb1); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904bbb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904bbd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904bbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000904bbf); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000904bc3); };
            c.w64(3, t73);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904bd4, 0x7000904bcf);
        }
        68 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904bd4); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904bd8); };
            c.w64(7, t75);
            blk = 20; continue;
        }
        69 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904be1); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904beb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904bed); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904bee); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000904bef); };
            c.w64(3, t76);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904c00, 0x7000904bfb);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904c05); };
            c.w64(7, t77);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904c0e); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904c18); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904c1a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904c1b); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000904c1c); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904c2d, 0x7000904c28);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904c32); };
            c.w64(7, t79);
            blk = 20; continue;
        }
        73 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904c3b); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904c45); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904c47); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904c48); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000904c49); };
            c.w64(3, t80);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904c5a, 0x7000904c55);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904c5f); };
            c.w64(7, t81);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904c68); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904c72); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000904c74); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000904c75); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000904c76); };
            c.w64(3, t82);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000904c87, 0x7000904c82);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904c8c); };
            c.w64(7, t83);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x7000904c98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000904cb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000904cb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000904cb8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000904cbe); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000904cc2); };
            c.w64(11, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000904cc6); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000904ccc); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000904cd3); };
            c.w64(3, t6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000904cd7); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000904cde); };
            c.w32(2, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904ce4); };
            c.w32(1, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x7000904ce8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000904cec); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000904cf0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x7000904cf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000904cf8); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000904cfc); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000904d05); };
            c.w64(11, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000904d0c); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904d10); };
            c.w32(6, t14);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000904d1e); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000904d22); };
            c.cmp(t15, c.r32(6), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000904d37); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            c.w32(3, c.r32(12));
            let r18 = c.r32(3) & 0xfffffffffffffffdu64;
            c.w32(3, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000904d4d); };
            c.cmp(t19, c.r32(12), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000904d5a); };
            c.cmp(t20, c.r32(12), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(3, c.r64(0));
            blk = 10; continue;
        }
        9 => {
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000904d7b); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            blk = 10; continue;
        }
        10 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904d81); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904d88); };
            c.w32(12, t25);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(12, c.r16(12));
            let r26 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r26);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000904da7); };
            c.w32(3, t27);
            let r28 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r28);
            let Some(t29) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904dad); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904db4); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000904dba); };
            c.w32(12, t33);
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000904dbf); };
            c.cmp(t34, c.r32(12), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(3, c.r64(0));
            let r35 = c.r64(3) | 0x3u64;
            c.w64(3, r35);
            c.logic(r35, 64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(6)), c.r32(3)) { return RegfileExit::fault(0x7000904dcc); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r36 = c.r64(2) & c.r64(11);
            c.w64(2, r36);
            c.logic(r36, 64);
            let Some(t37) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000904ddf); };
            c.logic(t37 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000904de9); }
            blk = 17; continue;
        }
        16 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000904df3); };
            c.w64(3, t38);
            if !c.st32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x7000904dfa); }
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000904dff); }
            blk = 17; continue;
        }
        17 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000904e0b); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t39) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904e18); };
            c.w32(3, t39);
            let r40 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r40);
            let Some(t41) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904e1e); };
            c.cmp(t41, 0xbau64, 16);
            if c.cc_b() { blk = 48; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904e2a); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xd08)) else { return RegfileExit::fault(0x7000904e2e); };
            c.w64(3, t42);
            c.w64(6, c.r64(1));
            return c.call(0x7000870914, 0x7000904e3d, 0x7000904e38);
        }
        20 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000904e3d); };
            c.w64(6, t43);
            let Some(t44) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904e41); };
            c.w32(8, t44);
            let r45 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000904e48); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000904e4f); };
            c.w32(11, t48);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xe4u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000904e62);
        }
        22 => {
            let Some(t49) = c.ld32(c.r64(8).wrapping_add(0x397)) else { return RegfileExit::fault(0x7000904e63); };
            c.cmp(t49, c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000904e70); };
            c.cmp(t50, c.r32(0), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000904e7d); };
            c.cmp(t51, c.r32(0), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000904e8a); };
            c.w64(8, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904e8e); };
            c.w64(11, t53);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904e92); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000904e96); };
            if !c.push64(t54) { return RegfileExit::fault(0x7000904e96); }
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000904e99); };
            if !c.push64(t55) { return RegfileExit::fault(0x7000904e99); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x7000904eac, 0x7000904ea7);
        }
        26 => {
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000904eac); };
            c.cmp(t56, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000904ebc); };
            c.cmp(t57, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t58) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904ec5); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            let Some(t60) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000904ecc); };
            c.logic(t60 & 0x10u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t61) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000904ed3); };
            c.cmp(t61, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000904ee0); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 39; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 34; continue;
        }
        33 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000904ef0); };
            c.w64(0, t63);
            c.w64(8, c.r64(0));
            blk = 37; continue;
        }
        35 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000904efc); };
            c.w64(0, t64);
            c.w64(8, c.r64(0));
            blk = 37; continue;
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904f08); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000904f0c); };
            c.w64(11, t66);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904f10); }
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000904f14); };
            if !c.push64(t67) { return RegfileExit::fault(0x7000904f14); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000904f17); };
            if !c.push64(t68) { return RegfileExit::fault(0x7000904f17); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            return c.call(0x70008a8174, 0x7000904f27, 0x7000904f22);
        }
        37 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000904f27); };
            c.w64(11, t69);
            let Some(t70) = c.ld64(c.r64(11).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904f2b); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000904f2f); };
            c.w64(11, t71);
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t72) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000904f37); };
            c.w32(11, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000904f3c); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000904f40); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000904f44); };
            c.w64(5, t75);
            c.w64(12, c.r64(8));
            let r76 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r76);
            return RegfileExit::jump(c.r64(1));
        }
        38 => {
            blk = 48; continue;
        }
        39 => {
            let Some(t77) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000904f53); };
            c.logic(t77 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 34; continue;
        }
        41 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000904f5e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 34; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 35; continue;
        }
        43 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000904f6f); };
            c.w64(2, t78);
            let Some(t79) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000904f73); };
            c.w32(8, t79);
            let r80 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r80);
            let Some(t81) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000904f7a); };
            c.logic(t81 & 0x2u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904f81); };
            c.w64(8, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000904f85); };
            c.w64(11, t83);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000904f89); }
            c.w32(8, 0xa4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904f93); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000904f95); };
            c.w64(3, t84);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000904fa6, 0x7000904fa1);
        }
        45 => {
            return RegfileExit::fault(0x7000904fa6);
        }
        46 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000904fa7); };
            c.w64(8, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000904fab); };
            c.w64(11, t86);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904faf); }
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000904fb3); };
            c.w64(0, t87);
            return c.call(0x7000857594, 0x7000904fbc, 0x7000904fb7);
        }
        47 => {
            blk = 37; continue;
        }
        48 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000904fc1); }
            c.w32(8, 0xaeu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000904fcb); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000904fcd); };
            c.w64(3, t88);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000904fe1, 0x7000904fdc);
        }
        49 => {
            return RegfileExit::fault(0x7000904fe1);
        }
        50 => {
            c.w32(3, c.r32(3));
            let r89 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r89);
            c.w64(2, 0xfffffffffffc0000u64);
            let r90 = c.r64(2) & c.r64(3);
            c.w64(2, r90);
            c.logic(r90, 64);
            let Some(t91) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000904ff1); };
            c.logic(t91 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000904fff); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000905000); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700090500d, 0x7000905008);
        }
        52 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700090500d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700090500f); }
            blk = 15; continue;
        }
        53 => {
            return RegfileExit::jump(0x7000905018);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905034); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905038); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090503e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905042); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000905046); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090504c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905053); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905057); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x700090505e); };
            c.w32(3, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905064); };
            c.w32(2, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000905068); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090506c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000905070); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000905074); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090507d); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000905084); };
            c.w64(2, t13);
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000905092); }
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(1, c.r64(11));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(8));
            return c.call(0x70008436f4, 0x70009050aa, 0x70009050a5);
        }
        3 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009050aa); };
            c.w64(7, t14);
            let Some(t15) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009050ae); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009050b2); };
            c.w64(8, t16);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009050ba); };
            c.w32(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009050bf); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009050c3); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009050c7); };
            c.w64(5, t20);
            c.w64(12, c.r64(7));
            let r21 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r21);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70009050d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009050f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009050f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009050f8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009050fe); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000905102); };
            c.w64(2, t2);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700090510a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700090510e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000905112); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000905116); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700090511a); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a1af4, 0x7000905126, 0x7000905121);
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000905126); };
            c.w64(3, t3);
            let Some(t4) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090512a); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090512e); };
            c.w64(7, t5);
            let Some(t6) = c.ld8(c.r64(2).wrapping_add(c.r64(7)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000905132); };
            c.w64(8, (t6 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x7000905138); }
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000905140); };
            c.w32(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000905145); };
            c.w64(15, t8);
            let Some(t9) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905149); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090514d); };
            c.w64(5, t10);
            c.w64(12, c.r64(2));
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000905154); };
            c.w64(0, t11);
            let r12 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r12);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000905160);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905174); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905178); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090517e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000905182); };
            c.w64(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000905186); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090518a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700090518e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905196); };
            c.w32(7, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000905199); };
            c.cmp(t4, c.r32(7), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0xeu64);
            blk = 6; continue;
        }
        3 => {
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70009051ad); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a1014, 0x70009051b9, 0x70009051b4);
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009051b9); };
            c.w64(3, t5);
            c.w32(2, 0xfeu64);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009051c2); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009051c6); };
            c.w64(9, t7);
            blk = 6; continue;
        }
        5 => {
            c.w32(2, 0x2u64);
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009051d1); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009051d5); };
            c.w64(3, t9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009051d9); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009051df); };
            c.w32(7, t12);
            let r13 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r13);
            c.w64(8, c.r64(9));
            let Some(t14) = c.ld8(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009051e8); };
            c.w32(11, t14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009051ee); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009051f1); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009051fa); };
            c.w64(7, t17);
            blk = 8; continue;
        }
        8 => {
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000905201); };
            c.cmp(t18, c.r32(7), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090520a); };
            c.w32(1, t19);
            c.w32(2, c.r32(2));
            c.w32(6, c.r32(1));
            let r20 = c.r32(2) | c.r32(6);
            c.w32(2, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700090521c); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000905221); }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, c.r64(8).wrapping_add(0x2));
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700090522c); };
            c.w32(2, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905231); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000905235); };
            c.w64(5, t23);
            c.w64(12, c.r64(3));
            let r24 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r24);
            return RegfileExit::jump(c.r64(1));
        }
        12 => {
            return RegfileExit::jump(0x7000905244);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905254(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905254); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905258); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090525e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000905262); };
            c.w64(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000905266); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090526a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700090526e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905276); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090527c); };
            c.cmp(t5, c.r32(7), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0xeu64);
            blk = 8; continue;
        }
        3 => {
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090528c); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000905297); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70009052a3, 0x700090529e);
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009052a3); };
            c.w64(3, t7);
            c.w32(2, 0xfeu64);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009052ac); };
            c.w64(15, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009052b0); };
            c.w64(9, t9);
            blk = 8; continue;
        }
        6 => {
            c.w32(2, 0x40u64);
            blk = 8; continue;
        }
        7 => {
            c.w32(2, 0x2u64);
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009052c2); };
            c.w64(7, t10);
            let Some(t11) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009052c6); };
            c.w64(3, t11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009052ca); };
            c.w32(7, t12);
            let r13 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r13);
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009052d0); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            c.w64(8, c.r64(9));
            let Some(t16) = c.ld8(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009052d9); };
            c.w32(11, t16);
            let Some(t17) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009052df); };
            c.w32(1, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009052e2); };
            c.cmp(t18, c.r32(1), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009052eb); };
            c.w64(7, t19);
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009052f2); };
            c.cmp(t20, c.r32(7), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009052fb); };
            c.w32(1, t21);
            c.w32(2, c.r32(2));
            c.w32(6, c.r32(1));
            let r22 = c.r32(2) | c.r32(6);
            c.w32(2, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700090530d); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000905312); }
            blk = 13; continue;
        }
        13 => {
            c.w64(9, c.r64(8).wrapping_add(0x2));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700090531d); };
            c.w32(2, t23);
            let Some(t24) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905322); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000905326); };
            c.w64(5, t25);
            c.w64(12, c.r64(3));
            let r26 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            return RegfileExit::jump(0x7000905334);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905354); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905358); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090535e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000905362); };
            c.w64(2, t2);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700090536a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700090536e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000905372); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000905376); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700090537a); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a2054, 0x7000905386, 0x7000905381);
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000905386); };
            c.w64(3, t3);
            let Some(t4) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090538a); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090538e); };
            c.w64(7, t5);
            let Some(t6) = c.ld8(c.r64(2).wrapping_add(c.r64(7)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000905392); };
            c.w64(8, (t6 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x7000905398); }
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009053a0); };
            c.w32(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009053a5); };
            c.w64(15, t8);
            let Some(t9) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009053a9); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009053ad); };
            c.w64(5, t10);
            c.w64(12, c.r64(2));
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70009053b4); };
            c.w64(0, t11);
            let r12 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r12);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009053c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009053d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009053d7); };
            c.w64(2, t0);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009053df); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009053e5); };
            c.cmp(t3, 0x40u64, 16);
            if c.cc_ae() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009053ec); };
            c.w64(3, t4);
            let r5 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70009053f4); };
            c.w32(2, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009053f9); };
            c.w64(1, t7);
            c.w64(12, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905402); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905406); }
            let r8 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r8);
            blk = 5; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090540e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905412); }
            let r9 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r9);
            blk = 5; continue;
        }
        5 => {
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700090541c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000905420); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000905424); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000905428); }
            c.w64(6, c.r64(2));
            return c.call(0x700089c794, 0x7000905434, 0x700090542f);
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905434); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905438); };
            c.w64(15, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090543c); };
            c.w64(9, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000905443); }
            blk = 2; continue;
        }
        7 => {
            return RegfileExit::jump(0x7000905448);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(8, c.r64(5));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700090545b); }
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090545f); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905463); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000905469); };
            c.w32(11, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090546f); };
            c.w32(0, t4);
            let r5 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r5);
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905477); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090547d); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905485); };
            c.w32(0, t10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000905488); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000905491); };
            c.w64(7, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905498); };
            c.w32(0, t13);
            let Some(t14) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090549e); };
            c.w64(3, t14);
            let Some(t15) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009054a2); };
            c.w32(2, t15);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009054ab); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009054b8); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009054c8); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70009054cf); };
            c.w64(11, t20);
            c.w64(0, c.r64(11).wrapping_add(0x1c));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009054d6); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70009054dd); };
            c.cmp(t22, c.r64(0), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x70009054e6); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009054ed); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009054f4); };
            c.w32(11, t25);
            let r26 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r26);
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009054fb); };
            c.w32(0, t27);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x700090550d);
        }
        8 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x700090550e); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000905518); };
            c.w32(11, t30);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700090551c); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000905520); };
            c.w64(11, t31);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000905527); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700090552b); }
            let Some(t32) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090552f); };
            c.w32(11, t32);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000905533); }
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000905537); };
            c.w32(11, t33);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700090553b); }
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090553f); };
            c.w32(11, t34);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x7000905543); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x700090554a); }
            blk = 9; continue;
        }
        9 => {
            let Some(t35) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090554e); };
            c.w64(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905555); };
            c.w64(8, t36);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t37) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000905560); };
            c.w32(11, t37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000905565); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090556c); };
            c.w64(1, t39);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905578); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090557c); }
            let r40 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r40);
            blk = 14; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905584); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905588); }
            let r41 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r41);
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000905593); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000905597); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700090559b); }
            return c.call(0x70007febf4, 0x70009055a4, 0x700090559f);
        }
        12 => {
            c.w64(7, c.r64(0));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009055a7); };
            c.w64(8, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009055ab); };
            c.w64(3, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009055af); };
            c.w64(1, t44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009055b6); }
            blk = 6; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009055bc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009055c0); }
            let r45 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r45);
            blk = 14; continue;
        }
        14 => {
            let r46 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r46);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009055c8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009055c9); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009055ca); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009055cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70009055cd); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x4u64);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2290)) else { return RegfileExit::fault(0x70009055d9); };
            c.w64(3, t47);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70009055e8, 0x70009055e3);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009055eb); };
            c.w64(8, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009055f2); }
            blk = 9; continue;
        }
        16 => {
            return RegfileExit::jump(0x70009055fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905614); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905618); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090561e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905622); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905626); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090562d); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905634); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000905638); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700090563c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000905640); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000905644); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090564d); };
            c.w64(11, t9);
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000905654); };
            c.w32(0, t10);
            let Some(t11) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090565a); };
            c.w64(3, t11);
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905661); };
            c.w32(2, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000905667); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090566b); };
            c.cmp(t13, c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x20u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000905681); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000905689); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905696); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090569d); };
            c.w32(0, t18);
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009056a1); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009056a8); };
            c.w32(11, t21);
            let r22 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r22);
            c.w64(6, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t23) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x70009056b2); };
            c.w32(2, t23);
            let Some(t24) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009056b6); };
            c.w32(8, t24);
            c.w64(12, c.r64(6));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70009056c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70009056c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70009056c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70009056cc); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 30; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70009056da); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70009056e7); };
            c.cmp(t26, c.r32(8), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 43; continue; }
            blk = 9; continue;
        }
        9 => {
            let r27 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r27);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000905707); };
            c.w64(7, t28);
            let Some(t29) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700090570e); };
            c.w64(7, t29);
            c.w64(0, c.r64(6).wrapping_add(0x8));
            c.w64(3, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000905719); };
            c.w64(2, t30);
            let Some(t31) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000905720); };
            c.cmp(t31, c.r64(3), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 41; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000905735); };
            c.w64(0, t32);
            if !c.st64(c.r64(0), c.r64(3)) { return RegfileExit::fault(0x700090573c); }
            let r33 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r33);
            blk = 12; continue;
        }
        12 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000905743); }
            c.w64(0, c.r64(7));
            let r34 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r34);
            c.logic(r34, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000905750); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000905754); }
            let Some(t35) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000905758); };
            c.logic(t35 & 0x4u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(6).wrapping_add(0x7));
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(3, 0x7u64);
            blk = 15; continue;
        }
        15 => {
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700090576d); };
            c.w32(2, t36);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x7000905771); }
            let r37 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r37);
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_ne() { blk = 15; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 19; continue;
        }
        17 => {
            c.w64(2, c.r64(11).wrapping_add(0x7));
            c.w64(11, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r38 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r38);
            let r39 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r39);
            c.logic(r39, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000905792); }
            c.w64(1, c.r64(11));
            c.w64(8, c.r64(6));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700090579d); };
            c.w64(0, t40);
            c.w64(10, 0x70009057abu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70009057ab); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70009057af); }
            return c.call(c.r64(0), 0x70009057b5, 0x70009057b3);
        }
        18 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70009057b5); }
            let Some(t41) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70009057bd); };
            c.w64(4, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009057c2); };
            c.w64(15, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009057c6); };
            c.w64(9, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009057ca); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70009057ce); };
            c.w64(7, t45);
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(7));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009057d5); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009057d9); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009057dd); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009057e1); };
            c.w64(2, t49);
            blk = 31; continue;
        }
        20 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 49; continue; }
            blk = 21; continue;
        }
        21 => {
            if let Some(sr) = c.shl(c.r64(6), 0x2u64, 64) { c.w64(6, sr); }
            c.w64(11, c.r64(6).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70009057ff); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 22; continue;
        }
        22 => {
            let r50 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000905813); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700090581a); };
            c.w64(3, t52);
            let Some(t53) = c.ld64(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090581d); };
            c.w64(0, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x7000905821); }
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let r54 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r54);
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000905835); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000905839); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000905841); }
            c.w32(0, 0x2u64);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000905848); };
            c.w64(3, t55);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000905856, 0x7000905851);
        }
        24 => {
            blk = 27; continue;
        }
        25 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(3)));
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000905865); };
            c.w64(11, t56);
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x700090586c); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 27; continue;
        }
        27 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000905876); };
            c.w64(7, t57);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700090587a); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r58 = c.r64(11) & c.r64(0);
            c.w64(11, r58);
            c.logic(r58, 64);
            let Some(t59) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000905887); };
            c.logic(t59 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000905892); };
            c.w64(7, t60);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000905896); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000905899); };
            c.w64(7, t61);
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r62 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r62);
            let r63 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r63);
            c.logic(r63, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70009058b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70009058b5); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009058b9); };
            c.w64(8, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70009058bd); };
            c.w64(0, t65);
            c.w64(10, 0x70009058cbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70009058cb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70009058cf); }
            return c.call(c.r64(0), 0x70009058d5, 0x70009058d3);
        }
        29 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70009058d5); }
            let Some(t66) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70009058dd); };
            c.w64(4, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70009058e2); };
            c.w64(11, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009058e6); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009058ea); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009058ee); };
            c.w64(3, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009058f2); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009058f6); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009058fa); };
            c.w64(0, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009058fe); };
            c.w64(2, t74);
            blk = 31; continue;
        }
        30 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000905904); };
            c.w64(11, t75);
            blk = 31; continue;
        }
        31 => {
            let r76 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r76);
            let r77 = c.r32(2) & 0x3fu64;
            c.w32(2, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_le() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0x2u64);
            blk = 33; continue;
        }
        33 => {
            c.w64(6, c.r64(3));
            let Some(t78) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090591e); };
            c.w32(3, t78);
            let r79 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r79);
            let Some(t80) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905924); };
            c.w32(3, t80);
            let r81 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r81);
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090592a); };
            c.w32(6, t82);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r83 = c.add(c.r64(2), 0x69u64, 64);
            c.w64(2, r83);
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::fault(0x700090593f);
        }
        35 => {
            let Some(t84) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000905940); };
            c.w32(3, t84);
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000905944); };
            c.w64(2, t85);
            let Some(t86) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090594b); };
            c.w64(6, t86);
            c.w64(8, c.r64(6).wrapping_add(0x18));
            let r87 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000905956); };
            c.w64(12, t88);
            let Some(t89) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700090595d); };
            c.cmp(t89, c.r64(8), 64);
            if c.cc_be() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x7000905967); }
            c.w64(2, c.r64(6).wrapping_add(0x1));
            blk = 37; continue;
        }
        37 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700090596e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x7000905971); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000905974); };
            c.w64(0, t90);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700090597b); }
            c.w64(0, c.r64(2).wrapping_add(0x10));
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000905982); };
            c.w64(3, t91);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000905989); }
            c.w64(3, c.r64(1));
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700090598f); }
            let Some(t92) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905992); };
            c.w32(0, t92);
            let r93 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r93);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x7000905998); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700090599b); }
            let Some(t94) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090599f); };
            c.w64(11, t94);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70009059a3); };
            c.w32(0, t95);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t96) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70009059b7); };
            c.w64(1, t96);
            let Some(t97) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009059bb); };
            c.w64(5, t97);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(2));
            let r98 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r98);
            return RegfileExit::jump(c.r64(1));
        }
        39 => {
            let r99 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r99);
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(2)) { return RegfileExit::fault(0x70009059cf); }
            let Some(t100) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70009059d7); };
            c.w32(7, t100);
            let Some(t101) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009059dc); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009059e0); };
            c.w64(5, t102);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(2));
            let r103 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r103);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            blk = 53; continue;
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70009059f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x70009059f9); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x7000905a05, 0x7000905a00);
        }
        42 => {
            c.w64(7, c.r64(0));
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905a08); };
            c.w64(15, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000905a0c); };
            c.w64(9, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000905a10); };
            c.w64(1, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000905a14); };
            c.w64(11, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000905a18); };
            c.w64(8, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000905a1c); };
            c.w64(6, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000905a20); };
            c.w64(12, t110);
            blk = 12; continue;
        }
        43 => {
            c.w64(0, c.r64(9));
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000905a30); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000905a38); };
            c.w64(3, t111);
            c.w64(6, c.r64(2));
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x7000905a4a, 0x7000905a45);
        }
        44 => {
            return RegfileExit::fault(0x7000905a4a);
        }
        45 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000905a4f); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000905a53); };
            c.w64(3, t112);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000905a63, 0x7000905a5e);
        }
        46 => {
            return RegfileExit::fault(0x7000905a63);
        }
        47 => {
            c.w64(0, c.r64(9));
            c.w32(3, c.r64(0).wrapping_add(c.r64(0)));
            let r113 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r113);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000905a6d); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000905a71); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000905a79); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000905a7b); };
            c.w64(3, t114);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000905a8e, 0x7000905a89);
        }
        48 => {
            blk = 27; continue;
        }
        49 => {
            c.w64(3, c.r64(9));
            c.w32(11, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000905a9a); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000905a9e); };
            c.w64(3, t115);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000905aae, 0x7000905aa9);
        }
        50 => {
            return RegfileExit::fault(0x7000905aae);
        }
        51 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000905ab4); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x7000905ab8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000905abc); }
            return c.call(0x70007febf4, 0x7000905ac5, 0x7000905ac0);
        }
        52 => {
            c.w64(2, c.r64(0));
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905ac8); };
            c.w64(15, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905acc); };
            c.w64(7, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000905ad0); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000905ad4); };
            c.w64(11, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000905ad8); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000905adc); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000905ae0); };
            c.w64(3, t122);
            blk = 37; continue;
        }
        53 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(6, c.r64(9).wrapping_add(0x4));
            let r123 = c.r64(2) & 0x1fu64;
            c.w64(2, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000905af5); };
            c.w32(8, t124);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000905afb); };
            c.w32(9, t125);
            let r126 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r126);
            let r127 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r127);
            let Some(t128) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000905b05); };
            c.w32(8, t128);
            let r129 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r129);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000905b0d); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000905b11); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000905b13); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000905b14); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000905b16); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000905b17); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x4u64);
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x2270)) else { return RegfileExit::fault(0x7000905b23); };
            c.w64(3, t130);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000905b32, 0x7000905b2d);
        }
        54 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905b32); };
            c.w64(7, t131);
            let Some(t132) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905b36); };
            c.w64(11, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000905b3a); };
            c.w64(9, t133);
            let Some(t134) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000905b3e); };
            c.w32(3, t134);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905b53); };
            c.w64(15, t135);
            let Some(t136) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905b57); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000905b5b); };
            c.w64(5, t137);
            c.w64(12, c.r64(11));
            let r138 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r138);
            return RegfileExit::jump(c.r64(1));
        }
        56 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x7000905b68); }
            let r139 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r139);
            let Some(t140) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000905b74); };
            c.w32(7, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905b79); };
            c.w64(15, t141);
            let Some(t142) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905b7d); };
            c.w64(1, t142);
            let Some(t143) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000905b81); };
            c.w64(5, t143);
            c.w64(12, c.r64(11));
            let r144 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r144);
            return RegfileExit::jump(c.r64(1));
        }
        57 => {
            c.w32(7, c.r32(7));
            let r145 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r145);
            c.w64(11, 0xfffffffffffc0000u64);
            let r146 = c.r64(11) & c.r64(7);
            c.w64(11, r146);
            c.logic(r146, 64);
            let Some(t147) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000905b9d); };
            c.logic(t147 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(3, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000905bac); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000905bad); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f13b4, 0x7000905bba, 0x7000905bb5);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000905bba); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000905bbc); }
            blk = 28; continue;
        }
        60 => {
            return RegfileExit::jump(0x7000905bc4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905bd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905bd8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000905bde); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000905be2); };
            c.w64(2, t2);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000905bea); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000905bee); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000905bf2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000905bf6); }
            c.w64(6, c.r64(2));
            return c.call(0x7000845874, 0x7000905c02, 0x7000905bfd);
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905c02); };
            c.w64(3, t3);
            let Some(t4) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905c06); };
            c.w64(3, t4);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000905c0a); };
            c.w64(2, t5);
            c.w64(9, c.r64(2).wrapping_add(0x1));
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000905c12); };
            c.w32(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905c17); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905c1b); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000905c1f); };
            c.w64(5, t9);
            c.w64(12, c.r64(3));
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000905c30);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905c34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905c37); };
            c.w64(6, t0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905c3b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905c41); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905c47); };
            c.w32(7, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000905c4a); };
            c.cmp(t6, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000905c53); };
            c.w64(6, t7);
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000905c5a); };
            c.w64(3, t8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000905c5e); };
            c.w32(7, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000905c64); };
            c.cmp(t10, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7).wrapping_mul(2).wrapping_add(0x1f));
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000905c7f); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905c90); };
            c.w32(6, t13);
            let r14 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r14);
            let r15 = c.r32(6) & 0x3fu64;
            c.w32(6, r15);
            c.logic(r15, 32);
            let Some(t16) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905c98); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let r18 = c.add(c.r64(6), 0x69u64, 64);
            c.w64(6, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905ca5); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905cab); };
            c.w32(8, t21);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(6), c.r64(8), 64);
            if c.cc_b() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x7000905cba);
        }
        6 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000905cbb); };
            c.w32(3, t22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000905cbf); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000905cc6); };
            c.w64(8, t24);
            c.w64(11, c.r64(8).wrapping_add(0x18));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000905ccd); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000905cd4); };
            c.cmp(t26, c.r64(11), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x7000905cdd); }
            c.w64(6, c.r64(8).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000905ce4); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000905ce9); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000905cec); };
            c.w64(3, t27);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000905cf3); }
            c.w64(8, c.r64(6).wrapping_add(0x10));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000905cfa); };
            c.w64(11, t28);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000905d01); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000905d05); }
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905d09); };
            c.w32(8, t29);
            let r30 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r30);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x7000905d11); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000905d15); }
            blk = 9; continue;
        }
        9 => {
            let r31 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r31);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000905d1c); };
            c.w32(3, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000905d21); };
            c.w64(1, t33);
            c.w64(0, c.r64(6));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905d2a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905d2e); }
            let r34 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x7000905d34); };
            c.w64(7, t35);
            let Some(t36) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000905d3b); };
            c.w64(11, t36);
            c.w64(0, c.r64(11).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x7000905d42); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000905d46); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000905d4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000905d4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000905d52); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000905d56); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x7000905d5a); };
            c.w64(2, t37);
            let Some(t38) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000905d61); };
            c.cmp(t38, c.r64(0), 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(2, 0x1cu64);
            return c.call(0x70007fec74, 0x7000905d70, 0x7000905d6b);
        }
        12 => {
            blk = 14; continue;
        }
        13 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000905d72); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 14; continue;
        }
        14 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x7000905d7c); };
            c.w64(3, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000905d83); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000905d88); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000905d8b); }
            if !c.st32(c.r64(0).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x7000905d8e); }
            if !c.st32(c.r64(0).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x7000905d95); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x7000905d9c); };
            c.w64(3, t40);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000905da3); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x7000905da6); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000905dad); };
            c.w64(6, t42);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x7000905db0); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r43 = c.r64(7) & c.r64(0);
            c.w64(7, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000905dc3); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x7000905dcd); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000905dd0); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000905dd4); };
            c.w64(8, t46);
            if !c.st32(c.r64(6).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000905dd8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r47 = c.r64(3) & c.r64(6);
            c.w64(3, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000905dea); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(7, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905df7); };
            c.w64(12, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000905dfb); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905dff); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000905e03); };
            c.w64(3, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000905e0a); }
            blk = 4; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905e10); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905e14); }
            let r53 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r53);
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x7000905e1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000905e23); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000905e27); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000905e2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000905e2f); }
            return c.call(0x70007febf4, 0x7000905e38, 0x7000905e33);
        }
        20 => {
            c.w64(6, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905e3b); };
            c.w64(12, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000905e3f); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905e43); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000905e47); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000905e4b); };
            c.w64(3, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000905e52); }
            blk = 8; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905e58); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905e5c); }
            let r59 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r59);
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905e62); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r61);
            let Some(t62) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905e68); };
            c.w32(3, t62);
            let r63 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r63);
            let Some(t64) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905e6e); };
            c.w32(6, t64);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x7000905e77); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000905e7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000905e7f); }
            c.cmp(c.r64(6), 0x69u64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x7000905e89);
        }
        23 => {
            let Some(t65) = c.ld32(c.r64(3).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000905e8a); };
            c.w32(3, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000905e90); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000905e97); };
            c.w64(7, t67);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000905e9e); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000905ea2); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000905ea9); };
            c.cmp(t69, c.r64(8), 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000905eb8, 0x7000905eb3);
        }
        25 => {
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x7000905eba); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(6));
            blk = 27; continue;
        }
        27 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000905ec4); };
            c.w64(3, t70);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000905ec8); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000905ecd); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000905ed0); };
            c.w64(3, t71);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000905ed7); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000905eda); }
            c.w64(6, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905ee0); };
            c.w64(12, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000905ee4); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000905ee8); };
            c.w64(9, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000905eef); }
            blk = 9; continue;
        }
        28 => {
            c.w32(0, c.r32(0));
            let r75 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r75);
            c.w64(3, 0xfffffffffffc0000u64);
            let r76 = c.r64(3) & c.r64(0);
            c.w64(3, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000905f04); };
            c.logic(t77 & 0x2u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(7, c.r64(6).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000905f12); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000905f13); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(6));
            return c.call(0x70007f0e54, 0x7000905f20, 0x7000905f1b);
        }
        30 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000905f20); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000905f22); }
            blk = 18; continue;
        }
        31 => {
            c.w32(6, c.r32(6));
            let r78 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r78);
            c.w64(7, 0xfffffffffffc0000u64);
            let r79 = c.r64(7) & c.r64(6);
            c.w64(7, r79);
            c.logic(r79, 64);
            let Some(t80) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000905f37); };
            c.logic(t80 & 0x2u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000905f45); }
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x7000905f4e, 0x7000905f49);
        }
        33 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000905f4e); }
            blk = 16; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000905f58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000905f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000905f74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000905f78); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000905f7e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000905f82); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000905f86); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905f8d); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000905f94); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000905f98); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000905f9c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000905fa0); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000905fa4); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000905fb2); };
            c.w64(0, t9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000905fb9); };
            c.w32(3, t10);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000905fc2); };
            c.w32(2, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000905fc8); };
            c.w64(11, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000905fcf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000905fd3); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000905fd7); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(2) & 0x20u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000905fed); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000905ff4); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906001); };
            c.w32(6, t16);
            let r17 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r17);
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000906007); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090600e); };
            c.w32(3, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000906012); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000906016); }
            c.logic(c.r32(3) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r32(3) & 0x200000u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090602e); };
            c.w32(11, t21);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(3, c.r64(11));
            blk = 11; continue;
        }
        10 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090603d); };
            c.w64(3, t22);
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000906044); };
            c.w64(11, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090604b); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 100; continue;
        }
        13 => {
            let Some(t25) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090605d); };
            c.w32(3, t25);
            let r26 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r26);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000906068); };
            c.w64(3, t27);
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090606f); };
            c.w32(0, t28);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x1000u64, 64);
            if c.cc_a() { blk = 99; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000906089); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000906090); };
            c.w64(7, t30);
            c.w64(1, c.r64(2).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(1).wrapping_add(c.r64(7)));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009060a0); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70009060a7); };
            c.cmp(t32, c.r64(8), 64);
            if c.cc_be() { blk = 78; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(12), c.r64(8)) { return RegfileExit::fault(0x70009060b0); }
            let r33 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r33);
            blk = 18; continue;
        }
        18 => {
            let r34 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70009060ba); };
            c.w64(8, t35);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70009060c1); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70009060c5); }
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009060c8); };
            c.w32(2, t36);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 80; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70009060dd); }
            let Some(t37) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009060e0); };
            c.w32(8, t37);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70009060f4); }
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009060f7); };
            c.w32(2, t38);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 21; continue;
        }
        21 => {
            let r39 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700090610c); }
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090610f); };
            c.w32(0, t40);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_be() { blk = 83; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x7000906127); }
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090612a); };
            c.w32(0, t41);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x4u64, 64);
            if c.cc_be() { blk = 84; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700090613f); }
            c.w64(0, c.r64(7).wrapping_add(c.r64(1)).wrapping_sub(0x1));
            c.w64(2, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(2), c.r32(11)) { return RegfileExit::fault(0x7000906150); }
            let r42 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r42);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 24; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t43) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090615c); };
            c.w32(11, t43);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let r44 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r44);
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000906175); };
            c.w32(0, t45);
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x7000906179); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(11, c.r64(7));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906186); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090618a); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090618e); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000906192); };
            c.w64(8, t49);
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906196); };
            c.w32(0, t50);
            let r51 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700090619c); }
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009061a0); };
            c.cmp(t52, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009061ad); };
            c.w32(3, t53);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009061b3); };
            c.w32(2, t54);
            c.w64(12, c.r64(3));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70009061c6); };
            c.cmp(t55, c.r32(2), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70009061d3); };
            c.cmp(t56, c.r32(2), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 87; continue; }
            blk = 33; continue;
        }
        33 => {
            let r57 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r57);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009061f3); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70009061fa); };
            c.w64(11, t59);
            c.w64(1, c.r64(3).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(11)));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000906205); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090620c); };
            c.cmp(t61, c.r64(6), 64);
            if c.cc_be() { blk = 85; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x7000906215); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700090621c); }
            c.w64(11, c.r64(7));
            let r62 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r62);
            c.logic(r62, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000906229); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700090622d); }
            let Some(t63) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000906231); };
            c.logic(t63 & 0x4u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(11, c.r64(3).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(3, 0x7u64);
            blk = 38; continue;
        }
        38 => {
            let Some(t64) = c.ld32(c.r64(0).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000906247); };
            c.w32(2, t64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x700090624a); }
            let r65 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r65);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 42; continue;
        }
        40 => {
            c.w64(2, c.r64(0).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r66 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r66);
            let r67 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r67);
            c.logic(r67, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700090626b); }
            c.w64(8, c.r64(3));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000906273); };
            c.w64(0, t68);
            c.w64(10, 0x7000906281u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000906281); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000906285); }
            return c.call(c.r64(0), 0x700090628b, 0x7000906289);
        }
        41 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700090628b); }
            let Some(t69) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000906293); };
            c.w64(4, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906298); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090629c); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009062a0); };
            c.w64(7, t72);
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(7));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009062a7); };
            c.w64(7, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009062ab); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009062af); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009062b3); };
            c.w64(11, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009062b7); };
            c.w64(8, t77);
            blk = 51; continue;
        }
        43 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 93; continue; }
            blk = 44; continue;
        }
        44 => {
            if let Some(sr) = c.shl(c.r64(3), 0x2u64, 64) { c.w64(3, sr); }
            c.w64(11, c.r64(3).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009062d5); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 89; continue; }
            blk = 45; continue;
        }
        45 => {
            let r78 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009062e9); };
            c.w64(3, t79);
            let Some(t80) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009062f0); };
            c.w64(1, t80);
            c.w64(6, c.r64(11).wrapping_add(c.r64(1)));
            let Some(t81) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009062f7); };
            c.cmp(t81, c.r64(6), 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000906301); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000906308); }
            c.w64(3, 0xfffffffffffc0000u64);
            let r82 = c.r64(3) & c.r64(11);
            c.w64(3, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000906316); };
            c.logic(t83 & 0x4u64, 8);
            if c.cc_ne() { blk = 107; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000906320); }
            c.w64(2, c.r64(0).wrapping_add(0x7));
            c.w64(1, c.r64(11).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r84 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r84);
            let r85 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r85);
            c.logic(r85, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000906337); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700090633c); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000906340); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000906344); };
            c.w64(0, t87);
            c.w64(10, 0x7000906352u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000906352); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000906356); }
            return c.call(c.r64(0), 0x700090635c, 0x700090635a);
        }
        49 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700090635c); }
            let Some(t88) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000906364); };
            c.w64(4, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000906369); };
            c.w64(0, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090636d); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906371); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000906375); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000906379); };
            c.w64(6, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090637d); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000906381); };
            c.w64(11, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000906385); };
            c.w64(8, t96);
            blk = 51; continue;
        }
        50 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090638b); };
            c.w64(0, t97);
            blk = 51; continue;
        }
        51 => {
            c.w64(3, c.r64(8));
            let Some(t98) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906395); };
            c.w32(2, t98);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090639d); };
            c.w64(8, t99);
            let Some(t100) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70009063a4); };
            c.w64(12, t100);
            c.w64(15, c.r64(2).wrapping_add(0x8));
            c.w64(6, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009063af); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70009063b6); };
            c.cmp(t102, c.r64(6), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x70009063bf); }
            c.w64(1, c.r64(12).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70009063c7); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70009063ca); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70009063ce); }
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70009063d5); };
            c.w64(0, t103);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70009063dc); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009063e0); };
            c.w64(0, t104);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70009063e4); }
            let Some(t105) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009063e8); };
            c.w32(11, t105);
            let r106 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r106);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70009063f0); }
            c.cmp(c.r64(2), 0xcu64, 64);
            if c.cc_e() { blk = 74; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009063fe); };
            c.w64(3, t107);
            c.w32(0, 0xcu64);
            c.w64(11, c.r64(0));
            blk = 55; continue;
        }
        55 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090640a); };
            c.w32(6, t108);
            let r109 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r109);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t110) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000906417); };
            c.w32(8, t110);
            let Some(t111) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090641b); };
            c.cmp(t111, c.r32(8), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st32(c.r64(1).wrapping_add(c.r64(0)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000906424); }
            let r112 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r112);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 74; continue;
        }
        59 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_e() { blk = 63; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(6, c.r64(0));
            blk = 61; continue;
        }
        61 => {
            let Some(t113) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090643e); };
            c.w32(8, t113);
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000906443); }
            let r114 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r114);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_ne() { blk = 61; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 64; continue;
        }
        63 => {
            c.w32(3, 0x1u64);
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000906463); };
            c.w64(3, t115);
            blk = 66; continue;
        }
        66 => {
            c.w64(6, c.r64(0).wrapping_sub(0x1));
            let Some(t116) = c.ld32(c.r64(1).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090646e); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090647c); };
            c.w32(12, t118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000906480); };
            c.cmp(t119, c.r32(12), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000906489); }
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090648f); };
            c.w64(8, t120);
            let Some(t121) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000906496); };
            c.w64(12, t121);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090649e); };
            c.w64(9, t122);
            let Some(t123) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70009064a5); };
            c.cmp(t123, c.r64(15), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x70009064ae); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70009064b6); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009064ba); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(8)) { return RegfileExit::fault(0x70009064c0); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r124 = c.r64(9) & c.r64(1);
            c.w64(9, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009064d4); };
            c.logic(t125 & 0x4u64, 8);
            if c.cc_ne() { blk = 104; continue; }
            blk = 72; continue;
        }
        72 => {
            let r126 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r126);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ne() { blk = 66; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009064e8); };
            c.w64(9, t127);
            blk = 74; continue;
        }
        74 => {
            let Some(t128) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009064ec); };
            c.w64(11, t128);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0x4));
            let Some(t129) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70009064f7); };
            c.w32(0, t129);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090650b); };
            c.w64(15, t130);
            let Some(t131) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090650f); };
            c.w64(7, t131);
            let Some(t132) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906513); };
            c.w64(5, t132);
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(7));
            c.w64(12, c.r64(11));
            let r133 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r133);
            return RegfileExit::jump(c.r64(1));
        }
        76 => {
            let r134 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r134);
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(1)) { return RegfileExit::fault(0x700090652a); }
            let Some(t135) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000906532); };
            c.w32(7, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906537); };
            c.w64(15, t136);
            let Some(t137) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090653b); };
            c.w64(7, t137);
            let Some(t138) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090653f); };
            c.w64(5, t138);
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(7));
            c.w64(12, c.r64(11));
            let r139 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r139);
            return RegfileExit::jump(c.r64(1));
        }
        77 => {
            blk = 100; continue;
        }
        78 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000906557); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090655b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700090655f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000906563); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x700090656f, 0x700090656a);
        }
        79 => {
            c.w64(7, c.r64(0));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906572); };
            c.w64(15, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000906576); };
            c.w64(6, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090657a); };
            c.w64(3, t142);
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090657e); };
            c.w64(11, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000906585); };
            c.w64(1, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000906589); };
            c.w64(0, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090658d); };
            c.w64(2, t146);
            blk = 18; continue;
        }
        80 => {
            return RegfileExit::fault(0x7000906596);
        }
        81 => {
            return RegfileExit::fault(0x7000906597);
        }
        82 => {
            return RegfileExit::fault(0x7000906598);
        }
        83 => {
            return RegfileExit::fault(0x7000906599);
        }
        84 => {
            return RegfileExit::fault(0x700090659a);
        }
        85 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700090659b); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x700090659f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009065a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70009065a7); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70009065b3, 0x70009065ae);
        }
        86 => {
            c.w64(7, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009065b6); };
            c.w64(15, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009065ba); };
            c.w64(9, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70009065be); };
            c.w64(0, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009065c2); };
            c.w64(2, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009065c6); };
            c.w64(3, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70009065ca); };
            c.w64(12, t152);
            blk = 35; continue;
        }
        87 => {
            c.w64(11, c.r64(9));
            let r153 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r153);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009065d9); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70009065e1); };
            c.w64(3, t154);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70009065f0, 0x70009065eb);
        }
        88 => {
            return RegfileExit::fault(0x70009065f0);
        }
        89 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009065f5); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70009065f9); };
            c.w64(3, t155);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000906609, 0x7000906604);
        }
        90 => {
            return RegfileExit::fault(0x7000906609);
        }
        91 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let r156 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r156);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000906611); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000906615); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000906619); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700090661a); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x700090661e); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x7000906622); }
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000906626); };
            c.w64(3, t157);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700090663a, 0x7000906635);
        }
        92 => {
            c.w64(11, c.r64(0));
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700090663d); };
            c.w64(0, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000906641); };
            c.w64(2, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000906645); };
            c.w64(12, t160);
            blk = 47; continue;
        }
        93 => {
            c.w64(11, c.r64(9));
            let r161 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r161);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000906654); }
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000906658); };
            c.w64(3, t162);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000906668, 0x7000906663);
        }
        94 => {
            return RegfileExit::fault(0x7000906668);
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000906669); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700090666d); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000906679, 0x7000906674);
        }
        96 => {
            c.w64(1, c.r64(0));
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090667c); };
            c.w64(7, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000906680); };
            c.w64(9, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000906684); };
            c.w64(2, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000906688); };
            c.w64(0, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090668c); };
            c.w64(11, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000906690); };
            c.w64(3, t168);
            blk = 53; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000906699); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090669d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009066a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70009066a5); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70009066a9); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x70009066b6, 0x70009066b1);
        }
        98 => {
            c.w64(8, c.r64(0));
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009066b9); };
            c.w64(7, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009066bd); };
            c.w64(1, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009066c1); };
            c.w64(2, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009066c5); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009066c9); };
            c.w64(6, t173);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70009066cd); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009066d2); };
            c.w64(3, t174);
            c.w32(11, 0xcu64);
            blk = 70; continue;
        }
        99 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009066e4); };
            c.w64(0, t175);
            blk = 100; continue;
        }
        100 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(3, c.r64(9).wrapping_add(0x4));
            let Some(t176) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009066f0); };
            c.w32(1, t176);
            let Some(t177) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009066f6); };
            c.w32(6, t177);
            let r178 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r178);
            let r179 = c.r64(2) & 0x1fu64;
            c.w64(2, r179);
            c.logic(r179, 64);
            let Some(t180) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906702); };
            c.w64(8, t180);
            let Some(t181) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906706); };
            c.w32(1, t181);
            let r182 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r182);
            let r183 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r183);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090670f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000906713); }
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000906714); };
            if !c.push64(t184) { return RegfileExit::fault(0x7000906714); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000906717); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000906718); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000906719); }
            c.w64(6, c.r64(8));
            c.w32(0, 0x4u64);
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x2280)) else { return RegfileExit::fault(0x7000906725); };
            c.w64(3, t185);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000906734, 0x700090672f);
        }
        101 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906734); };
            c.w64(7, t186);
            let Some(t187) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906738); };
            c.w64(11, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090673c); };
            c.w64(9, t188);
            let Some(t189) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000906740); };
            c.w32(3, t189);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906755); };
            c.w64(15, t190);
            let Some(t191) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906759); };
            c.w64(1, t191);
            let Some(t192) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090675d); };
            c.w64(5, t192);
            c.w64(12, c.r64(11));
            let r193 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r193);
            return RegfileExit::jump(c.r64(1));
        }
        103 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x700090676a); }
            let r194 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r194);
            let Some(t195) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000906776); };
            c.w32(7, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090677b); };
            c.w64(15, t196);
            let Some(t197) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090677f); };
            c.w64(1, t197);
            let Some(t198) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906783); };
            c.w64(5, t198);
            c.w64(12, c.r64(11));
            let r199 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r199);
            return RegfileExit::jump(c.r64(1));
        }
        104 => {
            c.w32(8, c.r32(8));
            let r200 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r200);
            c.w64(9, 0xfffffffffffc0000u64);
            let r201 = c.r64(9) & c.r64(8);
            c.w64(9, r201);
            c.logic(r201, 64);
            let Some(t202) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009067a0); };
            c.logic(t202 & 0x2u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009067af); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009067b0); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70009067bd, 0x70009067b8);
        }
        106 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009067bd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009067bf); }
            blk = 72; continue;
        }
        107 => {
            c.w32(2, c.r32(2));
            let r203 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r203);
            c.w64(3, 0xfffffffffffc0000u64);
            let r204 = c.r64(3) & c.r64(2);
            c.w64(3, r204);
            c.logic(r204, 64);
            let Some(t205) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009067d4); };
            c.logic(t205 & 0x2u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(1, c.r64(11).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009067e2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009067e3); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f13b4, 0x70009067f0, 0x70009067eb);
        }
        109 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009067f0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009067f2); }
            blk = 48; continue;
        }
        110 => {
            return RegfileExit::jump(0x70009067fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906817); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090681b); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000906821); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906827); };
            c.w32(6, t5);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x8cu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000906839);
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x237)) else { return RegfileExit::fault(0x700090683a); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000906843); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000906849); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000906850); };
            c.w64(7, t11);
            let Some(t12) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906853); };
            c.w32(8, t12);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000906864); };
            c.w64(11, t13);
            let Some(t14) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090686b); };
            c.cmp(t14, c.r64(8), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x7000906870); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000906877); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090687a); };
            c.w64(3, t15);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000906881); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000906884); }
            c.w64(3, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000906891); };
            c.w64(7, t16);
            blk = 6; continue;
        }
        6 => {
            let r17 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r17);
            if !c.st32(c.r64(3).wrapping_add(c.r64(6)), c.r32(7)) { return RegfileExit::fault(0x700090689c); }
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            let r18 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009068a9); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009068ae); };
            c.w64(1, t20);
            c.w64(0, c.r64(6));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009068b7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009068bb); }
            let r21 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x70009068c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009068c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70009068c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009068cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70009068d1); }
            return c.call(0x70007febf4, 0x70009068da, 0x70009068d5);
        }
        9 => {
            c.w64(6, c.r64(0));
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009068dd); };
            c.w64(12, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009068e1); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009068e5); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009068e9); };
            c.w64(2, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009068ed); };
            c.w64(3, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009068f4); }
            blk = 4; continue;
        }
        10 => {
            return RegfileExit::jump(0x70009068f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000906914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000906918); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090691e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906922); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000906926); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090692c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906933); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906937); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x700090693e); };
            c.w32(0, t9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906944); };
            c.w32(3, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090694a); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090694e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000906952); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000906956); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090695a); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000906963); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090696a); };
            c.w64(2, t14);
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            let r15 = c.r64(0) & 0x1fu64;
            c.w64(0, r15);
            c.logic(r15, 64);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r16 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r16);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090697e); }
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(8));
            return c.call(0x70008425f4, 0x7000906996, 0x7000906991);
        }
        3 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906996); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090699a); };
            c.w64(7, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090699e); };
            c.w64(8, t19);
            c.w64(9, c.r64(8).wrapping_add(0x4));
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009069a6); };
            c.w32(8, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009069ab); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009069af); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009069b3); };
            c.w64(5, t23);
            c.w64(12, c.r64(7));
            let r24 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r24);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70009069c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009069d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009069d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009069d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009069de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009069e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009069e6); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009069ea); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009069f1); };
            c.w32(0, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009069f7); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009069ff); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906a03); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000906a0a); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906a10); };
            c.w32(3, t13);
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000906a17); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000906a1e); };
            c.w32(11, t17);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000906a22); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000906a26); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000906a2a); }
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000906a2e); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000906a37); };
            c.w64(8, t19);
            blk = 2; continue;
        }
        2 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000906a42); }
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(8));
            return c.call(0x70008a8734, 0x7000906a51, 0x7000906a4c);
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000906a51); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906a55); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906a59); };
            c.w64(11, t22);
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000906a61); };
            c.w32(11, t23);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906a77); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906a7b); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906a7f); };
            c.w64(5, t26);
            c.w64(12, c.r64(8));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x7000906a8c); }
            let r28 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r28);
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000906a98); };
            c.w32(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906a9d); };
            c.w64(15, t30);
            let Some(t31) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906aa1); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906aa5); };
            c.w64(5, t32);
            c.w64(12, c.r64(8));
            let r33 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r33);
            return RegfileExit::jump(c.r64(1));
        }
        6 => {
            return RegfileExit::jump(0x7000906ab4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906ad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000906ad4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000906ad8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906ade); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906ae2); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000906ae6); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000906aec); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906af2); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906afa); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906afe); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906b04); };
            c.w32(3, t11);
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000906b0a); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906b10); };
            c.w32(8, t13);
            let r14 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000906b18); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000906b1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000906b23); }
            let Some(t17) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906b27); };
            c.cmp(t17, 0x81u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000906b30); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            blk = 2; continue;
        }
        2 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906b36); };
            c.w32(11, t20);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_ae() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906b49); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            let r23 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000906b54); }
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000906b5d); }
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(7));
            return c.call(0x70007f9f34, 0x7000906b6f, 0x7000906b6a);
        }
        5 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906b6f); };
            c.w64(2, t24);
            let Some(t25) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906b73); };
            c.w64(2, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906b77); };
            c.w64(9, t26);
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000906b7b); };
            c.w32(7, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000906b80); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906b84); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906b88); };
            c.w64(5, t30);
            c.w64(12, c.r64(2));
            let r31 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(c.r64(1));
        }
        6 => {
            return RegfileExit::fault(0x7000906b95);
        }
        7 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000906b9b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000906b9f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000906ba1); }
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x26d8)) else { return RegfileExit::fault(0x7000906baa); };
            c.w64(3, t32);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x7000906bb9, 0x7000906bb4);
        }
        9 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906bb9); };
            c.w64(2, t33);
            let Some(t34) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906bbd); };
            c.w64(2, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906bc1); };
            c.w64(9, t35);
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000906bc5); };
            c.w32(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000906bca); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906bce); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906bd2); };
            c.w64(5, t39);
            c.w64(12, c.r64(2));
            let r40 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r40);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000906bdf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000906be3); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000906be5); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x26e0)) else { return RegfileExit::fault(0x7000906be6); };
            c.w64(3, t41);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000906bfa, 0x7000906bf5);
        }
        11 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906bfa); };
            c.w64(2, t42);
            let Some(t43) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906bfe); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906c02); };
            c.w64(9, t44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000906c06); };
            c.w32(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000906c0b); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906c0f); };
            c.w64(1, t47);
            let Some(t48) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906c13); };
            c.w64(5, t48);
            c.w64(12, c.r64(2));
            let r49 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r49);
            return RegfileExit::jump(c.r64(1));
        }
        12 => {
            return RegfileExit::jump(0x7000906c24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906c34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000906c34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000906c38); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906c42); };
            c.w64(7, t1);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000906c46); }
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000906c4a); };
            c.w32(2, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906c50); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906c58); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906c5c); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000906c64); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000906c65); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2708)) else { return RegfileExit::fault(0x7000906c69); };
            c.w64(3, t8);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000906c7d, 0x7000906c78);
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906c7d); };
            c.w64(2, t9);
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000906c81); };
            c.w64(7, t10);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906c88); };
            c.w64(2, t11);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000906c93); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000906c98); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906c9f); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906ca3); };
            c.w64(5, t15);
            c.w64(12, c.r64(2));
            let r16 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r16);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000906cb4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906cd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000906cd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000906cd8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906ce2); };
            c.w64(7, t1);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000906ce6); }
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000906cea); };
            c.w64(2, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000906cf0); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906cf6); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906cfe); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906d02); };
            c.w64(0, t7);
            let Some(t8) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906d06); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000906d0e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000906d0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000906d11); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2710)) else { return RegfileExit::fault(0x7000906d15); };
            c.w64(3, t10);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x7000906d2c, 0x7000906d27);
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906d2c); };
            c.w64(2, t11);
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000906d30); };
            c.w64(7, t12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906d37); };
            c.w64(2, t13);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000906d42); };
            c.w32(7, t14);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000906d47); };
            c.w64(15, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906d4e); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906d52); };
            c.w64(5, t17);
            c.w64(12, c.r64(2));
            let r18 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000906d60);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906d74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000906d74); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906d7a); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w64(7, c.r64(5));
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000906d85); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906d8b); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906d8f); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000906d93); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000906d9a); };
            c.w64(11, t7);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x10));
            c.w64(0, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000906da9); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000906db0); };
            c.cmp(t9, c.r64(0), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000906db5); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000906dbc); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000906dc3); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906dca); };
            c.w32(0, t14);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x79u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x7000906dda);
        }
        4 => {
            let r15 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r15);
            let r16 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x7000906de2); };
            c.w32(11, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000906de9); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000906ded); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000906df1); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000906df4); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000906e02); };
            c.w64(6, t18);
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000906e09); }
            let r19 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r19);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            let r20 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000906e1a); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906e1f); };
            c.w64(1, t22);
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000906e28); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000906e2c); }
            let r23 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x7000906e32); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000906e36); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000906e3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000906e3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000906e42); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000906e46); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000906e4a); }
            return c.call(0x70007febf4, 0x7000906e53, 0x7000906e4e);
        }
        9 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906e56); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000906e5a); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906e5e); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000906e62); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000906e66); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000906e6a); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000906e6e); };
            c.w64(8, t30);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000906e75); }
            blk = 2; continue;
        }
        10 => {
            return RegfileExit::jump(0x7000906e7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906e94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000906e94); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906e9a); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w64(7, c.r64(5));
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000906ea5); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906eab); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906eaf); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000906eb3); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000906eba); };
            c.w64(11, t7);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x10));
            c.w64(0, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000906ec9); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000906ed0); };
            c.cmp(t9, c.r64(0), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000906ed5); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000906edc); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000906ee3); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000906eea); };
            c.w32(0, t14);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x7bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x7000906efa);
        }
        4 => {
            let r15 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r15);
            let r16 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x1f3)) else { return RegfileExit::fault(0x7000906f02); };
            c.w32(11, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000906f09); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000906f0d); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000906f11); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000906f14); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000906f22); };
            c.w64(6, t18);
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000906f29); }
            let r19 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r19);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            let r20 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000906f3a); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906f3f); };
            c.w64(1, t22);
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000906f48); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000906f4c); }
            let r23 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x7000906f52); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000906f56); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000906f5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000906f5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000906f62); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000906f66); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000906f6a); }
            return c.call(0x70007febf4, 0x7000906f73, 0x7000906f6e);
        }
        9 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000906f76); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000906f7a); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000906f7e); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000906f82); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000906f86); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000906f8a); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000906f8e); };
            c.w64(8, t30);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000906f95); }
            blk = 2; continue;
        }
        10 => {
            return RegfileExit::jump(0x7000906f9c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000906fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000906fb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000906fb8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000906fc2); };
            c.w64(7, t1);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000906fc6); }
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000906fca); };
            c.w64(2, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000906fd0); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906fd6); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000906fde); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000906fe2); };
            c.w64(0, t7);
            let Some(t8) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000906fe6); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000906fee); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000906fef); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000906ff1); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2718)) else { return RegfileExit::fault(0x7000906ff5); };
            c.w64(3, t10);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700090700c, 0x7000907007);
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090700c); };
            c.w64(2, t11);
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000907010); };
            c.w64(7, t12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907017); };
            c.w64(2, t13);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000907022); };
            c.w32(7, t14);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000907027); };
            c.w64(15, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090702e); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000907032); };
            c.w64(5, t17);
            c.w64(12, c.r64(2));
            let r18 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000907040);
        }
            _ => unreachable!(),
        }
    }
}
