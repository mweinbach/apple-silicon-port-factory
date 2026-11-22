#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008256f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008256f4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008256fa); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6du64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700082570a);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x700082570b); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000825711); };
            c.w64(8, t4);
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082573c); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000825743); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(4).wrapping_add(0x18));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000825752); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000825759); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000825762); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082576a); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082576e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825772); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000825779); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000825785); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000825789); };
            c.w64(9, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000825790); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000825794); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            blk = 11; continue;
        }
        8 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008257a5); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008257ac); };
            c.w64(11, t12);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008257b3); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008257ba); };
            c.cmp(t14, c.r64(15), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008257c4); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008257cb); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008257cf); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008257d3); };
            c.w64(12, t15);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008257da); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008257de); }
            c.w32(0, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008257ec); };
            c.w64(1, t16);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008257f3); }
            c.w64(1, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let r17 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r17);
            if !c.movsd_mr(c.r64(1).wrapping_add(c.r64(12)), 0) { return RegfileExit::fault(0x700082580a); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x7000825819);
        }
        15 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082581c); }
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082581d); };
            if !c.push64(t18) { return RegfileExit::fault(0x700082581d); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x7000825821); };
            c.w64(3, t19);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x700082582d); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x7000825832); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825837); }
            return RegfileExit::jump(0x700086ed74);
        }
        16 => {
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000825846); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700082584d); };
            c.w64(10, t21);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x7000825851); }
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x6u64) { return RegfileExit::fault(0x7000825856); }
            let r22 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(0x700086ed74);
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825868); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082586c); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825872); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000825876); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700082587a); }
            return c.call(0x70007febf4, 0x7000825883, 0x700082587e);
        }
        18 => {
            c.w64(11, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825886); };
            c.w64(7, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082588a); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082588e); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000825892); };
            c.w64(9, t27);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825899); }
            blk = 7; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082589f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008258a3); }
            let r28 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r28);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008258ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008258b2); }
            return c.call(0x70007febf4, 0x70008258bb, 0x70008258b6);
        }
        20 => {
            c.w64(9, c.r64(0));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008258be); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008258c2); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008258c6); };
            c.w64(1, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008258cd); }
            blk = 10; continue;
        }
        21 => {
            return RegfileExit::jump(0x70008258d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008258f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008258f4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008258fa); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6eu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700082590a);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x700082590b); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000825911); };
            c.w64(8, t4);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), 0x7ff8u64, 32);
            if c.cc_ae() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082593c); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000825943); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(4).wrapping_add(0x18));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000825952); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000825959); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000825962); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082596a); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082596e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825972); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000825979); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000825985); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000825989); };
            c.w64(9, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000825990); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000825994); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(1));
            blk = 11; continue;
        }
        8 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008259a5); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008259ac); };
            c.w64(11, t12);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008259b3); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008259ba); };
            c.cmp(t14, c.r64(15), 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008259c4); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008259cb); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008259cf); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008259d3); };
            c.w64(12, t15);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008259da); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008259de); }
            c.w32(1, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008259ec); };
            c.w64(1, t16);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008259f3); }
            c.w64(1, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let r17 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r17);
            if !c.movsd_mr(c.r64(1).wrapping_add(c.r64(12)), 0) { return RegfileExit::fault(0x7000825a0a); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x7000825a19);
        }
        15 => {
            blk = 20; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825a1e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000825a22); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825a28); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000825a2c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000825a30); }
            return c.call(0x70007febf4, 0x7000825a39, 0x7000825a34);
        }
        17 => {
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000825a3c); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825a40); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000825a44); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000825a48); };
            c.w64(9, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825a4f); }
            blk = 7; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825a55); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000825a59); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825a64); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000825a68); }
            return c.call(0x70007febf4, 0x7000825a71, 0x7000825a6c);
        }
        19 => {
            c.w64(9, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000825a74); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825a78); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000825a7c); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825a83); }
            blk = 10; continue;
        }
        20 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000825a89); }
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000825a8a); };
            if !c.push64(t27) { return RegfileExit::fault(0x7000825a8a); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x7000825a8e); };
            c.w64(3, t28);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x7000825a9a); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x7000825a9f); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825aa4); }
            return RegfileExit::jump(0x700086ed74);
        }
        21 => {
            return RegfileExit::jump(0x7000825ab0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000825ab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825ab4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000825ab8); }
            c.w64(2, c.r64(5));
            c.w64(1, 0xfffffffffffffff8u64);
            if !c.st64(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x7000825ac4); }
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            let r0 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r0);
            c.w32(0, c.r64(2).wrapping_add(0x2));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000825ad3); };
            c.w64(5, t1);
            let r2 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r2);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000825adb); }
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000825adc); };
            if !c.push64(t3) { return RegfileExit::fault(0x7000825adc); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x7000825ae0); };
            c.w64(3, t4);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825ae7); }
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x7000825af4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000825b14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16c8)) else { return RegfileExit::fault(0x7000825b14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000825b34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16d0)) else { return RegfileExit::fault(0x7000825b34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000825b54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825b54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000825b58); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000825b6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000825b6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000825b72); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000825b76); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000825b83); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000825b90); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000825b96); };
            c.cmp(t4, c.r32(7), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825ba3); };
            c.w32(7, t5);
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 7; continue;
        }
        6 => {
            let r6 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r6);
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_ae() { blk = 100; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825bc6); };
            c.cmp(t7, c.r32(3), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 6; continue;
        }
        10 => {
            let r8 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r8);
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000825bd6); };
            c.cmp(c.r64(2), t9, 64);
            if c.cc_ae() { blk = 100; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825be0); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000825bed); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825bf4); };
            c.cmp(t14, 0x41u64, 16);
            if c.cc_ne() { blk = 10; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000825bfc); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000825bfd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000825bfe); }
            c.w32(0, 0x2u64);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000825c07); };
            c.w64(3, t15);
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000825c19, 0x7000825c14);
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000825c19); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825c26); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000825c2a); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000825c2e); };
            c.w64(6, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000825c32); };
            c.w64(2, t20);
            blk = 10; continue;
        }
        17 => {
            c.w64(8, c.r64(3));
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825c3b); };
            c.w32(9, t21);
            c.w64(11, c.r64(0));
            blk = 19; continue;
        }
        18 => {
            let r22 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r22);
            blk = 19; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000825c48); };
            c.cmp(c.r64(2), t23, 64);
            if c.cc_ae() { blk = 100; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t24) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825c52); };
            c.w32(12, t24);
            let r25 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r25);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), c.r32(12), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000825c69); };
            c.w32(15, t26);
            let r27 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r27);
            let Some(t28) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825c71); };
            c.w32(15, t28);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_ae() { blk = 18; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t29) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825c7d); };
            c.w32(0, t29);
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(15));
            let r30 = c.shl(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r30);
            let r31 = c.r32(0) | c.r32(7);
            c.w32(0, r31);
            c.logic(r31, 32);
            c.logic(c.r16(0) & 0x2020u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r16(0) & 0x1111u64, 16);
            if c.cc_ne() { blk = 103; continue; }
            blk = 26; continue;
        }
        26 => {
            let r32 = c.r32(0) & 0x808u64;
            c.w32(0, r32);
            c.logic(r32, 32);
            c.logic(c.r8(7) & 0x7u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.logic(c.r8(15) & 0x7u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(0), 0x808u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t33) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825cd8); };
            c.w64(0, t33);
            blk = 32; continue;
        }
        31 => {
            c.w64(0, c.r64(8).wrapping_add(0xb));
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t34) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825ce8); };
            c.w64(12, t34);
            blk = 35; continue;
        }
        34 => {
            let r35 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r35);
            blk = 35; continue;
        }
        35 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        36 => {
            let r36 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r36);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t37) = c.ld16(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x7000825d05); };
            c.w32(3, t37);
            let Some(t38) = c.ld8(c.r64(12).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000825d0a); };
            c.w32(1, t38);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 36; continue;
        }
        40 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t39) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825d20); };
            c.w64(0, t39);
            blk = 43; continue;
        }
        42 => {
            c.w64(0, c.r64(8).wrapping_add(0xb));
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t40) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825d30); };
            c.w64(12, t40);
            blk = 47; continue;
        }
        45 => {
            let r41 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r41);
            blk = 47; continue;
        }
        46 => {
            let r42 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r42);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t43) = c.ld8(c.r64(0).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x7000825d4a); };
            c.w32(15, t43);
            let Some(t44) = c.ld16(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x7000825d4f); };
            c.w32(3, t44);
            c.cmp(c.r32(3), c.r32(15), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 46; continue;
        }
        50 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t45) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825d64); };
            c.w64(1, t45);
            blk = 53; continue;
        }
        52 => {
            c.w64(1, c.r64(8).wrapping_add(0xb));
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t46) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825d74); };
            c.w64(12, t46);
            blk = 56; continue;
        }
        55 => {
            let r47 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r47);
            blk = 56; continue;
        }
        56 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 58; continue;
        }
        57 => {
            let r48 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r48);
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t49) = c.ld16(c.r64(12).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x7000825d91); };
            c.w32(0, t49);
            let Some(t50) = c.ld16(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x7000825d96); };
            c.cmp(t50, c.r16(0), 16);
            if c.cc_ne() { blk = 18; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 57; continue;
        }
        61 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t51) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825da8); };
            c.w64(1, t51);
            blk = 64; continue;
        }
        63 => {
            c.w64(1, c.r64(8).wrapping_add(0xb));
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t52) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000825db8); };
            c.w64(12, t52);
            blk = 67; continue;
        }
        66 => {
            let r53 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r53);
            blk = 67; continue;
        }
        67 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 69; continue;
        }
        68 => {
            let r54 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r54);
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t55) = c.ld8(c.r64(12).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000825dd5); };
            c.w32(0, t55);
            let Some(t56) = c.ld8(c.r64(1).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000825dda); };
            c.cmp(t56, c.r8(0), 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 68; continue;
        }
        72 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000825de6); }
            blk = 80; continue;
        }
        73 => {
            c.w64(11, c.r64(0));
            blk = 75; continue;
        }
        74 => {
            let r57 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r57);
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_ae() { blk = 100; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t58) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825dff); };
            c.w32(7, t58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000825e04); };
            c.cmp(t59, c.r32(7), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000825e11); };
            c.cmp(t60, c.r32(7), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 74; continue;
        }
        79 => {
            let r61 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r61);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 80; continue;
        }
        80 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 84; continue; }
            blk = 81; continue;
        }
        81 => {
            if c.cc_e() { blk = 91; continue; }
            blk = 82; continue;
        }
        82 => {
            blk = 84; continue;
        }
        83 => {
            let r62 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r62);
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_ae() { blk = 100; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t63) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825e41); };
            c.w32(7, t63);
            let r64 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r64);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t65) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000825e4e); };
            c.w32(8, t65);
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000825e52); };
            c.cmp(t66, c.r32(8), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 87; continue;
        }
        87 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000825e5b); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 102; continue; }
            blk = 88; continue;
        }
        88 => {
            if c.cc_e() { blk = 83; continue; }
            blk = 89; continue;
        }
        89 => {
            blk = 102; continue;
        }
        90 => {
            let r67 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r67);
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_ae() { blk = 100; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000825e73); };
            c.w32(7, t68);
            let r69 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r69);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t70) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000825e80); };
            c.w32(8, t70);
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000825e84); };
            c.cmp(t71, c.r32(8), 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000825e8d); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 90; continue; }
            blk = 95; continue;
        }
        95 => {
            if c.cc_e() { blk = 102; continue; }
            blk = 96; continue;
        }
        96 => {
            blk = 90; continue;
        }
        97 => {
            let r72 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r72);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 90; continue; }
            blk = 98; continue;
        }
        98 => {
            if c.cc_e() { blk = 102; continue; }
            blk = 99; continue;
        }
        99 => {
            blk = 90; continue;
        }
        100 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000825eb0); };
            c.w64(0, t73);
            blk = 101; continue;
        }
        101 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825eba); }
            return c.ret(0x0, 0x7000825ebb);
        }
        102 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000825ebc); };
            c.w64(0, t74);
            blk = 101; continue;
        }
        103 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000825ec5); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000825ec7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000825ec9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000825ecd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000825ed1); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2778)) else { return RegfileExit::fault(0x7000825ed5); };
            c.w64(3, t75);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000825ee6, 0x7000825ee1);
        }
        104 => {
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000825ee6); };
            c.cmp(t76, c.r32(0), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825eef); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000825ef3); };
            c.w64(8, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000825ef7); };
            c.w64(2, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000825efb); };
            c.w64(7, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000825eff); };
            c.w64(6, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000825f03); };
            c.w64(9, t82);
            blk = 18; continue;
        }
        106 => {
            return RegfileExit::jump(0x7000825f10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000825f14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_g() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000825f19); };
            c.w64(0, t0);
            return c.ret(0x0, 0x7000825f20);
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000825f26); };
            c.w32(6, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000825f29); };
            c.cmp(t2, c.r32(6), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(6, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000825f39); }
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            blk = 8; continue;
        }
        7 => {
            let r3 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r3);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(3), c.r64(6), 64);
            if c.cc_ae() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000825f5a); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            if c.cc_e() { blk = 7; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 20; continue;
        }
        12 => {
            let r4 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r4);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 13; continue;
        }
        13 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 15; continue;
        }
        14 => {
            let r5 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r5);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000825f8b); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 14; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 14; continue;
        }
        19 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000825f9b); };
            c.w64(0, t6);
            return c.ret(0x0, 0x7000825fa2);
        }
        20 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000825fa3); };
            c.w64(0, t7);
            return c.ret(0x0, 0x7000825faa);
        }
        21 => {
            return RegfileExit::jump(0x7000825fac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000825fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_g() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000825fb9); };
            c.w64(0, t0);
            return c.ret(0x0, 0x7000825fc0);
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000825fca); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000825fd3); };
            c.w32(6, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000825fd6); };
            c.cmp(t3, c.r32(6), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000825fe3); }
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 9; continue;
        }
        8 => {
            let r4 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r4);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ae() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(1, 0xfff7fffffff7ffffu64);
            let Some(t5) = c.ld64(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826015); };
            c.cmp(t5, c.r64(1), 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x700082601c); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 27; continue; }
            blk = 12; continue;
        }
        12 => {
            if c.cc_e() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 27; continue;
        }
        14 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 16; continue;
        }
        15 => {
            let r6 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r6);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ae() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t7) = c.ld64(0x700082600du64) else { return RegfileExit::fault(0x7000826043); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082604a); };
            c.cmp(t8, c.r64(1), 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 15; continue;
        }
        19 => {
            let r9 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r9);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 20; continue;
        }
        20 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 22; continue;
        }
        21 => {
            let r10 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r10);
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ae() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000826074); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 21; continue; }
            blk = 24; continue;
        }
        24 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            blk = 21; continue;
        }
        26 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000826084); };
            c.w64(0, t11);
            return c.ret(0x0, 0x700082608b);
        }
        27 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082608c); };
            c.w64(0, t12);
            return c.ret(0x0, 0x7000826093);
        }
        28 => {
            return RegfileExit::jump(0x7000826098);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008260b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008260b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008260b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008260b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008260ba); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008260bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(8, c.r64(7).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008260d7); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008260e5); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008260eb); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008260fc); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826103); };
            c.cmp(t7, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(15, 0x1u64);
            blk = 8; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000826122); };
            c.w32(15, t8);
            c.w64(0, c.r64(15));
            let r9 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r9);
            let r10 = c.r32(0) & 0x3fu64;
            c.w32(0, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(0), 0xbu64, 32);
            if c.cc_g() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082613a); };
            c.w32(0, t11);
            let r12 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r12);
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826140); };
            c.w32(0, t13);
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826146); };
            c.w32(3, t15);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x4cu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000826155);
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000826156); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082615c); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700082616a); };
            c.w64(0, t18);
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826171); };
            c.cmp(t19, c.r32(12), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082617b); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_le() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t22) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000826188); };
            c.w64(0, t22);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000826190); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 35; continue;
        }
        18 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ge() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r24 = c.add(c.r64(8), c.r64(0), 64);
            c.w64(8, r24);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_l() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(12, c.r64(8));
            blk = 22; continue;
        }
        21 => {
            c.w64(12, c.r64(8));
            blk = 22; continue;
        }
        22 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r25 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008261cb); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_le() { blk = 34; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(8, c.r8(15));
            let r26 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r26);
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008261e0); };
            c.w32(0, t27);
            let r28 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r28);
            let r29 = c.r32(8) & 0x3fu64;
            c.w32(8, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            let r30 = c.r32(15) & 0xfcu64;
            c.w32(15, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(15), 0x10u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(15), 0x14u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_le() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 34; continue;
        }
        28 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(3, c.r64(7));
            return c.call(0x7000825fb4, 0x7000826218, 0x7000826213);
        }
        29 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826218); };
            c.w64(8, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082621f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000826220); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826226); }
            return c.ret(0x0, 0x7000826228);
        }
        30 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(3, c.r64(7));
            return c.call(0x7000825f14, 0x7000826235, 0x7000826230);
        }
        31 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826235); };
            c.w64(8, t32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082623c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082623d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826243); }
            return c.ret(0x0, 0x7000826245);
        }
        32 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(3, c.r64(7));
            return c.call(0x7000825b54, 0x7000826252, 0x700082624d);
        }
        33 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826252); };
            c.w64(7, t33);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000826259); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082625a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826260); }
            return c.ret(0x0, 0x7000826262);
        }
        34 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000826263); };
            c.w64(0, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082626d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082626e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826274); }
            return c.ret(0x0, 0x7000826276);
        }
        35 => {
            c.w64(12, c.r64(11).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x700082627b); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t35) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000826285); };
            c.w64(8, t35);
            blk = 38; continue;
        }
        37 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082628b); };
            c.w64(8, t36);
            blk = 38; continue;
        }
        38 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000826292); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000826294); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000826295); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1c68)) else { return RegfileExit::fault(0x7000826297); };
            c.w64(3, t37);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008262a8, 0x70008262a3);
        }
        39 => {
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008262a8); };
            c.w64(7, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008262af); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008262b0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008262b6); }
            return c.ret(0x0, 0x70008262b8);
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008262b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008262bd); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008262c3); };
            c.w64(3, t39);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008262d2, 0x70008262cd);
        }
        41 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008262d2); };
            c.w64(6, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008262d6); };
            c.w64(0, t41);
            blk = 1; continue;
        }
        42 => {
            return RegfileExit::jump(0x70008262e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008262f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008262f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008262f8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008262fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000826302); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000826306); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826313); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000826319); };
            c.cmp(t3, c.r32(7), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082632c); };
            c.w32(7, t4);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000826336); }
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            blk = 6; continue;
        }
        5 => {
            let r5 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r5);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_ae() { blk = 87; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826359); };
            c.cmp(t6, c.r32(3), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 5; continue;
        }
        9 => {
            let r7 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r7);
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000826369); };
            c.cmp(c.r64(2), t8, 64);
            if c.cc_ae() { blk = 87; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826373); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826380); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826387); };
            c.cmp(t13, 0x41u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082638f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000826390); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000826391); }
            c.w32(0, 0x2u64);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700082639a); };
            c.w64(3, t14);
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008263ac, 0x70008263a7);
        }
        14 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008263ac); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008263b5); };
            c.w64(2, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008263b9); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008263bd); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008263c1); };
            c.w64(6, t19);
            blk = 9; continue;
        }
        16 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008263c7); };
            c.w64(2, t20);
            blk = 89; continue;
        }
        17 => {
            c.w64(8, c.r64(3));
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008263d3); };
            c.w32(9, t21);
            c.w64(11, c.r64(0));
            blk = 19; continue;
        }
        18 => {
            let r22 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r22);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_ae() { blk = 87; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008263e9); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), c.r32(12), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826400); };
            c.w32(15, t25);
            let r26 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r26);
            let Some(t27) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826408); };
            c.w32(15, t27);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_ae() { blk = 18; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826414); };
            c.w32(0, t28);
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(15));
            let r29 = c.shl(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            let r30 = c.r32(0) | c.r32(7);
            c.w32(0, r30);
            c.logic(r30, 32);
            c.logic(c.r16(0) & 0x2020u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r16(0) & 0x1111u64, 16);
            if c.cc_ne() { blk = 90; continue; }
            blk = 26; continue;
        }
        26 => {
            let r31 = c.r32(0) & 0x808u64;
            c.w32(0, r31);
            c.logic(r31, 32);
            c.logic(c.r8(7) & 0x7u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.logic(c.r8(15) & 0x7u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(0), 0x808u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t32) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082646f); };
            c.w64(0, t32);
            blk = 32; continue;
        }
        31 => {
            c.w64(0, c.r64(8).wrapping_add(0xb));
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t33) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082647f); };
            c.w64(12, t33);
            blk = 35; continue;
        }
        34 => {
            let r34 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r34);
            blk = 35; continue;
        }
        35 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        36 => {
            let r35 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r35);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t36) = c.ld16(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x700082649c); };
            c.w32(3, t36);
            let Some(t37) = c.ld8(c.r64(12).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008264a1); };
            c.w32(11, t37);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008264ab); };
            c.w64(11, t38);
            blk = 18; continue;
        }
        40 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t39) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008264bb); };
            c.w64(3, t39);
            blk = 43; continue;
        }
        42 => {
            c.w64(3, c.r64(8).wrapping_add(0xb));
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t40) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008264cb); };
            c.w64(12, t40);
            blk = 47; continue;
        }
        45 => {
            let r41 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r41);
            blk = 47; continue;
        }
        46 => {
            let r42 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r42);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t43) = c.ld8(c.r64(3).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008264e5); };
            c.w32(15, t43);
            let Some(t44) = c.ld16(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(2))) else { return RegfileExit::fault(0x70008264ea); };
            c.w32(11, t44);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008264f4); };
            c.w64(11, t45);
            blk = 18; continue;
        }
        50 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t46) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826502); };
            c.w64(0, t46);
            blk = 53; continue;
        }
        52 => {
            c.w64(0, c.r64(8).wrapping_add(0xb));
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t47) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826512); };
            c.w64(12, t47);
            blk = 56; continue;
        }
        55 => {
            let r48 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r48);
            blk = 56; continue;
        }
        56 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 58; continue;
        }
        57 => {
            let r49 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r49);
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t50) = c.ld16(c.r64(12).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x700082652f); };
            c.w32(3, t50);
            let Some(t51) = c.ld16(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x7000826534); };
            c.cmp(t51, c.r16(3), 16);
            if c.cc_ne() { blk = 18; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 57; continue;
        }
        61 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t52) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826546); };
            c.w64(0, t52);
            blk = 64; continue;
        }
        63 => {
            c.w64(0, c.r64(8).wrapping_add(0xb));
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t53) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826556); };
            c.w64(12, t53);
            blk = 67; continue;
        }
        66 => {
            let r54 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r54);
            blk = 67; continue;
        }
        67 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 69; continue;
        }
        68 => {
            let r55 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r55);
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t56) = c.ld8(c.r64(12).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000826573); };
            c.w32(3, t56);
            let Some(t57) = c.ld8(c.r64(0).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000826578); };
            c.cmp(t57, c.r8(3), 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 68; continue;
        }
        72 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000826584); }
            blk = 74; continue;
        }
        73 => {
            let r58 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r58);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 74; continue;
        }
        74 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 87; continue; }
            blk = 75; continue;
        }
        75 => {
            if c.cc_ne() { blk = 87; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w64(11, c.r64(0));
            blk = 78; continue;
        }
        77 => {
            let r59 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r59);
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_ae() { blk = 87; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t60) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008265b7); };
            c.w32(7, t60);
            let r61 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r61);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008265c5); };
            c.w32(8, t62);
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008265c9); };
            c.cmp(t63, c.r32(8), 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008265d2); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 77; continue; }
            blk = 82; continue;
        }
        82 => {
            if c.cc_e() { blk = 89; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 77; continue;
        }
        84 => {
            let r64 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r64);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 77; continue; }
            blk = 85; continue;
        }
        85 => {
            if c.cc_e() { blk = 89; continue; }
            blk = 86; continue;
        }
        86 => {
            blk = 77; continue;
        }
        87 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 88; continue;
        }
        88 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008265ff); }
            return c.ret(0x0, 0x7000826600);
        }
        89 => {
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            blk = 88; continue;
        }
        90 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000826606); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000826608); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700082660a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700082660e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000826612); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2778)) else { return RegfileExit::fault(0x7000826616); };
            c.w64(3, t65);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000826627, 0x7000826622);
        }
        91 => {
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000826627); };
            c.cmp(t66, c.r32(0), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000826630); };
            c.w64(2, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000826634); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826638); };
            c.w64(8, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082663c); };
            c.w64(7, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000826640); };
            c.w64(6, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000826644); };
            c.w64(9, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000826648); };
            c.w64(1, t73);
            blk = 18; continue;
        }
        93 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000826651); };
            c.w64(2, t74);
            blk = 89; continue;
        }
        94 => {
            return RegfileExit::jump(0x7000826658);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000826674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_g() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, 0xfffffffffffffffeu64);
            return c.ret(0x0, 0x7000826680);
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826686); };
            c.w32(6, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000826689); };
            c.cmp(t1, c.r32(6), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000826692); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 14; continue;
        }
        7 => {
            let r2 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 10; continue;
        }
        9 => {
            let r3 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r3);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ae() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x70008266c2); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 9; continue; }
            blk = 12; continue;
        }
        12 => {
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            c.w64(0, 0xfffffffffffffffeu64);
            return c.ret(0x0, 0x70008266d9);
        }
        15 => {
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008266dd);
        }
        16 => {
            return RegfileExit::jump(0x70008266e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008266f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_g() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, 0xfffffffffffffffeu64);
            return c.ret(0x0, 0x7000826700);
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826706); };
            c.w32(6, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000826709); };
            c.cmp(t1, c.r32(6), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000826712); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 14; continue;
        }
        7 => {
            let r2 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 10; continue;
        }
        9 => {
            let r3 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r3);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ae() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000826742); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 9; continue; }
            blk = 12; continue;
        }
        12 => {
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            c.w64(0, 0xfffffffffffffffeu64);
            return c.ret(0x0, 0x7000826759);
        }
        15 => {
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x700082675d);
        }
        16 => {
            return RegfileExit::jump(0x7000826760);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000826774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000826774); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000826778); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000826779); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082677a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082677f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(8, c.r64(7).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000826797); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008267a5); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008267ab); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008267bc); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008267c3); };
            c.cmp(t7, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(15, 0x1u64);
            blk = 8; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008267e2); };
            c.w32(15, t8);
            c.w64(0, c.r64(15));
            let r9 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r9);
            let r10 = c.r32(0) & 0x3fu64;
            c.w32(0, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(0), 0xbu64, 32);
            if c.cc_g() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008267fa); };
            c.w32(0, t11);
            let r12 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r12);
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826800); };
            c.w32(0, t13);
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826806); };
            c.w32(3, t15);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x4cu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000826815);
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000826816); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082681c); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700082682a); };
            c.w64(0, t18);
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826831); };
            c.cmp(t19, c.r32(12), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082683b); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_le() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t22) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000826848); };
            c.w64(0, t22);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000826850); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 35; continue;
        }
        18 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ge() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r24 = c.add(c.r64(8), c.r64(0), 64);
            c.w64(8, r24);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_l() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(12, c.r64(8));
            blk = 22; continue;
        }
        21 => {
            c.w64(12, c.r64(8));
            blk = 22; continue;
        }
        22 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r25 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700082688b); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_le() { blk = 34; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(8, c.r8(15));
            let r26 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r26);
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008268a0); };
            c.w32(0, t27);
            let r28 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r28);
            let r29 = c.r32(8) & 0x3fu64;
            c.w32(8, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            let r30 = c.r32(15) & 0xfcu64;
            c.w32(15, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(15), 0x10u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(15), 0x14u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_le() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 34; continue;
        }
        28 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(3, c.r64(7));
            return c.call(0x70008266f4, 0x70008268d8, 0x70008268d3);
        }
        29 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008268d8); };
            c.w64(8, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008268df); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008268e0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008268e6); }
            return c.ret(0x0, 0x70008268e8);
        }
        30 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(3, c.r64(7));
            return c.call(0x7000826674, 0x70008268f5, 0x70008268f0);
        }
        31 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008268f5); };
            c.w64(8, t32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008268fc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008268fd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826903); }
            return c.ret(0x0, 0x7000826905);
        }
        32 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(3, c.r64(7));
            return c.call(0x70008262f4, 0x7000826912, 0x700082690d);
        }
        33 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826912); };
            c.w64(7, t33);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000826919); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082691a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826920); }
            return c.ret(0x0, 0x7000826922);
        }
        34 => {
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082692d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082692e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826934); }
            return c.ret(0x0, 0x7000826936);
        }
        35 => {
            c.w64(12, c.r64(11).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x700082693b); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t34) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000826945); };
            c.w64(8, t34);
            blk = 38; continue;
        }
        37 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082694b); };
            c.w64(8, t35);
            blk = 38; continue;
        }
        38 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000826952); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000826954); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000826955); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1c70)) else { return RegfileExit::fault(0x7000826957); };
            c.w64(3, t36);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000826968, 0x7000826963);
        }
        39 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826968); };
            c.w64(7, t37);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082696f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000826970); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826976); }
            return c.ret(0x0, 0x7000826978);
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000826979); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082697d); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000826983); };
            c.w64(3, t38);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000826992, 0x700082698d);
        }
        41 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000826992); };
            c.w64(6, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826996); };
            c.w64(0, t40);
            blk = 1; continue;
        }
        42 => {
            return RegfileExit::jump(0x70008269a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008269b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16d8)) else { return RegfileExit::fault(0x70008269b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008269d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008269d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008269d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008269d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008269da); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008269df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 30; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008269f3); };
            c.w64(8, t2);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826a02); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826a09); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(12, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.w32(11, 0x1u64);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000826a29); };
            c.w32(12, t6);
            let r7 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r7);
            let r8 = c.r32(12) & 0x3fu64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_a() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826a3c); };
            c.w32(12, t9);
            let r10 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826a43); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            let Some(t13) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826a4b); };
            c.w32(15, t13);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4cu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000826a5c);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000826a5d); };
            c.w32(12, t14);
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000826a65); };
            c.cmp(t15, c.r32(12), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000826a6b); };
            c.w64(9, t16);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826a75); };
            c.cmp(t17, c.r32(12), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, c.r64(11));
            blk = 13; continue;
        }
        12 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t18) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826a8d); };
            c.w32(9, t18);
            let Some(t19) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826a91); };
            c.w32(11, t19);
            let r20 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r20);
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826a98); };
            c.logic(t21 & 0x200000u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000826aa6); };
            c.w32(11, t22);
            let r23 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r23);
            let Some(t24) = c.ld16(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826aad); };
            c.w32(12, t24);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x7000826ab8);
        }
        17 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826ab9); };
            c.w32(11, t25);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826ad7); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826ade); };
            c.w32(12, t28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000826ae2); };
            c.cmp(t29, c.r32(12), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 20; continue;
        }
        20 => {
            let r30 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r30);
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826af3); };
            c.w32(12, t31);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(9).wrapping_add(c.r64(9)).wrapping_add(0x10));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_l() { blk = 35; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000826b0f); }
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826b13); };
            c.w32(1, t32);
            let r33 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r33);
            let Some(t34) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000826b1a); };
            c.w32(1, t34);
            let r35 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r35);
            let r36 = c.r32(1) & 0x3fu64;
            c.w32(1, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_le() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(8, 0xfff7fffffff7ffffu64);
            let Some(t37) = c.ld64(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826b36); };
            c.cmp(t37, c.r64(8), 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000826b41); }
            if !c.st64(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000826b48); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000826b4d); };
            c.w64(8, t38);
            let Some(t39) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000826b54); };
            c.w64(9, t39);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000826b5b); };
            c.w64(12, t40);
            let Some(t41) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000826b62); };
            c.cmp(t41, c.r64(11), 64);
            if c.cc_be() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000826b6c); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000826b73); };
            c.w64(9, t42);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000826b7a); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000826b7e); }
            let r43 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r43);
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000826b8e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000826b8f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826b95); }
            return c.ret(0x0, 0x7000826b97);
        }
        26 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826b98); };
            c.w32(1, t44);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 34; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826bab); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000826bb3); };
            c.w64(8, t47);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000826bba); }
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000826bbf); };
            c.cmp(t48, c.r32(1), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            let r49 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r49);
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000826bd5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000826bd6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826bdc); }
            return c.ret(0x0, 0x7000826bde);
        }
        29 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let r50 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000826be6); };
            c.w64(0, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000826bf0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000826bf1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826bf7); }
            return c.ret(0x0, 0x7000826bf9);
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000826bfa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000826bfe); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000826c04); };
            c.w64(3, t52);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000826c13, 0x7000826c0e);
        }
        31 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826c13); };
            c.w64(0, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000826c17); };
            c.w64(6, t54);
            blk = 1; continue;
        }
        32 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000826c25); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x7000826c29); }
            return c.call(0x70007febf4, 0x7000826c33, 0x7000826c2e);
        }
        33 => {
            c.w64(8, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000826c36); };
            c.w64(1, t55);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x7000826c3a); }
            blk = 25; continue;
        }
        34 => {
            return RegfileExit::fault(0x7000826c44);
        }
        35 => {
            let Some(t56) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000826c45); };
            c.w64(1, t56);
            let Some(t57) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000826c49); };
            c.w64(5, t57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000826c4d); };
            c.w64(2, t58);
            c.w64(7, c.r64(1));
            let r59 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r59);
            return RegfileExit::jump(0x70008269b4);
        }
        36 => {
            return RegfileExit::jump(0x7000826c64);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000826c74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16e0)) else { return RegfileExit::fault(0x7000826c74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000826c94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000826c94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000826c98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000826c99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000826c9a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000826c9f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000826ca3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 112; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            let Some(t2) = c.ld64(c.r64(9).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000826cb4); };
            c.w64(11, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000826cb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000826cbc); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826cca); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826cd1); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(3, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.w32(15, 0x1u64);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000826cef); };
            c.w32(3, t6);
            let r7 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r7);
            let r8 = c.r32(3) & 0x3fu64;
            c.w32(3, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_a() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826d00); };
            c.w32(3, t9);
            let r10 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826d06); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826d0c); };
            c.w32(2, t13);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000826d1b);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000826d1c); };
            c.w32(3, t14);
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000826d22); };
            c.cmp(t15, c.r32(3), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000826d2c); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826d33); };
            c.cmp(t17, c.r32(12), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(3, c.r64(15));
            blk = 14; continue;
        }
        12 => {
            c.w32(15, 0x1u64);
            blk = 13; continue;
        }
        13 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t18) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826d52); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826d59); };
            c.w32(2, t20);
            c.w64(1, c.r64(2));
            let r21 = c.r32(1) & 0x8100000u64;
            c.w32(1, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(1), 0x8000000u64, 32);
            if c.cc_ne() { blk = 131; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r32(2) & 0x200000u64, 32);
            if c.cc_ne() { blk = 131; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000826d7d); };
            c.w32(2, t22);
            let r23 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r23);
            let Some(t24) = c.ld16(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826d83); };
            c.w32(1, t24);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x7000826d8d);
        }
        19 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000826d8e); };
            c.w32(2, t25);
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t26) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000826d9a); };
            c.w32(8, t26);
            let r27 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r27);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r28 = c.r32(8) & 0x3fu64;
            c.w32(8, r28);
            c.logic(r28, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000826daa); }
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_g() { blk = 47; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826db8); };
            c.w32(8, t29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(3, c.r64(0).wrapping_add(c.r64(8)));
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826dc6); };
            c.w32(2, t30);
            let r31 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r31);
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826dcd); };
            c.w32(1, t32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000826dd9); }
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_ge() { blk = 34; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(7, c.r64(3));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(3, c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x10));
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 40; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 117; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000826e0b); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000826e12); };
            c.w64(15, t34);
            c.w64(6, c.r64(3).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(6).wrapping_add(c.r64(15)));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000826e21); };
            c.w64(12, t35);
            let Some(t36) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000826e28); };
            c.cmp(t36, c.r64(8), 64);
            if c.cc_be() { blk = 114; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7), c.r64(8)) { return RegfileExit::fault(0x7000826e32); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            c.w32(12, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000826e3d); };
            c.w64(15, t37);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000826e44); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000826e48); }
            c.w64(12, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.w64(15, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000826e61); };
            c.w64(3, t38);
            blk = 28; continue;
        }
        28 => {
            let r39 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r39);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(3)) { return RegfileExit::fault(0x7000826e6c); }
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ne() { blk = 28; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let r40 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r40);
            let Some(t41) = c.ld32(c.r64(12).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000826e7f); };
            c.w32(15, t41);
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(15)) { return RegfileExit::fault(0x7000826e83); }
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_ne() { blk = 30; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000826e8d); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r42 = c.r64(12) & c.r64(11);
            c.w64(12, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000826ea1); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 141; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(2, c.r64(8));
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000826eb0); };
            c.w64(6, t44);
            c.w32(12, 0);
            c.logic(0, 32);
            c.w32(15, 0x1u64);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000826ebd); };
            c.w64(8, t45);
            blk = 34; continue;
        }
        34 => {
            c.w64(3, c.r64(9).wrapping_add(c.r64(0).wrapping_mul(8)));
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(3, c.r64(9));
            blk = 36; continue;
        }
        36 => {
            let Some(t46) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000826ecd); };
            c.w64(1, t46);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t47) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826ed5); };
            c.w32(7, t47);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 116; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(0).wrapping_mul(8)));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000826eeb); }
            let r48 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r48);
            let r49 = c.add(c.r64(3), 0x8u64, 64);
            c.w64(3, r49);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_ne() { blk = 36; continue; }
            blk = 39; continue;
        }
        39 => {
            let r50 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r50);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000826f00); }
            c.w64(9, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000826f0e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000826f0f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000826f15); }
            return c.ret(0x0, 0x7000826f17);
        }
        40 => {
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826f18); };
            c.w32(3, t51);
            let r52 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r52);
            c.w32(3, c.r32(3));
            c.w32(2, c.r32(8));
            let r53 = c.sub(c.r32(2), c.r32(3), 32);
            c.w32(2, r53);
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000826f29); }
            let Some(t54) = c.ld64(c.r64(9).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x7000826f2d); };
            c.w64(8, t54);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000826f34); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 41; continue;
        }
        41 => {
            let r55 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r55);
            let Some(t56) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826f46); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000826f4d); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(8));
            return c.call(0x7000871cb4, 0x7000826f5f, 0x7000826f5a);
        }
        42 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000826f5f); };
            c.w64(0, t58);
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826f63); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000826f6a); };
            c.w32(8, t61);
            let r62 = c.r32(8) & 0xfcu64;
            c.w32(8, r62);
            c.logic(r62, 32);
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000826f7c); };
            c.w64(8, t63);
            let Some(t64) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000826f80); };
            c.w32(8, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000826f84); };
            c.cmp(t65, c.r32(8), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000826f91); };
            c.w64(3, t66);
            c.w64(11, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826f98); };
            c.w64(0, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000826f9c); };
            c.w64(9, t68);
            c.w32(12, 0);
            c.logic(0, 32);
            c.w32(15, 0x1u64);
            blk = 50; continue;
        }
        45 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000826fab); };
            c.w64(3, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000826faf); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000826fb3); };
            c.w64(9, t71);
            blk = 106; continue;
        }
        46 => {
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(11));
            blk = 106; continue;
        }
        47 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_g() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000826fcd); }
            c.w64(0, c.r64(11));
            blk = 84; continue;
        }
        49 => {
            c.w64(3, c.r64(12));
            blk = 50; continue;
        }
        50 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000826fdc); };
            c.w32(8, t72);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(0));
            let r73 = c.sub(c.r64(2), c.r64(3), 64);
            c.w64(2, r73);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r74 = c.add(c.r64(2), c.r64(8), 64);
            c.w64(2, r74);
            let Some(t75) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000826fef); };
            c.w32(1, t75);
            let r76 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r76);
            let Some(t77) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000826ff6); };
            c.w32(7, t77);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(7));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000827002); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000827006); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_ge() { blk = 67; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(6, c.r64(2));
            if let Some(sr) = c.shr(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(2, c.r64(6).wrapping_add(c.r64(2)).wrapping_add(0x10));
            c.cmp(c.r64(2), 0x3fffu64, 64);
            if c.cc_ae() { blk = 78; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(2), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 124; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(6, c.r64(2).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(6), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 119; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082704d); };
            c.w64(8, t78);
            let Some(t79) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000827054); };
            c.w64(15, t79);
            c.w64(3, c.r64(6).wrapping_add(c.r64(15)));
            let Some(t80) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082705b); };
            c.cmp(t80, c.r64(3), 64);
            if c.cc_be() { blk = 121; continue; }
            blk = 55; continue;
        }
        55 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x7000827065); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 56; continue;
        }
        56 => {
            c.w32(15, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000827070); };
            c.w64(3, t81);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000827077); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700082707b); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082707f); };
            c.w64(15, t82);
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000827086); }
            c.w64(15, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.w64(3, c.r64(2).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let r83 = c.sub(c.r64(3), 0x8u64, 64);
            c.w64(3, r83);
            if !c.movsd_mr(c.r64(3).wrapping_add(c.r64(8)), 0) { return RegfileExit::fault(0x70008270a5); }
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_ne() { blk = 57; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 64; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(3, 0xfff7fffffff7ffffu64);
            blk = 60; continue;
        }
        60 => {
            let r84 = c.sub(c.r64(15), 0x8u64, 64);
            c.w64(15, r84);
            let Some(t85) = c.ld64(c.r64(15).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008270c4); };
            c.cmp(t85, c.r64(3), 64);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(c.r64(1))) { return RegfileExit::fault(0x70008270ca); }
            if !c.movsd_mr(c.r64(15).wrapping_add(c.r64(8)), 0) { return RegfileExit::fault(0x70008270d0); }
            blk = 63; continue;
        }
        62 => {
            if !c.st64(c.r64(15).wrapping_add(c.r64(8)), c.r64(3)) { return RegfileExit::fault(0x70008270d8); }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 60; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008270e2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r86 = c.r64(15) & c.r64(11);
            c.w64(15, r86);
            c.logic(r86, 64);
            let Some(t87) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008270f6); };
            c.logic(t87 & 0x4u64, 8);
            if c.cc_ne() { blk = 138; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(1, c.r64(8));
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000827104); };
            c.w64(3, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000827108); };
            c.w64(8, t89);
            c.w32(15, 0x1u64);
            blk = 67; continue;
        }
        67 => {
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(2, c.r64(0).wrapping_add(c.r64(9).wrapping_mul(8)));
            c.w64(6, c.r64(0).wrapping_add(c.r64(3).wrapping_mul(8)));
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t90) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000827128); };
            c.w64(2, t90);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t91) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000827130); };
            c.w32(7, t91);
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000827133); };
            c.cmp(t92, c.r32(7), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 72; continue;
        }
        71 => {
            c.w64(7, c.r64(15));
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000827148); }
            blk = 75; continue;
        }
        74 => {
            let r93 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r93);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 75; continue;
        }
        75 => {
            let Some(t94) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000827159); };
            c.w32(2, t94);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_ae() { blk = 123; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(9).wrapping_mul(8)));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000827179); }
            let r95 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r95);
            let r96 = c.add(c.r64(6), 0x8u64, 64);
            c.w64(6, r96);
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_ne() { blk = 68; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x7000827191); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082719c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082719d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008271a3); }
            return c.ret(0x0, 0x70008271a5);
        }
        78 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 79; continue;
        }
        79 => {
            let Some(t97) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008271af); };
            c.w32(15, t97);
            let r98 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r98);
            c.w32(15, c.r32(15));
            c.w32(2, c.r32(8));
            let r99 = c.sub(c.r32(2), c.r32(15), 32);
            c.w32(2, r99);
            c.w64(15, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r100 = c.add(c.r64(3), c.r64(15), 64);
            c.w64(3, r100);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008271c8); }
            let Some(t101) = c.ld64(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008271cc); };
            c.w64(8, t101);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t102) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008271da); };
            c.w32(15, t102);
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008271de); };
            c.cmp(t103, c.r32(15), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(11));
            blk = 106; continue;
        }
        82 => {
            let r104 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r104);
            let Some(t105) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008271f9); };
            c.w32(15, t105);
            let r106 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r106);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000827200); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(8));
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082720d); };
            c.w64(6, t107);
            return c.call(0x7000871cb4, 0x7000827216, 0x7000827211);
        }
        83 => {
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000827216); };
            c.w64(0, t108);
            let Some(t109) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082721a); };
            c.w32(8, t109);
            let r110 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r110);
            let Some(t111) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000827221); };
            c.w32(8, t111);
            let r112 = c.r32(8) & 0xfcu64;
            c.w32(8, r112);
            c.logic(r112, 32);
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t113) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000827237); };
            c.w32(8, t113);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082723e); };
            c.w64(11, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000827242); };
            c.w64(3, t115);
            let r116 = c.sub(c.r64(11), c.r64(3), 64);
            c.w64(11, r116);
            c.w32(8, c.r32(8));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r117 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r117);
            let Some(t118) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000827252); };
            c.w32(12, t118);
            let r119 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r119);
            let Some(t120) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000827259); };
            c.w32(15, t120);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(15));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ge() { blk = 97; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(2, c.r64(11));
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w64(11, c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x10));
            c.cmp(c.r64(11), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 103; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 129; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827295); };
            c.w64(8, t121);
            let Some(t122) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082729c); };
            c.w64(2, t122);
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008272ab); };
            c.w64(7, t123);
            let Some(t124) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008272b2); };
            c.cmp(t124, c.r64(6), 64);
            if c.cc_be() { blk = 126; continue; }
            blk = 88; continue;
        }
        88 => {
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x70008272bb); }
            c.w64(8, c.r64(2).wrapping_add(0x1));
            blk = 89; continue;
        }
        89 => {
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008272c6); };
            c.w64(1, t125);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008272cd); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008272d1); }
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 92; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008272ea); };
            c.w64(2, t126);
            blk = 91; continue;
        }
        91 => {
            let r127 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r127);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x70008272f5); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 91; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            let r128 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r128);
            let Some(t129) = c.ld32(c.r64(15).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000827308); };
            c.w32(11, t129);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x700082730c); }
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 93; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000827316); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r130 = c.r64(11) & c.r64(0);
            c.w64(11, r130);
            c.logic(r130, 64);
            let Some(t131) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082732a); };
            c.logic(t131 & 0x4u64, 8);
            if c.cc_ne() { blk = 135; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(12, c.r64(8));
            blk = 97; continue;
        }
        97 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000827338); };
            c.w64(11, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082733c); };
            c.w64(15, t133);
            c.w64(8, c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8)));
            c.w64(3, c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8)));
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t134) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082734d); };
            c.w32(8, t134);
            let Some(t135) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000827352); };
            c.w64(2, t135);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 128; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700082736c); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r136 = c.r64(1) & c.r64(12);
            c.w64(1, r136);
            c.logic(r136, 64);
            let Some(t137) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082737f); };
            c.logic(t137 & 0x4u64, 8);
            if c.cc_ne() { blk = 132; continue; }
            blk = 101; continue;
        }
        101 => {
            let r138 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r138);
            c.w64(8, c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8)));
            let r139 = c.add(c.r64(3), 0x8u64, 64);
            c.w64(3, r139);
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_ne() { blk = 98; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082739e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008273ac); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008273ad); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008273b3); }
            return c.ret(0x0, 0x70008273b5);
        }
        103 => {
            let r140 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r140);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008273b9); }
            c.w32(9, c.r32(9));
            let r141 = c.sub(c.r32(9), c.r32(8), 32);
            c.w32(9, r141);
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r142 = c.add(c.r64(3), c.r64(8), 64);
            c.w64(3, r142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008273cc); };
            c.w64(9, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008273d0); };
            c.w64(8, t144);
            blk = 106; continue;
        }
        104 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008273d6); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008273da); };
            c.w64(8, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008273de); };
            c.w64(9, t147);
            blk = 106; continue;
        }
        105 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(11));
            blk = 106; continue;
        }
        106 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(9, c.r64(9).wrapping_add(c.r64(3).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008273f5); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 107; continue;
        }
        107 => {
            blk = 109; continue;
        }
        108 => {
            c.w64(11, c.r64(8));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000827403); };
            c.w64(0, t148);
            blk = 109; continue;
        }
        109 => {
            let Some(t149) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000827407); };
            c.w64(12, t149);
            let Some(t150) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082740a); };
            c.w32(3, t150);
            let r151 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r151);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000827410); }
            c.w64(1, c.r64(12));
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000827417); };
            c.w64(6, t152);
            return c.call(0x7000871cb4, 0x7000827420, 0x700082741b);
        }
        110 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000827420); };
            c.w64(9, t153);
            let r154 = c.add(c.r64(9), 0x8u64, 64);
            c.w64(9, r154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000827428); };
            c.w64(8, t155);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 108; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000827431); };
            c.w64(8, t156);
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000827435); };
            c.w32(0, t157);
            let r158 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082743c); };
            c.w64(8, t159);
            let r160 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r160);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000827447); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000827448); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082744e); }
            return c.ret(0x0, 0x7000827450);
        }
        112 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000827451); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000827457); };
            c.w64(3, t161);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000827466, 0x7000827461);
        }
        113 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000827466); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082746a); };
            c.w64(6, t163);
            blk = 1; continue;
        }
        114 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000827473); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000827477); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700082747b); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000827487, 0x7000827482);
        }
        115 => {
            c.w64(8, c.r64(0));
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082748a); };
            c.w64(0, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082748e); };
            c.w64(11, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000827492); };
            c.w64(9, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000827496); };
            c.w64(2, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082749a); };
            c.w64(3, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082749e); };
            c.w64(1, t169);
            blk = 26; continue;
        }
        116 => {
            return RegfileExit::fault(0x70008274a7);
        }
        117 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008274a8); };
            c.w64(3, t170);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008274b9, 0x70008274b4);
        }
        118 => {
            return RegfileExit::fault(0x70008274b9);
        }
        119 => {
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008274ba); };
            c.w64(3, t171);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008274cb, 0x70008274c6);
        }
        120 => {
            return RegfileExit::fault(0x70008274cb);
        }
        121 => {
            c.w32(8, c.r64(6).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008274d0); }
            c.w64(6, c.r64(12));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008274d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008274d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008274da); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008274de); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008274e2); };
            c.w64(3, t172);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008274f6, 0x70008274f1);
        }
        122 => {
            c.w64(8, c.r64(0));
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008274f9); };
            c.w64(0, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008274fd); };
            c.w64(11, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000827501); };
            c.w64(9, t175);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000827508); };
            c.w64(1, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082750c); };
            c.w64(2, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000827510); };
            c.w64(7, t178);
            blk = 56; continue;
        }
        123 => {
            return RegfileExit::fault(0x7000827519);
        }
        124 => {
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700082751a); };
            c.w64(3, t179);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700082752b, 0x7000827526);
        }
        125 => {
            return RegfileExit::fault(0x700082752b);
        }
        126 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082752c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000827530); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000827534); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x7000827538); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000827544, 0x700082753f);
        }
        127 => {
            c.w64(8, c.r64(0));
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000827547); };
            c.w64(0, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082754b); };
            c.w64(3, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082754f); };
            c.w64(9, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000827553); };
            c.w64(12, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000827557); };
            c.w64(11, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082755b); };
            c.w64(15, t185);
            blk = 89; continue;
        }
        128 => {
            return RegfileExit::fault(0x7000827564);
        }
        129 => {
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000827565); };
            c.w64(3, t186);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000827575, 0x7000827570);
        }
        130 => {
            return RegfileExit::fault(0x7000827575);
        }
        131 => {
            let Some(t187) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000827576); };
            c.w64(8, t187);
            let Some(t188) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700082757a); };
            c.w64(5, t188);
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082757e); };
            c.w64(2, t189);
            c.w64(7, c.r64(8));
            let r190 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r190);
            return RegfileExit::jump(0x7000826c74);
        }
        132 => {
            c.w32(2, c.r32(2));
            let r191 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r191);
            c.w64(1, 0xfffffffffffc0000u64);
            let r192 = c.r64(1) & c.r64(2);
            c.w64(1, r192);
            c.logic(r192, 64);
            let Some(t193) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008275a0); };
            c.logic(t193 & 0x2u64, 8);
            if c.cc_e() { blk = 101; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008275ae); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008275af); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008275bc, 0x70008275b7);
        }
        134 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008275bc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008275be); }
            blk = 101; continue;
        }
        135 => {
            c.w32(8, c.r32(8));
            let r194 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r194);
            c.w64(11, 0xfffffffffffc0000u64);
            let r195 = c.r64(11) & c.r64(8);
            c.w64(11, r195);
            c.logic(r195, 64);
            let Some(t196) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008275d4); };
            c.logic(t196 & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(12, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008275e3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008275e4); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008275f1, 0x70008275ec);
        }
        137 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008275f1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008275f3); }
            blk = 96; continue;
        }
        138 => {
            c.w32(8, c.r32(8));
            let r197 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r197);
            c.w64(15, 0xfffffffffffc0000u64);
            let r198 = c.r64(15) & c.r64(8);
            c.w64(15, r198);
            c.logic(r198, 64);
            let Some(t199) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827609); };
            c.logic(t199 & 0x2u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(3, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000827618); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000827619); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000827626, 0x7000827621);
        }
        140 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000827626); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000827628); }
            blk = 66; continue;
        }
        141 => {
            c.w32(8, c.r32(8));
            let r200 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r200);
            c.w64(12, 0xfffffffffffc0000u64);
            let r201 = c.r64(12) & c.r64(8);
            c.w64(12, r201);
            c.logic(r201, 64);
            let Some(t202) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082763e); };
            c.logic(t202 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(15, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082764e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082764f); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700082765c, 0x7000827657);
        }
        143 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700082765c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700082765e); }
            blk = 33; continue;
        }
        144 => {
            return RegfileExit::jump(0x7000827668);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000827674(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16e8)) else { return RegfileExit::fault(0x7000827674); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000827694(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16f0)) else { return RegfileExit::fault(0x7000827694); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008276b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008276b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008276b8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008276be); };
            c.w32(7, t1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008276c1); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008276c8); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            c.w64(12, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008276d2); };
            c.w32(9, t6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008276d7); };
            c.w32(15, t7);
            c.w64(0, c.r64(12));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008276e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008276e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008276e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008276ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008276f1); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008276ff); };
            c.cmp(t8, c.r32(15), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700082770c); };
            c.cmp(t9, c.r32(15), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            let r10 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r10);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082772b); };
            c.w64(12, t11);
            let Some(t12) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000827732); };
            c.w64(12, t12);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(3, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082773e); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000827745); };
            c.cmp(t14, c.r64(3), 64);
            if c.cc_be() { blk = 33; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 33; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082775b); };
            c.w64(2, t15);
            if !c.st64(c.r64(2), c.r64(3)) { return RegfileExit::fault(0x7000827762); }
            let r16 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r16);
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000827769); }
            c.w64(15, c.r64(12));
            let r17 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r17);
            c.logic(r17, 64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000827778); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700082777d); }
            let Some(t18) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827781); };
            c.logic(t18 & 0x4u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(15, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(0, 0x7u64);
            blk = 10; continue;
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x7000827797); };
            c.w32(3, t19);
            if !c.st32(c.r64(12).wrapping_add(c.r64(0)), c.r32(3)) { return RegfileExit::fault(0x700082779b); }
            let r20 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r20);
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ne() { blk = 10; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 14; continue;
        }
        12 => {
            c.w64(2, c.r64(11).wrapping_add(0x7));
            c.w64(1, c.r64(12).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r21 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r21);
            let r22 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r22);
            c.logic(r22, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008277be); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008277c3); };
            c.w64(0, t23);
            c.w64(10, 0x70008277d1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008277d1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008277d5); }
            return c.call(c.r64(0), 0x70008277db, 0x70008277d9);
        }
        13 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008277db); }
            let Some(t24) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008277e3); };
            c.w64(4, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008277e8); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008277ec); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008277f0); };
            c.w64(9, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008277f4); };
            c.w64(12, t28);
            blk = 14; continue;
        }
        14 => {
            c.w64(11, c.r64(12));
            blk = 26; continue;
        }
        15 => {
            c.cmp(c.r64(0), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 41; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, c.r64(12).wrapping_mul(4));
            c.w64(12, c.r64(8).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000827818); }
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 17; continue;
        }
        17 => {
            let r29 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082782b); };
            c.w64(3, t30);
            let Some(t31) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000827832); };
            c.w64(2, t31);
            let Some(t32) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827835); };
            c.w64(3, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000827839); }
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let r33 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r33);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000827849); }
            c.w32(12, 0x4u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000827851); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000827855); };
            c.w64(3, t34);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000827869, 0x7000827864);
        }
        19 => {
            blk = 22; continue;
        }
        20 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(2)));
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827878); };
            c.w64(12, t35);
            if !c.st64(c.r64(12), c.r64(1)) { return RegfileExit::fault(0x700082787f); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 22; continue;
        }
        22 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082788a); };
            c.w64(7, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700082788e); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r37 = c.r64(9) & c.r64(0);
            c.w64(9, r37);
            c.logic(r37, 64);
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082789b); };
            c.logic(t38 & 0x4u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008278a6); };
            c.w64(7, t39);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008278aa); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008278ad); };
            c.w64(7, t40);
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r41 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r41);
            let r42 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r42);
            c.logic(r42, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008278c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008278c9); }
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008278cd); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008278d1); };
            c.w64(0, t44);
            c.w64(10, 0x70008278dfu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008278df); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008278e3); }
            return c.call(c.r64(0), 0x70008278e9, 0x70008278e7);
        }
        24 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008278e9); }
            let Some(t45) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008278f1); };
            c.w64(4, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008278f6); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008278fa); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008278fe); };
            c.w64(6, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000827902); };
            c.w64(9, t49);
            blk = 26; continue;
        }
        25 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000827908); };
            c.w64(0, t50);
            c.w64(11, c.r64(0));
            blk = 26; continue;
        }
        26 => {
            let r51 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r51);
            let r52 = c.r32(9) & 0x3fu64;
            c.w32(9, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_le() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(9, 0x2u64);
            blk = 28; continue;
        }
        28 => {
            c.w64(12, c.r64(6));
            let Some(t53) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000827929); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000827931); };
            c.w32(12, t55);
            let r56 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r56);
            let Some(t57) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000827939); };
            c.w32(15, t57);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r58 = c.add(c.r64(9), 0x69u64, 64);
            c.w64(9, r58);
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x7000827950);
        }
        30 => {
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000827951); };
            c.w32(9, t59);
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827956); };
            c.w64(12, t60);
            let Some(t61) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082795d); };
            c.w64(15, t61);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000827965); };
            c.w64(3, t62);
            let Some(t63) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700082796c); };
            c.cmp(t63, c.r64(0), 64);
            if c.cc_be() { blk = 43; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000827975); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 32; continue;
        }
        32 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082797d); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000827982); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000827987); };
            c.w64(7, t64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700082798e); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000827993); }
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082799e); }
            return c.ret(0x0, 0x700082799f);
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008279a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008279a4); }
            return c.call(0x70007febb4, 0x70008279ad, 0x70008279a8);
        }
        34 => {
            c.w64(12, c.r64(0));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008279b0); };
            c.w64(7, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008279b4); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008279b8); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008279bc); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008279c0); };
            c.w64(15, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008279c4); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008279c8); };
            c.w64(0, t71);
            blk = 7; continue;
        }
        35 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008279d3); };
            c.w64(3, t72);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008279e4, 0x70008279df);
        }
        36 => {
            return RegfileExit::fault(0x70008279e4);
        }
        37 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008279e5); };
            c.w64(3, t73);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008279f5, 0x70008279f0);
        }
        38 => {
            return RegfileExit::fault(0x70008279f5);
        }
        39 => {
            let r74 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r74);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008279f9); }
            c.w32(12, 0x4u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000827a01); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000827a03); };
            c.w64(3, t75);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000827a16, 0x7000827a11);
        }
        40 => {
            blk = 22; continue;
        }
        41 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000827a1b); };
            c.w64(3, t76);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000827a2b, 0x7000827a26);
        }
        42 => {
            return RegfileExit::fault(0x7000827a2b);
        }
        43 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000827a31); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000827a35); }
            return c.call(0x70007febf4, 0x7000827a3e, 0x7000827a39);
        }
        44 => {
            c.w64(12, c.r64(0));
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000827a41); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000827a45); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000827a49); };
            c.w64(9, t79);
            blk = 32; continue;
        }
        45 => {
            c.w32(7, c.r32(7));
            let r80 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r80);
            c.w64(9, 0xfffffffffffc0000u64);
            let r81 = c.r64(9) & c.r64(7);
            c.w64(9, r81);
            c.logic(r81, 64);
            let Some(t82) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827a61); };
            c.logic(t82 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000827a70); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000827a71); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000827a7e, 0x7000827a79);
        }
        47 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000827a7e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000827a80); }
            blk = 23; continue;
        }
        48 => {
            return RegfileExit::jump(0x7000827a88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000827a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000827a94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000827a98); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000827a9e); };
            c.w32(7, t1);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000827aa4); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t4) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000827ab1); };
            c.w32(11, t4);
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000827ac2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000827ac6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000827aca); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000827ace); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x7000827ad2); }
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000827ae0); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000827ae6); };
            c.w32(3, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000827ae9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000827aed); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000827afb); };
            c.cmp(t8, c.r32(3), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000827b08); };
            c.cmp(t9, c.r32(3), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 99; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827b26); };
            c.w64(9, t10);
            let Some(t11) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000827b2d); };
            c.w64(9, t11);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(15, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000827b38); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000827b3f); };
            c.cmp(t13, c.r64(15), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 97; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827b55); };
            c.w64(2, t14);
            if !c.st64(c.r64(2), c.r64(15)) { return RegfileExit::fault(0x7000827b5c); }
            let r15 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r15);
            blk = 8; continue;
        }
        8 => {
            let r16 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r16);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000827b66); }
            c.w64(15, c.r64(9));
            let r17 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r17);
            c.logic(r17, 64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000827b74); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x7000827b78); }
            let Some(t18) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827b7c); };
            c.logic(t18 & 0x4u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(12, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(15, 0x7u64);
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000827b93); };
            c.w32(3, t19);
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(3)) { return RegfileExit::fault(0x7000827b97); }
            let r20 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r20);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_ne() { blk = 11; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 15; continue;
        }
        13 => {
            c.w64(2, c.r64(0).wrapping_add(0x7));
            c.w64(1, c.r64(9).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r21 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r21);
            let r22 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r22);
            c.logic(r22, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000827bb9); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000827bbe); };
            c.w64(0, t23);
            c.w64(10, 0x7000827bccu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000827bcc); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000827bd0); }
            return c.call(c.r64(0), 0x7000827bd6, 0x7000827bd4);
        }
        14 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000827bd6); }
            let Some(t24) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000827bde); };
            c.w64(4, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000827be3); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000827be7); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000827beb); };
            c.w64(11, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000827bef); };
            c.w64(9, t28);
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(9));
            blk = 27; continue;
        }
        16 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 105; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(8, c.r64(9).wrapping_mul(4));
            c.w64(9, c.r64(8).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000827c14); }
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 101; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827c25); };
            c.w64(15, t29);
            let Some(t30) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000827c2c); };
            c.w64(2, t30);
            let Some(t31) = c.ld64(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827c2f); };
            c.w64(15, t31);
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_be() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let r32 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r32);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000827c3f); }
            c.w32(9, 0x4u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000827c47); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000827c50); };
            c.w64(3, t33);
            c.w64(9, c.r64(3));
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x7000827c62, 0x7000827c5d);
        }
        20 => {
            blk = 23; continue;
        }
        21 => {
            c.w64(1, c.r64(9).wrapping_add(c.r64(2)));
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827c71); };
            c.w64(9, t34);
            if !c.st64(c.r64(9), c.r64(1)) { return RegfileExit::fault(0x7000827c78); }
            c.w64(9, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 23; continue;
        }
        23 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000827c82); };
            c.w64(7, t35);
            let r36 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000827c88); };
            c.w64(9, t37);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000827c8c); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r38 = c.r64(11) & c.r64(0);
            c.w64(11, r38);
            c.logic(r38, 64);
            let Some(t39) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827c9a); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000827ca5); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000827ca8); };
            c.w64(7, t40);
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r41 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r41);
            let r42 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r42);
            c.logic(r42, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000827cbf); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000827cc4); }
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000827cc8); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000827ccc); };
            c.w64(0, t44);
            c.w64(10, 0x7000827cdau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000827cda); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000827cde); }
            return c.call(c.r64(0), 0x7000827ce4, 0x7000827ce2);
        }
        25 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000827ce4); }
            let Some(t45) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000827cec); };
            c.w64(4, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000827cf1); };
            c.w64(0, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000827cf5); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000827cf9); };
            c.w64(6, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000827cfd); };
            c.w64(11, t49);
            blk = 27; continue;
        }
        26 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000827d03); };
            c.w64(0, t50);
            blk = 27; continue;
        }
        27 => {
            let r51 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r51);
            let r52 = c.r32(11) & 0x3fu64;
            c.w32(11, r52);
            c.logic(r52, 32);
            blk = 90; continue;
        }
        28 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000827d17); };
            c.w32(0, t53);
            let r54 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000827d1d); };
            c.w32(3, t55);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000827d20); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000827d24); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000827d32); };
            c.cmp(t56, c.r32(3), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000827d3f); };
            c.cmp(t57, c.r32(3), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000827d48); };
            c.w64(3, t58);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 109; continue; }
            blk = 33; continue;
        }
        33 => {
            let r59 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r59);
            c.w64(2, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827d67); };
            c.w64(15, t60);
            let Some(t61) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000827d6e); };
            c.w64(15, t61);
            c.w64(1, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000827d75); };
            c.w64(8, t62);
            let Some(t63) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000827d7c); };
            c.cmp(t63, c.r64(1), 64);
            if c.cc_be() { blk = 107; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 107; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827d92); };
            c.w64(2, t64);
            if !c.st64(c.r64(2), c.r64(1)) { return RegfileExit::fault(0x7000827d99); }
            let r65 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r65);
            blk = 36; continue;
        }
        36 => {
            c.w64(2, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000827da8); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000827dac); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000827db6); };
            c.w64(12, t66);
            blk = 38; continue;
        }
        38 => {
            let r67 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r67);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(12)) { return RegfileExit::fault(0x7000827dc1); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 41; continue;
        }
        40 => {
            c.w32(12, 0x1u64);
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 43; continue;
        }
        43 => {
            let r68 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r68);
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x7000827dea); };
            c.w32(12, t69);
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000827dee); };
            c.cmp(t70, c.r32(12), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(12)) { return RegfileExit::fault(0x7000827df7); }
            blk = 46; continue;
        }
        45 => {
            c.w32(2, 0x1u64);
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 43; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 87; continue;
        }
        48 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 87; continue;
        }
        49 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 121; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827e36); };
            c.w64(2, t71);
            let Some(t72) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000827e3d); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827e40); };
            c.w64(2, t73);
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let r74 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r74);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000827e50); }
            c.w32(15, 0x4u64);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000827e58); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000827e61); };
            c.w64(3, t75);
            c.w64(15, c.r64(3));
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x7000827e73, 0x7000827e6e);
        }
        53 => {
            blk = 56; continue;
        }
        54 => {
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827e82); };
            c.w64(15, t76);
            if !c.st64(c.r64(15), c.r64(8)) { return RegfileExit::fault(0x7000827e89); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 56; continue;
        }
        56 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000827e93); };
            c.w64(7, t77);
            c.w32(9, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000827e9b); };
            c.w64(3, t78);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000827e9f); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r79 = c.r64(11) & c.r64(0);
            c.w64(11, r79);
            c.logic(r79, 64);
            let Some(t80) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000827eac); };
            c.logic(t80 & 0x4u64, 8);
            if c.cc_ne() { blk = 128; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000827eb7); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000827ebb); };
            c.w64(9, t81);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000827ec2); }
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000827ec8); };
            c.w64(9, t82);
            c.w64(11, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(11));
            blk = 59; continue;
        }
        59 => {
            let r83 = c.sub(c.r64(15), 0x8u64, 64);
            c.w64(15, r83);
            if !c.movsd_mr(c.r64(15).wrapping_add(c.r64(0)), 0) { return RegfileExit::fault(0x7000827eed); }
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 62; continue;
        }
        61 => {
            c.w32(15, 0x1u64);
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(15, c.r64(11));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000827f11); };
            c.w64(3, t84);
            c.w64(2, 0xfff7fffffff7ffffu64);
            blk = 64; continue;
        }
        64 => {
            let r85 = c.sub(c.r64(15), 0x8u64, 64);
            c.w64(15, r85);
            let Some(t86) = c.ld64(c.r64(15).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000827f23); };
            c.cmp(t86, c.r64(2), 64);
            if c.cc_e() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(c.r64(3))) { return RegfileExit::fault(0x7000827f29); }
            if !c.movsd_mr(c.r64(15).wrapping_add(c.r64(0)), 0) { return RegfileExit::fault(0x7000827f2f); }
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 64; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 85; continue;
        }
        67 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 119; continue; }
            blk = 68; continue;
        }
        68 => {
            let r87 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r87);
            c.w64(15, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827f61); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000827f68); };
            c.w64(1, t89);
            c.cmp(c.r64(0), 0x7u64, 64);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.w64(6, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000827f79); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000827f80); };
            c.cmp(t91, c.r64(6), 64);
            if c.cc_be() { blk = 115; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 115; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827f96); };
            c.w64(15, t92);
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x7000827f9d); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 71; continue;
        }
        71 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000827fa4); };
            c.w64(1, t93);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000827fab); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000827faf); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(7, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000827fc0); };
            c.w64(0, t94);
            blk = 73; continue;
        }
        73 => {
            let r95 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r95);
            if !c.st32(c.r64(7).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x7000827fcb); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 73; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(7, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            blk = 77; continue;
        }
        76 => {
            let r96 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r96);
            blk = 77; continue;
        }
        77 => {
            let r97 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r97);
            let Some(t98) = c.ld64(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000827ff1); };
            c.cmp(t98, c.r64(2), 64);
            if c.cc_e() { blk = 82; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(3))) { return RegfileExit::fault(0x7000827ff7); }
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000827ffd); };
            c.w64(9, t99);
            let Some(t100) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000828004); };
            c.w64(12, t100);
            c.w64(0, c.r64(12).wrapping_add(0xc));
            c.w64(1, c.r64(7).wrapping_sub(0x4));
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000828010); };
            c.w64(6, t101);
            let Some(t102) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000828017); };
            c.cmp(t102, c.r64(0), 64);
            if c.cc_be() { blk = 117; continue; }
            blk = 79; continue;
        }
        79 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x7000828020); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 80; continue;
        }
        80 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000828028); };
            c.w64(12, t103);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700082802f); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000828033); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x7000828039); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r104 = c.r64(12) & c.r64(15);
            c.w64(12, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082804d); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 125; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 76; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w32(2, 0x1u64);
            c.w64(0, c.r64(15));
            blk = 85; continue;
        }
        84 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 85; continue;
        }
        85 => {
            c.w64(15, c.r64(0));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082806e); };
            c.w64(11, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000828072); };
            c.w64(7, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000828076); };
            c.w64(6, t108);
            blk = 87; continue;
        }
        86 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082807c); };
            c.w64(0, t109);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(0));
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            let r110 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r110);
            let r111 = c.r32(11) & 0x3fu64;
            c.w32(11, r111);
            c.logic(r111, 32);
            c.w64(0, c.r64(15));
            blk = 90; continue;
        }
        89 => {
            c.w32(11, 0x2u64);
            c.w64(0, c.r64(15));
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_le() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w32(11, 0x2u64);
            blk = 92; continue;
        }
        92 => {
            c.w64(9, c.r64(6));
            let Some(t112) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008280b2); };
            c.w32(9, t112);
            let r113 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r113);
            let Some(t114) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008280b9); };
            c.w32(9, t114);
            let r115 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r115);
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008280c0); };
            c.w32(12, t116);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r117 = c.add(c.r64(11), 0x69u64, 64);
            c.w64(11, r117);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            return RegfileExit::fault(0x70008280d6);
        }
        94 => {
            let Some(t118) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008280d7); };
            c.w32(9, t118);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008280dc); };
            c.w64(11, t119);
            let Some(t120) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008280e3); };
            c.w64(12, t120);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008280eb); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008280f2); };
            c.cmp(t122, c.r64(15), 64);
            if c.cc_be() { blk = 123; continue; }
            blk = 95; continue;
        }
        95 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008280fb); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 96; continue;
        }
        96 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000828103); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000828107); }
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082810b); };
            c.w64(7, t123);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000828112); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000828116); }
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000828120); }
            return c.ret(0x0, 0x7000828121);
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000828122); }
            return c.call(0x70007febb4, 0x700082812b, 0x7000828126);
        }
        98 => {
            c.w64(9, c.r64(0));
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082812e); };
            c.w64(7, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000828132); };
            c.w64(6, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000828136); };
            c.w64(11, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082813a); };
            c.w64(12, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082813e); };
            c.w64(0, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000828142); };
            c.w64(3, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828146); };
            c.w64(8, t130);
            blk = 8; continue;
        }
        99 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000828153); };
            c.w64(3, t131);
            c.w64(2, c.r64(0));
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000828165, 0x7000828160);
        }
        100 => {
            return RegfileExit::fault(0x7000828165);
        }
        101 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000828166); };
            c.w64(3, t132);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000828176, 0x7000828171);
        }
        102 => {
            return RegfileExit::fault(0x7000828176);
        }
        103 => {
            let r133 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r133);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082817a); }
            c.w32(9, 0x4u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000828182); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700082818b); };
            c.w64(3, t134);
            c.w64(9, c.r64(3));
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x700082819d, 0x7000828198);
        }
        104 => {
            blk = 23; continue;
        }
        105 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008281a2); };
            c.w64(3, t135);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008281b2, 0x70008281ad);
        }
        106 => {
            return RegfileExit::fault(0x70008281b2);
        }
        107 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008281b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008281b7); }
            return c.call(0x70007febb4, 0x70008281c0, 0x70008281bb);
        }
        108 => {
            c.w64(15, c.r64(0));
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008281c3); };
            c.w64(7, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008281c7); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008281cb); };
            c.w64(11, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008281cf); };
            c.w64(0, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008281d3); };
            c.w64(9, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008281d7); };
            c.w64(3, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008281db); };
            c.w64(12, t142);
            blk = 36; continue;
        }
        109 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008281e4); };
            c.w64(3, t143);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008281f4, 0x70008281ef);
        }
        110 => {
            return RegfileExit::fault(0x70008281f4);
        }
        111 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008281f5); };
            c.w64(3, t144);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000828205, 0x7000828200);
        }
        112 => {
            return RegfileExit::fault(0x7000828205);
        }
        113 => {
            let r145 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r145);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000828209); }
            c.w32(15, 0x4u64);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000828211); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700082821a); };
            c.w64(3, t146);
            c.w64(15, c.r64(3));
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700082822c, 0x7000828227);
        }
        114 => {
            blk = 56; continue;
        }
        115 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000828231); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x7000828235); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000828239); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700082823d); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x7000828249, 0x7000828244);
        }
        116 => {
            c.w64(15, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082824c); };
            c.w64(3, t147);
            let Some(t148) = c.ld64(0x7000827f17u64) else { return RegfileExit::fault(0x7000828250); };
            c.w64(2, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000828257); };
            c.w64(11, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082825b); };
            c.w64(9, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082825f); };
            c.w64(12, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000828263); };
            c.w64(0, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000828267); };
            c.w64(7, t153);
            blk = 71; continue;
        }
        117 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000828275); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000828279); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700082827d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000828281); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000828285); }
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x7000828292, 0x700082828d);
        }
        118 => {
            c.w64(9, c.r64(0));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000828295); };
            c.w64(15, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828299); };
            c.w64(7, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082829d); };
            c.w64(11, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008282a1); };
            c.w64(1, t157);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008282a5); }
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008282aa); };
            c.w64(3, t158);
            let Some(t159) = c.ld64(0x7000827f17u64) else { return RegfileExit::fault(0x70008282ae); };
            c.w64(2, t159);
            blk = 80; continue;
        }
        119 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008282ba); };
            c.w64(3, t160);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008282ca, 0x70008282c5);
        }
        120 => {
            return RegfileExit::fault(0x70008282ca);
        }
        121 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008282cb); };
            c.w64(3, t161);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008282db, 0x70008282d6);
        }
        122 => {
            return RegfileExit::fault(0x70008282db);
        }
        123 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008282e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008282e5); }
            return c.call(0x70007febf4, 0x70008282ee, 0x70008282e9);
        }
        124 => {
            c.w64(11, c.r64(0));
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008282f1); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008282f5); };
            c.w64(7, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008282f9); };
            c.w64(9, t164);
            blk = 96; continue;
        }
        125 => {
            c.w32(9, c.r32(9));
            let r165 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r165);
            c.w64(12, 0xfffffffffffc0000u64);
            let r166 = c.r64(12) & c.r64(9);
            c.w64(12, r166);
            c.logic(r166, 64);
            let Some(t167) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000828312); };
            c.logic(t167 & 0x2u64, 8);
            if c.cc_e() { blk = 82; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(0, c.r64(15).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828322); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000828323); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000828330, 0x700082832b);
        }
        127 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000828330); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000828332); }
            blk = 82; continue;
        }
        128 => {
            c.w32(3, c.r32(3));
            let r168 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r168);
            c.w64(11, 0xfffffffffffc0000u64);
            let r169 = c.r64(11) & c.r64(3);
            c.w64(11, r169);
            c.logic(r169, 64);
            let Some(t170) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000828347); };
            c.logic(t170 & 0x2u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(12, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828356); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000828357); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000828364, 0x700082835f);
        }
        130 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000828364); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000828366); }
            blk = 57; continue;
        }
        131 => {
            c.w32(9, c.r32(9));
            let r171 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r171);
            c.w64(11, 0xfffffffffffc0000u64);
            let r172 = c.r64(11) & c.r64(9);
            c.w64(11, r172);
            c.logic(r172, 64);
            let Some(t173) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082837c); };
            c.logic(t173 & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(12, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082838b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082838c); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000828399, 0x7000828394);
        }
        133 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000828399); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700082839b); }
            blk = 24; continue;
        }
        134 => {
            return RegfileExit::jump(0x70008283a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008283b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008283b4); };
            c.w32(7, t0);
            let r1 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1);
            let Some(t2) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008283ba); };
            c.w32(7, t2);
            let r3 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r3);
            let r4 = c.r32(7) & 0x3fu64;
            c.w32(7, r4);
            c.logic(r4, 32);
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008283c4); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r7 = c.add(c.r64(7), 0x69u64, 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008283d2); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008283d9); };
            c.w32(11, t10);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008283e8);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008283e9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008283ed); }
            let r11 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r11);
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008283f3); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008283fd); };
            c.w32(7, t14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000828402); };
            c.w32(9, t15);
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700082840c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000828410); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000828414); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000828418); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000828426); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000828433); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700082843c); };
            c.w64(9, t18);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 33; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(0, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000828458); };
            c.w64(12, t19);
            let Some(t20) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082845f); };
            c.w64(12, t20);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(1, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082846b); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000828472); };
            c.cmp(t22, c.r64(1), 64);
            if c.cc_be() { blk = 31; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 31; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000828488); };
            c.w64(2, t23);
            if !c.st64(c.r64(2), c.r64(1)) { return RegfileExit::fault(0x700082848f); }
            let r24 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r24);
            blk = 10; continue;
        }
        10 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000828499); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700082849e); }
            c.w64(9, c.r64(12));
            let r25 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r25);
            c.logic(r25, 64);
            c.w64(0, c.r64(3).wrapping_mul(2).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008284b5); }
            let Some(t26) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008284b9); };
            c.logic(t26 & 0x4u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(9, c.r64(3).wrapping_add(c.r64(15)));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_e() { blk = 28; continue; }
            blk = 12; continue;
        }
        12 => {
            let r27 = c.neg(c.r64(3), 64);
            c.w64(3, r27);
            blk = 13; continue;
        }
        13 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008284db); };
            c.w32(2, t28);
            c.w64(1, c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)));
            if !c.st32(c.r64(12).wrapping_add(c.r64(1)), c.r32(2)) { return RegfileExit::fault(0x70008284e3); }
            let r29 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r29);
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_ne() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 28; continue;
        }
        15 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(11)));
            c.w64(1, c.r64(12).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r30 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r30);
            let r31 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r31);
            c.logic(r31, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000828509); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700082850e); };
            c.w64(0, t32);
            c.w64(10, 0x700082851cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082851c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000828520); }
            return c.call(c.r64(0), 0x7000828526, 0x7000828524);
        }
        16 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000828526); }
            let Some(t33) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082852e); };
            c.w64(4, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000828533); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000828537); };
            c.w64(15, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082853b); };
            c.w64(12, t36);
            blk = 28; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 39; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(12).wrapping_mul(4));
            c.w64(12, c.r64(8).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700082855d); }
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 35; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000828571); };
            c.w64(3, t37);
            let Some(t38) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000828578); };
            c.w64(2, t38);
            let Some(t39) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082857b); };
            c.w64(1, t39);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700082857f); }
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let r40 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700082858f); }
            c.w32(12, 0x4u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000828597); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700082859b); };
            c.w64(3, t41);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008285ac, 0x70008285a7);
        }
        21 => {
            blk = 24; continue;
        }
        22 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(2)));
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008285bb); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008285c5); };
            c.w64(7, t42);
            c.w32(9, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008285cd); };
            c.w64(11, t43);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008285d1); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r44 = c.r64(12) & c.r64(0);
            c.w64(12, r44);
            c.logic(r44, 64);
            let Some(t45) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008285df); };
            c.logic(t45 & 0x4u64, 8);
            if c.cc_ne() { blk = 43; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008285eb); }
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008285ef); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008285f3); };
            c.w64(9, t47);
            c.w64(2, c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r48 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r48);
            let r49 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r49);
            c.logic(r49, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082860b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000828610); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828614); };
            c.w64(8, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000828618); };
            c.w64(0, t51);
            c.w64(10, 0x7000828626u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000828626); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082862a); }
            return c.call(c.r64(0), 0x7000828630, 0x700082862e);
        }
        26 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000828630); }
            let Some(t52) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000828638); };
            c.w64(4, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082863d); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000828641); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000828645); };
            c.w64(15, t55);
            blk = 28; continue;
        }
        27 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082864b); };
            c.w64(0, t56);
            c.w64(12, c.r64(0));
            blk = 28; continue;
        }
        28 => {
            c.w64(9, c.r64(15));
            let r57 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082865b); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000828662); };
            c.w64(15, t59);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000828669); };
            c.w64(3, t60);
            let Some(t61) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000828670); };
            c.cmp(t61, c.r64(0), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x7000828679); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 30; continue;
        }
        30 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000828680); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000828684); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000828688); };
            c.w64(7, t62);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700082868f); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000828693); }
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082869d); }
            return c.ret(0x0, 0x700082869e);
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700082869f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008286a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008286a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008286ab); }
            return c.call(0x70007febb4, 0x70008286b4, 0x70008286af);
        }
        32 => {
            c.w64(12, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008286b7); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008286bb); };
            c.w64(15, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008286bf); };
            c.w64(11, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008286c3); };
            c.w64(3, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008286c7); };
            c.w64(8, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008286cb); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008286cf); };
            c.w64(0, t69);
            blk = 10; continue;
        }
        33 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008286dc); };
            c.w64(3, t70);
            return c.call(0x700086ed74, 0x70008286e8, 0x70008286e3);
        }
        34 => {
            return RegfileExit::fault(0x70008286e8);
        }
        35 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008286e9); };
            c.w64(3, t71);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008286f9, 0x70008286f4);
        }
        36 => {
            return RegfileExit::fault(0x70008286f9);
        }
        37 => {
            let r72 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r72);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008286fd); }
            c.w32(12, 0x4u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000828705); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000828707); };
            c.w64(3, t73);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082871a, 0x7000828715);
        }
        38 => {
            blk = 24; continue;
        }
        39 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700082871f); };
            c.w64(3, t74);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082872f, 0x700082872a);
        }
        40 => {
            return RegfileExit::fault(0x700082872f);
        }
        41 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x7000828735); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000828739); }
            return c.call(0x70007febf4, 0x7000828742, 0x700082873d);
        }
        42 => {
            c.w64(11, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000828745); };
            c.w64(12, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000828749); };
            c.w64(7, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082874d); };
            c.w64(9, t77);
            blk = 30; continue;
        }
        43 => {
            c.w32(11, c.r32(11));
            let r78 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r78);
            c.w64(12, 0xfffffffffffc0000u64);
            let r79 = c.r64(12) & c.r64(11);
            c.w64(12, r79);
            c.logic(r79, 64);
            let Some(t80) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000828766); };
            c.logic(t80 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828776); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000828777); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000828784, 0x700082877f);
        }
        45 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000828784); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000828786); }
            blk = 25; continue;
        }
        46 => {
            return RegfileExit::jump(0x7000828790);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000828794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000828794); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000828798); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828799); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082879a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082879f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008287a9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008287b5); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008287bb); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008287c7); };
            c.w32(1, t6);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4au64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008287d6);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x12f)) else { return RegfileExit::fault(0x70008287d7); };
            c.w32(1, t7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008287dd); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008287e4); };
            c.w64(8, t9);
            c.w64(9, c.r64(8).wrapping_add(0x18));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008287eb); };
            c.w64(11, t10);
            let Some(t11) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008287f2); };
            c.cmp(t11, c.r64(9), 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008287f7); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008287fe); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000828801); };
            c.w64(1, t12);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000828808); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700082880b); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700082880e); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x7000828813); }
            c.w32(1, 0x4u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x700082881b); }
            c.w64(0, c.r64(7));
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000828821); };
            c.w64(1, t13);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000828828); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x700082882f);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000828832); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000828839); }
            return c.ret(0x0, 0x700082883b);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700082883c); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000828840); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082884e, 0x7000828849);
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082884e); };
            c.w64(6, t15);
            blk = 1; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000828857); }
            return c.call(0x70008a2054, 0x7000828860, 0x700082885b);
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828860); };
            c.w64(6, t16);
            blk = 3; continue;
        }
        14 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082886e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000828872); }
            return c.call(0x70007febb4, 0x700082887b, 0x7000828876);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082887e); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828882); };
            c.w64(1, t18);
            blk = 7; continue;
        }
        16 => {
            return RegfileExit::jump(0x700082888c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000828894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000828894); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000828898); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828899); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082889a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082889f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008288a9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008288b5); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008288bb); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008288c7); };
            c.w32(1, t6);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4au64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008288d6);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x12f)) else { return RegfileExit::fault(0x70008288d7); };
            c.w32(1, t7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008288dd); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008288e4); };
            c.w64(8, t9);
            c.w64(9, c.r64(8).wrapping_add(0x18));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008288eb); };
            c.w64(11, t10);
            let Some(t11) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008288f2); };
            c.cmp(t11, c.r64(9), 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008288f7); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008288fe); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000828901); };
            c.w64(1, t12);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000828908); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700082890b); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700082890e); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x7000828913); }
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x7000828916); }
            c.w64(0, c.r64(7));
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082891c); };
            c.w64(1, t13);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000828923); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x700082892a);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082892d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000828934); }
            return c.ret(0x0, 0x7000828936);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000828937); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082893b); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000828949, 0x7000828944);
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828949); };
            c.w64(6, t15);
            blk = 1; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000828952); }
            return c.call(0x70008a2054, 0x700082895b, 0x7000828956);
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082895b); };
            c.w64(6, t16);
            blk = 3; continue;
        }
        14 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000828969); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700082896d); }
            return c.call(0x70007febb4, 0x7000828976, 0x7000828971);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000828979); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082897d); };
            c.w64(1, t18);
            blk = 7; continue;
        }
        16 => {
            return RegfileExit::jump(0x7000828988);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000828994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000828994); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000828998); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828999); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082899a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082899f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008289a9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008289b5); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008289bb); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008289c7); };
            c.w32(1, t6);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4au64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008289d6);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x12f)) else { return RegfileExit::fault(0x70008289d7); };
            c.w32(1, t7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008289dd); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008289e4); };
            c.w64(8, t9);
            c.w64(9, c.r64(8).wrapping_add(0x18));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008289eb); };
            c.w64(11, t10);
            let Some(t11) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008289f2); };
            c.cmp(t11, c.r64(9), 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008289f7); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008289fe); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000828a01); };
            c.w64(1, t12);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000828a08); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000828a0b); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x7000828a0e); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x7000828a13); }
            c.w32(1, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x7000828a1b); }
            c.w64(0, c.r64(7));
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000828a21); };
            c.w64(1, t13);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000828a28); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x7000828a2f);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000828a32); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000828a39); }
            return c.ret(0x0, 0x7000828a3b);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000828a3c); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000828a40); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000828a4e, 0x7000828a49);
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828a4e); };
            c.w64(6, t15);
            blk = 1; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000828a57); }
            return c.call(0x70008a2054, 0x7000828a60, 0x7000828a5b);
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828a60); };
            c.w64(6, t16);
            blk = 3; continue;
        }
        14 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000828a6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000828a72); }
            return c.call(0x70007febb4, 0x7000828a7b, 0x7000828a76);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000828a7e); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828a82); };
            c.w64(1, t18);
            blk = 7; continue;
        }
        16 => {
            return RegfileExit::jump(0x7000828a8c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000828a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000828a94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000828a98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828a99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000828a9a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000828a9f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000828aa3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 187; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000828aad); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 304; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000828abb); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000828ac2); };
            c.cmp(t5, 0x836u64, 16);
            if c.cc_ne() { blk = 304; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000828acf); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000828ad6); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000828adc); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            c.w32(11, c.r32(3));
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t12) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000828af1); };
            c.w32(15, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000828af6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000828afa); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000828afe); }
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 122; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0x82eu64, 16);
            if c.cc_ne() { blk = 189; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 10; continue;
        }
        7 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000828b28); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(10, 0x8000000000000000u64);
            let r13 = c.r64(11) | c.r64(10);
            c.w64(11, r13);
            c.logic(r13, 64);
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000828b62); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000828b69); };
            c.w32(15, t16);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000828b6d); }
            c.w64(0, c.r64(15));
            let r17 = c.r32(0) & 0x1u64;
            c.w32(0, r17);
            c.logic(r17, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000828b7f); };
            c.logic(t18 & 0x4u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000828b8b); };
            c.w64(12, t19);
            c.w64(0, c.r64(12));
            blk = 44; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000828b97); };
            c.w32(15, t20);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t21) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000828bb0); };
            c.w64(12, t21);
            let Some(t22) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000828bb5); };
            c.w64(15, t22);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t23) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000828bbe); };
            c.w64(0, t23);
            let r24 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r24);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t25) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000828bce); };
            c.w64(12, t25);
            c.w64(0, c.r64(12));
            blk = 44; continue;
        }
        19 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t26) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000828be3); };
            c.w32(0, t26);
            let r27 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            let r28 = c.r32(0) & 0x3fu64;
            c.w32(0, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(0), 0x21u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(0), 0x24u64, 32);
            if c.cc_l() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(0), 0x24u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(0), 0x25u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(0), 0x26u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(0), 0x21u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(0), 0x22u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(0), 0x23u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(0), 0x1eu64, 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(0), 0x1eu64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(0), 0x1fu64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(0), 0x20u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(0), 0x1cu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(0), 0x1du64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::fault(0x7000828c38);
        }
        35 => {
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            blk = 39; continue;
        }
        36 => {
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            blk = 39; continue;
        }
        37 => {
            c.w32(0, 0x4u64);
            c.w64(1, c.r64(0));
            blk = 39; continue;
        }
        38 => {
            c.w32(0, 0x8u64);
            c.w64(1, c.r64(0));
            blk = 39; continue;
        }
        39 => {
            let r29 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r29);
            c.w64(0, c.r64(12));
            c.cqo();
            c.idiv(c.r64(1), 64);
            blk = 44; continue;
        }
        40 => {
            c.w32(7, 0x66u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828c71); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000828c72); };
            c.w64(7, t30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000828c79); };
            c.w32(7, t31);
            let r32 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r32);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000828c7f); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000828c80); };
            c.w64(3, t33);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000828c91, 0x7000828c8c);
        }
        41 => {
            return RegfileExit::fault(0x7000828c91);
        }
        42 => {
            c.w64(10, c.r64(4));
            let r34 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            let r35 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r35);
            c.logic(r35, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000828c9d); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000828ca8); };
            c.w64(0, t36);
            c.w64(10, 0x7000828cb6u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000828cb6); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000828cba); }
            return c.call(c.r64(0), 0x7000828cc0, 0x7000828cbe);
        }
        43 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000828cc0); }
            let Some(t37) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000828cc8); };
            c.w64(4, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000828ccd); };
            c.w64(11, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000828cd1); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000828cd5); };
            c.w64(6, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000828cd9); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000828cdd); };
            c.w64(8, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000828ce1); };
            c.w64(9, t43);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000828cea); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000828cf1); };
            c.w64(7, t45);
            blk = 182; continue;
        }
        46 => {
            c.w64(12, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 217; continue; }
            blk = 47; continue;
        }
        47 => {
            let r46 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r46);
            blk = 48; continue;
        }
        48 => {
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x7000828d11); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r47 = c.r64(15) & c.r64(7);
            c.w64(15, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000828d25); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 315; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t49) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000828d30); };
            c.w32(12, t49);
            let r50 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t51) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000828d41); };
            c.w32(9, t51);
            let r52 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r52);
            c.w64(0, c.r64(8));
            let Some(t53) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000828d4d); };
            c.w64(8, t53);
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000828d51); };
            c.w32(12, t54);
            let r55 = c.r32(9) & 0x3fu64;
            c.w32(9, r55);
            c.logic(r55, 32);
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_l() { blk = 70; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 62; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_l() { blk = 58; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 226; continue;
        }
        58 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 226; continue;
        }
        62 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 66; continue;
        }
        66 => {
            blk = 226; continue;
        }
        67 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 226; continue;
        }
        70 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_l() { blk = 80; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 76; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 226; continue;
        }
        76 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 79; continue;
        }
        79 => {
            blk = 226; continue;
        }
        80 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_l() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 226; continue;
        }
        85 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            blk = 226; continue;
        }
        88 => {
            let r56 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r56);
            let Some(t57) = c.ld8(c.r64(11).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000828e7a); };
            c.w32(8, t57);
            let r58 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r58);
            blk = 163; continue;
        }
        89 => {
            let r59 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r59);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000828e8a); };
            c.w32(8, (t60 as u8 as i8 as i64 as u64));
            let r61 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r61);
            blk = 163; continue;
        }
        90 => {
            let r62 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r62);
            let Some(t63) = c.ld16(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x7000828e9a); };
            c.w32(8, t63);
            let r64 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r64);
            blk = 163; continue;
        }
        91 => {
            let r65 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r65);
            let Some(t66) = c.ld16(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x7000828eaa); };
            c.w32(8, (t66 as u16 as i16 as i64 as u64));
            let r67 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r67);
            blk = 163; continue;
        }
        92 => {
            let r68 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r68);
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x7000828eba); };
            c.w32(8, t69);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 227; continue; }
            blk = 93; continue;
        }
        93 => {
            let r70 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r70);
            blk = 163; continue;
        }
        94 => {
            let r71 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r71);
            let Some(t72) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x7000828ed6); };
            c.w32(8, t72);
            c.w64(9, c.r64(8));
            let r73 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r73);
            if c.cc_o() { blk = 232; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 163; continue;
        }
        96 => {
            let r74 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r74);
            if !c.movss_rm(0, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4))) { return RegfileExit::fault(0x7000828ef1); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000828ef7); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000828efe); };
            c.w64(9, t76);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000828f05); };
            c.w64(12, t77);
            let Some(t78) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000828f0c); };
            c.cmp(t78, c.r64(11), 64);
            if c.cc_be() { blk = 237; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000828f16); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 98; continue;
        }
        98 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000828f21); };
            c.w64(9, t79);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000828f28); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000828f2c); }
            blk = 163; continue;
        }
        99 => {
            let r80 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r80);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) { return RegfileExit::fault(0x7000828f3a); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000828f40); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000828f47); };
            c.w64(9, t82);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000828f4e); };
            c.w64(12, t83);
            let Some(t84) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000828f55); };
            c.cmp(t84, c.r64(11), 64);
            if c.cc_be() { blk = 239; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000828f5f); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000828f66); };
            c.w64(9, t85);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000828f6d); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000828f71); }
            blk = 163; continue;
        }
        102 => {
            let r86 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r86);
            let Some(t87) = c.ld8(c.r64(11).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000828f7f); };
            c.w32(8, t87);
            let r88 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r88);
            blk = 163; continue;
        }
        103 => {
            let r89 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r89);
            let Some(t90) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000828f8f); };
            c.w64(8, t90);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 107; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000828f99); };
            c.w64(9, t91);
            let Some(t92) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000828fa0); };
            c.w64(11, t92);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000828fa7); };
            c.w64(15, t93);
            let Some(t94) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000828fae); };
            c.cmp(t94, c.r64(12), 64);
            if c.cc_be() { blk = 241; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000828fb7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 106; continue;
        }
        106 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000828fbe); };
            c.w64(11, t95);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000828fc5); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000828fc9); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000828fd1); }
            blk = 110; continue;
        }
        107 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000828fd7); };
            c.w64(8, t96);
            let Some(t97) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000828fde); };
            c.w64(9, t97);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000828fe5); };
            c.w64(11, t98);
            let Some(t99) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000828fec); };
            c.cmp(t99, c.r64(12), 64);
            if c.cc_be() { blk = 243; continue; }
            blk = 108; continue;
        }
        108 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000828ff5); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 109; continue;
        }
        109 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000828ffc); };
            c.w64(9, t100);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000829003); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x7000829007); }
            c.w64(9, c.r64(8));
            blk = 110; continue;
        }
        110 => {
            c.w64(8, c.r64(9));
            blk = 163; continue;
        }
        111 => {
            let r101 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r101);
            let Some(t102) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700082901d); };
            c.w64(8, t102);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 117; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000829027); };
            c.w64(9, t103);
            let Some(t104) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082902e); };
            c.w64(11, t104);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000829035); };
            c.w64(15, t105);
            let Some(t106) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082903c); };
            c.cmp(t106, c.r64(12), 64);
            if c.cc_be() { blk = 245; continue; }
            blk = 113; continue;
        }
        113 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000829045); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 114; continue;
        }
        114 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700082904c); };
            c.w64(11, t107);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000829053); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700082905d); }
            let r108 = c.neg(c.r64(8), 64);
            c.w64(8, r108);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000829068); }
            blk = 120; continue;
        }
        116 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700082906e); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000829076); }
            blk = 120; continue;
        }
        117 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082907c); };
            c.w64(8, t109);
            let Some(t110) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000829083); };
            c.w64(9, t110);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082908a); };
            c.w64(11, t111);
            let Some(t112) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000829091); };
            c.cmp(t112, c.r64(12), 64);
            if c.cc_be() { blk = 247; continue; }
            blk = 118; continue;
        }
        118 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700082909a); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 119; continue;
        }
        119 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008290a1); };
            c.w64(9, t113);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008290a8); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008290ac); }
            c.w64(9, c.r64(8));
            blk = 120; continue;
        }
        120 => {
            c.w64(8, c.r64(9));
            blk = 163; continue;
        }
        121 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008290bf); };
            c.w64(7, t114);
            blk = 182; continue;
        }
        122 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(3));
            let r115 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r115);
            blk = 124; continue;
        }
        124 => {
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008290db); };
            c.w32(12, t116);
            let r117 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r117);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 250; continue; }
            blk = 125; continue;
        }
        125 => {
            let r118 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r118);
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ae() { blk = 251; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w32(12, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r32(12), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 258; continue; }
            blk = 128; continue;
        }
        128 => {
            let r119 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r119);
            blk = 129; continue;
        }
        129 => {
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700082910c); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r120 = c.r64(15) & c.r64(7);
            c.w64(15, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000829120); };
            c.logic(t121 & 0x4u64, 8);
            if c.cc_ne() { blk = 312; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t122) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082912b); };
            c.w32(12, t122);
            let r123 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r123);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t124) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x700082913c); };
            c.w32(12, t124);
            let r125 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r125);
            c.w32(11, c.r32(11));
            c.w64(0, c.r64(8));
            let Some(t126) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082914b); };
            c.w32(8, t126);
            let r127 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r127);
            let r128 = c.r32(12) & 0x3fu64;
            c.w32(12, r128);
            c.logic(r128, 32);
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_l() { blk = 138; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(12), 0xbu64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 137; continue;
        }
        137 => {
            blk = 263; continue;
        }
        138 => {
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(12), 0x7u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 263; continue;
        }
        142 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_l() { blk = 147; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 146; continue;
        }
        146 => {
            blk = 263; continue;
        }
        147 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 263; continue;
        }
        151 => {
            c.w64(9, c.r64(8));
            let Some(t129) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008291d7); };
            c.w32(8, t129);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 279; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t130) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008291ea); };
            c.w32(8, t130);
            blk = 163; continue;
        }
        153 => {
            let Some(t131) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008291f4); };
            c.w32(12, t131);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 280; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t132) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000829207); };
            c.w32(8, t132);
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082920c); };
            c.cmp(t133, c.r32(8), 32);
            if c.cc_e() { blk = 281; continue; }
            blk = 155; continue;
        }
        155 => {
            blk = 163; continue;
        }
        156 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x700082921e); }
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000829225); };
            c.w64(8, t134);
            let Some(t135) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082922c); };
            c.w64(9, t135);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000829233); };
            c.w64(12, t136);
            let Some(t137) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082923a); };
            c.cmp(t137, c.r64(11), 64);
            if c.cc_be() { blk = 282; continue; }
            blk = 157; continue;
        }
        157 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000829244); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 158; continue;
        }
        158 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082924b); };
            c.w64(9, t138);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000829252); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000829256); }
            blk = 163; continue;
        }
        159 => {
            c.w64(12, 0xfff7fffffff7ffffu64);
            let Some(t139) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000829268); };
            c.cmp(t139, c.r64(12), 64);
            if c.cc_e() { blk = 286; continue; }
            blk = 160; continue;
        }
        160 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000829273); }
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082927a); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000829281); };
            c.w64(9, t141);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000829288); };
            c.w64(12, t142);
            let Some(t143) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082928f); };
            c.cmp(t143, c.r64(11), 64);
            if c.cc_be() { blk = 284; continue; }
            blk = 161; continue;
        }
        161 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000829299); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 162; continue;
        }
        162 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008292a0); };
            c.w64(9, t144);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008292a7); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008292ab); }
            blk = 163; continue;
        }
        163 => {
            let Some(t145) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008292b1); };
            c.w32(7, t145);
            let r146 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r146);
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t147) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008292bf); };
            c.w32(7, t147);
            let r148 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r148);
            let Some(t149) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008292c5); };
            c.w32(7, t149);
            let r150 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r150);
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008292cb); };
            c.w64(9, t151);
            let Some(t152) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008292d2); };
            c.w64(11, t152);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008292d9); };
            c.w64(15, t153);
            let Some(t154) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008292e0); };
            c.cmp(t154, c.r64(12), 64);
            if c.cc_be() { blk = 294; continue; }
            blk = 165; continue;
        }
        165 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008292e9); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 166; continue;
        }
        166 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008292f0); };
            c.w64(11, t155);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008292f7); }
            c.w32(11, 0x4u64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000829301); }
            let Some(t156) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829305); };
            c.w32(12, t156);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 296; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000829319); }
            let Some(t157) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082931d); };
            c.w32(12, t157);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 297; continue; }
            blk = 168; continue;
        }
        168 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000829331); }
            let Some(t158) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829335); };
            c.w32(8, t158);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            return RegfileExit::fault(0x7000829345);
        }
        170 => {
            let Some(t159) = c.ld32(c.r64(7).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000829346); };
            c.w32(8, t159);
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082934d); };
            c.w64(12, t160);
            let Some(t161) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000829354); };
            c.w64(15, t161);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082935c); };
            c.w64(3, t162);
            let Some(t163) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000829363); };
            c.cmp(t163, c.r64(0), 64);
            if c.cc_be() { blk = 298; continue; }
            blk = 171; continue;
        }
        171 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700082936c); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 172; continue;
        }
        172 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000829374); }
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000829379); };
            c.w64(8, t164);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000829380); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000829385); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700082938a); }
            let Some(t165) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082938f); };
            c.w32(9, t165);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 174; continue; }
            blk = 173; continue;
        }
        173 => {
            return RegfileExit::fault(0x700082939f);
        }
        174 => {
            let Some(t166) = c.ld32(c.r64(7).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008293a0); };
            c.w32(7, t166);
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008293a6); };
            c.w64(9, t167);
            let Some(t168) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008293ad); };
            c.w64(11, t168);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008293b4); };
            c.w64(0, t169);
            let Some(t170) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008293bb); };
            c.cmp(t170, c.r64(15), 64);
            if c.cc_be() { blk = 300; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008293c4); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 176; continue;
        }
        176 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008293cb); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008293cf); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008293d3); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008293d7); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008293db); };
            c.w64(7, t171);
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008293e2); }
            c.w64(0, c.r64(9));
            blk = 177; continue;
        }
        177 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008293e9); };
            c.w64(1, t172);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008293f0); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            return c.ret(0x8, 0x70008293f7);
        }
        179 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008293fa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000829401); }
            return c.ret(0x0, 0x7000829403);
        }
        180 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000829404); };
            c.w64(7, t173);
            c.w64(3, c.r64(8));
            blk = 182; continue;
        }
        181 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000829410); };
            c.w64(7, t174);
            blk = 182; continue;
        }
        182 => {
            let Some(t175) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000829417); };
            c.w32(8, t175);
            let r176 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r176);
            let Some(t177) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082941e); };
            c.w32(8, t177);
            let r178 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r178);
            let Some(t179) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829425); };
            c.w32(9, t179);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            return RegfileExit::fault(0x7000829435);
        }
        184 => {
            let Some(t180) = c.ld32(c.r64(8).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x7000829436); };
            c.w32(8, t180);
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082943d); };
            c.w64(9, t181);
            let Some(t182) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000829444); };
            c.w64(11, t182);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082944b); };
            c.w64(15, t183);
            let Some(t184) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000829452); };
            c.cmp(t184, c.r64(12), 64);
            if c.cc_be() { blk = 302; continue; }
            blk = 185; continue;
        }
        185 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700082945b); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 186; continue;
        }
        186 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000829462); }
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000829466); };
            c.w64(8, t185);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700082946d); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000829471); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000829475); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000829479); }
            c.w64(0, c.r64(9));
            blk = 177; continue;
        }
        187 => {
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000829485); };
            c.w64(3, t186);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000829493, 0x700082948e);
        }
        188 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829493); };
            c.w64(6, t187);
            blk = 1; continue;
        }
        189 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700082949c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008294a0); }
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008294a4); };
            c.w64(3, t188);
            c.w64(0, c.r64(8));
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70008294b6, 0x70008294b1);
        }
        190 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008294b6); };
            c.w64(6, t189);
            return c.call(0x70008a17f4, 0x70008294bf, 0x70008294ba);
        }
        191 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t190) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008294c9); };
            c.cmp(t190, 0x0u64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008294cf); };
            c.w64(3, t191);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008294d3); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008294dd); }
            blk = 197; continue;
        }
        194 => {
            let r192 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r192);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 197; continue;
        }
        195 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 199; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008294f5); };
            c.w64(3, t193);
            let r194 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r194);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008294fb); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829508); };
            c.w64(3, t195);
            blk = 197; continue;
        }
        197 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_b() { blk = 201; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829512); };
            c.w64(0, t196);
            blk = 252; continue;
        }
        199 => {
            let Some(t197) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082951b); };
            c.cmp(c.r32(0), t197, 32);
            if c.cc_le() { blk = 216; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829524); };
            c.w64(3, t198);
            blk = 201; continue;
        }
        201 => {
            let Some(t199) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000829528); };
            c.cmp(t199, 0x0u64, 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700082952e); }
            blk = 206; continue;
        }
        203 => {
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000829535); };
            c.w64(11, t200);
            let r201 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r201);
            if c.cc_o() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 211; continue;
        }
        205 => {
            c.w64(7, c.r64(3));
            let r202 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r202);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 206; continue;
        }
        206 => {
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000829551); };
            c.w64(7, t203);
            let Some(t204) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000829558); };
            c.w64(8, t204);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700082955f); }
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000829564); };
            c.w64(11, t205);
            let Some(t206) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700082956b); };
            c.cmp(t206, c.r64(9), 64);
            if c.cc_a() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700082957a, 0x7000829575);
        }
        208 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700082957a); }
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082957f); };
            c.w64(3, t207);
            blk = 210; continue;
        }
        209 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000829585); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 210; continue;
        }
        210 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008295a1); };
            c.w64(7, t208);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008295a8); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008295ab); }
            c.w64(7, c.r64(0));
            blk = 211; continue;
        }
        211 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008295b3); };
            c.w64(8, t209);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008295b7); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r210 = c.r64(9) & c.r64(8);
            c.w64(9, r210);
            c.logic(r210, 64);
            let Some(t211) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008295cb); };
            c.logic(t211 & 0x4u64, 8);
            if c.cc_ne() { blk = 309; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t212) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008295d6); };
            c.w32(7, t212);
            let r213 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r213);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 215; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(7, c.r64(8));
            blk = 292; continue;
        }
        215 => {
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008295e9); };
            c.w64(7, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008295f0); };
            c.w64(6, t215);
            blk = 182; continue;
        }
        216 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008295f9); };
            c.w64(0, t216);
            blk = 252; continue;
        }
        217 => {
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000829602); };
            c.w64(15, t217);
            let Some(t218) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000829609); };
            c.w64(0, t218);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000829610); }
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000829614); };
            c.w64(1, t219);
            let Some(t220) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082961b); };
            c.cmp(t220, c.r64(2), 64);
            if c.cc_a() { blk = 220; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700082962a, 0x7000829625);
        }
        219 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082962a); };
            c.w64(12, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082962e); };
            c.w64(7, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829632); };
            c.w64(6, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829636); };
            c.w64(3, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082963a); };
            c.w64(8, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082963e); };
            c.w64(9, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000829642); };
            c.w64(11, t227);
            blk = 221; continue;
        }
        220 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x7000829648); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 221; continue;
        }
        221 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 225; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            let r228 = c.r64(10) | 0x1u64;
            c.w64(10, r228);
            c.logic(r228, 64);
            blk = 224; continue;
        }
        224 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 225; continue;
        }
        225 => {
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000829679); };
            c.w64(12, t229);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000829680); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000829684); }
            c.w64(12, c.r64(0));
            blk = 48; continue;
        }
        226 => {
            return RegfileExit::fault(0x7000829691);
        }
        227 => {
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000829692); };
            c.w64(9, t230);
            let Some(t231) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000829699); };
            c.w64(11, t231);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008296a0); }
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008296a4); };
            c.w64(15, t232);
            let Some(t233) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008296ab); };
            c.cmp(t233, c.r64(12), 64);
            if c.cc_a() { blk = 230; continue; }
            blk = 228; continue;
        }
        228 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008296ba, 0x70008296b5);
        }
        229 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008296ba); };
            c.w64(8, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008296be); };
            c.w64(7, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008296c2); };
            c.w64(6, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008296c6); };
            c.w64(3, t237);
            blk = 231; continue;
        }
        230 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008296cc); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 231; continue;
        }
        231 => {
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008296d6); };
            c.w64(9, t238);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008296dd); }
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008296ed); }
            c.w64(8, c.r64(0));
            blk = 163; continue;
        }
        232 => {
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008296fa); };
            c.w64(9, t239);
            let Some(t240) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000829701); };
            c.w64(11, t240);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000829708); }
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082970c); };
            c.w64(15, t241);
            let Some(t242) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000829713); };
            c.cmp(t242, c.r64(12), 64);
            if c.cc_a() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000829722, 0x700082971d);
        }
        234 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829722); };
            c.w64(8, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829726); };
            c.w64(7, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082972a); };
            c.w64(6, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082972e); };
            c.w64(3, t246);
            blk = 236; continue;
        }
        235 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000829734); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 236; continue;
        }
        236 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000829747); };
            c.w64(8, t247);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700082974e); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000829752); }
            c.w64(8, c.r64(0));
            blk = 163; continue;
        }
        237 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000829764); }
            return c.call(0x70007febf4, 0x700082976e, 0x7000829769);
        }
        238 => {
            c.w64(8, c.r64(0));
            let Some(t248) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829771); };
            c.w64(7, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829775); };
            c.w64(6, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829779); };
            c.w64(3, t250);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700082977d); }
            blk = 98; continue;
        }
        239 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700082978c); }
            return c.call(0x70007febf4, 0x7000829796, 0x7000829791);
        }
        240 => {
            c.w64(8, c.r64(0));
            let Some(t251) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829799); };
            c.w64(7, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082979d); };
            c.w64(6, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008297a1); };
            c.w64(3, t253);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008297a5); }
            blk = 101; continue;
        }
        241 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008297b4); }
            return c.call(0x70007febb4, 0x70008297bd, 0x70008297b8);
        }
        242 => {
            c.w64(9, c.r64(0));
            let Some(t254) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008297c0); };
            c.w64(7, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008297c4); };
            c.w64(6, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008297c8); };
            c.w64(3, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008297cc); };
            c.w64(8, t257);
            blk = 106; continue;
        }
        243 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008297df, 0x70008297da);
        }
        244 => {
            c.w64(8, c.r64(0));
            let Some(t258) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008297e2); };
            c.w64(7, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008297e6); };
            c.w64(6, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008297ea); };
            c.w64(3, t260);
            blk = 109; continue;
        }
        245 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008297f8); }
            return c.call(0x70007febb4, 0x7000829801, 0x70008297fc);
        }
        246 => {
            c.w64(9, c.r64(0));
            let Some(t261) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829804); };
            c.w64(7, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829808); };
            c.w64(6, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082980c); };
            c.w64(3, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829810); };
            c.w64(8, t264);
            blk = 114; continue;
        }
        247 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000829823, 0x700082981e);
        }
        248 => {
            c.w64(8, c.r64(0));
            let Some(t265) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829826); };
            c.w64(7, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082982a); };
            c.w64(6, t266);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082982e); };
            c.w64(3, t267);
            blk = 119; continue;
        }
        249 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000829837); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            blk = 124; continue;
        }
        250 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000829846); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            blk = 126; continue;
        }
        251 => {
            c.w64(0, c.r64(8));
            blk = 252; continue;
        }
        252 => {
            let Some(t268) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082985a); };
            c.w32(7, t268);
            let r269 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r269);
            let Some(t270) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000829860); };
            c.cmp(t270, 0x834u64, 16);
            if c.cc_e() { blk = 254; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000829868); };
            c.w64(7, t271);
            let Some(t272) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082986f); };
            c.w32(7, t272);
            let r273 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r273);
            blk = 255; continue;
        }
        254 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000829877); };
            c.w64(7, t274);
            let Some(t275) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082987e); };
            c.w32(7, t275);
            let r276 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r276);
            blk = 255; continue;
        }
        255 => {
            let Some(t277) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829884); };
            c.w64(8, t277);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000829888); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 257; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r278 = c.r64(9) & c.r64(8);
            c.w64(9, r278);
            c.logic(r278, 64);
            let Some(t279) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082989c); };
            c.logic(t279 & 0x4u64, 8);
            if c.cc_ne() { blk = 306; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t280) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008298a7); };
            c.w64(3, t280);
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008298ae); };
            c.w64(7, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008298b5); };
            c.w64(6, t282);
            blk = 182; continue;
        }
        258 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008298be); };
            c.w64(15, t283);
            let Some(t284) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008298c5); };
            c.w64(0, t284);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008298cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008298d0); }
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008298d4); };
            c.w64(1, t285);
            let Some(t286) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008298db); };
            c.cmp(t286, c.r64(2), 64);
            if c.cc_a() { blk = 261; continue; }
            blk = 259; continue;
        }
        259 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008298ea, 0x70008298e5);
        }
        260 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008298ea); };
            c.w64(12, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008298ee); };
            c.w64(7, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008298f2); };
            c.w64(6, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008298f6); };
            c.w64(3, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008298fa); };
            c.w64(8, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008298fe); };
            c.w64(9, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000829902); };
            c.w64(11, t293);
            blk = 262; continue;
        }
        261 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x7000829908); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 262; continue;
        }
        262 => {
            c.w32(10, c.r32(12));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082991e); };
            c.w64(12, t294);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000829925); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000829929); }
            c.w64(12, c.r64(0));
            blk = 129; continue;
        }
        263 => {
            let Some(t295) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000829936); };
            c.w32(12, t295);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r296 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r296);
            c.w32(2, c.r32(11));
            c.w64(10, c.r64(4));
            let r297 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r297);
            let r298 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r298);
            c.logic(r298, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000829952); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000829957); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700082995b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700082995f); }
            c.w64(1, c.r64(13));
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x7000829966); };
            c.w64(0, t299);
            c.w64(10, 0x7000829974u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000829974); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000829978); }
            return c.call(c.r64(0), 0x700082997e, 0x700082997c);
        }
        264 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082997e); }
            let Some(t300) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000829986); };
            c.w64(4, t300);
            c.w32(7, c.r32(0));
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082998d); };
            let r302 = c.r64(7) & t301;
            c.w64(7, r302);
            c.logic(r302, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000829997); };
            c.w64(7, t303);
            c.w64(11, c.r64(8));
            blk = 265; continue;
        }
        265 => {
            c.w64(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x4));
            c.w64(15, c.r64(12).wrapping_mul(4));
            let Some(t304) = c.ld32(c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008299ab); };
            c.w32(0, t304);
            let r305 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r305);
            let Some(t306) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008299b3); };
            c.cmp(t306, c.r32(0), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 266; continue;
        }
        266 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 271; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t307) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008299c4); };
            c.cmp(t307, c.r32(0), 32);
            if c.cc_e() { blk = 272; continue; }
            blk = 268; continue;
        }
        268 => {
            c.xorpd_rr(0, 0);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008299d1); };
            c.cvtsi2sd(0, (t308 as i64));
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008299d7); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 272; continue; }
            blk = 269; continue;
        }
        269 => {
            if c.cc_e() { blk = 273; continue; }
            blk = 270; continue;
        }
        270 => {
            blk = 272; continue;
        }
        271 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008299ec); };
            c.cmp(c.r64(0), t309, 64);
            if c.cc_e() { blk = 273; continue; }
            blk = 272; continue;
        }
        272 => {
            let r310 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r310);
            let r311 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r311);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008299f9); };
            c.w64(12, t312);
            let r313 = c.r64(12) & c.r64(9);
            c.w64(12, r313);
            c.logic(r313, 64);
            c.w64(9, c.r64(12));
            blk = 265; continue;
        }
        273 => {
            let Some(t314) = c.ld32(c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000829a05); };
            c.w32(8, t314);
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 275; continue; }
            blk = 274; continue;
        }
        274 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829a10); };
            c.w64(3, t315);
            blk = 292; continue;
        }
        275 => {
            let Some(t316) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829a19); };
            c.w32(8, t316);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            return RegfileExit::fault(0x7000829a2d);
        }
        277 => {
            let Some(t317) = c.ld32(c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000829a2e); };
            c.w32(8, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829a33); };
            c.w64(3, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829a37); };
            c.w64(7, t319);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829a3b); };
            c.w64(6, t320);
            blk = 163; continue;
        }
        278 => {
            let Some(t321) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829a44); };
            c.w64(7, t321);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829a48); };
            c.w64(6, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829a4c); };
            c.w64(3, t323);
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829a50); };
            c.w64(0, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000829a54); };
            c.w64(9, t325);
            blk = 287; continue;
        }
        279 => {
            return RegfileExit::fault(0x7000829a5a);
        }
        280 => {
            return RegfileExit::fault(0x7000829a5b);
        }
        281 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 287; continue;
        }
        282 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000829a66); }
            return c.call(0x70007febf4, 0x7000829a70, 0x7000829a6b);
        }
        283 => {
            c.w64(8, c.r64(0));
            let Some(t326) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829a73); };
            c.w64(7, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829a77); };
            c.w64(6, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829a7b); };
            c.w64(3, t328);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000829a7f); }
            blk = 158; continue;
        }
        284 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000829a8e); }
            return c.call(0x70007febf4, 0x7000829a98, 0x7000829a93);
        }
        285 => {
            c.w64(8, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829a9b); };
            c.w64(7, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829a9f); };
            c.w64(6, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829aa3); };
            c.w64(3, t331);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000829aa7); }
            blk = 162; continue;
        }
        286 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 287; continue;
        }
        287 => {
            let Some(t332) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000829ab4); };
            c.w64(11, t332);
            let Some(t333) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000829abb); };
            c.cmp(t333, c.r32(8), 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t334) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000829ac1); };
            c.w32(8, t334);
            let r335 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r335);
            let Some(t336) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000829ac8); };
            c.w32(8, t336);
            let r337 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r337);
            let Some(t338) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829acf); };
            c.w32(11, t338);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            return RegfileExit::fault(0x7000829adf);
        }
        290 => {
            let Some(t339) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000829ae0); };
            c.w32(8, t339);
            let Some(t340) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000829ae7); };
            c.cmp(t340, c.r32(8), 32);
            if c.cc_ne() { blk = 292; continue; }
            blk = 291; continue;
        }
        291 => {
            let Some(t341) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000829aed); };
            c.w64(8, t341);
            blk = 163; continue;
        }
        292 => {
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829af9); };
            c.w64(0, t342);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829afd); };
            c.w64(6, t343);
            return c.call(0x7000870914, 0x7000829b06, 0x7000829b01);
        }
        293 => {
            c.w64(8, c.r64(0));
            let Some(t344) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000829b09); };
            c.w64(7, t344);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829b0d); };
            c.w64(6, t345);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829b11); };
            c.w64(3, t346);
            blk = 163; continue;
        }
        294 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000829b1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000829b23); }
            return c.call(0x70007febf4, 0x7000829b2c, 0x7000829b27);
        }
        295 => {
            c.w64(9, c.r64(0));
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829b2f); };
            c.w64(3, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829b33); };
            c.w64(8, t348);
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000829b37); };
            c.w64(7, t349);
            blk = 166; continue;
        }
        296 => {
            return RegfileExit::fault(0x7000829b40);
        }
        297 => {
            return RegfileExit::fault(0x7000829b41);
        }
        298 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000829b47); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000829b4b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000829b4f); }
            return c.call(0x70007febf4, 0x7000829b58, 0x7000829b53);
        }
        299 => {
            c.w64(12, c.r64(0));
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000829b5b); };
            c.w64(7, t350);
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000829b5f); };
            c.w64(9, t351);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829b63); };
            c.w64(8, t352);
            c.w32(11, 0x4u64);
            blk = 172; continue;
        }
        300 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x7000829b77); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000829b7b); }
            return c.call(0x70007febf4, 0x7000829b84, 0x7000829b7f);
        }
        301 => {
            c.w64(9, c.r64(0));
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829b87); };
            c.w64(12, t353);
            let Some(t354) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000829b8b); };
            c.w64(8, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829b92); };
            c.w64(7, t355);
            blk = 176; continue;
        }
        302 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000829ba0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000829ba4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000829ba8); }
            return c.call(0x70007febf4, 0x7000829bb1, 0x7000829bac);
        }
        303 => {
            c.w64(9, c.r64(0));
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000829bb4); };
            c.w64(7, t356);
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000829bb8); };
            c.w64(3, t357);
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000829bbc); };
            c.w64(8, t358);
            blk = 186; continue;
        }
        304 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000829bcb); }
            let Some(t359) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000829bcd); };
            c.w64(8, t359);
            let Some(t360) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000829bd4); };
            c.w32(8, t360);
            let r361 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r361);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000829bdb); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000829bdd); }
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000829bde); };
            c.w64(3, t362);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000829bef, 0x7000829bea);
        }
        305 => {
            return RegfileExit::fault(0x7000829bef);
        }
        306 => {
            c.w32(7, c.r32(7));
            let r363 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r363);
            c.w64(9, 0xfffffffffffc0000u64);
            let r364 = c.r64(9) & c.r64(7);
            c.w64(9, r364);
            c.logic(r364, 64);
            let Some(t365) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000829bff); };
            c.logic(t365 & 0x2u64, 8);
            if c.cc_e() { blk = 257; continue; }
            blk = 307; continue;
        }
        307 => {
            c.w64(11, c.r64(8).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000829c0e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000829c0f); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000829c1c, 0x7000829c17);
        }
        308 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000829c1c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000829c1e); }
            blk = 257; continue;
        }
        309 => {
            c.w32(7, c.r32(7));
            let r366 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r366);
            c.w64(9, 0xfffffffffffc0000u64);
            let r367 = c.r64(9) & c.r64(7);
            c.w64(9, r367);
            c.logic(r367, 64);
            let Some(t368) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000829c33); };
            c.logic(t368 & 0x2u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 310; continue;
        }
        310 => {
            c.w64(11, c.r64(8).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000829c42); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000829c43); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000829c50, 0x7000829c4b);
        }
        311 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000829c50); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000829c52); }
            blk = 213; continue;
        }
        312 => {
            c.w32(12, c.r32(12));
            let r369 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r369);
            c.w64(15, 0xfffffffffffc0000u64);
            let r370 = c.r64(15) & c.r64(12);
            c.w64(15, r370);
            c.logic(r370, 64);
            let Some(t371) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000829c68); };
            c.logic(t371 & 0x2u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 313; continue;
        }
        313 => {
            c.w64(0, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000829c77); }
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000829c81, 0x7000829c7c);
        }
        314 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000829c81); }
            blk = 131; continue;
        }
        315 => {
            c.w32(12, c.r32(12));
            let r372 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r372);
            c.w64(15, 0xfffffffffffc0000u64);
            let r373 = c.r64(15) & c.r64(12);
            c.w64(15, r373);
            c.logic(r373, 64);
            let Some(t374) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000829c98); };
            c.logic(t374 & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 316; continue;
        }
        316 => {
            c.w64(0, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000829ca7); }
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000829cb1, 0x7000829cac);
        }
        317 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000829cb1); }
            blk = 50; continue;
        }
        318 => {
            return RegfileExit::jump(0x7000829cbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000829cd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000829cd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000829cd8); }
            let r0 = c.sub(c.r64(4), 0x70u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000829cde); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000829ce2); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000829ce6); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x7000829cea); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000829cee); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000829cf2); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 301; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x7000829d07); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x7000829d0b); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000829d14); };
            c.w64(3, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(0)) { return RegfileExit::fault(0x7000829d18); }
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000829d1c); };
            c.w64(1, t3);
            blk = 3; continue;
        }
        3 => {
            c.w32(15, c.r32(11));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x7000829d23); }
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x7000829d36); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000829d40); }
            let Some(t4) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000829d46); };
            c.cmp(t4, 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000829d4c); }
            blk = 9; continue;
        }
        6 => {
            c.w64(0, c.r64(1));
            let r5 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r5);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 9; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000829d62); };
            c.cmp(t6, 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(11));
            let r7 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r7);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000829d6d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 9; continue;
        }
        9 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 300; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_le() { blk = 300; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000829d8f); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 323; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000829d9c); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000829da2); };
            c.w32(8, t10);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 323; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 317; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t11) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000829dc9); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000829dd0); };
            c.w32(3, t13);
            c.cmp(c.r16(3), 0x42u64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000829dda); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(2) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(1, 0x1fffffffffffffu64);
            let r14 = c.add(c.r64(1), c.r64(2), 64);
            c.w64(1, r14);
            c.w64(6, 0x3ffffffffffffeu64);
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_be() { blk = 109; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000829e16); };
            c.w64(1, t15);
            blk = 21; continue;
        }
        21 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000829e1a); };
            c.w64(6, t16);
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_e() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_ae() { blk = 303; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829e2e); };
            c.w32(2, t17);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 323; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(3), 0x25u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(3), 0x2du64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(3) & 0x20u64, 8);
            if c.cc_ne() { blk = 323; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(3, c.r64(11));
            blk = 32; continue;
        }
        29 => {
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000829e5d); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            blk = 32; continue;
        }
        30 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r20 = c.r64(2) & 0xffffffu64;
            c.w64(2, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000829e71); };
            c.w64(3, t21);
            blk = 111; continue;
        }
        31 => {
            c.w64(3, c.r64(11));
            blk = 32; continue;
        }
        32 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000829e7d); };
            c.w64(2, t22);
            blk = 34; continue;
        }
        33 => {
            let Some(t23) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000829e83); };
            c.w32(0, t23);
            let r24 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r24);
            let Some(t25) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000829e89); };
            c.w32(8, t25);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 63; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000829e9b); };
            c.w32(1, t26);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000829eaa); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            c.logic(c.r32(1) & 0xffc00u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 37; continue;
        }
        37 => {
            let r29 = c.shr(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r29);
            let r30 = c.r32(1) & 0x3ffu64;
            c.w32(1, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_be() { blk = 52; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t31) = c.ld16(c.r64(2).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000829ece); };
            c.w32(6, (t31 as u16 as i16 as i64 as u64));
            let Some(t32) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829ed2); };
            c.w32(7, t32);
            let r33 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r33);
            let r34 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000829edb); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 40; continue;
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000829ee4); }
            blk = 40; continue;
        }
        40 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000829ee8); };
            c.w64(11, t35);
            let r36 = c.sub(c.r32(11), c.r32(15), 32);
            c.w32(11, r36);
            let r37 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.add(c.r32(11), c.r32(15), 32);
            c.w32(11, r38);
            c.w32(12, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000829efc); };
            c.w32(12, t39);
            let r40 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r41);
            let r42 = c.r32(12) & 0x3ffu64;
            c.w32(12, r42);
            c.logic(r42, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000829f16); };
            c.w32(12, t43);
            let r44 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r44);
            let Some(t45) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829f1e); };
            c.w32(12, t45);
            let r46 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r46);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_be() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(15, c.r64(11).wrapping_add(0x1));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000829f30); };
            c.w64(11, t47);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000829f39); }
            blk = 45; continue;
        }
        44 => {
            let r48 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r48);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_l() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000829f4f); };
            c.w32(11, t49);
            let r50 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r50);
            let r51 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r51);
            let r52 = c.r32(11) & 0x3ffu64;
            c.w32(11, r52);
            c.logic(r52, 32);
            c.w32(12, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000829f69); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829f71); };
            c.w32(0, t55);
            let r56 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r56);
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_a() { blk = 244; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000829f8b); };
            c.w64(0, t57);
            blk = 74; continue;
        }
        50 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000829f94); };
            c.w64(0, t58);
            blk = 74; continue;
        }
        51 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000829f9d); };
            c.w64(0, t59);
            blk = 74; continue;
        }
        52 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let r60 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r60);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 74; continue; }
            blk = 53; continue;
        }
        53 => {
            let r61 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r61);
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000829fbb); };
            c.cmp(t62, c.r32(3), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 53; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 74; continue;
        }
        56 => {
            let Some(t63) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829fd1); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000829fde); };
            c.w64(11, t65);
            blk = 58; continue;
        }
        58 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000829fe5); };
            c.w32(12, t66);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r67 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r67);
            let Some(t68) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000829ff3); };
            c.w32(15, t68);
            let r69 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r69);
            let r70 = c.r64(15) & c.r64(12);
            c.w64(15, r70);
            c.logic(r70, 64);
            c.w64(2, c.r64(9));
            blk = 60; continue;
        }
        59 => {
            let r71 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r71);
            let r72 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r72);
            c.w64(1, c.r64(12));
            let r73 = c.r64(1) & c.r64(15);
            c.w64(1, r73);
            c.logic(r73, 64);
            c.w64(15, c.r64(1));
            blk = 60; continue;
        }
        60 => {
            c.w64(1, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t74) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a018); };
            c.w32(1, t74);
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082a01d); };
            c.cmp(t75, c.r32(1), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 59; continue;
        }
        63 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 311; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t76) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082a041); };
            c.logic(t76 & 0x24u64, 8);
            if c.cc_ne() { blk = 316; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t77) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082a04b); };
            c.w32(11, t77);
            let r78 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r78);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700082a058); };
            c.w64(11, t79);
            blk = 67; continue;
        }
        67 => {
            let Some(t80) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082a05f); };
            c.w32(12, t80);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r81 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r81);
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082a06d); };
            c.w32(15, t82);
            let r83 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            let r84 = c.r64(15) & c.r64(12);
            c.w64(15, r84);
            c.logic(r84, 64);
            c.w64(2, c.r64(9));
            blk = 69; continue;
        }
        68 => {
            let r85 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r85);
            let r86 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r86);
            c.w64(1, c.r64(12));
            let r87 = c.r64(1) & c.r64(15);
            c.w64(1, r87);
            c.logic(r87, 64);
            c.w64(15, c.r64(1));
            blk = 69; continue;
        }
        69 => {
            c.w64(1, c.r64(15).wrapping_add(0x5));
            let Some(t88) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a091); };
            c.w32(6, t88);
            let r89 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r89);
            let Some(t90) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082a099); };
            c.cmp(t90, c.r32(6), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082a0a2); };
            c.cmp(t91, c.r32(6), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082a0ab); };
            c.cmp(t92, c.r32(3), 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t93) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082a0b0); };
            c.w32(11, t93);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_ae() { blk = 349; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t94) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a0c3); };
            c.w32(11, t94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082a0c7); };
            c.cmp(t95, c.r32(11), 32);
            if c.cc_ne() { blk = 244; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t96) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a0e1); };
            c.w32(8, t96);
            let r97 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r97);
            let Some(t98) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a0e8); };
            c.w32(8, t98);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 105; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t99) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a0f8); };
            c.w32(11, t99);
            let r100 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r100);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 105; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(12, c.r64(3));
            c.w64(8, c.r64(9));
            blk = 80; continue;
        }
        80 => {
            c.w64(15, c.r64(11));
            let r101 = c.r32(15) & 0x7u64;
            c.w32(15, r101);
            c.logic(r101, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 305; continue;
        }
        85 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            blk = 305; continue;
        }
        88 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082a163); };
            c.w32(11, t102);
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700082a168); };
            c.cmp(t103, c.r32(11), 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a175); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a17d); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a185); };
            c.w32(11, t108);
            blk = 80; continue;
        }
        90 => {
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082a18c); };
            c.w32(11, t109);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r110 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r110);
            let Some(t111) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a19a); };
            c.w32(12, t111);
            let r112 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r112);
            let Some(t113) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a1a2); };
            c.w32(11, t113);
            let r114 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r114);
            let Some(t115) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a1aa); };
            c.w32(11, t115);
            blk = 80; continue;
        }
        91 => {
            let Some(t116) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a1b4); };
            c.w32(12, t116);
            let r117 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r117);
            let Some(t118) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a1bc); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a1c4); };
            c.w32(11, t120);
            blk = 80; continue;
        }
        92 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 95; continue;
        }
        93 => {
            c.w32(15, 0x1u64);
            blk = 95; continue;
        }
        94 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(9));
            c.w64(12, c.r64(3));
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 307; continue; }
            blk = 96; continue;
        }
        96 => {
            let r121 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r121);
            blk = 97; continue;
        }
        97 => {
            let r122 = c.r32(11) & 0x8u64;
            c.w32(11, r122);
            c.logic(r122, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t123) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x700082a1fc); };
            c.w32(8, t123);
            blk = 100; continue;
        }
        99 => {
            let Some(t124) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700082a203); };
            c.w32(8, t124);
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 315; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 312; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082a236); };
            c.w64(11, t125);
            let Some(t126) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a23a); };
            c.w32(8, t126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700082a23e); };
            c.cmp(t127, c.r32(8), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t128) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a247); };
            c.logic(t128 & 0x1u64, 8);
            if c.cc_ne() { blk = 245; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t129) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082a252); };
            c.w32(2, t129);
            let r130 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r130);
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700082a258); };
            c.cmp(t131, c.r32(2), 32);
            if c.cc_e() { blk = 245; continue; }
            blk = 108; continue;
        }
        108 => {
            blk = 33; continue;
        }
        109 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082a26a); };
            c.w64(3, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082a26e); };
            c.w64(1, t133);
            blk = 111; continue;
        }
        110 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(2, c.r64(12));
            blk = 111; continue;
        }
        111 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700082a27d); }
            c.w64(12, c.r64(8));
            c.w64(8, c.r64(3));
            blk = 113; continue;
        }
        112 => {
            let Some(t134) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a289); };
            c.w32(0, t134);
            let r135 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r135);
            let Some(t136) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a290); };
            c.w32(12, t136);
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(12), 0x410u64, 32);
            if c.cc_le() { blk = 323; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t137) = c.ld8(c.r64(0).wrapping_add(0xa)) else { return RegfileExit::fault(0x700082a2a2); };
            c.w32(12, t137);
            let r138 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r138);
            let r139 = c.r32(12) & 0x3fu64;
            c.w32(12, r139);
            c.logic(r139, 32);
            c.cmp(c.r32(12), 0x15u64, 32);
            if c.cc_l() { blk = 149; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_l() { blk = 133; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(12), 0x23u64, 32);
            if c.cc_l() { blk = 125; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(12), 0x25u64, 32);
            if c.cc_l() { blk = 122; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(12), 0x25u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(12), 0x26u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(12), 0x28u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 121; continue;
        }
        121 => {
            blk = 323; continue;
        }
        122 => {
            c.cmp(c.r32(12), 0x23u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(12), 0x24u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 323; continue;
        }
        125 => {
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_l() { blk = 130; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(12), 0x21u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(12), 0x22u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 129; continue;
        }
        129 => {
            blk = 323; continue;
        }
        130 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(12), 0x1fu64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 132; continue;
        }
        132 => {
            blk = 323; continue;
        }
        133 => {
            c.cmp(c.r32(12), 0x19u64, 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(12), 0x1bu64, 32);
            if c.cc_l() { blk = 139; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(12), 0x1bu64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(12), 0x1cu64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(12), 0x1du64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 138; continue;
        }
        138 => {
            blk = 323; continue;
        }
        139 => {
            c.cmp(c.r32(12), 0x19u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(12), 0x1au64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 323; continue;
        }
        142 => {
            c.cmp(c.r32(12), 0x17u64, 32);
            if c.cc_l() { blk = 146; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(12), 0x17u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(12), 0x18u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 145; continue;
        }
        145 => {
            blk = 323; continue;
        }
        146 => {
            c.cmp(c.r32(12), 0x15u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(12), 0x16u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 148; continue;
        }
        148 => {
            blk = 323; continue;
        }
        149 => {
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_l() { blk = 167; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(12), 0x10u64, 32);
            if c.cc_l() { blk = 159; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(12), 0x12u64, 32);
            if c.cc_l() { blk = 156; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(12), 0x12u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(12), 0x13u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r32(12), 0x14u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 155; continue;
        }
        155 => {
            blk = 323; continue;
        }
        156 => {
            c.cmp(c.r32(12), 0x10u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(12), 0x11u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 158; continue;
        }
        158 => {
            blk = 323; continue;
        }
        159 => {
            c.cmp(c.r32(12), 0xbu64, 32);
            if c.cc_l() { blk = 164; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(12), 0xbu64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(12), 0xcu64, 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(12), 0xfu64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 163; continue;
        }
        163 => {
            blk = 323; continue;
        }
        164 => {
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 166; continue;
        }
        166 => {
            blk = 323; continue;
        }
        167 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_l() { blk = 176; continue; }
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_l() { blk = 173; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(12), 0x7u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 172; continue;
        }
        172 => {
            blk = 323; continue;
        }
        173 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 175; continue;
        }
        175 => {
            blk = 323; continue;
        }
        176 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 180; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 179; continue;
        }
        179 => {
            blk = 323; continue;
        }
        180 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 181; continue;
        }
        181 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            blk = 323; continue;
        }
        183 => {
            blk = 188; continue;
        }
        184 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a4cb); };
            c.w32(8, t140);
            let r141 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r141);
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082a4d2); };
            c.w32(12, t142);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_ae() { blk = 189; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w64(12, 0xfff7fffffff7ffffu64);
            let Some(t143) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a4eb); };
            c.cmp(t143, c.r64(12), 64);
            if c.cc_e() { blk = 207; continue; }
            blk = 186; continue;
        }
        186 => {
            blk = 244; continue;
        }
        187 => {
            let Some(t144) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a4fb); };
            c.w32(12, t144);
            let r145 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r145);
            let Some(t146) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a502); };
            c.w32(12, t146);
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_b() { blk = 244; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t147) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a510); };
            c.w32(8, t147);
            let r148 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r148);
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082a517); };
            c.w32(12, t149);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_b() { blk = 191; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 323; continue; }
            blk = 190; continue;
        }
        190 => {
            blk = 207; continue;
        }
        191 => {
            let Some(t150) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a53a); };
            c.w32(8, t150);
            let Some(t151) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082a53f); };
            c.cmp(t151, c.r32(8), 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 192; continue;
        }
        192 => {
            blk = 244; continue;
        }
        193 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a551); };
            c.w32(12, t152);
            let r153 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r153);
            let Some(t154) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a558); };
            c.w32(12, t154);
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_b() { blk = 244; continue; }
            blk = 194; continue;
        }
        194 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x700082a566); }
            c.w32(12, 0xfffffffeu64);
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_b() { blk = 323; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t155) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a579); };
            c.w32(8, t155);
            let r156 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r156);
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082a580); };
            c.w32(12, t157);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r158 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r158);
            c.w32(6, c.r32(2));
            c.w64(10, c.r64(4));
            let r159 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r159);
            let r160 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r160);
            c.logic(r160, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082a59b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700082a5a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700082a5a4); }
            c.w64(2, c.r64(6));
            c.w64(1, c.r64(13));
            c.w64(7, c.r64(1));
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700082a5b1); };
            c.w64(0, t161);
            c.w64(10, 0x700082a5bfu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082a5bf); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082a5c3); }
            return c.call(c.r64(0), 0x700082a5c9, 0x700082a5c7);
        }
        196 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082a5c9); }
            let Some(t162) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082a5d1); };
            c.w64(4, t162);
            c.w32(8, c.r32(0));
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082a5d9); };
            let r164 = c.r64(8) & t163;
            c.w64(8, r164);
            c.logic(r164, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082a5e0); };
            c.w64(11, t165);
            blk = 198; continue;
        }
        197 => {
            let r166 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r166);
            let r167 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082a5ed); };
            c.w64(12, t168);
            let r169 = c.r64(12) & c.r64(8);
            c.w64(12, r169);
            c.logic(r169, 64);
            c.w64(8, c.r64(12));
            blk = 198; continue;
        }
        198 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t170) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a5fc); };
            c.w32(12, t170);
            let r171 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082a604); };
            c.cmp(t172, c.r32(12), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 199; continue;
        }
        199 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 204; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t173) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082a613); };
            c.cmp(t173, c.r32(12), 32);
            if c.cc_e() { blk = 197; continue; }
            blk = 201; continue;
        }
        201 => {
            c.xorpd_rr(0, 0);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082a620); };
            c.cvtsi2sd(0, (t174 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082a626); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 197; continue; }
            blk = 202; continue;
        }
        202 => {
            if c.cc_e() { blk = 244; continue; }
            blk = 203; continue;
        }
        203 => {
            blk = 197; continue;
        }
        204 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082a641); };
            c.cmp(c.r64(12), t175, 64);
            if c.cc_e() { blk = 244; continue; }
            blk = 205; continue;
        }
        205 => {
            blk = 197; continue;
        }
        206 => {
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082a64d); };
            c.w64(0, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082a651); };
            c.w64(11, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082a655); };
            c.w64(2, t178);
            blk = 207; continue;
        }
        207 => {
            let Some(t179) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082a659); };
            c.w32(8, t179);
            let r180 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082a660); };
            c.w64(1, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082a664); };
            c.w64(3, t182);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t183) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700082a66b); };
            c.cmp(t183, c.r32(8), 32);
            if c.cc_e() { blk = 245; continue; }
            blk = 208; continue;
        }
        208 => {
            blk = 112; continue;
        }
        209 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a67d); };
            c.w32(9, t184);
            let r185 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r185);
            let Some(t186) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082a684); };
            c.logic(t186 & 0x4u64, 8);
            if c.cc_ne() { blk = 245; continue; }
            blk = 210; continue;
        }
        210 => {
            c.w64(9, c.r64(2));
            let Some(t187) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082a692); };
            c.cmp(t187, c.r64(9), 64);
            if c.cc_a() { blk = 244; continue; }
            blk = 211; continue;
        }
        211 => {
            blk = 245; continue;
        }
        212 => {
            let Some(t188) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a6a1); };
            c.w32(12, t188);
            let r189 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r189);
            let Some(t190) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082a6a8); };
            c.w32(15, t190);
            c.w64(9, c.r64(2));
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 213; continue;
        }
        213 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 245; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t191) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a6c4); };
            c.w64(12, t191);
            let Some(t192) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082a6c9); };
            c.w64(15, t192);
            let Some(t193) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700082a6cd); };
            c.logic(t193 & 0x1u64, 8);
            if c.cc_ne() { blk = 217; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t194) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082a6d4); };
            c.w64(0, t194);
            let r195 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r195);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 245; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t196) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082a6e4); };
            c.w64(8, t196);
            c.w64(0, c.r64(8));
            blk = 243; continue;
        }
        217 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 245; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t197) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a6f9); };
            c.w32(8, t197);
            let r198 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r198);
            let Some(t199) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700082a700); };
            c.w32(8, t199);
            let r200 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r200);
            let r201 = c.r32(8) & 0x3fu64;
            c.w32(8, r201);
            c.logic(r201, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 228; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 224; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 222; continue;
        }
        222 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 223; continue;
        }
        223 => {
            blk = 299; continue;
        }
        224 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 225; continue;
        }
        225 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 227; continue;
        }
        227 => {
            blk = 299; continue;
        }
        228 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 233; continue; }
            blk = 229; continue;
        }
        229 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 231; continue;
        }
        231 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 232; continue;
        }
        232 => {
            blk = 299; continue;
        }
        233 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            blk = 299; continue;
        }
        236 => {
            c.w32(8, 0x1u64);
            blk = 240; continue;
        }
        237 => {
            c.w32(8, 0x2u64);
            blk = 240; continue;
        }
        238 => {
            c.w32(8, 0x4u64);
            blk = 240; continue;
        }
        239 => {
            c.w32(8, 0x8u64);
            blk = 240; continue;
        }
        240 => {
            let r202 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r202);
            c.w64(0, c.r64(12));
            c.cqo();
            c.idiv(c.r64(8), 64);
            blk = 243; continue;
        }
        241 => {
            c.w64(10, c.r64(4));
            let r203 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r203);
            let r204 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r204);
            c.logic(r204, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082a7b7); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700082a7c2); };
            c.w64(0, t205);
            c.w64(10, 0x700082a7d0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082a7d0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082a7d4); }
            return c.call(c.r64(0), 0x700082a7da, 0x700082a7d8);
        }
        242 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082a7da); }
            let Some(t206) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082a7e2); };
            c.w64(4, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082a7e7); };
            c.w64(9, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082a7eb); };
            c.w64(11, t208);
            blk = 243; continue;
        }
        243 => {
            c.w64(2, c.r64(9));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082a7f7); };
            c.w64(8, t209);
            blk = 246; continue;
        }
        245 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700082a800); };
            c.w64(8, t210);
            blk = 246; continue;
        }
        246 => {
            let Some(t211) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082a807); };
            c.cmp(t211, c.r32(8), 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a810); };
            c.w64(0, t212);
            blk = 291; continue;
        }
        248 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082a819); };
            c.w64(0, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082a81d); };
            c.w64(3, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082a821); };
            c.w64(6, t215);
            return c.call(0x7000870914, 0x700082a82a, 0x700082a825);
        }
        249 => {
            let Some(t216) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082a82a); };
            c.w32(8, t216);
            let Some(t217) = c.ld8(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082a82e); };
            c.logic(t217 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700082a83a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700082a83e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700082a842); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082a84c); };
            c.w64(11, t218);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700082a850); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 271; continue; }
            blk = 251; continue;
        }
        251 => {
            blk = 254; continue;
        }
        252 => {
            let Some(t219) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082a866); };
            c.cmp(t219, 0x0u64, 32);
            if c.cc_le() { blk = 270; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082a870); };
            c.w64(11, t220);
            blk = 254; continue;
        }
        254 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 271; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t221) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082a87c); };
            c.w32(12, t221);
            let r222 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r222);
            let Some(t223) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082a883); };
            c.w32(12, t223);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 260; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w32(12, c.r16(12));
            c.cmp(c.r32(12), 0xbcu64, 32);
            if c.cc_ne() { blk = 271; continue; }
            blk = 257; continue;
        }
        257 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082a8a2); }
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x1c78)) else { return RegfileExit::fault(0x700082a8a3); };
            c.w64(3, t224);
            c.w32(0, 0x1u64);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082a8af); };
            c.w64(6, t225);
            return c.call(0x700086ed74, 0x700082a8b8, 0x700082a8b3);
        }
        258 => {
            let Some(t226) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082a8b8); };
            c.cmp(t226, c.r32(0), 32);
            if c.cc_e() { blk = 325; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082a8c5); };
            c.w64(0, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082a8c9); };
            c.w64(11, t228);
            blk = 271; continue;
        }
        260 => {
            c.w64(3, c.r64(0));
            let Some(t229) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082a8d5); };
            c.w32(1, t229);
            let r230 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r230);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            c.w64(9, c.r64(11));
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700082a8e4); }
            blk = 265; continue;
        }
        262 => {
            let r231 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r231);
            if c.cc_o() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(11));
            blk = 268; continue;
        }
        264 => {
            c.w64(8, c.r64(11));
            let r232 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r232);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            c.w64(9, c.r64(11));
            blk = 265; continue;
        }
        265 => {
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082a90c); };
            c.w64(8, t233);
            let Some(t234) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082a913); };
            c.w64(11, t234);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082a91a); };
            c.w64(15, t235);
            let Some(t236) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082a921); };
            c.cmp(t236, c.r64(12), 64);
            if c.cc_be() { blk = 343; continue; }
            blk = 266; continue;
        }
        266 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700082a92a); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 267; continue;
        }
        267 => {
            c.w64(10, 0xbff0000000000000u64);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082a944); };
            c.w64(11, t237);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700082a94b); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082a94f); }
            blk = 268; continue;
        }
        268 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082a955); };
            c.w64(0, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082a959); };
            c.w64(6, t239);
            c.w64(7, c.r64(8));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a960); };
            c.w64(2, t240);
            return c.call(0x7000829cd4, 0x700082a969, 0x700082a964);
        }
        269 => {
            blk = 291; continue;
        }
        270 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082a96e); };
            c.w64(11, t241);
            blk = 271; continue;
        }
        271 => {
            let Some(t242) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a972); };
            c.w32(8, t242);
            let Some(t243) = c.ld8(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a976); };
            c.logic(t243 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700082a982); };
            c.w64(12, t244);
            let Some(t245) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082a989); };
            c.w32(12, t245);
            let r246 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r246);
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700082a99d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700082a9a1); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 275; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a9ab); };
            c.w64(2, t247);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700082a9af); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082a9ba); }
            blk = 277; continue;
        }
        274 => {
            let r248 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r248);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 277; continue;
        }
        275 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a9d7); };
            c.w64(2, t249);
            let r250 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r250);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082a9dd); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a9ec); };
            c.w64(2, t251);
            blk = 277; continue;
        }
        277 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 352; continue; }
            blk = 278; continue;
        }
        278 => {
            blk = 281; continue;
        }
        279 => {
            let Some(t252) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082a9fc); };
            c.cmp(c.r32(12), t252, 32);
            if c.cc_le() { blk = 351; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082aa06); };
            c.w64(2, t253);
            blk = 281; continue;
        }
        281 => {
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082aa0a); };
            if !c.push64(t254) { return RegfileExit::fault(0x700082aa0a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082aa0d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082aa0e); }
            c.w32(0, 0x3u64);
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2340)) else { return RegfileExit::fault(0x700082aa14); };
            c.w64(3, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082aa1b); };
            c.w64(6, t256);
            return c.call(0x700086ed74, 0x700082aa24, 0x700082aa1f);
        }
        282 => {
            let Some(t257) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082aa24); };
            c.cmp(t257, 0x0u64, 32);
            if c.cc_ne() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082aa2a); };
            c.w64(8, t258);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700082aa2e); }
            blk = 287; continue;
        }
        284 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082aa36); };
            c.w64(8, t259);
            let r260 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r260);
            if c.cc_o() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 290; continue;
        }
        286 => {
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082aa45); };
            c.w64(8, t261);
            let r262 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r262);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 287; continue;
        }
        287 => {
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082aa55); };
            c.w64(8, t263);
            let Some(t264) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082aa5c); };
            c.w64(9, t264);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082aa63); };
            c.w64(12, t265);
            let Some(t266) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082aa6a); };
            c.cmp(t266, c.r64(11), 64);
            if c.cc_be() { blk = 345; continue; }
            blk = 288; continue;
        }
        288 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082aa74); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 289; continue;
        }
        289 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082aa8d); };
            c.w64(9, t267);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082aa94); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082aa98); }
            c.w64(2, c.r64(8));
            blk = 290; continue;
        }
        290 => {
            c.w64(0, c.r64(2));
            blk = 291; continue;
        }
        291 => {
            let Some(t268) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082aaa4); };
            c.cmp(t268, 0x0u64, 32);
            if c.cc_ne() { blk = 293; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082aaaa); };
            c.w64(11, t269);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700082aaae); }
            blk = 296; continue;
        }
        293 => {
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082aab6); };
            c.w64(8, t270);
            let r271 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r271);
            if c.cc_o() { blk = 295; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 2; continue;
        }
        295 => {
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082aac8); };
            c.w64(11, t272);
            let r273 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r273);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 296; continue;
        }
        296 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082aad8); };
            c.w64(8, t274);
            let Some(t275) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082aadf); };
            c.w64(9, t275);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082aae6); };
            c.w64(12, t276);
            let Some(t277) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082aaed); };
            c.cmp(t277, c.r64(11), 64);
            if c.cc_be() { blk = 347; continue; }
            blk = 297; continue;
        }
        297 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082aaf7); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 298; continue;
        }
        298 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082ab10); };
            c.w64(9, t278);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082ab17); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082ab1b); }
            c.w64(11, c.r64(8));
            blk = 2; continue;
        }
        299 => {
            return RegfileExit::fault(0x700082ab29);
        }
        300 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082ab2a); };
            c.w64(0, t279);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082ab31); }
            return c.ret(0x0, 0x700082ab32);
        }
        301 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x700082ab33); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t280) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ab39); };
            c.w64(3, t280);
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700082ab4b, 0x700082ab46);
        }
        302 => {
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082ab4b); };
            c.w64(1, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ab4f); };
            c.w64(6, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082ab53); };
            c.w64(0, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082ab57); };
            c.w64(7, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082ab5b); };
            c.w64(3, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082ab5f); };
            c.w64(2, t286);
            blk = 1; continue;
        }
        303 => {
            c.cmp(c.r16(3), 0x43u64, 16);
            if c.cc_ne() { blk = 323; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t287) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082ab72); };
            c.w32(3, t287);
            let r288 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r288);
            blk = 32; continue;
        }
        305 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700082ab7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x700082ab82); }
            blk = 309; continue;
        }
        306 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700082ab88); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x700082ab8c); }
            blk = 309; continue;
        }
        307 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700082ab92); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x700082ab96); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 309; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t289) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082aba0); };
            c.w64(12, t289);
            blk = 97; continue;
        }
        309 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082abaa); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082abab); }
            c.w64(6, c.r64(9));
            c.w32(0, 0x2u64);
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700082abb5); };
            c.w64(3, t290);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700082abc7, 0x700082abc2);
        }
        310 => {
            let r291 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r291);
            c.w64(8, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082abcf); };
            c.w64(0, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082abd3); };
            c.w64(3, t293);
            blk = 100; continue;
        }
        311 => {
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082abdc); };
            c.w64(11, t294);
            blk = 323; continue;
        }
        312 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082abe2); };
            c.w64(11, t295);
            blk = 323; continue;
        }
        313 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082abe8); };
            c.w64(11, t296);
            blk = 323; continue;
        }
        314 => {
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082abee); };
            c.w64(11, t297);
            blk = 323; continue;
        }
        315 => {
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082abf4); };
            c.w64(11, t298);
            blk = 323; continue;
        }
        316 => {
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082abfa); };
            c.w64(11, t299);
            blk = 323; continue;
        }
        317 => {
            c.w64(0, c.r64(11));
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ac03); };
            c.w64(6, t300);
            return c.call(0x70008a1af4, 0x700082ac0c, 0x700082ac07);
        }
        318 => {
            let Some(t301) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ac0c); };
            c.w32(8, t301);
            let Some(t302) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700082ac10); };
            c.cmp(t302, c.r32(8), 32);
            if c.cc_ne() { blk = 321; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t303) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ac19); };
            c.logic(t303 & 0x1u64, 8);
            if c.cc_e() { blk = 321; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082ac1f); };
            c.w64(11, t304);
            blk = 245; continue;
        }
        321 => {
            c.w64(3, c.r64(0));
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082ac2b); };
            c.w64(0, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ac2f); };
            c.w64(6, t306);
            return c.call(0x70008bced4, 0x700082ac38, 0x700082ac33);
        }
        322 => {
            c.w64(8, c.r64(0));
            blk = 246; continue;
        }
        323 => {
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082ac40); };
            if !c.push64(t307) { return RegfileExit::fault(0x700082ac40); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700082ac43); }
            let Some(t308) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700082ac45); };
            c.w64(3, t308);
            c.w32(0, 0x2u64);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ac51); };
            c.w64(6, t309);
            return c.call(0x700086ed74, 0x700082ac5a, 0x700082ac55);
        }
        324 => {
            c.w64(8, c.r64(0));
            blk = 246; continue;
        }
        325 => {
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700082ac62); };
            c.w64(3, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082ac69); };
            c.w64(0, t311);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ac6d); };
            c.w64(6, t312);
            return c.call(0x7000870914, 0x700082ac76, 0x700082ac71);
        }
        326 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 329; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ac7a); };
            c.w64(6, t313);
            return c.call(0x70008a17f4, 0x700082ac83, 0x700082ac7e);
        }
        328 => {
            blk = 331; continue;
        }
        329 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 331; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 331; continue;
        }
        331 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700082ac8c); }
            let Some(t314) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082ac90); };
            c.cmp(t314, 0x0u64, 32);
            if c.cc_ne() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082ac96); };
            c.w64(8, t315);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700082ac9a); }
            blk = 336; continue;
        }
        333 => {
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082aca2); };
            c.w64(8, t316);
            let r317 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r317);
            if c.cc_o() { blk = 335; continue; }
            blk = 334; continue;
        }
        334 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            blk = 341; continue;
        }
        335 => {
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082acb4); };
            c.w64(8, t318);
            let r319 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r319);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082acc4); };
            c.w64(8, t320);
            blk = 336; continue;
        }
        336 => {
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082acc8); };
            c.w64(9, t321);
            let Some(t322) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082accf); };
            c.w64(11, t322);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700082acd6); }
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082acdb); };
            c.w64(15, t323);
            let Some(t324) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082ace2); };
            c.cmp(t324, c.r64(12), 64);
            if c.cc_a() { blk = 339; continue; }
            blk = 337; continue;
        }
        337 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700082acf1, 0x700082acec);
        }
        338 => {
            c.w64(9, c.r64(0));
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082acf4); };
            c.w64(8, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082acf8); };
            c.w64(0, t326);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700082acfc); }
            blk = 340; continue;
        }
        339 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700082ad03); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 340; continue;
        }
        340 => {
            let Some(t327) = c.ld64(0x700082a933u64) else { return RegfileExit::fault(0x700082ad0a); };
            c.w64(10, t327);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082ad1a); };
            c.w64(11, t328);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700082ad21); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082ad25); }
            blk = 341; continue;
        }
        341 => {
            c.w64(1, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082ad2e); };
            c.w64(0, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082ad32); };
            c.w64(3, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082ad36); };
            c.w64(2, t331);
            c.w64(7, c.r64(9));
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ad3d); };
            c.w64(6, t332);
            return c.call(0x7000829cd4, 0x700082ad46, 0x700082ad41);
        }
        342 => {
            blk = 291; continue;
        }
        343 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700082ad50); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700082ad54); }
            return c.call(0x70007febf4, 0x700082ad5e, 0x700082ad59);
        }
        344 => {
            c.w64(8, c.r64(0));
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082ad61); };
            c.w64(3, t333);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082ad65); };
            c.w64(1, t334);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700082ad69); }
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082ad6e); };
            c.w64(9, t335);
            blk = 267; continue;
        }
        345 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700082ad7c); }
            return c.call(0x70007febf4, 0x700082ad86, 0x700082ad81);
        }
        346 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700082ad89); }
            blk = 289; continue;
        }
        347 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700082ad98); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(0)) { return RegfileExit::fault(0x700082ad9d); }
            return c.call(0x70007febf4, 0x700082ada6, 0x700082ada1);
        }
        348 => {
            c.w64(8, c.r64(0));
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082ada9); };
            c.w64(0, t336);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700082adad); }
            blk = 298; continue;
        }
        349 => {
            return RegfileExit::fault(0x700082adb7);
        }
        350 => {
            blk = 352; continue;
        }
        351 => {
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082adba); };
            c.w64(2, t337);
            blk = 352; continue;
        }
        352 => {
            c.w32(8, 0x256u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082adc4); }
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082adc6); };
            if !c.push64(t338) { return RegfileExit::fault(0x700082adc6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082adc9); }
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082adca); };
            c.w64(3, t339);
            c.w32(0, 0x3u64);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082add6); };
            c.w64(6, t340);
            return c.call(0x700086ed74, 0x700082addf, 0x700082adda);
        }
        353 => {
            return RegfileExit::fault(0x700082addf);
        }
        354 => {
            return RegfileExit::jump(0x700082ade4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082adf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082adf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082adf8); }
            let r0 = c.sub(c.r64(4), 0x70u64, 64);
            c.w64(4, r0);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700082ae09); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ae0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x700082ae11); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(3)) { return RegfileExit::fault(0x700082ae15); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700082ae19); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700082ae1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(8)) { return RegfileExit::fault(0x700082ae21); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082ae2a); };
            c.w64(3, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700082ae2e); }
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ae32); };
            c.w64(1, t2);
            blk = 2; continue;
        }
        2 => {
            c.w32(0, c.r32(11));
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700082ae46); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700082ae4a); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700082ae53); }
            let Some(t3) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082ae59); };
            c.cmp(t3, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700082ae5f); }
            blk = 8; continue;
        }
        5 => {
            c.w64(6, c.r64(1));
            let r4 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r4);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            blk = 8; continue;
        }
        6 => {
            let Some(t5) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700082ae75); };
            c.cmp(t5, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(11));
            let r6 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r6);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700082ae80); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(6) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 300; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 11; continue;
        }
        10 => {
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_le() { blk = 300; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700082aea2); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 322; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082aeaf); };
            c.w32(6, t7);
            let r8 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r8);
            let Some(t9) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082aeb5); };
            c.w32(8, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700082aeba); }
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 322; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 316; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082aedf); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082aee6); };
            c.w32(3, t12);
            c.cmp(c.r16(3), 0x42u64, 16);
            if c.cc_ne() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700082aef0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(1) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(7, 0x1fffffffffffffu64);
            let r13 = c.add(c.r64(7), c.r64(1), 64);
            c.w64(7, r13);
            c.w64(8, 0x3ffffffffffffeu64);
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_be() { blk = 108; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082af2c); };
            c.w64(8, t14);
            blk = 20; continue;
        }
        20 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082af30); };
            c.w64(7, t15);
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_ae() { blk = 302; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082af44); };
            c.w32(1, t16);
            c.logic(c.r32(1) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(1) & 0x2u64, 8);
            if c.cc_e() { blk = 322; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(3), 0x25u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(3), 0x2du64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(3) & 0x20u64, 8);
            if c.cc_ne() { blk = 322; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(3, c.r64(11));
            blk = 31; continue;
        }
        28 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082af73); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            blk = 31; continue;
        }
        29 => {
            if let Some(sr) = c.shr(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r19 = c.r64(1) & 0xffffffu64;
            c.w64(1, r19);
            c.logic(r19, 64);
            c.w64(12, c.r64(1));
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082af8a); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082af8e); };
            c.w64(3, t21);
            blk = 110; continue;
        }
        30 => {
            c.w64(3, c.r64(11));
            blk = 31; continue;
        }
        31 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082af9a); };
            c.w64(1, t22);
            blk = 33; continue;
        }
        32 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082afa0); };
            c.w32(6, t23);
            let r24 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r24);
            let Some(t25) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082afa6); };
            c.w32(8, t25);
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 62; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t26) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082afb8); };
            c.w32(7, t26);
            c.logic(c.r32(7) & 0x200000u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t27) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082afc7); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            c.logic(c.r32(7) & 0xffc00u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 36; continue;
        }
        36 => {
            let r29 = c.shr(c.r32(7), 0xau64, 32).unwrap_or(c.r32(7));
            c.w32(7, r29);
            let r30 = c.r32(7) & 0x3ffu64;
            c.w32(7, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_be() { blk = 51; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t31) = c.ld16(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x700082afeb); };
            c.w32(0, (t31 as u16 as i16 as i64 as u64));
            let Some(t32) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082afef); };
            c.w32(2, t32);
            let r33 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r33);
            let r34 = c.sub(c.r32(0), 0x1u64, 32);
            c.w32(0, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700082aff8); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 39; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700082b001); }
            blk = 39; continue;
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082b005); };
            c.w64(15, t35);
            let r36 = c.sub(c.r32(15), c.r32(11), 32);
            c.w32(15, r36);
            let r37 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r37);
            let r38 = c.add(c.r32(15), c.r32(11), 32);
            c.w32(15, r38);
            c.w32(12, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082b019); };
            c.w32(12, t39);
            let r40 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r41);
            let r42 = c.r32(12) & 0x3ffu64;
            c.w32(12, r42);
            c.logic(r42, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b033); };
            c.w32(12, t43);
            let r44 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r44);
            let Some(t45) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b03b); };
            c.w32(12, t45);
            let r46 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r46);
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_be() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(11, c.r64(15).wrapping_add(0x1));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082b04d); };
            c.w64(15, t47);
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(6)) { return RegfileExit::fault(0x700082b056); }
            blk = 44; continue;
        }
        43 => {
            let r48 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r48);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_l() { blk = 50; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(12, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082b06c); };
            c.w32(12, t49);
            let r50 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            let r51 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r51);
            let r52 = c.r32(12) & 0x3ffu64;
            c.w32(12, r52);
            c.logic(r52, 32);
            c.w32(15, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t53) = c.ld32(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b086); };
            c.w32(15, t53);
            let r54 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r54);
            let Some(t55) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b08e); };
            c.w32(6, t55);
            let r56 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r56);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_a() { blk = 243; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082b0a7); };
            c.w64(6, t57);
            blk = 73; continue;
        }
        49 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082b0b0); };
            c.w64(6, t58);
            blk = 73; continue;
        }
        50 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082b0b9); };
            c.w64(6, t59);
            blk = 73; continue;
        }
        51 => {
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            let r60 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r60);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 73; continue; }
            blk = 52; continue;
        }
        52 => {
            let r61 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r61);
            let Some(t62) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b0d7); };
            c.cmp(t62, c.r32(3), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 52; continue; }
            blk = 54; continue;
        }
        54 => {
            blk = 73; continue;
        }
        55 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b0ed); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700082b0fa); };
            c.w64(11, t65);
            blk = 57; continue;
        }
        57 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b101); };
            c.w32(12, t66);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r67 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r67);
            let Some(t68) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b10f); };
            c.w32(15, t68);
            let r69 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r69);
            let r70 = c.r64(15) & c.r64(12);
            c.w64(15, r70);
            c.logic(r70, 64);
            c.w64(0, c.r64(9));
            blk = 59; continue;
        }
        58 => {
            let r71 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r71);
            let r72 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r72);
            c.w64(2, c.r64(12));
            let r73 = c.r64(2) & c.r64(15);
            c.w64(2, r73);
            c.logic(r73, 64);
            c.w64(15, c.r64(2));
            blk = 59; continue;
        }
        59 => {
            c.w64(2, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t74) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b134); };
            c.w32(2, t74);
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082b139); };
            c.cmp(t75, c.r32(2), 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 58; continue;
        }
        62 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 310; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t76) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082b15d); };
            c.logic(t76 & 0x24u64, 8);
            if c.cc_ne() { blk = 315; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t77) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b167); };
            c.w32(11, t77);
            let r78 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r78);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700082b174); };
            c.w64(11, t79);
            blk = 66; continue;
        }
        66 => {
            let Some(t80) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b17b); };
            c.w32(12, t80);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r81 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r81);
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b189); };
            c.w32(15, t82);
            let r83 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            let r84 = c.r64(15) & c.r64(12);
            c.w64(15, r84);
            c.logic(r84, 64);
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        67 => {
            let r85 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r85);
            let r86 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r86);
            c.w64(2, c.r64(12));
            let r87 = c.r64(2) & c.r64(15);
            c.w64(2, r87);
            c.logic(r87, 64);
            c.w64(15, c.r64(2));
            blk = 68; continue;
        }
        68 => {
            c.w64(2, c.r64(15).wrapping_add(0x5));
            let Some(t88) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b1ad); };
            c.w32(1, t88);
            let r89 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r89);
            let Some(t90) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082b1b5); };
            c.cmp(t90, c.r32(1), 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082b1be); };
            c.cmp(t91, c.r32(1), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t92) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b1c7); };
            c.cmp(t92, c.r32(3), 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t93) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b1cc); };
            c.w32(11, t93);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 348; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t94) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b1df); };
            c.w32(11, t94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082b1e3); };
            c.cmp(t95, c.r32(11), 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t96) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b1fd); };
            c.w32(8, t96);
            let r97 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r97);
            let Some(t98) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b204); };
            c.w32(8, t98);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 104; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t99) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b214); };
            c.w32(11, t99);
            let r100 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r100);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 104; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(12, c.r64(3));
            c.w64(8, c.r64(9));
            blk = 79; continue;
        }
        79 => {
            c.w64(15, c.r64(11));
            let r101 = c.r32(15) & 0x7u64;
            c.w32(15, r101);
            c.logic(r101, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 84; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 304; continue;
        }
        84 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            blk = 304; continue;
        }
        87 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b27f); };
            c.w32(11, t102);
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700082b284); };
            c.cmp(t103, c.r32(11), 32);
            if c.cc_ne() { blk = 305; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b291); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b299); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b2a1); };
            c.w32(11, t108);
            blk = 79; continue;
        }
        89 => {
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b2a8); };
            c.w32(11, t109);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r110 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r110);
            let Some(t111) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b2b6); };
            c.w32(12, t111);
            let r112 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r112);
            let Some(t113) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b2be); };
            c.w32(11, t113);
            let r114 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r114);
            let Some(t115) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b2c6); };
            c.w32(11, t115);
            blk = 79; continue;
        }
        90 => {
            let Some(t116) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b2d0); };
            c.w32(12, t116);
            let r117 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r117);
            let Some(t118) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b2d8); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b2e0); };
            c.w32(11, t120);
            blk = 79; continue;
        }
        91 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 94; continue;
        }
        92 => {
            c.w32(15, 0x1u64);
            blk = 94; continue;
        }
        93 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(9));
            c.w64(12, c.r64(3));
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 95; continue;
        }
        95 => {
            let r121 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r121);
            blk = 96; continue;
        }
        96 => {
            let r122 = c.r32(11) & 0x8u64;
            c.w32(11, r122);
            c.logic(r122, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t123) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x700082b318); };
            c.w32(8, t123);
            blk = 99; continue;
        }
        98 => {
            let Some(t124) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700082b31f); };
            c.w32(8, t124);
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 312; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 311; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082b352); };
            c.w64(11, t125);
            let Some(t126) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b356); };
            c.w32(8, t126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700082b35a); };
            c.cmp(t127, c.r32(8), 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t128) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b363); };
            c.logic(t128 & 0x1u64, 8);
            if c.cc_ne() { blk = 244; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t129) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b36e); };
            c.w32(1, t129);
            let r130 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r130);
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700082b374); };
            c.cmp(t131, c.r32(1), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 107; continue;
        }
        107 => {
            blk = 32; continue;
        }
        108 => {
            c.w64(12, c.r64(1));
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082b389); };
            c.w64(1, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082b38d); };
            c.w64(3, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082b391); };
            c.w64(8, t134);
            blk = 110; continue;
        }
        109 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 110; continue;
        }
        110 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700082b39d); }
            c.w64(15, c.r64(8));
            c.w64(8, c.r64(3));
            blk = 112; continue;
        }
        111 => {
            let Some(t135) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b3a9); };
            c.w32(6, t135);
            let r136 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r136);
            let Some(t137) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b3b0); };
            c.w32(15, t137);
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(15), 0x410u64, 32);
            if c.cc_le() { blk = 322; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t138) = c.ld8(c.r64(6).wrapping_add(0xa)) else { return RegfileExit::fault(0x700082b3c2); };
            c.w32(15, t138);
            let r139 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r139);
            let r140 = c.r32(15) & 0x3fu64;
            c.w32(15, r140);
            c.logic(r140, 32);
            c.cmp(c.r32(15), 0x15u64, 32);
            if c.cc_l() { blk = 148; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 132; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_l() { blk = 124; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_l() { blk = 121; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(15), 0x28u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 322; continue;
        }
        121 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 123; continue;
        }
        123 => {
            blk = 322; continue;
        }
        124 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_l() { blk = 129; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 322; continue;
        }
        129 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 322; continue;
        }
        132 => {
            c.cmp(c.r32(15), 0x19u64, 32);
            if c.cc_l() { blk = 141; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(15), 0x1bu64, 32);
            if c.cc_l() { blk = 138; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(15), 0x1bu64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 137; continue;
        }
        137 => {
            blk = 322; continue;
        }
        138 => {
            c.cmp(c.r32(15), 0x19u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(15), 0x1au64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 140; continue;
        }
        140 => {
            blk = 322; continue;
        }
        141 => {
            c.cmp(c.r32(15), 0x17u64, 32);
            if c.cc_l() { blk = 145; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r32(15), 0x17u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(15), 0x18u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 322; continue;
        }
        145 => {
            c.cmp(c.r32(15), 0x15u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(15), 0x16u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 147; continue;
        }
        147 => {
            blk = 322; continue;
        }
        148 => {
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_l() { blk = 166; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(15), 0x10u64, 32);
            if c.cc_l() { blk = 158; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(15), 0x12u64, 32);
            if c.cc_l() { blk = 155; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(15), 0x12u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(15), 0x13u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(15), 0x14u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 154; continue;
        }
        154 => {
            blk = 322; continue;
        }
        155 => {
            c.cmp(c.r32(15), 0x10u64, 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(15), 0x11u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 157; continue;
        }
        157 => {
            blk = 322; continue;
        }
        158 => {
            c.cmp(c.r32(15), 0xbu64, 32);
            if c.cc_l() { blk = 163; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(15), 0xbu64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(15), 0xcu64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(15), 0xfu64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 162; continue;
        }
        162 => {
            blk = 322; continue;
        }
        163 => {
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(15), 0xau64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 322; continue;
        }
        166 => {
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_l() { blk = 175; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(15), 0x6u64, 32);
            if c.cc_l() { blk = 172; continue; }
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(15), 0x6u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(15), 0x7u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 171; continue;
        }
        171 => {
            blk = 322; continue;
        }
        172 => {
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 174; continue;
        }
        174 => {
            blk = 322; continue;
        }
        175 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 179; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 322; continue;
        }
        179 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 180; continue;
        }
        180 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 181; continue;
        }
        181 => {
            blk = 322; continue;
        }
        182 => {
            blk = 187; continue;
        }
        183 => {
            let Some(t141) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b5eb); };
            c.w32(8, t141);
            let r142 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r142);
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b5f2); };
            c.w32(15, t143);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_ae() { blk = 188; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(15, 0xfff7fffffff7ffffu64);
            let Some(t144) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b60b); };
            c.cmp(t144, c.r64(15), 64);
            if c.cc_e() { blk = 206; continue; }
            blk = 185; continue;
        }
        185 => {
            blk = 243; continue;
        }
        186 => {
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b61b); };
            c.w32(15, t145);
            let r146 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r146);
            let Some(t147) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b622); };
            c.w32(15, t147);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 243; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t148) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b62f); };
            c.w32(8, t148);
            let r149 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r149);
            let Some(t150) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082b636); };
            c.w32(15, t150);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 190; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 322; continue; }
            blk = 189; continue;
        }
        189 => {
            blk = 206; continue;
        }
        190 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b659); };
            c.w32(8, t151);
            let Some(t152) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082b65e); };
            c.cmp(t152, c.r32(8), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 191; continue;
        }
        191 => {
            blk = 243; continue;
        }
        192 => {
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b670); };
            c.w32(15, t153);
            let r154 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r154);
            let Some(t155) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b677); };
            c.w32(15, t155);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 243; continue; }
            blk = 193; continue;
        }
        193 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(6)) { return RegfileExit::fault(0x700082b684); }
            c.w32(15, 0xfffffffeu64);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_b() { blk = 322; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b697); };
            c.w32(8, t156);
            let r157 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r157);
            let Some(t158) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b69e); };
            c.w32(15, t158);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r159 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r159);
            c.w32(7, c.r32(12));
            c.w64(10, c.r64(4));
            let r160 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r160);
            let r161 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r161);
            c.logic(r161, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082b6ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700082b6bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700082b6c3); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700082b6cd); };
            c.w64(0, t162);
            c.w64(10, 0x700082b6dbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082b6db); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082b6df); }
            return c.call(c.r64(0), 0x700082b6e5, 0x700082b6e3);
        }
        195 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082b6e5); }
            let Some(t163) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082b6ed); };
            c.w64(4, t163);
            c.w32(8, c.r32(0));
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082b6f5); };
            let r165 = c.r64(8) & t164;
            c.w64(8, r165);
            c.logic(r165, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082b6fc); };
            c.w64(11, t166);
            blk = 197; continue;
        }
        196 => {
            let r167 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r167);
            let r168 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082b709); };
            c.w64(12, t169);
            let r170 = c.r64(12) & c.r64(8);
            c.w64(12, r170);
            c.logic(r170, 64);
            c.w64(8, c.r64(12));
            blk = 197; continue;
        }
        197 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t171) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b718); };
            c.w32(12, t171);
            let r172 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r172);
            let Some(t173) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082b720); };
            c.cmp(t173, c.r32(12), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 198; continue;
        }
        198 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 203; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t174) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082b72f); };
            c.cmp(t174, c.r32(12), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 200; continue;
        }
        200 => {
            c.xorpd_rr(0, 0);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082b73c); };
            c.cvtsi2sd(0, (t175 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082b742); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 196; continue; }
            blk = 201; continue;
        }
        201 => {
            if c.cc_e() { blk = 243; continue; }
            blk = 202; continue;
        }
        202 => {
            blk = 196; continue;
        }
        203 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082b75d); };
            c.cmp(c.r64(12), t176, 64);
            if c.cc_e() { blk = 243; continue; }
            blk = 204; continue;
        }
        204 => {
            blk = 196; continue;
        }
        205 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082b769); };
            c.w64(6, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082b76d); };
            c.w64(11, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082b771); };
            c.w64(12, t179);
            blk = 206; continue;
        }
        206 => {
            let Some(t180) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b775); };
            c.w32(8, t180);
            let r181 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082b77c); };
            c.w64(1, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082b780); };
            c.w64(3, t183);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700082b787); };
            c.cmp(t184, c.r32(8), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 207; continue;
        }
        207 => {
            blk = 111; continue;
        }
        208 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b799); };
            c.w32(9, t185);
            let r186 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r186);
            let Some(t187) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082b7a0); };
            c.logic(t187 & 0x4u64, 8);
            if c.cc_ne() { blk = 244; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w64(1, c.r64(12));
            let Some(t188) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082b7ae); };
            c.cmp(t188, c.r64(1), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 210; continue;
        }
        210 => {
            blk = 244; continue;
        }
        211 => {
            let Some(t189) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b7bd); };
            c.w32(9, t189);
            let r190 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r190);
            let Some(t191) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082b7c4); };
            c.w32(15, t191);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 240; continue; }
            blk = 212; continue;
        }
        212 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 244; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t192) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082b7dc); };
            c.w64(9, t192);
            let Some(t193) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082b7e0); };
            c.w64(15, t193);
            let Some(t194) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700082b7e4); };
            c.logic(t194 & 0x1u64, 8);
            if c.cc_ne() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t195) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082b7eb); };
            c.w64(0, t195);
            let r196 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r196);
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_a() { blk = 244; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t197) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082b7fb); };
            c.w64(8, t197);
            c.w64(0, c.r64(8));
            blk = 242; continue;
        }
        216 => {
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_a() { blk = 244; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t198) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b810); };
            c.w32(8, t198);
            let r199 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r199);
            let Some(t200) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700082b817); };
            c.w32(8, t200);
            let r201 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r201);
            let r202 = c.r32(8) & 0x3fu64;
            c.w32(8, r202);
            c.logic(r202, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 227; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 223; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 222; continue;
        }
        222 => {
            blk = 299; continue;
        }
        223 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 225; continue;
        }
        225 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 226; continue;
        }
        226 => {
            blk = 299; continue;
        }
        227 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 232; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 229; continue;
        }
        229 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 231; continue;
        }
        231 => {
            blk = 299; continue;
        }
        232 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 234; continue;
        }
        234 => {
            blk = 299; continue;
        }
        235 => {
            c.w32(8, 0x1u64);
            blk = 239; continue;
        }
        236 => {
            c.w32(8, 0x2u64);
            blk = 239; continue;
        }
        237 => {
            c.w32(8, 0x4u64);
            blk = 239; continue;
        }
        238 => {
            c.w32(8, 0x8u64);
            blk = 239; continue;
        }
        239 => {
            let r203 = c.sub(c.r64(9), c.r64(15), 64);
            c.w64(9, r203);
            c.w64(0, c.r64(9));
            c.cqo();
            c.idiv(c.r64(8), 64);
            blk = 242; continue;
        }
        240 => {
            c.w64(10, c.r64(4));
            let r204 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r204);
            let r205 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r205);
            c.logic(r205, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082b8ce); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700082b8d9); };
            c.w64(0, t206);
            c.w64(10, 0x700082b8e7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082b8e7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082b8eb); }
            return c.call(c.r64(0), 0x700082b8f1, 0x700082b8ef);
        }
        241 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082b8f1); }
            let Some(t207) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082b8f9); };
            c.w64(4, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082b8fe); };
            c.w64(12, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082b902); };
            c.w64(11, t209);
            blk = 242; continue;
        }
        242 => {
            c.w64(1, c.r64(12));
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_ae() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082b90e); };
            c.w64(8, t210);
            blk = 245; continue;
        }
        244 => {
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700082b917); };
            c.w64(8, t211);
            blk = 245; continue;
        }
        245 => {
            let Some(t212) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082b91e); };
            c.cmp(t212, c.r32(8), 32);
            if c.cc_e() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082b927); };
            c.w64(0, t213);
            blk = 291; continue;
        }
        247 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082b930); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082b934); };
            c.w64(3, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082b938); };
            c.w64(6, t216);
            return c.call(0x7000870914, 0x700082b941, 0x700082b93c);
        }
        248 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082b941); };
            if !c.push64(t217) { return RegfileExit::fault(0x700082b941); }
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082b944); };
            if !c.push64(t218) { return RegfileExit::fault(0x700082b944); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082b947); }
            let Some(t219) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082b948); };
            if !c.push64(t219) { return RegfileExit::fault(0x700082b948); }
            c.w32(0, 0x3u64);
            let Some(t220) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082b950); };
            c.w64(7, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082b954); };
            c.w64(6, t221);
            return c.call(0x70007f2274, 0x700082b95d, 0x700082b958);
        }
        249 => {
            let Some(t222) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082b95d); };
            c.w32(8, t222);
            let Some(t223) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082b961); };
            c.logic(t223 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700082b96d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700082b971); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700082b975); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082b97f); };
            c.w64(11, t224);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700082b983); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 271; continue; }
            blk = 251; continue;
        }
        251 => {
            blk = 254; continue;
        }
        252 => {
            let Some(t225) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082b999); };
            c.cmp(t225, 0x0u64, 32);
            if c.cc_le() { blk = 270; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082b9a3); };
            c.w64(11, t226);
            blk = 254; continue;
        }
        254 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 271; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t227) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082b9af); };
            c.w32(12, t227);
            let r228 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r228);
            let Some(t229) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082b9b6); };
            c.w32(12, t229);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 260; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w32(12, c.r16(12));
            c.cmp(c.r32(12), 0xbcu64, 32);
            if c.cc_ne() { blk = 271; continue; }
            blk = 257; continue;
        }
        257 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082b9d5); }
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x1c78)) else { return RegfileExit::fault(0x700082b9d6); };
            c.w64(3, t230);
            c.w32(0, 0x1u64);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082b9e2); };
            c.w64(6, t231);
            return c.call(0x700086ed74, 0x700082b9eb, 0x700082b9e6);
        }
        258 => {
            let Some(t232) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082b9eb); };
            c.cmp(t232, c.r32(0), 32);
            if c.cc_e() { blk = 324; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082b9f8); };
            c.w64(0, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082b9fc); };
            c.w64(11, t234);
            blk = 271; continue;
        }
        260 => {
            c.w64(3, c.r64(0));
            let Some(t235) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082ba08); };
            c.w32(1, t235);
            let r236 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r236);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            c.w64(9, c.r64(11));
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700082ba17); }
            blk = 265; continue;
        }
        262 => {
            let r237 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r237);
            if c.cc_o() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(11));
            blk = 268; continue;
        }
        264 => {
            c.w64(8, c.r64(11));
            let r238 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r238);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            c.w64(9, c.r64(11));
            blk = 265; continue;
        }
        265 => {
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082ba3f); };
            c.w64(8, t239);
            let Some(t240) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082ba46); };
            c.w64(11, t240);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082ba4d); };
            c.w64(15, t241);
            let Some(t242) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082ba54); };
            c.cmp(t242, c.r64(12), 64);
            if c.cc_be() { blk = 342; continue; }
            blk = 266; continue;
        }
        266 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700082ba5d); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 267; continue;
        }
        267 => {
            c.w64(10, 0xbff0000000000000u64);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082ba77); };
            c.w64(11, t243);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700082ba7e); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082ba82); }
            blk = 268; continue;
        }
        268 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082ba88); };
            c.w64(0, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ba8c); };
            c.w64(6, t245);
            c.w64(7, c.r64(8));
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082ba93); };
            c.w64(2, t246);
            return c.call(0x7000829cd4, 0x700082ba9c, 0x700082ba97);
        }
        269 => {
            blk = 291; continue;
        }
        270 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082baa1); };
            c.w64(11, t247);
            blk = 271; continue;
        }
        271 => {
            let Some(t248) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082baa5); };
            c.w32(8, t248);
            let Some(t249) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082baa9); };
            c.logic(t249 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700082bab5); };
            c.w64(12, t250);
            let Some(t251) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082babc); };
            c.w32(12, t251);
            let r252 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r252);
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700082bad0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082bad4); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 275; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bade); };
            c.w64(2, t253);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700082bae2); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082baed); }
            blk = 277; continue;
        }
        274 => {
            let r254 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r254);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 277; continue;
        }
        275 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bb0a); };
            c.w64(2, t255);
            let r256 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r256);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082bb10); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bb1f); };
            c.w64(2, t257);
            blk = 277; continue;
        }
        277 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 351; continue; }
            blk = 278; continue;
        }
        278 => {
            blk = 281; continue;
        }
        279 => {
            let Some(t258) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bb2f); };
            c.cmp(c.r32(12), t258, 32);
            if c.cc_le() { blk = 350; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bb39); };
            c.w64(2, t259);
            blk = 281; continue;
        }
        281 => {
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082bb3d); };
            if !c.push64(t260) { return RegfileExit::fault(0x700082bb3d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082bb40); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082bb41); }
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2340)) else { return RegfileExit::fault(0x700082bb42); };
            c.w64(3, t261);
            c.w32(0, 0x3u64);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082bb4e); };
            c.w64(6, t262);
            return c.call(0x700086ed74, 0x700082bb57, 0x700082bb52);
        }
        282 => {
            let Some(t263) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082bb57); };
            c.cmp(t263, 0x0u64, 32);
            if c.cc_ne() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bb5d); };
            c.w64(0, t264);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700082bb61); }
            blk = 287; continue;
        }
        284 => {
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082bb68); };
            c.w64(8, t265);
            let r266 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r266);
            if c.cc_o() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 290; continue;
        }
        286 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bb77); };
            c.w64(0, t267);
            let r268 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r268);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 287; continue;
        }
        287 => {
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082bb85); };
            c.w64(8, t269);
            let Some(t270) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082bb8c); };
            c.w64(9, t270);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082bb93); };
            c.w64(12, t271);
            let Some(t272) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082bb9a); };
            c.cmp(t272, c.r64(11), 64);
            if c.cc_be() { blk = 344; continue; }
            blk = 288; continue;
        }
        288 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082bba4); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 289; continue;
        }
        289 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082bbbd); };
            c.w64(9, t273);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082bbc4); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082bbc8); }
            c.w64(2, c.r64(8));
            blk = 290; continue;
        }
        290 => {
            c.w64(0, c.r64(2));
            blk = 291; continue;
        }
        291 => {
            let Some(t274) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082bbd4); };
            c.cmp(t274, 0x0u64, 32);
            if c.cc_ne() { blk = 293; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bbda); };
            c.w64(11, t275);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700082bbde); }
            blk = 296; continue;
        }
        293 => {
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082bbe6); };
            c.w64(8, t276);
            let r277 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r277);
            if c.cc_o() { blk = 295; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        295 => {
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bbf8); };
            c.w64(11, t278);
            let r279 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r279);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 296; continue;
        }
        296 => {
            let Some(t280) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082bc08); };
            c.w64(8, t280);
            let Some(t281) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082bc0f); };
            c.w64(9, t281);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082bc16); };
            c.w64(12, t282);
            let Some(t283) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082bc1d); };
            c.cmp(t283, c.r64(11), 64);
            if c.cc_be() { blk = 346; continue; }
            blk = 297; continue;
        }
        297 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082bc27); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 298; continue;
        }
        298 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082bc40); };
            c.w64(9, t284);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082bc47); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082bc4b); }
            c.w64(11, c.r64(8));
            blk = 1; continue;
        }
        299 => {
            return RegfileExit::fault(0x700082bc59);
        }
        300 => {
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bc5a); };
            c.w64(0, t285);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082bc61); }
            return c.ret(0x10, 0x700082bc62);
        }
        301 => {
            blk = 322; continue;
        }
        302 => {
            c.cmp(c.r16(3), 0x43u64, 16);
            if c.cc_ne() { blk = 322; continue; }
            blk = 303; continue;
        }
        303 => {
            let Some(t286) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082bc74); };
            c.w32(3, t286);
            let r287 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r287);
            blk = 31; continue;
        }
        304 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700082bc80); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(6)) { return RegfileExit::fault(0x700082bc84); }
            blk = 308; continue;
        }
        305 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700082bc8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(6)) { return RegfileExit::fault(0x700082bc8e); }
            blk = 308; continue;
        }
        306 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700082bc94); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(6)) { return RegfileExit::fault(0x700082bc98); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 308; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t288) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082bca2); };
            c.w64(12, t288);
            blk = 96; continue;
        }
        308 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082bcac); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082bcad); }
            c.w64(6, c.r64(9));
            c.w32(0, 0x2u64);
            let Some(t289) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700082bcb7); };
            c.w64(3, t289);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700082bcc6, 0x700082bcc1);
        }
        309 => {
            let r290 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r290);
            c.w64(8, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082bcce); };
            c.w64(6, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082bcd2); };
            c.w64(3, t292);
            blk = 99; continue;
        }
        310 => {
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bcdb); };
            c.w64(11, t293);
            blk = 322; continue;
        }
        311 => {
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bce1); };
            c.w64(11, t294);
            blk = 322; continue;
        }
        312 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bce7); };
            c.w64(11, t295);
            blk = 322; continue;
        }
        313 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bced); };
            c.w64(11, t296);
            blk = 322; continue;
        }
        314 => {
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bcf3); };
            c.w64(11, t297);
            blk = 322; continue;
        }
        315 => {
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bcf9); };
            c.w64(11, t298);
            blk = 322; continue;
        }
        316 => {
            c.w64(0, c.r64(11));
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082bd02); };
            c.w64(6, t299);
            return c.call(0x70008a1af4, 0x700082bd0b, 0x700082bd06);
        }
        317 => {
            let Some(t300) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082bd0b); };
            c.w32(8, t300);
            let Some(t301) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700082bd0f); };
            c.cmp(t301, c.r32(8), 32);
            if c.cc_ne() { blk = 320; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t302) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082bd18); };
            c.logic(t302 & 0x1u64, 8);
            if c.cc_e() { blk = 320; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bd1e); };
            c.w64(11, t303);
            blk = 244; continue;
        }
        320 => {
            c.w64(3, c.r64(0));
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082bd2a); };
            c.w64(0, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082bd2e); };
            c.w64(6, t305);
            return c.call(0x70008bced4, 0x700082bd37, 0x700082bd32);
        }
        321 => {
            c.w64(8, c.r64(0));
            blk = 245; continue;
        }
        322 => {
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700082bd3f); };
            if !c.push64(t306) { return RegfileExit::fault(0x700082bd3f); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700082bd42); }
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700082bd44); };
            c.w64(3, t307);
            c.w32(0, 0x2u64);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082bd50); };
            c.w64(6, t308);
            return c.call(0x700086ed74, 0x700082bd59, 0x700082bd54);
        }
        323 => {
            c.w64(8, c.r64(0));
            blk = 245; continue;
        }
        324 => {
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700082bd61); };
            c.w64(3, t309);
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082bd68); };
            c.w64(0, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082bd6c); };
            c.w64(6, t311);
            return c.call(0x7000870914, 0x700082bd75, 0x700082bd70);
        }
        325 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 328; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082bd79); };
            c.w64(6, t312);
            return c.call(0x70008a17f4, 0x700082bd82, 0x700082bd7d);
        }
        327 => {
            blk = 330; continue;
        }
        328 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 330; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 330; continue;
        }
        330 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700082bd8b); }
            let Some(t313) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700082bd8f); };
            c.cmp(t313, 0x0u64, 32);
            if c.cc_ne() { blk = 332; continue; }
            blk = 331; continue;
        }
        331 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082bd95); };
            c.w64(8, t314);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700082bd99); }
            blk = 335; continue;
        }
        332 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082bda1); };
            c.w64(8, t315);
            let r316 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r316);
            if c.cc_o() { blk = 334; continue; }
            blk = 333; continue;
        }
        333 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            blk = 340; continue;
        }
        334 => {
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082bdb3); };
            c.w64(8, t317);
            let r318 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r318);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082bdc3); };
            c.w64(8, t319);
            blk = 335; continue;
        }
        335 => {
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082bdc7); };
            c.w64(9, t320);
            let Some(t321) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082bdce); };
            c.w64(11, t321);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x700082bdd5); }
            let Some(t322) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082bdda); };
            c.w64(15, t322);
            let Some(t323) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082bde1); };
            c.cmp(t323, c.r64(12), 64);
            if c.cc_a() { blk = 338; continue; }
            blk = 336; continue;
        }
        336 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700082bdf0, 0x700082bdeb);
        }
        337 => {
            c.w64(9, c.r64(0));
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082bdf3); };
            c.w64(8, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082bdf7); };
            c.w64(0, t325);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700082bdfb); }
            blk = 339; continue;
        }
        338 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700082be02); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 339; continue;
        }
        339 => {
            let Some(t326) = c.ld64(0x700082ba66u64) else { return RegfileExit::fault(0x700082be09); };
            c.w64(10, t326);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t327) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082be19); };
            c.w64(11, t327);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700082be20); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700082be24); }
            blk = 340; continue;
        }
        340 => {
            c.w64(1, c.r64(0));
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082be2d); };
            c.w64(0, t328);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082be31); };
            c.w64(3, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082be35); };
            c.w64(2, t330);
            c.w64(7, c.r64(9));
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082be3c); };
            c.w64(6, t331);
            return c.call(0x7000829cd4, 0x700082be45, 0x700082be40);
        }
        341 => {
            blk = 291; continue;
        }
        342 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700082be4f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x700082be53); }
            return c.call(0x70007febf4, 0x700082be5d, 0x700082be58);
        }
        343 => {
            c.w64(8, c.r64(0));
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700082be60); };
            c.w64(3, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082be64); };
            c.w64(1, t333);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700082be68); }
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700082be6d); };
            c.w64(9, t334);
            blk = 267; continue;
        }
        344 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x700082be7b); }
            return c.call(0x70007febf4, 0x700082be85, 0x700082be80);
        }
        345 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700082be88); }
            blk = 289; continue;
        }
        346 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700082be97); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700082be9c); }
            return c.call(0x70007febf4, 0x700082bea5, 0x700082bea0);
        }
        347 => {
            c.w64(8, c.r64(0));
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082bea8); };
            c.w64(0, t335);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700082beac); }
            blk = 298; continue;
        }
        348 => {
            return RegfileExit::fault(0x700082beb6);
        }
        349 => {
            blk = 351; continue;
        }
        350 => {
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082beb9); };
            c.w64(2, t336);
            blk = 351; continue;
        }
        351 => {
            c.w32(8, 0x256u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082bec3); }
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082bec5); };
            if !c.push64(t337) { return RegfileExit::fault(0x700082bec5); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082bec8); }
            let Some(t338) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082bec9); };
            c.w64(3, t338);
            c.w32(0, 0x3u64);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082bed5); };
            c.w64(6, t339);
            return c.call(0x700086ed74, 0x700082bede, 0x700082bed9);
        }
        352 => {
            return RegfileExit::fault(0x700082bede);
        }
        353 => {
            return RegfileExit::jump(0x700082bee0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082bef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082bef4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082bef8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082bef9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082befa); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700082beff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082bf03); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082bf17); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082bf1f); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082bf29); };
            c.w64(8, t3);
            c.w64(12, c.r64(8));
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082bf32); };
            c.w64(12, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082bf39); };
            c.w64(8, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700082bf40); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082bf4e); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let Some(t8) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082bf55); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 25; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700082bf62); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700082bf66); };
            c.w64(3, t9);
            c.w64(0, c.r64(9));
            return c.call(0x7000870914, 0x700082bf75, 0x700082bf70);
        }
        7 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082bf79); };
            c.w64(6, t10);
            return c.call(0x70008a17f4, 0x700082bf82, 0x700082bf7d);
        }
        9 => {
            blk = 14; continue;
        }
        10 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(2, c.r64(0));
            blk = 13; continue;
        }
        12 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700082bf93); }
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082bf97); };
            c.w64(8, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082bf9b); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(0, c.r64(8));
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082bfad); };
            c.w64(6, t13);
            return c.call(0x7000899a14, 0x700082bfb6, 0x700082bfb1);
        }
        17 => {
            c.w64(8, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082bfb9); };
            c.w64(0, t14);
            blk = 19; continue;
        }
        18 => {
            c.w32(8, 0x2u64);
            blk = 19; continue;
        }
        19 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082bfc5); };
            if !c.push64(t15) { return RegfileExit::fault(0x700082bfc5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700082bfc8); }
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082bfcc); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1c80)) else { return RegfileExit::fault(0x700082bfd0); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700082bfe4, 0x700082bfdf);
        }
        20 => {
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082bfe6); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082bfe7); };
            c.w64(8, t18);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082bfee); }
            c.w64(7, c.r64(0));
            c.w64(2, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082bffb); };
            c.w64(6, t19);
            return c.call(0x70007f7214, 0x700082c004, 0x700082bfff);
        }
        21 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082c004); };
            c.w64(6, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082c008); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082c00c); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082c010); };
            c.w64(1, t22);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082c016); };
            c.w64(7, t23);
            return c.call(0x7000829cd4, 0x700082c01f, 0x700082c01a);
        }
        22 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082c01f); };
            c.w64(8, t24);
            let r25 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c027); };
            c.w64(0, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082c02e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082c02f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082c035); }
            return c.ret(0x0, 0x700082c037);
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082c038); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082c03c); };
            c.w64(3, t27);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700082c04d, 0x700082c048);
        }
        24 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082c04d); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c051); };
            c.w64(0, t29);
            blk = 1; continue;
        }
        25 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2054, 0x700082c062, 0x700082c05d);
        }
        26 => {
            c.w64(9, c.r64(0));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082c065); };
            c.w64(6, t30);
            blk = 6; continue;
        }
        27 => {
            return RegfileExit::jump(0x700082c070);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082c074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082c074); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082c078); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082c079); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082c07a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082c07f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082c083); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(5));
            c.w64(11, c.r64(9).wrapping_add(0x18));
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082c09e); };
            c.w64(12, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700082c0a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082c0a7); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082c0b1); };
            c.w64(9, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082c0b7); };
            c.w64(9, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082c0be); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c0cc); };
            c.w32(15, t5);
            let r6 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r6);
            let Some(t7) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082c0d4); };
            c.cmp(t7, 0xbau64, 16);
            if c.cc_b() { blk = 25; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700082c0e1); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700082c0e5); };
            c.w64(3, t8);
            c.w64(0, c.r64(12));
            return c.call(0x7000870914, 0x700082c0f4, 0x700082c0ef);
        }
        7 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c0f8); };
            c.w64(6, t9);
            return c.call(0x70008a17f4, 0x700082c101, 0x700082c0fc);
        }
        9 => {
            blk = 14; continue;
        }
        10 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(2, c.r64(0));
            blk = 13; continue;
        }
        12 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700082c112); }
            let Some(t10) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082c116); };
            c.logic(t10 & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082c120); };
            c.w64(9, t11);
            let Some(t12) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c124); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082c12b); };
            c.logic(t14 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082c136); };
            c.cmp(t15, 0x1u64, 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082c13d); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082c141); };
            c.w64(8, t17);
            c.w64(11, c.r64(8));
            blk = 19; continue;
        }
        18 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082c14a); };
            c.w64(8, t18);
            c.w64(11, c.r64(8));
            blk = 19; continue;
        }
        19 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082c154); };
            if !c.push64(t19) { return RegfileExit::fault(0x700082c154); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700082c157); }
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c15b); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1c80)) else { return RegfileExit::fault(0x700082c15f); };
            c.w64(3, t21);
            c.w32(0, 0x1u64);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700082c173, 0x700082c16e);
        }
        20 => {
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082c175); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082c176); };
            c.w64(8, t22);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082c17d); }
            c.w64(7, c.r64(0));
            c.w64(2, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c18a); };
            c.w64(6, t23);
            return c.call(0x70007f7214, 0x700082c193, 0x700082c18e);
        }
        21 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082c193); };
            if !c.push64(t24) { return RegfileExit::fault(0x700082c193); }
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082c196); };
            if !c.push64(t25) { return RegfileExit::fault(0x700082c196); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700082c199); }
            c.w32(7, 0x2u64);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082c1a2); };
            c.w64(3, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082c1a6); };
            c.w64(1, t27);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c1ac); };
            c.w64(6, t28);
            return c.call(0x700082adf4, 0x700082c1b5, 0x700082c1b0);
        }
        22 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082c1b5); };
            c.w64(8, t29);
            let r30 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082c1bd); };
            c.w64(0, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082c1c4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082c1c5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082c1cb); }
            return c.ret(0x0, 0x700082c1cd);
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700082c1ce); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082c1d2); };
            c.w64(3, t32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700082c1e3, 0x700082c1de);
        }
        24 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c1e3); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082c1e7); };
            c.w64(0, t34);
            blk = 1; continue;
        }
        25 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a2054, 0x700082c1f8, 0x700082c1f3);
        }
        26 => {
            c.w64(12, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c1fb); };
            c.w64(6, t35);
            blk = 6; continue;
        }
        27 => {
            c.w32(8, 0x9cu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082c20a); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082c20c); };
            c.w64(3, t36);
            c.w32(0, 0x1u64);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c218); };
            c.w64(6, t37);
            return c.call(0x700086ed74, 0x700082c221, 0x700082c21c);
        }
        28 => {
            return RegfileExit::fault(0x700082c221);
        }
        29 => {
            return RegfileExit::jump(0x700082c224);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082c234(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16f8)) else { return RegfileExit::fault(0x700082c234); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082c254(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1700)) else { return RegfileExit::fault(0x700082c254); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082c274(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1708)) else { return RegfileExit::fault(0x700082c274); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082c294(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1710)) else { return RegfileExit::fault(0x700082c294); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082c2b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082c2b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082c2b8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082c2be); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c2c4); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x15)) else { return RegfileExit::fault(0x700082c2cb); };
            c.w32(1, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c2cf); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082c2d6); }
            let Some(t8) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082c2da); };
            c.cmp(t8, 0x5eu64, 16);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082c2e2); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            blk = 2; continue;
        }
        2 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c2e9); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082c2f0); };
            c.cmp(t13, 0x69u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c2f8); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            blk = 4; continue;
        }
        4 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082c2ff); };
            c.w64(8, (t16 as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let r17 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r17);
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 40; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082c31a); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082c321); };
            c.w64(9, t19);
            c.w64(2, c.r64(1).wrapping_mul(4).wrapping_add(0x8));
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082c330); };
            c.w64(12, t20);
            let Some(t21) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082c337); };
            c.cmp(t21, c.r64(11), 64);
            if c.cc_be() { blk = 30; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 30; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082c34e); }
            let r22 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r22);
            blk = 8; continue;
        }
        8 => {
            c.w32(11, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700082c359); };
            c.w64(15, t23);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700082c360); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700082c364); }
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082c376); };
            c.w64(11, t24);
            blk = 10; continue;
        }
        10 => {
            let r25 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r25);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x700082c381); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 10; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t26) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c38b); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082c391); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c397); };
            c.w32(11, t30);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xeau64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700082c3aa);
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082c3ab); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082c3b5); };
            c.w64(15, t33);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082c3b8); };
            c.w32(11, t34);
            c.w64(2, c.r64(15).wrapping_add(0x14));
            let Some(t35) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082c3c0); };
            c.cmp(t35, c.r64(2), 64);
            if c.cc_be() { blk = 32; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(8), c.r64(2)) { return RegfileExit::fault(0x700082c3ca); }
            let r36 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r36);
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700082c3d1); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082c3d5); };
            c.w64(11, t37);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700082c3dc); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700082c3e0); }
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700082c3e6); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700082c3ea); }
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c3ee); };
            c.w32(7, t38);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700082c3f7); }
            c.cmp(c.r64(7), 0x2fu64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700082c401);
        }
        17 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0xc3)) else { return RegfileExit::fault(0x700082c402); };
            c.w32(1, t39);
            let Some(t40) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082c408); };
            c.w64(7, t40);
            c.w64(15, c.r64(7).wrapping_add(0x2c));
            let Some(t41) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082c40f); };
            c.cmp(t41, c.r64(15), 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x700082c419); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082c420); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700082c424); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700082c428); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700082c42c); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x700082c430); }
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x700082c434); }
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x700082c438); }
            if !c.st32(c.r64(8).wrapping_add(0x1b), c.r32(2)) { return RegfileExit::fault(0x700082c43c); }
            c.w64(1, 0xfffffffffffffffcu64);
            if !c.st32(c.r64(8).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700082c447); }
            if !c.st32(c.r64(8).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x700082c44b); }
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082c44f); };
            c.w64(0, t42);
            if !c.st32(c.r64(8).wrapping_add(0x27), c.r32(0)) { return RegfileExit::fault(0x700082c453); }
            let Some(t43) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c457); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082c45d); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c463); };
            c.w32(9, t47);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700082c46d); }
            c.cmp(c.r64(9), 0x8eu64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x700082c47a);
        }
        21 => {
            let Some(t48) = c.ld32(c.r64(1).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700082c47b); };
            c.w32(7, t48);
            let r49 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r49);
            let Some(t50) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c484); };
            c.w32(1, t50);
            let r51 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r51);
            let Some(t52) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082c48a); };
            c.logic(t52 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082c490); };
            c.w64(1, t53);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t54) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c49c); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082c4a3); };
            c.w32(9, t56);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x700082c4bd);
        }
        26 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082c4be); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082c4bf); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082c4c0); }
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2134, 0x700082c4ce, 0x700082c4c9);
        }
        27 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082c4ce); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082c4d2); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c4d6); };
            c.w64(6, t59);
            blk = 28; continue;
        }
        28 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082c4da); };
            c.w64(1, t60);
            let Some(t61) = c.ld8(c.r64(1)) else { return RegfileExit::fault(0x700082c4e1); };
            c.logic(t61 & 0xeu64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082c4f0); }
            return c.ret(0x0, 0x700082c4f1);
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700082c4f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700082c4f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700082c4fa); }
            return c.call(0x70007febb4, 0x700082c503, 0x700082c4fe);
        }
        31 => {
            c.w64(9, c.r64(0));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c506); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082c50a); };
            c.w64(3, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082c50e); };
            c.w64(0, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082c512); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082c519); };
            c.w64(12, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082c520); };
            c.w64(1, t67);
            blk = 8; continue;
        }
        32 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700082c52e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700082c532); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700082c536); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700082c53a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082c53e); }
            return c.call(0x70007febf4, 0x700082c547, 0x700082c542);
        }
        33 => {
            c.w64(15, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c54a); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082c54e); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082c552); };
            c.w64(3, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082c556); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082c55a); };
            c.w64(8, t72);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082c561); };
            c.w64(12, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082c568); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082c56c); };
            c.w64(11, t75);
            blk = 15; continue;
        }
        34 => {
            c.w32(2, 0x2cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700082c57a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700082c57e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700082c582); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700082c586); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700082c592, 0x700082c58d);
        }
        35 => {
            c.w64(8, c.r64(0));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c597); };
            c.w64(6, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082c59b); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082c59f); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082c5a3); };
            c.w64(0, t79);
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082c5a7); };
            c.w64(11, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082c5ae); };
            c.w64(1, t81);
            blk = 19; continue;
        }
        36 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082c5b7); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082c5b8); };
            c.w64(3, t82);
            c.w32(0, 0x1u64);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082c5c4); };
            c.w64(6, t83);
            return c.call(0x700086ed74, 0x700082c5cd, 0x700082c5c8);
        }
        37 => {
            blk = 27; continue;
        }
        38 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082c5d2); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x1e88)) else { return RegfileExit::fault(0x700082c5d3); };
            c.w64(3, t84);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082c5e4, 0x700082c5df);
        }
        39 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082c5e4); };
            c.w64(8, t85);
            blk = 29; continue;
        }
        40 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700082c5ed); };
            c.w64(3, t86);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082c5fd, 0x700082c5f8);
        }
        41 => {
            return RegfileExit::fault(0x700082c5fd);
        }
        42 => {
            return RegfileExit::jump(0x700082c600);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082c614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082c614); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082c618); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700082c61e); };
            c.w32(0, t1);
            let r2 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700082c624); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082c628); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082c62c); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700082c630); };
            c.w64(1, t3);
            c.w64(2, c.r64(1));
            return c.call(0x70008b1fb4, 0x700082c63f, 0x700082c63a);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2e10)) else { return RegfileExit::fault(0x700082c63f); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082c646); };
            c.cmp(t5, 0x0u64, 64);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x700082c64c); };
            c.w64(2, t6);
            let Some(t7) = c.ld8(c.r64(2)) else { return RegfileExit::fault(0x700082c653); };
            c.cmp(t7, 0x0u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082c658); };
            c.w64(0, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082c65f); }
            return c.ret(0x0, 0x700082c660);
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700082c661); };
            c.w64(5, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1ea0)) else { return RegfileExit::fault(0x700082c665); };
            c.w64(3, t10);
            c.w32(0, 0x2u64);
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x700082c671); };
            c.w64(10, t11);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(10)) { return RegfileExit::fault(0x700082c676); }
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082c67b); };
            c.w64(10, t12);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082c680); }
            let Some(t13) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082c685); };
            c.w64(10, t13);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082c68a); }
            let Some(t14) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700082c68f); };
            c.w64(6, t14);
            let r15 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r15);
            return RegfileExit::jump(0x700086ed74);
        }
        5 => {
            return RegfileExit::jump(0x700082c6a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082c6b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082c6b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082c6b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700082c6be); };
            c.w32(0, t1);
            let r2 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700082c6c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082c6c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082c6cc); }
            return c.call(0x70008ba734, 0x700082c6d5, 0x700082c6d0);
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2e10)) else { return RegfileExit::fault(0x700082c6d5); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082c6dc); };
            c.cmp(t4, 0x0u64, 64);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x700082c6e2); };
            c.w64(2, t5);
            let Some(t6) = c.ld8(c.r64(2)) else { return RegfileExit::fault(0x700082c6e9); };
            c.cmp(t6, 0x0u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082c6ee); };
            c.w64(0, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082c6f5); }
            return c.ret(0x0, 0x700082c6f6);
        }
        4 => {
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700082c6f7); };
            c.w64(5, t8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1ea0)) else { return RegfileExit::fault(0x700082c6fb); };
            c.w64(3, t9);
            c.w32(0, 0x2u64);
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x700082c707); };
            c.w64(10, t10);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(10)) { return RegfileExit::fault(0x700082c70c); }
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082c711); };
            c.w64(10, t11);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082c716); }
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082c71b); };
            c.w64(10, t12);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082c720); }
            let Some(t13) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700082c725); };
            c.w64(6, t13);
            let r14 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r14);
            return RegfileExit::jump(0x700086ed74);
        }
        5 => {
            return RegfileExit::jump(0x700082c734);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082c754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082c754); };
            c.w64(0, t0);
            return c.ret(0x10, 0x700082c759);
        }
        1 => {
            return RegfileExit::jump(0x700082c760);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082c774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082c774); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082c778); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700082c77e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700082c785); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700082c789); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700082c78d); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2e10)) else { return RegfileExit::fault(0x700082c791); };
            c.w64(9, t3);
            let Some(t4) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082c798); };
            c.cmp(t4, 0x0u64, 64);
            if c.cc_ne() { blk = 61; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c7ab); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082c7b2); };
            c.cmp(t7, 0x83du64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c7bf); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082c7c6); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c7cd); };
            c.w32(12, t12);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x97u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x700082c7e0);
        }
        5 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0x263)) else { return RegfileExit::fault(0x700082c7e1); };
            c.w32(12, t13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082c7e8); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x700082c7f5); };
            c.w64(12, t15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082c7fc); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c807); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082c80e); };
            c.w64(12, t19);
            let Some(t20) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082c815); };
            c.w64(15, t20);
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082c819); };
            c.w32(11, t21);
            let r22 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r22);
            c.w64(2, c.r64(15).wrapping_add(0x14));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082c824); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082c82b); };
            c.cmp(t24, c.r64(2), 64);
            if c.cc_be() { blk = 69; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x700082c834); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c83c); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7du64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x700082c84c);
        }
        11 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700082c84d); };
            c.w32(15, t26);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700082c854); }
            c.w32(15, 0x6u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700082c85f); }
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c864); };
            c.w32(15, t27);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700082c874);
        }
        13 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082c875); };
            c.w32(15, t28);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700082c879); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700082c87e); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700082c883); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082c888); };
            c.w64(15, t29);
            let Some(t30) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082c88f); };
            c.w64(0, t30);
            c.w64(2, c.r64(0).wrapping_add(0x1c));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082c896); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082c89d); };
            c.cmp(t32, c.r64(2), 64);
            if c.cc_be() { blk = 71; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700082c8a6); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c8ad); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xbeu64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700082c8bf);
        }
        17 => {
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082c8c0); };
            c.w32(0, t34);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700082c8c7); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082c8cb); };
            c.w64(0, t35);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700082c8d2); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700082c8d6); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082c8da); };
            c.w64(0, t36);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x700082c8e1); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1050)) else { return RegfileExit::fault(0x700082c8e5); };
            c.w64(0, t37);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700082c8ec); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700082c8f0); }
            let Some(t38) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c8f4); };
            c.w32(0, t38);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t39) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4))) else { return RegfileExit::fault(0x700082c901); };
            c.w64(0, t39);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700082c905); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082c909); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700082c910); };
            c.w64(2, t41);
            c.w64(1, c.r64(2).wrapping_add(0x1c));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082c917); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700082c91e); };
            c.cmp(t43, c.r64(1), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700082c927); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c92e); };
            c.w32(2, t44);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x700082c941);
        }
        21 => {
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082c942); };
            c.w32(2, t45);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700082c949); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082c94c); };
            c.w64(2, t46);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700082c953); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700082c956); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082c959); };
            c.w64(2, t47);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700082c960); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1048)) else { return RegfileExit::fault(0x700082c963); };
            c.w64(2, t48);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700082c96a); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700082c96d); }
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c971); };
            c.w32(12, t49);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t50) = c.ld64(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x700082c97f); };
            c.w64(12, t50);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x700082c983); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082c987); };
            c.w64(12, t51);
            let Some(t52) = c.ld32(c.r64(12)) else { return RegfileExit::fault(0x700082c98e); };
            c.w32(12, t52);
            c.logic(c.r8(12) & 0xeu64, 8);
            if c.cc_ne() { blk = 100; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082c9a6); };
            c.w64(9, t53);
            blk = 24; continue;
        }
        24 => {
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(15));
            c.w64(2, c.r64(9));
            c.w64(6, c.r64(11));
            return c.call(0x70008b2ef4, 0x700082c9c1, 0x700082c9bc);
        }
        25 => {
            blk = 60; continue;
        }
        26 => {
            let Some(t54) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082c9c6); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082c9cd); };
            c.w32(9, t56);
            let r57 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082c9d4); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700082c9db); };
            c.w64(12, t59);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082c9e3); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082c9ea); };
            c.cmp(t61, c.r64(15), 64);
            if c.cc_be() { blk = 102; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700082c9f3); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082c9fb); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x700082ca0b);
        }
        30 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700082ca0c); };
            c.w32(12, t63);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700082ca13); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082ca1d); }
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082ca21); };
            c.w32(12, t64);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x700082ca31);
        }
        32 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ca32); };
            c.w32(12, t65);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082ca36); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082ca3a); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700082ca3e); }
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082ca42); };
            c.w32(12, t66);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xeau64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x700082ca55);
        }
        34 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082ca56); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082ca60); };
            c.w64(15, t69);
            let Some(t70) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082ca67); };
            c.w64(0, t70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082ca6a); };
            c.w32(12, t71);
            c.w64(2, c.r64(0).wrapping_add(0x14));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082ca73); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082ca7a); };
            c.cmp(t73, c.r64(2), 64);
            if c.cc_be() { blk = 104; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700082ca83); }
            let r74 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r74);
            blk = 36; continue;
        }
        36 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700082ca8a); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082ca8e); };
            c.w64(12, t75);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082ca95); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082ca99); }
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700082caa0); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700082caa4); }
            let Some(t76) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082caa8); };
            c.w64(2, t76);
            c.w64(7, c.r64(2).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082caaf); }
            let Some(t77) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082cab3); };
            c.cmp(t77, c.r64(7), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x700082cabc); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 38; continue;
        }
        38 => {
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082cac3); };
            c.w32(2, t78);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700082cacd); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x700082cada);
        }
        40 => {
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082cadb); };
            c.w32(2, t79);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700082cae2); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082cae6); };
            c.w64(2, t80);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700082caed); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700082caf1); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082caf5); };
            c.w64(2, t81);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700082cafc); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1050)) else { return RegfileExit::fault(0x700082cb00); };
            c.w64(2, t82);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700082cb07); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700082cb0b); }
            let Some(t83) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082cb0f); };
            c.w32(2, t83);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(13).wrapping_add(0x63f0));
            let Some(t84) = c.ld64(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x700082cb1c); };
            c.w64(2, t84);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x700082cb20); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082cb24); };
            c.w64(2, t85);
            let Some(t86) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082cb2b); };
            c.w64(1, t86);
            c.w64(7, c.r64(1).wrapping_add(0x1c));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082cb32); };
            c.w64(12, t87);
            let Some(t88) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082cb39); };
            c.cmp(t88, c.r64(7), 64);
            if c.cc_be() { blk = 108; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x700082cb43); }
            c.w64(12, c.r64(1).wrapping_add(0x1));
            blk = 42; continue;
        }
        42 => {
            let Some(t89) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082cb4a); };
            c.w32(2, t89);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700082cb54); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x700082cb61);
        }
        44 => {
            let Some(t90) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082cb62); };
            c.w32(9, t90);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082cb69); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082cb6e); };
            c.w64(9, t91);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700082cb75); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700082cb7a); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082cb7f); };
            c.w64(9, t92);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x700082cb86); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1048)) else { return RegfileExit::fault(0x700082cb8b); };
            c.w64(9, t93);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082cb92); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700082cb97); }
            let Some(t94) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082cb9c); };
            c.w32(9, t94);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(13).wrapping_add(0x63f0));
            let Some(t95) = c.ld64(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700082cbaa); };
            c.w64(9, t95);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x700082cbae); }
            let Some(t96) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082cbb3); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082cbba); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082cbc1); };
            c.w32(11, t100);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700082cbd4);
        }
        46 => {
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700082cbd5); };
            c.w32(7, t101);
            let r102 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r102);
            let Some(t103) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082cbdf); };
            c.w32(9, t103);
            let r104 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r104);
            let Some(t105) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082cbe6); };
            c.logic(t105 & 0x2u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t106) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082cbf3); };
            c.w32(9, t106);
            let r107 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r107);
            let Some(t108) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082cbfa); };
            c.w32(9, t108);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(9, c.r64(8));
            blk = 53; continue;
        }
        51 => {
            return RegfileExit::fault(0x700082cc19);
        }
        52 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082cc1a); };
            c.w64(9, t109);
            blk = 53; continue;
        }
        53 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082cc21); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082cc23); }
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082cc24); };
            c.w64(9, t110);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082cc2b); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700082cc3a, 0x700082cc35);
        }
        54 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cc3a); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cc3e); };
            c.w64(0, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082cc42); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082cc46); };
            c.w64(12, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cc4a); };
            c.w64(6, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cc4e); };
            c.w64(3, t116);
            blk = 55; continue;
        }
        55 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082cc52); };
            c.w64(9, t117);
            let Some(t118) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082cc59); };
            c.w32(9, t118);
            c.logic(c.r8(9) & 0xeu64, 8);
            if c.cc_ne() { blk = 137; continue; }
            blk = 56; continue;
        }
        56 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 112; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082cc70); };
            c.w64(9, t119);
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700082cc77); }
            return c.call(0x70008ba734, 0x700082cc80, 0x700082cc7b);
        }
        59 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cc80); };
            c.w64(0, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082cc84); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082cc88); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082cc8c); };
            c.w64(2, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cc90); };
            c.w64(6, t124);
            return c.call(0x70008b2ef4, 0x700082cc99, 0x700082cc94);
        }
        60 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cc99); };
            c.w64(0, t125);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082cca0); }
            return c.ret(0x0, 0x700082cca1);
        }
        61 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082cca2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082cca4); }
            c.w32(0, 0x1u64);
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1e90)) else { return RegfileExit::fault(0x700082ccad); };
            c.w64(3, t126);
            c.w64(9, c.r64(0));
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700082ccbf, 0x700082ccba);
        }
        62 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082ccbf); };
            c.w64(8, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082ccc3); };
            c.w64(6, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082ccc7); };
            c.w64(3, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cccb); };
            c.w64(0, t130);
            blk = 1; continue;
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082ccd4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700082ccd8); }
            c.w64(0, c.r64(3));
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x700082ccdf); };
            c.w64(3, t131);
            c.w64(12, c.r64(3));
            return c.call(0x7000870914, 0x700082ccee, 0x700082cce9);
        }
        64 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ccee); };
            c.w64(11, t132);
            let Some(t133) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082ccf2); };
            c.w32(8, t133);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            return RegfileExit::fault(0x700082cd05);
        }
        66 => {
            let Some(t134) = c.ld32(c.r64(11).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082cd06); };
            c.cmp(t134, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cd0f); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cd13); };
            c.w64(6, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cd17); };
            c.w64(3, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cd1b); };
            c.w64(0, t138);
            blk = 26; continue;
        }
        68 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cd24); };
            c.w64(8, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cd28); };
            c.w64(6, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cd2c); };
            c.w64(3, t141);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cd33); };
            c.w64(0, t142);
            blk = 7; continue;
        }
        69 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082cd41); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082cd45); }
            return c.call(0x70007febf4, 0x700082cd4e, 0x700082cd49);
        }
        70 => {
            c.w64(12, c.r64(0));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cd51); };
            c.w64(8, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cd55); };
            c.w64(6, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cd59); };
            c.w64(3, t145);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cd60); };
            c.w64(0, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082cd64); };
            c.w64(11, t147);
            blk = 9; continue;
        }
        71 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082cd72); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700082cd76); }
            return c.call(0x70007febf4, 0x700082cd7f, 0x700082cd7a);
        }
        72 => {
            c.w64(15, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cd82); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cd86); };
            c.w64(6, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cd8a); };
            c.w64(3, t150);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082cd91); };
            c.w64(11, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082cd95); };
            c.w64(12, t152);
            blk = 15; continue;
        }
        73 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700082cda3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082cda7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700082cdab); }
            return c.call(0x70007febf4, 0x700082cdb4, 0x700082cdaf);
        }
        74 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082cdb4); };
            c.w64(12, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cdb8); };
            c.w64(8, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cdbc); };
            c.w64(6, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cdc0); };
            c.w64(3, t156);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cdc7); };
            c.w64(15, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082cdcb); };
            c.w64(11, t158);
            blk = 19; continue;
        }
        75 => {
            let Some(t159) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082cdd4); };
            c.w32(12, t159);
            let r160 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r160);
            let Some(t161) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082cddb); };
            c.w32(12, t161);
            let r162 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r162);
            let Some(t163) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082cde3); };
            c.w32(2, t163);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700082cdee); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082cdf2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082cdf6); }
            c.cmp(c.r64(2), 0xeau64, 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x700082ce03);
        }
        77 => {
            let Some(t164) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082ce04); };
            c.w32(12, t164);
            let r165 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r165);
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082ce0f); };
            c.w64(2, t166);
            let Some(t167) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082ce16); };
            c.w64(1, t167);
            let Some(t168) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082ce19); };
            c.w32(12, t168);
            c.w64(7, c.r64(1).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700082ce22); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700082ce26); }
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082ce2a); };
            c.w64(1, t169);
            let Some(t170) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082ce31); };
            c.cmp(t170, c.r64(7), 64);
            if c.cc_a() { blk = 80; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700082ce40, 0x700082ce3b);
        }
        79 => {
            blk = 81; continue;
        }
        80 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x700082ce42); }
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082ce45); };
            c.w64(2, t171);
            let r172 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r172);
            c.w64(0, c.r64(2));
            blk = 81; continue;
        }
        81 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082ce50); };
            c.w64(8, t173);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700082ce54); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082ce58); };
            c.w64(8, t174);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700082ce5f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700082ce63); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082ce6a); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082ce6e); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082ce72); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082ce76); }
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082ce7a); };
            c.w64(8, t175);
            let Some(t176) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700082ce81); };
            c.w32(8, t176);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700082ce84); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700082ce88); }
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
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082ce9c); };
            c.w64(6, t177);
            blk = 96; continue;
        }
        84 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cea5); };
            c.w64(6, t178);
            let Some(t179) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082cea9); };
            c.w32(9, t179);
            let r180 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r180);
            let Some(t181) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082ceb0); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082ceb7); };
            c.w32(11, t183);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            return RegfileExit::fault(0x700082ceca);
        }
        86 => {
            let Some(t184) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700082cecb); };
            c.w32(7, t184);
            let r185 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r185);
            let Some(t186) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ced5); };
            c.w32(9, t186);
            let r187 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r187);
            let Some(t188) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082cedc); };
            c.logic(t188 & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cee3); };
            c.w64(3, t189);
            let Some(t190) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082cee7); };
            c.w32(9, t190);
            let r191 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r191);
            let Some(t192) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ceee); };
            c.w32(9, t192);
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
            return RegfileExit::fault(0x700082cf0d);
        }
        91 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082cf0e); };
            c.w64(9, t193);
            blk = 92; continue;
        }
        92 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082cf15); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082cf17); }
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082cf18); };
            c.w64(9, t194);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082cf1f); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700082cf2e, 0x700082cf29);
        }
        93 => {
            blk = 95; continue;
        }
        94 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082cf30); }
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082cf31); };
            c.w64(3, t195);
            c.w32(0, 0x1u64);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cf3d); };
            c.w64(6, t196);
            return c.call(0x700086ed74, 0x700082cf46, 0x700082cf41);
        }
        95 => {
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cf46); };
            c.w64(6, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082cf4a); };
            c.w64(0, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082cf4e); };
            c.w64(8, t199);
            blk = 96; continue;
        }
        96 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082cf58); }
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cf59); };
            if !c.push64(t200) { return RegfileExit::fault(0x700082cf59); }
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700082cf5c); };
            c.w64(3, t201);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700082cf70, 0x700082cf6b);
        }
        98 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082cf70); };
            c.w64(0, t202);
            blk = 99; continue;
        }
        99 => {
            c.w64(9, c.r64(0));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cf77); };
            c.w64(3, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cf7b); };
            c.w64(15, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082cf7f); };
            c.w64(0, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082cf83); };
            c.w64(11, t206);
            blk = 24; continue;
        }
        100 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082cf8c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082cf8d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082cf8e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082cf90); }
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082cf91); };
            c.w64(9, t207);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082cf98); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700082cf9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082cf9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082cfa2); }
            c.w32(0, 0x5u64);
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x25a0)) else { return RegfileExit::fault(0x700082cfab); };
            c.w64(3, t208);
            c.w64(9, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700082cfbd, 0x700082cfb8);
        }
        101 => {
            c.w64(9, c.r64(0));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cfc0); };
            c.w64(3, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cfc4); };
            c.w64(15, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082cfc8); };
            c.w64(0, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082cfcc); };
            c.w64(11, t212);
            blk = 24; continue;
        }
        102 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082cfda); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082cfde); }
            return c.call(0x70007febf4, 0x700082cfe7, 0x700082cfe2);
        }
        103 => {
            c.w64(11, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082cfea); };
            c.w64(8, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082cfee); };
            c.w64(6, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082cff2); };
            c.w64(3, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082cff6); };
            c.w64(9, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082cffa); };
            c.w64(0, t217);
            blk = 28; continue;
        }
        104 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082d008); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082d00c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x700082d010); }
            return c.call(0x70007febf4, 0x700082d019, 0x700082d014);
        }
        105 => {
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d019); };
            c.w64(12, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d01d); };
            c.w64(8, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d021); };
            c.w64(6, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d025); };
            c.w64(3, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d029); };
            c.w64(11, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082d02d); };
            c.w64(9, t223);
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d031); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d038); };
            c.w64(1, t225);
            blk = 36; continue;
        }
        106 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082d049); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082d04d); }
            return c.call(0x70007febf4, 0x700082d056, 0x700082d051);
        }
        107 => {
            c.w64(15, c.r64(0));
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d059); };
            c.w64(8, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d05d); };
            c.w64(0, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d061); };
            c.w64(6, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d065); };
            c.w64(3, t229);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d06c); };
            c.w64(11, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082d070); };
            c.w64(9, t231);
            blk = 38; continue;
        }
        108 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082d07e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082d082); }
            return c.call(0x70007febf4, 0x700082d08b, 0x700082d086);
        }
        109 => {
            c.w64(12, c.r64(0));
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d08e); };
            c.w64(8, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d092); };
            c.w64(0, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d096); };
            c.w64(15, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d09a); };
            c.w64(6, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d09e); };
            c.w64(3, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d0a2); };
            c.w64(11, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082d0a6); };
            c.w64(9, t238);
            blk = 42; continue;
        }
        110 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082d0af); }
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082d0b0); };
            c.w64(3, t239);
            c.w32(0, 0x1u64);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d0bc); };
            c.w64(6, t240);
            return c.call(0x700086ed74, 0x700082d0c5, 0x700082d0c0);
        }
        111 => {
            blk = 54; continue;
        }
        112 => {
            let Some(t241) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d0ca); };
            c.w32(9, t241);
            let r242 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r242);
            let Some(t243) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082d0d1); };
            c.w32(9, t243);
            let r244 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r244);
            let Some(t245) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d0d8); };
            c.w32(11, t245);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xeau64, 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return RegfileExit::fault(0x700082d0eb);
        }
        114 => {
            let Some(t246) = c.ld32(c.r64(9).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082d0ec); };
            c.w32(9, t246);
            let r247 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r247);
            let Some(t248) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082d0f6); };
            c.w32(9, t248);
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d0fa); };
            c.w64(11, t249);
            let Some(t250) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700082d101); };
            c.w64(2, t250);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082d108); }
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d10c); };
            c.w64(7, t251);
            let Some(t252) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700082d113); };
            c.cmp(t252, c.r64(1), 64);
            if c.cc_a() { blk = 117; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700082d122, 0x700082d11d);
        }
        116 => {
            c.w64(11, c.r64(0));
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d125); };
            c.w64(8, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d129); };
            c.w64(0, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d12d); };
            c.w64(15, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d131); };
            c.w64(12, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d135); };
            c.w64(6, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d139); };
            c.w64(3, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082d13d); };
            c.w64(9, t259);
            blk = 118; continue;
        }
        117 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700082d143); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            blk = 118; continue;
        }
        118 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082d14a); }
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082d14e); };
            c.w64(9, t260);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700082d155); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700082d159); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082d160); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082d164); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082d168); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082d16c); }
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082d170); };
            c.w64(9, t261);
            let Some(t262) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082d177); };
            c.w32(9, t262);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082d17a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082d17e); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 119; continue;
        }
        119 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t263) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d196); };
            c.w32(2, t263);
            let r264 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r264);
            let Some(t265) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082d19c); };
            c.w32(2, t265);
            let r266 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r266);
            let Some(t267) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d1a2); };
            c.w32(1, t267);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x8eu64, 64);
            if c.cc_a() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            return RegfileExit::fault(0x700082d1b4);
        }
        122 => {
            let Some(t268) = c.ld32(c.r64(2).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700082d1b5); };
            c.w32(7, t268);
            let r269 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r269);
            let Some(t270) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d1be); };
            c.w32(2, t270);
            let r271 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r271);
            let Some(t272) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082d1c4); };
            c.logic(t272 & 0x2u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 123; continue;
        }
        123 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t273) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d1d2); };
            c.w32(2, t273);
            let r274 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r274);
            let Some(t275) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082d1d8); };
            c.w32(2, t275);
            c.cmp(c.r16(2), 0x83du64, 16);
            if c.cc_ne() { blk = 128; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0x83du64, 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(2, c.r64(0));
            blk = 129; continue;
        }
        127 => {
            return RegfileExit::fault(0x700082d1f3);
        }
        128 => {
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082d1f4); };
            c.w64(2, t276);
            blk = 129; continue;
        }
        129 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082d1fb); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700082d1fc); }
            let Some(t277) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082d1fe); };
            c.w64(2, t277);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082d205); }
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2134, 0x700082d213, 0x700082d20e);
        }
        130 => {
            blk = 132; continue;
        }
        131 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082d215); }
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082d216); };
            c.w64(3, t278);
            c.w32(0, 0x1u64);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d222); };
            c.w64(6, t279);
            return c.call(0x700086ed74, 0x700082d22b, 0x700082d226);
        }
        132 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d22b); };
            c.w64(8, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d22f); };
            c.w64(0, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d233); };
            c.w64(15, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d237); };
            c.w64(12, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d23b); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d23f); };
            c.w64(3, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d243); };
            c.w64(11, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082d247); };
            c.w64(9, t287);
            blk = 133; continue;
        }
        133 => {
            c.logic(c.r8(9) & 0x6u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 134; continue;
        }
        134 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700082d251); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082d253); }
            c.w32(0, 0x2u64);
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700082d259); };
            c.w64(3, t288);
            c.w64(9, c.r64(3));
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700082d26b, 0x700082d266);
        }
        135 => {
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d26b); };
            c.w64(8, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d26f); };
            c.w64(0, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d273); };
            c.w64(15, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d277); };
            c.w64(12, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d27b); };
            c.w64(6, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d27f); };
            c.w64(3, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d283); };
            c.w64(11, t295);
            blk = 136; continue;
        }
        136 => {
            c.w64(9, c.r64(11));
            blk = 58; continue;
        }
        137 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082d28f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082d290); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082d291); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700082d293); }
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082d295); };
            c.w64(9, t296);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082d29c); }
            c.w32(0, 0x5u64);
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x25a8)) else { return RegfileExit::fault(0x700082d2a3); };
            c.w64(3, t297);
            c.w64(9, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700082d2b5, 0x700082d2b0);
        }
        138 => {
            c.w64(9, c.r64(0));
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d2b8); };
            c.w64(0, t298);
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d2bc); };
            c.w64(6, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d2c0); };
            c.w64(3, t300);
            blk = 58; continue;
        }
        139 => {
            return RegfileExit::jump(0x700082d2cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082d2d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082d2d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082d2d8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700082d2de); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700082d2e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700082d2e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700082d2ed); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2e10)) else { return RegfileExit::fault(0x700082d2f1); };
            c.w64(9, t3);
            let Some(t4) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082d2f8); };
            c.cmp(t4, 0x0u64, 64);
            if c.cc_ne() { blk = 61; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d30b); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082d312); };
            c.cmp(t7, 0x83du64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d31f); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082d326); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d32d); };
            c.w32(12, t12);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x97u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x700082d340);
        }
        5 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0x263)) else { return RegfileExit::fault(0x700082d341); };
            c.w32(12, t13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082d348); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x700082d355); };
            c.w64(12, t15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082d35c); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d367); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d36e); };
            c.w64(12, t19);
            let Some(t20) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082d375); };
            c.w64(15, t20);
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082d379); };
            c.w32(11, t21);
            let r22 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r22);
            c.w64(2, c.r64(15).wrapping_add(0x14));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d384); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082d38b); };
            c.cmp(t24, c.r64(2), 64);
            if c.cc_be() { blk = 69; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x700082d394); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d39c); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7du64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x700082d3ac);
        }
        11 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700082d3ad); };
            c.w32(15, t26);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700082d3b4); }
            c.w32(15, 0x6u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700082d3bf); }
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d3c4); };
            c.w32(15, t27);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700082d3d4);
        }
        13 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082d3d5); };
            c.w32(15, t28);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700082d3d9); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700082d3de); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700082d3e3); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d3e8); };
            c.w64(15, t29);
            let Some(t30) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082d3ef); };
            c.w64(0, t30);
            c.w64(2, c.r64(0).wrapping_add(0x1c));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d3f6); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082d3fd); };
            c.cmp(t32, c.r64(2), 64);
            if c.cc_be() { blk = 71; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700082d406); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d40d); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xbeu64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700082d41f);
        }
        17 => {
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082d420); };
            c.w32(0, t34);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700082d427); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082d42b); };
            c.w64(0, t35);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700082d432); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700082d436); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082d43a); };
            c.w64(0, t36);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x700082d441); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1050)) else { return RegfileExit::fault(0x700082d445); };
            c.w64(0, t37);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700082d44c); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700082d450); }
            let Some(t38) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d454); };
            c.w32(0, t38);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t39) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4))) else { return RegfileExit::fault(0x700082d461); };
            c.w64(0, t39);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700082d465); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d469); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700082d470); };
            c.w64(2, t41);
            c.w64(1, c.r64(2).wrapping_add(0x1c));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d477); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700082d47e); };
            c.cmp(t43, c.r64(1), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700082d487); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d48e); };
            c.w32(2, t44);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x700082d4a1);
        }
        21 => {
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082d4a2); };
            c.w32(2, t45);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700082d4a9); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082d4ac); };
            c.w64(2, t46);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700082d4b3); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700082d4b6); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082d4b9); };
            c.w64(2, t47);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700082d4c0); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1048)) else { return RegfileExit::fault(0x700082d4c3); };
            c.w64(2, t48);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700082d4ca); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700082d4cd); }
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d4d1); };
            c.w32(12, t49);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t50) = c.ld64(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x700082d4df); };
            c.w64(12, t50);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x700082d4e3); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082d4e7); };
            c.w64(12, t51);
            let Some(t52) = c.ld32(c.r64(12)) else { return RegfileExit::fault(0x700082d4ee); };
            c.w32(12, t52);
            c.logic(c.r8(12) & 0xeu64, 8);
            if c.cc_ne() { blk = 100; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082d506); };
            c.w64(9, t53);
            blk = 24; continue;
        }
        24 => {
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(15));
            c.w64(2, c.r64(9));
            c.w64(6, c.r64(11));
            return c.call(0x70008b2ef4, 0x700082d521, 0x700082d51c);
        }
        25 => {
            blk = 60; continue;
        }
        26 => {
            let Some(t54) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d526); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082d52d); };
            c.w32(9, t56);
            let r57 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d534); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700082d53b); };
            c.w64(12, t59);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d543); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082d54a); };
            c.cmp(t61, c.r64(15), 64);
            if c.cc_be() { blk = 102; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700082d553); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d55b); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x700082d56b);
        }
        30 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700082d56c); };
            c.w32(12, t63);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700082d573); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082d57d); }
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d581); };
            c.w32(12, t64);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x700082d591);
        }
        32 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082d592); };
            c.w32(12, t65);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082d596); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082d59a); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700082d59e); }
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d5a2); };
            c.w32(12, t66);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xeau64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x700082d5b5);
        }
        34 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082d5b6); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d5c0); };
            c.w64(15, t69);
            let Some(t70) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082d5c7); };
            c.w64(0, t70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082d5ca); };
            c.w32(12, t71);
            c.w64(2, c.r64(0).wrapping_add(0x14));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d5d3); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082d5da); };
            c.cmp(t73, c.r64(2), 64);
            if c.cc_be() { blk = 104; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700082d5e3); }
            let r74 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r74);
            blk = 36; continue;
        }
        36 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700082d5ea); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082d5ee); };
            c.w64(12, t75);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082d5f5); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082d5f9); }
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700082d600); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700082d604); }
            let Some(t76) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700082d608); };
            c.w64(2, t76);
            c.w64(7, c.r64(2).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082d60f); }
            let Some(t77) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082d613); };
            c.cmp(t77, c.r64(7), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x700082d61c); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 38; continue;
        }
        38 => {
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d623); };
            c.w32(2, t78);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700082d62d); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x700082d63a);
        }
        40 => {
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082d63b); };
            c.w32(2, t79);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700082d642); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082d646); };
            c.w64(2, t80);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700082d64d); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700082d651); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082d655); };
            c.w64(2, t81);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700082d65c); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1050)) else { return RegfileExit::fault(0x700082d660); };
            c.w64(2, t82);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700082d667); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700082d66b); }
            let Some(t83) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d66f); };
            c.w32(2, t83);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(13).wrapping_add(0x63f0));
            let Some(t84) = c.ld64(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x700082d67c); };
            c.w64(2, t84);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x700082d680); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d684); };
            c.w64(2, t85);
            let Some(t86) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082d68b); };
            c.w64(1, t86);
            c.w64(7, c.r64(1).wrapping_add(0x1c));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d692); };
            c.w64(12, t87);
            let Some(t88) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082d699); };
            c.cmp(t88, c.r64(7), 64);
            if c.cc_be() { blk = 108; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x700082d6a3); }
            c.w64(12, c.r64(1).wrapping_add(0x1));
            blk = 42; continue;
        }
        42 => {
            let Some(t89) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d6aa); };
            c.w32(2, t89);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700082d6b4); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x700082d6c1);
        }
        44 => {
            let Some(t90) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700082d6c2); };
            c.w32(9, t90);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082d6c9); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082d6ce); };
            c.w64(9, t91);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700082d6d5); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700082d6da); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700082d6df); };
            c.w64(9, t92);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x700082d6e6); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1048)) else { return RegfileExit::fault(0x700082d6eb); };
            c.w64(9, t93);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082d6f2); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700082d6f7); }
            let Some(t94) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d6fc); };
            c.w32(9, t94);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(13).wrapping_add(0x63f0));
            let Some(t95) = c.ld64(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700082d70a); };
            c.w64(9, t95);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x700082d70e); }
            let Some(t96) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d713); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082d71a); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d721); };
            c.w32(11, t100);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700082d734);
        }
        46 => {
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700082d735); };
            c.w32(7, t101);
            let r102 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r102);
            let Some(t103) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d73f); };
            c.w32(9, t103);
            let r104 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r104);
            let Some(t105) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082d746); };
            c.logic(t105 & 0x2u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t106) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d753); };
            c.w32(9, t106);
            let r107 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r107);
            let Some(t108) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082d75a); };
            c.w32(9, t108);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(9, c.r64(8));
            blk = 53; continue;
        }
        51 => {
            return RegfileExit::fault(0x700082d779);
        }
        52 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082d77a); };
            c.w64(9, t109);
            blk = 53; continue;
        }
        53 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082d781); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082d783); }
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082d784); };
            c.w64(9, t110);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082d78b); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700082d79a, 0x700082d795);
        }
        54 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d79a); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d79e); };
            c.w64(0, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d7a2); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d7a6); };
            c.w64(12, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d7aa); };
            c.w64(6, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d7ae); };
            c.w64(3, t116);
            blk = 55; continue;
        }
        55 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082d7b2); };
            c.w64(9, t117);
            let Some(t118) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082d7b9); };
            c.w32(9, t118);
            c.logic(c.r8(9) & 0xeu64, 8);
            if c.cc_ne() { blk = 137; continue; }
            blk = 56; continue;
        }
        56 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 112; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082d7d0); };
            c.w64(9, t119);
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700082d7d7); }
            return c.call(0x70008ba734, 0x700082d7e0, 0x700082d7db);
        }
        59 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d7e0); };
            c.w64(0, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d7e4); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d7e8); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d7ec); };
            c.w64(2, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d7f0); };
            c.w64(6, t124);
            return c.call(0x70008b2ef4, 0x700082d7f9, 0x700082d7f4);
        }
        60 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d7f9); };
            c.w64(0, t125);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082d800); }
            return c.ret(0x0, 0x700082d801);
        }
        61 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082d802); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082d804); }
            c.w32(0, 0x1u64);
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1e90)) else { return RegfileExit::fault(0x700082d80d); };
            c.w64(3, t126);
            c.w64(9, c.r64(0));
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700082d81f, 0x700082d81a);
        }
        62 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d81f); };
            c.w64(8, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d823); };
            c.w64(6, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d827); };
            c.w64(3, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d82b); };
            c.w64(0, t130);
            blk = 1; continue;
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082d834); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700082d838); }
            c.w64(0, c.r64(3));
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x700082d83f); };
            c.w64(3, t131);
            c.w64(12, c.r64(3));
            return c.call(0x7000870914, 0x700082d84e, 0x700082d849);
        }
        64 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082d84e); };
            c.w64(11, t132);
            let Some(t133) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d852); };
            c.w32(8, t133);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            return RegfileExit::fault(0x700082d865);
        }
        66 => {
            let Some(t134) = c.ld32(c.r64(11).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082d866); };
            c.cmp(t134, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d86f); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d873); };
            c.w64(6, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d877); };
            c.w64(3, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d87b); };
            c.w64(0, t138);
            blk = 26; continue;
        }
        68 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d884); };
            c.w64(8, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d888); };
            c.w64(6, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d88c); };
            c.w64(3, t141);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d893); };
            c.w64(0, t142);
            blk = 7; continue;
        }
        69 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082d8a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082d8a5); }
            return c.call(0x70007febf4, 0x700082d8ae, 0x700082d8a9);
        }
        70 => {
            c.w64(12, c.r64(0));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d8b1); };
            c.w64(8, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d8b5); };
            c.w64(6, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d8b9); };
            c.w64(3, t145);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d8c0); };
            c.w64(0, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d8c4); };
            c.w64(11, t147);
            blk = 9; continue;
        }
        71 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082d8d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700082d8d6); }
            return c.call(0x70007febf4, 0x700082d8df, 0x700082d8da);
        }
        72 => {
            c.w64(15, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d8e2); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d8e6); };
            c.w64(6, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d8ea); };
            c.w64(3, t150);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d8f1); };
            c.w64(11, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d8f5); };
            c.w64(12, t152);
            blk = 15; continue;
        }
        73 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700082d903); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082d907); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700082d90b); }
            return c.call(0x70007febf4, 0x700082d914, 0x700082d90f);
        }
        74 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d914); };
            c.w64(12, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082d918); };
            c.w64(8, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d91c); };
            c.w64(6, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082d920); };
            c.w64(3, t156);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082d927); };
            c.w64(15, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082d92b); };
            c.w64(11, t158);
            blk = 19; continue;
        }
        75 => {
            let Some(t159) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082d934); };
            c.w32(12, t159);
            let r160 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r160);
            let Some(t161) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082d93b); };
            c.w32(12, t161);
            let r162 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r162);
            let Some(t163) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082d943); };
            c.w32(2, t163);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700082d94e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082d952); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082d956); }
            c.cmp(c.r64(2), 0xeau64, 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x700082d963);
        }
        77 => {
            let Some(t164) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082d964); };
            c.w32(12, t164);
            let r165 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r165);
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082d96f); };
            c.w64(2, t166);
            let Some(t167) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700082d976); };
            c.w64(1, t167);
            let Some(t168) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082d979); };
            c.w32(12, t168);
            c.w64(7, c.r64(1).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700082d982); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700082d986); }
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082d98a); };
            c.w64(1, t169);
            let Some(t170) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700082d991); };
            c.cmp(t170, c.r64(7), 64);
            if c.cc_a() { blk = 80; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700082d9a0, 0x700082d99b);
        }
        79 => {
            blk = 81; continue;
        }
        80 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x700082d9a2); }
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082d9a5); };
            c.w64(2, t171);
            let r172 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r172);
            c.w64(0, c.r64(2));
            blk = 81; continue;
        }
        81 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082d9b0); };
            c.w64(8, t173);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700082d9b4); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082d9b8); };
            c.w64(8, t174);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700082d9bf); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700082d9c3); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082d9ca); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082d9ce); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082d9d2); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082d9d6); }
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082d9da); };
            c.w64(8, t175);
            let Some(t176) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700082d9e1); };
            c.w32(8, t176);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700082d9e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700082d9e8); }
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
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082d9fc); };
            c.w64(6, t177);
            blk = 96; continue;
        }
        84 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082da05); };
            c.w64(6, t178);
            let Some(t179) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082da09); };
            c.w32(9, t179);
            let r180 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r180);
            let Some(t181) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082da10); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082da17); };
            c.w32(11, t183);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            return RegfileExit::fault(0x700082da2a);
        }
        86 => {
            let Some(t184) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700082da2b); };
            c.w32(7, t184);
            let r185 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r185);
            let Some(t186) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082da35); };
            c.w32(9, t186);
            let r187 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r187);
            let Some(t188) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082da3c); };
            c.logic(t188 & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082da43); };
            c.w64(3, t189);
            let Some(t190) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082da47); };
            c.w32(9, t190);
            let r191 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r191);
            let Some(t192) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082da4e); };
            c.w32(9, t192);
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
            return RegfileExit::fault(0x700082da6d);
        }
        91 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082da6e); };
            c.w64(9, t193);
            blk = 92; continue;
        }
        92 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082da75); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082da77); }
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082da78); };
            c.w64(9, t194);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082da7f); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700082da8e, 0x700082da89);
        }
        93 => {
            blk = 95; continue;
        }
        94 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082da90); }
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082da91); };
            c.w64(3, t195);
            c.w32(0, 0x1u64);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082da9d); };
            c.w64(6, t196);
            return c.call(0x700086ed74, 0x700082daa6, 0x700082daa1);
        }
        95 => {
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082daa6); };
            c.w64(6, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082daaa); };
            c.w64(0, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082daae); };
            c.w64(8, t199);
            blk = 96; continue;
        }
        96 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082dab8); }
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082dab9); };
            if !c.push64(t200) { return RegfileExit::fault(0x700082dab9); }
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700082dabc); };
            c.w64(3, t201);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700082dad0, 0x700082dacb);
        }
        98 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082dad0); };
            c.w64(0, t202);
            blk = 99; continue;
        }
        99 => {
            c.w64(9, c.r64(0));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082dad7); };
            c.w64(3, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082dadb); };
            c.w64(15, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082dadf); };
            c.w64(0, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082dae3); };
            c.w64(11, t206);
            blk = 24; continue;
        }
        100 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082daec); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082daed); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082daee); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082daf0); }
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700082daf1); };
            c.w64(9, t207);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082daf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700082dafa); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700082dafe); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700082db02); }
            c.w32(0, 0x5u64);
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x25a0)) else { return RegfileExit::fault(0x700082db0b); };
            c.w64(3, t208);
            c.w64(9, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700082db1d, 0x700082db18);
        }
        101 => {
            c.w64(9, c.r64(0));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082db20); };
            c.w64(3, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082db24); };
            c.w64(15, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082db28); };
            c.w64(0, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082db2c); };
            c.w64(11, t212);
            blk = 24; continue;
        }
        102 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082db3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082db3e); }
            return c.call(0x70007febf4, 0x700082db47, 0x700082db42);
        }
        103 => {
            c.w64(11, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082db4a); };
            c.w64(8, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082db4e); };
            c.w64(6, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082db52); };
            c.w64(3, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082db56); };
            c.w64(9, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082db5a); };
            c.w64(0, t217);
            blk = 28; continue;
        }
        104 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082db68); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082db6c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x700082db70); }
            return c.call(0x70007febf4, 0x700082db79, 0x700082db74);
        }
        105 => {
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082db79); };
            c.w64(12, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082db7d); };
            c.w64(8, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082db81); };
            c.w64(6, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082db85); };
            c.w64(3, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082db89); };
            c.w64(11, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082db8d); };
            c.w64(9, t223);
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082db91); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082db98); };
            c.w64(1, t225);
            blk = 36; continue;
        }
        106 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082dba9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082dbad); }
            return c.call(0x70007febf4, 0x700082dbb6, 0x700082dbb1);
        }
        107 => {
            c.w64(15, c.r64(0));
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082dbb9); };
            c.w64(8, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082dbbd); };
            c.w64(0, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082dbc1); };
            c.w64(6, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082dbc5); };
            c.w64(3, t229);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082dbcc); };
            c.w64(11, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082dbd0); };
            c.w64(9, t231);
            blk = 38; continue;
        }
        108 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082dbde); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082dbe2); }
            return c.call(0x70007febf4, 0x700082dbeb, 0x700082dbe6);
        }
        109 => {
            c.w64(12, c.r64(0));
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082dbee); };
            c.w64(8, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082dbf2); };
            c.w64(0, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082dbf6); };
            c.w64(15, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082dbfa); };
            c.w64(6, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082dbfe); };
            c.w64(3, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082dc02); };
            c.w64(11, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082dc06); };
            c.w64(9, t238);
            blk = 42; continue;
        }
        110 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082dc0f); }
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082dc10); };
            c.w64(3, t239);
            c.w32(0, 0x1u64);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082dc1c); };
            c.w64(6, t240);
            return c.call(0x700086ed74, 0x700082dc25, 0x700082dc20);
        }
        111 => {
            blk = 54; continue;
        }
        112 => {
            let Some(t241) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082dc2a); };
            c.w32(9, t241);
            let r242 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r242);
            let Some(t243) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082dc31); };
            c.w32(9, t243);
            let r244 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r244);
            let Some(t245) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082dc38); };
            c.w32(11, t245);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xeau64, 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return RegfileExit::fault(0x700082dc4b);
        }
        114 => {
            let Some(t246) = c.ld32(c.r64(9).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700082dc4c); };
            c.w32(9, t246);
            let r247 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r247);
            let Some(t248) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082dc56); };
            c.w32(9, t248);
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082dc5a); };
            c.w64(11, t249);
            let Some(t250) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700082dc61); };
            c.w64(2, t250);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082dc68); }
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082dc6c); };
            c.w64(7, t251);
            let Some(t252) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700082dc73); };
            c.cmp(t252, c.r64(1), 64);
            if c.cc_a() { blk = 117; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700082dc82, 0x700082dc7d);
        }
        116 => {
            c.w64(11, c.r64(0));
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082dc85); };
            c.w64(8, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082dc89); };
            c.w64(0, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082dc8d); };
            c.w64(15, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082dc91); };
            c.w64(12, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082dc95); };
            c.w64(6, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082dc99); };
            c.w64(3, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082dc9d); };
            c.w64(9, t259);
            blk = 118; continue;
        }
        117 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700082dca3); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            blk = 118; continue;
        }
        118 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082dcaa); }
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082dcae); };
            c.w64(9, t260);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700082dcb5); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700082dcb9); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082dcc0); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082dcc4); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700082dcc8); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700082dccc); }
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700082dcd0); };
            c.w64(9, t261);
            let Some(t262) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082dcd7); };
            c.w32(9, t262);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700082dcda); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700082dcde); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 119; continue;
        }
        119 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t263) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082dcf6); };
            c.w32(2, t263);
            let r264 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r264);
            let Some(t265) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082dcfc); };
            c.w32(2, t265);
            let r266 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r266);
            let Some(t267) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082dd02); };
            c.w32(1, t267);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x8eu64, 64);
            if c.cc_a() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            return RegfileExit::fault(0x700082dd14);
        }
        122 => {
            let Some(t268) = c.ld32(c.r64(2).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700082dd15); };
            c.w32(7, t268);
            let r269 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r269);
            let Some(t270) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082dd1e); };
            c.w32(2, t270);
            let r271 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r271);
            let Some(t272) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082dd24); };
            c.logic(t272 & 0x2u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 123; continue;
        }
        123 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t273) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082dd32); };
            c.w32(2, t273);
            let r274 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r274);
            let Some(t275) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082dd38); };
            c.w32(2, t275);
            c.cmp(c.r16(2), 0x83du64, 16);
            if c.cc_ne() { blk = 128; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0x83du64, 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(2, c.r64(0));
            blk = 129; continue;
        }
        127 => {
            return RegfileExit::fault(0x700082dd53);
        }
        128 => {
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082dd54); };
            c.w64(2, t276);
            blk = 129; continue;
        }
        129 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082dd5b); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700082dd5c); }
            let Some(t277) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082dd5e); };
            c.w64(2, t277);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082dd65); }
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2134, 0x700082dd73, 0x700082dd6e);
        }
        130 => {
            blk = 132; continue;
        }
        131 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082dd75); }
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082dd76); };
            c.w64(3, t278);
            c.w32(0, 0x1u64);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082dd82); };
            c.w64(6, t279);
            return c.call(0x700086ed74, 0x700082dd8b, 0x700082dd86);
        }
        132 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082dd8b); };
            c.w64(8, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082dd8f); };
            c.w64(0, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082dd93); };
            c.w64(15, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082dd97); };
            c.w64(12, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082dd9b); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082dd9f); };
            c.w64(3, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082dda3); };
            c.w64(11, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700082dda7); };
            c.w64(9, t287);
            blk = 133; continue;
        }
        133 => {
            c.logic(c.r8(9) & 0x6u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 134; continue;
        }
        134 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700082ddb1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ddb3); }
            c.w32(0, 0x2u64);
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700082ddb9); };
            c.w64(3, t288);
            c.w64(9, c.r64(3));
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700082ddcb, 0x700082ddc6);
        }
        135 => {
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082ddcb); };
            c.w64(8, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ddcf); };
            c.w64(0, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ddd3); };
            c.w64(15, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082ddd7); };
            c.w64(12, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082dddb); };
            c.w64(6, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082dddf); };
            c.w64(3, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082dde3); };
            c.w64(11, t295);
            blk = 136; continue;
        }
        136 => {
            c.w64(9, c.r64(11));
            blk = 58; continue;
        }
        137 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082ddef); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ddf0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082ddf1); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700082ddf3); }
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700082ddf5); };
            c.w64(9, t296);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700082ddfc); }
            c.w32(0, 0x5u64);
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x25a8)) else { return RegfileExit::fault(0x700082de03); };
            c.w64(3, t297);
            c.w64(9, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700082de15, 0x700082de10);
        }
        138 => {
            c.w64(9, c.r64(0));
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082de18); };
            c.w64(0, t298);
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082de1c); };
            c.w64(6, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082de20); };
            c.w64(3, t300);
            blk = 58; continue;
        }
        139 => {
            return RegfileExit::jump(0x700082de2c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082de34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082de34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082de38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082de39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082de3a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082de3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082de45); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700082de54);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082de55); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x700082de5b); };
            c.w64(1, t5);
            let Some(t6) = c.ld8(c.r64(1)) else { return RegfileExit::fault(0x700082de62); };
            c.cmp(t6, 0x0u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(1, 0x4u64);
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x700082de6c); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082de6f); };
            c.w64(0, t7);
            return c.call(0x70007fa8f4, 0x700082de78, 0x700082de73);
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082de78); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082de7f); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082de86); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x10, 0x700082de8d);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082de90); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082de97); }
            return c.ret(0x0, 0x700082de99);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700082de9a); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082de9e); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082deac, 0x700082dea7);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082deac); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        10 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700082deb2); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082deb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700082deb9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700082debd); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1e98)) else { return RegfileExit::fault(0x700082dec1); };
            c.w64(3, t14);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082ded2, 0x700082decd);
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ded2); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082ded6); };
            c.w64(6, t16);
            blk = 4; continue;
        }
        12 => {
            return RegfileExit::jump(0x700082dee0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082def4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082def4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082def8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082def9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082defa); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082deff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082df05); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700082df14);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082df15); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x700082df1b); };
            c.w64(1, t5);
            let Some(t6) = c.ld8(c.r64(1)) else { return RegfileExit::fault(0x700082df22); };
            c.cmp(t6, 0x0u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x700082df29); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082df2c); };
            c.w64(0, t7);
            return c.call(0x70007fa8f4, 0x700082df35, 0x700082df30);
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082df35); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082df3c); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082df43); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x10, 0x700082df4a);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082df4d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082df54); }
            return c.ret(0x0, 0x700082df56);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700082df57); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082df5b); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082df69, 0x700082df64);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082df69); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        10 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700082df6f); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082df75); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700082df76); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700082df7a); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1e98)) else { return RegfileExit::fault(0x700082df7e); };
            c.w64(3, t14);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082df8f, 0x700082df8a);
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082df8f); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082df93); };
            c.w64(6, t16);
            blk = 4; continue;
        }
        12 => {
            return RegfileExit::jump(0x700082df9c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082dfb4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1718)) else { return RegfileExit::fault(0x700082dfb4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082dfd4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1720)) else { return RegfileExit::fault(0x700082dfd4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082dff4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1728)) else { return RegfileExit::fault(0x700082dff4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e014(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1730)) else { return RegfileExit::fault(0x700082e014); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e034(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1738)) else { return RegfileExit::fault(0x700082e034); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e054(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1740)) else { return RegfileExit::fault(0x700082e054); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e074(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1748)) else { return RegfileExit::fault(0x700082e074); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e094(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1750)) else { return RegfileExit::fault(0x700082e094); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e0b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1758)) else { return RegfileExit::fault(0x700082e0b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e0d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1760)) else { return RegfileExit::fault(0x700082e0d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e0f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1768)) else { return RegfileExit::fault(0x700082e0f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e114(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1770)) else { return RegfileExit::fault(0x700082e114); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e134(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1778)) else { return RegfileExit::fault(0x700082e134); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e154(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1780)) else { return RegfileExit::fault(0x700082e154); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e174(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1788)) else { return RegfileExit::fault(0x700082e174); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e194(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1790)) else { return RegfileExit::fault(0x700082e194); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e1b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1798)) else { return RegfileExit::fault(0x700082e1b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e1d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17a0)) else { return RegfileExit::fault(0x700082e1d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e1f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17a8)) else { return RegfileExit::fault(0x700082e1f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e214(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17b0)) else { return RegfileExit::fault(0x700082e214); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e234(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17b8)) else { return RegfileExit::fault(0x700082e234); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e254(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17c0)) else { return RegfileExit::fault(0x700082e254); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e274(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17c8)) else { return RegfileExit::fault(0x700082e274); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e294(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17d0)) else { return RegfileExit::fault(0x700082e294); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e2b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17d8)) else { return RegfileExit::fault(0x700082e2b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e2d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17e0)) else { return RegfileExit::fault(0x700082e2d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e2f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17e8)) else { return RegfileExit::fault(0x700082e2f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e314(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17f0)) else { return RegfileExit::fault(0x700082e314); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e334(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x17f8)) else { return RegfileExit::fault(0x700082e334); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e354(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1800)) else { return RegfileExit::fault(0x700082e354); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e374(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1808)) else { return RegfileExit::fault(0x700082e374); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e394(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1810)) else { return RegfileExit::fault(0x700082e394); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e3b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1818)) else { return RegfileExit::fault(0x700082e3b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e3d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1820)) else { return RegfileExit::fault(0x700082e3d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e3f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1828)) else { return RegfileExit::fault(0x700082e3f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e414(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1830)) else { return RegfileExit::fault(0x700082e414); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e434(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1838)) else { return RegfileExit::fault(0x700082e434); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e454(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1840)) else { return RegfileExit::fault(0x700082e454); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e474(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1848)) else { return RegfileExit::fault(0x700082e474); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e494(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1850)) else { return RegfileExit::fault(0x700082e494); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e4b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1858)) else { return RegfileExit::fault(0x700082e4b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e4d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1860)) else { return RegfileExit::fault(0x700082e4d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e4f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1868)) else { return RegfileExit::fault(0x700082e4f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e514(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1870)) else { return RegfileExit::fault(0x700082e514); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e534(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1878)) else { return RegfileExit::fault(0x700082e534); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e554(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1880)) else { return RegfileExit::fault(0x700082e554); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e574(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1888)) else { return RegfileExit::fault(0x700082e574); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e594(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1890)) else { return RegfileExit::fault(0x700082e594); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e5b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1898)) else { return RegfileExit::fault(0x700082e5b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e5d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18a0)) else { return RegfileExit::fault(0x700082e5d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e5f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18a8)) else { return RegfileExit::fault(0x700082e5f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e614(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18b0)) else { return RegfileExit::fault(0x700082e614); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082e634(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082e634); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082e638); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e639); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082e63a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082e63f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082e645); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082e652); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082e658); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082e664); };
            c.w32(7, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2d78)) else { return RegfileExit::fault(0x700082e667); };
            c.w64(9, t7);
            let Some(t8) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082e66e); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082e673); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082e679); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082e680); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x700082e687);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082e68a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082e691); }
            return c.ret(0x0, 0x700082e693);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082e694); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082e698); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082e6a6, 0x700082e6a1);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082e6a6); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        10 => {
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082e6b7); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x6u64);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082e6c5); };
            c.w64(0, t16);
            c.w64(10, 0x700082e6d3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082e6d3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082e6d7); }
            return c.call(c.r64(0), 0x700082e6dd, 0x700082e6db);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082e6dd); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082e6e5); };
            c.w64(4, t17);
            blk = 5; continue;
        }
        12 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e6f1); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082e6f2); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082e703, 0x700082e6fe);
        }
        13 => {
            return RegfileExit::fault(0x700082e703);
        }
        14 => {
            return RegfileExit::jump(0x700082e708);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082e714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082e714); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082e718); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e719); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082e71a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082e71f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082e725); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082e732); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082e738); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082e744); };
            c.w32(7, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2d78)) else { return RegfileExit::fault(0x700082e747); };
            c.w64(9, t7);
            let Some(t8) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082e74e); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082e753); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082e759); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082e760); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x700082e767);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082e76a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082e771); }
            return c.ret(0x0, 0x700082e773);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082e774); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082e778); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082e786, 0x700082e781);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082e786); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        10 => {
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082e797); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x8u64);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082e7a5); };
            c.w64(0, t16);
            c.w64(10, 0x700082e7b3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082e7b3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082e7b7); }
            return c.call(c.r64(0), 0x700082e7bd, 0x700082e7bb);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082e7bd); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082e7c5); };
            c.w64(4, t17);
            blk = 5; continue;
        }
        12 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e7d1); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082e7d2); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082e7e3, 0x700082e7de);
        }
        13 => {
            return RegfileExit::fault(0x700082e7e3);
        }
        14 => {
            return RegfileExit::jump(0x700082e7e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082e7f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082e7f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082e7f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e7f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082e7fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082e7ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082e805); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082e812); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082e818); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082e824); };
            c.w32(7, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2d78)) else { return RegfileExit::fault(0x700082e827); };
            c.w64(9, t7);
            let Some(t8) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082e82e); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082e833); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082e839); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082e840); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x700082e847);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082e84a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082e851); }
            return c.ret(0x0, 0x700082e853);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082e854); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082e858); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082e866, 0x700082e861);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082e866); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        10 => {
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082e877); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x2u64);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082e885); };
            c.w64(0, t16);
            c.w64(10, 0x700082e893u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082e893); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082e897); }
            return c.call(c.r64(0), 0x700082e89d, 0x700082e89b);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082e89d); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082e8a5); };
            c.w64(4, t17);
            blk = 5; continue;
        }
        12 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e8b1); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082e8b2); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082e8c3, 0x700082e8be);
        }
        13 => {
            return RegfileExit::fault(0x700082e8c3);
        }
        14 => {
            return RegfileExit::jump(0x700082e8c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082e8d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082e8d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082e8d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e8d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082e8da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082e8df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082e8e5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082e8f2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082e8f8); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082e904); };
            c.w32(7, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2d78)) else { return RegfileExit::fault(0x700082e907); };
            c.w64(9, t7);
            let Some(t8) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082e90e); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082e913); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082e919); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082e920); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x700082e927);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082e92a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082e931); }
            return c.ret(0x0, 0x700082e933);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082e934); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082e938); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082e946, 0x700082e941);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082e946); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        10 => {
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082e957); }
            c.w64(1, c.r64(13));
            c.w32(8, 0xau64);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082e965); };
            c.w64(0, t16);
            c.w64(10, 0x700082e973u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082e973); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082e977); }
            return c.call(c.r64(0), 0x700082e97d, 0x700082e97b);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082e97d); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082e985); };
            c.w64(4, t17);
            blk = 5; continue;
        }
        12 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e991); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082e992); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082e9a3, 0x700082e99e);
        }
        13 => {
            return RegfileExit::fault(0x700082e9a3);
        }
        14 => {
            return RegfileExit::jump(0x700082e9a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082e9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082e9b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082e9b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082e9b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082e9ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082e9bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082e9c5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082e9d2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082e9d8); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082e9eb); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x10u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082e9f9); };
            c.w64(0, t8);
            c.w64(10, 0x700082ea07u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082ea07); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082ea0b); }
            return c.call(c.r64(0), 0x700082ea11, 0x700082ea0f);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082ea11); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082ea19); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ea1e); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082ea25); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082ea2c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082ea2f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ea36); }
            return c.ret(0x0, 0x700082ea38);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ea39); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ea3d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082ea4b, 0x700082ea46);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ea4b); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ea59); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082ea5a); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082ea6b, 0x700082ea66);
        }
        10 => {
            return RegfileExit::fault(0x700082ea6b);
        }
        11 => {
            return RegfileExit::jump(0x700082ea70);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082ea74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082ea74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082ea78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ea79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ea7a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082ea7f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082ea85); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ea92); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ea98); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082eaa4); };
            c.w32(7, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2d78)) else { return RegfileExit::fault(0x700082eaa7); };
            c.w64(9, t7);
            let Some(t8) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082eaae); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x23)) else { return RegfileExit::fault(0x700082eab3); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082eab9); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082eac0); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x700082eac7);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082eaca); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ead1); }
            return c.ret(0x0, 0x700082ead3);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ead4); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ead8); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082eae6, 0x700082eae1);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082eae6); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        10 => {
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082eaf7); }
            c.w64(1, c.r64(13));
            c.w32(8, 0xcu64);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082eb05); };
            c.w64(0, t16);
            c.w64(10, 0x700082eb13u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082eb13); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082eb17); }
            return c.call(c.r64(0), 0x700082eb1d, 0x700082eb1b);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082eb1d); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082eb25); };
            c.w64(4, t17);
            blk = 5; continue;
        }
        12 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082eb31); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082eb32); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082eb43, 0x700082eb3e);
        }
        13 => {
            return RegfileExit::fault(0x700082eb43);
        }
        14 => {
            return RegfileExit::jump(0x700082eb48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082eb54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082eb54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082eb58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082eb59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082eb5a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082eb5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082eb65); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082eb72); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082eb78); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082eb84); };
            c.w32(7, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2d78)) else { return RegfileExit::fault(0x700082eb87); };
            c.w64(9, t7);
            let Some(t8) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082eb8e); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082eb93); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082eb99); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082eba0); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x700082eba7);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082ebaa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ebb1); }
            return c.ret(0x0, 0x700082ebb3);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ebb4); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ebb8); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082ebc6, 0x700082ebc1);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ebc6); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        10 => {
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082ebd7); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x4u64);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082ebe5); };
            c.w64(0, t16);
            c.w64(10, 0x700082ebf3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082ebf3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082ebf7); }
            return c.call(c.r64(0), 0x700082ebfd, 0x700082ebfb);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082ebfd); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082ec05); };
            c.w64(4, t17);
            blk = 5; continue;
        }
        12 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ec11); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082ec12); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082ec23, 0x700082ec1e);
        }
        13 => {
            return RegfileExit::fault(0x700082ec23);
        }
        14 => {
            return RegfileExit::jump(0x700082ec28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082ec34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082ec34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082ec38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ec39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ec3a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082ec3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082ec45); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ec52); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ec58); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700082ec64); };
            c.w32(7, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2d78)) else { return RegfileExit::fault(0x700082ec67); };
            c.w64(9, t7);
            let Some(t8) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700082ec6e); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700082ec73); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ec79); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082ec80); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x700082ec87);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082ec8a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ec91); }
            return c.ret(0x0, 0x700082ec93);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ec94); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ec98); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082eca6, 0x700082eca1);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082eca6); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        10 => {
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082ecb7); }
            c.w64(1, c.r64(13));
            c.w32(8, 0xeu64);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082ecc5); };
            c.w64(0, t16);
            c.w64(10, 0x700082ecd3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082ecd3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082ecd7); }
            return c.call(c.r64(0), 0x700082ecdd, 0x700082ecdb);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082ecdd); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082ece5); };
            c.w64(4, t17);
            blk = 5; continue;
        }
        12 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ecf1); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082ecf2); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082ed03, 0x700082ecfe);
        }
        13 => {
            return RegfileExit::fault(0x700082ed03);
        }
        14 => {
            return RegfileExit::jump(0x700082ed08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082ed14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082ed14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082ed18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ed19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ed1a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082ed1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082ed25); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ed2e); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ed34); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082ed3c); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ed42); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082ed49); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x8, 0x700082ed50);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082ed53); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ed5a); }
            return c.ret(0x0, 0x700082ed5c);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ed5d); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ed61); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082ed6f, 0x700082ed6a);
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ed6f); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        8 => {
            c.w32(2, 0xc4u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082ed7a); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082ed7b); };
            c.w64(3, t11);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082ed8c, 0x700082ed87);
        }
        9 => {
            return RegfileExit::fault(0x700082ed8c);
        }
        10 => {
            return RegfileExit::jump(0x700082ed90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082ed94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082ed94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082ed98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ed99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ed9a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082ed9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082eda5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082edb2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082edb8); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082edcb); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x2au64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082edd9); };
            c.w64(0, t8);
            c.w64(10, 0x700082ede7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082ede7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082edeb); }
            return c.call(c.r64(0), 0x700082edf1, 0x700082edef);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082edf1); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082edf9); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082edfe); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082ee05); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082ee0c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082ee0f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ee16); }
            return c.ret(0x0, 0x700082ee18);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ee19); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ee1d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082ee2b, 0x700082ee26);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ee2b); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ee39); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082ee3a); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082ee4b, 0x700082ee46);
        }
        10 => {
            return RegfileExit::fault(0x700082ee4b);
        }
        11 => {
            return RegfileExit::jump(0x700082ee50);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082ee54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082ee54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082ee58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ee59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ee5a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082ee5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082ee65); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ee72); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ee78); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082ee8b); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x1au64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082ee99); };
            c.w64(0, t8);
            c.w64(10, 0x700082eea7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082eea7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082eeab); }
            return c.call(c.r64(0), 0x700082eeb1, 0x700082eeaf);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082eeb1); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082eeb9); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082eebe); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082eec5); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082eecc);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082eecf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082eed6); }
            return c.ret(0x0, 0x700082eed8);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082eed9); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082eedd); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082eeeb, 0x700082eee6);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082eeeb); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082eef9); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082eefa); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082ef0b, 0x700082ef06);
        }
        10 => {
            return RegfileExit::fault(0x700082ef0b);
        }
        11 => {
            return RegfileExit::jump(0x700082ef10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082ef14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082ef14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082ef18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ef19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ef1a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082ef1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082ef25); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ef32); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ef38); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082ef4b); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x1cu64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082ef59); };
            c.w64(0, t8);
            c.w64(10, 0x700082ef67u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082ef67); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082ef6b); }
            return c.call(c.r64(0), 0x700082ef71, 0x700082ef6f);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082ef71); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082ef79); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082ef7e); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082ef85); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082ef8c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082ef8f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ef96); }
            return c.ret(0x0, 0x700082ef98);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082ef99); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082ef9d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082efab, 0x700082efa6);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082efab); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082efb9); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082efba); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082efcb, 0x700082efc6);
        }
        10 => {
            return RegfileExit::fault(0x700082efcb);
        }
        11 => {
            return RegfileExit::jump(0x700082efd0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082efd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082efd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082efd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082efd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082efda); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082efdf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082efe5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082eff2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082eff8); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082f00b); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x16u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082f019); };
            c.w64(0, t8);
            c.w64(10, 0x700082f027u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082f027); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082f02b); }
            return c.call(c.r64(0), 0x700082f031, 0x700082f02f);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082f031); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082f039); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f03e); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f045); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082f04c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f04f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f056); }
            return c.ret(0x0, 0x700082f058);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f059); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f05d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f06b, 0x700082f066);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f06b); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f079); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f07a); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082f08b, 0x700082f086);
        }
        10 => {
            return RegfileExit::fault(0x700082f08b);
        }
        11 => {
            return RegfileExit::jump(0x700082f090);
        }
            _ => unreachable!(),
        }
    }
}
