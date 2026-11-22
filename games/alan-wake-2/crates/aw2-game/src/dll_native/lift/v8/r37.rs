#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700092ae54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ae57); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ae5b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092ae62); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092ae66); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092ae6d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092ae72); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ae77); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092ae7b); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ae84); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            c.w32(1, c.r32(0));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ae8f); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(11, c.r32(3));
            let r13 = c.sub(c.r32(1), c.r32(11), 32);
            c.w32(1, r13);
            if c.cc_o() { blk = 9; continue; }
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
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092aebb); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092aecd); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092aed2); }
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092aeda); };
            c.w64(7, t16);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092aee5); };
            c.w32(8, t17);
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092aeea); };
            c.w64(1, t18);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092aef6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092aefa); }
            let r19 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r19);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092af00); };
            c.w64(11, t20);
            let Some(t21) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092af07); };
            c.w64(1, t21);
            let r22 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r22);
            c.w64(12, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092af10); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092af14); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092af18); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092af1c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092af20); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092af24); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700092af28); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x700092af2c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092af30); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092af34); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700092af3b); };
            c.cmp(t24, c.r64(12), 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092af4d, 0x700092af48);
        }
        11 => {
            c.w64(11, c.r64(0));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092af50); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092af54); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092af58); };
            c.w64(7, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092af5c); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092af60); };
            c.w64(8, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092af64); };
            c.w64(2, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092af68); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092af6c); };
            c.w64(3, t32);
            blk = 13; continue;
        }
        12 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x700092af72); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092af75); };
            c.w64(11, t33);
            let r34 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r34);
            blk = 13; continue;
        }
        13 => {
            let r35 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r35);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092af93); };
            c.w64(0, t36);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700092af9a); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700092af9e); }
            c.w32(0, 0xeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092afac); }
            blk = 5; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092afb2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092afb6); }
            let r37 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r37);
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092afbc); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092afc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092afc6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092afca); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092afce); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700092afd2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092afd6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092afda); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700092afde); }
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092afe2); };
            c.cmp(t40, c.r32(1), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 15; continue;
        }
        15 => {
            let r41 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r41);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092aff1); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700092affe); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x700092b003); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b00d); };
            c.w32(11, t42);
            c.w32(0, c.r32(11));
            let r43 = c.r32(0) | 0xeu64;
            c.w32(0, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092b020); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b025); }
            blk = 18; continue;
        }
        18 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092b02d); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092b034); };
            c.w64(11, t45);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092b03b); };
            c.w64(3, t46);
            let Some(t47) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700092b042); };
            c.cmp(t47, c.r64(0), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092b051, 0x700092b04c);
        }
        20 => {
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700092b051); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700092b056); }
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700092b05d); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 22; continue;
        }
        22 => {
            c.arith_sd(1, 0, c.xmm_u64(1));
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b06b); };
            c.w64(7, t48);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700092b072); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092b075); }
            c.w64(11, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b07d); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b081); };
            c.w64(9, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b085); };
            c.w64(7, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b08c); }
            blk = 8; continue;
        }
        23 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r52 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r52);
            let Some(t53) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b099); };
            c.w32(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700092b09d); }
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 31; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b0b6); };
            c.w32(1, t54);
            let r55 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r55);
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b0bc); };
            c.cmp(t56, c.r32(1), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(1, 0x1eu64);
            blk = 45; continue;
        }
        28 => {
            let Some(t57) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b0cf); };
            c.cmp(t57, 0x43u64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(1, 0x1eu64);
            blk = 45; continue;
        }
        30 => {
            c.w32(1, 0x1eu64);
            blk = 45; continue;
        }
        31 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t58) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b0f7); };
            c.w32(1, t58);
            let r59 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r59);
            let Some(t60) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b0fd); };
            c.cmp(t60, 0x41u64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b10d); };
            c.w32(1, t61);
            c.w32(0, c.r32(1));
            let r62 = c.r32(0) | 0x40u64;
            c.w32(0, r62);
            c.logic(r62, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092b11e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b123); }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092b12b); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092b12f); };
            c.w64(0, t63);
            c.w64(6, c.r64(11));
            return c.call(0x700089bcd4, 0x700092b13b, 0x700092b136);
        }
        37 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(11, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b142); };
            c.w64(15, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b146); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b14a); };
            c.w64(7, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b151); }
            blk = 8; continue;
        }
        39 => {
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092b157); };
            c.cmp(t67, 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092b15d); };
            c.w64(8, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092b161); };
            c.w64(2, t69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b165); };
            c.w32(7, t70);
            c.w32(11, c.r32(7));
            let r71 = c.r32(11) | 0xfeu64;
            c.w32(11, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700092b17c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b181); }
            blk = 42; continue;
        }
        42 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b189); };
            c.w64(7, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092b18d); };
            c.w64(11, t73);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092b191); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092b19a); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700092b19b); };
            c.w64(3, t74);
            c.w32(0, 0x1u64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092b1a7); };
            c.w64(6, t75);
            return c.call(0x700086ed74, 0x700092b1b0, 0x700092b1ab);
        }
        43 => {
            return RegfileExit::fault(0x700092b1b0);
        }
        44 => {
            c.w32(1, 0xfeu64);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b1bb); };
            c.w32(6, t76);
            c.w32(1, c.r32(1));
            c.w32(11, c.r32(6));
            let r77 = c.r32(1) | c.r32(11);
            c.w32(1, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092b1cf); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b1d4); }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092b1dc); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092b1e0); };
            c.w64(6, t78);
            return c.call(0x70008adb14, 0x700092b1e9, 0x700092b1e4);
        }
        49 => {
            c.w64(11, c.r64(0));
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b1ec); };
            c.w64(15, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b1f0); };
            c.w64(9, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b1f4); };
            c.w64(7, t81);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b1fb); }
            blk = 8; continue;
        }
        50 => {
            return RegfileExit::jump(0x700092b204);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092b214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b217); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092b21b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092b222); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092b226); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092b22d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092b232); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b237); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092b23b); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092b244); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092b24d); };
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
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700092b29b); };
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
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b2b9); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092b2cb); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b2d0); }
            blk = 16; continue;
        }
        16 => {
            let Some(t22) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b2d8); };
            c.w64(7, t22);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092b2e3); };
            c.w32(8, t23);
            let Some(t24) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092b2e8); };
            c.w64(1, t24);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b2f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b2f8); }
            let r25 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r25);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092b2fe); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092b305); };
            c.w64(0, t27);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092b30c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092b310); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092b314); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700092b318); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700092b31c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700092b320); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700092b324); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092b328); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092b32f); };
            c.cmp(t29, c.r64(3), 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092b342, 0x700092b33d);
        }
        19 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092b342); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b346); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b34a); };
            c.w64(9, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b34e); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092b352); };
            c.w64(8, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092b356); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092b35a); };
            c.w64(1, t36);
            blk = 21; continue;
        }
        20 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700092b360); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(6) as u32 as i32 as i64));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b372); };
            c.w64(11, t37);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092b379); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092b37d); }
            c.w64(11, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b388); }
            blk = 10; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b38e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b392); }
            let r38 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092b398); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700092b39f); };
            c.w64(6, t40);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092b3a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092b3aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092b3ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700092b3b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700092b3b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700092b3ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092b3be); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700092b3c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700092b3c6); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092b3ca); };
            c.w64(6, t41);
            let Some(t42) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700092b3d1); };
            c.cmp(t42, c.r64(12), 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(2, 0xcu64);
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x700092b3e3, 0x700092b3de);
        }
        24 => {
            c.w64(3, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b3e6); };
            c.w64(15, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b3ea); };
            c.w64(9, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b3ee); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092b3f2); };
            c.w64(8, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092b3f6); };
            c.w64(2, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092b3fa); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092b3fe); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092b402); };
            c.w64(11, t50);
            blk = 26; continue;
        }
        25 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x700092b408); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092b40b); };
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
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b428); };
            c.w64(11, t53);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092b42f); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092b433); }
            c.w64(11, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b43e); }
            blk = 10; continue;
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b444); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b448); }
            let r54 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b44e); };
            c.w32(6, t55);
            let r56 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r56);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092b454); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092b458); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092b45c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092b460); }
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b464); };
            c.cmp(t57, c.r32(6), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            let r58 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r58);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092b473); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x700092b480); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700092b485); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t59) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b48f); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092b4a2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b4a7); }
            blk = 31; continue;
        }
        31 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092b4af); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092b4b6); };
            c.w64(11, t62);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092b4bd); };
            c.w64(3, t63);
            let Some(t64) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700092b4c4); };
            c.cmp(t64, c.r64(0), 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092b4d3, 0x700092b4ce);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700092b4d3); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700092b4d8); }
            blk = 35; continue;
        }
        34 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700092b4df); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 35; continue;
        }
        35 => {
            c.arith_sd(2, 1, c.xmm_u64(0));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b4ed); };
            c.w64(7, t65);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700092b4f4); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700092b4f7); }
            c.w64(11, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b4ff); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b503); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b507); };
            c.w64(7, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b50e); }
            blk = 16; continue;
        }
        36 => {
            let Some(t69) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b514); };
            c.w32(6, t69);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r70 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r70);
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
            let Some(t71) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b534); };
            c.w32(6, t71);
            let r72 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b53a); };
            c.cmp(t73, c.r32(6), 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(6, 0x1eu64);
            blk = 52; continue;
        }
        41 => {
            let Some(t74) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b54d); };
            c.cmp(t74, 0x43u64, 16);
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
            let Some(t75) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b571); };
            c.w32(6, t75);
            let r76 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r76);
            let Some(t77) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b577); };
            c.cmp(t77, 0x41u64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b583); };
            c.w32(1, t78);
            c.w32(6, c.r32(1));
            let r79 = c.r32(6) | 0x40u64;
            c.w32(6, r79);
            c.logic(r79, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700092b594); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b599); }
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092b5a1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092b5a5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092b5a6); }
            c.w32(8, 0x4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092b5ad); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092b5af); };
            c.w64(3, t80);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700092b5c3, 0x700092b5be);
        }
        50 => {
            c.w64(11, c.r64(0));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b5c6); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b5ca); };
            c.w64(9, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b5ce); };
            c.w64(7, t83);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b5d5); }
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
            let Some(t84) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b5e5); };
            c.w32(1, t84);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r85 = c.r32(6) | c.r32(11);
            c.w32(6, r85);
            c.logic(r85, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092b5f9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b5fe); }
            blk = 55; continue;
        }
        55 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092b606); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092b60a); };
            c.w64(6, t86);
            return c.call(0x70008adcd4, 0x700092b613, 0x700092b60e);
        }
        56 => {
            c.w64(11, c.r64(0));
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b616); };
            c.w64(15, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b61a); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b61e); };
            c.w64(7, t89);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b625); }
            blk = 16; continue;
        }
        57 => {
            return RegfileExit::jump(0x700092b62c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092b634(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b637); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092b63b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092b642); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092b646); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092b64d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092b652); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b657); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092b65b); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092b664); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092b66d); };
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
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b6d7); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092b6e9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b6ee); }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b6f6); };
            c.w64(7, t18);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092b701); };
            c.w32(8, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092b706); };
            c.w64(1, t20);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b712); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b716); }
            let r21 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r21);
            blk = 18; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b71e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b722); }
            let r22 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r22);
            blk = 18; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b72a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b72e); }
            let r23 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r23);
            blk = 18; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b736); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b73a); }
            let r24 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r24);
            blk = 18; continue;
        }
        18 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092b740); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092b747); };
            c.w64(3, t26);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092b74e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092b752); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092b756); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700092b75a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092b75e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700092b762); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092b766); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700092b76a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x700092b76e); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092b772); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700092b779); };
            c.cmp(t28, c.r64(2), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092b788, 0x700092b783);
        }
        20 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092b788); };
            c.w64(11, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b78c); };
            c.w64(15, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b790); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b794); };
            c.w64(7, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092b798); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092b79c); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092b7a0); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092b7a4); };
            c.w64(12, t36);
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700092b7aa); }
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092b7ad); };
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
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b7ce); };
            c.w64(11, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092b7d5); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700092b7d9); }
            c.w64(11, c.r64(0));
            c.w32(0, 0x6u64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b7e9); }
            blk = 10; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092b7ef); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092b7f3); }
            let r40 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b7f9); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092b7ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092b803); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092b807); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092b80b); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b80f); };
            c.cmp(t43, c.r32(2), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            let r44 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r44);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092b81e); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700092b82b); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x700092b830); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t45) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b83a); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092b84d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b852); }
            blk = 27; continue;
        }
        27 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092b85a); };
            c.w64(8, t47);
            let Some(t48) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092b861); };
            c.w64(11, t48);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092b868); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700092b86f); };
            c.cmp(t50, c.r64(0), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092b87e, 0x700092b879);
        }
        29 => {
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700092b87e); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700092b883); }
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700092b88a); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 31; continue;
        }
        31 => {
            c.arith_sd(3, 0, c.xmm_u64(1));
            c.mov128_rr(0, 0);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b89b); };
            c.w64(7, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700092b8a2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092b8a5); }
            c.w64(11, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b8ad); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b8b1); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b8b5); };
            c.w64(7, t54);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b8bc); }
            blk = 13; continue;
        }
        32 => {
            let Some(t55) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b8c2); };
            c.w32(2, t55);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r56 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r56);
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
            let Some(t57) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b8e2); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092b8e8); };
            c.cmp(t59, c.r32(2), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0x1eu64);
            blk = 48; continue;
        }
        37 => {
            let Some(t60) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b8fb); };
            c.cmp(t60, 0x43u64, 16);
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
            let Some(t61) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092b91f); };
            c.w32(2, t61);
            let r62 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r62);
            let Some(t63) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092b925); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b931); };
            c.w32(2, t64);
            c.w32(6, c.r32(2));
            let r65 = c.r32(6) | 0x40u64;
            c.w32(6, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700092b942); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b947); }
            blk = 45; continue;
        }
        45 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092b94f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092b953); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092b954); }
            c.w32(8, 0x6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092b95b); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092b95d); };
            c.w64(3, t66);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700092b971, 0x700092b96c);
        }
        46 => {
            c.w64(11, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b974); };
            c.w64(15, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b978); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b97c); };
            c.w64(7, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b983); }
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
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092b993); };
            c.w32(6, t70);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r71 = c.r32(2) | c.r32(11);
            c.w32(2, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092b9a7); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092b9ac); }
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092b9b4); }
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092b9b8); };
            c.w64(6, t72);
            return c.call(0x70008adf94, 0x700092b9c1, 0x700092b9bc);
        }
        52 => {
            c.w64(11, c.r64(0));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b9c4); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092b9c8); };
            c.w64(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092b9cc); };
            c.w64(7, t75);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092b9d3); }
            blk = 13; continue;
        }
        53 => {
            return RegfileExit::jump(0x700092b9dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092b9f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092b9f7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092b9fb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092ba02); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092ba06); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092ba0d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092ba12); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ba17); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092ba1b); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ba24); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ba2d); };
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
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092ba74); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092ba86); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ba8b); }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ba93); };
            c.w64(7, t17);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092ba9e); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092baa3); };
            c.w64(1, t19);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092baaf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092bab3); }
            let r20 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r20);
            c.cdq();
            c.idiv(c.r32(3), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092babc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092bac0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092bac4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700092bac8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092bacc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700092bad0); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092bad4); }
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
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092baf1); };
            c.w64(11, t22);
            let Some(t23) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092baf8); };
            c.w64(0, t23);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092baff); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092bb06); };
            c.cmp(t25, c.r64(3), 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092bb19, 0x700092bb14);
        }
        18 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092bb19); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092bb1d); };
            c.w64(15, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092bb21); };
            c.w64(9, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092bb25); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092bb29); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092bb2d); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092bb31); };
            c.w64(6, t32);
            blk = 20; continue;
        }
        19 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700092bb37); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 20; continue;
        }
        20 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092bb49); };
            c.w64(11, t33);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092bb50); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092bb54); }
            blk = 21; continue;
        }
        21 => {
            c.w64(11, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092bb5f); }
            blk = 6; continue;
        }
        22 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700092bb65); };
            c.w64(11, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092bb6f); }
            blk = 6; continue;
        }
        23 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700092bb75); };
            c.w64(11, t35);
            blk = 6; continue;
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092bb81); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092bb85); }
            let r36 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092bb8b); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092bb91); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092bb95); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092bb99); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092bb9d); }
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092bba1); };
            c.cmp(t39, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            let r40 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r40);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092bbb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092bbb5); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700092bbb9); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092bbc3); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092bbd6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092bbdb); }
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092bbe3); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092bbea); };
            c.w64(11, t44);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092bbf1); };
            c.w64(2, t45);
            let Some(t46) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700092bbf8); };
            c.cmp(t46, c.r64(0), 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092bc07, 0x700092bc02);
        }
        30 => {
            c.w64(8, c.r64(0));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092bc0a); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092bc0e); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092bc12); };
            c.w64(7, t49);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700092bc16); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092bc1b); };
            c.w64(3, t50);
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700092bc21); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 32; continue;
        }
        32 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            let r51 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r51);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x700092bc34); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700092bc39); }
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092bc3c); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700092bc41); }
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
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092bc50); }
            if !c.popfq() { return RegfileExit::fault(0x700092bc51); }
            if c.cc_p() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x700092bc56); }
            if !c.movsd_rm(0, c.r64(4)) { return RegfileExit::fault(0x700092bc59); }
            let r54 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092bc62); };
            c.w64(11, t55);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092bc69); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092bc6d); }
            c.w64(11, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092bc79); }
            blk = 12; continue;
        }
        35 => {
            let Some(t56) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092bc7f); };
            c.w32(2, t56);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r57 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r57);
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
            let Some(t58) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092bc9f); };
            c.w32(2, t58);
            let r59 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r59);
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092bca5); };
            c.cmp(t60, c.r32(2), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(2, 0x1eu64);
            blk = 51; continue;
        }
        40 => {
            let Some(t61) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092bcb8); };
            c.cmp(t61, 0x43u64, 16);
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
            let Some(t62) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092bcdc); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            let Some(t64) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092bce2); };
            c.cmp(t64, 0x41u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092bcee); };
            c.w32(2, t65);
            c.w32(6, c.r32(2));
            let r66 = c.r32(6) | 0x40u64;
            c.w32(6, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700092bcff); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092bd04); }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092bd0c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092bd10); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092bd11); }
            c.w32(8, 0x8u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092bd18); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092bd1a); };
            c.w64(3, t67);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700092bd2e, 0x700092bd29);
        }
        49 => {
            c.w64(11, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092bd31); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092bd35); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092bd39); };
            c.w64(7, t70);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092bd40); }
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
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092bd50); };
            c.w32(6, t71);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r72 = c.r32(2) | c.r32(11);
            c.w32(2, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092bd64); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092bd69); }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092bd71); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092bd75); };
            c.w64(6, t73);
            return c.call(0x70008ae1b4, 0x700092bd7e, 0x700092bd79);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092bd81); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092bd85); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092bd89); };
            c.w64(7, t76);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092bd90); }
            blk = 12; continue;
        }
        56 => {
            return RegfileExit::jump(0x700092bd98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092bdb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092bdb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092bdb8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092bdbe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092bdc2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092bdc6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092bdcd); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092bdd1); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092bdd8); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092bddd); };
            c.w32(2, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092bde2); };
            c.w32(1, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092bde6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700092bdea); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092bdee); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092bdf2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092bdf6); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092bdfa); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092be03); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            let r13 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092be0c); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092be19); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700092be1d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092be29); };
            c.w64(11, t15);
            let Some(t16) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092be30); };
            c.w64(6, t16);
            let r17 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092be39); };
            c.w64(0, t18);
            let Some(t19) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092be40); };
            c.cmp(t19, c.r64(12), 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x700092be49); }
            c.w64(11, c.r64(6).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092be50); };
            c.w64(0, t20);
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092be66); };
            c.w64(0, t22);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700092be6d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092be71); }
            c.w64(10, c.r64(4));
            let r23 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r23);
            let r24 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r24);
            c.logic(r24, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700092be80); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x700092be85); };
            c.w64(0, t25);
            c.w64(10, 0x700092be93u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700092be93); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700092be97); }
            return c.call(c.r64(0), 0x700092be9d, 0x700092be9b);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700092be9d); }
            let Some(t26) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700092bea5); };
            c.w64(4, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092beaa); };
            c.w64(0, t27);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092beae); }
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092beb3); };
            c.cmp(t28, 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092beb9); };
            c.w64(8, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092bebd); };
            c.w64(7, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092bec1); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092bed4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092bed9); }
            blk = 9; continue;
        }
        9 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092bee1); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092bee5); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092bee9); };
            c.w64(8, t35);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092bef1); };
            c.w32(8, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092bef6); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092befa); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092befe); };
            c.w64(5, t39);
            c.w64(12, c.r64(7));
            let r40 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r40);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x700092bf10); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092bf1c, 0x700092bf17);
        }
        11 => {
            c.w64(11, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092bf1f); };
            c.w64(3, t41);
            blk = 5; continue;
        }
        12 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092bf28); };
            c.w32(6, t42);
            let r43 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092bf2e); }
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092bf32); };
            c.cmp(t44, c.r32(6), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 13; continue;
        }
        13 => {
            let r45 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r45);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092bf41); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x700092bf4e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x700092bf53); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092bf5d); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092bf70); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092bf75); }
            blk = 16; continue;
        }
        16 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092bf7d); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092bf84); };
            c.w64(11, t49);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092bf8b); };
            c.w64(3, t50);
            let Some(t51) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700092bf92); };
            c.cmp(t51, c.r64(0), 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092bfa1, 0x700092bf9c);
        }
        18 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x700092bfa1); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700092bfa6); }
            blk = 20; continue;
        }
        19 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700092bfad); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        20 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092bfb7); };
            c.w64(7, t52);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700092bfbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092bfc1); }
            c.w64(10, c.r64(4));
            let r53 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r53);
            let r54 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r54);
            c.logic(r54, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700092bfd0); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x700092bfd5); };
            c.w64(0, t55);
            c.w64(10, 0x700092bfe3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700092bfe3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700092bfe7); }
            return c.call(c.r64(0), 0x700092bfed, 0x700092bfeb);
        }
        21 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700092bfed); }
            let Some(t56) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700092bff5); };
            c.w64(4, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092bffa); };
            c.w64(0, t57);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092bffe); }
            blk = 9; continue;
        }
        22 => {
            let Some(t58) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092c008); };
            c.w32(6, t58);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r59 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r59);
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
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c028); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c02e); };
            c.cmp(t62, c.r32(6), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(6, 0x1eu64);
            blk = 38; continue;
        }
        27 => {
            let Some(t63) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092c041); };
            c.cmp(t63, 0x43u64, 16);
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
            let Some(t64) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c05f); };
            c.w32(6, t64);
            let r65 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r65);
            let Some(t66) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092c065); };
            c.cmp(t66, 0x41u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c071); };
            c.w32(1, t67);
            c.w32(6, c.r32(1));
            let r68 = c.r32(6) | 0x40u64;
            c.w32(6, r68);
            c.logic(r68, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700092c082); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c087); }
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092c08f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092c093); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092c094); }
            c.w32(8, 0xau64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092c09b); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092c09d); };
            c.w64(3, t69);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700092c0b1, 0x700092c0ac);
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
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c0c0); };
            c.w32(1, t70);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r71 = c.r32(6) | c.r32(11);
            c.w32(6, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092c0d4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c0d9); }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092c0e1); }
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c0e5); };
            c.w64(6, t72);
            return c.call(0x70008ae454, 0x700092c0ee, 0x700092c0e9);
        }
        42 => {
            blk = 9; continue;
        }
        43 => {
            return RegfileExit::jump(0x700092c0f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092c114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092c114); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092c118); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c11e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c122); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092c126); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092c12d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092c134); };
            c.w32(11, t7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092c139); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c13e); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092c142); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092c146); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092c14a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092c14e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092c152); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092c156); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c15f); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092c166); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c16a); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092c177); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092c17b); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092c181); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c189); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c190); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092c19d); };
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
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092c1b7); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x700092c1c3, 0x700092c1be);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092c1c3); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c1cc); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092c1d0); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c1d4); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c1d8); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092c1dc); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092c1e0); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092c1e6); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c209); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c211); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c224); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c229); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092c231); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092c23a); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700092c23b); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700092c24f, 0x700092c24a);
        }
        15 => {
            return RegfileExit::fault(0x700092c24f);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092c250); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            let r39 = c.r32(11) | c.r32(0);
            c.w32(11, r39);
            c.logic(r39, 32);
            c.w64(0, c.r64(11));
            let r40 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r40);
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
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c294); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c29a); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c2b0); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c2b5); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c2bd); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092c2c5); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092c2ca); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c2ce); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092c2de); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092c2e5); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092c2ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092c2f0); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092c2f4); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092c2fb); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700092c30e, 0x700092c309);
        }
        29 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092c30e); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c312); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092c316); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c31a); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c31e); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092c322); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092c326); };
            c.w64(11, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x700092c32c); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c333); };
            c.w64(1, t62);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700092c33a); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092c346); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092c357); }
            return c.call(0x70008708b4, 0x700092c361, 0x700092c35c);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092c361); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x700092c370);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092c374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092c374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092c378); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c37e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c382); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092c386); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092c38d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092c394); };
            c.w32(11, t7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092c399); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c39e); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092c3a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092c3a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092c3aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092c3ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092c3b2); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092c3b6); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c3bf); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092c3c6); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c3ca); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092c3d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092c3db); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092c3e1); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c3e9); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c3f0); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092c3fd); };
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
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092c417); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x700092c423, 0x700092c41e);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092c423); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c42c); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092c430); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c434); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c438); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092c43c); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092c440); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092c446); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c469); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c471); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c484); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c489); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092c491); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092c49a); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700092c49b); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700092c4af, 0x700092c4aa);
        }
        15 => {
            return RegfileExit::fault(0x700092c4af);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092c4b0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            let r39 = c.r32(11) ^ c.r32(0);
            c.w32(11, r39);
            c.logic(r39, 32);
            c.w64(0, c.r64(11));
            let r40 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r40);
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
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c4f4); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c4fa); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c510); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c515); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c51d); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092c525); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092c52a); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c52e); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092c53e); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092c545); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092c54c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092c550); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092c554); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092c55b); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700092c56e, 0x700092c569);
        }
        29 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092c56e); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c572); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092c576); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c57a); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c57e); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092c582); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092c586); };
            c.w64(11, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x700092c58c); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c593); };
            c.w64(1, t62);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700092c59a); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092c5a6); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092c5b7); }
            return c.call(0x70008708b4, 0x700092c5c1, 0x700092c5bc);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092c5c1); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x700092c5d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092c5d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092c5d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092c5d8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c5de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c5e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092c5e6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092c5ed); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092c5f4); };
            c.w32(11, t7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092c5f9); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c5fe); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092c602); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092c606); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092c60a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092c60e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092c612); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092c616); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c61f); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092c626); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c62a); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092c637); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092c63b); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092c641); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c649); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c650); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092c65d); };
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
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092c677); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x700092c683, 0x700092c67e);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092c683); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c68c); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092c690); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c694); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c698); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092c69c); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092c6a0); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092c6a6); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c6c9); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c6d1); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c6e4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c6e9); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092c6f1); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092c6fa); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700092c6fb); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700092c70f, 0x700092c70a);
        }
        15 => {
            return RegfileExit::fault(0x700092c70f);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092c710); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            let r39 = c.r32(11) & c.r32(0);
            c.w32(11, r39);
            c.logic(r39, 32);
            c.w64(0, c.r64(11));
            let r40 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r40);
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
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c754); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c75a); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c770); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c775); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c77d); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092c785); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092c78a); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c78e); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092c79e); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092c7a5); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092c7ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092c7b0); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092c7b4); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092c7bb); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700092c7ce, 0x700092c7c9);
        }
        29 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092c7ce); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c7d2); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092c7d6); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c7da); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c7de); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092c7e2); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092c7e6); };
            c.w64(11, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x700092c7ec); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c7f3); };
            c.w64(1, t62);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700092c7fa); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092c806); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092c817); }
            return c.call(0x70008708b4, 0x700092c821, 0x700092c81c);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092c821); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x700092c830);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092c834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092c834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092c838); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c83e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c842); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092c846); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092c84d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092c854); };
            c.w32(11, t7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092c859); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c85e); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092c862); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092c866); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092c86a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092c86e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092c872); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092c876); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c87f); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092c886); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092c88a); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092c897); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092c89b); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092c8a1); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092c8a9); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092c8b0); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092c8bd); };
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
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092c8d7); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x700092c8e3, 0x700092c8de);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092c8e3); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092c8ec); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092c8f0); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c8f4); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092c8f8); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092c8fc); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092c900); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092c906); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c926); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c92e); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c941); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c946); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092c94e); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092c957); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700092c958); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700092c96c, 0x700092c967);
        }
        15 => {
            return RegfileExit::fault(0x700092c96c);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092c96d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            c.w64(1, c.r64(11));
            let r39 = c.shl(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            c.w64(11, c.r64(0));
            let r40 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r40);
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
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092c9b3); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092c9b9); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092c9cf); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092c9d4); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092c9dc); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092c9e4); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092c9e9); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092c9ed); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092c9fd); };
            c.w64(11, t51);
            let Some(t52) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092ca04); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092ca0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700092ca0f); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092ca13); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092ca1a); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092ca2d, 0x700092ca28);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ca30); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092ca34); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ca38); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092ca3c); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092ca40); };
            c.w64(2, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092ca44); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092ca48); };
            c.w64(3, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x700092ca4e); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092ca55); };
            c.w64(1, t62);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700092ca5c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092ca68); }
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092ca77); }
            return c.call(0x70008708b4, 0x700092ca81, 0x700092ca7c);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092ca81); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x700092ca90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092ca94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092ca94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092ca98); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092ca9e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092caa2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092caa6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092caad); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092cab4); };
            c.w32(11, t7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092cab9); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092cabe); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092cac2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092cac6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092caca); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092cace); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092cad2); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092cad6); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092cadf); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092cae6); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092caea); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092caf7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092cafb); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092cb01); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092cb09); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092cb10); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092cb1d); };
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
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092cb37); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x700092cb43, 0x700092cb3e);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092cb43); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092cb4c); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092cb50); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092cb54); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092cb58); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092cb5c); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092cb60); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092cb66); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092cb86); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092cb8e); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092cba1); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092cba6); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092cbae); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092cbb7); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700092cbb8); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700092cbcc, 0x700092cbc7);
        }
        15 => {
            return RegfileExit::fault(0x700092cbcc);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092cbcd); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            c.w64(1, c.r64(11));
            let r39 = c.sar(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            c.w64(11, c.r64(0));
            let r40 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r40);
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
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092cc13); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092cc19); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092cc2f); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092cc34); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092cc3c); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092cc44); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092cc49); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092cc4d); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092cc5d); };
            c.w64(11, t51);
            let Some(t52) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092cc64); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092cc6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700092cc6f); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092cc73); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092cc7a); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092cc8d, 0x700092cc88);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092cc90); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092cc94); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092cc98); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092cc9c); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092cca0); };
            c.w64(2, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092cca4); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092cca8); };
            c.w64(3, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x700092ccae); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092ccb5); };
            c.w64(1, t62);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700092ccbc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092ccc8); }
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092ccd7); }
            return c.call(0x70008708b4, 0x700092cce1, 0x700092ccdc);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092cce1); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x700092ccf0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092ccf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092ccf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092ccf8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092ccfe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092cd02); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092cd06); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092cd0d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092cd14); };
            c.w32(11, t7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092cd19); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092cd1e); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092cd22); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092cd26); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092cd2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700092cd2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092cd32); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092cd36); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092cd3f); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092cd46); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092cd4a); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092cd57); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092cd5b); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700092cd61); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092cd69); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092cd70); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092cd7d); };
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
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092cd97); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x700092cda3, 0x700092cd9e);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092cda3); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092cdac); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092cdb0); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092cdb4); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092cdb8); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092cdbc); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092cdc0); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092cdc6); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092cde6); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092cdee); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092ce01); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ce06); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092ce0e); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092ce17); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700092ce18); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700092ce2c, 0x700092ce27);
        }
        15 => {
            return RegfileExit::fault(0x700092ce2c);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092ce2d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            c.w64(1, c.r64(11));
            let r39 = c.shr(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
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
            let Some(t40) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092ce73); };
            c.cmp(t40, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092ce79); };
            c.w32(1, t41);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r42 = c.r32(2) | c.r32(0);
            c.w32(2, r42);
            c.logic(r42, 32);
            let r43 = c.r32(2) | c.r32(6);
            c.w32(2, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092ce8f); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ce94); }
            blk = 26; continue;
        }
        26 => {
            let Some(t44) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ce9c); };
            c.w64(7, t44);
            let r45 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r45);
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092cea4); };
            c.w32(8, t46);
            let Some(t47) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092cea9); };
            c.w64(1, t47);
            let Some(t48) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092cead); };
            c.w64(5, t48);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r49 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092cebd); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092cec4); };
            c.w64(1, t51);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092cecb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700092cecf); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092ced3); };
            c.w64(12, t52);
            let Some(t53) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092ceda); };
            c.cmp(t53, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092ceed, 0x700092cee8);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092cef0); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092cef4); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092cef8); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092cefc); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092cf00); };
            c.w64(2, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092cf04); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092cf08); };
            c.w64(3, t60);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x700092cf0e); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092cf15); };
            c.w64(1, t61);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700092cf1c); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092cf2c); }
            blk = 20; continue;
        }
        32 => {
            let r62 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092cf3b); }
            return c.call(0x70008708b4, 0x700092cf45, 0x700092cf40);
        }
        33 => {
            let Some(t63) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092cf45); };
            c.w32(0, t63);
            let r64 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r64);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x700092cf54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092cf74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092cf77); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092cf7b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092cf82); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092cf86); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092cf8d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092cf92); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092cf96); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092cf9f); };
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
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092cfae); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092cfb4); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092cfbd); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092cfd5); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092cfe2); }
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
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092d007); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092d00e); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092d017); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700092d01e); };
            c.cmp(t22, c.r64(1), 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700092d027); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092d040); };
            c.w64(0, t23);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700092d047); }
            let r24 = c.r32(2) | 0xeu64;
            c.w32(2, r24);
            c.logic(r24, 32);
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700092d04e); }
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092d05a); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092d063); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092d077); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092d07c); }
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d084); };
            c.w64(7, t28);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092d08f); };
            c.w32(8, t29);
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d094); };
            c.w64(1, t30);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d09d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d0a1); }
            let r31 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r31);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let r32 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r32);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092d0ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d0b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d0b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d0b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d0bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d0c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700092d0c5); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x700092d0d1, 0x700092d0cc);
        }
        20 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092d0d1); };
            c.w64(11, t33);
            c.w32(2, 0xfeu64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d0da); };
            c.w64(15, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d0de); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d0e2); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d0e6); };
            c.w64(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d0ea); };
            c.w64(3, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d0f1); }
            blk = 3; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d0f7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d0fb); }
            let r39 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r39);
            c.w32(2, c.r32(2));
            let r40 = c.r32(2) | 0x40u64;
            c.w32(2, r40);
            c.logic(r40, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092d10f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092d113); }
            c.w32(0, 0x1cu64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092d119); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d11a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d11e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d122); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d126); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d12a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092d12e); }
            c.w64(6, c.r64(11));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x700092d135); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700092d149, 0x700092d144);
        }
        22 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092d149); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d14d); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d151); };
            c.w64(15, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d155); };
            c.w64(9, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d159); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d15d); };
            c.w64(8, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d164); }
            blk = 15; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d16a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d16e); }
            let r49 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700092d174); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d17d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d181); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d185); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d189); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d18d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700092d191); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092d19e, 0x700092d199);
        }
        24 => {
            c.w64(11, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d1a1); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d1a5); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d1a9); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d1ad); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d1b1); };
            c.w64(3, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092d1b5); };
            c.w64(2, t55);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700092d1b9); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d1c1); }
            blk = 14; continue;
        }
        25 => {
            return RegfileExit::jump(0x700092d1c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092d1d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d1d7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092d1db); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092d1e2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092d1e6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092d1ed); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092d1f2); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092d1f6); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092d1ff); };
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
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092d20e); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092d214); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092d21d); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092d235); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092d242); }
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
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092d267); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092d26e); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092d277); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700092d27e); };
            c.cmp(t22, c.r64(1), 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700092d287); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.w64(10, 0xbff0000000000000u64);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092d2a1); };
            c.w64(0, t23);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700092d2a8); }
            let r24 = c.r32(2) | 0xeu64;
            c.w32(2, r24);
            c.logic(r24, 32);
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700092d2af); }
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092d2bb); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092d2c4); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092d2d8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092d2dd); }
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d2e5); };
            c.w64(7, t28);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092d2f0); };
            c.w32(8, t29);
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d2f5); };
            c.w64(1, t30);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d2fe); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d302); }
            let r31 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r31);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let r32 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r32);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092d30e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d312); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d316); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d31a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d31e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d322); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700092d326); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x700092d332, 0x700092d32d);
        }
        20 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092d332); };
            c.w64(11, t33);
            c.w32(2, 0xfeu64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d33b); };
            c.w64(15, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d33f); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d343); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d347); };
            c.w64(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d34b); };
            c.w64(3, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d352); }
            blk = 3; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d358); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d35c); }
            let r39 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r39);
            c.w32(2, c.r32(2));
            let r40 = c.r32(2) | 0x40u64;
            c.w32(2, r40);
            c.logic(r40, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092d370); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092d374); }
            c.w32(0, 0x1eu64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092d37a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d37b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d37f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d383); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d387); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d38b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092d38f); }
            c.w64(6, c.r64(11));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x700092d396); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700092d3aa, 0x700092d3a5);
        }
        22 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092d3aa); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d3ae); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d3b2); };
            c.w64(15, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d3b6); };
            c.w64(9, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d3ba); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d3be); };
            c.w64(8, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d3c5); }
            blk = 15; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d3cb); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d3cf); }
            let r49 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700092d3d5); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d3de); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d3e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d3e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d3ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d3ee); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700092d3f2); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092d3ff, 0x700092d3fa);
        }
        24 => {
            c.w64(11, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d402); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d406); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d40a); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d40e); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d412); };
            c.w64(3, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092d416); };
            c.w64(2, t55);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700092d41a); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d422); }
            blk = 14; continue;
        }
        25 => {
            return RegfileExit::jump(0x700092d42c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092d434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d437); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092d43b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092d442); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092d446); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092d44d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092d452); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092d456); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092d45f); };
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
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092d46e); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092d474); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092d47d); };
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
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092d495); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092d4a2); }
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
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092d4d2); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092d4d9); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let r21 = c.r32(2) | 0xeu64;
            c.w32(2, r21);
            c.logic(r21, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092d4e8); };
            c.w64(6, t22);
            let Some(t23) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700092d4ef); };
            c.cmp(t23, c.r64(1), 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700092d4f8); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x700092d4ff); };
            c.w64(10, t24);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x700092d506); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092d50a); };
            c.w64(0, t25);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700092d511); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092d515); }
            c.w64(0, c.r64(11));
            blk = 17; continue;
        }
        16 => {
            let r26 = c.r32(11) | 0xeu64;
            c.w32(11, r26);
            c.logic(r26, 32);
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700092d527); };
            c.w64(0, t27);
            blk = 17; continue;
        }
        17 => {
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092d52e); };
            c.cmp(t28, c.r32(8), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092d537); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092d54b); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092d550); }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d558); };
            c.w64(7, t31);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092d563); };
            c.w32(8, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d568); };
            c.w64(1, t33);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d571); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d575); }
            let r34 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r34);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let r35 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r35);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092d581); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d585); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d589); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d58d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d591); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d595); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700092d599); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x700092d5a5, 0x700092d5a0);
        }
        22 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092d5a5); };
            c.w64(11, t36);
            c.w32(2, 0xfeu64);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d5ae); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d5b2); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d5b6); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d5ba); };
            c.w64(8, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d5be); };
            c.w64(3, t41);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d5c5); }
            blk = 3; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d5cb); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d5cf); }
            let r42 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r42);
            c.w32(2, c.r32(2));
            let r43 = c.r32(2) | 0x40u64;
            c.w32(2, r43);
            c.logic(r43, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r44 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r44);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092d5e3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092d5e7); }
            c.w32(0, 0x1au64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092d5ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d5ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d5f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d5f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d5fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d5fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092d602); }
            c.w64(6, c.r64(11));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x700092d609); };
            c.w64(3, t45);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700092d61d, 0x700092d618);
        }
        24 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092d61d); };
            c.w64(2, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d621); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d625); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d629); };
            c.w64(9, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d62d); };
            c.w64(7, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d631); };
            c.w64(8, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d638); }
            blk = 17; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d63e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d642); }
            let r52 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092d648); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d651); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d655); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d659); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d65d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700092d661); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700092d665); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092d672, 0x700092d66d);
        }
        26 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d675); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d679); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d67d); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d681); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d685); };
            c.w64(3, t57);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700092d689); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092d68e); };
            c.w64(2, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092d695); }
            blk = 15; continue;
        }
        27 => {
            return RegfileExit::jump(0x700092d69c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092d6b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d6b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d6b8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092d6be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d6c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092d6c6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092d6cd); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092d6d1); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092d6d8); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092d6dd); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d6e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d6e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d6e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700092d6ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700092d6f1); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092d6f5); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092d6fe); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092d705); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d712); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700092d718); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092d720); };
            c.w32(6, t13);
            let r14 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092d726); };
            c.cmp(t15, c.r32(6), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092d72f); };
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
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r17 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r17);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092d749); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x700092d755, 0x700092d750);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092d755); };
            c.w64(3, t18);
            c.w32(1, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d75e); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d762); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d766); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d76a); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d76e); };
            c.w64(11, t23);
            blk = 3; continue;
        }
        9 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092d774); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(1, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(11, c.r32(1));
            let r26 = c.r32(11) | 0x2u64;
            c.w32(11, r26);
            c.logic(r26, 32);
            let r27 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092d78f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(0));
            c.w32(11, c.r32(1));
            let r28 = c.r32(11) | 0xeu64;
            c.w32(11, r28);
            c.logic(r28, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        13 => {
            let r29 = !c.r32(0);
            c.w32(0, r29);
            c.w64(2, c.r64(0));
            let r30 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r30);
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
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092d7cf); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092d7d5); };
            c.w32(1, t32);
            c.w32(11, c.r32(11));
            c.w32(2, c.r32(2));
            c.w32(6, c.r32(1));
            let r33 = c.r32(11) | c.r32(2);
            c.w32(11, r33);
            c.logic(r33, 32);
            let r34 = c.r32(11) | c.r32(6);
            c.w32(11, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092d7ef); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092d7f4); }
            blk = 21; continue;
        }
        21 => {
            let Some(t35) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d7fc); };
            c.w64(7, t35);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092d807); };
            c.w32(8, t36);
            let Some(t37) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d80c); };
            c.w64(1, t37);
            let Some(t38) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092d810); };
            c.w64(5, t38);
            c.w64(12, c.r64(7));
            let r39 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r39);
            return RegfileExit::jump(c.r64(1));
        }
        22 => {
            c.w64(2, c.r64(9));
            c.w64(1, c.r64(2).wrapping_sub(0x1));
            let r40 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r40);
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092d826); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092d82c); };
            c.w32(6, t42);
            c.w32(9, c.r32(6));
            let r43 = c.r32(9) | 0x40u64;
            c.w32(9, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(6, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x700092d840); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092d845); }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092d84d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092d851); }
            c.w32(8, 0x18u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092d858); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x700092d85a); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700092d86e, 0x700092d869);
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d86e); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d872); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d876); };
            c.w64(9, t47);
            blk = 21; continue;
        }
        27 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092d87c); };
            c.w64(2, t48);
            let Some(t49) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700092d883); };
            c.w64(1, t49);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700092d88a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700092d88e); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092d892); };
            c.w64(12, t50);
            let Some(t51) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700092d899); };
            c.cmp(t51, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092d8a9, 0x700092d8a4);
        }
        29 => {
            c.w64(2, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d8ac); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d8b0); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d8b4); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092d8b8); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092d8bc); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092d8c0); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092d8c4); };
            c.w64(3, t58);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x700092d8ca); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092d8d1); };
            c.w64(1, t59);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700092d8d8); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092d8e3); }
            c.w64(0, c.r64(2));
            blk = 15; continue;
        }
        32 => {
            let r60 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r60);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092d8f4); }
            return c.call(0x70008708b4, 0x700092d8fe, 0x700092d8f9);
        }
        33 => {
            let Some(t61) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092d8fe); };
            c.w32(0, t61);
            let r62 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            blk = 12; continue;
        }
        34 => {
            return RegfileExit::jump(0x700092d90c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092d914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d918); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092d91e); };
            c.w64(2, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092d923); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d927); };
            c.w64(2, t3);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092d92f); };
            c.w64(11, t4);
            let r5 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r5);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d936); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d93a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d93e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d942); }
            c.w32(1, 0x2u64);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x7000820e54, 0x700092d959, 0x700092d954);
        }
        1 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d959); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d95d); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d961); };
            c.w64(7, t8);
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700092d969); };
            c.w32(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d96e); };
            c.w64(15, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d972); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092d976); };
            c.w64(5, t12);
            c.w64(12, c.r64(2));
            let r13 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700092d984);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092d994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092d994); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092d998); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092d99e); };
            c.w64(2, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092d9a3); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d9a7); };
            c.w64(2, t3);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092d9af); };
            c.w64(11, t4);
            let r5 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r5);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092d9b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092d9ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092d9be); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092d9c2); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x7000820e54, 0x700092d9d6, 0x700092d9d1);
        }
        1 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d9d6); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092d9da); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092d9de); };
            c.w64(7, t8);
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700092d9e6); };
            c.w32(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092d9eb); };
            c.w64(15, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700092d9ef); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092d9f3); };
            c.w64(5, t12);
            c.w64(12, c.r64(2));
            let r13 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700092da04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092da14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092da14); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092da1a); };
            c.w64(2, (t2 as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(5));
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092da22); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            if !c.st64(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(8)), c.r64(3)) { return RegfileExit::fault(0x700092da28); }
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092da30); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092da35); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x700092da3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092da54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092da54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092da58); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092da62); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092da68); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092da6c); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092da70); };
            c.w64(8, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092da75); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092da7b); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092da7f); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092da85); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092da8a); };
            c.w32(15, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x700092da8f); };
            c.w32(0, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092da94); };
            c.w32(3, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700092da97); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092da9b); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092da9f); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092daa8); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092daaf); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dab3); };
            c.w32(6, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092dab6); };
            c.cmp(t18, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092dad3); };
            c.w32(7, t19);
            let r20 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x700092dadd); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092dae1); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, c.r32(7));
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700092daef); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092dafc); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092db09); };
            c.cmp(t25, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092db1f); };
            c.w32(15, t26);
            c.w64(11, c.r64(7));
            let r27 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r27);
            c.logic(r27, 64);
            c.w32(0, c.r32(15));
            let r28 = c.r32(0) & 0x3u64;
            c.w32(0, r28);
            c.logic(r28, 32);
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
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092db54); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092db5b); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092db62); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092db72);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092db73); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092db7d); };
            c.w64(9, (t35 as u32 as i32 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t36) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700092db85); };
            c.w64(9, t36);
            let r37 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092db96); };
            c.w32(9, t38);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r39 = c.r32(11) & 0x1u64;
            c.w32(11, r39);
            c.logic(r39, 32);
            let r40 = c.r32(12) & 0x3u64;
            c.w32(12, r40);
            c.logic(r40, 32);
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
            let r41 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r41);
            c.logic(r41, 64);
            let r42 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r42);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x700092dbc2); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092dbc7); }
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
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092dbdd); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            blk = 20; continue;
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dbe4); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092dbeb); };
            c.w32(11, t47);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092dc0d); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dc14); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dc1b); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092dc22); };
            c.w32(11, t55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092dc26); };
            c.cmp(t56, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r57 = c.r64(9) | 0x3u64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092dc37); };
            c.w64(15, t58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x700092dc3b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(2);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092dc4f); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092dc5a); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092dc61); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        26 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092dc6a); };
            c.w64(11, t62);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dc7d); };
            c.w32(9, t63);
            let r64 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r64);
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092dc84); };
            c.w32(9, t65);
            let r66 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r66);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092dc9a); };
            c.w32(9, t67);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dca3); };
            c.w32(12, t68);
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092dcaa); };
            c.w32(12, t70);
            let r71 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r71);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092dcc1); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r74 = c.r64(11) | 0x3u64;
            c.w64(11, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092dcd5); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x700092dcd9); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r76 = c.r64(9) & c.r64(2);
            c.w64(9, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092dced); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092dcf8); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092dcff); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        35 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092dd08); };
            c.w64(11, t79);
            blk = 56; continue;
        }
        36 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092dd11); };
            c.w32(15, t80);
            c.w32(7, c.r32(15));
            let r81 = c.r32(7) & 0x3u64;
            c.w32(7, r81);
            c.logic(r81, 32);
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
            let r82 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r82);
            c.logic(r82, 64);
            let r83 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x700092dd34); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092dd39); }
            blk = 39; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dd40); };
            c.w32(15, t84);
            let r85 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092dd47); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092dd4e); };
            c.w32(7, t88);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x700092dd5e);
        }
        41 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092dd5f); };
            c.cmp(t89, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092dd6a); };
            c.w64(9, (t90 as u32 as i32 as i64 as u64));
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092dd6f); };
            c.w32(12, t91);
            let Some(t92) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700092dd74); };
            c.w64(9, t92);
            c.w32(15, c.r32(12));
            let r93 = c.r32(15) & 0x3u64;
            c.w32(15, r93);
            c.logic(r93, 32);
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
            let r94 = c.r64(12) | 0x2u64;
            c.w64(12, r94);
            c.logic(r94, 64);
            let r95 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r95);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x700092dd98); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092dd9d); }
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
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092ddb3); };
            c.w32(12, t96);
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            blk = 49; continue;
        }
        49 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ddbb); };
            c.w32(15, t98);
            let r99 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r99);
            let Some(t100) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ddc3); };
            c.w32(15, t100);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r101 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r101);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092dde1); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092dde9); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ddf0); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ddf8); };
            c.w32(15, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ddfc); };
            c.cmp(t109, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r110 = c.r64(9) | 0x3u64;
            c.w64(9, r110);
            c.logic(r110, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x700092de07); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r111 = c.r64(12) & c.r64(2);
            c.w64(12, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092de1b); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092de23); }
            blk = 56; continue;
        }
        55 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092de2c); };
            c.w64(9, t113);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700092de33); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092de38); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092de3f); };
            c.w64(11, t114);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092de4a); };
            c.w64(1, t115);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t116) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092de51); };
            c.w64(5, t116);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r117 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r117);
            return RegfileExit::jump(0x70007fcbb4);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            c.w64(12, 0xfffffffffffc0000u64);
            let r119 = c.r64(12) & c.r64(9);
            c.w64(12, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092de77); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092de83); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092de84); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x700092de91, 0x700092de8c);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092de91); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092de93); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r121 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r121);
            c.w64(9, 0xfffffffffffc0000u64);
            let r122 = c.r64(9) & c.r64(11);
            c.w64(9, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092dea6); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092deb5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092deb6); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700092dec3, 0x700092debe);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092dec3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092dec5); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r124 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r124);
            c.w64(11, 0xfffffffffffc0000u64);
            let r125 = c.r64(11) & c.r64(9);
            c.w64(11, r125);
            c.logic(r125, 64);
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092dedb); };
            c.logic(t126 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092deea); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092deeb); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700092def8, 0x700092def3);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092def8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092defa); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x700092df04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092df14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092df14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092df18); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092df22); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092df28); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092df2c); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092df30); };
            c.w64(8, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092df35); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092df3b); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092df3f); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092df45); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092df4a); };
            c.w32(15, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x700092df4f); };
            c.w32(0, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092df54); };
            c.w32(3, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700092df57); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092df5b); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092df5f); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092df68); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092df6f); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092df73); };
            c.w32(6, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092df76); };
            c.cmp(t18, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092df93); };
            c.w32(7, t19);
            let r20 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x700092df9d); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092dfa1); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, c.r32(7));
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700092dfaf); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092dfbc); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092dfc9); };
            c.cmp(t25, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092dfdf); };
            c.w32(15, t26);
            c.w64(11, c.r64(7));
            let r27 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r27);
            c.logic(r27, 64);
            c.w32(0, c.r32(15));
            let r28 = c.r32(0) & 0x3u64;
            c.w32(0, r28);
            c.logic(r28, 32);
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
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e014); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e01b); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092e022); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092e032);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092e033); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092e03d); };
            c.w64(9, (t35 as u32 as i32 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t36) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e045); };
            c.w64(9, t36);
            let r37 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e056); };
            c.w32(9, t38);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r39 = c.r32(11) & 0x1u64;
            c.w32(11, r39);
            c.logic(r39, 32);
            let r40 = c.r32(12) & 0x3u64;
            c.w32(12, r40);
            c.logic(r40, 32);
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
            let r41 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r41);
            c.logic(r41, 64);
            let r42 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r42);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x700092e082); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e087); }
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
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092e09d); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            blk = 20; continue;
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e0a4); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e0ab); };
            c.w32(11, t47);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092e0cd); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e0d4); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e0db); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e0e2); };
            c.w32(11, t55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e0e6); };
            c.cmp(t56, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r57 = c.r64(9) | 0x3u64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092e0f7); };
            c.w64(15, t58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x700092e0fb); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(2);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e10f); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e11a); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092e121); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        26 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092e12a); };
            c.w64(11, t62);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e13d); };
            c.w32(9, t63);
            let r64 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r64);
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e144); };
            c.w32(9, t65);
            let r66 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r66);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e15a); };
            c.w32(9, t67);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e163); };
            c.w32(12, t68);
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e16a); };
            c.w32(12, t70);
            let r71 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r71);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e181); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r74 = c.r64(11) | 0x3u64;
            c.w64(11, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092e195); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x700092e199); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r76 = c.r64(9) & c.r64(2);
            c.w64(9, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e1ad); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e1b8); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092e1bf); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        35 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092e1c8); };
            c.w64(11, t79);
            blk = 56; continue;
        }
        36 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e1d1); };
            c.w32(15, t80);
            c.w32(7, c.r32(15));
            let r81 = c.r32(7) & 0x3u64;
            c.w32(7, r81);
            c.logic(r81, 32);
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
            let r82 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r82);
            c.logic(r82, 64);
            let r83 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x700092e1f4); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e1f9); }
            blk = 39; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e200); };
            c.w32(15, t84);
            let r85 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e207); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092e20e); };
            c.w32(7, t88);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x700092e21e);
        }
        41 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092e21f); };
            c.cmp(t89, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092e22a); };
            c.w64(9, (t90 as u32 as i32 as i64 as u64));
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e22f); };
            c.w32(12, t91);
            let Some(t92) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e234); };
            c.w64(9, t92);
            c.w32(15, c.r32(12));
            let r93 = c.r32(15) & 0x3u64;
            c.w32(15, r93);
            c.logic(r93, 32);
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
            let r94 = c.r64(12) | 0x2u64;
            c.w64(12, r94);
            c.logic(r94, 64);
            let r95 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r95);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x700092e258); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e25d); }
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
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092e273); };
            c.w32(12, t96);
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            blk = 49; continue;
        }
        49 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e27b); };
            c.w32(15, t98);
            let r99 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r99);
            let Some(t100) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e283); };
            c.w32(15, t100);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r101 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r101);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092e2a1); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e2a9); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e2b0); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e2b8); };
            c.w32(15, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e2bc); };
            c.cmp(t109, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r110 = c.r64(9) | 0x3u64;
            c.w64(9, r110);
            c.logic(r110, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x700092e2c7); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r111 = c.r64(12) & c.r64(2);
            c.w64(12, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e2db); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e2e3); }
            blk = 56; continue;
        }
        55 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092e2ec); };
            c.w64(9, t113);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700092e2f3); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e2f8); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092e2ff); };
            c.w64(11, t114);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092e30a); };
            c.w64(1, t115);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t116) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092e311); };
            c.w64(5, t116);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r117 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r117);
            return RegfileExit::jump(0x70007fcbb4);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            c.w64(12, 0xfffffffffffc0000u64);
            let r119 = c.r64(12) & c.r64(9);
            c.w64(12, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e337); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092e343); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092e344); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x700092e351, 0x700092e34c);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092e351); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092e353); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r121 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r121);
            c.w64(9, 0xfffffffffffc0000u64);
            let r122 = c.r64(9) & c.r64(11);
            c.w64(9, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e366); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092e375); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092e376); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700092e383, 0x700092e37e);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092e383); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092e385); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r124 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r124);
            c.w64(11, 0xfffffffffffc0000u64);
            let r125 = c.r64(11) & c.r64(9);
            c.w64(11, r125);
            c.logic(r125, 64);
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e39b); };
            c.logic(t126 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092e3aa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092e3ab); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700092e3b8, 0x700092e3b3);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092e3b8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092e3ba); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x700092e3c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092e3d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092e3d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092e3d8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092e3e2); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700092e3e8); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092e3ec); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092e3f0); };
            c.w64(1, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e3f5); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e3fb); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092e3ff); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092e405); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e40a); };
            c.w32(11, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092e40e); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092e417); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092e41e); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e422); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092e426); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e443); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092e44e); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092e453); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700092e460); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092e46c); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092e479); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e48f); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(8, c.r32(6));
            let r26 = c.r32(8) & 0x3u64;
            c.w32(8, r26);
            c.logic(r26, 32);
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
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e4c6); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e4cd); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092e4d4); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092e4e4);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092e4e5); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092e4ef); };
            c.w64(8, (t33 as u32 as i32 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t34) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e4f6); };
            c.w64(8, t34);
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e506); };
            c.w32(8, t36);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r37 = c.r32(0) & 0x1u64;
            c.w32(0, r37);
            c.logic(r37, 32);
            let r38 = c.r32(6) & 0x3u64;
            c.w32(6, r38);
            c.logic(r38, 32);
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
            let r39 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r39);
            c.logic(r39, 64);
            let r40 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r40);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x700092e52e); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e533); }
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
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092e548); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e54f); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            let Some(t45) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e556); };
            c.w32(0, t45);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r46 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r46);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092e571); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e578); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e57f); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e586); };
            c.w32(0, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e589); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r55 = c.r64(8) | 0x3u64;
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092e59a); };
            c.w64(0, t56);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x700092e59e); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r57 = c.r64(15) & c.r64(3);
            c.w64(15, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e5b2); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e5bd); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e5d2); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e5d9); };
            c.w32(8, t61);
            let r62 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r62);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e5ef); };
            c.w32(8, t63);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e5fc); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            let Some(t66) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e602); };
            c.w32(0, t66);
            let r67 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r67);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e614); };
            c.w32(0, t68);
            let r69 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r69);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r70 = c.r64(0) | 0x3u64;
            c.w64(0, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092e627); };
            c.w64(8, t71);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092e62b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r72 = c.r64(15) & c.r64(3);
            c.w64(15, r72);
            c.logic(r72, 64);
            let Some(t73) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e63d); };
            c.logic(t73 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e648); }
            blk = 54; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e654); };
            c.w32(0, t74);
            c.w32(6, c.r32(0));
            let r75 = c.r32(6) & 0x3u64;
            c.w32(6, r75);
            c.logic(r75, 32);
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
            let r76 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r76);
            c.logic(r76, 64);
            let r77 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r77);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092e673); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e678); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e67f); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e686); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092e68c); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700092e69b);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092e69c); };
            c.cmp(t83, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t84) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092e6a6); };
            c.w64(0, (t84 as u32 as i32 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e6ab); };
            c.w32(6, t85);
            let Some(t86) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e6b0); };
            c.w64(0, t86);
            c.w32(7, c.r32(6));
            let r87 = c.r32(7) & 0x3u64;
            c.w32(7, r87);
            c.logic(r87, 32);
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
            let r88 = c.r64(6) | 0x2u64;
            c.w64(6, r88);
            c.logic(r88, 64);
            let r89 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r89);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x700092e6d0); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e6d5); }
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
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092e6e9); };
            c.w32(6, t90);
            let r91 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r91);
            blk = 47; continue;
        }
        47 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e6ef); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e6f5); };
            c.w32(7, t94);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r95 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r95);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t96) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092e70e); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e714); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e71b); };
            c.w32(6, t100);
            let r101 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r101);
            let Some(t102) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e721); };
            c.w32(7, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e724); };
            c.cmp(t103, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r104 = c.r64(0) | 0x3u64;
            c.w64(0, r104);
            c.logic(r104, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092e72d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r105 = c.r64(15) & c.r64(3);
            c.w64(15, r105);
            c.logic(r105, 64);
            let Some(t106) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e73f); };
            c.logic(t106 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e746); }
            blk = 54; continue;
        }
        53 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092e74f); };
            c.w64(8, t107);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700092e756); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e75b); }
            blk = 54; continue;
        }
        54 => {
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092e762); };
            c.w64(3, (t108 as u32 as i32 as i64 as u64));
            let Some(t109) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e767); };
            c.w64(3, t109);
            let Some(t110) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092e76b); };
            c.w64(5, t110);
            let r111 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r111);
            let Some(t112) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700092e773); };
            if !c.push64(t112) { return RegfileExit::fault(0x700092e773); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x700092e77b); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r113 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r113);
            c.w64(15, 0xfffffffffffc0000u64);
            let r114 = c.r64(15) & c.r64(0);
            c.w64(15, r114);
            c.logic(r114, 64);
            let Some(t115) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e797); };
            c.logic(t115 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092e7a2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092e7a3); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092e7b0, 0x700092e7ab);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092e7b0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092e7b2); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r116 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r116);
            c.w64(15, 0xfffffffffffc0000u64);
            let r117 = c.r64(15) & c.r64(0);
            c.w64(15, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e7c4); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092e7d3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092e7d4); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092e7e1, 0x700092e7dc);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092e7e1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092e7e3); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r119 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r119);
            c.w64(15, 0xfffffffffffc0000u64);
            let r120 = c.r64(15) & c.r64(8);
            c.w64(15, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092e7f9); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092e808); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092e809); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092e816, 0x700092e811);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092e816); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092e818); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x700092e820);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092e834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092e834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092e838); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092e842); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700092e848); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092e84c); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092e850); };
            c.w64(1, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e855); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e85b); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092e85f); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x700092e865); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e86a); };
            c.w32(11, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092e86e); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092e877); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092e87e); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e882); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092e886); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e8a3); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092e8ae); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092e8b3); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700092e8c0); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092e8cc); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092e8d9); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e8ef); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(8, c.r32(6));
            let r26 = c.r32(8) & 0x3u64;
            c.w32(8, r26);
            c.logic(r26, 32);
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
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e926); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e92d); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092e934); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092e944);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092e945); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092e94f); };
            c.w64(8, (t33 as u32 as i32 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t34) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092e956); };
            c.w64(8, t34);
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092e966); };
            c.w32(8, t36);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r37 = c.r32(0) & 0x1u64;
            c.w32(0, r37);
            c.logic(r37, 32);
            let r38 = c.r32(6) & 0x3u64;
            c.w32(6, r38);
            c.logic(r38, 32);
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
            let r39 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r39);
            c.logic(r39, 64);
            let r40 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r40);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x700092e98e); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092e993); }
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
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092e9a8); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e9af); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            let Some(t45) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092e9b6); };
            c.w32(0, t45);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r46 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r46);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092e9d1); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e9d8); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092e9df); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e9e6); };
            c.w32(0, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092e9e9); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r55 = c.r64(8) | 0x3u64;
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092e9fa); };
            c.w64(0, t56);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x700092e9fe); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r57 = c.r64(15) & c.r64(3);
            c.w64(15, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092ea12); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ea1d); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ea32); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ea39); };
            c.w32(8, t61);
            let r62 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r62);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ea4f); };
            c.w32(8, t63);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ea5c); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            let Some(t66) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ea62); };
            c.w32(0, t66);
            let r67 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r67);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ea74); };
            c.w32(0, t68);
            let r69 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r69);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r70 = c.r64(0) | 0x3u64;
            c.w64(0, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ea87); };
            c.w64(8, t71);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092ea8b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r72 = c.r64(15) & c.r64(3);
            c.w64(15, r72);
            c.logic(r72, 64);
            let Some(t73) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092ea9d); };
            c.logic(t73 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092eaa8); }
            blk = 54; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092eab4); };
            c.w32(0, t74);
            c.w32(6, c.r32(0));
            let r75 = c.r32(6) & 0x3u64;
            c.w32(6, r75);
            c.logic(r75, 32);
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
            let r76 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r76);
            c.logic(r76, 64);
            let r77 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r77);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092ead3); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ead8); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092eadf); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092eae6); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092eaec); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700092eafb);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092eafc); };
            c.cmp(t83, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t84) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092eb06); };
            c.w64(0, (t84 as u32 as i32 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092eb0b); };
            c.w32(6, t85);
            let Some(t86) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092eb10); };
            c.w64(0, t86);
            c.w32(7, c.r32(6));
            let r87 = c.r32(7) & 0x3u64;
            c.w32(7, r87);
            c.logic(r87, 32);
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
            let r88 = c.r64(6) | 0x2u64;
            c.w64(6, r88);
            c.logic(r88, 64);
            let r89 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r89);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x700092eb30); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092eb35); }
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
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092eb49); };
            c.w32(6, t90);
            let r91 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r91);
            blk = 47; continue;
        }
        47 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092eb4f); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092eb55); };
            c.w32(7, t94);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r95 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r95);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t96) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092eb6e); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092eb74); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092eb7b); };
            c.w32(6, t100);
            let r101 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r101);
            let Some(t102) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092eb81); };
            c.w32(7, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092eb84); };
            c.cmp(t103, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r104 = c.r64(0) | 0x3u64;
            c.w64(0, r104);
            c.logic(r104, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092eb8d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r105 = c.r64(15) & c.r64(3);
            c.w64(15, r105);
            c.logic(r105, 64);
            let Some(t106) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092eb9f); };
            c.logic(t106 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092eba6); }
            blk = 54; continue;
        }
        53 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092ebaf); };
            c.w64(8, t107);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700092ebb6); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ebbb); }
            blk = 54; continue;
        }
        54 => {
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092ebc2); };
            c.w64(3, (t108 as u32 as i32 as i64 as u64));
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092ebc7); };
            c.w64(8, (t109 as u32 as i32 as i64 as u64));
            let Some(t110) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092ebcc); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092ebd0); };
            c.w64(2, t111);
            let Some(t112) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092ebd4); };
            c.w64(5, t112);
            let r113 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r113);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092ebdc); }
            let Some(t114) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092ebdd); };
            if !c.push64(t114) { return RegfileExit::fault(0x700092ebdd); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x700092ebe9); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r115 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r115);
            c.w64(15, 0xfffffffffffc0000u64);
            let r116 = c.r64(15) & c.r64(0);
            c.w64(15, r116);
            c.logic(r116, 64);
            let Some(t117) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092ec05); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092ec10); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092ec11); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092ec1e, 0x700092ec19);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092ec1e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092ec20); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r118 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r118);
            c.w64(15, 0xfffffffffffc0000u64);
            let r119 = c.r64(15) & c.r64(0);
            c.w64(15, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092ec32); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092ec41); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092ec42); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092ec4f, 0x700092ec4a);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092ec4f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092ec51); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r121 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r121);
            c.w64(15, 0xfffffffffffc0000u64);
            let r122 = c.r64(15) & c.r64(8);
            c.w64(15, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092ec67); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092ec76); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092ec77); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092ec84, 0x700092ec7f);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092ec84); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092ec86); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x700092ec90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092ec94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092ec94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092ec98); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092eca2); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700092eca8); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ecac); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092ecb0); };
            c.w64(1, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ecb5); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700092ecbb); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092ecbf); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x11)) else { return RegfileExit::fault(0x700092ecc5); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ecca); };
            c.w32(11, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092ecce); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ecd7); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092ecde); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ece2); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092ece6); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092ed03); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092ed0e); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092ed13); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700092ed20); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092ed2c); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092ed39); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092ed4f); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(8, c.r32(6));
            let r26 = c.r32(8) & 0x3u64;
            c.w32(8, r26);
            c.logic(r26, 32);
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
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ed86); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ed8d); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092ed94); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092eda4);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092eda5); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092edaf); };
            c.w64(8, (t33 as u32 as i32 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t34) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092edb6); };
            c.w64(8, t34);
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092edc6); };
            c.w32(8, t36);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r37 = c.r32(0) & 0x1u64;
            c.w32(0, r37);
            c.logic(r37, 32);
            let r38 = c.r32(6) & 0x3u64;
            c.w32(6, r38);
            c.logic(r38, 32);
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
            let r39 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r39);
            c.logic(r39, 64);
            let r40 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r40);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x700092edee); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092edf3); }
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
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092ee08); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ee0f); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            let Some(t45) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ee16); };
            c.w32(0, t45);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r46 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r46);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092ee31); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ee38); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ee3f); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ee46); };
            c.w32(0, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ee49); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r55 = c.r64(8) | 0x3u64;
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ee5a); };
            c.w64(0, t56);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x700092ee5e); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r57 = c.r64(15) & c.r64(3);
            c.w64(15, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092ee72); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ee7d); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ee92); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ee99); };
            c.w32(8, t61);
            let r62 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r62);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092eeaf); };
            c.w32(8, t63);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092eebc); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            let Some(t66) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092eec2); };
            c.w32(0, t66);
            let r67 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r67);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092eed4); };
            c.w32(0, t68);
            let r69 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r69);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r70 = c.r64(0) | 0x3u64;
            c.w64(0, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092eee7); };
            c.w64(8, t71);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092eeeb); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r72 = c.r64(15) & c.r64(3);
            c.w64(15, r72);
            c.logic(r72, 64);
            let Some(t73) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092eefd); };
            c.logic(t73 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ef08); }
            blk = 54; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092ef14); };
            c.w32(0, t74);
            c.w32(6, c.r32(0));
            let r75 = c.r32(6) & 0x3u64;
            c.w32(6, r75);
            c.logic(r75, 32);
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
            let r76 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r76);
            c.logic(r76, 64);
            let r77 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r77);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092ef33); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ef38); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ef3f); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ef46); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092ef4c); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700092ef5b);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092ef5c); };
            c.cmp(t83, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t84) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092ef66); };
            c.w64(0, (t84 as u32 as i32 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092ef6b); };
            c.w32(6, t85);
            let Some(t86) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092ef70); };
            c.w64(0, t86);
            c.w32(7, c.r32(6));
            let r87 = c.r32(7) & 0x3u64;
            c.w32(7, r87);
            c.logic(r87, 32);
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
            let r88 = c.r64(6) | 0x2u64;
            c.w64(6, r88);
            c.logic(r88, 64);
            let r89 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r89);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x700092ef90); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ef95); }
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
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092efa9); };
            c.w32(6, t90);
            let r91 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r91);
            blk = 47; continue;
        }
        47 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092efaf); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092efb5); };
            c.w32(7, t94);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r95 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r95);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t96) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092efce); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092efd4); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092efdb); };
            c.w32(6, t100);
            let r101 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r101);
            let Some(t102) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092efe1); };
            c.w32(7, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092efe4); };
            c.cmp(t103, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r104 = c.r64(0) | 0x3u64;
            c.w64(0, r104);
            c.logic(r104, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092efed); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r105 = c.r64(15) & c.r64(3);
            c.w64(15, r105);
            c.logic(r105, 64);
            let Some(t106) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092efff); };
            c.logic(t106 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f006); }
            blk = 54; continue;
        }
        53 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092f00f); };
            c.w64(8, t107);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700092f016); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f01b); }
            blk = 54; continue;
        }
        54 => {
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092f022); };
            c.w64(3, (t108 as u32 as i32 as i64 as u64));
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092f027); };
            c.w64(8, (t109 as u32 as i32 as i64 as u64));
            let Some(t110) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x700092f02c); };
            c.w64(9, (t110 as u32 as i32 as i64 as u64));
            let Some(t111) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092f031); };
            c.w64(3, t111);
            let Some(t112) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092f035); };
            c.w64(8, t112);
            let Some(t113) = c.ld64(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700092f039); };
            c.w64(2, t113);
            let Some(t114) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092f03d); };
            c.w64(5, t114);
            let r115 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r115);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f045); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092f047); }
            let Some(t116) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700092f048); };
            if !c.push64(t116) { return RegfileExit::fault(0x700092f048); }
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700092f054); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(15, 0xfffffffffffc0000u64);
            let r118 = c.r64(15) & c.r64(0);
            c.w64(15, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f070); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f07b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f07c); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092f089, 0x700092f084);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f089); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f08b); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r120 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r120);
            c.w64(15, 0xfffffffffffc0000u64);
            let r121 = c.r64(15) & c.r64(0);
            c.w64(15, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f0a0); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f0af); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f0b0); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092f0bd, 0x700092f0b8);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f0bd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f0bf); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r123 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r123);
            c.w64(15, 0xfffffffffffc0000u64);
            let r124 = c.r64(15) & c.r64(8);
            c.w64(15, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f0d5); };
            c.logic(t125 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f0e4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f0e5); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092f0f2, 0x700092f0ed);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f0f2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f0f4); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x700092f0fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092f114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092f114); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092f118); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092f122); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092f128); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092f12c); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092f130); };
            c.w64(8, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f135); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092f13b); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092f13f); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092f145); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092f14a); };
            c.w32(0, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x700092f14f); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f154); };
            c.w32(12, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700092f158); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092f15c); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092f165); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092f16c); };
            c.w64(12, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f170); };
            c.w32(15, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092f174); };
            c.cmp(t18, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(9).wrapping_mul(4));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f191); };
            c.w32(3, t19);
            let r20 = c.add(c.r32(3), 0x8u64, 32);
            c.w32(3, r20);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x700092f19c); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092f1a1); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            c.w32(6, c.r32(3));
            let r23 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r23);
            c.logic(r23, 32);
            c.cmp(c.r32(8), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092f1b7); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092f1c4); };
            c.cmp(t25, c.r32(3), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f1da); };
            c.w32(6, t26);
            c.w64(7, c.r64(3));
            let r27 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r27);
            c.logic(r27, 64);
            c.w32(0, c.r32(6));
            let r28 = c.r32(0) & 0x3u64;
            c.w32(0, r28);
            c.logic(r28, 32);
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
            let Some(t29) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f20c); };
            c.w32(0, t29);
            let r30 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r30);
            let Some(t31) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f214); };
            c.w32(0, t31);
            let r32 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r32);
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092f21a); };
            c.w32(6, t33);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092f229);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092f22a); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(0, c.r32(3));
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092f239); };
            c.w64(3, t36);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f248); };
            c.w32(0, t37);
            c.w32(3, c.r32(8));
            c.w32(6, c.r32(0));
            let r38 = c.r32(3) & 0x1u64;
            c.w32(3, r38);
            c.logic(r38, 32);
            let r39 = c.r32(6) & 0x3u64;
            c.w32(6, r39);
            c.logic(r39, 32);
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
            let r40 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r40);
            c.logic(r40, 64);
            let r41 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r41);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092f26d); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f272); }
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
            let Some(t42) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092f287); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            blk = 20; continue;
        }
        20 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f28d); };
            c.w32(3, t44);
            let r45 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r45);
            let Some(t46) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f293); };
            c.w32(3, t46);
            c.cmp(c.r16(3), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(3, c.r16(3));
            let r47 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r47);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092f2b0); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f2b6); };
            c.w32(3, t50);
            let r51 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f2be); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f2c4); };
            c.w32(3, t54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f2c7); };
            c.cmp(t55, c.r32(3), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(15, c.r64(8));
            let r56 = c.r64(15) | 0x3u64;
            c.w64(15, r56);
            c.logic(r56, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x700092f2d7); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r57 = c.r64(0) & c.r64(2);
            c.w64(0, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f2eb); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f2f5); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f30b); };
            c.w32(0, t59);
            let r60 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r60);
            let Some(t61) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f312); };
            c.w32(0, t61);
            let r62 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r62);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f324); };
            c.w32(0, t63);
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f330); };
            c.w32(3, t64);
            let r65 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r65);
            let Some(t66) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f336); };
            c.w32(3, t66);
            let r67 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r67);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f349); };
            c.w32(3, t68);
            let r69 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r69);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r70 = c.r64(3) | 0x3u64;
            c.w64(3, r70);
            c.logic(r70, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(3)) { return RegfileExit::fault(0x700092f35b); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r71 = c.r64(15) & c.r64(2);
            c.w64(15, r71);
            c.logic(r71, 64);
            let Some(t72) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f36e); };
            c.logic(t72 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f379); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092f380); };
            c.w64(0, t73);
            blk = 55; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f389); };
            c.w32(3, t74);
            c.w32(6, c.r32(3));
            let r75 = c.r32(6) & 0x3u64;
            c.w32(6, r75);
            c.logic(r75, 32);
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
            let r76 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r76);
            c.logic(r76, 64);
            let r77 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r77);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x700092f3a9); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f3ae); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f3b5); };
            c.w32(3, t78);
            let r79 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r79);
            let Some(t80) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f3bd); };
            c.w32(3, t80);
            let r81 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r81);
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092f3c3); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700092f3d2);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(3).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092f3d3); };
            c.cmp(t83, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(3, c.r64(8));
            blk = 45; continue;
        }
        41 => {
            let Some(t84) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f3de); };
            c.w32(3, t84);
            c.w32(6, c.r32(3));
            let r85 = c.r32(6) & 0x3u64;
            c.w32(6, r85);
            c.logic(r85, 32);
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
            let r86 = c.r64(3) | 0x2u64;
            c.w64(3, r86);
            c.logic(r86, 64);
            let r87 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r87);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x700092f3fe); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f403); }
            blk = 44; continue;
        }
        44 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092f40a); };
            c.w64(3, t88);
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
            let Some(t89) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092f41f); };
            c.w32(6, t89);
            let r90 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r90);
            blk = 48; continue;
        }
        48 => {
            let Some(t91) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f425); };
            c.w32(7, t91);
            let r92 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r92);
            let Some(t93) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f42b); };
            c.w32(7, t93);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r16(7));
            let r94 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r94);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t95) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092f444); };
            c.w32(6, t95);
            let r96 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r96);
            let Some(t97) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f44a); };
            c.w32(7, t97);
            let r98 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r98);
            let Some(t99) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f452); };
            c.w32(6, t99);
            let r100 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r100);
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f458); };
            c.w32(7, t101);
            let Some(t102) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f45b); };
            c.cmp(t102, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r103 = c.r64(3) | 0x3u64;
            c.w64(3, r103);
            c.logic(r103, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(3)) { return RegfileExit::fault(0x700092f464); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r104 = c.r64(15) & c.r64(2);
            c.w64(15, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f477); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f47e); }
            blk = 55; continue;
        }
        54 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092f487); };
            c.w64(9, t106);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700092f48e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f493); }
            blk = 55; continue;
        }
        55 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t107) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092f4a1); };
            c.w64(5, t107);
            let Some(t108) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700092f4a5); };
            c.w64(0, t108);
            c.w64(6, c.r64(12));
            c.w64(7, c.r64(8));
            let r109 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r109);
            return RegfileExit::jump(0x70007fcc14);
        }
        56 => {
            c.w32(3, c.r32(3));
            let r110 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r110);
            c.w64(15, 0xfffffffffffc0000u64);
            let r111 = c.r64(15) & c.r64(3);
            c.w64(15, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f4c7); };
            c.logic(t112 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f4d2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f4d3); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092f4e0, 0x700092f4db);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f4e0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f4e2); }
            blk = 53; continue;
        }
        59 => {
            c.w32(3, c.r32(3));
            let r113 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r113);
            c.w64(15, 0xfffffffffffc0000u64);
            let r114 = c.r64(15) & c.r64(3);
            c.w64(15, r114);
            c.logic(r114, 64);
            let Some(t115) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f4f4); };
            c.logic(t115 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f503); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f504); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x700092f511, 0x700092f50c);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f511); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f513); }
            blk = 33; continue;
        }
        62 => {
            c.w32(15, c.r32(15));
            let r116 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r116);
            c.w64(0, 0xfffffffffffc0000u64);
            let r117 = c.r64(0) & c.r64(15);
            c.w64(0, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f529); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(3, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f537); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f538); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(3));
            return c.call(0x70007f1594, 0x700092f545, 0x700092f540);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f545); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f547); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x700092f550);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092f554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            c.w64(2, c.r64(5));
            let r0 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r0);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700092f55d); }
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092f561); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092f565); };
            c.w64(1, (t2 as u32 as i32 as i64 as u64));
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f56a); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700092f570); };
            c.w64(1, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092f574); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092f57a); };
            c.w32(8, t8);
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f57f); };
            c.w32(9, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092f583); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092f58c); };
            c.w64(3, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092f593); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f597); };
            c.w32(9, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092f59b); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t15) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f5b8); };
            c.w32(11, t15);
            let r16 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r16);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x700092f5c4); }
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092f5c9); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            c.w32(12, c.r32(11));
            let r19 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092f5e1); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092f5ee); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f605); };
            c.w32(12, t22);
            c.w64(15, c.r64(11));
            let r23 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r23);
            c.logic(r23, 64);
            c.w32(0, c.r32(12));
            let r24 = c.r32(0) & 0x3u64;
            c.w32(0, r24);
            c.logic(r24, 32);
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
            let Some(t25) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f63a); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f641); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092f649); };
            c.w32(0, t29);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092f65a);
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092f65b); };
            c.cmp(t30, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(11, c.r32(11));
            let r31 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r31);
            c.logic(r31, 32);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092f66d); };
            c.w64(12, t32);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t33) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f67d); };
            c.w32(11, t33);
            c.w32(12, c.r32(1));
            c.w32(15, c.r32(11));
            let r34 = c.r32(12) & 0x1u64;
            c.w32(12, r34);
            c.logic(r34, 32);
            let r35 = c.r32(15) & 0x3u64;
            c.w32(15, r35);
            c.logic(r35, 32);
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
            let r36 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r36);
            c.logic(r36, 64);
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x700092f6a9); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f6ae); }
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
            let Some(t38) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092f6c4); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            blk = 20; continue;
        }
        20 => {
            let Some(t40) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f6cb); };
            c.w32(12, t40);
            let r41 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r41);
            let Some(t42) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f6d2); };
            c.w32(12, t42);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r43 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r43);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092f6f5); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f6fc); };
            c.w32(12, t46);
            let r47 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f703); };
            c.w32(11, t48);
            let r49 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r49);
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f70a); };
            c.w32(12, t50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f70f); };
            c.cmp(t51, c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(1));
            let r52 = c.r64(9) | 0x3u64;
            c.w64(9, r52);
            c.logic(r52, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x700092f720); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r53 = c.r64(11) & c.r64(3);
            c.w64(11, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f734); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f73f); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t55) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f754); };
            c.w32(11, t55);
            let r56 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r56);
            let Some(t57) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f75b); };
            c.w32(11, t57);
            let r58 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r58);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f771); };
            c.w32(11, t59);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t60) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f77e); };
            c.w32(12, t60);
            let r61 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r61);
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f785); };
            c.w32(12, t62);
            let r63 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r63);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t64) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f79c); };
            c.w32(12, t64);
            let r65 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r65);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r66 = c.r64(12) | 0x3u64;
            c.w64(12, r66);
            c.logic(r66, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x700092f7b0); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r67 = c.r64(9) & c.r64(3);
            c.w64(9, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f7c4); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f7cf); }
            blk = 55; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f7db); };
            c.w32(11, t69);
            c.w32(12, c.r32(11));
            let r70 = c.r32(12) & 0x3u64;
            c.w32(12, r70);
            c.logic(r70, 32);
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
            let r71 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r71);
            c.logic(r71, 64);
            let r72 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r72);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x700092f800); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f805); }
            blk = 37; continue;
        }
        37 => {
            let Some(t73) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f80c); };
            c.w32(11, t73);
            let r74 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r74);
            let Some(t75) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f813); };
            c.w32(11, t75);
            let r76 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r76);
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092f81a); };
            c.w32(12, t77);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700092f82a);
        }
        39 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092f82b); };
            c.cmp(t78, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(11, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t79) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092f836); };
            c.w32(11, t79);
            c.w32(12, c.r32(11));
            let r80 = c.r32(12) & 0x3u64;
            c.w32(12, r80);
            c.logic(r80, 32);
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
            let r81 = c.r64(11) | 0x2u64;
            c.w64(11, r81);
            c.logic(r81, 64);
            let r82 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r82);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x700092f85b); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f860); }
            blk = 44; continue;
        }
        44 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092f867); };
            c.w64(11, t83);
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
            let Some(t84) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092f87d); };
            c.w32(12, t84);
            let r85 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r85);
            blk = 48; continue;
        }
        48 => {
            let Some(t86) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f885); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092f88d); };
            c.w32(15, t88);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(15, c.r16(15));
            let r89 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r89);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092f8ab); };
            c.w32(12, t90);
            let r91 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r91);
            let Some(t92) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f8b3); };
            c.w32(15, t92);
            let r93 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r93);
            let Some(t94) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092f8ba); };
            c.w32(12, t94);
            let r95 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r95);
            let Some(t96) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f8c2); };
            c.w32(15, t96);
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f8c6); };
            c.cmp(t97, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r98 = c.r64(11) | 0x3u64;
            c.w64(11, r98);
            c.logic(r98, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x700092f8d1); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r99 = c.r64(9) & c.r64(3);
            c.w64(9, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f8e5); };
            c.logic(t100 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f8ec); }
            blk = 55; continue;
        }
        54 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092f8f5); };
            c.w64(8, t101);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700092f8fc); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092f901); }
            blk = 55; continue;
        }
        55 => {
            let Some(t102) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700092f908); };
            if !c.push64(t102) { return RegfileExit::fault(0x700092f908); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(2));
            c.w64(7, c.r64(1));
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092f913); };
            c.w64(10, t103);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x700092f91a); }
            return RegfileExit::jump(0x70007f2274);
        }
        56 => {
            c.w32(11, c.r32(11));
            let r104 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r104);
            c.w64(9, 0xfffffffffffc0000u64);
            let r105 = c.r64(9) & c.r64(11);
            c.w64(9, r105);
            c.logic(r105, 64);
            let Some(t106) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f934); };
            c.logic(t106 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f93f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f940); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700092f94d, 0x700092f948);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f94d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f94f); }
            blk = 53; continue;
        }
        59 => {
            c.w32(12, c.r32(12));
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            c.w64(9, 0xfffffffffffc0000u64);
            let r108 = c.r64(9) & c.r64(12);
            c.w64(9, r108);
            c.logic(r108, 64);
            let Some(t109) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f962); };
            c.logic(t109 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f971); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f972); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x700092f97f, 0x700092f97a);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f97f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f981); }
            blk = 33; continue;
        }
        62 => {
            c.w32(9, c.r32(9));
            let r110 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r110);
            c.w64(11, 0xfffffffffffc0000u64);
            let r111 = c.r64(11) & c.r64(9);
            c.w64(11, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092f997); };
            c.logic(t112 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092f9a6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092f9a7); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700092f9b4, 0x700092f9af);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092f9b4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092f9b6); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x700092f9c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092f9d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092f9d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092f9d8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092f9e2); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700092f9e8); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092f9ec); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092f9f0); };
            c.w64(1, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092f9f5); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700092f9fb); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092f9ff); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092fa05); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fa0a); };
            c.w32(11, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700092fa0e); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092fa12); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092fa1b); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092fa22); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fa26); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092fa2a); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092fa47); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092fa52); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092fa57); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092fa6c); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092fa79); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092fa8f); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(2, c.r32(6));
            let r26 = c.r32(2) & 0x3u64;
            c.w32(2, r26);
            c.logic(r26, 32);
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
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fac2); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            let Some(t29) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fac9); };
            c.w32(2, t29);
            let r30 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r30);
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092facf); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092fade);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092fadf); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            let r33 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r33);
            c.logic(r33, 32);
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092faed); };
            c.w64(0, t34);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092fafc); };
            c.w32(2, t35);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(2));
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
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r38 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r38);
            c.logic(r38, 64);
            let r39 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x700092fb21); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092fb26); }
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
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092fb3b); };
            c.w32(2, t40);
            let r41 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r41);
            blk = 20; continue;
        }
        20 => {
            let Some(t42) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fb41); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            let Some(t44) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092fb47); };
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
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t46) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092fb62); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fb68); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fb6f); };
            c.w32(2, t50);
            let r51 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fb75); };
            c.w32(0, t52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fb78); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(2, c.r64(1));
            let r54 = c.r64(2) | 0x3u64;
            c.w64(2, r54);
            c.logic(r54, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700092fb88); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r55 = c.r64(15) & c.r64(3);
            c.w64(15, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fb9b); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092fba6); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fbbb); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092fbc1); };
            c.w32(2, t59);
            let r60 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r60);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fbd4); };
            c.w32(2, t61);
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fbdf); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r63);
            let Some(t64) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092fbe5); };
            c.w32(0, t64);
            let r65 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r65);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fbf7); };
            c.w32(0, t66);
            let r67 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r67);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r68 = c.r64(0) | 0x3u64;
            c.w64(0, r68);
            c.logic(r68, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092fc09); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r69 = c.r64(2) & c.r64(3);
            c.w64(2, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fc1b); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092fc25); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092fc2c); };
            c.w64(2, t71);
            blk = 55; continue;
        }
        34 => {
            let Some(t72) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092fc35); };
            c.w32(0, t72);
            c.w32(6, c.r32(0));
            let r73 = c.r32(6) & 0x3u64;
            c.w32(6, r73);
            c.logic(r73, 32);
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
            let r74 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r74);
            c.logic(r74, 64);
            let r75 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r75);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092fc54); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092fc59); }
            blk = 37; continue;
        }
        37 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fc60); };
            c.w32(0, t76);
            let r77 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r77);
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fc67); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092fc6d); };
            c.w32(6, t80);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700092fc7c);
        }
        39 => {
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092fc7d); };
            c.cmp(t81, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092fc87); };
            c.w32(0, t82);
            c.w32(6, c.r32(0));
            let r83 = c.r32(6) & 0x3u64;
            c.w32(6, r83);
            c.logic(r83, 32);
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
            let r84 = c.r64(0) | 0x2u64;
            c.w64(0, r84);
            c.logic(r84, 64);
            let r85 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r85);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092fca6); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092fcab); }
            blk = 44; continue;
        }
        44 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092fcb2); };
            c.w64(0, t86);
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
            let Some(t87) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092fcc6); };
            c.w32(6, t87);
            let r88 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r88);
            blk = 48; continue;
        }
        48 => {
            let Some(t89) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fccc); };
            c.w32(7, t89);
            let r90 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r90);
            let Some(t91) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092fcd2); };
            c.w32(7, t91);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r16(7));
            let r92 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r92);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t93) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092fceb); };
            c.w32(6, t93);
            let r94 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r94);
            let Some(t95) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fcf1); };
            c.w32(7, t95);
            let r96 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r96);
            let Some(t97) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fcf8); };
            c.w32(6, t97);
            let r98 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r98);
            let Some(t99) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fcfe); };
            c.w32(7, t99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fd01); };
            c.cmp(t100, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r101 = c.r64(0) | 0x3u64;
            c.w64(0, r101);
            c.logic(r101, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700092fd0a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r102 = c.r64(15) & c.r64(3);
            c.w64(15, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fd1c); };
            c.logic(t103 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092fd23); }
            blk = 55; continue;
        }
        54 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092fd2c); };
            c.w64(2, t104);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700092fd33); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092fd38); }
            blk = 55; continue;
        }
        55 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092fd3f); };
            c.w64(3, (t105 as u32 as i32 as i64 as u64));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092fd44); };
            c.w64(2, t106);
            let Some(t107) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092fd48); };
            c.w64(3, t107);
            let Some(t108) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092fd4c); };
            c.w64(5, t108);
            let r109 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r109);
            let Some(t110) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fd54); };
            if !c.push64(t110) { return RegfileExit::fault(0x700092fd54); }
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x700092fd63); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092fd68); };
            c.w64(10, t111);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x700092fd6f); }
            return RegfileExit::jump(0x70007f2274);
        }
        56 => {
            c.w32(0, c.r32(0));
            let r112 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r112);
            c.w64(15, 0xfffffffffffc0000u64);
            let r113 = c.r64(15) & c.r64(0);
            c.w64(15, r113);
            c.logic(r113, 64);
            let Some(t114) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fd88); };
            c.logic(t114 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092fd93); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092fd94); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700092fda1, 0x700092fd9c);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092fda1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092fda3); }
            blk = 53; continue;
        }
        59 => {
            c.w32(0, c.r32(0));
            let r115 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r115);
            c.w64(2, 0xfffffffffffc0000u64);
            let r116 = c.r64(2) & c.r64(0);
            c.w64(2, r116);
            c.logic(r116, 64);
            let Some(t117) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fdb8); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092fdc6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092fdc7); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x700092fdd4, 0x700092fdcf);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092fdd4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092fdd6); }
            blk = 33; continue;
        }
        62 => {
            c.w32(2, c.r32(2));
            let r118 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r118);
            c.w64(15, 0xfffffffffffc0000u64);
            let r119 = c.r64(15) & c.r64(2);
            c.w64(15, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fdeb); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092fdfa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092fdfb); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x700092fe08, 0x700092fe03);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700092fe08); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700092fe0a); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x700092fe14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092fe34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092fe34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092fe38); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092fe42); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700092fe48); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092fe4c); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092fe50); };
            c.w64(1, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092fe55); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700092fe5b); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092fe5f); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x700092fe65); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fe6a); };
            c.w32(11, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700092fe6e); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092fe72); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092fe7b); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092fe82); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092fe86); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092fe8a); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092fea7); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x700092feb2); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092feb7); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700092fecc); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700092fed9); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092feef); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(2, c.r32(6));
            let r26 = c.r32(2) & 0x3u64;
            c.w32(2, r26);
            c.logic(r26, 32);
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
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ff22); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            let Some(t29) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ff29); };
            c.w32(2, t29);
            let r30 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r30);
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092ff2f); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700092ff3e);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x43)) else { return RegfileExit::fault(0x700092ff3f); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            let r33 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r33);
            c.logic(r33, 32);
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ff4d); };
            c.w64(0, t34);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092ff5c); };
            c.w32(2, t35);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(2));
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
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r38 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r38);
            c.logic(r38, 64);
            let r39 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x700092ff81); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ff86); }
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
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092ff9b); };
            c.w32(2, t40);
            let r41 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r41);
            blk = 20; continue;
        }
        20 => {
            let Some(t42) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ffa1); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            let Some(t44) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ffa7); };
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
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t46) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092ffc2); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ffc8); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ffcf); };
            c.w32(2, t50);
            let r51 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ffd5); };
            c.w32(0, t52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092ffd8); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(2, c.r64(1));
            let r54 = c.r64(2) | 0x3u64;
            c.w64(2, r54);
            c.logic(r54, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700092ffe8); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r55 = c.r64(15) & c.r64(3);
            c.w64(15, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700092fffb); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930006); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093001b); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930021); };
            c.w32(2, t59);
            let r60 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r60);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930034); };
            c.w32(2, t61);
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093003f); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r63);
            let Some(t64) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930045); };
            c.w32(0, t64);
            let r65 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r65);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930057); };
            c.w32(0, t66);
            let r67 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r67);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let r68 = c.r64(0) | 0x3u64;
            c.w64(0, r68);
            c.logic(r68, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000930069); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r69 = c.r64(2) & c.r64(3);
            c.w64(2, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093007b); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930085); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093008c); };
            c.w64(2, t71);
            blk = 55; continue;
        }
        34 => {
            let Some(t72) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000930095); };
            c.w32(0, t72);
            c.w32(6, c.r32(0));
            let r73 = c.r32(6) & 0x3u64;
            c.w32(6, r73);
            c.logic(r73, 32);
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
            let r74 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r74);
            c.logic(r74, 64);
            let r75 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r75);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009300b4); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009300b9); }
            blk = 37; continue;
        }
        37 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009300c0); };
            c.w32(0, t76);
            let r77 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r77);
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009300c7); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009300cd); };
            c.w32(6, t80);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x70009300dc);
        }
        39 => {
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009300dd); };
            c.cmp(t81, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009300e7); };
            c.w32(0, t82);
            c.w32(6, c.r32(0));
            let r83 = c.r32(6) & 0x3u64;
            c.w32(6, r83);
            c.logic(r83, 32);
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
            let r84 = c.r64(0) | 0x2u64;
            c.w64(0, r84);
            c.logic(r84, 64);
            let r85 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r85);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000930106); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700093010b); }
            blk = 44; continue;
        }
        44 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000930112); };
            c.w64(0, t86);
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
            let Some(t87) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000930126); };
            c.w32(6, t87);
            let r88 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r88);
            blk = 48; continue;
        }
        48 => {
            let Some(t89) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093012c); };
            c.w32(7, t89);
            let r90 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r90);
            let Some(t91) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930132); };
            c.w32(7, t91);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r16(7));
            let r92 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r92);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t93) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700093014b); };
            c.w32(6, t93);
            let r94 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r94);
            let Some(t95) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930151); };
            c.w32(7, t95);
            let r96 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r96);
            let Some(t97) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930158); };
            c.w32(6, t97);
            let r98 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r98);
            let Some(t99) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093015e); };
            c.w32(7, t99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930161); };
            c.cmp(t100, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r101 = c.r64(0) | 0x3u64;
            c.w64(0, r101);
            c.logic(r101, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700093016a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r102 = c.r64(15) & c.r64(3);
            c.w64(15, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093017c); };
            c.logic(t103 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930183); }
            blk = 55; continue;
        }
        54 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700093018c); };
            c.w64(2, t104);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000930193); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930198); }
            blk = 55; continue;
        }
        55 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700093019f); };
            c.w64(3, (t105 as u32 as i32 as i64 as u64));
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009301a4); };
            c.w64(2, (t106 as u32 as i32 as i64 as u64));
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009301a9); };
            c.w64(8, t107);
            let Some(t108) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009301ad); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009301b1); };
            c.w64(2, t109);
            let Some(t110) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009301b5); };
            c.w64(5, t110);
            let r111 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r111);
            let Some(t112) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70009301bd); };
            if !c.push64(t112) { return RegfileExit::fault(0x70009301bd); }
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x70009301cc); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70009301d1); }
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009301d6); };
            c.w64(10, t113);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x70009301dd); }
            return RegfileExit::jump(0x70007f2274);
        }
        56 => {
            c.w32(0, c.r32(0));
            let r114 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r114);
            c.w64(15, 0xfffffffffffc0000u64);
            let r115 = c.r64(15) & c.r64(0);
            c.w64(15, r115);
            c.logic(r115, 64);
            let Some(t116) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009301f6); };
            c.logic(t116 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000930201); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000930202); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700093020f, 0x700093020a);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700093020f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000930211); }
            blk = 53; continue;
        }
        59 => {
            c.w32(0, c.r32(0));
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(2, 0xfffffffffffc0000u64);
            let r118 = c.r64(2) & c.r64(0);
            c.w64(2, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000930226); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000930234); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000930235); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x7000930242, 0x700093023d);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000930242); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000930244); }
            blk = 33; continue;
        }
        62 => {
            c.w32(2, c.r32(2));
            let r120 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r120);
            c.w64(15, 0xfffffffffffc0000u64);
            let r121 = c.r64(15) & c.r64(2);
            c.w64(15, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000930259); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000930268); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000930269); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x7000930276, 0x7000930271);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000930276); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000930278); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000930280);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000930294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000930294); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000930298); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009302a2); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009302a8); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009302ac); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009302b0); };
            c.w64(8, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009302b5); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009302bb); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009302bf); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009302c5); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009302ca); };
            c.w32(15, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x70009302cf); };
            c.w32(0, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009302d4); };
            c.w32(3, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70009302d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009302db); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009302df); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009302e8); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009302ef); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009302f3); };
            c.w32(6, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009302f6); };
            c.cmp(t18, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000930313); };
            c.w32(7, t19);
            let r20 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x700093031d); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000930321); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, c.r32(7));
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700093032f); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700093033c); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000930349); };
            c.cmp(t25, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700093035f); };
            c.w32(15, t26);
            c.w64(11, c.r64(7));
            let r27 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r27);
            c.logic(r27, 64);
            c.w32(0, c.r32(15));
            let r28 = c.r32(0) & 0x3u64;
            c.w32(0, r28);
            c.logic(r28, 32);
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
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930394); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093039b); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009303a2); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009303b2);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009303b3); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009303bd); };
            c.w64(9, (t35 as u32 as i32 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t36) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70009303c5); };
            c.w64(9, t36);
            let r37 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009303d6); };
            c.w32(9, t38);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r39 = c.r32(11) & 0x1u64;
            c.w32(11, r39);
            c.logic(r39, 32);
            let r40 = c.r32(12) & 0x3u64;
            c.w32(12, r40);
            c.logic(r40, 32);
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
            let r41 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r41);
            c.logic(r41, 64);
            let r42 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r42);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x7000930402); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930407); }
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
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700093041d); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            blk = 20; continue;
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930424); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093042b); };
            c.w32(11, t47);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700093044d); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930454); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093045b); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930462); };
            c.w32(11, t55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930466); };
            c.cmp(t56, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r57 = c.r64(9) | 0x3u64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000930477); };
            c.w64(15, t58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x700093047b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(2);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093048f); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700093049a); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009304a1); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        26 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009304aa); };
            c.w64(11, t62);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009304bd); };
            c.w32(9, t63);
            let r64 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r64);
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009304c4); };
            c.w32(9, t65);
            let r66 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r66);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009304da); };
            c.w32(9, t67);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009304e3); };
            c.w32(12, t68);
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009304ea); };
            c.w32(12, t70);
            let r71 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r71);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930501); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r74 = c.r64(11) | 0x3u64;
            c.w64(11, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000930515); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x7000930519); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r76 = c.r64(9) & c.r64(2);
            c.w64(9, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093052d); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930538); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093053f); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        35 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000930548); };
            c.w64(11, t79);
            blk = 56; continue;
        }
        36 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000930551); };
            c.w32(15, t80);
            c.w32(7, c.r32(15));
            let r81 = c.r32(7) & 0x3u64;
            c.w32(7, r81);
            c.logic(r81, 32);
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
            let r82 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r82);
            c.logic(r82, 64);
            let r83 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x7000930574); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930579); }
            blk = 39; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930580); };
            c.w32(15, t84);
            let r85 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930587); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093058e); };
            c.w32(7, t88);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x700093059e);
        }
        41 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x700093059f); };
            c.cmp(t89, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009305aa); };
            c.w64(9, (t90 as u32 as i32 as i64 as u64));
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009305af); };
            c.w32(12, t91);
            let Some(t92) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70009305b4); };
            c.w64(9, t92);
            c.w32(15, c.r32(12));
            let r93 = c.r32(15) & 0x3u64;
            c.w32(15, r93);
            c.logic(r93, 32);
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
            let r94 = c.r64(12) | 0x2u64;
            c.w64(12, r94);
            c.logic(r94, 64);
            let r95 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r95);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70009305d8); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009305dd); }
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
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009305f3); };
            c.w32(12, t96);
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            blk = 49; continue;
        }
        49 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009305fb); };
            c.w32(15, t98);
            let r99 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r99);
            let Some(t100) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930603); };
            c.w32(15, t100);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r101 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r101);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000930621); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930629); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930630); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930638); };
            c.w32(15, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093063c); };
            c.cmp(t109, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r110 = c.r64(9) | 0x3u64;
            c.w64(9, r110);
            c.logic(r110, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x7000930647); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r111 = c.r64(12) & c.r64(2);
            c.w64(12, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093065b); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930663); }
            blk = 56; continue;
        }
        55 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700093066c); };
            c.w64(9, t113);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x7000930673); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930678); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093067f); };
            c.w64(11, t114);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093068a); };
            c.w64(1, t115);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t116) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000930691); };
            c.w64(5, t116);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r117 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r117);
            return RegfileExit::jump(0x70007fcc74);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            c.w64(12, 0xfffffffffffc0000u64);
            let r119 = c.r64(12) & c.r64(9);
            c.w64(12, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009306b7); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009306c3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009306c4); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x70009306d1, 0x70009306cc);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009306d1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009306d3); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r121 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r121);
            c.w64(9, 0xfffffffffffc0000u64);
            let r122 = c.r64(9) & c.r64(11);
            c.w64(9, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009306e6); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009306f5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009306f6); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000930703, 0x70009306fe);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000930703); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000930705); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r124 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r124);
            c.w64(11, 0xfffffffffffc0000u64);
            let r125 = c.r64(11) & c.r64(9);
            c.w64(11, r125);
            c.logic(r125, 64);
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093071b); };
            c.logic(t126 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700093072a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093072b); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000930738, 0x7000930733);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000930738); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700093073a); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x7000930744);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000930754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000930754); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000930758); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000930762); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000930768); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093076c); };
            c.w32(2, t3);
            let r4 = c.shl(c.r32(2), 0x5u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930775); };
            c.w32(0, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093077a); };
            c.w64(8, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093077e); };
            c.w64(11, (t7 as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2e30)) else { return RegfileExit::fault(0x7000930783); };
            c.w64(3, t8);
            let Some(t9) = c.ld64(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x10)) else { return RegfileExit::fault(0x700093078a); };
            c.w64(3, t9);
            c.w64(15, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000930793); }
            c.w64(6, c.r64(8));
            return c.call(0x700086ef74, 0x700093079f, 0x700093079a);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093079f); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009307a3); };
            c.w64(7, t11);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009307aa); };
            c.w64(2, t12);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0xc));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009307b5); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70009307ba); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009307c1); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009307c5); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009307d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009307f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009307f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009307f8); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000930802); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000930808); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093080c); };
            c.w32(2, t3);
            let r4 = c.shl(c.r32(2), 0x5u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930815); };
            c.w32(8, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093081a); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093081e); };
            c.w64(3, (t7 as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2e30)) else { return RegfileExit::fault(0x7000930823); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(2).wrapping_add(c.r64(1)).wrapping_add(0x10)) else { return RegfileExit::fault(0x700093082a); };
            c.w64(2, t9);
            c.w64(15, c.r64(7).wrapping_add(c.r64(3).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000930833); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000930837); }
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x700086f594, 0x7000930849, 0x7000930844);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000930849); };
            c.w64(7, t10);
            let Some(t11) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093084d); };
            c.w64(8, t11);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000930854); };
            c.w64(11, t12);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(3, c.r64(8).wrapping_add(0x1));
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700093085f); };
            c.w64(3, (t13 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x7000930864); }
            c.w64(0, c.r64(3).wrapping_sub(0x1));
            if !c.st64(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x700093086c); }
            c.w32(2, 0xfu64);
            c.w64(9, c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1));
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700093087a); };
            c.w32(2, t14);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700093087f); };
            c.w64(15, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000930886); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093088a); };
            c.w64(5, t17);
            c.w64(12, c.r64(11));
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000930891); };
            c.w64(0, t18);
            let r19 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r19);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009308a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009308b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            c.w64(1, c.r64(5));
            let r0 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r0);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009308bd); }
            let Some(t1) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009308c1); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009308c5); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009308cc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009308d3); };
            c.w32(11, t6);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009308da); };
            c.w32(15, t7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009308e0); };
            c.w32(0, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x6)) else { return RegfileExit::fault(0x70009308e5); };
            c.w32(9, t9);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70009308f2);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009308f3); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(2));
            return RegfileExit::jump(0x70007fcc14);
        }
        3 => {
            return RegfileExit::jump(0x7000930910);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000930914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000930914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000930918); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000930922); };
            c.w64(8, t1);
            let r2 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r2);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000930928); }
            let Some(t3) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093092c); };
            c.w64(7, t3);
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000930930); };
            c.w32(11, t4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000930936); };
            c.w32(0, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700093093b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700093093f); }
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
            let Some(t6) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930a0f); };
            c.w64(7, t6);
            c.w32(11, 0x58u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000930a19); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000930a1b); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000930a2f, 0x7000930a2a);
        }
        25 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000930a2f); };
            c.w64(0, t8);
            c.w64(7, c.r64(0));
            blk = 79; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t9) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930a45); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930a49); };
            c.w64(11, t10);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c774, 0x7000930a5a, 0x7000930a55);
        }
        27 => {
            blk = 79; continue;
        }
        28 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t11) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930a66); };
            c.w64(0, t11);
            let Some(t12) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930a6a); };
            c.w64(11, t12);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082d2d4, 0x7000930a7b, 0x7000930a76);
        }
        29 => {
            blk = 79; continue;
        }
        30 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t13) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930a87); };
            c.w64(0, t13);
            let Some(t14) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930a8b); };
            c.w64(11, t14);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c2b4, 0x7000930a9c, 0x7000930a97);
        }
        31 => {
            blk = 79; continue;
        }
        32 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t15) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000930aa8); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930aac); };
            c.w64(3, t16);
            let Some(t17) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930ab0); };
            c.w64(11, t17);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c614, 0x7000930ac1, 0x7000930abc);
        }
        33 => {
            blk = 79; continue;
        }
        34 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t18) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000930acd); };
            c.w64(0, t18);
            let Some(t19) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930ad1); };
            c.w64(3, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930ad5); };
            c.w64(11, t20);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c6b4, 0x7000930ae6, 0x7000930ae1);
        }
        35 => {
            blk = 79; continue;
        }
        36 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t21) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930af2); };
            c.w64(0, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930af6); };
            c.w64(11, t22);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700086bf14, 0x7000930b07, 0x7000930b02);
        }
        37 => {
            blk = 79; continue;
        }
        38 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t23) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930b13); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930b17); };
            c.w64(11, t24);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700086ca54, 0x7000930b28, 0x7000930b23);
        }
        39 => {
            blk = 79; continue;
        }
        40 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t25) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930b34); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930b38); };
            c.w64(11, t26);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000869974, 0x7000930b49, 0x7000930b44);
        }
        41 => {
            blk = 79; continue;
        }
        42 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t27) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000930b55); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930b59); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930b5d); };
            c.w64(11, t29);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000869834, 0x7000930b6e, 0x7000930b69);
        }
        43 => {
            blk = 79; continue;
        }
        44 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t30) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000930b7a); };
            c.w64(0, t30);
            let Some(t31) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930b7e); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930b82); };
            c.w64(11, t32);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x70008699f4, 0x7000930b93, 0x7000930b8e);
        }
        45 => {
            blk = 79; continue;
        }
        46 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t33) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930b9f); };
            c.w64(0, t33);
            let Some(t34) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930ba3); };
            c.w64(11, t34);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082fbd4, 0x7000930bb4, 0x7000930baf);
        }
        47 => {
            blk = 79; continue;
        }
        48 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t35) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000930bbc); };
            c.w64(7, t35);
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000930bc0); };
            c.w32(7, t36);
            let r37 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r37);
            c.w64(0, c.r64(7));
            blk = 79; continue;
        }
        49 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000930bd1); };
            c.w64(7, t38);
            c.w64(11, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000930bdc); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000930be0); };
            c.w64(0, t39);
            c.w64(7, c.r64(0));
            blk = 79; continue;
        }
        50 => {
            let Some(t40) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930bef); };
            c.w32(11, t40);
            let r41 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r41);
            let Some(t42) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930bf6); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930bfd); };
            c.w32(0, t44);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x7au64, 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x7000930c0d);
        }
        52 => {
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(0x1ef)) else { return RegfileExit::fault(0x7000930c0e); };
            c.w32(11, t45);
            c.w64(0, c.r64(7));
            blk = 54; continue;
        }
        53 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000930c1a); };
            c.w32(0, t46);
            let r47 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r47);
            blk = 54; continue;
        }
        54 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930c20); };
            c.cmp(t48, c.r32(11), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930c26); };
            c.w32(3, t49);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_a() { blk = 53; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x7000930c35);
        }
        57 => {
            let Some(t50) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930c36); };
            c.w32(11, t50);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x7000930c46);
        }
        59 => {
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000930c47); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000930c4e); };
            c.w32(11, t53);
            let r54 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000930c55); };
            c.cmp(t55, c.r32(11), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, c.r64(11));
            blk = 79; continue;
        }
        61 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x22a0)) else { return RegfileExit::fault(0x7000930c66); };
            c.w64(3, t56);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000930c77, 0x7000930c72);
        }
        62 => {
            blk = 79; continue;
        }
        63 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t57) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930c83); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930c87); };
            c.w64(11, t58);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000821334, 0x7000930c98, 0x7000930c93);
        }
        64 => {
            blk = 79; continue;
        }
        65 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t59) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930ca4); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930ca8); };
            c.w64(11, t60);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082faf4, 0x7000930cb9, 0x7000930cb4);
        }
        66 => {
            blk = 79; continue;
        }
        67 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t61) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000930cc1); };
            c.w64(11, t61);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000930cc5); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930cd3); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r63);
            let Some(t64) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930cda); };
            c.cmp(t64, 0xbau64, 16);
            if c.cc_b() { blk = 83; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000930ce6); };
            c.w64(3, t65);
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000870914, 0x7000930cf8, 0x7000930cf3);
        }
        70 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000930cf8); };
            c.w64(6, t66);
            let Some(t67) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930cfc); };
            c.w32(7, t67);
            let r68 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r68);
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930d02); };
            c.w32(7, t69);
            let r70 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r70);
            let Some(t71) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930d08); };
            c.w32(8, t71);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2du64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::fault(0x7000930d18);
        }
        72 => {
            let Some(t72) = c.ld32(c.r64(7).wrapping_add(0xbb)) else { return RegfileExit::fault(0x7000930d19); };
            c.w32(7, t72);
            let r73 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r73);
            let Some(t74) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930d22); };
            c.w32(8, t74);
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000930d27); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000930d2e); };
            c.w64(3, t76);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000930d3d); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000930d44); };
            c.cmp(t78, c.r64(8), 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x7000930d4d); }
            c.w64(8, c.r64(3).wrapping_add(0x1));
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000930d54); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000930d58); };
            c.w64(7, t79);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000930d5f); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000930d63); }
            c.w64(7, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000930d71); };
            c.w64(11, t80);
            blk = 76; continue;
        }
        76 => {
            let r81 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r81);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000930d7c); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 76; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000930d86); };
            c.w64(7, t82);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000930d8a); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x7000930d8e); }
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(8));
            blk = 79; continue;
        }
        79 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000930d95); };
            c.w64(7, t83);
            let Some(t84) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000930d99); };
            c.w64(8, t84);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t85) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000930da0); };
            c.w64(7, t85);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_add(0xb));
            let Some(t86) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000930dab); };
            c.w32(8, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000930db0); };
            c.w64(15, t87);
            let Some(t88) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000930db7); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000930dbb); };
            c.w64(5, t89);
            c.w64(12, c.r64(7));
            let r90 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r90);
            return RegfileExit::jump(c.r64(1));
        }
        80 => {
            blk = 83; continue;
        }
        81 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000930dca); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x7000930dce); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000930dd2); }
            return c.call(0x70007febf4, 0x7000930ddb, 0x7000930dd6);
        }
        82 => {
            c.w64(8, c.r64(0));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000930dde); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000930de2); };
            c.w64(2, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000930de6); };
            c.w64(7, t93);
            blk = 74; continue;
        }
        83 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000930def); };
            c.w64(3, t94);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000930e00, 0x7000930dfb);
        }
        84 => {
            c.w64(8, c.r64(0));
            blk = 78; continue;
        }
        85 => {
            return RegfileExit::jump(0x7000930e08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000930e14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000930e14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000930e18); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000930e22); };
            c.w64(11, t1);
            let r2 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r2);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000930e29); }
            let Some(t3) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000930e2d); };
            c.w64(8, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000930e31); };
            c.w64(3, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930e36); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld64(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000930e3d); };
            c.w64(3, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930e41); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000930e48); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000930e4d); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x7000930e52); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930e57); };
            c.w32(7, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000930e5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x7000930e5f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000930e63); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000930e67); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000930e70); };
            c.w64(8, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000930e77); };
            c.w64(7, t16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930e7b); };
            c.w32(9, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000930e7f); };
            c.cmp(t18, c.r32(9), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, c.r64(6).wrapping_mul(4));
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000930e94); };
            c.w32(12, t19);
            let r20 = c.add(c.r32(12), 0x8u64, 32);
            c.w32(12, r20);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x7000930ea0); }
            c.w64(6, c.r64(6).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000930ead); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            c.w32(15, c.r32(12));
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000930ec5); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000930ed2); };
            c.cmp(t25, c.r32(12), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(15, c.r32(12));
            let r26 = c.r32(15) & 0x3u64;
            c.w32(15, r26);
            c.logic(r26, 32);
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
            let Some(t27) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930eee); };
            c.w32(15, t27);
            let r28 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r28);
            let Some(t29) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930ef6); };
            c.cmp(t29, 0x5au64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t30) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930f06); };
            c.w32(6, t30);
            let r31 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r31);
            let Some(t32) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930f0c); };
            c.w32(6, t32);
            let r33 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r33);
            let Some(t34) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930f12); };
            c.w32(15, t34);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000930f22);
        }
        12 => {
            let Some(t35) = c.ld32(c.r64(6).wrapping_add(0xb3)) else { return RegfileExit::fault(0x7000930f23); };
            c.cmp(t35, c.r32(0), 32);
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
            let Some(t36) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930f42); };
            c.w32(12, t36);
            let r37 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930f49); };
            c.w32(12, t38);
            let r39 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r39);
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000930f51); };
            c.w32(15, t40);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x7000930f62);
        }
        18 => {
            let Some(t41) = c.ld32(c.r64(12).wrapping_add(0xb3)) else { return RegfileExit::fault(0x7000930f63); };
            c.cmp(t41, c.r32(0), 32);
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
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000930f7e); };
            c.w32(12, t42);
            let r43 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r43);
            blk = 22; continue;
        }
        22 => {
            let Some(t44) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930f86); };
            c.w32(15, t44);
            let r45 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r45);
            let Some(t46) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000930f8e); };
            c.w32(15, t46);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(15, c.r16(15));
            let r47 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r47);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t48) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000930fac); };
            c.w32(12, t48);
            let r49 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r49);
            let Some(t50) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930fb4); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            let Some(t52) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000930fbb); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            let Some(t54) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930fc3); };
            c.w32(15, t54);
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000930fc7); };
            c.cmp(t55, c.r32(15), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(0));
            let r56 = c.r64(9) | 0x3u64;
            c.w64(9, r56);
            c.logic(r56, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(9)) { return RegfileExit::fault(0x7000930fd5); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r57 = c.r64(12) & c.r64(8);
            c.w64(12, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000930fe9); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000930ff5); }
            blk = 38; continue;
        }
        28 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000931002); };
            c.w64(6, t59);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x7000931009); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700093100e); }
            blk = 38; continue;
        }
        29 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x700093101b); };
            c.w64(9, t60);
            let Some(t61) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000931022); };
            c.w64(12, t61);
            c.w64(15, c.r64(12).wrapping_add(0x1c));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x700093102a); };
            c.w64(0, t62);
            let Some(t63) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000931031); };
            c.cmp(t63, c.r64(15), 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700093103a); }
            c.w64(0, c.r64(12).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x7000931042); };
            c.w64(9, t64);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000931049); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000931050); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000931054); }
            if !c.st32(c.r64(0).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x7000931058); }
            if !c.st32(c.r64(0).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x700093105f); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x7000931066); };
            c.w64(9, t65);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700093106d); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x7000931071); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000931078); };
            c.w64(12, t67);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x700093107b); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r68 = c.r64(15) & c.r64(0);
            c.w64(15, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093108f); };
            c.logic(t69 & 0x4u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x700093109a); }
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x700093109d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r70 = c.r64(3) & c.r64(8);
            c.w64(3, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009310af); };
            c.logic(t71 & 0x4u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009310b9); }
            c.w64(12, c.r64(0));
            blk = 36; continue;
        }
        36 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(6, c.r64(7));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009310d4); };
            c.w64(7, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009310d8); };
            c.w64(2, t73);
            c.w64(3, c.r64(12));
            return c.call(0x70007fcd34, 0x70009310e4, 0x70009310df);
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
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009310fc); };
            c.w64(3, t74);
            return c.call(0x70007fccd4, 0x7000931108, 0x7000931103);
        }
        39 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000931108); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093110c); };
            c.w64(11, t76);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t77) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931113); };
            c.w64(8, t77);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0x12));
            let Some(t78) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700093111e); };
            c.w32(11, t78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000931123); };
            c.w64(15, t79);
            let Some(t80) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700093112a); };
            c.w64(1, t80);
            let Some(t81) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093112e); };
            c.w64(5, t81);
            c.w64(12, c.r64(8));
            let r82 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r82);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w32(0, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000931140); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000931144); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000931148); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700093114c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000931150); }
            c.w64(2, c.r64(0));
            return c.call(0x70007fec74, 0x700093115c, 0x7000931157);
        }
        41 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700093115c); };
            c.w64(6, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000931160); };
            c.w64(11, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931164); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000931168); };
            c.w64(7, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700093116c); };
            c.w64(2, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000931170); };
            c.w64(8, t88);
            blk = 31; continue;
        }
        42 => {
            c.w32(0, c.r32(0));
            let r89 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r89);
            c.w64(3, 0xfffffffffffc0000u64);
            let r90 = c.r64(3) & c.r64(0);
            c.w64(3, r90);
            c.logic(r90, 64);
            let Some(t91) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000931188); };
            c.logic(t91 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000931196); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931197); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70009311a4, 0x700093119f);
        }
        44 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009311a4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009311a6); }
            blk = 35; continue;
        }
        45 => {
            c.w32(12, c.r32(12));
            let r92 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r92);
            c.w64(15, 0xfffffffffffc0000u64);
            let r93 = c.r64(15) & c.r64(12);
            c.w64(15, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009311bc); };
            c.logic(t94 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(3, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009311cb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009311cc); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70009311d9, 0x70009311d4);
        }
        47 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009311d9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009311db); }
            blk = 33; continue;
        }
        48 => {
            c.w32(9, c.r32(9));
            let r95 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r95);
            c.w64(12, 0xfffffffffffc0000u64);
            let r96 = c.r64(12) & c.r64(9);
            c.w64(12, r96);
            c.logic(r96, 64);
            let Some(t97) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009311f1); };
            c.logic(t97 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000931201); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931202); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700093120f, 0x700093120a);
        }
        50 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700093120f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000931211); }
            blk = 27; continue;
        }
        51 => {
            return RegfileExit::jump(0x7000931218);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000931234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000931234); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000931238); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000931242); };
            c.w64(11, t1);
            let r2 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r2);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000931249); }
            let Some(t3) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093124d); };
            c.w64(8, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000931251); };
            c.w64(3, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000931256); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld64(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093125d); };
            c.w64(3, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000931261); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000931268); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700093126d); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x7000931272); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931277); };
            c.w32(7, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x700093127b); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700093127f); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(8));
            blk = 3; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700093128d); };
            c.w64(7, t15);
            blk = 3; continue;
        }
        3 => {
            let Some(t16) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000931294); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000931298); };
            c.w64(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700093129f); };
            c.cmp(t18, c.r32(7), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(6).wrapping_mul(4));
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009312b0); };
            c.w32(15, t19);
            let r20 = c.add(c.r32(15), 0x8u64, 32);
            c.w32(15, r20);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70009312bc); }
            c.w64(6, c.r64(6).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009312c9); };
            c.w32(15, t21);
            c.w32(8, c.r32(15));
            let r22 = c.r32(8) & 0xfffffffffffffffdu64;
            c.w32(8, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(7, c.r64(3));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009312ed); };
            c.w64(3, t23);
            c.w64(6, c.r64(9));
            return c.call(0x70007fcd94, 0x70009312fc, 0x70009312f7);
        }
        6 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009312fc); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000931300); };
            c.w64(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931307); };
            c.w64(8, t26);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0x12));
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000931312); };
            c.w32(11, t27);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000931317); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700093131e); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000931322); };
            c.w64(5, t30);
            c.w64(12, c.r64(8));
            let r31 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700093132f); };
            c.cmp(t32, c.r32(15), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(8, c.r32(15));
            let r33 = c.r32(8) & 0x3u64;
            c.w32(8, r33);
            c.logic(r33, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000931345); };
            c.cmp(t34, c.r32(15), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 21; continue;
        }
        11 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(8, c.r64(0));
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931368); };
            c.w32(15, t35);
            let r36 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r36);
            let Some(t37) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093136f); };
            c.w32(15, t37);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700093137c); };
            c.w32(8, t38);
            let r39 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r39);
            blk = 14; continue;
        }
        16 => {
            let r40 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r40);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000931392); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931399); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009313a0); };
            c.w32(15, t45);
            let r46 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r46);
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009313a7); };
            c.w32(8, t47);
            let Some(t48) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009313ab); };
            c.cmp(t48, c.r32(8), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(0));
            let r49 = c.r64(8) | 0x3u64;
            c.w64(8, r49);
            c.logic(r49, 64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(6)), c.r32(8)) { return RegfileExit::fault(0x70009313b8); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r50 = c.r64(12) & c.r64(7);
            c.w64(12, r50);
            c.logic(r50, 64);
            let Some(t51) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009313cc); };
            c.logic(t51 & 0x4u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009313d4); }
            blk = 5; continue;
        }
        21 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009313e0); };
            c.w64(8, t52);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70009313e7); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009313ec); }
            blk = 5; continue;
        }
        22 => {
            c.w32(8, c.r32(8));
            let r53 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r53);
            c.w64(12, 0xfffffffffffc0000u64);
            let r54 = c.r64(12) & c.r64(8);
            c.w64(12, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000931408); };
            c.logic(t55 & 0x2u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(15, c.r64(7).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931414); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700093141e, 0x7000931419);
        }
        24 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700093141e); }
            blk = 20; continue;
        }
        25 => {
            return RegfileExit::jump(0x7000931424);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000931434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000931434); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000931438); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093143e); };
            c.w64(7, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000931442); };
            c.w64(8, (t2 as u32 as i32 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000931447); };
            c.w64(11, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700093144b); };
            c.w64(8, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700093144f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000931453); }
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
            let Some(t5) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931499); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009314a0); };
            c.w32(15, t7);
            let r8 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009314a7); };
            c.w32(11, t9);
            let Some(t10) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009314ad); };
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
            let Some(t11) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009314ec); };
            c.logic(t11 & 0x10u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 88; continue;
        }
        12 => {
            c.w32(11, c.r32(3));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70009314ff); };
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
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700093151c); };
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
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000931562); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 23; continue;
        }
        23 => {
            let r18 = c.r32(3) | 0x8u64;
            c.w32(3, r18);
            c.logic(r18, 32);
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000931572); };
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
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009315af); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009315b5); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t25) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009315c2); };
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
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70009315f0); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 35; continue;
        }
        35 => {
            let r27 = c.r32(11) | 0x8u64;
            c.w32(11, r27);
            c.logic(r27, 32);
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000931601); };
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
            let Some(t30) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093161c); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000931623); };
            c.cmp(t32, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t33) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000931630); };
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
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000931670); };
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70009316be); }
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
            let Some(t43) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009316e8); };
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
            let Some(t46) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x700093171a); };
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700093177c); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000931782); }
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
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009317d7); }
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
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70009317f0); };
            c.w64(0, t59);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009317f7); }
            blk = 82; continue;
        }
        80 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        81 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000931800); };
            c.w64(0, t60);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000931807); }
            blk = 82; continue;
        }
        82 => {
            let Some(t61) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093180b); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093180f); };
            c.w64(7, t62);
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000931813); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093181a); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            c.w64(11, c.r64(9));
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000931824); };
            c.w32(3, t67);
            let Some(t68) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931829); };
            c.w32(2, t68);
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700093182d); };
            c.cmp(t69, c.r32(2), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000931836); };
            c.w64(8, t70);
            blk = 84; continue;
        }
        84 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700093183d); };
            c.cmp(t71, c.r32(8), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t72) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000931846); };
            c.w32(2, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700093184b); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700093185c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000931861); }
            blk = 87; continue;
        }
        87 => {
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700093186d); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000931872); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000931876); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093187a); };
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
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r81 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r81);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700093189b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700093189f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009318a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009318a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009318ab); }
            c.w64(0, c.r64(8));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009318b2); };
            c.w64(6, t82);
            return c.call(0x70008a2374, 0x70009318bb, 0x70009318b6);
        }
        89 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009318bb); };
            c.w64(3, t83);
            c.w64(8, c.r64(0));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009318c2); };
            c.w64(0, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009318c6); };
            c.w64(9, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009318ca); };
            c.w64(7, t86);
            c.w32(1, 0x20u64);
            c.w32(2, 0x60u64);
            blk = 1; continue;
        }
        90 => {
            c.w32(11, c.r32(3));
            let r87 = c.r32(11) | 0x100u64;
            c.w32(11, r87);
            c.logic(r87, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009318e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009318eb); }
            let Some(t88) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009318ef); };
            c.cmp(t88, c.r32(15), 32);
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
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000931910); };
            c.cmp(t89, c.r32(15), 32);
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
            let r90 = c.r32(11) | 0x8u64;
            c.w32(11, r90);
            c.logic(r90, 32);
            let Some(t91) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000931930); };
            c.w32(0, t91);
            let r92 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r92);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        97 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 103; continue;
        }
        98 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000931943); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000931947); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700093194b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700093194f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000931953); }
            c.cmp(c.r16(6), 0x40u64, 16);
            if c.cc_b() { blk = 114; continue; }
            blk = 99; continue;
        }
        99 => {
            blk = 26; continue;
        }
        100 => {
            let r93 = c.r32(11) | 0x200u64;
            c.w32(11, r93);
            c.logic(r93, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r94 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r94);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000931977); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700093197b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700093197f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000931983); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000931987); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700093198b); };
            c.w64(6, t95);
            return c.call(0x70008a2374, 0x7000931994, 0x700093198f);
        }
        101 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000931994); };
            c.w64(3, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000931998); };
            c.w64(8, t97);
            c.w32(2, 0x60u64);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009319a1); };
            c.w64(9, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009319a5); };
            c.w64(7, t99);
            c.w32(1, 0x20u64);
            blk = 1; continue;
        }
        102 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 103; continue;
        }
        103 => {
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009319b6); };
            c.w32(3, t100);
            let r101 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r101);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009319bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009319c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70009319c4); }
            let Some(t102) = c.ld8(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009319c8); };
            c.logic(t102 & 0x20u64, 8);
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
            let r103 = c.r32(3) | c.r32(11);
            c.w32(3, r103);
            c.logic(r103, 32);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w64(6, c.r64(11).wrapping_sub(0x1));
            let r104 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r104);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009319ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009319ee); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009319f2); };
            c.w64(6, t105);
            return c.call(0x70008a0fd4, 0x70009319fb, 0x70009319f6);
        }
        107 => {
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009319fb); };
            c.w64(3, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009319ff); };
            c.w64(8, t107);
            c.w32(2, 0x60u64);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931a08); };
            c.w64(9, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931a0c); };
            c.w64(7, t109);
            c.w32(1, 0x20u64);
            blk = 1; continue;
        }
        108 => {
            let r110 = c.r32(11) | 0x60u64;
            c.w32(11, r110);
            c.logic(r110, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r111 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r111);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000931a2a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931a2e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000931a30); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000931a31); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x1d50)) else { return RegfileExit::fault(0x7000931a3c); };
            c.w64(3, t112);
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000931a4e, 0x7000931a49);
        }
        109 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931a4e); };
            c.w64(7, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931a52); };
            c.w64(9, t114);
            blk = 82; continue;
        }
        110 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r115 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r115);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000931a67); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931a6b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000931a6d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000931a6e); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000931a74); };
            c.w64(3, t116);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000931a88, 0x7000931a83);
        }
        111 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931a88); };
            c.w64(7, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931a8c); };
            c.w64(9, t118);
            blk = 82; continue;
        }
        112 => {
            let r119 = c.r32(11) | 0x6u64;
            c.w32(11, r119);
            c.logic(r119, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r120 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r120);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000931aa5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931aa9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000931aab); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000931aac); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x7000931ab2); };
            c.w64(3, t121);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000931ac6, 0x7000931ac1);
        }
        113 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931ac6); };
            c.w64(7, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931aca); };
            c.w64(9, t123);
            blk = 82; continue;
        }
        114 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r124 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r124);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000931ada); }
            c.w64(2, c.r64(8));
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000931ae1); };
            c.w64(6, t125);
            return c.call(0x70007fb534, 0x7000931aea, 0x7000931ae5);
        }
        115 => {
            let Some(t126) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000931aea); };
            c.logic(t126 & 0x20u64, 8);
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
            let Some(t127) = c.ld8(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000931afc); };
            c.logic(t127 & 0x20u64, 8);
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
            let Some(t128) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000931b13); };
            c.w32(11, t128);
            let r129 = c.r32(8) | c.r32(7);
            c.w32(8, r129);
            c.logic(r129, 32);
            let r130 = c.r32(8) | c.r32(11);
            c.w32(8, r130);
            c.logic(r130, 32);
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000931b20); }
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931b24); };
            c.w64(7, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931b28); };
            c.w64(9, t132);
            blk = 82; continue;
        }
        122 => {
            let r133 = c.r32(11) | 0x100u64;
            c.w32(11, r133);
            c.logic(r133, 32);
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r134 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r134);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000931b44); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000931b48); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931b49); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000931b4b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000931b4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000931b53); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x7000931b59); };
            c.w64(3, t135);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000931b6d, 0x7000931b68);
        }
        123 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931b6d); };
            c.w64(7, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931b71); };
            c.w64(9, t137);
            blk = 82; continue;
        }
        124 => {
            return RegfileExit::jump(0x7000931b7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000931b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000931b97); };
            c.w64(8, (t0 as u32 as i32 as i64 as u64));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000931b9c); };
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
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931bc7); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000931bce); };
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
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000931be4); };
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
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931bfd); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000931c04); };
            c.w32(0, t8);
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_ae() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000931c10); };
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
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000931c4c); };
            c.cmp(t10, c.r32(8), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(8, 0x210u64);
            blk = 61; continue;
        }
        15 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000931c64); };
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
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931c97); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000931c9e); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931cb4); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000931cb8); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000931cc5); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000931ccb); }
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000931cf0); }
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
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931d21); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000931d28); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000931d31); };
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
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000931d52); }
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
            c.w32(8, 0x2u64);
            blk = 61; continue;
        }
        42 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931d93); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000931d9a); };
            c.cmp(t25, c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t26) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000931da7); };
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
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000931ddf); };
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000931e19); }
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
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000931e2d); };
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
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000931e44); };
            c.w64(0, t29);
            blk = 65; continue;
        }
        65 => {
            let Some(t30) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000931e4b); };
            c.w64(11, t30);
            let Some(t31) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931e4f); };
            c.w64(7, t31);
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000931e53); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000931e5a); };
            c.w32(11, t34);
            let r35 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r35);
            c.w64(3, c.r64(9));
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000931e64); };
            c.w32(2, t36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931e68); };
            c.w32(1, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000931e6c); };
            c.cmp(t38, c.r32(1), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000931e75); };
            c.w64(11, t39);
            blk = 67; continue;
        }
        67 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000931e7c); };
            c.cmp(t40, c.r32(11), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000931e85); };
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
            if !c.st32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000931e9a); }
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000931e9f); }
            blk = 70; continue;
        }
        70 => {
            c.w64(9, c.r64(3).wrapping_add(0x9));
            let Some(t43) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000931eab); };
            c.w32(8, t43);
            let Some(t44) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000931eb0); };
            c.w64(1, t44);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        71 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000931eb9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000931ebd); }
            let r45 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r45);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r46 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r46);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000931eca); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000931ece); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000931ed0); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000931ed1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000931ed5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000931ed9); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000931edd); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000931ef0, 0x7000931eeb);
        }
        72 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931ef0); };
            c.w64(7, t48);
            c.w32(8, 0x100u64);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000931efa); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931efe); };
            c.w64(9, t50);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000931f05); }
            blk = 65; continue;
        }
        73 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000931f0b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000931f0f); }
            let r51 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000931f15); };
            c.w32(11, t52);
            let r53 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r53);
            let Some(t54) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000931f1c); };
            c.w32(11, t54);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000931f21); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000931f25); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000931f29); }
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000931f37); }
            blk = 35; continue;
        }
        75 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let r55 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r55);
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
            let r56 = c.r32(11) | c.r32(3);
            c.w32(11, r56);
            c.logic(r56, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000931f73); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000931f77); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007fb534, 0x7000931f85, 0x7000931f80);
        }
        82 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000931f85); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931f89); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000931f8d); };
            c.w64(15, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000931f91); };
            c.w64(9, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000931f98); }
            blk = 65; continue;
        }
        83 => {
            return RegfileExit::jump(0x7000931fa0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000931fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000931fb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000931fb8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000931fbe); };
            c.w64(7, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000931fc3); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000931fcb); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000931fd2); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000931fd6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000931fda); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000931fe8); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000931fec); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000931ff0); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000931ff4); };
            c.w64(3, t9);
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093200e); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932015); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093202a); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000932056); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700093205d); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700093206b); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000932070); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000932079); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700093208a); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r20 = c.r32(7) | 0x6u64;
            c.w32(7, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r21 = c.r32(7) | 0x2u64;
            c.w32(7, r21);
            c.logic(r21, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70009320b3); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70009320bc); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009320c6); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009320ca); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009320ce); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009320d5); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009320dc); };
            c.w64(11, t30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009320e0); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009320e5); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009320e9); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009320f2); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009320f9); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000932102); };
            c.w32(2, t36);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r37 = c.r32(1) | c.r32(6);
            c.w32(1, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000932114); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000932119); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000932125); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093212a); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700093212e); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000932132); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093213f); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932144); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000932148); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700093214c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000932150); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932154); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932175); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093217e); };
            c.cmp(t45, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(7, c.r32(7));
            let r46 = c.r32(7) | 0x1eu64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000932194); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009321a0); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x70009321ac, 0x70009321a7);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009321ac); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009321b0); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009321b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009321b8); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009321bc); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009321c0); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x70009321c9, 0x70009321c4);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009321c9); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009321cd); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009321d1); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009321da); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x70009321e9, 0x70009321e4);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009321e9); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009321ed); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009321f1); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009321fa); };
            c.w32(9, t55);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932224); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932233, 0x700093222e);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000932233); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932237); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700093223b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093223f); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000932243); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000932247); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x7000932250, 0x700093224b);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932260); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700093226c, 0x7000932267);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932276); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932283); };
            c.w32(9, t61);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009322a5); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009322b1, 0x70009322ac);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009322bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009322bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009322c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009322c7); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009322cb); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009322de); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009322ea, 0x70009322e5);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009322f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009322f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009322fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932300); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932304); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932316); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932325, 0x7000932320);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093232a); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093232e); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932337); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700093233b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700093233f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932343); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932347); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932359); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932365, 0x7000932360);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932375); }
            c.w32(11, 0x28u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700093237f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932381); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932382); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000932388); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000932397, 0x7000932392);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093239c); };
            c.w64(8, t67);
            blk = 20; continue;
        }
        63 => {
            c.w32(7, c.r32(7));
            let r68 = c.r32(7) | 0x60u64;
            c.w32(7, r68);
            c.logic(r68, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70009323b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70009323b4); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb9f4, 0x70009323c0, 0x70009323bb);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009323c0); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009323c4); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009323d3); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009323dd); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009323df); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009323e0); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70009323e1); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009323f2, 0x70009323ed);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009323f7); };
            c.w64(8, t72);
            blk = 20; continue;
        }
        67 => {
            c.w32(7, c.r32(7));
            let r73 = c.r32(7) | 0x100u64;
            c.w32(7, r73);
            c.logic(r73, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000932411); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093241b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093241d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700093241e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700093241f); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000932423); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932434, 0x700093242f);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932434); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932438); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000932444); }
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700093244d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093244e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700093244f); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000932450); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932461, 0x700093245c);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932466); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700093246f); }
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000932478); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932479); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093247a); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700093247b); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700093248c, 0x7000932487);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932491); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700093249d); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009324a7); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009324a9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009324aa); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009324ab); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009324bc, 0x70009324b7);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009324c1); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009324ca); }
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009324d3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009324d4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009324d5); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009324d6); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009324e7, 0x70009324e2);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009324ec); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x70009324f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000932514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000932514); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000932518); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093251e); };
            c.w64(7, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000932523); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700093252b); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000932532); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000932536); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700093253a); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000932548); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700093254c); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932550); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000932554); };
            c.w64(3, t9);
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093256e); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932575); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093258a); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009325b6); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009325bd); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70009325cb); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009325d0); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70009325d9); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009325ea); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r20 = c.r32(7) | 0x6u64;
            c.w32(7, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r21 = c.r32(7) | 0x2u64;
            c.w32(7, r21);
            c.logic(r21, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000932613); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700093261c); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000932626); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093262a); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093262e); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000932635); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093263c); };
            c.w64(11, t30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000932640); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000932645); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000932649); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000932652); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000932659); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000932662); };
            c.w32(2, t36);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r37 = c.r32(1) | c.r32(6);
            c.w32(1, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000932674); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000932679); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000932685); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093268a); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700093268e); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000932692); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093269f); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009326a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009326a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009326ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009326b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009326b4); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009326d5); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009326de); };
            c.cmp(t45, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(7, c.r32(7));
            let r46 = c.r32(7) | 0x1eu64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70009326f4); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932700); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x700093270c, 0x7000932707);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700093270c); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932710); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932714); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000932718); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700093271c); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000932720); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x7000932729, 0x7000932724);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932729); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093272d); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000932731); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700093273a); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000932749, 0x7000932744);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932749); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093274d); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000932751); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093275a); };
            c.w32(9, t55);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932784); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932793, 0x700093278e);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000932793); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932797); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700093279b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093279f); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009327a3); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009327a7); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x70009327b0, 0x70009327ab);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009327c0); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x70009327cc, 0x70009327c7);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009327d6); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009327e3); };
            c.w32(9, t61);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932805); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932811, 0x700093280c);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700093281b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700093281f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000932823); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932827); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093282b); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700093283e); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700093284a, 0x7000932845);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932854); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000932858); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700093285c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932860); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932864); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932876); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932885, 0x7000932880);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093288a); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093288e); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932897); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700093289b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700093289f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009328a3); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009328a7); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009328b9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009328c5, 0x70009328c0);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009328d5); }
            c.w32(11, 0x24u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009328df); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009328e1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009328e2); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70009328e8); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70009328f7, 0x70009328f2);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009328fc); };
            c.w64(8, t67);
            blk = 20; continue;
        }
        63 => {
            c.w32(7, c.r32(7));
            let r68 = c.r32(7) | 0x60u64;
            c.w32(7, r68);
            c.logic(r68, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000932910); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000932914); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb774, 0x7000932920, 0x700093291b);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932920); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932924); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000932933); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093293d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093293f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932940); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000932941); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932952, 0x700093294d);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932957); };
            c.w64(8, t72);
            blk = 20; continue;
        }
        67 => {
            c.w32(7, c.r32(7));
            let r73 = c.r32(7) | 0x100u64;
            c.w32(7, r73);
            c.logic(r73, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000932971); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093297b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093297d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700093297e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700093297f); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000932983); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932994, 0x700093298f);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932994); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932998); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70009329a4); }
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009329ad); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009329ae); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009329af); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009329b0); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009329c1, 0x70009329bc);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009329c6); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009329cf); }
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009329d8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009329d9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009329da); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009329db); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009329ec, 0x70009329e7);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009329f1); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009329fd); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000932a07); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932a09); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932a0a); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000932a0b); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932a1c, 0x7000932a17);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932a21); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932a2a); }
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000932a33); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932a34); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932a35); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000932a36); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932a47, 0x7000932a42);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932a4c); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x7000932a58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000932a74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000932a74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000932a78); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000932a7e); };
            c.w64(7, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000932a83); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000932a8b); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000932a92); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000932a96); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000932a9a); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000932aa8); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000932aac); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932ab0); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000932ab4); };
            c.w64(3, t9);
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000932ace); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932ad5); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000932aea); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000932b16); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932b1d); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000932b2b); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000932b30); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000932b39); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000932b4a); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r20 = c.r32(7) | 0x6u64;
            c.w32(7, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r21 = c.r32(7) | 0x2u64;
            c.w32(7, r21);
            c.logic(r21, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000932b73); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000932b7c); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000932b86); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932b8a); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000932b8e); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000932b95); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000932b9c); };
            c.w64(11, t30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000932ba0); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000932ba5); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000932ba9); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000932bb2); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000932bb9); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000932bc2); };
            c.w32(2, t36);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r37 = c.r32(1) | c.r32(6);
            c.w32(1, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000932bd4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000932bd9); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000932be5); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000932bea); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000932bee); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000932bf2); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932bff); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932c04); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000932c08); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000932c0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000932c10); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932c14); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932c35); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932c3e); };
            c.cmp(t45, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(7, c.r32(7));
            let r46 = c.r32(7) | 0x1eu64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000932c54); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932c60); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x7000932c6c, 0x7000932c67);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000932c6c); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932c70); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932c74); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000932c78); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000932c7c); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000932c80); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x7000932c89, 0x7000932c84);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932c89); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000932c8d); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000932c91); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932c9a); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000932ca9, 0x7000932ca4);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932ca9); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000932cad); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000932cb1); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932cba); };
            c.w32(9, t55);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932ce4); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932cf3, 0x7000932cee);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000932cf3); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932cf7); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932cfb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000932cff); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000932d03); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000932d07); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x7000932d10, 0x7000932d0b);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932d20); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000932d2c, 0x7000932d27);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000932d36); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932d43); };
            c.w32(9, t61);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932d65); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932d71, 0x7000932d6c);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932d7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000932d7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000932d83); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932d87); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932d8b); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932d9e); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932daa, 0x7000932da5);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932db4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000932db8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000932dbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932dc0); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932dc4); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932dd6); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932de5, 0x7000932de0);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000932dea); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000932dee); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000932df7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000932dfb); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000932dff); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000932e03); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000932e07); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932e19); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000932e25, 0x7000932e20);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000932e35); }
            c.w32(11, 0x2au64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000932e3f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932e41); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932e42); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000932e48); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000932e57, 0x7000932e52);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932e5c); };
            c.w64(8, t67);
            blk = 20; continue;
        }
        63 => {
            c.w32(7, c.r32(7));
            let r68 = c.r32(7) | 0x60u64;
            c.w32(7, r68);
            c.logic(r68, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000932e70); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000932e74); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fbb34, 0x7000932e80, 0x7000932e7b);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932e80); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932e84); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000932e93); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000932e9d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932e9f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932ea0); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000932ea1); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932eb2, 0x7000932ead);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932eb7); };
            c.w64(8, t72);
            blk = 20; continue;
        }
        67 => {
            c.w32(7, c.r32(7));
            let r73 = c.r32(7) | 0x100u64;
            c.w32(7, r73);
            c.logic(r73, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000932ed1); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000932edb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932edd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932ede); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000932edf); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000932ee3); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932ef4, 0x7000932eef);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000932ef4); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932ef8); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000932f04); }
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000932f0d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932f0e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932f0f); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000932f10); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932f21, 0x7000932f1c);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932f26); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932f2f); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000932f38); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932f39); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932f3a); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000932f3b); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932f4c, 0x7000932f47);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932f51); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932f5d); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000932f67); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932f69); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932f6a); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000932f6b); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932f7c, 0x7000932f77);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932f81); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000932f8a); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000932f93); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000932f94); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000932f95); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000932f96); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000932fa7, 0x7000932fa2);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000932fac); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x7000932fb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000932fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000932fd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000932fd8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000932fde); };
            c.w64(7, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000932fe3); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000932feb); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000932ff2); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000932ff6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000932ffa); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000933008); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700093300c); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933010); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000933014); };
            c.w64(3, t9);
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093302e); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000933035); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093304a); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933076); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700093307d); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700093308b); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000933090); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000933099); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009330aa); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r20 = c.r32(7) | 0x6u64;
            c.w32(7, r20);
            c.logic(r20, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r21 = c.r32(7) | 0x2u64;
            c.w32(7, r21);
            c.logic(r21, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70009330d3); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70009330dc); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009330e6); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009330ea); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009330ee); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009330f5); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009330fc); };
            c.w64(11, t30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000933100); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933105); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000933109); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933112); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933119); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000933122); };
            c.w32(2, t36);
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(2));
            let r37 = c.r32(1) | c.r32(6);
            c.w32(1, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000933134); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000933139); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000933145); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093314a); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700093314e); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933152); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093315f); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000933164); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000933168); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700093316c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000933170); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000933174); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000933195); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093319e); };
            c.cmp(t45, 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, c.r64(1));
            blk = 33; continue;
        }
        32 => {
            c.w32(7, c.r32(7));
            let r46 = c.r32(7) | 0x1eu64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70009331b4); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009331c0); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x70009331cc, 0x70009331c7);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009331cc); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009331d0); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009331d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009331d8); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009331dc); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009331e0); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x70009331e9, 0x70009331e4);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009331e9); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009331ed); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009331f1); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009331fa); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x7000933209, 0x7000933204);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000933209); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093320d); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000933211); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093321a); };
            c.w32(9, t55);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000933244); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000933253, 0x700093324e);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000933253); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933257); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700093325b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093325f); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000933263); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000933267); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x7000933270, 0x700093326b);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000933280); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700093328c, 0x7000933287);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000933296); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009332a3); };
            c.w32(9, t61);
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
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009332c5); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009332d1, 0x70009332cc);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009332db); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009332df); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009332e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009332e7); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009332eb); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009332fe); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700093330a, 0x7000933305);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000933314); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000933318); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700093331c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000933320); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933324); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000933336); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000933345, 0x7000933340);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093334a); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093334e); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000933357); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700093335b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700093335f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000933363); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933367); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000933379); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000933385, 0x7000933380);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000933395); }
            c.w32(11, 0x26u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700093339f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009333a1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009333a2); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70009333a8); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70009333b7, 0x70009333b2);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009333bc); };
            c.w64(8, t67);
            blk = 20; continue;
        }
        63 => {
            c.w32(7, c.r32(7));
            let r68 = c.r32(7) | 0x60u64;
            c.w32(7, r68);
            c.logic(r68, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70009333d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70009333d4); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb8b4, 0x70009333e0, 0x70009333db);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009333e0); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009333e4); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009333f3); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009333fd); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009333ff); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000933400); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000933401); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000933412, 0x700093340d);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933417); };
            c.w64(8, t72);
            blk = 20; continue;
        }
        67 => {
            c.w32(7, c.r32(7));
            let r73 = c.r32(7) | 0x100u64;
            c.w32(7, r73);
            c.logic(r73, 32);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000933431); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093343b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093343d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700093343e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700093343f); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000933443); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000933454, 0x700093344f);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000933454); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933458); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000933464); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700093346d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093346e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700093346f); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000933470); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000933481, 0x700093347c);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933486); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700093348f); }
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000933498); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000933499); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093349a); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700093349b); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009334ac, 0x70009334a7);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009334b1); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009334bd); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009334c7); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009334c9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009334ca); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009334cb); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009334dc, 0x70009334d7);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009334e1); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009334ea); }
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009334f3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009334f4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009334f5); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70009334f6); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000933507, 0x7000933502);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093350c); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x7000933518);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000933534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933534); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933538); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093353e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933542); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000933546); };
            c.w64(3, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093354b); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000933552); };
            c.w64(3, t6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933556); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700093355d); };
            c.w32(2, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933562); };
            c.w32(1, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x7000933566); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700093356a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700093356e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x7000933572); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000933576); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700093357a); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933583); };
            c.w64(11, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093358a); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093358e); };
            c.w32(6, t14);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700093359c); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009335a0); };
            c.cmp(t15, c.r32(6), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009335b5); };
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
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009335cb); };
            c.cmp(t19, c.r32(12), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70009335d8); };
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
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009335f9); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            blk = 10; continue;
        }
        10 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009335ff); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933606); };
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
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000933625); };
            c.w32(3, t27);
            let r28 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r28);
            let Some(t29) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093362b); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933632); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000933638); };
            c.w32(12, t33);
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093363d); };
            c.cmp(t34, c.r32(12), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(3, c.r64(0));
            let r35 = c.r64(3) | 0x3u64;
            c.w64(3, r35);
            c.logic(r35, 64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(6)), c.r32(3)) { return RegfileExit::fault(0x700093364a); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r36 = c.r64(2) & c.r64(11);
            c.w64(2, r36);
            c.logic(r36, 64);
            let Some(t37) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093365d); };
            c.logic(t37 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000933667); }
            blk = 17; continue;
        }
        16 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000933671); };
            c.w64(3, t38);
            if !c.st32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x7000933678); }
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700093367d); }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r39 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700093368c); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933699); };
            c.w32(3, t40);
            let r41 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r41);
            let Some(t42) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093369f); };
            c.cmp(t42, 0xbau64, 16);
            if c.cc_b() { blk = 48; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009336ab); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xd08)) else { return RegfileExit::fault(0x70009336af); };
            c.w64(3, t43);
            c.w64(6, c.r64(1));
            return c.call(0x7000870914, 0x70009336be, 0x70009336b9);
        }
        20 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009336be); };
            c.w64(6, t44);
            let Some(t45) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009336c2); };
            c.w32(8, t45);
            let r46 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r46);
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009336c9); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009336d0); };
            c.w32(11, t49);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xe4u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x70009336e3);
        }
        22 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(0x397)) else { return RegfileExit::fault(0x70009336e4); };
            c.cmp(t50, c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70009336f1); };
            c.cmp(t51, c.r32(0), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009336fe); };
            c.cmp(t52, c.r32(0), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093370b); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700093370f); };
            c.w64(11, t54);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000933713); }
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000933717); };
            if !c.push64(t55) { return RegfileExit::fault(0x7000933717); }
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700093371a); };
            if !c.push64(t56) { return RegfileExit::fault(0x700093371a); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x700093372d, 0x7000933728);
        }
        26 => {
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700093372d); };
            c.cmp(t57, c.r32(0), 32);
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
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700093373d); };
            c.cmp(t58, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933746); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700093374d); };
            c.logic(t61 & 0x10u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000933754); };
            c.cmp(t62, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t63) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933761); };
            c.cmp(t63, 0x41u64, 16);
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
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000933771); };
            c.w64(0, t64);
            c.w64(8, c.r64(0));
            blk = 37; continue;
        }
        35 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700093377d); };
            c.w64(0, t65);
            c.w64(8, c.r64(0));
            blk = 37; continue;
        }
        36 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000933789); };
            c.w64(8, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093378d); };
            c.w64(11, t67);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000933791); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000933795); };
            if !c.push64(t68) { return RegfileExit::fault(0x7000933795); }
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000933798); };
            if !c.push64(t69) { return RegfileExit::fault(0x7000933798); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            return c.call(0x70008a8174, 0x70009337a8, 0x70009337a3);
        }
        37 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009337a8); };
            c.w64(11, t70);
            let Some(t71) = c.ld64(c.r64(11).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009337ac); };
            c.w64(8, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009337b0); };
            c.w64(11, t72);
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t73) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009337b8); };
            c.w32(11, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009337bd); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009337c1); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009337c5); };
            c.w64(5, t76);
            c.w64(12, c.r64(8));
            let r77 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r77);
            return RegfileExit::jump(c.r64(1));
        }
        38 => {
            blk = 48; continue;
        }
        39 => {
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009337d4); };
            c.logic(t78 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 34; continue;
        }
        41 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009337df); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 34; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 35; continue;
        }
        43 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009337f0); };
            c.w64(2, t79);
            let Some(t80) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009337f4); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            let Some(t82) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009337fb); };
            c.logic(t82 & 0x2u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000933802); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933806); };
            c.w64(11, t84);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700093380a); }
            c.w32(8, 0xa4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000933814); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000933816); };
            c.w64(3, t85);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000933827, 0x7000933822);
        }
        45 => {
            return RegfileExit::fault(0x7000933827);
        }
        46 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933828); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700093382c); };
            c.w64(11, t87);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000933830); }
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000933834); };
            c.w64(0, t88);
            return c.call(0x7000857594, 0x700093383d, 0x7000933838);
        }
        47 => {
            blk = 37; continue;
        }
        48 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000933842); }
            c.w32(8, 0xaeu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093384c); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700093384e); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000933862, 0x700093385d);
        }
        49 => {
            return RegfileExit::fault(0x7000933862);
        }
        50 => {
            c.w32(3, c.r32(3));
            let r90 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r90);
            c.w64(2, 0xfffffffffffc0000u64);
            let r91 = c.r64(2) & c.r64(3);
            c.w64(2, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000933872); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000933880); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000933881); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700093388e, 0x7000933889);
        }
        52 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700093388e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000933890); }
            blk = 15; continue;
        }
        53 => {
            return RegfileExit::jump(0x7000933898);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009338b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009338b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009338b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009338be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009338c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009338c6); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009338cb); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009338d2); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009338d6); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009338dd); };
            c.w32(3, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009338e2); };
            c.w32(2, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009338e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009338ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009338ee); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009338f2); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009338fb); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000933906); };
            c.w64(1, t13);
            let r14 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r14);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000933912); }
            c.w64(2, c.r64(0));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(8));
            return c.call(0x70008436f4, 0x700093392a, 0x7000933925);
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093392a); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093392e); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933932); };
            c.w64(8, t17);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700093393a); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093393f); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000933943); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933947); };
            c.w64(5, t21);
            c.w64(12, c.r64(7));
            let r22 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000933958);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000933974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933974); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933978); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933982); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000933986); };
            c.w64(7, t2);
            let r3 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r3);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700093398c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000933990); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000933994); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000933998); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700093399c); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a1af4, 0x70009339a8, 0x70009339a3);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009339a8); };
            c.w64(3, t4);
            let Some(t5) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009339ac); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009339b0); };
            c.w64(7, t6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(c.r64(7)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009339b4); };
            c.w64(8, (t7 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70009339b9); }
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009339c1); };
            c.w32(3, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009339c6); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009339ca); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009339ce); };
            c.w64(5, t11);
            c.w64(12, c.r64(2));
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70009339d5); };
            c.w64(0, t12);
            let r13 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009339e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009339f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009339f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009339f8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009339fe); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000933a02); };
            c.w64(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000933a06); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000933a0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000933a0e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933a16); };
            c.w32(7, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000933a19); };
            c.cmp(t4, c.r32(7), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0xeu64);
            blk = 6; continue;
        }
        3 => {
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let r5 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r5);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000933a2f); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a1014, 0x7000933a3b, 0x7000933a36);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933a3b); };
            c.w64(3, t6);
            c.w32(2, 0xfeu64);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933a44); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933a48); };
            c.w64(9, t8);
            blk = 6; continue;
        }
        5 => {
            c.w32(2, 0x2u64);
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933a53); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933a57); };
            c.w64(3, t10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000933a5b); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933a61); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            c.w64(8, c.r64(9));
            let Some(t15) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000933a6a); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933a6f); };
            c.w32(1, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000933a72); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933a7b); };
            c.w64(7, t18);
            blk = 8; continue;
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933a82); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000933a8b); };
            c.w32(1, t20);
            c.w32(2, c.r32(2));
            c.w32(6, c.r32(1));
            let r21 = c.r32(2) | c.r32(6);
            c.w32(2, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000933a9d); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000933aa2); }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t22) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000933aad); };
            c.w32(2, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000933ab2); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933ab6); };
            c.w64(5, t24);
            c.w64(12, c.r64(3));
            let r25 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        12 => {
            return RegfileExit::jump(0x7000933ac4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000933ad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933ad4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933ad8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933ade); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000933ae2); };
            c.w64(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000933ae6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000933aea); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000933aee); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933af6); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000933afc); };
            c.cmp(t5, c.r32(7), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0xeu64);
            blk = 8; continue;
        }
        3 => {
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933b0c); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let r7 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r7);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000933b19); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x7000933b25, 0x7000933b20);
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933b25); };
            c.w64(3, t8);
            c.w32(2, 0xfeu64);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933b2e); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933b32); };
            c.w64(9, t10);
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
            let Some(t11) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933b44); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933b48); };
            c.w64(3, t12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000933b4c); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933b52); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            c.w64(8, c.r64(9));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000933b5b); };
            c.w32(11, t17);
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933b60); };
            c.w32(1, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000933b63); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933b6c); };
            c.w64(7, t20);
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933b73); };
            c.cmp(t21, c.r32(7), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000933b7c); };
            c.w32(1, t22);
            c.w32(2, c.r32(2));
            c.w32(6, c.r32(1));
            let r23 = c.r32(2) | c.r32(6);
            c.w32(2, r23);
            c.logic(r23, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000933b8e); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000933b93); }
            blk = 13; continue;
        }
        13 => {
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t24) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000933b9e); };
            c.w32(2, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000933ba3); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933ba7); };
            c.w64(5, t26);
            c.w64(12, c.r64(3));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            return RegfileExit::jump(0x7000933bb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000933bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933bd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933bd8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933be2); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000933be6); };
            c.w64(7, t2);
            let r3 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r3);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000933bec); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000933bf0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000933bf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000933bf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000933bfc); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a2054, 0x7000933c08, 0x7000933c03);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000933c08); };
            c.w64(3, t4);
            let Some(t5) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933c0c); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933c10); };
            c.w64(7, t6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(c.r64(7)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000933c14); };
            c.w64(8, (t7 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x7000933c19); }
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000933c21); };
            c.w32(3, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933c26); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000933c2a); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933c2e); };
            c.w64(5, t11);
            c.w64(12, c.r64(2));
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000933c35); };
            c.w64(0, t12);
            let r13 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000933c44);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000933c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            c.w64(8, c.r64(5));
            let r0 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r0);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000933c5d); }
            let Some(t1) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933c61); };
            c.w64(7, t1);
            let Some(t2) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000933c65); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000933c6b); };
            c.w32(11, t4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933c70); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933c78); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933c7e); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933c86); };
            c.w32(0, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000933c89); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933c92); };
            c.w64(7, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000933c99); };
            c.w32(0, t14);
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000933c9f); };
            c.w64(3, t15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000933ca3); };
            c.w64(2, (t16 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933cab); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000933cb8); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000933cc8); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000933ccf); };
            c.w64(11, t21);
            c.w64(0, c.r64(11).wrapping_add(0x1c));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000933cd6); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000933cdd); };
            c.cmp(t23, c.r64(0), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000933ce6); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933ced); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000933cf4); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933cfb); };
            c.w32(0, t28);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x7000933d0d);
        }
        8 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x7000933d0e); };
            c.w32(11, t29);
            let r30 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r30);
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000933d18); };
            c.w32(11, t31);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000933d1c); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000933d20); };
            c.w64(11, t32);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000933d27); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000933d2b); }
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933d2f); };
            c.w32(11, t33);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000933d33); }
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933d37); };
            c.w32(11, t34);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000933d3b); }
            let Some(t35) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000933d3f); };
            c.w32(11, t35);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x7000933d43); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x7000933d4a); }
            blk = 9; continue;
        }
        9 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000933d4e); };
            c.w64(11, t36);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t37) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933d55); };
            c.w64(8, t37);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0xb));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000933d60); };
            c.w32(11, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000933d65); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000933d6c); };
            c.w64(1, t40);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933d78); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933d7c); }
            let r41 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r41);
            blk = 14; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933d84); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933d88); }
            let r42 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r42);
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000933d93); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000933d97); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000933d9b); }
            return c.call(0x70007febf4, 0x7000933da4, 0x7000933d9f);
        }
        12 => {
            c.w64(7, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933da7); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933dab); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933daf); };
            c.w64(1, t45);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000933db6); }
            blk = 6; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933dbc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933dc0); }
            let r46 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r46);
            blk = 14; continue;
        }
        14 => {
            let r47 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r47);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000933dc8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000933dc9); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000933dca); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000933dcc); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000933dcd); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x4u64);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2290)) else { return RegfileExit::fault(0x7000933dd9); };
            c.w64(3, t48);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x7000933de8, 0x7000933de3);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933deb); };
            c.w64(8, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000933df2); }
            blk = 9; continue;
        }
        16 => {
            return RegfileExit::jump(0x7000933dfc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000933e14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000933e14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000933e18); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000933e1e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933e22); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000933e26); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933e2d); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933e34); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000933e38); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000933e3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000933e40); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000933e44); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933e4d); };
            c.w64(11, t9);
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000933e54); };
            c.w64(0, (t10 as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000933e59); };
            c.w64(3, t11);
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000933e60); };
            c.w32(2, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000933e66); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000933e6a); };
            c.cmp(t13, c.r32(11), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x20u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000933e80); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000933e88); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000933e95); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000933e9c); };
            c.w32(0, t18);
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933ea0); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000933ea7); };
            c.w32(11, t21);
            let r22 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r22);
            c.w64(6, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t23) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000933eb1); };
            c.w32(2, t23);
            let Some(t24) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000933eb5); };
            c.w32(8, t24);
            c.w64(12, c.r64(6));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000933ebf); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000933ec3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000933ec7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x7000933ecb); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 30; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000933ed9); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000933ee6); };
            c.cmp(t26, c.r32(8), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 41; continue; }
            blk = 9; continue;
        }
        9 => {
            let r27 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r27);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000933f06); };
            c.w64(7, t28);
            let Some(t29) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000933f0d); };
            c.w64(7, t29);
            c.w64(0, c.r64(6).wrapping_add(0x8));
            c.w64(3, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000933f18); };
            c.w64(2, t30);
            let Some(t31) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000933f1f); };
            c.cmp(t31, c.r64(3), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 39; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000933f34); };
            c.w64(0, t32);
            if !c.st64(c.r64(0), c.r64(3)) { return RegfileExit::fault(0x7000933f3b); }
            let r33 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r33);
            blk = 12; continue;
        }
        12 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000933f42); }
            c.w64(0, c.r64(7));
            let r34 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r34);
            c.logic(r34, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000933f4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000933f53); }
            let Some(t35) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000933f57); };
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
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000933f6c); };
            c.w32(2, t36);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x7000933f70); }
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000933f91); }
            c.w64(1, c.r64(11));
            c.w64(8, c.r64(6));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000933f9c); };
            c.w64(0, t40);
            c.w64(10, 0x7000933faau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000933faa); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000933fae); }
            return c.call(c.r64(0), 0x7000933fb4, 0x7000933fb2);
        }
        18 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000933fb4); }
            let Some(t41) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000933fbc); };
            c.w64(4, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000933fc1); };
            c.w64(15, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000933fc5); };
            c.w64(9, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000933fc9); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000933fcd); };
            c.w64(7, t45);
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(7));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000933fd4); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000933fd8); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000933fdc); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000933fe0); };
            c.w64(2, t49);
            blk = 31; continue;
        }
        20 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 47; continue; }
            blk = 21; continue;
        }
        21 => {
            if let Some(sr) = c.shl(c.r64(6), 0x2u64, 64) { c.w64(6, sr); }
            c.w64(11, c.r64(6).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000933ffe); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 22; continue;
        }
        22 => {
            let r50 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000934012); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000934019); };
            c.w64(3, t52);
            let Some(t53) = c.ld64(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093401c); };
            c.w64(0, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x7000934020); }
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r54 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r54);
            let r55 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r55);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000934036); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700093403a); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000934042); }
            c.w32(0, 0x2u64);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000934049); };
            c.w64(3, t56);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000934057, 0x7000934052);
        }
        24 => {
            blk = 27; continue;
        }
        25 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(3)));
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000934066); };
            c.w64(11, t57);
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x700093406d); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 27; continue;
        }
        27 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000934077); };
            c.w64(7, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700093407b); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(0);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934088); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000934093); };
            c.w64(7, t61);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000934097); }
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700093409a); };
            c.w64(7, t62);
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r63 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r63);
            let r64 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r64);
            c.logic(r64, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70009340b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70009340b6); }
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009340ba); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70009340be); };
            c.w64(0, t66);
            c.w64(10, 0x70009340ccu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70009340cc); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70009340d0); }
            return c.call(c.r64(0), 0x70009340d6, 0x70009340d4);
        }
        29 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70009340d6); }
            let Some(t67) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70009340de); };
            c.w64(4, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70009340e3); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009340e7); };
            c.w64(15, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009340eb); };
            c.w64(9, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009340ef); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009340f3); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009340f7); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009340fb); };
            c.w64(0, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009340ff); };
            c.w64(2, t75);
            blk = 31; continue;
        }
        30 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000934105); };
            c.w64(11, t76);
            blk = 31; continue;
        }
        31 => {
            let r77 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r77);
            let r78 = c.r32(2) & 0x3fu64;
            c.w32(2, r78);
            c.logic(r78, 32);
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
            let Some(t79) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093411f); };
            c.w32(3, t79);
            let r80 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r80);
            let Some(t81) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000934125); };
            c.w32(3, t81);
            let r82 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r82);
            let Some(t83) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093412b); };
            c.w32(6, t83);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r84 = c.add(c.r64(2), 0x69u64, 64);
            c.w64(2, r84);
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::fault(0x7000934140);
        }
        35 => {
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000934141); };
            c.w32(3, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000934145); };
            c.w64(2, t86);
            let Some(t87) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700093414c); };
            c.w64(6, t87);
            c.w64(8, c.r64(6).wrapping_add(0x18));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000934153); };
            c.w64(12, t88);
            let Some(t89) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700093415a); };
            c.cmp(t89, c.r64(8), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x7000934164); }
            c.w64(2, c.r64(6).wrapping_add(0x1));
            blk = 37; continue;
        }
        37 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0xa));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000934172); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x7000934175); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000934178); };
            c.w64(0, t90);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700093417f); }
            c.w64(0, c.r64(2).wrapping_add(0x10));
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000934186); };
            c.w64(3, t91);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700093418d); }
            c.w64(3, c.r64(1));
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000934193); }
            let Some(t92) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000934196); };
            c.w32(0, t92);
            let r93 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r93);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x700093419c); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700093419f); }
            let Some(t94) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009341a3); };
            c.w64(7, t94);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009341a7); };
            c.w32(11, t95);
            let Some(t96) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009341ac); };
            c.w64(1, t96);
            let Some(t97) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009341b0); };
            c.w64(5, t97);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(2));
            let r98 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r98);
            return RegfileExit::jump(c.r64(1));
        }
        38 => {
            blk = 51; continue;
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70009341c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x70009341c9); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70009341d5, 0x70009341d0);
        }
        40 => {
            c.w64(7, c.r64(0));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009341d8); };
            c.w64(15, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009341dc); };
            c.w64(9, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009341e0); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009341e4); };
            c.w64(11, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70009341e8); };
            c.w64(8, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009341ec); };
            c.w64(6, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009341f0); };
            c.w64(12, t105);
            blk = 12; continue;
        }
        41 => {
            c.w64(0, c.r64(9));
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            let r106 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r106);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934203); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700093420b); };
            c.w64(3, t107);
            c.w64(6, c.r64(2));
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700093421d, 0x7000934218);
        }
        42 => {
            return RegfileExit::fault(0x700093421d);
        }
        43 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r108 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r108);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934225); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000934229); };
            c.w64(3, t109);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000934239, 0x7000934234);
        }
        44 => {
            return RegfileExit::fault(0x7000934239);
        }
        45 => {
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(0).wrapping_sub(0x1));
            let r110 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r110);
            let r111 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r111);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000934246); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700093424a); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000934252); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000934254); };
            c.w64(3, t112);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000934267, 0x7000934262);
        }
        46 => {
            blk = 27; continue;
        }
        47 => {
            c.w64(11, c.r64(9));
            let r113 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r113);
            let r114 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r114);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934276); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700093427a); };
            c.w64(3, t115);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700093428a, 0x7000934285);
        }
        48 => {
            return RegfileExit::fault(0x700093428a);
        }
        49 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x7000934290); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000934294); }
            return c.call(0x70007febf4, 0x700093429d, 0x7000934298);
        }
        50 => {
            c.w64(2, c.r64(0));
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009342a0); };
            c.w64(15, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009342a4); };
            c.w64(9, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009342a8); };
            c.w64(7, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70009342ac); };
            c.w64(11, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009342b0); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009342b4); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009342b8); };
            c.w64(3, t122);
            blk = 37; continue;
        }
        51 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r123 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r123);
            let r124 = c.r64(2) & 0x1fu64;
            c.w64(2, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009342cb); };
            c.w32(6, t125);
            let Some(t126) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009342d0); };
            c.w32(8, t126);
            let r127 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r127);
            let r128 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r128);
            let Some(t129) = c.ld32(c.r64(8).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009342da); };
            c.w32(6, t129);
            let r130 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r130);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70009342e2); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009342e6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009342e8); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70009342e9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009342ea); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x4u64);
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2270)) else { return RegfileExit::fault(0x70009342f3); };
            c.w64(3, t131);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000934302, 0x70009342fd);
        }
        52 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934302); };
            c.w64(7, t132);
            let Some(t133) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934306); };
            c.w64(7, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093430a); };
            c.w64(8, t134);
            c.w64(9, c.r64(8).wrapping_add(0xa));
            let Some(t135) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000934312); };
            c.w32(11, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934317); };
            c.w64(15, t136);
            let Some(t137) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700093431b); };
            c.w64(1, t137);
            let Some(t138) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093431f); };
            c.w64(5, t138);
            c.w64(12, c.r64(7));
            let r139 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r139);
            return RegfileExit::jump(c.r64(1));
        }
        53 => {
            c.w32(7, c.r32(7));
            let r140 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r140);
            c.w64(11, 0xfffffffffffc0000u64);
            let r141 = c.r64(11) & c.r64(7);
            c.w64(11, r141);
            c.logic(r141, 64);
            let Some(t142) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093433b); };
            c.logic(t142 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(3, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700093434a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093434b); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f13b4, 0x7000934358, 0x7000934353);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000934358); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700093435a); }
            blk = 28; continue;
        }
        56 => {
            return RegfileExit::jump(0x7000934364);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000934374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934377); };
            c.w64(6, t0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093437b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934381); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934387); };
            c.w32(7, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700093438a); };
            c.cmp(t6, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000934393); };
            c.w64(6, t7);
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093439a); };
            c.w64(3, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093439e); };
            c.w32(7, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009343a3); };
            c.cmp(t10, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7).wrapping_mul(2).wrapping_add(0x1f));
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009343be); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009343cf); };
            c.w32(6, t13);
            let r14 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r14);
            let r15 = c.r32(6) & 0x3fu64;
            c.w32(6, r15);
            c.logic(r15, 32);
            let Some(t16) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009343d7); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let r18 = c.add(c.r64(6), 0x69u64, 64);
            c.w64(6, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009343e4); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009343ea); };
            c.w32(8, t21);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(6), c.r64(8), 64);
            if c.cc_b() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70009343f9);
        }
        6 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009343fa); };
            c.w32(3, t22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009343fe); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000934405); };
            c.w64(8, t24);
            c.w64(11, c.r64(8).wrapping_add(0x18));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700093440c); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000934413); };
            c.cmp(t26, c.r64(11), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x700093441c); }
            c.w64(6, c.r64(8).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000934423); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000934428); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700093442b); };
            c.w64(3, t27);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000934432); }
            c.w64(8, c.r64(6).wrapping_add(0x10));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000934439); };
            c.w64(11, t28);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000934440); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000934444); }
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000934448); };
            c.w32(8, t29);
            let r30 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r30);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x7000934450); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000934454); }
            blk = 9; continue;
        }
        9 => {
            let r31 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r31);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093445b); };
            c.w32(3, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000934460); };
            c.w64(1, t33);
            c.w64(0, c.r64(6));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000934469); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700093446d); }
            let r34 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x7000934473); };
            c.w64(7, t35);
            let Some(t36) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700093447a); };
            c.w64(11, t36);
            c.w64(0, c.r64(11).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x7000934481); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000934485); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000934489); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700093448d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000934491); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000934495); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x7000934499); };
            c.w64(2, t37);
            let Some(t38) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70009344a0); };
            c.cmp(t38, c.r64(0), 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(2, 0x1cu64);
            return c.call(0x70007fec74, 0x70009344af, 0x70009344aa);
        }
        12 => {
            blk = 14; continue;
        }
        13 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x70009344b1); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 14; continue;
        }
        14 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70009344bb); };
            c.w64(3, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70009344c2); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70009344c7); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70009344ca); }
            if !c.st32(c.r64(0).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70009344cd); }
            if !c.st32(c.r64(0).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x70009344d4); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x70009344db); };
            c.w64(3, t40);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70009344e2); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x70009344e5); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009344ec); };
            c.w64(6, t42);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x70009344ef); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r43 = c.r64(7) & c.r64(0);
            c.w64(7, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934502); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700093450c); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700093450f); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000934513); };
            c.w64(8, t46);
            if !c.st32(c.r64(6).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000934517); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r47 = c.r64(3) & c.r64(6);
            c.w64(3, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934529); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(7, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934536); };
            c.w64(12, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093453a); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093453e); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000934542); };
            c.w64(3, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000934549); }
            blk = 4; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700093454f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000934553); }
            let r53 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r53);
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x700093455e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000934562); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000934566); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700093456a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700093456e); }
            return c.call(0x70007febf4, 0x7000934577, 0x7000934572);
        }
        20 => {
            c.w64(6, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093457a); };
            c.w64(12, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093457e); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934582); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000934586); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093458a); };
            c.w64(3, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000934591); }
            blk = 8; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000934597); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700093459b); }
            let r59 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r59);
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009345a1); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r61);
            let Some(t62) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009345a7); };
            c.w32(3, t62);
            let r63 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r63);
            let Some(t64) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009345ad); };
            c.w32(6, t64);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x70009345b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009345ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70009345be); }
            c.cmp(c.r64(6), 0x69u64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70009345c8);
        }
        23 => {
            let Some(t65) = c.ld32(c.r64(3).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70009345c9); };
            c.w32(3, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009345cf); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70009345d6); };
            c.w64(7, t67);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009345dd); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009345e1); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009345e8); };
            c.cmp(t69, c.r64(8), 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70009345f7, 0x70009345f2);
        }
        25 => {
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x70009345f9); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(6));
            blk = 27; continue;
        }
        27 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000934603); };
            c.w64(3, t70);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000934607); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x700093460c); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700093460f); };
            c.w64(3, t71);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000934616); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000934619); }
            c.w64(6, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093461f); };
            c.w64(12, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000934623); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934627); };
            c.w64(9, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700093462e); }
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
            let Some(t77) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934643); };
            c.logic(t77 & 0x2u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(7, c.r64(6).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000934651); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000934652); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(6));
            return c.call(0x70007f0e54, 0x700093465f, 0x700093465a);
        }
        30 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700093465f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000934661); }
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
            let Some(t80) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934676); };
            c.logic(t80 & 0x2u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000934684); }
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x700093468d, 0x7000934688);
        }
        33 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700093468d); }
            blk = 16; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000934694);
        }
            _ => unreachable!(),
        }
    }
}
