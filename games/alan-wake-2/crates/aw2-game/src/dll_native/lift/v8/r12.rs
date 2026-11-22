#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000851014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085103a); };
            c.w32(7, t0);
            let r1 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851040); };
            c.cmp(t2, c.r32(7), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851057); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 70; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000851072); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851079); };
            c.w32(11, t6);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_ae() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000851086); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(7, 0x80u64);
            blk = 61; continue;
        }
        12 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_b() { blk = 35; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008510c5); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(7, 0x210u64);
            blk = 61; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70008510dc); };
            c.cmp(t9, c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(7, 0x210u64);
            blk = 61; continue;
        }
        19 => {
            c.w32(7, 0x200u64);
            blk = 61; continue;
        }
        20 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000851110); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851116); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 35; continue;
        }
        22 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085112c); };
            c.w32(7, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085112f); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700085113c); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851141); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(7, 0x6u64);
            blk = 61; continue;
        }
        27 => {
            c.w32(7, 0x6u64);
            blk = 64; continue;
        }
        28 => {
            let r15 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r15);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851164); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(7, 0x6u64);
            blk = 61; continue;
        }
        31 => {
            c.w32(7, 0x6u64);
            blk = 64; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t16) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000851193); };
            c.w32(7, t16);
            let r17 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851199); };
            c.cmp(t18, c.r32(7), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t19) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008511a2); };
            c.cmp(t19, 0x43u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(7, 0x3feu64);
            blk = 61; continue;
        }
        36 => {
            c.w32(7, 0x1eu64);
            blk = 61; continue;
        }
        37 => {
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008511bf); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(7, 0x6u64);
            blk = 61; continue;
        }
        40 => {
            c.w32(7, 0x6u64);
            blk = 64; continue;
        }
        41 => {
            c.w32(7, 0x2u64);
            c.w64(8, c.r64(7));
            blk = 61; continue;
        }
        42 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008511fe); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851204); };
            c.cmp(t23, c.r32(7), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t24) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851211); };
            c.w32(8, t24);
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
            c.w32(7, 0x80u64);
            blk = 64; continue;
        }
        49 => {
            c.w32(7, 0x100u64);
            blk = 64; continue;
        }
        50 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000851244); };
            c.cmp(t25, c.r32(7), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(7, 0x210u64);
            blk = 64; continue;
        }
        52 => {
            c.w32(7, 0x8u64);
            blk = 64; continue;
        }
        53 => {
            c.w32(7, 0x200u64);
            blk = 64; continue;
        }
        54 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(7, 0x60u64);
            blk = 57; continue;
        }
        56 => {
            c.w32(7, 0x20u64);
            blk = 57; continue;
        }
        57 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 64; continue;
        }
        58 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851279); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            if c.cc_e() { blk = 62; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(7, 0x6u64);
            blk = 61; continue;
        }
        61 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085128b); };
            c.w64(0, t26);
            blk = 65; continue;
        }
        62 => {
            c.w32(7, 0x6u64);
            blk = 64; continue;
        }
        63 => {
            c.w32(7, 0x2u64);
            blk = 64; continue;
        }
        64 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008512a0); };
            c.w64(0, t27);
            blk = 65; continue;
        }
        65 => {
            let Some(t28) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008512a7); };
            c.w32(8, t28);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r29 = c.r32(7) | c.r32(9);
            c.w32(7, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008512bc); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008512c0); }
            blk = 67; continue;
        }
        67 => {
            return c.ret(0x0, 0x70008512c7);
        }
        68 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008512c8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008512cc); }
            let r30 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r30);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008512d2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008512d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008512d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008512d8); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008512dc); };
            c.w64(3, t31);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008512ef, 0x70008512ea);
        }
        69 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008512ef); };
            c.w64(1, t32);
            c.w32(7, 0x100u64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008512f8); };
            c.w64(2, t33);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008512ff); }
            blk = 65; continue;
        }
        70 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851302); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851306); }
            let r34 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085130c); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851312); };
            c.w32(7, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851316); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700085131a); }
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851327); }
            blk = 35; continue;
        }
        72 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(8, 0x60u64);
            blk = 75; continue;
        }
        74 => {
            c.w32(8, 0x20u64);
            blk = 75; continue;
        }
        75 => {
            c.logic(c.r8(7) & 0x20u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(7, 0x60u64);
            blk = 78; continue;
        }
        77 => {
            c.w32(7, 0x20u64);
            blk = 78; continue;
        }
        78 => {
            c.w32(7, c.r32(7));
            c.w32(8, c.r32(8));
            let r38 = c.r32(7) | c.r32(8);
            c.w32(7, r38);
            c.logic(r38, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000851360); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb534, 0x700085136f, 0x700085136a);
        }
        79 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085136f); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851373); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851377); };
            c.w64(2, t41);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085137e); }
            blk = 65; continue;
        }
        80 => {
            return RegfileExit::jump(0x7000851388);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000851394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(9));
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085139f); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008513a3); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008513a7); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008513ab); };
            c.w64(6, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008513b2); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(9, 0x3feu64);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008513ca); };
            c.w32(12, t4);
            let r5 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008513d1); };
            c.cmp(t6, c.r32(12), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008513e6); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000851412); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851419); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851427); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700085142c); }
            blk = 15; continue;
        }
        13 => {
            let r12 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r12);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851435); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r13 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r13);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851446); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(8));
            let r14 = c.r32(7) | 0x6u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(8));
            let r15 = c.r32(7) | 0x2u64;
            c.w32(7, r15);
            c.logic(r15, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000851471); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        19 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085147a); };
            c.w64(0, t17);
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000851481); };
            c.w32(7, t18);
            c.w32(8, c.r32(9));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000851496); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700085149a); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x70008514a1);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008514a2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008514a6); }
            let r20 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008514ac); };
            c.w32(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008514b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008514b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008514ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008514be); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008514c2); }
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008514e3); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008514ec); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(8, c.r64(9));
            blk = 30; continue;
        }
        29 => {
            c.w32(8, c.r32(8));
            let r24 = c.r32(8) | 0x1eu64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000851503); }
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x7000851517, 0x7000851512);
        }
        32 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000851517); };
            c.w64(6, t25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085151b); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085151f); };
            c.w64(0, t26);
            return c.call(0x70008a10d4, 0x7000851528, 0x7000851523);
        }
        33 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000851528); };
            c.w64(9, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085152c); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851530); };
            c.w64(0, t28);
            blk = 1; continue;
        }
        34 => {
            return c.call(0x70008a2474, 0x700085153e, 0x7000851539);
        }
        35 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085153e); };
            c.w64(9, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851542); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851546); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        36 => {
            let Some(t31) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085154f); };
            c.w32(11, t31);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            return c.call(0x70008a10d4, 0x7000851577, 0x7000851572);
        }
        40 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000851577); };
            c.w64(6, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085157b); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085157f); };
            c.w64(0, t33);
            return c.call(0x70008a11d4, 0x7000851588, 0x7000851583);
        }
        41 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x700085159b, 0x7000851596);
        }
        43 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008515a6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008515b3); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008515d6, 0x70008515d1);
        }
        48 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008515e1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008515e5); }
            let r36 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r36);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008515eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008515ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008515f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008515f7); }
            let Some(t37) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008515fb); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700085160f, 0x700085160a);
        }
        51 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085161a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085161e); }
            let r38 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851624); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851628); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700085162c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851630); }
            let Some(t39) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851634); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            return c.call(0x70008a10d4, 0x7000851646, 0x7000851641);
        }
        54 => {
            c.w32(9, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085164c); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851650); };
            c.w64(0, t40);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851659); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085165d); }
            let r41 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r41);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851663); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851667); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085166b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700085166f); }
            let Some(t42) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851673); };
            c.cmp(t42, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000851687, 0x7000851682);
        }
        57 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851698); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700085169a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085169b); }
            c.w64(6, c.r64(7));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700085169f); };
            c.w64(3, t43);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008516b3, 0x70008516ae);
        }
        59 => {
            c.w32(9, 0x3feu64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008516b9); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008516bd); };
            c.w64(2, t45);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008516c4); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(8));
            let r46 = c.r32(7) | 0x60u64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008516d3); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb9f4, 0x70008516e2, 0x70008516dd);
        }
        61 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008516e2); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008516e6); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008516ea); };
            c.w64(2, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008516f1); }
            blk = 20; continue;
        }
        62 => {
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008516fd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008516ff); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851700); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000851701); };
            c.w64(3, t50);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851715, 0x7000851710);
        }
        63 => {
            c.w32(9, 0x3feu64);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085171b); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085171f); };
            c.w64(2, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851726); }
            blk = 20; continue;
        }
        64 => {
            c.w32(8, c.r32(8));
            let r53 = c.r32(8) | 0x100u64;
            c.w32(8, r53);
            c.logic(r53, 32);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085173f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851741); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851742); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000851743); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000851747); };
            c.w64(3, t54);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700085175b, 0x7000851756);
        }
        65 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085175b); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085175f); };
            c.w64(1, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851763); };
            c.w64(2, t57);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085176a); }
            blk = 20; continue;
        }
        66 => {
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851776); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851778); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851779); }
            c.w64(6, c.r64(7));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085177d); };
            c.w64(3, t58);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000851791, 0x700085178c);
        }
        67 => {
            c.w32(9, 0x3feu64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851797); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085179b); };
            c.w64(2, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008517a2); }
            blk = 20; continue;
        }
        68 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008517ae); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008517b0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008517b1); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008517b2); };
            c.w64(3, t61);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008517c6, 0x70008517c1);
        }
        69 => {
            c.w32(9, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008517cc); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008517d0); };
            c.w64(2, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008517d7); }
            blk = 20; continue;
        }
        70 => {
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008517e3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008517e5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008517e6); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008517e7); };
            c.w64(3, t64);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008517fb, 0x70008517f6);
        }
        71 => {
            c.w32(9, 0x3feu64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851801); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851805); };
            c.w64(2, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085180c); }
            blk = 20; continue;
        }
        72 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851818); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700085181a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085181b); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085181c); };
            c.w64(3, t67);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851830, 0x700085182b);
        }
        73 => {
            c.w32(9, 0x3feu64);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851836); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085183a); };
            c.w64(2, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851841); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x7000851848);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000851854(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(9));
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085185f); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851863); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851867); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085186b); };
            c.w64(6, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851872); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(9, 0x3feu64);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085188a); };
            c.w32(12, t4);
            let r5 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851891); };
            c.cmp(t6, c.r32(12), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008518a6); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008518d2); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008518d9); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008518e7); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008518ec); }
            blk = 15; continue;
        }
        13 => {
            let r12 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r12);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008518f5); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r13 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r13);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851906); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(8));
            let r14 = c.r32(7) | 0x6u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(8));
            let r15 = c.r32(7) | 0x2u64;
            c.w32(7, r15);
            c.logic(r15, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000851931); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        19 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085193a); };
            c.w64(0, t17);
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000851941); };
            c.w32(7, t18);
            c.w32(8, c.r32(9));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000851956); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700085195a); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x7000851961);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851962); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851966); }
            let r20 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085196c); };
            c.w32(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851972); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851976); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700085197a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085197e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851982); }
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008519a3); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008519ac); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(8, c.r64(9));
            blk = 30; continue;
        }
        29 => {
            c.w32(8, c.r32(8));
            let r24 = c.r32(8) | 0x1eu64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008519c3); }
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x70008519d7, 0x70008519d2);
        }
        32 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008519d7); };
            c.w64(6, t25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008519db); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008519df); };
            c.w64(0, t26);
            return c.call(0x70008a10d4, 0x70008519e8, 0x70008519e3);
        }
        33 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008519e8); };
            c.w64(9, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008519ec); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008519f0); };
            c.w64(0, t28);
            blk = 1; continue;
        }
        34 => {
            return c.call(0x70008a2474, 0x70008519fe, 0x70008519f9);
        }
        35 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008519fe); };
            c.w64(9, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851a02); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851a06); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        36 => {
            let Some(t31) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851a0f); };
            c.w32(11, t31);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            return c.call(0x70008a10d4, 0x7000851a37, 0x7000851a32);
        }
        40 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000851a37); };
            c.w64(6, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851a3b); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851a3f); };
            c.w64(0, t33);
            return c.call(0x70008a11d4, 0x7000851a48, 0x7000851a43);
        }
        41 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x7000851a5b, 0x7000851a56);
        }
        43 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851a66); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851a73); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000851a96, 0x7000851a91);
        }
        48 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851aa1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851aa5); }
            let r36 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r36);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851aab); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851aaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851ab3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851ab7); }
            let Some(t37) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851abb); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000851acf, 0x7000851aca);
        }
        51 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851ada); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851ade); }
            let r38 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851ae4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851ae8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000851aec); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851af0); }
            let Some(t39) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851af4); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            return c.call(0x70008a10d4, 0x7000851b06, 0x7000851b01);
        }
        54 => {
            c.w32(9, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851b0c); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851b10); };
            c.w64(0, t40);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851b19); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851b1d); }
            let r41 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r41);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851b23); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851b27); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851b2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851b2f); }
            let Some(t42) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851b33); };
            c.cmp(t42, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000851b47, 0x7000851b42);
        }
        57 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851b58); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851b5a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851b5b); }
            c.w64(6, c.r64(7));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000851b5f); };
            c.w64(3, t43);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000851b73, 0x7000851b6e);
        }
        59 => {
            c.w32(9, 0x3feu64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851b79); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851b7d); };
            c.w64(2, t45);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851b84); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(8));
            let r46 = c.r32(7) | 0x60u64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000851b93); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb774, 0x7000851ba2, 0x7000851b9d);
        }
        61 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000851ba2); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851ba6); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851baa); };
            c.w64(2, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851bb1); }
            blk = 20; continue;
        }
        62 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851bbd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851bbf); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851bc0); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000851bc1); };
            c.w64(3, t50);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851bd5, 0x7000851bd0);
        }
        63 => {
            c.w32(9, 0x3feu64);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851bdb); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851bdf); };
            c.w64(2, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851be6); }
            blk = 20; continue;
        }
        64 => {
            c.w32(8, c.r32(8));
            let r53 = c.r32(8) | 0x100u64;
            c.w32(8, r53);
            c.logic(r53, 32);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851bff); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851c01); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851c02); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000851c03); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000851c07); };
            c.w64(3, t54);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851c1b, 0x7000851c16);
        }
        65 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000851c1b); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851c1f); };
            c.w64(1, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851c23); };
            c.w64(2, t57);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851c2a); }
            blk = 20; continue;
        }
        66 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851c36); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851c38); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851c39); }
            c.w64(6, c.r64(7));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000851c3d); };
            c.w64(3, t58);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000851c51, 0x7000851c4c);
        }
        67 => {
            c.w32(9, 0x3feu64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851c57); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851c5b); };
            c.w64(2, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851c62); }
            blk = 20; continue;
        }
        68 => {
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851c6e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851c70); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851c71); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000851c72); };
            c.w64(3, t61);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851c86, 0x7000851c81);
        }
        69 => {
            c.w32(9, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851c8c); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851c90); };
            c.w64(2, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851c97); }
            blk = 20; continue;
        }
        70 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851ca3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851ca5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851ca6); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000851ca7); };
            c.w64(3, t64);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851cbb, 0x7000851cb6);
        }
        71 => {
            c.w32(9, 0x3feu64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851cc1); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851cc5); };
            c.w64(2, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851ccc); }
            blk = 20; continue;
        }
        72 => {
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000851cd8); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000851cda); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000851cdb); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000851cdc); };
            c.w64(3, t67);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851cf0, 0x7000851ceb);
        }
        73 => {
            c.w32(9, 0x3feu64);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851cf6); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851cfa); };
            c.w64(2, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851d01); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x7000851d08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000851d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(9));
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000851d1f); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000851d23); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000851d27); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000851d2b); };
            c.w64(6, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000851d32); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(9, 0x3feu64);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000851d4a); };
            c.w32(12, t4);
            let r5 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851d51); };
            c.cmp(t6, c.r32(12), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000851d66); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000851d92); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851d99); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851da7); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851dac); }
            blk = 15; continue;
        }
        13 => {
            let r12 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r12);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851db5); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r13 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r13);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000851dc6); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(8));
            let r14 = c.r32(7) | 0x6u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(8));
            let r15 = c.r32(7) | 0x2u64;
            c.w32(7, r15);
            c.logic(r15, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000851df1); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        19 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000851dfa); };
            c.w64(0, t17);
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000851e01); };
            c.w32(7, t18);
            c.w32(8, c.r32(9));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000851e16); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000851e1a); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x7000851e21);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851e22); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851e26); }
            let r20 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851e2c); };
            c.w32(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851e32); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851e36); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000851e3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851e3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851e42); }
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851e63); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851e6c); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(8, c.r64(9));
            blk = 30; continue;
        }
        29 => {
            c.w32(8, c.r32(8));
            let r24 = c.r32(8) | 0x1eu64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000851e83); }
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x7000851e97, 0x7000851e92);
        }
        32 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000851e97); };
            c.w64(6, t25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000851e9b); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000851e9f); };
            c.w64(0, t26);
            return c.call(0x70008a10d4, 0x7000851ea8, 0x7000851ea3);
        }
        33 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000851ea8); };
            c.w64(9, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851eac); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851eb0); };
            c.w64(0, t28);
            blk = 1; continue;
        }
        34 => {
            return c.call(0x70008a2474, 0x7000851ebe, 0x7000851eb9);
        }
        35 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000851ebe); };
            c.w64(9, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851ec2); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851ec6); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        36 => {
            let Some(t31) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851ecf); };
            c.w32(11, t31);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            return c.call(0x70008a10d4, 0x7000851ef7, 0x7000851ef2);
        }
        40 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000851ef7); };
            c.w64(6, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851efb); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851eff); };
            c.w64(0, t33);
            return c.call(0x70008a11d4, 0x7000851f08, 0x7000851f03);
        }
        41 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x7000851f1b, 0x7000851f16);
        }
        43 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000851f26); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851f33); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000851f56, 0x7000851f51);
        }
        48 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851f61); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851f65); }
            let r36 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r36);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851f6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851f6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851f73); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851f77); }
            let Some(t37) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851f7b); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000851f8f, 0x7000851f8a);
        }
        51 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851f9a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851f9e); }
            let r38 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851fa4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851fa8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000851fac); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851fb0); }
            let Some(t39) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851fb4); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            return c.call(0x70008a10d4, 0x7000851fc6, 0x7000851fc1);
        }
        54 => {
            c.w32(9, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851fcc); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000851fd0); };
            c.w64(0, t40);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000851fd9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000851fdd); }
            let r41 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r41);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000851fe3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000851fe7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000851feb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000851fef); }
            let Some(t42) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000851ff3); };
            c.cmp(t42, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000852007, 0x7000852002);
        }
        57 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000852018); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700085201a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085201b); }
            c.w64(6, c.r64(7));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700085201f); };
            c.w64(3, t43);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000852033, 0x700085202e);
        }
        59 => {
            c.w32(9, 0x3feu64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852039); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085203d); };
            c.w64(2, t45);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852044); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(8));
            let r46 = c.r32(7) | 0x60u64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000852053); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fbb34, 0x7000852062, 0x700085205d);
        }
        61 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000852062); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852066); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085206a); };
            c.w64(2, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852071); }
            blk = 20; continue;
        }
        62 => {
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085207d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085207f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000852080); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000852081); };
            c.w64(3, t50);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000852095, 0x7000852090);
        }
        63 => {
            c.w32(9, 0x3feu64);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085209b); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085209f); };
            c.w64(2, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008520a6); }
            blk = 20; continue;
        }
        64 => {
            c.w32(8, c.r32(8));
            let r53 = c.r32(8) | 0x100u64;
            c.w32(8, r53);
            c.logic(r53, 32);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008520bf); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008520c1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008520c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008520c3); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x70008520c7); };
            c.w64(3, t54);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008520db, 0x70008520d6);
        }
        65 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008520db); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008520df); };
            c.w64(1, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008520e3); };
            c.w64(2, t57);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008520ea); }
            blk = 20; continue;
        }
        66 => {
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008520f6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008520f8); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008520f9); }
            c.w64(6, c.r64(7));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008520fd); };
            c.w64(3, t58);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000852111, 0x700085210c);
        }
        67 => {
            c.w32(9, 0x3feu64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852117); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085211b); };
            c.w64(2, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852122); }
            blk = 20; continue;
        }
        68 => {
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085212e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000852130); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000852131); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000852132); };
            c.w64(3, t61);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000852146, 0x7000852141);
        }
        69 => {
            c.w32(9, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085214c); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852150); };
            c.w64(2, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852157); }
            blk = 20; continue;
        }
        70 => {
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000852163); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000852165); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000852166); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000852167); };
            c.w64(3, t64);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700085217b, 0x7000852176);
        }
        71 => {
            c.w32(9, 0x3feu64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852181); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852185); };
            c.w64(2, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085218c); }
            blk = 20; continue;
        }
        72 => {
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000852198); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700085219a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085219b); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085219c); };
            c.w64(3, t67);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008521b0, 0x70008521ab);
        }
        73 => {
            c.w32(9, 0x3feu64);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008521b6); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008521ba); };
            c.w64(2, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008521c1); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x70008521c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008521d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(8, c.r64(9));
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008521df); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008521e3); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008521e7); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008521eb); };
            c.w64(6, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008521f2); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(9, 0x3feu64);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085220a); };
            c.w32(12, t4);
            let r5 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852211); };
            c.cmp(t6, c.r32(12), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000852226); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000852252); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852259); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000852267); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700085226c); }
            blk = 15; continue;
        }
        13 => {
            let r12 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r12);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000852275); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r13 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r13);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000852286); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(8));
            let r14 = c.r32(7) | 0x6u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(8));
            let r15 = c.r32(7) | 0x2u64;
            c.w32(7, r15);
            c.logic(r15, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008522b1); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        19 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008522ba); };
            c.w64(0, t17);
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008522c1); };
            c.w32(7, t18);
            c.w32(8, c.r32(9));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008522d6); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008522da); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x70008522e1);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008522e2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008522e6); }
            let r20 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008522ec); };
            c.w32(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008522f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008522f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008522fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008522fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000852302); }
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852323); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085232c); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(8, c.r64(9));
            blk = 30; continue;
        }
        29 => {
            c.w32(8, c.r32(8));
            let r24 = c.r32(8) | 0x1eu64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000852343); }
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x7000852357, 0x7000852352);
        }
        32 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000852357); };
            c.w64(6, t25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085235b); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085235f); };
            c.w64(0, t26);
            return c.call(0x70008a10d4, 0x7000852368, 0x7000852363);
        }
        33 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000852368); };
            c.w64(9, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085236c); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852370); };
            c.w64(0, t28);
            blk = 1; continue;
        }
        34 => {
            return c.call(0x70008a2474, 0x700085237e, 0x7000852379);
        }
        35 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085237e); };
            c.w64(9, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000852382); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852386); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        36 => {
            let Some(t31) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085238f); };
            c.w32(11, t31);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            return c.call(0x70008a10d4, 0x70008523b7, 0x70008523b2);
        }
        40 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008523b7); };
            c.w64(6, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008523bb); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008523bf); };
            c.w64(0, t33);
            return c.call(0x70008a11d4, 0x70008523c8, 0x70008523c3);
        }
        41 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x70008523db, 0x70008523d6);
        }
        43 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008523e6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008523f3); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x7000852416, 0x7000852411);
        }
        48 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852421); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852425); }
            let r36 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r36);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700085242b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700085242f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000852433); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000852437); }
            let Some(t37) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085243b); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700085244f, 0x700085244a);
        }
        51 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085245a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085245e); }
            let r38 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000852464); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000852468); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700085246c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000852470); }
            let Some(t39) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000852474); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            return c.call(0x70008a10d4, 0x7000852486, 0x7000852481);
        }
        54 => {
            c.w32(9, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085248c); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852490); };
            c.w64(0, t40);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852499); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085249d); }
            let r41 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r41);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008524a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008524a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008524ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008524af); }
            let Some(t42) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008524b3); };
            c.cmp(t42, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008524c7, 0x70008524c2);
        }
        57 => {
            c.w32(9, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008524d8); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008524da); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008524db); }
            c.w64(6, c.r64(7));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008524df); };
            c.w64(3, t43);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008524f3, 0x70008524ee);
        }
        59 => {
            c.w32(9, 0x3feu64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008524f9); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008524fd); };
            c.w64(2, t45);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852504); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(8));
            let r46 = c.r32(7) | 0x60u64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000852513); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb8b4, 0x7000852522, 0x700085251d);
        }
        61 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000852522); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852526); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085252a); };
            c.w64(2, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852531); }
            blk = 20; continue;
        }
        62 => {
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085253d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085253f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000852540); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000852541); };
            c.w64(3, t50);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000852555, 0x7000852550);
        }
        63 => {
            c.w32(9, 0x3feu64);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085255b); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085255f); };
            c.w64(2, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852566); }
            blk = 20; continue;
        }
        64 => {
            c.w32(8, c.r32(8));
            let r53 = c.r32(8) | 0x100u64;
            c.w32(8, r53);
            c.logic(r53, 32);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085257f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000852581); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000852582); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000852583); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x7000852587); };
            c.w64(3, t54);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700085259b, 0x7000852596);
        }
        65 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085259b); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085259f); };
            c.w64(1, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008525a3); };
            c.w64(2, t57);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008525aa); }
            blk = 20; continue;
        }
        66 => {
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008525b6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008525b8); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008525b9); }
            c.w64(6, c.r64(7));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008525bd); };
            c.w64(3, t58);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008525d1, 0x70008525cc);
        }
        67 => {
            c.w32(9, 0x3feu64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008525d7); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008525db); };
            c.w64(2, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008525e2); }
            blk = 20; continue;
        }
        68 => {
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008525ee); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008525f0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008525f1); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008525f2); };
            c.w64(3, t61);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000852606, 0x7000852601);
        }
        69 => {
            c.w32(9, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085260c); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852610); };
            c.w64(2, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852617); }
            blk = 20; continue;
        }
        70 => {
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000852623); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000852625); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000852626); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x7000852627); };
            c.w64(3, t64);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700085263b, 0x7000852636);
        }
        71 => {
            c.w32(9, 0x3feu64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852641); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852645); };
            c.w64(2, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085264c); }
            blk = 20; continue;
        }
        72 => {
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000852658); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700085265a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085265b); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085265c); };
            c.w64(3, t67);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000852670, 0x700085266b);
        }
        73 => {
            c.w32(9, 0x3feu64);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852676); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085267a); };
            c.w64(2, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852681); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x7000852688);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000852694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852694); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852698); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700085269e); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008526a2); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008526a6); };
            c.w64(1, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008526aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008526ae); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008526b5); }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008526bd); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008526c4); };
            c.cmp(t6, c.r32(9), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008526cd); };
            c.w32(8, t7);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x70008526ec, 0x70008526e7);
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008526ec); };
            c.w64(7, t8);
            c.w32(8, 0xfeu64);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008526f6); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008526fa); };
            c.w64(3, t10);
            blk = 1; continue;
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000852700); };
            c.w32(0, t11);
            let r12 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r12);
            c.w32(8, 0x1eu64);
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(8));
            let r13 = c.r32(7) | 0x2u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            let r14 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r14);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 11; continue;
        }
        9 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700085271b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 29; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(0, c.r32(0));
            c.w32(7, c.r32(8));
            let r15 = c.r32(7) | 0xeu64;
            c.w32(7, r15);
            c.logic(r15, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 11; continue;
        }
        11 => {
            let r16 = !c.r32(0);
            c.w32(0, r16);
            c.w64(8, c.r64(0));
            let r17 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r17);
            if c.cc_o() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(9, 0xeu64);
            blk = 16; continue;
        }
        15 => {
            c.w32(9, 0x2u64);
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(3));
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000852762); };
            c.w32(12, t18);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(9));
            c.w32(15, c.r32(12));
            let r19 = c.r32(7) | c.r32(9);
            c.w32(7, r19);
            c.logic(r19, 32);
            let r20 = c.r32(7) | c.r32(15);
            c.w32(7, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700085277d); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000852782); }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, c.r64(8));
            blk = 19; continue;
        }
        19 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085278f); }
            return c.ret(0x0, 0x7000852790);
        }
        20 => {
            c.w64(9, c.r64(3));
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000852794); };
            c.w32(8, t21);
            c.w32(11, c.r32(8));
            let r22 = c.r32(11) | 0x40u64;
            c.w32(11, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008527a8); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008527ad); }
            blk = 22; continue;
        }
        22 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008527b4); }
            c.w32(1, 0x18u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008527ba); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008527bb); };
            c.w64(3, t23);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008527cf, 0x70008527ca);
        }
        23 => {
            blk = 19; continue;
        }
        24 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008527d1); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008527d8); };
            c.w64(9, t25);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008527df); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008527e3); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008527e7); };
            c.w64(12, t26);
            let Some(t27) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008527ee); };
            c.cmp(t27, c.r64(11), 64);
            if c.cc_a() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008527fe, 0x70008527f9);
        }
        26 => {
            c.w64(8, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852801); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852805); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852809); };
            c.w64(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085280d); };
            c.w64(0, t31);
            blk = 28; continue;
        }
        27 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000852813); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085281a); };
            c.w64(9, t32);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000852821); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085282d); }
            blk = 13; continue;
        }
        29 => {
            let r33 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r33);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085283c); }
            return c.call(0x70008708b4, 0x7000852846, 0x7000852841);
        }
        30 => {
            let Some(t34) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000852846); };
            c.w32(0, t34);
            let r35 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r35);
            blk = 10; continue;
        }
        31 => {
            return RegfileExit::jump(0x7000852854);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000852874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000852877); };
            c.w64(7, t0);
            let Some(t1) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085287b); };
            c.w64(1, t1);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 1; continue;
        }
        1 => {
            c.w32(9, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000852889); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852890); };
            c.cmp(t4, c.r32(9), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000852899); };
            c.w32(9, t5);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008528b4); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            c.w32(8, 0x1eu64);
            blk = 1; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008528c2); }
            blk = 10; continue;
        }
        7 => {
            let r8 = c.add(c.r32(9), 0xfffffffffffffffeu64, 32);
            c.w32(9, r8);
            if c.cc_o() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, c.r32(8));
            let r9 = c.r32(7) | 0x2u64;
            c.w32(7, r9);
            c.logic(r9, 32);
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        9 => {
            let r10 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r10);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008528e7); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008528ee); };
            c.w64(9, t12);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            c.w32(8, c.r32(8));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008528f8); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008528ff); };
            c.cmp(t14, c.r64(11), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x7000852909); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 12; continue;
        }
        12 => {
            c.w64(10, 0xbff0000000000000u64);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852923); };
            c.w64(9, t15);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700085292a); }
            let r16 = c.r32(8) | 0xeu64;
            c.w32(8, r16);
            c.logic(r16, 32);
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000852932); }
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 13; continue;
        }
        13 => {
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085293d); };
            c.w32(7, t17);
            c.w32(8, c.r32(8));
            c.w32(9, c.r32(7));
            let r18 = c.r32(8) | c.r32(9);
            c.w32(8, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000852952); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000852956); }
            blk = 15; continue;
        }
        15 => {
            return c.ret(0x0, 0x700085295d);
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085295e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852962); }
            let r19 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r19);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852968); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700085296c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000852970); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700085297c, 0x7000852977);
        }
        17 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085297c); };
            c.w64(7, t20);
            c.w32(8, 0xfeu64);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852986); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085298a); };
            c.w64(3, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852991); }
            blk = 1; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852997); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085299b); }
            let r23 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r23);
            c.w32(8, c.r32(8));
            let r24 = c.r32(8) | 0x40u64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008529ab); }
            c.w32(9, 0x1eu64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008529b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008529b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008529b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008529bc); }
            c.w64(6, c.r64(7));
            c.w32(0, 0x2u64);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008529c8); };
            c.w64(3, t25);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x70008529d7, 0x70008529d2);
        }
        19 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008529d7); };
            c.w64(8, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008529db); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008529df); };
            c.w64(1, t28);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008529e6); }
            blk = 13; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008529ec); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008529f0); }
            let r29 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r29);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008529fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008529ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000852a03); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000852a07); }
            return c.call(0x70007febf4, 0x7000852a11, 0x7000852a0c);
        }
        21 => {
            c.w64(7, c.r64(0));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852a14); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852a18); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852a1c); };
            c.w64(8, t32);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000852a20); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852a28); }
            blk = 12; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000852a30);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000852a34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000852a37); };
            c.w64(7, t0);
            let Some(t1) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000852a3b); };
            c.w64(1, t1);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 1; continue;
        }
        1 => {
            c.w32(9, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000852a49); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852a50); };
            c.cmp(t4, c.r32(9), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000852a59); };
            c.w32(9, t5);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000852a74); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            c.w32(8, 0x1eu64);
            blk = 1; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000852a82); }
            blk = 10; continue;
        }
        7 => {
            let r8 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r8);
            if c.cc_o() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, c.r32(8));
            let r9 = c.r32(7) | 0x2u64;
            c.w32(7, r9);
            c.logic(r9, 32);
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        9 => {
            let r10 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r10);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000852aa7); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000852aae); };
            c.w64(9, t12);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            c.w32(8, c.r32(8));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000852ab8); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000852abf); };
            c.cmp(t14, c.r64(11), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x7000852ac9); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 12; continue;
        }
        12 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852ae2); };
            c.w64(9, t15);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000852ae9); }
            let r16 = c.r32(8) | 0xeu64;
            c.w32(8, r16);
            c.logic(r16, 32);
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000852af1); }
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 13; continue;
        }
        13 => {
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000852afc); };
            c.w32(7, t17);
            c.w32(8, c.r32(8));
            c.w32(9, c.r32(7));
            let r18 = c.r32(8) | c.r32(9);
            c.w32(8, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000852b11); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000852b15); }
            blk = 15; continue;
        }
        15 => {
            return c.ret(0x0, 0x7000852b1c);
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852b1d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852b21); }
            let r19 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r19);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852b27); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000852b2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000852b2f); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x7000852b3b, 0x7000852b36);
        }
        17 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852b3b); };
            c.w64(7, t20);
            c.w32(8, 0xfeu64);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852b45); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852b49); };
            c.w64(3, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852b50); }
            blk = 1; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852b56); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852b5a); }
            let r23 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r23);
            c.w32(8, c.r32(8));
            let r24 = c.r32(8) | 0x40u64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000852b6a); }
            c.w32(9, 0x1cu64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000852b71); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852b73); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000852b77); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000852b7b); }
            c.w64(6, c.r64(7));
            c.w32(0, 0x2u64);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000852b87); };
            c.w64(3, t25);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x7000852b96, 0x7000852b91);
        }
        19 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000852b96); };
            c.w64(8, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852b9a); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852b9e); };
            c.w64(1, t28);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852ba5); }
            blk = 13; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852bab); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852baf); }
            let r29 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r29);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852bba); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000852bbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000852bc2); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000852bc6); }
            return c.call(0x70007febf4, 0x7000852bd0, 0x7000852bcb);
        }
        21 => {
            c.w64(7, c.r64(0));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852bd3); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852bd7); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852bdb); };
            c.w64(8, t32);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000852bdf); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852be7); }
            blk = 12; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000852bf0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000852bf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000852bf7); };
            c.w64(7, t0);
            let Some(t1) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000852bfb); };
            c.w64(1, t1);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 1; continue;
        }
        1 => {
            c.w32(9, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000852c09); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852c10); };
            c.cmp(t4, c.r32(9), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000852c19); };
            c.w32(9, t5);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000852c34); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            c.w32(8, 0x1eu64);
            blk = 1; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000852c42); }
            blk = 11; continue;
        }
        7 => {
            c.w32(7, c.r32(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(11, 0x80000000u64);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let r8 = c.neg(c.r32(9), 32);
            c.w32(9, r8);
            let r9 = c.r32(7) | 0x2u64;
            c.w32(7, r9);
            c.logic(r9, 32);
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        10 => {
            let r10 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r10);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 11; continue;
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000852c75); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000852c7c); };
            c.w64(9, t12);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            c.w32(8, c.r32(8));
            let r13 = c.r32(8) | 0xeu64;
            c.w32(8, r13);
            c.logic(r13, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000852c8d); };
            c.w64(12, t14);
            let Some(t15) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000852c94); };
            c.cmp(t15, c.r64(11), 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x7000852c9e); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 13; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000852ca5); };
            c.w64(10, t16);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x7000852cac); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852cb0); };
            c.w64(9, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000852cb7); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000852cbb); }
            c.w64(0, c.r64(7));
            blk = 15; continue;
        }
        14 => {
            let r18 = c.r32(7) | 0xeu64;
            c.w32(7, r18);
            c.logic(r18, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x7000852ccb); };
            c.w64(0, t19);
            blk = 15; continue;
        }
        15 => {
            let Some(t20) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000852cd2); };
            c.w32(7, t20);
            c.w32(8, c.r32(8));
            c.w32(9, c.r32(7));
            let r21 = c.r32(8) | c.r32(9);
            c.w32(8, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000852ce7); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000852ceb); }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x0, 0x7000852cf2);
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852cf3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852cf7); }
            let r22 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852cfd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000852d01); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000852d05); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x7000852d11, 0x7000852d0c);
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852d11); };
            c.w64(7, t23);
            c.w32(8, 0xfeu64);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852d1b); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852d1f); };
            c.w64(3, t25);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852d26); }
            blk = 1; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852d2c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852d30); }
            let r26 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r26);
            c.w32(8, c.r32(8));
            let r27 = c.r32(8) | 0x40u64;
            c.w32(8, r27);
            c.logic(r27, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000852d40); }
            c.w32(9, 0x1au64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000852d47); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852d49); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000852d4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000852d51); }
            c.w64(6, c.r64(7));
            c.w32(0, 0x2u64);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000852d5d); };
            c.w64(3, t28);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x7000852d6c, 0x7000852d67);
        }
        21 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000852d6c); };
            c.w64(8, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852d70); };
            c.w64(3, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852d74); };
            c.w64(1, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852d7b); }
            blk = 15; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852d81); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852d85); }
            let r32 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r32);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852d90); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000852d94); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x7000852d98); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000852d9d); }
            return c.call(0x70007febf4, 0x7000852da6, 0x7000852da1);
        }
        23 => {
            c.w64(7, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852da9); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852dad); };
            c.w64(3, t34);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x7000852db1); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000852db6); };
            c.w64(8, t35);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852dbd); }
            blk = 13; continue;
        }
        24 => {
            return RegfileExit::jump(0x7000852dc4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000852dd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000852dd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000852dd8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000852dde); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000852de2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000852de6); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000852df0); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852df7); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000852e00); };
            c.w32(7, t4);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.call(0x70008a10d4, 0x7000852e1a, 0x7000852e15);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852e1a); };
            c.w64(6, t5);
            c.w32(7, 0xfeu64);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852e23); };
            c.w64(1, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852e27); };
            c.w64(3, t7);
            blk = 1; continue;
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000852e2d); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            c.w32(7, 0x1eu64);
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(7));
            let r10 = c.r32(7) | 0x2u64;
            c.w32(7, r10);
            c.logic(r10, 32);
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 11; continue;
        }
        9 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000852e46); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 29; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(0, c.r32(0));
            c.w32(7, c.r32(7));
            let r12 = c.r32(7) | 0xeu64;
            c.w32(7, r12);
            c.logic(r12, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 11; continue;
        }
        11 => {
            let r13 = !c.r32(0);
            c.w32(0, r13);
            c.w64(8, c.r64(0));
            let r14 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r14);
            if c.cc_o() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(9, 0xeu64);
            blk = 16; continue;
        }
        15 => {
            c.w32(9, 0x2u64);
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(3));
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000852e8c); };
            c.w32(12, t15);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(9));
            c.w32(15, c.r32(12));
            let r16 = c.r32(7) | c.r32(9);
            c.w32(7, r16);
            c.logic(r16, 32);
            let r17 = c.r32(7) | c.r32(15);
            c.w32(7, r17);
            c.logic(r17, 32);
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000852ea7); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000852eac); }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, c.r64(8));
            blk = 19; continue;
        }
        19 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000852eb9); }
            return c.ret(0x0, 0x7000852eba);
        }
        20 => {
            c.w64(8, c.r64(3));
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000852ebe); };
            c.w32(7, t18);
            c.w32(9, c.r32(7));
            let r19 = c.r32(9) | 0x40u64;
            c.w32(9, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000852ed2); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000852ed7); }
            blk = 22; continue;
        }
        22 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000852ede); }
            c.w32(1, 0x18u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000852ee4); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000852ee5); };
            c.w64(3, t20);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000852ef6, 0x7000852ef1);
        }
        23 => {
            blk = 19; continue;
        }
        24 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000852ef8); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000852eff); };
            c.w64(9, t22);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000852f06); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000852f0a); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000852f0e); };
            c.w64(12, t23);
            let Some(t24) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000852f15); };
            c.cmp(t24, c.r64(11), 64);
            if c.cc_a() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000852f25, 0x7000852f20);
        }
        26 => {
            c.w64(8, c.r64(0));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000852f28); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000852f2c); };
            c.w64(3, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000852f30); };
            c.w64(7, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000852f34); };
            c.w64(0, t28);
            blk = 28; continue;
        }
        27 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000852f3a); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852f41); };
            c.w64(9, t29);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000852f48); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000852f54); }
            blk = 13; continue;
        }
        29 => {
            let r30 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r30);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000852f63); }
            return c.call(0x70008708b4, 0x7000852f6d, 0x7000852f68);
        }
        30 => {
            let Some(t31) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000852f6d); };
            c.w32(0, t31);
            let r32 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r32);
            blk = 10; continue;
        }
        31 => {
            return RegfileExit::jump(0x7000852f7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000852f94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 1; continue;
        }
        1 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000852f9d); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000852fa4); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000852fad); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000852fc8); };
            c.w32(0, t4);
            let r5 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r5);
            c.w32(7, 0x1eu64);
            blk = 1; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000852fd5); }
            blk = 10; continue;
        }
        7 => {
            let r6 = c.add(c.r32(8), 0xfffffffffffffffeu64, 32);
            c.w32(8, r6);
            if c.cc_o() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, c.r32(7));
            let r7 = c.r32(7) | 0x2u64;
            c.w32(7, r7);
            c.logic(r7, 32);
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        9 => {
            let r8 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r8);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000852ff9); };
            c.w64(8, t9);
            let Some(t10) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000853000); };
            c.w64(9, t10);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            c.w32(7, c.r32(7));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853009); };
            c.w64(12, t11);
            let Some(t12) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000853010); };
            c.cmp(t12, c.r64(11), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700085301a); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 12; continue;
        }
        12 => {
            c.w64(10, 0xbff0000000000000u64);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000853034); };
            c.w64(9, t13);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700085303b); }
            let r14 = c.r32(7) | 0xeu64;
            c.w32(7, r14);
            c.logic(r14, 32);
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000853042); }
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 13; continue;
        }
        13 => {
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085304e); };
            c.w32(8, t15);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r16 = c.r32(7) | c.r32(9);
            c.w32(7, r16);
            c.logic(r16, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000853063); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000853067); }
            blk = 15; continue;
        }
        15 => {
            return c.ret(0x0, 0x700085306e);
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085306f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000853073); }
            let r17 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r17);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000853079); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700085307d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000853081); }
            return c.call(0x70008a10d4, 0x700085308a, 0x7000853085);
        }
        17 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085308a); };
            c.w64(6, t18);
            c.w32(7, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000853093); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000853097); };
            c.w64(3, t20);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085309e); }
            blk = 1; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008530a4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008530a8); }
            let r21 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r21);
            c.w32(7, c.r32(7));
            let r22 = c.r32(7) | 0x40u64;
            c.w32(7, r22);
            c.logic(r22, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008530b6); }
            c.w32(8, 0x1eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008530bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008530bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008530c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008530c7); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008530cb); };
            c.w64(3, t23);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008530df, 0x70008530da);
        }
        19 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008530df); };
            c.w64(7, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008530e3); };
            c.w64(3, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008530e7); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008530ee); }
            blk = 13; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008530f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008530f8); }
            let r27 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r27);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000853103); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000853107); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700085310b); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700085310f); }
            return c.call(0x70007febf4, 0x7000853119, 0x7000853114);
        }
        21 => {
            c.w64(8, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085311c); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000853120); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853124); };
            c.w64(7, t30);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000853128); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000853130); }
            blk = 12; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000853138);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000853154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 1; continue;
        }
        1 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085315d); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000853164); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085316d); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000853188); };
            c.w32(0, t4);
            let r5 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r5);
            c.w32(7, 0x1eu64);
            blk = 1; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000853195); }
            blk = 10; continue;
        }
        7 => {
            let r6 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r6);
            if c.cc_o() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, c.r32(7));
            let r7 = c.r32(7) | 0x2u64;
            c.w32(7, r7);
            c.logic(r7, 32);
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        9 => {
            let r8 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r8);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008531b9); };
            c.w64(8, t9);
            let Some(t10) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008531c0); };
            c.w64(9, t10);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            c.w32(7, c.r32(7));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008531c9); };
            c.w64(12, t11);
            let Some(t12) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008531d0); };
            c.cmp(t12, c.r64(11), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008531da); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 12; continue;
        }
        12 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008531f3); };
            c.w64(9, t13);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008531fa); }
            let r14 = c.r32(7) | 0xeu64;
            c.w32(7, r14);
            c.logic(r14, 32);
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000853201); }
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 13; continue;
        }
        13 => {
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085320d); };
            c.w32(8, t15);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r16 = c.r32(7) | c.r32(9);
            c.w32(7, r16);
            c.logic(r16, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000853222); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000853226); }
            blk = 15; continue;
        }
        15 => {
            return c.ret(0x0, 0x700085322d);
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085322e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000853232); }
            let r17 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r17);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000853238); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700085323c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000853240); }
            return c.call(0x70008a10d4, 0x7000853249, 0x7000853244);
        }
        17 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853249); };
            c.w64(6, t18);
            c.w32(7, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000853252); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000853256); };
            c.w64(3, t20);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085325d); }
            blk = 1; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000853263); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000853267); }
            let r21 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r21);
            c.w32(7, c.r32(7));
            let r22 = c.r32(7) | 0x40u64;
            c.w32(7, r22);
            c.logic(r22, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000853275); }
            c.w32(8, 0x1cu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085327c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700085327e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000853282); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000853286); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x700085328a); };
            c.w64(3, t23);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700085329e, 0x7000853299);
        }
        19 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085329e); };
            c.w64(7, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008532a2); };
            c.w64(3, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008532a6); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008532ad); }
            blk = 13; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008532b3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008532b7); }
            let r27 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r27);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008532c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008532c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008532ca); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008532ce); }
            return c.call(0x70007febf4, 0x70008532d8, 0x70008532d3);
        }
        21 => {
            c.w64(8, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008532db); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008532df); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008532e3); };
            c.w64(7, t30);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008532e7); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008532ef); }
            blk = 12; continue;
        }
        22 => {
            return RegfileExit::jump(0x70008532f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000853314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 1; continue;
        }
        1 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085331d); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000853324); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085332d); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000853348); };
            c.w32(0, t4);
            let r5 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r5);
            c.w32(7, 0x1eu64);
            blk = 1; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000853355); }
            blk = 11; continue;
        }
        7 => {
            c.w32(9, c.r32(7));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(11, 0x80000000u64);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let r6 = c.neg(c.r32(8), 32);
            c.w32(8, r6);
            let r7 = c.r32(9) | 0x2u64;
            c.w32(9, r7);
            c.logic(r7, 32);
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        10 => {
            let r8 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r8);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853389); };
            c.w64(8, t9);
            let Some(t10) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000853390); };
            c.w64(9, t10);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            c.w32(7, c.r32(7));
            let r11 = c.r32(7) | 0xeu64;
            c.w32(7, r11);
            c.logic(r11, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085339f); };
            c.w64(12, t12);
            let Some(t13) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008533a6); };
            c.cmp(t13, c.r64(11), 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008533b0); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 13; continue;
        }
        13 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008533b7); };
            c.w64(10, t14);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008533be); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008533c2); };
            c.w64(9, t15);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008533c9); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008533cd); }
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        14 => {
            let r16 = c.r32(9) | 0xeu64;
            c.w32(9, r16);
            c.logic(r16, 32);
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008533df); };
            c.w64(0, t17);
            blk = 15; continue;
        }
        15 => {
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008533e6); };
            c.w32(8, t18);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r19 = c.r32(7) | c.r32(9);
            c.w32(7, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008533fb); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008533ff); }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x0, 0x7000853406);
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000853407); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085340b); }
            let r20 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r20);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000853411); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000853415); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000853419); }
            return c.call(0x70008a10d4, 0x7000853422, 0x700085341d);
        }
        19 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853422); };
            c.w64(6, t21);
            c.w32(7, 0xfeu64);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085342b); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085342f); };
            c.w64(3, t23);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000853436); }
            blk = 1; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085343c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000853440); }
            let r24 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r24);
            c.w32(7, c.r32(7));
            let r25 = c.r32(7) | 0x40u64;
            c.w32(7, r25);
            c.logic(r25, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085344e); }
            c.w32(8, 0x1au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000853455); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000853457); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700085345b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085345f); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000853463); };
            c.w64(3, t26);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000853477, 0x7000853472);
        }
        21 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000853477); };
            c.w64(7, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085347b); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085347f); };
            c.w64(1, t29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000853486); }
            blk = 15; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085348c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000853490); }
            let r30 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r30);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700085349b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700085349f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008534a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008534a8); }
            return c.call(0x70007febf4, 0x70008534b1, 0x70008534ac);
        }
        23 => {
            c.w64(8, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008534b4); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008534b8); };
            c.w64(3, t32);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008534bc); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008534c1); };
            c.w64(7, t33);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008534c8); }
            blk = 13; continue;
        }
        24 => {
            return RegfileExit::jump(0x70008534d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008534d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008534d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008534d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008534d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008534da); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008534df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008534e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008534f0); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(5));
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008534fd); };
            c.w64(1, t2);
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000853503); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853513); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853519); };
            c.cmp(t6, 0xbau64, 16);
            if c.cc_b() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000853525); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(7, c.r64(5));
            c.w64(8, c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x18));
            c.w32(9, 0x8u64);
            c.w64(7, c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x18));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000853542); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 10; continue;
        }
        9 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085354d); };
            c.w64(6, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853551); };
            c.w64(1, t8);
            c.w64(8, c.r64(2));
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000853558); };
            c.w64(9, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700085355b); }
            c.w64(0, c.r64(1));
            c.w64(3, c.r64(9));
            return c.call(0x7000821b34, 0x700085356a, 0x7000853565);
        }
        11 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085356a); };
            c.w64(7, t10);
            let r11 = c.add(c.r64(7), 0x8u64, 64);
            c.w64(7, r11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000853572); };
            c.w64(2, t12);
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085357b); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085357f); };
            c.w64(2, t14);
            blk = 13; continue;
        }
        13 => {
            let r15 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r15);
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085358d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085358e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000853594); }
            return c.ret(0x0, 0x7000853596);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000853597); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085359b); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008535ac, 0x70008535a7);
        }
        15 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008535ac); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008535b0); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        16 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a2054, 0x70008535c1, 0x70008535bc);
        }
        17 => {
            c.w64(1, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008535c4); };
            c.w64(2, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008535c8); };
            c.w64(6, t20);
            blk = 6; continue;
        }
        18 => {
            return RegfileExit::jump(0x70008535d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008535f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008535f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008535f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008535f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008535fa); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008535ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(7, c.r64(5));
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000853615); };
            c.w64(7, t2);
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085361b); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(8, c.r64(5));
            let r4 = c.add(c.r64(8), 0x18u64, 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085362f); };
            c.w64(8, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000853635); };
            c.w64(8, t6);
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700085363c); };
            c.cmp(t7, c.r32(7), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085365b); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853662); };
            c.cmp(t10, 0xbau64, 16);
            if c.cc_b() { blk = 56; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000853679); };
            c.cmp(t11, c.r32(8), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853682); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853689); };
            c.cmp(t14, 0x410u64, 16);
            if c.cc_be() { blk = 56; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853696); };
            c.w32(12, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085369a); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008536a7); };
            c.w32(11, t17);
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r32(11) & 0xffc00u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008536cf); };
            c.w32(9, t18);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x8cu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x70008536e2);
        }
        19 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008536e3); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008536ed); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008536f4); };
            c.cmp(t23, c.r32(7), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t24) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008536fa); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000853701); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            c.w32(11, c.r32(9));
            let r28 = c.r32(11) & 0x3u64;
            c.w32(11, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853723); };
            c.w32(11, t29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0xe08)) else { return RegfileExit::fault(0x7000853727); };
            c.cmp(t30, c.r32(11), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000853734); };
            c.w32(9, t31);
            let r32 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085373b); };
            c.cmp(t33, c.r32(9), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 25; continue;
        }
        25 => {
            let r34 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r34);
            c.logic(r34, 64);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853756); };
            c.w64(7, t35);
            blk = 39; continue;
        }
        26 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853762); };
            c.w64(7, t36);
            blk = 39; continue;
        }
        27 => {
            let Some(t37) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085376e); };
            c.w32(7, t37);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xb1u64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x7000853780);
        }
        29 => {
            let Some(t38) = c.ld32(c.r64(6).wrapping_add(0x2cb)) else { return RegfileExit::fault(0x7000853781); };
            c.w32(9, t38);
            let r39 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085378b); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000853792); };
            c.w64(8, t41);
            c.w64(11, c.r64(8).wrapping_add(0x4c));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853799); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008537a0); };
            c.cmp(t43, c.r64(11), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008537aa); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008537b1); };
            c.w64(8, t44);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008537b8); }
            c.w32(8, 0x22u64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008537c2); }
            let Some(t45) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008537c6); };
            c.w32(8, t45);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008537dd); }
            let Some(t46) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008537e1); };
            c.w32(11, t46);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008537f5); }
            let Some(t47) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008537f9); };
            c.w32(11, t47);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_be() { blk = 51; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(11, 0x8u64);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000853813); }
            let Some(t48) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853817); };
            c.w32(11, t48);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(11, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x7000853831); }
            let Some(t49) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853835); };
            c.w32(11, t49);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x7000853849); }
            c.w64(8, c.r64(7).wrapping_add(0x4b));
            c.w64(11, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085385a); };
            c.w64(12, t50);
            blk = 38; continue;
        }
        38 => {
            if !c.st32(c.r64(11), c.r32(12)) { return RegfileExit::fault(0x7000853861); }
            let r51 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r51);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t52) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085386d); };
            c.w32(8, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853872); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000853879); };
            c.w64(12, t54);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853888); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700085388f); };
            c.cmp(t56, c.r64(8), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x7000853898); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 41; continue;
        }
        41 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008538a4); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008538a8); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008538ac); };
            c.w64(7, t57);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008538b3); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008538bd); };
            c.w64(7, t58);
            blk = 43; continue;
        }
        43 => {
            let r59 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r59);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(7)) { return RegfileExit::fault(0x70008538c8); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 43; continue; }
            blk = 44; continue;
        }
        44 => {
            let r60 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r60);
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008538dc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008538dd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008538e3); }
            return c.ret(0x0, 0x70008538e5);
        }
        45 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008538e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008538ea); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008538ee); };
            c.w64(3, t61);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008538ff, 0x70008538fa);
        }
        46 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008538ff); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853903); };
            c.w64(0, t63);
            blk = 1; continue;
        }
        47 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000853911); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000853915); }
            return c.call(0x70007febf4, 0x700085391e, 0x7000853919);
        }
        48 => {
            c.w64(7, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853921); };
            c.w64(1, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000853925); };
            c.w64(9, t65);
            blk = 31; continue;
        }
        49 => {
            return RegfileExit::fault(0x700085392e);
        }
        50 => {
            return RegfileExit::fault(0x700085392f);
        }
        51 => {
            return RegfileExit::fault(0x7000853930);
        }
        52 => {
            return RegfileExit::fault(0x7000853931);
        }
        53 => {
            return RegfileExit::fault(0x7000853932);
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000853933); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000853937); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700085393b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700085393f); }
            return c.call(0x70007febf4, 0x7000853948, 0x7000853943);
        }
        55 => {
            c.w64(8, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085394b); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085394f); };
            c.w64(7, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000853953); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000853957); };
            c.w64(2, t69);
            blk = 41; continue;
        }
        56 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000853960); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000853961); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000853963); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2410)) else { return RegfileExit::fault(0x7000853967); };
            c.w64(3, t70);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000853978, 0x7000853973);
        }
        57 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853978); };
            c.w64(1, t71);
            let r72 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r72);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000853983); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000853984); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085398a); }
            return c.ret(0x0, 0x700085398c);
        }
        58 => {
            return RegfileExit::jump(0x7000853990);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000853994(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a38)) else { return RegfileExit::fault(0x7000853994); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008539b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a40)) else { return RegfileExit::fault(0x70008539b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008539d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a48)) else { return RegfileExit::fault(0x70008539d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008539f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a50)) else { return RegfileExit::fault(0x70008539f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000853a14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000853a14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000853a18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000853a19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000853a1a); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000853a1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000853a23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000853a2d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853a39); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853a3f); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 58; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853a4b); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853a51); };
            c.w32(7, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000853a55); }
            c.cmp(c.r16(7), 0xbdu64, 16);
            if c.cc_b() { blk = 84; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000853a64); };
            c.w32(8, t9);
            let r10 = c.r32(8) & 0xfcu64;
            c.w32(8, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(8), 0x40u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(8), 0x3cu64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r11 = c.r32(9) | c.r32(8);
            c.w32(9, r11);
            c.logic(r11, 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(7, c.r16(7));
            c.cmp(c.r32(7), 0x410u64, 32);
            if c.cc_le() { blk = 84; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000853aa0); };
            c.w32(7, t12);
            c.logic(c.r32(7) & 0x200000u64, 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853aaf); };
            c.w32(8, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853ab3); };
            c.cmp(t14, c.r32(8), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853ac0); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000853ac7); };
            c.w32(8, t17);
            let r18 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r18);
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853ace); };
            c.w32(9, t19);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x7000853ade);
        }
        10 => {
            let r20 = c.r64(7) & 0x3ffu64;
            c.w64(7, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000853ae6); };
            c.w32(8, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000853aed); }
            c.cmp(c.r64(7), 0x3ffu64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 76; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 74; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853b15); };
            c.w64(9, t22);
            let Some(t23) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000853b1c); };
            c.w64(9, t23);
            c.w64(2, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853b2b); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000853b32); };
            c.cmp(t25, c.r64(11), 64);
            if c.cc_be() { blk = 60; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 60; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853b49); };
            c.w64(12, t26);
            if !c.st64(c.r64(12), c.r64(11)) { return RegfileExit::fault(0x7000853b50); }
            let r27 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r27);
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.w32(12, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000853b64); };
            c.w64(15, t28);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000853b6b); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000853b6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000853b73); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000853b7d); };
            c.w64(12, t29);
            blk = 18; continue;
        }
        18 => {
            let r30 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r30);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x7000853b88); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t31) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000853b92); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853b99); };
            c.w64(6, t33);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x7000853ba0); }
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(3));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000853ba9); }
            blk = 21; continue;
        }
        20 => {
            let r34 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000853bb3); }
            blk = 21; continue;
        }
        21 => {
            let r35 = c.imul2(c.r64(3), 0x3u64, 64);
            c.w64(12, r35);
            if let Some(sr) = c.shl(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(c.r64(12)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000853bbf); };
            c.w32(7, t36);
            let r37 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r37);
            let Some(t38) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853bc7); };
            c.w32(2, t38);
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000853bca); };
            c.cmp(t39, c.r32(2), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000853bd3); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000853bd7); };
            c.w64(9, t41);
            blk = 48; continue;
        }
        23 => {
            let Some(t42) = c.ld32(c.r64(11).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000853be0); };
            c.w32(12, t42);
            c.w32(2, c.r32(12));
            c.w64(9, c.r64(2));
            let r43 = c.r32(9) & 0x2u64;
            c.w32(9, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000853bff); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000853c03); };
            c.w64(9, t45);
            blk = 48; continue;
        }
        26 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(9, c.r32(3));
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t46) = c.ld32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000853c1c); };
            c.w32(9, t46);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853c21); };
            c.w64(11, t47);
            blk = 40; continue;
        }
        28 => {
            let r48 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r48);
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t49) = c.ld8(c.r64(1).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000853c34); };
            c.w32(9, t49);
            let Some(t50) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853c39); };
            c.w32(11, t50);
            let r51 = c.r32(2) & 0x380u64;
            c.w32(2, r51);
            c.logic(r51, 32);
            let r52 = c.r64(12) & 0x3ffu64;
            c.w64(12, r52);
            c.logic(r52, 64);
            c.cmp(c.r32(2), 0x100u64, 32);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            let r53 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r53);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853c5f); };
            c.w32(12, t54);
            let r55 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r55);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(12, c.r64(6));
            blk = 31; continue;
        }
        31 => {
            let r56 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r56);
            let Some(t57) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000853c72); };
            c.w32(9, t57);
            let r58 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r58);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000853c7f); }
            blk = 36; continue;
        }
        33 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853c87); };
            c.w64(11, t59);
            blk = 40; continue;
        }
        34 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t60) = c.ld32(c.r64(0).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853c95); };
            c.w32(9, t60);
            let r61 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r61);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000853c9d); }
            blk = 36; continue;
        }
        36 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853ca3); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000853caa); };
            c.w64(9, t63);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853cb1); };
            c.w64(2, t64);
            let Some(t65) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000853cb8); };
            c.cmp(t65, c.r64(12), 64);
            if c.cc_be() { blk = 62; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x7000853cc1); }
            let r66 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r66);
            blk = 38; continue;
        }
        38 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000853cc8); };
            c.w64(12, t67);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000853ccf); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000853cd3); }
            blk = 40; continue;
        }
        39 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853cdb); };
            c.w32(9, t68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853ce0); };
            c.w64(11, t69);
            blk = 40; continue;
        }
        40 => {
            let Some(t70) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000853ce7); };
            c.w64(12, t70);
            c.w64(2, c.r64(12).wrapping_add(0x20));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853cef); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000853cf6); };
            c.cmp(t72, c.r64(2), 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000853cff); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 42; continue;
        }
        42 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000853d07); }
            c.w32(12, 0x4u64);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x7000853d11); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000853d15); }
            c.w64(0, c.r64(11).wrapping_add(0x10));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000853d1d); }
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000853d21); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000853d25); }
            let Some(t73) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853d29); };
            c.w32(12, t73);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000853d3d); }
            let Some(t74) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853d40); };
            c.w32(7, t74);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 67; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000853d53); }
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000853d57); };
            c.w64(0, t75);
            let Some(t76) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853d5b); };
            c.w32(7, t76);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000853d64); };
            c.w64(2, t77);
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ae() { blk = 68; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, c.r64(2).wrapping_add(0x1));
            c.w64(9, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(0).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x7000853d7d); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r78 = c.r64(12) & c.r64(0);
            c.w64(12, r78);
            c.logic(r78, 64);
            let Some(t79) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000853d91); };
            c.logic(t79 & 0x4u64, 8);
            if c.cc_ne() { blk = 86; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(2, c.r64(7));
            c.w64(9, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853da3); };
            c.w64(0, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000853da7); };
            c.w64(11, t81);
            blk = 48; continue;
        }
        48 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000853dab); };
            c.w64(7, t82);
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_ne() { blk = 20; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853dc2); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000853dc9); };
            c.w64(7, t84);
            c.w64(11, c.r64(7).wrapping_add(0x10));
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853dd0); };
            c.w64(12, t85);
            let Some(t86) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000853dd7); };
            c.cmp(t86, c.r64(11), 64);
            if c.cc_be() { blk = 69; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x7000853de1); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 52; continue;
        }
        52 => {
            c.w32(7, c.r64(2).wrapping_add(c.r64(2)));
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000853deb); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000853def); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000853df2); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000853df5); }
            c.w64(0, c.r64(1));
            blk = 53; continue;
        }
        53 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000853dfc); };
            c.w64(1, t87);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000853e03); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            return c.ret(0x10, 0x7000853e0a);
        }
        55 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000853e0d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000853e14); }
            return c.ret(0x0, 0x7000853e16);
        }
        56 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000853e19); };
            c.w64(3, t88);
            return c.call(0x700086ed74, 0x7000853e25, 0x7000853e20);
        }
        57 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000853e25); };
            c.w64(6, t89);
            blk = 1; continue;
        }
        58 => {
            return c.call(0x70008a2054, 0x7000853e33, 0x7000853e2e);
        }
        59 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000853e33); };
            c.w64(6, t90);
            blk = 3; continue;
        }
        60 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000853e3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(1)) { return RegfileExit::fault(0x7000853e40); }
            return c.call(0x70007febb4, 0x7000853e49, 0x7000853e44);
        }
        61 => {
            c.w64(9, c.r64(0));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853e4c); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000853e50); };
            c.w64(6, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000853e54); };
            c.w64(8, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000853e58); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000853e5c); };
            c.w64(1, t95);
            blk = 16; continue;
        }
        62 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000853e6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000853e6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x7000853e72); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000853e76); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(1)) { return RegfileExit::fault(0x7000853e7b); }
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x7000853e87, 0x7000853e82);
        }
        63 => {
            c.w64(9, c.r64(0));
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853e8a); };
            c.w64(0, t96);
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853e8e); };
            c.w64(6, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000853e95); };
            c.w64(8, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000853e99); };
            c.w64(3, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000853e9d); };
            c.w64(7, t100);
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000853ea1); };
            c.w64(15, t101);
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853ea8); };
            c.w64(11, t102);
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853eaf); };
            c.w64(2, t103);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000853eb6); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000853ebb); };
            c.w64(1, t104);
            blk = 38; continue;
        }
        64 => {
            c.w32(2, 0x20u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000853ec9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000853ecd); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x7000853ed1); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x7000853ed5); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(1)) { return RegfileExit::fault(0x7000853ed9); }
            return c.call(0x70007febf4, 0x7000853ee2, 0x7000853edd);
        }
        65 => {
            c.w64(11, c.r64(0));
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853ee5); };
            c.w64(6, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000853eec); };
            c.w64(8, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000853ef0); };
            c.w64(3, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000853ef4); };
            c.w64(9, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000853ef8); };
            c.w64(7, t109);
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000853efc); };
            c.w64(15, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000853f03); };
            c.w64(1, t111);
            blk = 42; continue;
        }
        66 => {
            return RegfileExit::fault(0x7000853f0c);
        }
        67 => {
            return RegfileExit::fault(0x7000853f0d);
        }
        68 => {
            return RegfileExit::fault(0x7000853f0e);
        }
        69 => {
            c.w32(1, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000853f14); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000853f18); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000853f24, 0x7000853f1f);
        }
        70 => {
            c.w64(1, c.r64(0));
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853f27); };
            c.w64(6, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000853f2e); };
            c.w64(9, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000853f32); };
            c.w64(8, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000853f36); };
            c.w64(2, t115);
            blk = 52; continue;
        }
        71 => {
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000853f3f); };
            c.w64(6, t116);
            blk = 72; continue;
        }
        72 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000853f43); }
            c.w32(0, 0x1u64);
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2418)) else { return RegfileExit::fault(0x7000853f49); };
            c.w64(3, t117);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000853f58, 0x7000853f53);
        }
        73 => {
            blk = 53; continue;
        }
        74 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000853f5d); };
            c.w64(3, t118);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000853f6d, 0x7000853f68);
        }
        75 => {
            return RegfileExit::fault(0x7000853f6d);
        }
        76 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853f6e); };
            c.w64(6, t119);
            blk = 77; continue;
        }
        77 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000853f75); };
            c.w64(7, t120);
            let Some(t121) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000853f79); };
            c.w32(1, t121);
            let r122 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r122);
            let Some(t123) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000853f7f); };
            c.w32(1, t123);
            let r124 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r124);
            let Some(t125) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000853f85); };
            c.w32(7, t125);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x6bu64, 64);
            if c.cc_a() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            return RegfileExit::fault(0x7000853f94);
        }
        79 => {
            let Some(t126) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000853f95); };
            c.w32(1, t126);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000853f9b); };
            c.w64(7, t127);
            let Some(t128) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000853fa2); };
            c.w64(8, t128);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000853fa9); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000853fad); };
            c.w64(11, t129);
            let Some(t130) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000853fb4); };
            c.cmp(t130, c.r64(9), 64);
            if c.cc_a() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000853fc3, 0x7000853fbe);
        }
        81 => {
            blk = 83; continue;
        }
        82 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000853fc5); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 83; continue;
        }
        83 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000853fcf); };
            c.w64(1, t131);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000853fd3); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000853fd8); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000853fdb); };
            c.w64(6, t132);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000853fe2); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000853fe5); }
            blk = 53; continue;
        }
        84 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000853fed); }
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2420)) else { return RegfileExit::fault(0x7000853fee); };
            c.w64(3, t133);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000853fff, 0x7000853ffa);
        }
        85 => {
            blk = 53; continue;
        }
        86 => {
            c.w32(11, c.r32(11));
            let r134 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r134);
            c.w64(12, 0xfffffffffffc0000u64);
            let r135 = c.r64(12) & c.r64(11);
            c.w64(12, r135);
            c.logic(r135, 64);
            let Some(t136) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000854014); };
            c.logic(t136 & 0x2u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000854024); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000854025); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000854032, 0x700085402d);
        }
        88 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000854032); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000854034); }
            blk = 47; continue;
        }
        89 => {
            return RegfileExit::jump(0x700085403c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000854054(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a58)) else { return RegfileExit::fault(0x7000854054); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000854074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000854074); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000854078); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000854079); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085407a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700085407f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000854083); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 126; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000854090); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(8, c.r64(5));
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085409d); };
            c.w64(8, t2);
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008540a3); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(5));
            let r4 = c.add(c.r64(9), 0x18u64, 64);
            c.w64(9, r4);
            let Some(t5) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008540b7); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008540bd); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008540ce); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008540d5); };
            c.cmp(t9, 0xbau64, 16);
            if c.cc_b() { blk = 128; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008540e2); }
            c.w64(0, c.r64(9));
            return c.call(0x70008a1af4, 0x70008540ee, 0x70008540e9);
        }
        10 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008540ee); };
            c.w64(8, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008540f2); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008540f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008540fd); }
            let Some(t13) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854101); };
            c.cmp(t13, 0x410u64, 16);
            if c.cc_be() { blk = 146; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854115); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085411c); };
            c.w32(9, t16);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000854128); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r17 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r17);
            c.w64(11, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 146; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 130; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854179); };
            c.w32(11, t18);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(0));
            blk = 29; continue;
        }
        24 => {
            c.w64(10, c.r64(4));
            let r19 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r19);
            let r20 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r20);
            c.logic(r20, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008541b8); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(0));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x70008541c3); };
            c.w64(0, t21);
            c.w64(10, 0x70008541d1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008541d1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008541d5); }
            return c.call(c.r64(0), 0x70008541db, 0x70008541d9);
        }
        25 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008541db); }
            let Some(t22) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008541e3); };
            c.w64(4, t22);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(9, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008541f3); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008541f7); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008541fb); };
            c.w64(7, t25);
            blk = 29; continue;
        }
        27 => {
            c.w64(9, c.r64(0));
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000854204); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(11));
            blk = 29; continue;
        }
        28 => {
            c.w64(9, c.r64(0));
            blk = 29; continue;
        }
        29 => {
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000854216); };
            c.w32(11, t28);
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000854227); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            c.logic(c.r32(11) & 0xffc00u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 31; continue;
        }
        31 => {
            let r31 = c.shr(c.r32(11), 0xau64, 32).unwrap_or(c.r32(11));
            c.w32(11, r31);
            let r32 = c.r32(11) & 0x3ffu64;
            c.w32(11, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_be() { blk = 44; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t33) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000854250); };
            c.w32(15, (t33 as u16 as i16 as i64 as u64));
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854256); };
            c.w32(3, t34);
            let r35 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r35);
            let r36 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r36);
            c.w64(1, c.r64(15));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        33 => {
            c.w64(1, c.r64(6));
            blk = 34; continue;
        }
        34 => {
            c.w64(6, c.r64(1));
            let r37 = c.sub(c.r32(6), c.r32(2), 32);
            c.w32(6, r37);
            let r38 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r38);
            let r39 = c.add(c.r32(6), c.r32(2), 32);
            c.w32(6, r39);
            c.w32(0, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085427a); };
            c.w32(0, t40);
            let r41 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r41);
            let r42 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r42);
            let r43 = c.r32(0) & 0x3ffu64;
            c.w32(0, r43);
            c.logic(r43, 32);
            c.w32(0, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085428f); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            let Some(t46) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854297); };
            c.w32(0, t46);
            let r47 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r47);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_be() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(0, c.r64(6).wrapping_add(0x1));
            c.w64(2, c.r64(0));
            c.w64(6, c.r64(1));
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 39; continue;
        }
        38 => {
            let r48 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r48);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(15), c.r32(2), 32);
            if c.cc_l() { blk = 125; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(0, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008542c2); };
            c.w32(0, t49);
            let r50 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r50);
            let r51 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r51);
            let r52 = c.r32(0) & 0x3ffu64;
            c.w32(0, r52);
            c.logic(r52, 32);
            c.w32(1, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t53) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008542d7); };
            c.w32(6, t53);
            let r54 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r54);
            let Some(t55) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008542df); };
            c.w32(8, t55);
            let r56 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r56);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_be() { blk = 125; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 48; continue;
        }
        44 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r57 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(1, r57);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 45; continue;
        }
        45 => {
            let r58 = c.sub(c.r64(1), 0x3u64, 64);
            c.w64(1, r58);
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854315); };
            c.cmp(t59, c.r32(9), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ne() { blk = 45; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 125; continue;
        }
        48 => {
            c.w64(8, c.r64(1).wrapping_mul(4));
            let Some(t60) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085432f); };
            c.w32(9, t60);
            c.w32(11, c.r32(9));
            c.w64(15, c.r64(9));
            let r61 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r61);
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t62) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000854343); };
            c.w32(7, t62);
            let r63 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085434b); };
            c.w64(11, t64);
            blk = 61; continue;
        }
        50 => {
            c.w64(8, c.r64(15));
            if let Some(sr) = c.shr(c.r64(8), 0x13u64, 64) { c.w64(8, sr); }
            let Some(t65) = c.ld8(c.r64(7).wrapping_add(0x4)) else { return RegfileExit::fault(0x700085435b); };
            c.w32(9, t65);
            let r66 = c.r64(8) & 0x3ffu64;
            c.w64(8, r66);
            c.logic(r66, 64);
            let r67 = c.r32(11) & 0x380u64;
            c.w32(11, r67);
            c.logic(r67, 32);
            let r68 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r68);
            c.cmp(c.r32(11), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t69) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854380); };
            c.w32(7, t69);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_b() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000854389); };
            c.w64(11, t70);
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085438d); };
            c.w32(12, t71);
            let r72 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r72);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085439a); };
            c.w64(12, t73);
            blk = 53; continue;
        }
        53 => {
            let r74 = c.sub(c.r64(8), c.r64(7), 64);
            c.w64(8, r74);
            let Some(t75) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008543a4); };
            c.w32(7, t75);
            let r76 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r76);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008543b2); }
            blk = 57; continue;
        }
        55 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008543bf); };
            c.w64(7, t77);
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008543c3); };
            c.w32(8, t78);
            let r79 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r79);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008543cb); }
            c.w64(11, c.r64(7));
            blk = 57; continue;
        }
        57 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008543d4); };
            c.w64(7, t80);
            let Some(t81) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008543db); };
            c.w64(8, t81);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008543e2); };
            c.w64(12, t82);
            let Some(t83) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008543e9); };
            c.cmp(t83, c.r64(9), 64);
            if c.cc_be() { blk = 136; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008543f3); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 59; continue;
        }
        59 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008543fa); };
            c.w64(8, t84);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000854401); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000854405); }
            blk = 61; continue;
        }
        60 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085440c); };
            c.w64(11, t85);
            let Some(t86) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854410); };
            c.w32(7, t86);
            let r87 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r87);
            blk = 61; continue;
        }
        61 => {
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(15));
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(11));
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000854424); };
            c.w64(0, t88);
            blk = 70; continue;
        }
        62 => {
            let Some(t89) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085442d); };
            c.w32(7, t89);
            let r90 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r90);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700085443a); };
            c.w64(7, t91);
            blk = 64; continue;
        }
        64 => {
            let Some(t92) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854441); };
            c.w32(11, t92);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r93 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r93);
            let Some(t94) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085444f); };
            c.w32(12, t94);
            let r95 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r95);
            let r96 = c.r64(12) & c.r64(11);
            c.w64(12, r96);
            c.logic(r96, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(3));
            blk = 66; continue;
        }
        65 => {
            let r97 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r97);
            let r98 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r98);
            c.w64(2, c.r64(11));
            let r99 = c.r64(2) & c.r64(12);
            c.w64(2, r99);
            c.logic(r99, 64);
            c.w64(12, c.r64(2));
            blk = 66; continue;
        }
        66 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(2).wrapping_mul(4));
            let Some(t100) = c.ld32(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085447e); };
            c.w32(6, t100);
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854482); };
            c.cmp(t101, c.r32(6), 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(9), c.r32(6), 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t102) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854494); };
            c.w32(9, t102);
            let Some(t103) = c.ld32(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854498); };
            c.w32(11, t103);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ae() { blk = 138; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t104) = c.ld32(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008544b0); };
            c.w32(7, t104);
            let r105 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r105);
            let r106 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r106);
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(11));
            blk = 70; continue;
        }
        70 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t107) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008544ca); };
            c.w32(11, t107);
            let Some(t108) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008544ce); };
            c.cmp(t108, c.r32(11), 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t109) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008544db); };
            c.w32(11, t109);
            let r110 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r110);
            let Some(t111) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008544e2); };
            c.w32(12, t111);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 86; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(3, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_be() { blk = 79; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 146; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t112) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085450c); };
            c.w32(9, t112);
            let Some(t113) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000854510); };
            c.cmp(t113, c.r32(9), 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085451d); };
            c.w32(9, t114);
            let r115 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r115);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085452e); };
            c.w32(11, t116);
            let r117 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r117);
            let Some(t118) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854535); };
            c.cmp(t118, 0x40u64, 16);
            if c.cc_ae() { blk = 146; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t119) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854541); };
            c.w32(9, t119);
            let r120 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r120);
            blk = 88; continue;
        }
        79 => {
            let Some(t121) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085454d); };
            c.w32(9, t121);
            let Some(t122) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000854551); };
            c.cmp(t122, c.r32(9), 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t123) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085455e); };
            c.w32(9, t123);
            c.w64(11, c.r64(9));
            let r124 = c.r32(11) & 0xc0u64;
            c.w32(11, r124);
            c.logic(r124, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t125) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000854576); };
            c.w32(11, t125);
            let r126 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r126);
            let Some(t127) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085457d); };
            c.w32(11, t127);
            let r128 = c.r32(11) & 0x1fu64;
            c.w32(11, r128);
            c.logic(r128, 32);
            let r129 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r129);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 146; continue; }
            blk = 82; continue;
        }
        82 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 146; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085459d); };
            c.w32(9, t130);
            let r131 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r131);
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008545a4); };
            c.cmp(t132, c.r32(9), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t133) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008545b1); };
            c.w32(11, t133);
            let Some(t134) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008545b5); };
            c.cmp(t134, c.r32(11), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t135) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008545be); };
            c.w32(9, t135);
            let r136 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r136);
            blk = 88; continue;
        }
        86 => {
            let Some(t137) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008545c7); };
            c.w32(9, t137);
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008545cb); };
            c.cmp(t138, c.r32(9), 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008545d8); };
            c.w32(9, t139);
            let r140 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r140);
            blk = 88; continue;
        }
        88 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t141) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008545e9); };
            c.w32(11, t141);
            let Some(t142) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008545ed); };
            c.cmp(t142, c.r32(11), 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t143) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008545fa); };
            c.w32(11, t143);
            let r144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r144);
            let Some(t145) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854601); };
            c.w32(9, t145);
            let r146 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000854608); };
            c.cmp(t147, c.r32(9), 32);
            if c.cc_e() { blk = 140; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t148) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854615); };
            c.w32(12, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000854619); };
            c.cmp(t149, c.r32(12), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t150) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000854626); };
            c.cmp(t150, c.r32(11), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t151) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854633); };
            c.w32(12, t151);
            let Some(t152) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000854637); };
            c.cmp(t152, c.r32(12), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000854644); };
            c.w64(6, t153);
            let Some(t154) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854648); };
            c.w32(8, t154);
            let r155 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r155);
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085464f); };
            c.w32(8, t156);
            let r157 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r157);
            let Some(t158) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854656); };
            c.w32(12, t158);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x26u64, 64);
            if c.cc_a() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            return RegfileExit::fault(0x7000854666);
        }
        96 => {
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(0x9f)) else { return RegfileExit::fault(0x7000854667); };
            c.w32(8, t159);
            let r160 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r160);
            let Some(t161) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854671); };
            c.w32(12, t161);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000854676); };
            c.w64(15, t162);
            let Some(t163) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700085467d); };
            c.w64(0, t163);
            c.w64(2, c.r64(12).wrapping_mul(4));
            c.w64(12, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085468c); };
            c.w64(3, t164);
            let Some(t165) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000854693); };
            c.cmp(t165, c.r64(12), 64);
            if c.cc_be() { blk = 142; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(15), c.r64(12)) { return RegfileExit::fault(0x700085469c); }
            c.w64(12, c.r64(0).wrapping_add(0x1));
            blk = 98; continue;
        }
        98 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008546a3); }
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008546a8); };
            c.w64(8, t166);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008546af); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008546b4); }
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 101; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008546c3); };
            c.w64(15, t167);
            blk = 100; continue;
        }
        100 => {
            let r168 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r168);
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x70008546ce); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 100; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(8, c.r64(7));
            let r169 = c.r32(8) & 0x20u64;
            c.w32(8, r169);
            c.logic(r169, 32);
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008546e5); };
            c.w64(7, t170);
            blk = 104; continue;
        }
        103 => {
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008546ee); };
            c.w64(7, t171);
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008546fb); };
            c.w64(8, t172);
            blk = 107; continue;
        }
        106 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000854704); };
            c.w64(8, t173);
            blk = 107; continue;
        }
        107 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700085470b); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x7000854710); }
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x7000854715); }
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x700085471a); }
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085471f); };
            c.w64(11, t174);
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085472d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085472e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000854734); }
            return c.ret(0x0, 0x7000854736);
        }
        108 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000854737); };
            c.w64(6, t175);
            let Some(t176) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085473b); };
            c.w32(8, t176);
            let r177 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r177);
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000854742); };
            c.w32(8, t178);
            let r179 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r179);
            let Some(t180) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854749); };
            c.w32(11, t180);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3cu64, 64);
            if c.cc_a() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            return RegfileExit::fault(0x7000854759);
        }
        110 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xf7)) else { return RegfileExit::fault(0x700085475a); };
            c.w32(8, t181);
            let r182 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r182);
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000854764); };
            c.w64(11, t183);
            let Some(t184) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085476b); };
            c.w64(12, t184);
            let Some(t185) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085476e); };
            c.w32(15, t185);
            c.w64(2, c.r64(15).wrapping_mul(4));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085477f); };
            c.w64(0, t186);
            let Some(t187) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000854786); };
            c.cmp(t187, c.r64(15), 64);
            if c.cc_be() { blk = 144; continue; }
            blk = 111; continue;
        }
        111 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700085478f); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 112; continue;
        }
        112 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000854797); }
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085479b); };
            c.w64(8, t188);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008547a2); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008547a6); }
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008547b4); };
            c.w64(12, t189);
            blk = 114; continue;
        }
        114 => {
            let r190 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r190);
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008547bf); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 114; continue; }
            blk = 115; continue;
        }
        115 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(7) & 0x20u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.logic(c.r8(7) & 0x8u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008547e7); };
            c.w64(7, t191);
            blk = 118; continue;
        }
        117 => {
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008547f0); };
            c.w64(7, t192);
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008547fd); };
            c.w64(8, t193);
            blk = 121; continue;
        }
        120 => {
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000854806); };
            c.w64(8, t194);
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000854813); };
            c.w64(12, t195);
            blk = 124; continue;
        }
        123 => {
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085481c); };
            c.w64(12, t196);
            blk = 124; continue;
        }
        124 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000854823); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000854827); }
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x700085482b); }
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x700085482f); }
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000854833); };
            c.w64(7, t197);
            let r198 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r198);
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000854841); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000854842); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000854848); }
            return c.ret(0x0, 0x700085484a);
        }
        125 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085484b); };
            c.w64(7, t199);
            let r200 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r200);
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854853); };
            c.w64(0, t201);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085485d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085485e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000854864); }
            return c.ret(0x0, 0x7000854866);
        }
        126 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000854867); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085486d); };
            c.w64(3, t202);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x700085487c, 0x7000854877);
        }
        127 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085487c); };
            c.w64(6, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000854880); };
            c.w64(0, t204);
            blk = 1; continue;
        }
        128 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000854889); }
            c.w64(0, c.r64(8));
            return c.call(0x70008a2054, 0x7000854895, 0x7000854890);
        }
        129 => {
            c.w64(8, c.r64(0));
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000854898); };
            c.w64(6, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085489c); };
            c.w64(9, t206);
            blk = 9; continue;
        }
        130 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 146; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t207) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008548b0); };
            c.w32(9, t207);
            let r208 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r208);
            blk = 29; continue;
        }
        132 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 396; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r64(7), 0xfffffffffffffffeu64, 64);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008548d2); };
            c.w64(8, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008548d6); };
            c.w64(0, t210);
            blk = 146; continue;
        }
        135 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008548df); };
            c.w64(8, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008548e3); };
            c.w64(0, t212);
            blk = 146; continue;
        }
        136 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008548f1); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008548f5); }
            return c.call(0x70007febf4, 0x70008548ff, 0x70008548fa);
        }
        137 => {
            c.w64(7, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000854902); };
            c.w64(11, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000854906); };
            c.w64(15, t214);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700085490a); }
            blk = 59; continue;
        }
        138 => {
            return RegfileExit::fault(0x7000854914);
        }
        139 => {
            blk = 146; continue;
        }
        140 => {
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854917); };
            c.w64(9, t215);
            blk = 92; continue;
        }
        141 => {
            let Some(t216) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854923); };
            c.w64(11, t216);
            blk = 94; continue;
        }
        142 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700085492f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000854933); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000854937); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700085493b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700085493f); }
            return c.call(0x70007febf4, 0x7000854948, 0x7000854943);
        }
        143 => {
            c.w64(12, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085494b); };
            c.w64(7, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085494f); };
            c.w64(9, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000854953); };
            c.w64(11, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000854957); };
            c.w64(2, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085495b); };
            c.w64(8, t221);
            blk = 98; continue;
        }
        144 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000854964); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000854968); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700085496c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000854970); }
            return c.call(0x70007febf4, 0x7000854979, 0x7000854974);
        }
        145 => {
            c.w64(11, c.r64(0));
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085497c); };
            c.w64(9, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000854980); };
            c.w64(7, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000854984); };
            c.w64(2, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000854988); };
            c.w64(8, t225);
            blk = 112; continue;
        }
        146 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000854991); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000854993); }
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2390)) else { return RegfileExit::fault(0x7000854994); };
            c.w64(3, t226);
            c.w32(0, 0x2u64);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008549a0); };
            c.w64(6, t227);
            return c.call(0x700086ed74, 0x70008549a9, 0x70008549a4);
        }
        147 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008549a9); }
            let Some(t228) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008549ad); };
            c.cmp(t228, c.r32(0), 32);
            if c.cc_e() { blk = 396; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t229) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008549ba); };
            c.w32(7, t229);
            c.w32(8, c.r32(7));
            let r230 = c.r32(8) & 0x3d4u64;
            c.w32(8, r230);
            c.logic(r230, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008549c7); }
            c.cmp(c.r32(8), 0x314u64, 32);
            if c.cc_e() { blk = 380; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(8), 0xd4u64, 32);
            if c.cc_e() { blk = 361; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008549e5); };
            c.w64(6, t231);
            let Some(t232) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008549e9); };
            c.w32(8, t232);
            let r233 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r233);
            let Some(t234) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008549f0); };
            c.w32(8, t234);
            let r235 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r235);
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008549f7); };
            c.w32(9, t236);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xb2u64, 64);
            if c.cc_a() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            return RegfileExit::fault(0x7000854a0a);
        }
        152 => {
            let Some(t237) = c.ld32(c.r64(8).wrapping_add(0x2cf)) else { return RegfileExit::fault(0x7000854a0b); };
            c.w32(8, t237);
            let r238 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r238);
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000854a15); };
            c.w64(9, t239);
            let Some(t240) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000854a1c); };
            c.w64(11, t240);
            c.w64(12, c.r64(11).wrapping_add(0xdc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000854a26); }
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000854a2a); };
            c.w64(15, t241);
            let Some(t242) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000854a31); };
            c.cmp(t242, c.r64(12), 64);
            if c.cc_a() { blk = 155; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w32(2, 0xdcu64);
            return c.call(0x70007febf4, 0x7000854a40, 0x7000854a3b);
        }
        154 => {
            blk = 156; continue;
        }
        155 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000854a42); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 156; continue;
        }
        156 => {
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x7000854a4c); };
            c.w64(7, t243);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000854a53); }
            c.w32(7, 0x6au64);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000854a5b); }
            let Some(t244) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854a5e); };
            c.w32(7, t244);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000854a67); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            return RegfileExit::fault(0x7000854a71);
        }
        158 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000854a75); }
            let Some(t245) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854a79); };
            c.w32(7, t245);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_a() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            return RegfileExit::fault(0x7000854a88);
        }
        160 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x7000854a89); }
            let Some(t246) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854a8d); };
            c.w32(7, t246);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x2u64, 64);
            if c.cc_a() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            return RegfileExit::fault(0x7000854a9c);
        }
        162 => {
            c.w32(7, 0x20u64);
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000854aa2); }
            let Some(t247) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854aa5); };
            c.w32(7, t247);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x3u64, 64);
            if c.cc_a() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            return RegfileExit::fault(0x7000854ab4);
        }
        164 => {
            c.w32(7, 0x2u64);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x7000854aba); }
            let Some(t248) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854abd); };
            c.w32(7, t248);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_a() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            return RegfileExit::fault(0x7000854acc);
        }
        166 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x7000854acd); }
            c.w64(7, c.r64(0).wrapping_add(0xdb));
            c.w64(8, c.r64(0).wrapping_add(0x1b));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 169; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854ae1); };
            c.w64(9, t249);
            blk = 168; continue;
        }
        168 => {
            if !c.st32(c.r64(8), c.r32(9)) { return RegfileExit::fault(0x7000854ae8); }
            let r250 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r250);
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_ne() { blk = 168; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000854af4); };
            c.w64(7, t251);
            let Some(t252) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854af8); };
            c.w32(8, t252);
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000854afd); };
            c.w64(9, t253);
            let Some(t254) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000854b04); };
            c.w64(11, t254);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000854b13); }
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000854b17); };
            c.w64(12, t255);
            let Some(t256) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000854b1e); };
            c.cmp(t256, c.r64(8), 64);
            if c.cc_a() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            return c.call(0x70007febf4, 0x7000854b29, 0x7000854b24);
        }
        171 => {
            c.w64(8, c.r64(0));
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000854b2c); };
            c.w64(0, t257);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000854b33); };
            c.w64(2, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000854b37); };
            c.w64(7, t259);
            blk = 173; continue;
        }
        172 => {
            if !c.st64(c.r64(9), c.r64(8)) { return RegfileExit::fault(0x7000854b3d); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 173; continue;
        }
        173 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000854b44); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000854b48); }
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000854b4c); };
            c.w64(7, t260);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000854b53); }
            c.w64(7, c.r64(2));
            let r261 = c.sub(c.r64(7), 0x1u64, 64);
            c.w64(7, r261);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000854b5e); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 176; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854b68); };
            c.w64(9, t262);
            blk = 175; continue;
        }
        175 => {
            let r263 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r263);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x7000854b73); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 175; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000854b7d); };
            c.w64(7, t264);
            let Some(t265) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854b81); };
            c.w32(9, t265);
            let Some(t266) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000854b85); };
            c.cmp(t266, c.r32(9), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t267) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854b92); };
            c.w32(11, t267);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t268) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854b99); };
            c.w32(12, t268);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            return RegfileExit::fault(0x7000854ba6);
        }
        179 => {
            let Some(t269) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854ba7); };
            c.w32(3, t269);
            let r270 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r270);
            c.w64(2, c.r64(3));
            let r271 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r271);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r272 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r272);
            c.logic(r272, 64);
            let r273 = c.add(c.r32(2), c.r32(3), 32);
            c.w32(2, r273);
            c.w32(1, c.r32(12));
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 180; continue;
        }
        180 => {
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_a() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            return RegfileExit::fault(0x7000854bcd);
        }
        182 => {
            let r274 = c.sub(c.r32(1), c.r32(3), 32);
            c.w32(1, r274);
            let r275 = c.shr(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r275);
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t276) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000854bd5); };
            c.w32(2, t276);
            let r277 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r277);
            c.logic(r277, 64);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_a() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            return RegfileExit::fault(0x7000854beb);
        }
        185 => {
            let Some(t278) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000854bec); };
            c.w32(11, t278);
            let r279 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r279);
            c.w32(12, c.r64(11).wrapping_add(0x2));
            c.cmp(c.r32(12), 0xfffffeu64, 32);
            if c.cc_a() { blk = 360; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x7000854c07); }
            let Some(t280) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854c0b); };
            c.w32(12, t280);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            return RegfileExit::fault(0x7000854c1b);
        }
        188 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000854c1f); }
            let Some(t281) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854c23); };
            c.w32(12, t281);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r282 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r282);
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x7000854c31); };
            c.w64(3, t283);
            let Some(t284) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854c38); };
            c.w32(2, t284);
            let r285 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r285);
            let r286 = c.r64(2) & c.r64(12);
            c.w64(2, r286);
            c.logic(r286, 64);
            c.w64(1, c.r64(15));
            blk = 189; continue;
        }
        189 => {
            c.w64(6, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(6).wrapping_mul(4));
            let Some(t287) = c.ld32(c.r64(0).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854c51); };
            c.w32(7, t287);
            let Some(t288) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854c56); };
            c.cmp(t288, c.r32(7), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t289) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000854c5f); };
            c.cmp(t289, c.r32(7), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            let r290 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r290);
            c.w64(7, c.r64(1).wrapping_add(c.r64(2)));
            c.w64(8, c.r64(12));
            let r291 = c.r64(8) & c.r64(7);
            c.w64(8, r291);
            c.logic(r291, 64);
            c.w64(2, c.r64(8));
            blk = 189; continue;
        }
        192 => {
            let Some(t292) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854c7b); };
            c.w32(7, t292);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_b() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            return RegfileExit::fault(0x7000854c89);
        }
        194 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000854c8a); }
            let Some(t293) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854c8f); };
            c.w32(7, t293);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(12, c.r64(6).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_b() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            return RegfileExit::fault(0x7000854ca1);
        }
        196 => {
            if let Some(sr) = c.shl(c.r64(11), 0x8u64, 64) { c.w64(11, sr); }
            c.w32(7, c.r32(11));
            c.w64(12, c.r64(6).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(0).wrapping_add(c.r64(12)), c.r32(9)) { return RegfileExit::fault(0x7000854cb1); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r294 = c.r64(2) & c.r64(0);
            c.w64(2, r294);
            c.logic(r294, 64);
            let Some(t295) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000854cc5); };
            c.logic(t295 & 0x4u64, 8);
            if c.cc_ne() { blk = 412; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t296) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854ccf); };
            c.w32(9, t296);
            let Some(t297) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000854cd3); };
            c.cmp(t297, c.r32(9), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t298) = c.ld8(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854cdc); };
            c.logic(t298 & 0x1u64, 8);
            if c.cc_e() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let r299 = c.r32(7) | 0x20u64;
            c.w32(7, r299);
            c.logic(r299, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 202; continue;
        }
        201 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 202; continue;
        }
        202 => {
            let Some(t300) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854ced); };
            c.w32(9, t300);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(6).wrapping_add(0x2));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            return RegfileExit::fault(0x7000854d00);
        }
        204 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000854d01); }
            blk = 205; continue;
        }
        205 => {
            let Some(t301) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000854d06); };
            c.logic(t301 & 0x20u64, 8);
            if c.cc_ne() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000854d0c); };
            c.w64(7, t302);
            blk = 208; continue;
        }
        207 => {
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000854d15); };
            c.w64(7, t303);
            blk = 208; continue;
        }
        208 => {
            let Some(t304) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000854d1c); };
            c.logic(t304 & 0x40u64, 8);
            if c.cc_e() { blk = 237; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t305) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854d26); };
            c.w32(8, t305);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t306) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854d2d); };
            c.w32(9, t306);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            return RegfileExit::fault(0x7000854d3a);
        }
        211 => {
            let Some(t307) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854d3b); };
            c.w32(11, t307);
            let r308 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r308);
            c.w64(12, c.r64(11));
            let r309 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r309);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r310 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r310);
            c.logic(r310, 64);
            let r311 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r311);
            c.w32(3, c.r32(9));
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 214; continue; }
            blk = 213; continue;
        }
        213 => {
            return RegfileExit::fault(0x7000854d65);
        }
        214 => {
            let r312 = c.sub(c.r32(3), c.r32(11), 32);
            c.w32(3, r312);
            let r313 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r313);
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t314) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000854d6e); };
            c.w32(12, t314);
            let r315 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r315);
            c.logic(r315, 64);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_a() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            return RegfileExit::fault(0x7000854d85);
        }
        217 => {
            let Some(t316) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000854d86); };
            c.w32(8, t316);
            let r317 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r317);
            c.w32(9, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(9), 0xfffffeu64, 32);
            if c.cc_a() { blk = 360; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x7000854da1); }
            let Some(t318) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854da5); };
            c.w32(9, t318);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            return RegfileExit::fault(0x7000854db5);
        }
        220 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000854db9); }
            let Some(t319) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854dbd); };
            c.w32(9, t319);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r320 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r320);
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0xb88)) else { return RegfileExit::fault(0x7000854dcb); };
            c.w64(11, t321);
            let Some(t322) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854dd2); };
            c.w32(12, t322);
            let r323 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r323);
            let r324 = c.r64(12) & c.r64(9);
            c.w64(12, r324);
            c.logic(r324, 64);
            c.w64(3, c.r64(15));
            blk = 221; continue;
        }
        221 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(2).wrapping_mul(4));
            let Some(t325) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854ded); };
            c.w32(6, t325);
            let Some(t326) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854df1); };
            c.cmp(t326, c.r32(6), 32);
            if c.cc_e() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t327) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000854dfa); };
            c.cmp(t327, c.r32(6), 32);
            if c.cc_e() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            let r328 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r328);
            let r329 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r329);
            c.w64(2, c.r64(9));
            let r330 = c.r64(2) & c.r64(12);
            c.w64(2, r330);
            c.logic(r330, 64);
            c.w64(12, c.r64(2));
            blk = 221; continue;
        }
        224 => {
            let Some(t331) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854e15); };
            c.w32(9, t331);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            return RegfileExit::fault(0x7000854e24);
        }
        226 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000854e25); }
            let Some(t332) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854e2a); };
            c.w32(9, t332);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            return RegfileExit::fault(0x7000854e3d);
        }
        228 => {
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            c.w32(9, c.r32(8));
            c.w64(12, c.r64(2).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(0).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x7000854e4d); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r333 = c.r64(3) & c.r64(0);
            c.w64(3, r333);
            c.logic(r333, 64);
            let Some(t334) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000854e61); };
            c.logic(t334 & 0x4u64, 8);
            if c.cc_ne() { blk = 409; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t335) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854e6b); };
            c.w32(7, t335);
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000854e6f); };
            c.cmp(t336, c.r32(7), 32);
            if c.cc_ne() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t337) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854e78); };
            c.logic(t337 & 0x1u64, 8);
            if c.cc_e() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let r338 = c.r32(9) | 0x20u64;
            c.w32(9, r338);
            c.logic(r338, 32);
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 234; continue;
        }
        233 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 234; continue;
        }
        234 => {
            let Some(t339) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854e8b); };
            c.w32(8, t339);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(2).wrapping_add(0x2));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            return RegfileExit::fault(0x7000854e9e);
        }
        236 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000854e9f); }
            blk = 237; continue;
        }
        237 => {
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000854ea3); };
            c.w64(7, t340);
            let Some(t341) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000854ea7); };
            c.w32(8, t341);
            let Some(t342) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000854eab); };
            c.cmp(t342, c.r32(8), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t343) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854eb8); };
            c.w32(9, t343);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t344) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854ebf); };
            c.w32(11, t344);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 240; continue; }
            blk = 239; continue;
        }
        239 => {
            return RegfileExit::fault(0x7000854ecc);
        }
        240 => {
            let Some(t345) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854ecd); };
            c.w32(12, t345);
            let r346 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r346);
            c.w64(3, c.r64(12));
            let r347 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r347);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r348 = c.r64(3) & 0xfffffffffffffffeu64;
            c.w64(3, r348);
            c.logic(r348, 64);
            let r349 = c.add(c.r32(3), c.r32(12), 32);
            c.w32(3, r349);
            c.w32(2, c.r32(11));
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            return RegfileExit::fault(0x7000854ef6);
        }
        243 => {
            let r350 = c.sub(c.r32(2), c.r32(12), 32);
            c.w32(2, r350);
            let r351 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r351);
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t352) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000854eff); };
            c.w32(3, t352);
            let r353 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r353);
            c.logic(r353, 64);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 244; continue;
        }
        244 => {
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_a() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            return RegfileExit::fault(0x7000854f15);
        }
        246 => {
            let Some(t354) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000854f16); };
            c.w32(9, t354);
            let r355 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r355);
            c.w32(11, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r32(11), 0xfffffeu64, 32);
            if c.cc_a() { blk = 360; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x7000854f31); }
            let Some(t356) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854f35); };
            c.w32(11, t356);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            return RegfileExit::fault(0x7000854f45);
        }
        249 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000854f49); }
            let Some(t357) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000854f4d); };
            c.w32(11, t357);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r358 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r358);
            let Some(t359) = c.ld64(c.r64(13).wrapping_add(0x750)) else { return RegfileExit::fault(0x7000854f5b); };
            c.w64(12, t359);
            let Some(t360) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854f62); };
            c.w32(3, t360);
            let r361 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r361);
            let r362 = c.r64(3) & c.r64(11);
            c.w64(3, r362);
            c.logic(r362, 64);
            c.w64(2, c.r64(15));
            blk = 250; continue;
        }
        250 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(1).wrapping_mul(4));
            let Some(t363) = c.ld32(c.r64(0).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000854f7d); };
            c.w32(15, t363);
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000854f82); };
            c.cmp(t364, c.r32(15), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t365) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000854f8b); };
            c.cmp(t365, c.r32(15), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let r366 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r366);
            c.w64(15, c.r64(2).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(11));
            let r367 = c.r64(3) & c.r64(15);
            c.w64(3, r367);
            c.logic(r367, 64);
            blk = 250; continue;
        }
        253 => {
            let Some(t368) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854fa4); };
            c.w32(11, t368);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_b() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            return RegfileExit::fault(0x7000854fb3);
        }
        255 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(6)).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000854fb4); }
            let Some(t369) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000854fb9); };
            c.w32(11, t369);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 257; continue; }
            blk = 256; continue;
        }
        256 => {
            return RegfileExit::fault(0x7000854fcc);
        }
        257 => {
            if let Some(sr) = c.shl(c.r64(9), 0x8u64, 64) { c.w64(9, sr); }
            c.w32(11, c.r32(9));
            c.w64(15, c.r64(1).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(0).wrapping_add(c.r64(15)), c.r32(8)) { return RegfileExit::fault(0x7000854fdc); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 258; continue;
        }
        258 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r370 = c.r64(3) & c.r64(0);
            c.w64(3, r370);
            c.logic(r370, 64);
            let Some(t371) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000854ff0); };
            c.logic(t371 & 0x4u64, 8);
            if c.cc_ne() { blk = 406; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t372) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000854ffa); };
            c.w32(8, t372);
            let Some(t373) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000854fff); };
            c.cmp(t373, c.r32(8), 32);
            if c.cc_ne() { blk = 262; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t374) = c.ld8(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855008); };
            c.logic(t374 & 0x1u64, 8);
            if c.cc_e() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            let r375 = c.r32(11) | 0x20u64;
            c.w32(11, r375);
            c.logic(r375, 32);
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 263; continue;
        }
        262 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 263; continue;
        }
        263 => {
            let Some(t376) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085501c); };
            c.w32(9, t376);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(1).wrapping_add(0x2));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            return RegfileExit::fault(0x700085502f);
        }
        265 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(6)).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x7000855030); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 266; continue;
        }
        266 => {
            let Some(t377) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000855038); };
            c.w32(7, t377);
            let Some(t378) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085503b); };
            c.cmp(t378, c.r32(7), 32);
            if c.cc_e() { blk = 295; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t379) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855048); };
            c.w32(8, t379);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t380) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085504f); };
            c.w32(9, t380);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            return RegfileExit::fault(0x700085505c);
        }
        269 => {
            let Some(t381) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085505d); };
            c.w32(11, t381);
            let r382 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r382);
            c.w64(12, c.r64(11));
            let r383 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r383);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r384 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r384);
            c.logic(r384, 64);
            let r385 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r385);
            c.w32(3, c.r32(9));
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 270; continue;
        }
        270 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 272; continue; }
            blk = 271; continue;
        }
        271 => {
            return RegfileExit::fault(0x7000855087);
        }
        272 => {
            let r386 = c.sub(c.r32(3), c.r32(11), 32);
            c.w32(3, r386);
            let r387 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r387);
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t388) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000855090); };
            c.w32(12, t388);
            let r389 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r389);
            c.logic(r389, 64);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 273; continue;
        }
        273 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_a() { blk = 275; continue; }
            blk = 274; continue;
        }
        274 => {
            return RegfileExit::fault(0x70008550a7);
        }
        275 => {
            let Some(t390) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008550a8); };
            c.w32(8, t390);
            let r391 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r391);
            c.w32(9, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(9), 0xfffffeu64, 32);
            if c.cc_a() { blk = 360; continue; }
            blk = 276; continue;
        }
        276 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008550c3); }
            let Some(t392) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008550c7); };
            c.w32(9, t392);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 278; continue; }
            blk = 277; continue;
        }
        277 => {
            return RegfileExit::fault(0x70008550d7);
        }
        278 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008550db); }
            let Some(t393) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008550df); };
            c.w32(9, t393);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r394 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r394);
            let Some(t395) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x70008550ed); };
            c.w64(11, t395);
            let Some(t396) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008550f4); };
            c.w32(12, t396);
            let r397 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r397);
            let r398 = c.r64(12) & c.r64(9);
            c.w64(12, r398);
            c.logic(r398, 64);
            c.w64(3, c.r64(15));
            blk = 279; continue;
        }
        279 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(2).wrapping_mul(4));
            let Some(t399) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085510f); };
            c.w32(6, t399);
            let Some(t400) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000855113); };
            c.cmp(t400, c.r32(6), 32);
            if c.cc_e() { blk = 282; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t401) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085511c); };
            c.cmp(t401, c.r32(6), 32);
            if c.cc_e() { blk = 282; continue; }
            blk = 281; continue;
        }
        281 => {
            let r402 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r402);
            let r403 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r403);
            c.w64(2, c.r64(9));
            let r404 = c.r64(2) & c.r64(12);
            c.w64(2, r404);
            c.logic(r404, 64);
            c.w64(12, c.r64(2));
            blk = 279; continue;
        }
        282 => {
            let Some(t405) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855137); };
            c.w32(9, t405);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            return RegfileExit::fault(0x7000855146);
        }
        284 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000855147); }
            let Some(t406) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085514c); };
            c.w32(9, t406);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            return RegfileExit::fault(0x700085515f);
        }
        286 => {
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            c.w32(9, c.r32(8));
            c.w64(12, c.r64(2).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(0).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x700085516f); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 288; continue; }
            blk = 287; continue;
        }
        287 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r407 = c.r64(3) & c.r64(0);
            c.w64(3, r407);
            c.logic(r407, 64);
            let Some(t408) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000855183); };
            c.logic(t408 & 0x4u64, 8);
            if c.cc_ne() { blk = 403; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t409) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085518d); };
            c.w32(7, t409);
            let Some(t410) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000855191); };
            c.cmp(t410, c.r32(7), 32);
            if c.cc_ne() { blk = 291; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t411) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085519a); };
            c.logic(t411 & 0x1u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            let r412 = c.r32(9) | 0x20u64;
            c.w32(9, r412);
            c.logic(r412, 32);
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 292; continue;
        }
        291 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 292; continue;
        }
        292 => {
            let Some(t413) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008551ad); };
            c.w32(8, t413);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(2).wrapping_add(0x2));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            return RegfileExit::fault(0x70008551c0);
        }
        294 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008551c1); }
            blk = 295; continue;
        }
        295 => {
            let Some(t414) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008551c5); };
            c.logic(t414 & 0x2u64, 8);
            if c.cc_ne() { blk = 297; continue; }
            blk = 296; continue;
        }
        296 => {
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008551cb); };
            c.w64(7, t415);
            blk = 298; continue;
        }
        297 => {
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008551d4); };
            c.w64(7, t416);
            blk = 298; continue;
        }
        298 => {
            let Some(t417) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008551db); };
            c.logic(t417 & 0x4u64, 8);
            if c.cc_e() { blk = 327; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t418) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008551e5); };
            c.w32(8, t418);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t419) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008551ec); };
            c.w32(9, t419);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            return RegfileExit::fault(0x70008551f9);
        }
        301 => {
            let Some(t420) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008551fa); };
            c.w32(11, t420);
            let r421 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r421);
            c.w64(12, c.r64(11));
            let r422 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r422);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r423 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r423);
            c.logic(r423, 64);
            let r424 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r424);
            c.w32(3, c.r32(9));
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 302; continue;
        }
        302 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 304; continue; }
            blk = 303; continue;
        }
        303 => {
            return RegfileExit::fault(0x7000855224);
        }
        304 => {
            let r425 = c.sub(c.r32(3), c.r32(11), 32);
            c.w32(3, r425);
            let r426 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r426);
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t427) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085522d); };
            c.w32(12, t427);
            let r428 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r428);
            c.logic(r428, 64);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 305; continue;
        }
        305 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_a() { blk = 307; continue; }
            blk = 306; continue;
        }
        306 => {
            return RegfileExit::fault(0x7000855244);
        }
        307 => {
            let Some(t429) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000855245); };
            c.w32(8, t429);
            let r430 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r430);
            c.w32(9, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(9), 0xfffffeu64, 32);
            if c.cc_a() { blk = 360; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x7000855260); }
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855264); };
            c.w32(9, t431);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 310; continue; }
            blk = 309; continue;
        }
        309 => {
            return RegfileExit::fault(0x7000855274);
        }
        310 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000855278); }
            let Some(t432) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085527c); };
            c.w32(9, t432);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r433 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r433);
            let Some(t434) = c.ld64(c.r64(13).wrapping_add(0x6b0)) else { return RegfileExit::fault(0x700085528a); };
            c.w64(11, t434);
            let Some(t435) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855291); };
            c.w32(12, t435);
            let r436 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r436);
            let r437 = c.r64(12) & c.r64(9);
            c.w64(12, r437);
            c.logic(r437, 64);
            c.w64(3, c.r64(15));
            blk = 311; continue;
        }
        311 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(2).wrapping_mul(4));
            let Some(t438) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008552ac); };
            c.w32(6, t438);
            let Some(t439) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008552b0); };
            c.cmp(t439, c.r32(6), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t440) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008552b9); };
            c.cmp(t440, c.r32(6), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 313; continue;
        }
        313 => {
            let r441 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r441);
            let r442 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r442);
            c.w64(2, c.r64(9));
            let r443 = c.r64(2) & c.r64(12);
            c.w64(2, r443);
            c.logic(r443, 64);
            c.w64(12, c.r64(2));
            blk = 311; continue;
        }
        314 => {
            let Some(t444) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008552d4); };
            c.w32(9, t444);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 316; continue; }
            blk = 315; continue;
        }
        315 => {
            return RegfileExit::fault(0x70008552e3);
        }
        316 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008552e4); }
            let Some(t445) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008552e9); };
            c.w32(9, t445);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 318; continue; }
            blk = 317; continue;
        }
        317 => {
            return RegfileExit::fault(0x70008552fc);
        }
        318 => {
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            c.w32(9, c.r32(8));
            c.w64(12, c.r64(2).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(0).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x700085530c); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 320; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r446 = c.r64(3) & c.r64(0);
            c.w64(3, r446);
            c.logic(r446, 64);
            let Some(t447) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000855320); };
            c.logic(t447 & 0x4u64, 8);
            if c.cc_ne() { blk = 400; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t448) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085532a); };
            c.w32(7, t448);
            let Some(t449) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700085532e); };
            c.cmp(t449, c.r32(7), 32);
            if c.cc_ne() { blk = 323; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t450) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855337); };
            c.logic(t450 & 0x1u64, 8);
            if c.cc_e() { blk = 323; continue; }
            blk = 322; continue;
        }
        322 => {
            let r451 = c.r32(9) | 0x20u64;
            c.w32(9, r451);
            c.logic(r451, 32);
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 324; continue;
        }
        323 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 324; continue;
        }
        324 => {
            let Some(t452) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085534a); };
            c.w32(8, t452);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(2).wrapping_add(0x2));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 326; continue; }
            blk = 325; continue;
        }
        325 => {
            return RegfileExit::fault(0x700085535d);
        }
        326 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x700085535e); }
            blk = 327; continue;
        }
        327 => {
            let Some(t453) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000855362); };
            c.logic(t453 & 0x8u64, 8);
            if c.cc_ne() { blk = 329; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t454) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000855368); };
            c.w64(7, t454);
            blk = 330; continue;
        }
        329 => {
            let Some(t455) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000855371); };
            c.w64(7, t455);
            blk = 330; continue;
        }
        330 => {
            let Some(t456) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000855378); };
            c.logic(t456 & 0x10u64, 8);
            if c.cc_e() { blk = 359; continue; }
            blk = 331; continue;
        }
        331 => {
            let Some(t457) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855382); };
            c.w32(8, t457);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t458) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000855389); };
            c.w32(9, t458);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            return RegfileExit::fault(0x7000855396);
        }
        333 => {
            let Some(t459) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855397); };
            c.w32(11, t459);
            let r460 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r460);
            c.w64(12, c.r64(11));
            let r461 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r461);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r462 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r462);
            c.logic(r462, 64);
            let r463 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r463);
            c.w32(3, c.r32(9));
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 334; continue;
        }
        334 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 336; continue; }
            blk = 335; continue;
        }
        335 => {
            return RegfileExit::fault(0x70008553c1);
        }
        336 => {
            let r464 = c.sub(c.r32(3), c.r32(11), 32);
            c.w32(3, r464);
            let r465 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r465);
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t466) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008553ca); };
            c.w32(12, t466);
            let r467 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r467);
            c.logic(r467, 64);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_b() { blk = 360; continue; }
            blk = 337; continue;
        }
        337 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_a() { blk = 339; continue; }
            blk = 338; continue;
        }
        338 => {
            return RegfileExit::fault(0x70008553e1);
        }
        339 => {
            let Some(t468) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008553e2); };
            c.w32(8, t468);
            let r469 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r469);
            c.w32(9, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(9), 0xfffffeu64, 32);
            if c.cc_a() { blk = 360; continue; }
            blk = 340; continue;
        }
        340 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008553fd); }
            let Some(t470) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855401); };
            c.w32(9, t470);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 342; continue; }
            blk = 341; continue;
        }
        341 => {
            return RegfileExit::fault(0x7000855411);
        }
        342 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000855415); }
            let Some(t471) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000855419); };
            c.w32(9, t471);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r472 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r472);
            let Some(t473) = c.ld64(c.r64(13).wrapping_add(0x5f0)) else { return RegfileExit::fault(0x7000855427); };
            c.w64(11, t473);
            let Some(t474) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085542e); };
            c.w32(12, t474);
            let r475 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r475);
            let r476 = c.r64(12) & c.r64(9);
            c.w64(12, r476);
            c.logic(r476, 64);
            blk = 343; continue;
        }
        343 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            let Some(t477) = c.ld32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855446); };
            c.w32(1, t477);
            let Some(t478) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085544a); };
            c.cmp(t478, c.r32(1), 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 344; continue;
        }
        344 => {
            let Some(t479) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000855453); };
            c.cmp(t479, c.r32(1), 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 345; continue;
        }
        345 => {
            let r480 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r480);
            let r481 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r481);
            c.w64(3, c.r64(9));
            let r482 = c.r64(3) & c.r64(12);
            c.w64(3, r482);
            c.logic(r482, 64);
            c.w64(12, c.r64(3));
            blk = 343; continue;
        }
        346 => {
            let Some(t483) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085546e); };
            c.w32(9, t483);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_b() { blk = 348; continue; }
            blk = 347; continue;
        }
        347 => {
            return RegfileExit::fault(0x700085547d);
        }
        348 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700085547e); }
            let Some(t484) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855483); };
            c.w32(9, t484);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 350; continue; }
            blk = 349; continue;
        }
        349 => {
            return RegfileExit::fault(0x7000855496);
        }
        350 => {
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            c.w32(9, c.r32(8));
            c.w64(12, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(0).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x70008554a6); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 351; continue;
        }
        351 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r485 = c.r64(15) & c.r64(0);
            c.w64(15, r485);
            c.logic(r485, 64);
            let Some(t486) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008554ba); };
            c.logic(t486 & 0x4u64, 8);
            if c.cc_ne() { blk = 397; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t487) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008554c5); };
            c.w32(7, t487);
            let Some(t488) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008554c9); };
            c.cmp(t488, c.r32(7), 32);
            if c.cc_ne() { blk = 355; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t489) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008554d2); };
            c.logic(t489 & 0x1u64, 8);
            if c.cc_e() { blk = 355; continue; }
            blk = 354; continue;
        }
        354 => {
            let r490 = c.r32(9) | 0x20u64;
            c.w32(9, r490);
            c.logic(r490, 32);
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 356; continue;
        }
        355 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 356; continue;
        }
        356 => {
            let Some(t491) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008554e5); };
            c.w32(8, t491);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(3).wrapping_add(0x2));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 358; continue; }
            blk = 357; continue;
        }
        357 => {
            return RegfileExit::fault(0x70008554f8);
        }
        358 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008554f9); }
            blk = 359; continue;
        }
        359 => {
            let Some(t492) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008554fd); };
            c.w64(8, t492);
            c.w64(7, c.r64(8).wrapping_add(0x1));
            let Some(t493) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855505); };
            c.w64(0, t493);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085550c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085550d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000855513); }
            return c.ret(0x0, 0x7000855515);
        }
        360 => {
            return RegfileExit::fault(0x7000855516);
        }
        361 => {
            let Some(t494) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855517); };
            c.w64(8, t494);
            let Some(t495) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085551b); };
            c.w32(8, t495);
            let r496 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r496);
            let Some(t497) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000855522); };
            c.w32(8, t497);
            let r498 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r498);
            let Some(t499) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855529); };
            c.w32(9, t499);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t500) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855530); };
            c.w32(11, t500);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000855537); }
            c.cmp(c.r64(9), 0x3cu64, 64);
            if c.cc_a() { blk = 363; continue; }
            blk = 362; continue;
        }
        362 => {
            return RegfileExit::fault(0x7000855541);
        }
        363 => {
            let Some(t501) = c.ld32(c.r64(8).wrapping_add(0xf7)) else { return RegfileExit::fault(0x7000855542); };
            c.w32(8, t501);
            let r502 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r502);
            let Some(t503) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085554c); };
            c.w64(9, t503);
            let Some(t504) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000855553); };
            c.w64(12, t504);
            let Some(t505) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855556); };
            c.w32(15, t505);
            c.w64(2, c.r64(15).wrapping_mul(4));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000855567); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700085556b); }
            let Some(t506) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085556f); };
            c.w64(0, t506);
            let Some(t507) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000855576); };
            c.cmp(t507, c.r64(15), 64);
            if c.cc_a() { blk = 366; continue; }
            blk = 364; continue;
        }
        364 => {
            return c.call(0x70007febf4, 0x7000855580, 0x700085557b);
        }
        365 => {
            let Some(t508) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000855580); };
            c.w64(7, t508);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855584); };
            c.w64(11, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000855588); };
            c.w64(2, t510);
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085558c); };
            c.w64(8, t511);
            blk = 367; continue;
        }
        366 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000855592); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 367; continue;
        }
        367 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700085559d); }
            let Some(t512) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008555a1); };
            c.w64(8, t512);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008555a8); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008555ac); }
            c.w64(8, c.r64(2));
            let r513 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r513);
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 370; continue; }
            blk = 368; continue;
        }
        368 => {
            let Some(t514) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008555bd); };
            c.w64(9, t514);
            blk = 369; continue;
        }
        369 => {
            let r515 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r515);
            if !c.st32(c.r64(8).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x70008555c8); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 369; continue; }
            blk = 370; continue;
        }
        370 => {
            c.logic(c.r8(7) & 0x2u64, 8);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(7) & 0x8u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(7) & 0x20u64, 8);
            if c.cc_ne() { blk = 372; continue; }
            blk = 371; continue;
        }
        371 => {
            let Some(t516) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008555f0); };
            c.w64(7, t516);
            blk = 373; continue;
        }
        372 => {
            let Some(t517) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008555f9); };
            c.w64(7, t517);
            blk = 373; continue;
        }
        373 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 375; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t518) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000855606); };
            c.w64(8, t518);
            blk = 376; continue;
        }
        375 => {
            let Some(t519) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085560f); };
            c.w64(8, t519);
            blk = 376; continue;
        }
        376 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 378; continue; }
            blk = 377; continue;
        }
        377 => {
            let Some(t520) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085561c); };
            c.w64(9, t520);
            blk = 379; continue;
        }
        378 => {
            let Some(t521) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000855625); };
            c.w64(9, t521);
            blk = 379; continue;
        }
        379 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700085562c); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000855630); }
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x7000855633); }
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x7000855637); }
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085563b); };
            c.w64(9, t522);
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000855646); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000855647); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085564d); }
            return c.ret(0x0, 0x700085564f);
        }
        380 => {
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855650); };
            c.w64(8, t523);
            let Some(t524) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855654); };
            c.w32(8, t524);
            let r525 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r525);
            let Some(t526) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085565b); };
            c.w32(8, t526);
            let r527 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r527);
            let Some(t528) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855662); };
            c.w32(9, t528);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t529) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000855669); };
            c.w32(11, t529);
            let Some(t530) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085566d); };
            c.w32(12, t530);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000855674); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x7000855678); }
            c.cmp(c.r64(9), 0x26u64, 64);
            if c.cc_a() { blk = 382; continue; }
            blk = 381; continue;
        }
        381 => {
            return RegfileExit::fault(0x7000855682);
        }
        382 => {
            let Some(t531) = c.ld32(c.r64(8).wrapping_add(0x9f)) else { return RegfileExit::fault(0x7000855683); };
            c.w32(8, t531);
            let r532 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r532);
            let Some(t533) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085568d); };
            c.w64(9, t533);
            let Some(t534) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000855694); };
            c.w64(15, t534);
            let Some(t535) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855697); };
            c.w32(0, t535);
            c.w64(2, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008556a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008556ac); }
            let Some(t536) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008556b0); };
            c.w64(3, t536);
            let Some(t537) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008556b7); };
            c.cmp(t537, c.r64(0), 64);
            if c.cc_a() { blk = 385; continue; }
            blk = 383; continue;
        }
        383 => {
            return c.call(0x70007febf4, 0x70008556c1, 0x70008556bc);
        }
        384 => {
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008556c1); };
            c.w64(7, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008556c5); };
            c.w64(11, t539);
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008556c9); };
            c.w64(12, t540);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008556cd); };
            c.w64(2, t541);
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008556d1); };
            c.w64(8, t542);
            blk = 386; continue;
        }
        385 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x70008556d7); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 386; continue;
        }
        386 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008556e1); }
            let Some(t543) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008556e5); };
            c.w64(8, t543);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008556ec); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008556f0); }
            c.w64(8, c.r64(2));
            let r544 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r544);
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 389; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t545) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000855701); };
            c.w64(9, t545);
            blk = 388; continue;
        }
        388 => {
            let r546 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r546);
            if !c.st32(c.r64(8).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x700085570c); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 388; continue; }
            blk = 389; continue;
        }
        389 => {
            c.logic(c.r8(7) & 0x2u64, 8);
            if c.cc_ne() { blk = 391; continue; }
            blk = 390; continue;
        }
        390 => {
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085571c); };
            c.w64(8, t547);
            blk = 392; continue;
        }
        391 => {
            let Some(t548) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000855725); };
            c.w64(8, t548);
            blk = 392; continue;
        }
        392 => {
            c.logic(c.r8(7) & 0x8u64, 8);
            if c.cc_ne() { blk = 394; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t549) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000855732); };
            c.w64(7, t549);
            blk = 395; continue;
        }
        394 => {
            let Some(t550) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085573b); };
            c.w64(7, t550);
            blk = 395; continue;
        }
        395 => {
            c.w64(9, c.r64(12));
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000855745); }
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700085574c); }
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x7000855750); }
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x7000855754); }
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000855757); };
            c.w64(7, t551);
            let r552 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r552);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000855762); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000855763); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000855769); }
            return c.ret(0x0, 0x700085576b);
        }
        396 => {
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085576c); };
            c.w64(7, t553);
            let r554 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r554);
            let Some(t555) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000855774); };
            c.w64(0, t555);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085577e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085577f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000855785); }
            return c.ret(0x0, 0x7000855787);
        }
        397 => {
            c.w32(7, c.r32(7));
            let r556 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r556);
            c.w64(15, 0xfffffffffffc0000u64);
            let r557 = c.r64(15) & c.r64(7);
            c.w64(15, r557);
            c.logic(r557, 64);
            let Some(t558) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000855797); };
            c.logic(t558 & 0x2u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 398; continue;
        }
        398 => {
            c.w64(1, c.r64(0).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008557a6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008557a7); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008557b4, 0x70008557af);
        }
        399 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008557b4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008557b6); }
            blk = 352; continue;
        }
        400 => {
            c.w32(7, c.r32(7));
            let r559 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r559);
            c.w64(3, 0xfffffffffffc0000u64);
            let r560 = c.r64(3) & c.r64(7);
            c.w64(3, r560);
            c.logic(r560, 64);
            let Some(t561) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008557cb); };
            c.logic(t561 & 0x2u64, 8);
            if c.cc_e() { blk = 320; continue; }
            blk = 401; continue;
        }
        401 => {
            c.w64(6, c.r64(0).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008557d9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008557da); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008557e7, 0x70008557e2);
        }
        402 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008557e7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008557e9); }
            blk = 320; continue;
        }
        403 => {
            c.w32(7, c.r32(7));
            let r562 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r562);
            c.w64(3, 0xfffffffffffc0000u64);
            let r563 = c.r64(3) & c.r64(7);
            c.w64(3, r563);
            c.logic(r563, 64);
            let Some(t564) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008557fe); };
            c.logic(t564 & 0x2u64, 8);
            if c.cc_e() { blk = 288; continue; }
            blk = 404; continue;
        }
        404 => {
            c.w64(6, c.r64(0).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085580c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085580d); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x700085581a, 0x7000855815);
        }
        405 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085581a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700085581c); }
            blk = 288; continue;
        }
        406 => {
            c.w32(8, c.r32(8));
            let r565 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r565);
            c.w64(3, 0xfffffffffffc0000u64);
            let r566 = c.r64(3) & c.r64(8);
            c.w64(3, r566);
            c.logic(r566, 64);
            let Some(t567) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000855832); };
            c.logic(t567 & 0x2u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 407; continue;
        }
        407 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000855840); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000855841); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x700085584e, 0x7000855849);
        }
        408 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085584e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000855850); }
            blk = 259; continue;
        }
        409 => {
            c.w32(7, c.r32(7));
            let r568 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r568);
            c.w64(3, 0xfffffffffffc0000u64);
            let r569 = c.r64(3) & c.r64(7);
            c.w64(3, r569);
            c.logic(r569, 64);
            let Some(t570) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000855865); };
            c.logic(t570 & 0x2u64, 8);
            if c.cc_e() { blk = 230; continue; }
            blk = 410; continue;
        }
        410 => {
            c.w64(6, c.r64(0).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000855873); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000855874); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x7000855881, 0x700085587c);
        }
        411 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000855881); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000855883); }
            blk = 230; continue;
        }
        412 => {
            c.w32(9, c.r32(9));
            let r571 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r571);
            c.w64(2, 0xfffffffffffc0000u64);
            let r572 = c.r64(2) & c.r64(9);
            c.w64(2, r572);
            c.logic(r572, 64);
            let Some(t573) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000855899); };
            c.logic(t573 & 0x2u64, 8);
            if c.cc_e() { blk = 198; continue; }
            blk = 413; continue;
        }
        413 => {
            c.w64(1, c.r64(0).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008558a7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008558a8); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008558b5, 0x70008558b0);
        }
        414 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008558b5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008558b7); }
            blk = 198; continue;
        }
        415 => {
            return RegfileExit::jump(0x70008558c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008558d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a60)) else { return RegfileExit::fault(0x70008558d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008558f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008558f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008558f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008558f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008558fa); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008558ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000855909); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855916); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085591c); };
            c.cmp(t5, 0x411u64, 16);
            if c.cc_be() { blk = 25; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855928); };
            c.w32(8, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085592c); };
            c.cmp(t7, c.r32(8), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000855935); };
            c.cmp(t8, c.r32(8), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000855942); };
            c.w32(8, t9);
            c.w64(9, c.r64(8));
            let r10 = c.r64(9) & 0x3ffu64;
            c.w64(9, r10);
            c.logic(r10, 64);
            c.cmp(c.r64(9), 0x3ffu64, 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            if let Some(sr) = c.shr(c.r64(8), 0xau64, 64) { c.w64(8, sr); }
            let r11 = c.r64(8) & 0x3ffu64;
            c.w64(8, r11);
            c.logic(r11, 64);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 25; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 22; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085597b); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000855981); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000855987); };
            c.w32(7, t16);
            let r17 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085598d); };
            c.w32(8, t18);
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000855991); };
            c.w32(7, t19);
            let r20 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r20);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085599a); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70008559a9);
        }
        10 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008559aa); };
            c.w32(1, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008559b0); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008559b7); };
            c.w64(11, t25);
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0x18));
            c.w64(12, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008559c6); };
            c.w64(15, t26);
            let Some(t27) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008559cd); };
            c.cmp(t27, c.r64(12), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008559d6); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 12; continue;
        }
        12 => {
            c.w64(11, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let r28 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r28);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008559e8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008559ec); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008559f0); };
            c.w64(1, t29);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008559f7); }
            c.w64(1, c.r64(8).wrapping_add(0x10));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008559ff); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000855a03); };
            c.w64(12, t30);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000855a0a); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000855a0e); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let r31 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r31);
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000855a1c); };
            c.w32(9, t32);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x7000855a20); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        15 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000855a2d); };
            c.w64(1, t33);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000855a34); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return c.ret(0x10, 0x7000855a3b);
        }
        17 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000855a3e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000855a45); }
            return c.ret(0x0, 0x7000855a47);
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000855a48); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000855a4c); };
            c.w64(3, t34);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000855a5a, 0x7000855a55);
        }
        19 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000855a5a); };
            c.w64(6, t35);
            blk = 1; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000855a63); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000855a67); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000855a6b); }
            return c.call(0x70007febf4, 0x7000855a74, 0x7000855a6f);
        }
        21 => {
            c.w64(8, c.r64(0));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000855a77); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855a7b); };
            c.w64(1, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000855a7f); };
            c.w64(9, t38);
            blk = 12; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000855a88); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000855a8c); };
            c.w64(0, t39);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            blk = 27; continue;
        }
        23 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000855a9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000855a9b); }
            c.w32(0, 0x1u64);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2430)) else { return RegfileExit::fault(0x7000855aa4); };
            c.w64(3, t40);
            return c.call(0x700086ed74, 0x7000855ab0, 0x7000855aab);
        }
        24 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855ab0); };
            c.w32(7, t41);
            blk = 27; continue;
        }
        25 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000855ab5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000855ab6); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2428)) else { return RegfileExit::fault(0x7000855aba); };
            c.w64(3, t42);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000855acb, 0x7000855ac6);
        }
        26 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855acb); };
            c.w32(7, t43);
            blk = 27; continue;
        }
        27 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000855ace); };
            c.w64(6, t44);
            let Some(t45) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855ad2); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000855ad8); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855ade); };
            c.w32(8, t49);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000855ae8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000855aec); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x7000855af6);
        }
        29 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000855af7); };
            c.w32(1, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000855afd); };
            c.w64(8, t51);
            let Some(t52) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000855b04); };
            c.w64(9, t52);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000855b0b); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000855b0f); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000855b16); };
            c.cmp(t54, c.r64(11), 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000855b26, 0x7000855b21);
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000855b28); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 33; continue;
        }
        33 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000855b32); };
            c.w64(1, t55);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000855b36); }
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855b39); };
            c.w64(7, t56);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000855b3d); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000855b40); };
            c.w64(1, t57);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000855b47); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000855b4a); };
            c.w64(1, t58);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000855b4e); }
            blk = 15; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000855b58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000855b74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a68)) else { return RegfileExit::fault(0x7000855b74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000855b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000855b94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000855b98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000855b99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000855b9a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000855b9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000855ba5); };
            c.w64(1, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000855ba9); };
            c.cmp(t3, c.r32(1), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000855bb8); };
            c.cmp(t4, c.r32(1), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r5 = c.r32(8) | c.r32(9);
            c.w32(8, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000855bcc); };
            if !c.push64(t6) { return RegfileExit::fault(0x7000855bcc); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000855bcf); }
            c.w32(0, 0x2u64);
            return c.call(0x70008560b4, 0x7000855bda, 0x7000855bd5);
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000855bda); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000855be1); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x18, 0x7000855be8);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000855beb); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000855bf2); }
            return c.ret(0x0, 0x7000855bf4);
        }
        6 => {
            c.w32(1, 0x16cu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000855bfa); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000855bfb); };
            c.w64(3, t8);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000855c0c, 0x7000855c07);
        }
        7 => {
            return RegfileExit::fault(0x7000855c0c);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000855c0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000855c11); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000855c15); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000855c19); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000855c27, 0x7000855c22);
        }
        9 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000855c27); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855c2b); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000855c2f); };
            c.w64(2, t12);
            blk = 1; continue;
        }
        10 => {
            return RegfileExit::jump(0x7000855c3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000855c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000855c54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000855c58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000855c59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000855c5a); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000855c5b); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000855c65); };
            c.w64(0, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000855c69); };
            c.w64(2, t2);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855c8f); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000855c95); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855c9e); };
            c.w32(1, t6);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 30; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855cb2); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855cb8); };
            c.cmp(t9, 0x41u64, 16);
            if c.cc_ne() { blk = 30; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000855cc3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000855cc4); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000855cc7); };
            c.w64(3, t10);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000855cd8, 0x7000855cd3);
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000855cd8); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 30; continue;
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855cea); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855cf0); };
            c.cmp(t14, 0x40u64, 16);
            if c.cc_ae() { blk = 30; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x7000855d02, 0x7000855cfd);
        }
        13 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000855d02); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 30; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855d14); };
            c.w32(1, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000855d17); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000855d24); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000855d29); }
            blk = 22; continue;
        }
        17 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855d30); };
            c.w32(1, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000855d33); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 18; continue;
        }
        18 => {
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000855d42); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 22; continue;
        }
        19 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855d56); };
            c.w32(1, t21);
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000855d59); };
            c.cmp(t22, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            let r23 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r23);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000855d64); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 22; continue;
        }
        22 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            if c.cc_e() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            if c.cc_e() { blk = 30; continue; }
            blk = 26; continue;
        }
        26 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 34; continue; }
            blk = 27; continue;
        }
        27 => {
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 34; continue;
        }
        29 => {
            c.movq_rx(1, 1);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000855da1); };
            c.w64(0, t24);
            blk = 31; continue;
        }
        31 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000855da8); };
            c.w64(1, t25);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000855daf); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return c.ret(0x18, 0x7000855db6);
        }
        33 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000855db9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000855dc0); }
            return c.ret(0x0, 0x7000855dc2);
        }
        34 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000855dc3); };
            c.w64(0, t26);
            blk = 31; continue;
        }
        35 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000855dcc); };
            c.w64(3, t27);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000855dda, 0x7000855dd5);
        }
        36 => {
            blk = 1; continue;
        }
        37 => {
            return RegfileExit::jump(0x7000855de0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000855df4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a70)) else { return RegfileExit::fault(0x7000855df4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000855e14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a78)) else { return RegfileExit::fault(0x7000855e14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000855e34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000855e34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000855e38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000855e39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000855e3a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000855e3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000855e49); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855e56); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000855e5c); };
            c.w32(8, t5);
            let r6 = c.r64(8) & 0x3ffu64;
            c.w64(8, r6);
            c.logic(r6, 64);
            c.cmp(c.r64(8), 0x3ffu64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000855e74); };
            c.w32(9, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000855e78); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000855e81); };
            c.cmp(t9, c.r32(9), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855e98); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000855e9e); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000855ea4); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855eaa); };
            c.w32(9, t16);
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000855eae); };
            c.w32(7, t17);
            let r18 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r18);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855eb7); };
            c.w32(7, t19);
            let r20 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r20);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x7000855ec6);
        }
        8 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000855ec7); };
            c.w32(1, t21);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000855ecd); };
            c.w64(9, t22);
            let Some(t23) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000855ed4); };
            c.w64(11, t23);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x18));
            c.w64(12, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000855ee3); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000855eea); };
            c.cmp(t25, c.r64(12), 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000855ef3); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            c.w64(11, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let r26 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r26);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000855f05); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000855f09); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000855f0d); };
            c.w64(1, t27);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000855f14); }
            c.w64(1, c.r64(9).wrapping_add(0x10));
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000855f1c); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000855f20); };
            c.w64(12, t28);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000855f27); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000855f2b); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let r29 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r29);
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000855f39); };
            c.w32(8, t30);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x7000855f3d); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 11; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, c.r64(9));
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000855f4a); };
            c.w64(1, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000855f51); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return c.ret(0x10, 0x7000855f58);
        }
        15 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000855f5b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000855f62); }
            return c.ret(0x0, 0x7000855f64);
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000855f65); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000855f69); };
            c.w64(3, t32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000855f77, 0x7000855f72);
        }
        17 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000855f77); };
            c.w64(6, t33);
            blk = 1; continue;
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000855f80); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000855f84); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000855f88); }
            return c.call(0x70007febf4, 0x7000855f91, 0x7000855f8c);
        }
        19 => {
            c.w64(9, c.r64(0));
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000855f94); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000855f98); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000855f9c); };
            c.w64(8, t36);
            blk = 10; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000855fa5); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000855fa9); };
            c.w64(1, t37);
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        21 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000855fb4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000855fb5); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2448)) else { return RegfileExit::fault(0x7000855fb9); };
            c.w64(3, t38);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000855fca, 0x7000855fc5);
        }
        22 => {
            let Some(t39) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855fca); };
            c.w32(7, t39);
            c.w64(1, c.r64(0));
            blk = 23; continue;
        }
        23 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000855fd0); };
            c.w64(6, t40);
            let Some(t41) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000855fd4); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            let Some(t43) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000855fdb); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            let Some(t45) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000855fe2); };
            c.w32(9, t45);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000855fec); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000855ff0); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::fault(0x7000855ffa);
        }
        25 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000855ffb); };
            c.w32(8, t46);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000856002); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000856009); };
            c.w64(11, t48);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000856010); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000856014); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700085601b); };
            c.cmp(t50, c.r64(12), 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700085602a, 0x7000856025);
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700085602c); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 29; continue;
        }
        29 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000856036); };
            c.w64(1, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700085603a); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085603d); };
            c.w64(7, t52);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000856041); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000856044); };
            c.w64(1, t53);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700085604b); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085604e); };
            c.w64(1, t54);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000856052); }
            blk = 13; continue;
        }
        30 => {
            return RegfileExit::jump(0x700085605c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000856074(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a80)) else { return RegfileExit::fault(0x7000856074); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000856094(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a88)) else { return RegfileExit::fault(0x7000856094); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008560b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008560b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008560b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008560b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008560ba); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008560bf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008560c3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 202; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008560cd); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008560dc); };
            c.w64(8, t3);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 196; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008560ea); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008560f1); };
            c.w32(11, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008560f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008560fa); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856107); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085610d); };
            c.w32(7, t9);
            c.cmp(c.r16(7), 0x42u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000856117); }
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
            let r10 = c.add(c.r64(15), c.r64(12), 64);
            c.w64(15, r10);
            c.w64(0, 0x3ffffffffffffeu64);
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_be() { blk = 67; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_ae() { blk = 204; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856166); };
            c.w32(12, t11);
            c.logic(c.r32(12) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 199; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r16(7), 0x25u64, 16);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(7), 0x2du64, 16);
            if c.cc_e() { blk = 24; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(7) & 0x20u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(7, c.r64(2));
            blk = 27; continue;
        }
        15 => {
            c.w64(10, c.r64(4));
            let r12 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r12);
            let r13 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r13);
            c.logic(r13, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008561ae); }
            c.w64(1, c.r64(13));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x70008561b6); };
            c.w64(0, t14);
            c.w64(10, 0x70008561c4u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008561c4); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008561c8); }
            return c.call(c.r64(0), 0x70008561ce, 0x70008561cc);
        }
        16 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008561ce); }
            let Some(t15) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008561d6); };
            c.w64(4, t15);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(7), 0xfffffffffffffffeu64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(12, c.r64(7));
            let Some(t16) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008561f4); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008561f8); };
            c.w64(2, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008561fc); };
            c.w64(6, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000856200); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000856204); };
            c.w64(11, t20);
            blk = 67; continue;
        }
        20 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085620d); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000856211); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000856215); };
            c.w64(6, t23);
            blk = 199; continue;
        }
        21 => {
            let Some(t24) = c.ld16(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085621e); };
            c.cmp(t24, 0x410u64, 16);
            if c.cc_a() { blk = 201; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085622a); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085622e); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000856232); };
            c.w64(6, t27);
            blk = 199; continue;
        }
        23 => {
            c.w64(7, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085623e); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000856242); };
            c.w64(2, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000856246); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085624a); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085624e); };
            c.w64(11, t32);
            blk = 27; continue;
        }
        24 => {
            c.w64(15, c.r64(2));
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000856257); };
            c.w32(7, t33);
            let r34 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r34);
            c.w64(2, c.r64(15));
            blk = 27; continue;
        }
        25 => {
            if let Some(sr) = c.shr(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let r35 = c.r64(12) & 0xffffffu64;
            c.w64(12, r35);
            c.logic(r35, 64);
            blk = 67; continue;
        }
        26 => {
            c.w64(7, c.r64(2));
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t36) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000856282); };
            c.w32(11, t36);
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000856293); };
            c.w32(8, t37);
            let r38 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r38);
            c.logic(c.r32(11) & 0xffc00u64, 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 30; continue;
        }
        30 => {
            let r39 = c.shr(c.r32(11), 0xau64, 32).unwrap_or(c.r32(11));
            c.w32(11, r39);
            let r40 = c.r32(11) & 0x3ffu64;
            c.w32(11, r40);
            c.logic(r40, 32);
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_be() { blk = 43; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t41) = c.ld16(c.r64(8).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008562bc); };
            c.w32(9, (t41 as u16 as i16 as i64 as u64));
            let Some(t42) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008562c1); };
            c.w32(12, t42);
            let r43 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r43);
            let r44 = c.sub(c.r32(9), 0x1u64, 32);
            c.w32(9, r44);
            c.w64(15, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 33; continue;
        }
        32 => {
            c.w64(15, c.r64(3));
            blk = 33; continue;
        }
        33 => {
            c.w64(3, c.r64(15));
            let r45 = c.sub(c.r32(3), c.r32(0), 32);
            c.w32(3, r45);
            let r46 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r46);
            let r47 = c.add(c.r32(3), c.r32(0), 32);
            c.w32(3, r47);
            c.w32(2, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t48) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008562e6); };
            c.w32(2, t48);
            let r49 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r49);
            let r50 = c.shr(c.r32(2), 0x9u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r50);
            let r51 = c.r32(2) & 0x3ffu64;
            c.w32(2, r51);
            c.logic(r51, 32);
            c.w32(2, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008562fc); };
            c.w32(2, t52);
            let r53 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r53);
            let Some(t54) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856304); };
            c.w32(2, t54);
            let r55 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r55);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_be() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(0, c.r64(3).wrapping_add(0x1));
            c.w64(3, c.r64(15));
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 38; continue;
        }
        37 => {
            let r56 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r56);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_l() { blk = 201; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(15, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085632e); };
            c.w32(15, t57);
            let r58 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r58);
            let r59 = c.shr(c.r32(15), 0x9u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r59);
            let r60 = c.r32(15) & 0x3ffu64;
            c.w32(15, r60);
            c.logic(r60, 32);
            c.w32(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856348); };
            c.w32(3, t61);
            let r62 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r62);
            let Some(t63) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856350); };
            c.w32(2, t63);
            let r64 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r64);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(3), c.r32(7), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_be() { blk = 201; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 192; continue;
        }
        43 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            let r65 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r65);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 201; continue; }
            blk = 44; continue;
        }
        44 => {
            let r66 = c.sub(c.r64(9), 0x3u64, 64);
            c.w64(9, r66);
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856386); };
            c.cmp(t67, c.r32(7), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ne() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            blk = 201; continue;
        }
        47 => {
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085639c); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008563a9); };
            c.w64(8, t70);
            blk = 49; continue;
        }
        49 => {
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008563b0); };
            c.w32(9, t71);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r72 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008563be); };
            c.w32(11, t73);
            let r74 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r74);
            let r75 = c.r64(11) & c.r64(9);
            c.w64(11, r75);
            c.logic(r75, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 51; continue;
        }
        50 => {
            let r76 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r76);
            let r77 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r77);
            c.w64(12, c.r64(9));
            let r78 = c.r64(12) & c.r64(11);
            c.w64(12, r78);
            c.logic(r78, 64);
            c.w64(11, c.r64(12));
            blk = 51; continue;
        }
        51 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008563e3); };
            c.w32(12, t79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008563e8); };
            c.cmp(t80, c.r32(12), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 50; continue;
        }
        54 => {
            c.cmp(c.r16(11), 0xbdu64, 16);
            if c.cc_ne() { blk = 199; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085640c); };
            c.logic(t81 & 0x24u64, 8);
            if c.cc_ne() { blk = 199; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856417); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000856424); };
            c.w64(8, t84);
            blk = 58; continue;
        }
        58 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085642b); };
            c.w32(9, t85);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r86 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r86);
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856439); };
            c.w32(11, t87);
            let r88 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r88);
            let r89 = c.r64(11) & c.r64(9);
            c.w64(11, r89);
            c.logic(r89, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 60; continue;
        }
        59 => {
            let r90 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r90);
            let r91 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r91);
            c.w64(12, c.r64(9));
            let r92 = c.r64(12) & c.r64(11);
            c.w64(12, r92);
            c.logic(r92, 64);
            c.w64(11, c.r64(12));
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(11).wrapping_add(0x5));
            let Some(t93) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085645d); };
            c.w32(0, t93);
            let r94 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000856465); };
            c.cmp(t95, c.r32(0), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000856472); };
            c.cmp(t96, c.r32(0), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t97) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085647b); };
            c.cmp(t97, c.r32(7), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856480); };
            c.w32(7, t98);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_ae() { blk = 206; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t99) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000856493); };
            c.w32(7, t99);
            let Some(t100) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000856496); };
            c.cmp(t100, c.r32(7), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 65; continue;
        }
        65 => {
            blk = 192; continue;
        }
        66 => {
            c.w64(7, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(12, c.r64(7));
            blk = 67; continue;
        }
        67 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008564b1); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 199; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t101) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008564c1); };
            c.w32(7, t101);
            let r102 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r102);
            let r103 = c.r32(7) & 0x3fu64;
            c.w32(7, r103);
            c.logic(r103, 32);
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_l() { blk = 103; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_l() { blk = 87; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_l() { blk = 79; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_l() { blk = 76; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(7), 0x26u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(7), 0x28u64, 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 199; continue;
        }
        76 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 199; continue;
        }
        79 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_l() { blk = 84; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(7), 0x22u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 199; continue;
        }
        84 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(7), 0x1fu64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 86; continue;
        }
        86 => {
            blk = 199; continue;
        }
        87 => {
            c.cmp(c.r32(7), 0x19u64, 32);
            if c.cc_l() { blk = 96; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(7), 0x1bu64, 32);
            if c.cc_l() { blk = 93; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(7), 0x1bu64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(7), 0x1du64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 199; continue;
        }
        93 => {
            c.cmp(c.r32(7), 0x19u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(7), 0x1au64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 95; continue;
        }
        95 => {
            blk = 199; continue;
        }
        96 => {
            c.cmp(c.r32(7), 0x17u64, 32);
            if c.cc_l() { blk = 100; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(7), 0x17u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(7), 0x18u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 99; continue;
        }
        99 => {
            blk = 199; continue;
        }
        100 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 199; continue;
        }
        103 => {
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_l() { blk = 121; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(7), 0x10u64, 32);
            if c.cc_l() { blk = 113; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_l() { blk = 110; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 109; continue;
        }
        109 => {
            blk = 199; continue;
        }
        110 => {
            c.cmp(c.r32(7), 0x10u64, 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 112; continue;
        }
        112 => {
            blk = 199; continue;
        }
        113 => {
            c.cmp(c.r32(7), 0xbu64, 32);
            if c.cc_l() { blk = 118; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(7), 0xbu64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(7), 0xcu64, 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(7), 0xfu64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 117; continue;
        }
        117 => {
            blk = 199; continue;
        }
        118 => {
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(7), 0xau64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 199; continue;
        }
        121 => {
            c.cmp(c.r32(7), 0x4u64, 32);
            if c.cc_l() { blk = 130; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(7), 0x6u64, 32);
            if c.cc_l() { blk = 127; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(7), 0x6u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(7), 0x7u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 199; continue;
        }
        127 => {
            c.cmp(c.r32(7), 0x4u64, 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 129; continue;
        }
        129 => {
            blk = 199; continue;
        }
        130 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_l() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 133; continue;
        }
        133 => {
            blk = 199; continue;
        }
        134 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 136; continue;
        }
        136 => {
            blk = 199; continue;
        }
        137 => {
            blk = 199; continue;
        }
        138 => {
            let Some(t104) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008566b6); };
            c.w32(7, t104);
            let r105 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r105);
            let Some(t106) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008566bd); };
            c.w32(9, t106);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ae() { blk = 143; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(8, 0xfff7fffffff7ffffu64);
            let Some(t107) = c.ld64(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008566d6); };
            c.cmp(t107, c.r64(8), 64);
            if c.cc_e() { blk = 201; continue; }
            blk = 140; continue;
        }
        140 => {
            blk = 192; continue;
        }
        141 => {
            let Some(t108) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008566e6); };
            c.w32(7, t108);
            let r109 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r109);
            let Some(t110) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008566ed); };
            c.w32(7, t110);
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_b() { blk = 192; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008566f9); };
            c.w32(7, t111);
            let r112 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r112);
            let Some(t113) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856700); };
            c.w32(9, t113);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 145; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w32(7, 0xfffffffeu64);
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_b() { blk = 199; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 201; continue;
        }
        145 => {
            let Some(t114) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856722); };
            c.w32(7, t114);
            let Some(t115) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000856727); };
            c.cmp(t115, c.r32(7), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 146; continue;
        }
        146 => {
            blk = 192; continue;
        }
        147 => {
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000856739); };
            c.w32(7, t116);
            let r117 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r117);
            let Some(t118) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856740); };
            c.w32(7, t118);
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_b() { blk = 192; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w32(7, 0xfffffffeu64);
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_b() { blk = 199; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t119) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085675a); };
            c.w32(7, t119);
            let r120 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r120);
            let Some(t121) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856761); };
            c.w32(8, t121);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r122 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r122);
            c.w32(2, c.r32(12));
            c.w64(10, c.r64(4));
            let r123 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r123);
            let r124 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r124);
            c.logic(r124, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085677d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000856782); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000856786); }
            c.w64(1, c.r64(13));
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700085678d); };
            c.w64(0, t125);
            c.w64(10, 0x700085679bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085679b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085679f); }
            return c.call(c.r64(0), 0x70008567a5, 0x70008567a3);
        }
        150 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008567a5); }
            let Some(t126) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008567ad); };
            c.w64(4, t126);
            c.w32(7, c.r32(0));
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008567b4); };
            let r128 = c.r64(7) & t127;
            c.w64(7, r128);
            c.logic(r128, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008567bb); };
            c.w64(8, t129);
            blk = 152; continue;
        }
        151 => {
            let r130 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r130);
            let r131 = c.add(c.r64(7), c.r64(15), 64);
            c.w64(7, r131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008567c8); };
            c.w64(9, t132);
            let r133 = c.r64(9) & c.r64(7);
            c.w64(9, r133);
            c.logic(r133, 64);
            c.w64(7, c.r64(9));
            blk = 152; continue;
        }
        152 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t134) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008567d7); };
            c.w32(9, t134);
            let r135 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r135);
            let Some(t136) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008567df); };
            c.cmp(t136, c.r32(9), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 153; continue;
        }
        153 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008567f2); };
            c.cmp(t137, c.r32(9), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 155; continue;
        }
        155 => {
            c.xorpd_rr(0, 0);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008567ff); };
            c.cvtsi2sd(0, (t138 as i64));
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000856805); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 151; continue; }
            blk = 156; continue;
        }
        156 => {
            if c.cc_e() { blk = 192; continue; }
            blk = 157; continue;
        }
        157 => {
            blk = 151; continue;
        }
        158 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085681f); };
            c.cmp(c.r64(9), t139, 64);
            if c.cc_e() { blk = 192; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 151; continue;
        }
        160 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085682b); };
            c.w32(7, t140);
            let r141 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r141);
            let Some(t142) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000856832); };
            c.logic(t142 & 0x4u64, 8);
            if c.cc_ne() { blk = 201; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t143) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085683c); };
            c.cmp(t143, c.r64(12), 64);
            if c.cc_a() { blk = 192; continue; }
            blk = 162; continue;
        }
        162 => {
            blk = 201; continue;
        }
        163 => {
            let Some(t144) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085684b); };
            c.w32(7, t144);
            let r145 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r145);
            let Some(t146) = c.ld32(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000856852); };
            c.w32(9, t146);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 189; continue; }
            blk = 164; continue;
        }
        164 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_ne() { blk = 201; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t147) = c.ld64(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085686a); };
            c.w64(7, t147);
            let Some(t148) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085686e); };
            c.w64(9, t148);
            let Some(t149) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000856872); };
            c.logic(t149 & 0x1u64, 8);
            if c.cc_ne() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t150) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000856879); };
            c.w64(11, t150);
            let r151 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r151);
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_a() { blk = 201; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t152) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000856889); };
            c.w64(7, t152);
            c.w64(0, c.r64(7));
            blk = 191; continue;
        }
        168 => {
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_a() { blk = 201; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t153) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085689e); };
            c.w32(8, t153);
            let r154 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r154);
            let Some(t155) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008568a5); };
            c.w32(8, t155);
            let r156 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r156);
            let r157 = c.r32(8) & 0x3fu64;
            c.w32(8, r157);
            c.logic(r157, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 177; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 174; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 181; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 180; continue;
        }
        180 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 181; continue;
        }
        181 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            return RegfileExit::fault(0x700085690a);
        }
        184 => {
            c.w32(8, 0x1u64);
            blk = 188; continue;
        }
        185 => {
            c.w32(8, 0x2u64);
            blk = 188; continue;
        }
        186 => {
            c.w32(8, 0x4u64);
            blk = 188; continue;
        }
        187 => {
            c.w32(8, 0x8u64);
            blk = 188; continue;
        }
        188 => {
            let r158 = c.sub(c.r64(7), c.r64(9), 64);
            c.w64(7, r158);
            c.w64(0, c.r64(7));
            c.cqo();
            c.idiv(c.r64(8), 64);
            blk = 191; continue;
        }
        189 => {
            c.w64(10, c.r64(4));
            let r159 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r159);
            let r160 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r160);
            c.logic(r160, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000856941); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700085694c); };
            c.w64(0, t161);
            c.w64(10, 0x700085695au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085695a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085695e); }
            return c.call(c.r64(0), 0x7000856964, 0x7000856962);
        }
        190 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000856964); }
            let Some(t162) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085696c); };
            c.w64(4, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000856971); };
            c.w64(12, t163);
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_ae() { blk = 201; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085697a); };
            c.w64(0, t164);
            blk = 193; continue;
        }
        193 => {
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000856981); };
            c.w64(1, t165);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000856988); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            return c.ret(0x10, 0x700085698f);
        }
        195 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000856992); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000856999); }
            return c.ret(0x0, 0x700085699b);
        }
        196 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t166) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008569a1); };
            c.w32(7, t166);
            let r167 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008569a7); };
            c.cmp(t168, c.r32(7), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t169) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008569b0); };
            c.cmp(t169, 0x40u64, 16);
            if c.cc_be() { blk = 201; continue; }
            blk = 199; continue;
        }
        199 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008569b7); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008569b9); }
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x2438)) else { return RegfileExit::fault(0x70008569ba); };
            c.w64(3, t170);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008569cb, 0x70008569c6);
        }
        200 => {
            blk = 193; continue;
        }
        201 => {
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008569cd); };
            c.w64(0, t171);
            blk = 193; continue;
        }
        202 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008569d8); };
            c.w64(3, t172);
            return c.call(0x700086ed74, 0x70008569e4, 0x70008569df);
        }
        203 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008569e4); };
            c.w64(6, t173);
            blk = 1; continue;
        }
        204 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 199; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t174) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008569f3); };
            c.w32(7, t174);
            let r175 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r175);
            blk = 27; continue;
        }
        206 => {
            return RegfileExit::fault(0x70008569fe);
        }
        207 => {
            return RegfileExit::jump(0x7000856a00);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000856a14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000856a14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000856a18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000856a19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000856a1a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000856a1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000856a29); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000856a36); };
            c.w64(0, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000856a3a); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000856a47); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856a54); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            blk = 6; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856a5c); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856a62); };
            c.w32(7, t10);
            c.cmp(c.r16(7), 0x410u64, 16);
            if c.cc_be() { blk = 16; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856a6d); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000856a73); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000856a80); };
            c.w64(0, t14);
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000856a87); };
            c.w64(1, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000856a8e); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x10, 0x7000856a95);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000856a98); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000856a9f); }
            return c.ret(0x0, 0x7000856aa1);
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000856aa2); };
            c.w64(0, t16);
            blk = 10; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000856aab); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000856aaf); };
            c.w64(3, t17);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000856abd, 0x7000856ab8);
        }
        15 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000856abd); };
            c.w64(6, t18);
            blk = 1; continue;
        }
        16 => {
            c.cmp(c.r16(7), 0xbcu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t19) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000856acd); };
            c.logic(t19 & 0x24u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000856ad3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000856ad4); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x23b0)) else { return RegfileExit::fault(0x7000856ad5); };
            c.w64(3, t20);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000856ae6, 0x7000856ae1);
        }
        19 => {
            blk = 10; continue;
        }
        20 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856ae8); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            let Some(t23) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856aee); };
            c.cmp(t23, 0xbau64, 16);
            if c.cc_b() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            return c.call(0x70008a2054, 0x7000856afb, 0x7000856af6);
        }
        22 => {
            return RegfileExit::fault(0x7000856afb);
        }
        23 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000856afc); };
            c.w64(0, t24);
            blk = 10; continue;
        }
        24 => {
            return RegfileExit::jump(0x7000856b08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000856b14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a90)) else { return RegfileExit::fault(0x7000856b14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000856b34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a98)) else { return RegfileExit::fault(0x7000856b34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000856b54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1aa0)) else { return RegfileExit::fault(0x7000856b54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000856b74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1aa8)) else { return RegfileExit::fault(0x7000856b74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000856b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856b9c); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856ba2); };
            c.w32(1, t2);
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_be() { blk = 28; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(7, c.r64(1).wrapping_sub(0x80b));
            c.cmp(c.r32(7), 0xfu64, 32);
            if c.cc_be() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), 0x833u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(1), 0x839u64, 32);
            if c.cc_l() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), 0x839u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(1), 0x83au64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(1), 0x83eu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 25; continue;
        }
        9 => {
            c.cmp(c.r32(1), 0x833u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(1), 0x834u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 25; continue;
        }
        12 => {
            c.cmp(c.r32(1), 0x411u64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(1), 0x411u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(1), 0x421u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(1), 0x422u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 25; continue;
        }
        17 => {
            c.cmp(c.r32(1), 0xbcu64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(1), 0x410u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 25; continue;
        }
        20 => {
            blk = 25; continue;
        }
        21 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x508)) else { return RegfileExit::fault(0x7000856c4f); };
            c.w64(2, t3);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        22 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x9d0)) else { return RegfileExit::fault(0x7000856c5e); };
            c.w64(2, t4);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        23 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x4f8)) else { return RegfileExit::fault(0x7000856c6d); };
            c.w64(2, t5);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        24 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x620)) else { return RegfileExit::fault(0x7000856c7c); };
            c.w64(2, t6);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        25 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x920)) else { return RegfileExit::fault(0x7000856c8b); };
            c.w64(2, t7);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        26 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x6d0)) else { return RegfileExit::fault(0x7000856c9a); };
            c.w64(2, t8);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        27 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x738)) else { return RegfileExit::fault(0x7000856ca9); };
            c.w64(2, t9);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        28 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 39; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000856ccc); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000856cd5); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000856ce2); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000856cef); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x8f0)) else { return RegfileExit::fault(0x7000856cf8); };
            c.w64(0, t14);
            return c.ret(0x0, 0x7000856cff);
        }
        35 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xb38)) else { return RegfileExit::fault(0x7000856d00); };
            c.w64(0, t15);
            return c.ret(0x0, 0x7000856d07);
        }
        36 => {
            let Some(t16) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856d08); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000856d0e); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856d14); };
            c.w32(1, t20);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x33u64, 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::fault(0x7000856d23);
        }
        38 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0xd3)) else { return RegfileExit::fault(0x7000856d24); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            let Some(t23) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000856d2d); };
            c.w32(2, t23);
            let r24 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r24);
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856d33); };
            c.w32(2, t25);
            let r26 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r26);
            c.w64(1, c.r64(2));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x580)) else { return RegfileExit::fault(0x7000856d3c); };
            c.w64(2, t27);
            blk = 43; continue;
        }
        39 => {
            let Some(t28) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856d45); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000856d4b); };
            c.w32(2, t30);
            let r31 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r31);
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856d51); };
            c.w32(1, t32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xc7u64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x7000856d63);
        }
        41 => {
            let Some(t33) = c.ld32(c.r64(2).wrapping_add(0x323)) else { return RegfileExit::fault(0x7000856d64); };
            c.w32(2, t33);
            let r34 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r34);
            let Some(t35) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000856d6d); };
            c.w32(2, t35);
            let r36 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r36);
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856d73); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            c.w64(1, c.r64(2));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa90)) else { return RegfileExit::fault(0x7000856d7c); };
            c.w64(2, t39);
            blk = 43; continue;
        }
        42 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856d85); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            blk = 43; continue;
        }
        43 => {
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000856d8b); };
            c.cmp(t42, c.r32(1), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t43) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856d94); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000856d9a); };
            c.logic(t45 & 0x10000000u64, 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 42; continue;
        }
        46 => {
            c.w64(0, c.r64(2));
            return c.ret(0x0, 0x7000856dac);
        }
        47 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000856dad); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000856db1); }
            let r46 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r46);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000856db7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000856db8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000856dbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000856dc0); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x1c78)) else { return RegfileExit::fault(0x7000856dc4); };
            c.w64(3, t47);
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000856dd8, 0x7000856dd3);
        }
        48 => {
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000856dd8); };
            c.cmp(t48, c.r32(0), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000856de1); };
            c.w64(2, t49);
            let Some(t50) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000856de5); };
            c.logic(t50 & 0x2u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x910)) else { return RegfileExit::fault(0x7000856deb); };
            c.w64(2, t51);
            blk = 53; continue;
        }
        51 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x720)) else { return RegfileExit::fault(0x7000856df4); };
            c.w64(2, t52);
            blk = 53; continue;
        }
        52 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x500)) else { return RegfileExit::fault(0x7000856dfd); };
            c.w64(2, t53);
            blk = 53; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000856e04); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xd18)) else { return RegfileExit::fault(0x7000856e08); };
            c.w64(3, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000856e0f); };
            c.w64(0, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000856e13); };
            c.w64(6, t56);
            return c.call(0x7000870914, 0x7000856e1c, 0x7000856e17);
        }
        54 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856e20); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856e26); };
            c.cmp(t59, 0x40u64, 16);
            if c.cc_b() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000856e2d); };
            c.w64(0, t60);
            blk = 57; continue;
        }
        57 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000856e31); };
            c.w64(6, t61);
            c.w64(3, c.r64(0));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000856e38); };
            c.w64(0, t62);
            let Some(t63) = c.ld32(c.r64(0).wrapping_add(0x57)) else { return RegfileExit::fault(0x7000856e3f); };
            c.w32(0, t63);
            let r64 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r64);
            c.w64(2, c.r64(0));
            return c.call(0x700086f9d4, 0x7000856e4d, 0x7000856e48);
        }
        58 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000856e4d); };
            c.w64(6, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000856e51); };
            c.w64(3, t66);
            let Some(t67) = c.ld32(c.r64(3).wrapping_add(0x5b)) else { return RegfileExit::fault(0x7000856e58); };
            c.w32(3, t67);
            let r68 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r68);
            return c.call(0x700086f9d4, 0x7000856e63, 0x7000856e5e);
        }
        59 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000856e66); }
            return c.ret(0x0, 0x7000856e67);
        }
        60 => {
            let Some(t69) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000856e68); };
            c.w32(2, t69);
            let r70 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r70);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t71) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856e73); };
            c.w32(2, t71);
            let r72 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000856e79); };
            c.cmp(t73, c.r32(2), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000856e82); };
            c.cmp(t74, c.r32(2), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000856e8b); };
            c.cmp(t75, c.r32(2), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t76) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856e94); };
            c.cmp(t76, 0x41u64, 16);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xa90)) else { return RegfileExit::fault(0x7000856e9b); };
            c.w64(2, t77);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        66 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x920)) else { return RegfileExit::fault(0x7000856eaa); };
            c.w64(2, t78);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        67 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x920)) else { return RegfileExit::fault(0x7000856eb9); };
            c.w64(2, t79);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        68 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x580)) else { return RegfileExit::fault(0x7000856ec8); };
            c.w64(2, t80);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        69 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x908)) else { return RegfileExit::fault(0x7000856ed7); };
            c.w64(2, t81);
            c.w64(1, c.r64(0));
            blk = 43; continue;
        }
        70 => {
            let Some(t82) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856ee6); };
            c.w32(2, t82);
            let r83 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r83);
            let Some(t84) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000856eec); };
            c.w32(2, t84);
            let r85 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r85);
            let Some(t86) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856ef2); };
            c.w32(1, t86);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xc9u64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::fault(0x7000856f04);
        }
        72 => {
            let Some(t87) = c.ld32(c.r64(2).wrapping_add(0x32b)) else { return RegfileExit::fault(0x7000856f05); };
            c.w32(2, t87);
            let r88 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r88);
            let Some(t89) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000856f0e); };
            c.w32(2, t89);
            let r90 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r90);
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856f14); };
            c.w32(2, t91);
            let r92 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r92);
            c.w64(1, c.r64(2));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x920)) else { return RegfileExit::fault(0x7000856f1d); };
            c.w64(2, t93);
            blk = 43; continue;
        }
        73 => {
            let Some(t94) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856f29); };
            c.w32(2, t94);
            let r95 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r95);
            let Some(t96) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000856f2f); };
            c.w32(2, t96);
            let r97 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r97);
            let Some(t98) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856f35); };
            c.w32(1, t98);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x32u64, 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            return RegfileExit::fault(0x7000856f44);
        }
        75 => {
            let Some(t99) = c.ld32(c.r64(2).wrapping_add(0xcf)) else { return RegfileExit::fault(0x7000856f45); };
            c.w32(2, t99);
            let r100 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r100);
            let Some(t101) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000856f4e); };
            c.w32(2, t101);
            let r102 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r102);
            let Some(t103) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856f54); };
            c.w32(2, t103);
            let r104 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r104);
            c.w64(1, c.r64(2));
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x920)) else { return RegfileExit::fault(0x7000856f5d); };
            c.w64(2, t105);
            blk = 43; continue;
        }
        76 => {
            let Some(t106) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856f69); };
            c.w32(2, t106);
            let r107 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r107);
            let Some(t108) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000856f6f); };
            c.w32(2, t108);
            let r109 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r109);
            let Some(t110) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000856f75); };
            c.w32(1, t110);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x8bu64, 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::fault(0x7000856f87);
        }
        78 => {
            let Some(t111) = c.ld32(c.r64(2).wrapping_add(0x233)) else { return RegfileExit::fault(0x7000856f88); };
            c.w32(2, t111);
            let r112 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r112);
            let Some(t113) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000856f91); };
            c.w32(2, t113);
            let r114 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r114);
            let Some(t115) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000856f97); };
            c.w32(2, t115);
            let r116 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r116);
            c.w64(1, c.r64(2));
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x908)) else { return RegfileExit::fault(0x7000856fa0); };
            c.w64(2, t117);
            blk = 43; continue;
        }
        79 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000856fac); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000856fb0); }
            let r118 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r118);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000856fb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000856fba); }
            return c.call(0x70008a2054, 0x7000856fc3, 0x7000856fbe);
        }
        80 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000856fc3); };
            c.w64(6, t119);
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0xd18)) else { return RegfileExit::fault(0x7000856fc7); };
            c.w64(3, t120);
            return c.call(0x7000870914, 0x7000856fd3, 0x7000856fce);
        }
        81 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000856fd7); };
            c.w32(2, t121);
            let r122 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r122);
            let Some(t123) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000856fdd); };
            c.cmp(t123, 0x40u64, 16);
            if c.cc_b() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000856fe4); };
            c.w64(2, t124);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000856feb); }
            blk = 46; continue;
        }
        84 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000856ff1); };
            c.w64(6, t125);
            c.w64(3, c.r64(0));
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000856ff8); };
            c.w64(0, t126);
            let Some(t127) = c.ld32(c.r64(0).wrapping_add(0x57)) else { return RegfileExit::fault(0x7000856fff); };
            c.w32(0, t127);
            let r128 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r128);
            c.w64(2, c.r64(0));
            return c.call(0x700086f9d4, 0x700085700d, 0x7000857008);
        }
        85 => {
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085700d); };
            c.w64(3, t129);
            let Some(t130) = c.ld32(c.r64(3).wrapping_add(0x5b)) else { return RegfileExit::fault(0x7000857014); };
            c.w32(3, t130);
            let r131 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085701a); };
            c.w64(6, t132);
            return c.call(0x700086f9d4, 0x7000857023, 0x700085701e);
        }
        86 => {
            blk = 59; continue;
        }
        87 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000857028); };
            c.w64(2, t133);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085702f); }
            blk = 46; continue;
        }
        88 => {
            return RegfileExit::jump(0x7000857038);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000857054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857054); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000857058); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000857059); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085705a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700085705f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000857063); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085706d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857079); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085707f); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 51; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085708b); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857091); };
            c.w32(7, t8);
            c.cmp(c.r16(7), 0xbdu64, 16);
            if c.cc_b() { blk = 73; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008570a0); };
            c.w32(8, t9);
            let r10 = c.r32(8) & 0xfcu64;
            c.w32(8, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(8), 0x40u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(8), 0x3cu64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r11 = c.r32(9) | c.r32(8);
            c.w32(9, r11);
            c.logic(r11, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(7, c.r16(7));
            c.cmp(c.r32(7), 0x410u64, 32);
            if c.cc_le() { blk = 73; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008570dc); };
            c.w32(7, t12);
            c.logic(c.r32(7) & 0x200000u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008570eb); };
            c.w32(8, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008570ef); };
            c.cmp(t14, c.r32(8), 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008570fc); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000857103); };
            c.w32(8, t17);
            let r18 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r18);
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085710a); };
            c.w32(9, t19);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x700085711a);
        }
        10 => {
            let r20 = c.r64(7) & 0x3ffu64;
            c.w64(7, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000857122); };
            c.w32(8, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000857129); }
            c.cmp(c.r64(7), 0x3ffu64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 63; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000857151); };
            c.w64(9, t22);
            let Some(t23) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000857158); };
            c.w64(9, t23);
            c.w64(2, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000857167); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085716e); };
            c.cmp(t25, c.r64(11), 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 53; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000857185); };
            c.w64(12, t26);
            if !c.st64(c.r64(12), c.r64(11)) { return RegfileExit::fault(0x700085718c); }
            let r27 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r27);
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.w32(12, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008571a0); };
            c.w64(15, t28);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008571a7); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008571ab); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008571b5); };
            c.w64(12, t29);
            blk = 18; continue;
        }
        18 => {
            let r30 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r30);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x70008571c0); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t31) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008571ca); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008571d1); };
            c.w64(2, t33);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008571d8); }
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(3));
            c.w64(15, c.r64(3));
            blk = 21; continue;
        }
        20 => {
            let r34 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r34);
            blk = 21; continue;
        }
        21 => {
            let r35 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(6, r35);
            if let Some(sr) = c.shl(c.r64(6), 0x2u64, 64) { c.w64(6, sr); }
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008571f2); };
            c.w32(12, t36);
            let r37 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008571fa); };
            c.w32(12, t38);
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008571ff); };
            c.cmp(t39, c.r32(12), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t40) = c.ld32(c.r64(11).wrapping_add(c.r64(6)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085720c); };
            c.w32(12, t40);
            c.w32(6, c.r32(12));
            c.w64(8, c.r64(6));
            let r41 = c.r32(8) & 0x2u64;
            c.w32(8, r41);
            c.logic(r41, 32);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 41; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(8, c.r32(15));
            c.w32(8, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t42) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085723f); };
            c.w32(8, t42);
            blk = 37; continue;
        }
        26 => {
            let r43 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r43);
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t44) = c.ld8(c.r64(1).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000857250); };
            c.w32(8, t44);
            let Some(t45) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857255); };
            c.w32(11, t45);
            let r46 = c.r32(6) & 0x380u64;
            c.w32(6, r46);
            c.logic(r46, 32);
            let r47 = c.r64(12) & 0x3ffu64;
            c.w64(12, r47);
            c.logic(r47, 64);
            c.cmp(c.r32(6), 0x100u64, 32);
            c.w8(6, (c.cc_ne() as u64));
            c.w32(6, c.r8(6));
            let r48 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r48);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085727d); };
            c.w32(12, t49);
            let r50 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r50);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(12, c.r64(2));
            blk = 29; continue;
        }
        29 => {
            let r51 = c.sub(c.r64(8), c.r64(11), 64);
            c.w64(8, r51);
            let Some(t52) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857290); };
            c.w32(8, t52);
            let r53 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r53);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700085729d); }
            blk = 33; continue;
        }
        31 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008572aa); };
            c.w32(8, t54);
            let r55 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r55);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008572b2); }
            blk = 33; continue;
        }
        33 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008572b8); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008572bf); };
            c.w64(11, t57);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008572c6); };
            c.w64(6, t58);
            let Some(t59) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008572cd); };
            c.cmp(t59, c.r64(12), 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008572d6); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 35; continue;
        }
        35 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008572dd); };
            c.w64(11, t60);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008572e4); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008572e8); }
            blk = 37; continue;
        }
        36 => {
            let Some(t61) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008572f0); };
            c.w32(8, t61);
            blk = 37; continue;
        }
        37 => {
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008572f5); };
            c.w32(11, t62);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 57; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(11, c.r64(3).wrapping_add(0x1));
            c.w64(12, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)), c.r32(8)) { return RegfileExit::fault(0x7000857314); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r63 = c.r64(3) & c.r64(9);
            c.w64(3, r63);
            c.logic(r63, 64);
            let Some(t64) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000857328); };
            c.logic(t64 & 0x4u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(3, c.r64(11));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000857335); };
            c.w64(11, t65);
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_ne() { blk = 20; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085734c); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000857353); };
            c.w64(7, t67);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085735a); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000857361); };
            c.cmp(t69, c.r64(8), 64);
            if c.cc_be() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085736a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 45; continue;
        }
        45 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000857374); };
            c.w64(8, t70);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000857378); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700085737c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700085737f); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000857382); }
            c.w64(0, c.r64(1));
            blk = 46; continue;
        }
        46 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000857389); };
            c.w64(1, t71);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000857390); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            return c.ret(0x10, 0x7000857397);
        }
        48 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085739a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008573a1); }
            return c.ret(0x0, 0x70008573a3);
        }
        49 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008573a6); };
            c.w64(3, t72);
            return c.call(0x700086ed74, 0x70008573b2, 0x70008573ad);
        }
        50 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008573b2); };
            c.w64(6, t73);
            blk = 1; continue;
        }
        51 => {
            return c.call(0x70008a2054, 0x70008573c0, 0x70008573bb);
        }
        52 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008573c0); };
            c.w64(6, t74);
            blk = 3; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008573c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008573cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008573d1); }
            return c.call(0x70007febb4, 0x70008573da, 0x70008573d5);
        }
        54 => {
            c.w64(9, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008573dd); };
            c.w64(0, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008573e1); };
            c.w64(6, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008573e5); };
            c.w64(8, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008573e9); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008573ed); };
            c.w64(1, t79);
            blk = 16; continue;
        }
        55 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008573fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008573ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000857403); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000857407); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x700085740b); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700085740f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000857414); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000857420, 0x700085741b);
        }
        56 => {
            c.w64(8, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000857423); };
            c.w64(0, t80);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000857427); };
            c.w64(2, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085742e); };
            c.w64(9, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000857432); };
            c.w64(15, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000857436); };
            c.w64(7, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085743a); };
            c.w64(3, t85);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700085743e); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000857443); };
            c.w64(1, t86);
            blk = 35; continue;
        }
        57 => {
            return RegfileExit::fault(0x700085744c);
        }
        58 => {
            c.w32(1, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000857452); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000857456); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000857462, 0x700085745d);
        }
        59 => {
            c.w64(1, c.r64(0));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000857465); };
            c.w64(2, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085746c); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000857470); };
            c.w64(3, t89);
            blk = 45; continue;
        }
        60 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000857479); };
            c.w64(6, t90);
            blk = 61; continue;
        }
        61 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085747d); }
            c.w32(0, 0x1u64);
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2450)) else { return RegfileExit::fault(0x7000857483); };
            c.w64(3, t91);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000857492, 0x700085748d);
        }
        62 => {
            blk = 46; continue;
        }
        63 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000857497); };
            c.w64(3, t92);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008574a7, 0x70008574a2);
        }
        64 => {
            return RegfileExit::fault(0x70008574a7);
        }
        65 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008574a8); };
            c.w64(2, t93);
            blk = 66; continue;
        }
        66 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008574af); };
            c.w64(6, t94);
            let Some(t95) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008574b3); };
            c.w32(1, t95);
            let r96 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r96);
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008574b9); };
            c.w32(1, t97);
            let r98 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r98);
            let Some(t99) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008574bf); };
            c.w32(7, t99);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x6bu64, 64);
            if c.cc_a() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008574ce);
        }
        68 => {
            let Some(t100) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008574cf); };
            c.w32(1, t100);
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008574d5); };
            c.w64(7, t101);
            let Some(t102) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008574dc); };
            c.w64(8, t102);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008574e3); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008574e7); };
            c.w64(11, t103);
            let Some(t104) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008574ee); };
            c.cmp(t104, c.r64(9), 64);
            if c.cc_a() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w32(2, 0x10u64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x7000857500, 0x70008574fb);
        }
        70 => {
            blk = 72; continue;
        }
        71 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000857502); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 72; continue;
        }
        72 => {
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085750c); };
            c.w64(1, t105);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000857510); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000857515); }
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000857518); };
            c.w64(2, t106);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700085751f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000857522); }
            blk = 46; continue;
        }
        73 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085752a); }
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2458)) else { return RegfileExit::fault(0x700085752b); };
            c.w64(3, t107);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085753c, 0x7000857537);
        }
        74 => {
            blk = 46; continue;
        }
        75 => {
            c.w32(8, c.r32(8));
            let r108 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r108);
            c.w64(3, 0xfffffffffffc0000u64);
            let r109 = c.r64(3) & c.r64(8);
            c.w64(3, r109);
            c.logic(r109, 64);
            let Some(t110) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000857551); };
            c.logic(t110 & 0x2u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(6, c.r64(9).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085755f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000857560); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x700085756d, 0x7000857568);
        }
        77 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085756d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700085756f); }
            blk = 40; continue;
        }
        78 => {
            return RegfileExit::jump(0x7000857578);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000857594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008575a5); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008575ab); };
            c.w32(7, t2);
            let r3 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r3);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008575be); };
            c.w32(1, t4);
            c.w64(7, c.r64(1));
            let r5 = c.r32(7) & 0xc0u64;
            c.w32(7, r5);
            c.logic(r5, 32);
            c.cmp(c.r32(7), 0xc0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008575d3); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008575d9); };
            c.w32(7, t8);
            let r9 = c.r32(7) & 0x1fu64;
            c.w32(7, r9);
            c.logic(r9, 32);
            let r10 = c.sub(c.r32(7), 0x10u64, 32);
            c.w32(7, r10);
            c.cmp(c.r32(7), 0x4u64, 32);
            if c.cc_a() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008575f4); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008575fa); };
            c.w32(7, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008575fd); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000857606); };
            c.cmp(t15, c.r32(1), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 10; continue;
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000857611); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857617); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            blk = 12; continue;
        }
        11 => {
            let Some(t20) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085761f); };
            c.w32(2, t20);
            let r21 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r21);
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857625); };
            c.w32(7, t22);
            c.cmp(c.r16(7), 0x410u64, 16);
            if c.cc_be() { blk = 20; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000857630); };
            c.w32(2, t23);
            let r24 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000857636); };
            c.cmp(t25, c.r32(2), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000857643); };
            c.w64(0, t26);
            return c.ret(0x0, 0x700085764a);
        }
        16 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085764b); };
            c.w64(0, t27);
            return c.ret(0x0, 0x7000857652);
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857653); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000857657); }
            blk = 29; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085765b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085765f); }
            blk = 29; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857663); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000857667); }
            blk = 29; continue;
        }
        20 => {
            c.cmp(c.r16(7), 0xbcu64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t28) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000857672); };
            c.logic(t28 & 0x24u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857678); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085767c); }
            blk = 24; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857680); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000857684); }
            blk = 24; continue;
        }
        24 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000857686); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000857687); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x23b0)) else { return RegfileExit::fault(0x7000857688); };
            c.w64(3, t29);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000857699, 0x7000857694);
        }
        25 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085769c); }
            return c.ret(0x0, 0x700085769d);
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085769e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008576a2); }
            blk = 29; continue;
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008576a6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008576aa); }
            blk = 29; continue;
        }
        28 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008576ae); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008576b2); }
            blk = 29; continue;
        }
        29 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008576b4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008576b5); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2178)) else { return RegfileExit::fault(0x70008576b6); };
            c.w64(3, t30);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008576c7, 0x70008576c2);
        }
        30 => {
            blk = 25; continue;
        }
        31 => {
            return RegfileExit::jump(0x70008576cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008576d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008576d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008576d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008576de); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008576e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008576e6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008576f2); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008576f8); };
            c.cmp(t3, 0xbau64, 16);
            if c.cc_b() { blk = 32; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xd08)) else { return RegfileExit::fault(0x7000857704); };
            c.w64(3, t4);
            return c.call(0x7000870914, 0x7000857710, 0x700085770b);
        }
        3 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000857710); };
            c.w64(6, t5);
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857714); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085771a); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857720); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xe4u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000857733);
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x397)) else { return RegfileExit::fault(0x7000857734); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000857740); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085774d); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085775a); };
            if !c.push64(t14) { return RegfileExit::fault(0x700085775a); }
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085775d); };
            if !c.push64(t15) { return RegfileExit::fault(0x700085775d); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000857770, 0x700085776b);
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000857770); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700085777f); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857788); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            let Some(t20) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085778e); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000857794); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t22) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085779d); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 17; continue;
        }
        16 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008577ab); };
            c.w64(0, t23);
            blk = 18; continue;
        }
        18 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008577b5); }
            return c.ret(0x0, 0x70008577b6);
        }
        19 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008577b7); };
            c.w64(0, t24);
            blk = 18; continue;
        }
        20 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008577c0); };
            if !c.push64(t25) { return RegfileExit::fault(0x70008577c0); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008577c3); };
            if !c.push64(t26) { return RegfileExit::fault(0x70008577c3); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            return c.call(0x70008a8174, 0x70008577d3, 0x70008577ce);
        }
        21 => {
            blk = 18; continue;
        }
        22 => {
            blk = 32; continue;
        }
        23 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008577d7); };
            c.logic(t27 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 17; continue;
        }
        25 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008577e2); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 17; continue; }
            blk = 26; continue;
        }
        26 => {
            blk = 19; continue;
        }
        27 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008577f3); };
            c.w64(2, t28);
            let Some(t29) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008577f7); };
            c.w32(1, t29);
            let r30 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r30);
            let Some(t31) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008577fd); };
            c.logic(t31 & 0x2u64, 8);
            if c.cc_ne() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(1, 0xa4u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000857808); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000857809); };
            c.w64(3, t32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085781a, 0x7000857815);
        }
        29 => {
            return RegfileExit::fault(0x700085781a);
        }
        30 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085781b); };
            c.w64(0, t33);
            return c.call(0x7000857594, 0x7000857824, 0x700085781f);
        }
        31 => {
            blk = 18; continue;
        }
        32 => {
            c.w32(1, 0xaeu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085782b); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085782c); };
            c.w64(3, t34);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085783d, 0x7000857838);
        }
        33 => {
            return RegfileExit::fault(0x700085783d);
        }
        34 => {
            return RegfileExit::jump(0x7000857840);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000857854(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857854); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000857858); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085785e); };
            c.w32(8, t1);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700085786d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000857871); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x7000857875); }
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000857879); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085788e); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            c.w32(12, c.r32(11));
            let r5 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r5);
            c.logic(r5, 32);
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008578a6); };
            c.cmp(t6, c.r32(11), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008578b3); };
            c.cmp(t7, c.r32(11), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, c.r64(3));
            blk = 8; continue;
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008578d5); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008578dc); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008578e3); };
            c.w32(12, t12);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(12, c.r16(12));
            let r13 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r13);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000857902); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857909); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857910); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000857917); };
            c.w32(12, t20);
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085791c); };
            c.cmp(t21, c.r32(12), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(1, c.r64(3));
            let r22 = c.r64(1) | 0x3u64;
            c.w64(1, r22);
            c.logic(r22, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000857929); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r23 = c.r64(11) & c.r64(2);
            c.w64(11, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085793c); };
            c.logic(t24 & 0x4u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000857947); }
            blk = 15; continue;
        }
        14 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000857950); };
            c.w64(8, t25);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000857957); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700085795c); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085796d); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            let Some(t28) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857973); };
            c.cmp(t28, 0xbau64, 16);
            if c.cc_b() { blk = 47; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xd08)) else { return RegfileExit::fault(0x700085797f); };
            c.w64(1, t29);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(1));
            return c.call(0x7000870914, 0x7000857991, 0x700085798c);
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000857991); };
            c.w64(6, t30);
            let Some(t31) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857995); };
            c.w32(1, t31);
            let r32 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r32);
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085799b); };
            c.w32(1, t33);
            let r34 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r34);
            let Some(t35) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008579a1); };
            c.w32(8, t35);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xe4u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008579b4);
        }
        20 => {
            let Some(t36) = c.ld32(c.r64(1).wrapping_add(0x397)) else { return RegfileExit::fault(0x70008579b5); };
            c.cmp(t36, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008579c1); };
            c.cmp(t37, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008579ce); };
            c.cmp(t38, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008579db); };
            if !c.push64(t39) { return RegfileExit::fault(0x70008579db); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008579de); };
            if !c.push64(t40) { return RegfileExit::fault(0x70008579de); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008579f1, 0x70008579ec);
        }
        24 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008579f1); };
            c.cmp(t41, c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000857a00); };
            c.cmp(t42, c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857a09); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            let Some(t45) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000857a0f); };
            c.logic(t45 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000857a15); };
            c.cmp(t46, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t47) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857a1e); };
            c.cmp(t47, 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000857a2c); };
            c.w64(0, t48);
            blk = 33; continue;
        }
        33 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000857a36); }
            return c.ret(0x0, 0x7000857a37);
        }
        34 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000857a38); };
            c.w64(0, t49);
            blk = 33; continue;
        }
        35 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000857a41); };
            if !c.push64(t50) { return RegfileExit::fault(0x7000857a41); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000857a44); };
            if !c.push64(t51) { return RegfileExit::fault(0x7000857a44); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            return c.call(0x70008a8174, 0x7000857a54, 0x7000857a4f);
        }
        36 => {
            blk = 33; continue;
        }
        37 => {
            blk = 47; continue;
        }
        38 => {
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857a58); };
            c.logic(t52 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 32; continue;
        }
        40 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000857a63); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 32; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 34; continue;
        }
        42 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000857a74); };
            c.w64(2, t53);
            let Some(t54) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857a78); };
            c.w32(1, t54);
            let r55 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r55);
            let Some(t56) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000857a7e); };
            c.logic(t56 & 0x2u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, 0xa4u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000857a89); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000857a8a); };
            c.w64(3, t57);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000857a9b, 0x7000857a96);
        }
        44 => {
            return RegfileExit::fault(0x7000857a9b);
        }
        45 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000857a9c); };
            c.w64(0, t58);
            return c.call(0x7000857594, 0x7000857aa5, 0x7000857aa0);
        }
        46 => {
            blk = 33; continue;
        }
        47 => {
            c.w32(1, 0xaeu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000857aac); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000857aad); };
            c.w64(3, t59);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000857abe, 0x7000857ab9);
        }
        48 => {
            return RegfileExit::fault(0x7000857abe);
        }
        49 => {
            c.w32(1, c.r32(1));
            let r60 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r60);
            c.w64(11, 0xfffffffffffc0000u64);
            let r61 = c.r64(11) & c.r64(1);
            c.w64(11, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000857ace); };
            c.logic(t62 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000857add); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000857ade); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000857aeb, 0x7000857ae6);
        }
        51 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000857aeb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000857aed); }
            blk = 13; continue;
        }
        52 => {
            return RegfileExit::jump(0x7000857af4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000857b14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857b14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000857b18); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000857b1e); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000857b22); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000857b26); };
            c.w64(1, t3);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t4) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857b34); };
            c.w32(11, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000857b38); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000857b3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x7000857b40); }
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000857b44); };
            c.cmp(t5, c.r32(11), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(11, c.r64(3).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000857b59); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            c.w32(15, c.r32(12));
            let r8 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000857b71); };
            c.cmp(t9, c.r32(12), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000857b7e); };
            c.cmp(t10, c.r32(12), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(12, c.r64(0));
            blk = 8; continue;
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857ba0); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857ba8); };
            c.w32(15, t13);
            let r14 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r14);
            let Some(t15) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857bb0); };
            c.w32(15, t15);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(15, c.r16(15));
            let r16 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r16);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000857bce); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            let Some(t19) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857bd6); };
            c.w32(15, t19);
            let r20 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r20);
            let Some(t21) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857bdd); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000857be5); };
            c.w32(15, t23);
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000857be9); };
            c.cmp(t24, c.r32(15), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(12, c.r64(0));
            let r25 = c.r64(12) | 0x3u64;
            c.w64(12, r25);
            c.logic(r25, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x7000857bf7); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r26 = c.r64(15) & c.r64(8);
            c.w64(15, r26);
            c.logic(r26, 64);
            let Some(t27) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000857c0b); };
            c.logic(t27 & 0x4u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000857c16); }
            blk = 15; continue;
        }
        14 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000857c20); };
            c.w64(11, t28);
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000857c27); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000857c2c); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857c3e); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857c45); };
            c.cmp(t31, 0xbau64, 16);
            if c.cc_b() { blk = 47; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xd08)) else { return RegfileExit::fault(0x7000857c52); };
            c.w64(3, t32);
            c.w64(6, c.r64(1));
            return c.call(0x7000870914, 0x7000857c61, 0x7000857c5c);
        }
        18 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000857c61); };
            c.w64(6, t33);
            let Some(t34) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857c65); };
            c.w32(1, t34);
            let r35 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r35);
            let Some(t36) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000857c6b); };
            c.w32(1, t36);
            let r37 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r37);
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857c71); };
            c.w32(8, t38);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xe4u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x7000857c84);
        }
        20 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0x397)) else { return RegfileExit::fault(0x7000857c85); };
            c.cmp(t39, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000857c91); };
            c.cmp(t40, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000857c9e); };
            c.cmp(t41, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000857cab); };
            if !c.push64(t42) { return RegfileExit::fault(0x7000857cab); }
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000857cae); };
            if !c.push64(t43) { return RegfileExit::fault(0x7000857cae); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000857cc1, 0x7000857cbc);
        }
        24 => {
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000857cc1); };
            c.cmp(t44, c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000857cd0); };
            c.cmp(t45, c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857cd9); };
            c.w32(1, t46);
            let r47 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r47);
            let Some(t48) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000857cdf); };
            c.logic(t48 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t49) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000857ce5); };
            c.cmp(t49, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t50) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857cee); };
            c.cmp(t50, 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000857cfc); };
            c.w64(0, t51);
            blk = 33; continue;
        }
        33 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000857d06); }
            return c.ret(0x0, 0x7000857d07);
        }
        34 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000857d08); };
            c.w64(0, t52);
            blk = 33; continue;
        }
        35 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000857d11); };
            if !c.push64(t53) { return RegfileExit::fault(0x7000857d11); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000857d14); };
            if !c.push64(t54) { return RegfileExit::fault(0x7000857d14); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            return c.call(0x70008a8174, 0x7000857d24, 0x7000857d1f);
        }
        36 => {
            blk = 33; continue;
        }
        37 => {
            blk = 47; continue;
        }
        38 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857d28); };
            c.logic(t55 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 32; continue;
        }
        40 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000857d33); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 32; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 34; continue;
        }
        42 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000857d44); };
            c.w64(2, t56);
            let Some(t57) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857d48); };
            c.w32(1, t57);
            let r58 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r58);
            let Some(t59) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000857d4e); };
            c.logic(t59 & 0x2u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, 0xa4u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000857d59); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000857d5a); };
            c.w64(3, t60);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000857d6b, 0x7000857d66);
        }
        44 => {
            return RegfileExit::fault(0x7000857d6b);
        }
        45 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000857d6c); };
            c.w64(0, t61);
            return c.call(0x7000857594, 0x7000857d75, 0x7000857d70);
        }
        46 => {
            blk = 33; continue;
        }
        47 => {
            c.w32(8, 0xaeu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000857d7d); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000857d7f); };
            c.w64(3, t62);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000857d93, 0x7000857d8e);
        }
        48 => {
            return RegfileExit::fault(0x7000857d93);
        }
        49 => {
            c.w32(12, c.r32(12));
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            c.w64(15, 0xfffffffffffc0000u64);
            let r64 = c.r64(15) & c.r64(12);
            c.w64(15, r64);
            c.logic(r64, 64);
            let Some(t65) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000857da4); };
            c.logic(t65 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(3, c.r64(8).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000857db3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000857db4); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000857dc1, 0x7000857dbc);
        }
        51 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000857dc1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000857dc3); }
            blk = 13; continue;
        }
        52 => {
            return RegfileExit::jump(0x7000857dcc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000857dd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857dd4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857dda); };
            c.w32(1, t2);
            c.w64(7, c.r64(1));
            let r3 = c.r64(7) & 0x3ffu64;
            c.w64(7, r3);
            c.logic(r3, 64);
            c.cmp(c.r64(7), 0x3ffu64, 64);
            if c.cc_e() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(2));
            c.w64(7, c.r64(0));
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857df6); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000857dfa); };
            c.cmp(t5, c.r32(8), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000857e03); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857e0c); };
            c.cmp(t7, 0x834u64, 16);
            if c.cc_ne() { blk = 24; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857e1b); };
            c.cmp(t8, c.r32(8), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000857e25); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000857e2b); };
            c.cmp(t11, c.r32(7), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857e34); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857e3a); };
            c.w32(8, t14);
            c.logic(c.r16(8) & 0x3ffu64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 24; continue;
        }
        9 => {
            c.w64(0, c.r64(2));
            return c.ret(0x0, 0x7000857e4e);
        }
        10 => {
            blk = 24; continue;
        }
        11 => {
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857e60); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000857e6b); };
            c.w64(1, t17);
            blk = 14; continue;
        }
        14 => {
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857e72); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x7000857e81);
        }
        16 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857e84); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(1, c.r64(0));
            blk = 18; continue;
        }
        18 => {
            let Some(t20) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857e8c); };
            c.w32(7, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000857e8f); };
            c.cmp(t21, c.r32(7), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000857e98); };
            c.cmp(t22, c.r32(7), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857ea1); };
            c.cmp(t23, 0x834u64, 16);
            if c.cc_ne() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t24) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857eab); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000857eb0); };
            c.w32(1, t25);
            let r26 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r26);
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000857eb6); };
            c.cmp(t27, c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857ebf); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857ec5); };
            c.w32(7, t30);
            c.logic(c.r16(7) & 0x3ffu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t31) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000857ecf); };
            if !c.push64(t31) { return RegfileExit::fault(0x7000857ecf); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x1f50)) else { return RegfileExit::fault(0x7000857ed2); };
            c.w64(3, t32);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x7000857ed9); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        25 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000857ee8); };
            c.w64(0, t33);
            return c.ret(0x0, 0x7000857eef);
        }
        26 => {
            return RegfileExit::jump(0x7000857ef4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000857f14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857f14); };
            c.w32(6, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000857f17); };
            c.cmp(t1, c.r32(6), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000857f20); };
            c.w32(6, t2);
            let r3 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857f26); };
            c.w32(6, t4);
            let r5 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r5);
            let Some(t6) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857f2c); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857f32); };
            c.w32(7, t8);
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000857f35); };
            c.w32(8, t9);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857f3c); };
            c.w32(0, t10);
            let r11 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r11);
            let r12 = c.r64(8) & 0x3ffu64;
            c.w64(8, r12);
            c.logic(r12, 64);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_le() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(6, 0x6u64);
            blk = 4; continue;
        }
        3 => {
            c.w32(6, 0x2u64);
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000857f5d); };
            c.w32(7, t13);
            c.w32(6, c.r32(6));
            c.w32(9, c.r32(7));
            let r14 = c.r32(6) | c.r32(9);
            c.w32(6, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(7), c.r32(6), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x7000857f70); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000857f74); }
            blk = 6; continue;
        }
        6 => {
            c.w32(3, c.r64(8).wrapping_add(c.r64(8)));
            blk = 7; continue;
        }
        7 => {
            c.w64(2, c.r64(3));
            return c.ret(0x0, 0x7000857f82);
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000857f83); };
            c.w32(6, t15);
            c.w32(7, c.r32(6));
            let r16 = c.r32(7) | 0xeu64;
            c.w32(7, r16);
            c.logic(r16, 32);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(6, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x7000857f93); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000857f97); }
            blk = 10; continue;
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000857f9e); };
            c.w32(3, t17);
            blk = 7; continue;
        }
        11 => {
            return RegfileExit::jump(0x7000857fa4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000857fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000857fb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000857fb8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000857fbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000857fc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000857fc6); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857fd3); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000857fd9); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 243; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 242; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000857ffe); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858005); };
            c.w32(9, t6);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000858011); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r7);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 232; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085805e); };
            c.w32(11, t8);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(9, c.r64(0));
            blk = 19; continue;
        }
        16 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000858092); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            blk = 19; continue;
        }
        17 => {
            if let Some(sr) = c.shr(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let r11 = c.r64(11) & 0xffffffu64;
            c.w64(11, r11);
            c.logic(r11, 64);
            blk = 95; continue;
        }
        18 => {
            c.w64(9, c.r64(0));
            blk = 19; continue;
        }
        19 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008580ae); };
            c.w64(15, t12);
            c.w64(11, c.r64(3));
            blk = 21; continue;
        }
        20 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008580ba); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008580c1); };
            c.w32(8, t15);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 50; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008580d3); };
            c.w32(1, t16);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008580e2); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            c.logic(c.r32(1) & 0xffc00u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 24; continue;
        }
        24 => {
            let r19 = c.shr(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r19);
            let r20 = c.r32(1) & 0x3ffu64;
            c.w32(1, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_be() { blk = 39; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000858107); };
            c.w32(3, (t21 as u16 as i16 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085810c); };
            c.w32(6, t22);
            let r23 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r23);
            let r24 = c.sub(c.r32(3), 0x1u64, 32);
            c.w32(3, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000858116); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700085811e); }
            blk = 27; continue;
        }
        27 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000858122); };
            c.w64(2, t25);
            let r26 = c.sub(c.r32(2), c.r32(0), 32);
            c.w32(2, r26);
            let r27 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r27);
            let r28 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r28);
            c.w32(12, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000858133); };
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
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085814d); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000858155); };
            c.w32(12, t35);
            let r36 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r36);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_be() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, c.r64(2).wrapping_add(0x1));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000858166); };
            c.w64(2, t37);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700085816e); }
            blk = 32; continue;
        }
        31 => {
            let r38 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r38);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(12, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000858182); };
            c.w32(12, t39);
            let r40 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r41);
            let r42 = c.r32(12) & 0x3ffu64;
            c.w32(12, r42);
            c.logic(r42, 32);
            c.w32(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085819c); };
            c.w32(2, t43);
            let r44 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008581a4); };
            c.w32(7, t45);
            let r46 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r46);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_a() { blk = 226; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008581bc); };
            c.w64(7, t47);
            blk = 61; continue;
        }
        37 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008581c5); };
            c.w64(7, t48);
            blk = 61; continue;
        }
        38 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008581ce); };
            c.w64(7, t49);
            blk = 61; continue;
        }
        39 => {
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            let r50 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r50);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            let r51 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r51);
            let Some(t52) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008581ec); };
            c.cmp(t52, c.r32(9), 32);
            if c.cc_e() { blk = 226; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 40; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 61; continue;
        }
        43 => {
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000858202); };
            c.w32(11, t53);
            let r54 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r54);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(11, c.r64(15));
            blk = 45; continue;
        }
        45 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000858212); };
            c.w32(12, t55);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r56 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r56);
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000858220); };
            c.w32(0, t57);
            let r58 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            let r59 = c.r64(0) & c.r64(12);
            c.w64(0, r59);
            c.logic(r59, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        46 => {
            let r60 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r60);
            let r61 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r61);
            c.w64(2, c.r64(12));
            let r62 = c.r64(2) & c.r64(0);
            c.w64(2, r62);
            c.logic(r62, 64);
            c.w64(0, c.r64(2));
            blk = 47; continue;
        }
        47 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858243); };
            c.w32(2, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858248); };
            c.cmp(t64, c.r32(2), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_e() { blk = 226; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 46; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t65) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085826d); };
            c.logic(t65 & 0x24u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000858277); };
            c.w32(11, t66);
            let r67 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r67);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(15));
            blk = 54; continue;
        }
        54 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000858287); };
            c.w32(12, t68);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r69 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r69);
            let Some(t70) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000858295); };
            c.w32(0, t70);
            let r71 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r71);
            let r72 = c.r64(0) & c.r64(12);
            c.w64(0, r72);
            c.logic(r72, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r73 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r73);
            let r74 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r74);
            c.w64(2, c.r64(12));
            let r75 = c.r64(2) & c.r64(0);
            c.w64(2, r75);
            c.logic(r75, 64);
            c.w64(0, c.r64(2));
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008582b7); };
            c.w32(1, t76);
            let r77 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r77);
            let Some(t78) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008582bf); };
            c.cmp(t78, c.r32(1), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t79) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008582c8); };
            c.cmp(t79, c.r32(1), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t80) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008582d1); };
            c.cmp(t80, c.r32(9), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008582d7); };
            c.w32(11, t81);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 240; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008582ea); };
            c.w32(11, t82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008582ee); };
            c.cmp(t83, c.r32(11), 32);
            if c.cc_ne() { blk = 226; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t84) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858308); };
            c.w32(8, t84);
            let r85 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r85);
            let Some(t86) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085830f); };
            c.w32(8, t86);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 92; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t87) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085831f); };
            c.w32(11, t87);
            let r88 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r88);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 92; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 67; continue;
        }
        67 => {
            c.w64(0, c.r64(11));
            let r89 = c.r32(0) & 0x7u64;
            c.w32(0, r89);
            c.logic(r89, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 234; continue;
        }
        72 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 234; continue;
        }
        75 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000858385); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700085838a); };
            c.cmp(t91, c.r32(11), 32);
            if c.cc_ne() { blk = 235; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000858397); };
            c.w32(12, t92);
            let r93 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r93);
            let Some(t94) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085839f); };
            c.w32(11, t94);
            let r95 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r95);
            let Some(t96) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008583a7); };
            c.w32(11, t96);
            blk = 67; continue;
        }
        77 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008583ae); };
            c.w32(11, t97);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r98 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r98);
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008583bc); };
            c.w32(12, t99);
            let r100 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r100);
            let Some(t101) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008583c4); };
            c.w32(11, t101);
            let r102 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r102);
            let Some(t103) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008583cc); };
            c.w32(11, t103);
            blk = 67; continue;
        }
        78 => {
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008583d3); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008583db); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008583e3); };
            c.w32(11, t108);
            blk = 67; continue;
        }
        79 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        80 => {
            c.w32(0, 0x1u64);
            blk = 82; continue;
        }
        81 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 236; continue; }
            blk = 83; continue;
        }
        83 => {
            let r109 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r109);
            blk = 84; continue;
        }
        84 => {
            let r110 = c.r32(11) & 0x8u64;
            c.w32(11, r110);
            c.logic(r110, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t111) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x7000858419); };
            c.w32(8, t111);
            blk = 87; continue;
        }
        86 => {
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000858420); };
            c.w32(8, t112);
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 243; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t113) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000858453); };
            c.w32(11, t113);
            let r114 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r114);
            let Some(t115) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700085845a); };
            c.cmp(t115, c.r32(11), 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 93; continue;
        }
        93 => {
            blk = 20; continue;
        }
        94 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 95; continue;
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000858475); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            blk = 97; continue;
        }
        96 => {
            let Some(t116) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858484); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            let Some(t118) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085848b); };
            c.w32(9, t118);
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 243; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t119) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085849d); };
            c.w32(9, t119);
            let r120 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r120);
            let r121 = c.r32(9) & 0x3fu64;
            c.w32(9, r121);
            c.logic(r121, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 133; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 117; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 109; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 106; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(9), 0x28u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 243; continue;
        }
        106 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 108; continue;
        }
        108 => {
            blk = 243; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_l() { blk = 114; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 113; continue;
        }
        113 => {
            blk = 243; continue;
        }
        114 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 116; continue;
        }
        116 => {
            blk = 243; continue;
        }
        117 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 126; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 123; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 122; continue;
        }
        122 => {
            blk = 243; continue;
        }
        123 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 125; continue;
        }
        125 => {
            blk = 243; continue;
        }
        126 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 130; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 129; continue;
        }
        129 => {
            blk = 243; continue;
        }
        130 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 132; continue;
        }
        132 => {
            blk = 243; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_l() { blk = 151; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_l() { blk = 143; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_l() { blk = 140; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 243; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 142; continue;
        }
        142 => {
            blk = 243; continue;
        }
        143 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_l() { blk = 148; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(9), 0xfu64, 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 147; continue;
        }
        147 => {
            blk = 243; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 243; continue;
        }
        151 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 160; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 157; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 156; continue;
        }
        156 => {
            blk = 243; continue;
        }
        157 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 243; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 164; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 163; continue;
        }
        163 => {
            blk = 243; continue;
        }
        164 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 166; continue;
        }
        166 => {
            blk = 243; continue;
        }
        167 => {
            blk = 172; continue;
        }
        168 => {
            let Some(t122) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008586c6); };
            c.w32(7, t122);
            let r123 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r123);
            let Some(t124) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008586cc); };
            c.w32(9, t124);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 173; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t125) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008586e5); };
            c.cmp(t125, c.r64(9), 64);
            if c.cc_e() { blk = 191; continue; }
            blk = 170; continue;
        }
        170 => {
            blk = 226; continue;
        }
        171 => {
            let Some(t126) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008586f5); };
            c.w32(9, t126);
            let r127 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r127);
            let Some(t128) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008586fc); };
            c.w32(9, t128);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 226; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t129) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858709); };
            c.w32(7, t129);
            let r130 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r130);
            let Some(t131) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085870f); };
            c.w32(9, t131);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 175; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w32(7, 0xfffffffeu64);
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_b() { blk = 243; continue; }
            blk = 174; continue;
        }
        174 => {
            blk = 191; continue;
        }
        175 => {
            let Some(t132) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858731); };
            c.w32(7, t132);
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000858736); };
            c.cmp(t133, c.r32(7), 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 176; continue;
        }
        176 => {
            blk = 226; continue;
        }
        177 => {
            let Some(t134) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000858748); };
            c.w32(9, t134);
            let r135 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r135);
            let Some(t136) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085874f); };
            c.w32(9, t136);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 226; continue; }
            blk = 178; continue;
        }
        178 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700085875c); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 243; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t137) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085876f); };
            c.w32(7, t137);
            let r138 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r138);
            let Some(t139) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000858775); };
            c.w32(9, t139);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r140 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r140);
            c.w32(2, c.r32(11));
            c.w64(10, c.r64(4));
            let r141 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r141);
            let r142 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r142);
            c.logic(r142, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000858791); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000858796); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085879a); }
            c.w64(1, c.r64(13));
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x70008587a1); };
            c.w64(0, t143);
            c.w64(10, 0x70008587afu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008587af); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008587b3); }
            return c.call(c.r64(0), 0x70008587b9, 0x70008587b7);
        }
        180 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008587b9); }
            let Some(t144) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008587c1); };
            c.w64(4, t144);
            c.w32(7, c.r32(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008587c8); };
            let r146 = c.r64(7) & t145;
            c.w64(7, r146);
            c.logic(r146, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008587cf); };
            c.w64(9, t147);
            blk = 182; continue;
        }
        181 => {
            let r148 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r148);
            let r149 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008587dc); };
            c.w64(11, t150);
            let r151 = c.r64(11) & c.r64(7);
            c.w64(11, r151);
            c.logic(r151, 64);
            c.w64(7, c.r64(11));
            blk = 182; continue;
        }
        182 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t152) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008587eb); };
            c.w32(11, t152);
            let r153 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r153);
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008587f3); };
            c.cmp(t154, c.r32(11), 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 183; continue;
        }
        183 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000858802); };
            c.cmp(t155, c.r32(11), 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 185; continue;
        }
        185 => {
            c.xorpd_rr(0, 0);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085880f); };
            c.cvtsi2sd(0, (t156 as i64));
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000858815); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 181; continue; }
            blk = 186; continue;
        }
        186 => {
            if c.cc_e() { blk = 226; continue; }
            blk = 187; continue;
        }
        187 => {
            blk = 181; continue;
        }
        188 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085882f); };
            c.cmp(c.r64(11), t157, 64);
            if c.cc_e() { blk = 226; continue; }
            blk = 189; continue;
        }
        189 => {
            blk = 181; continue;
        }
        190 => {
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085883b); };
            c.w64(8, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085883f); };
            c.w64(11, t159);
            blk = 191; continue;
        }
        191 => {
            c.w64(7, c.r64(8));
            let Some(t160) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000858846); };
            c.w32(7, t160);
            let r161 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085884c); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000858850); };
            c.w64(3, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000858854); };
            c.w64(6, t164);
            let Some(t165) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000858858); };
            c.cmp(t165, c.r32(7), 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 192; continue;
        }
        192 => {
            blk = 96; continue;
        }
        193 => {
            let Some(t166) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085886a); };
            c.w32(8, t166);
            let r167 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r167);
            let Some(t168) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000858871); };
            c.logic(t168 & 0x4u64, 8);
            if c.cc_ne() { blk = 225; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t169) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085887c); };
            c.cmp(t169, c.r64(11), 64);
            if c.cc_a() { blk = 226; continue; }
            blk = 195; continue;
        }
        195 => {
            blk = 225; continue;
        }
        196 => {
            let Some(t170) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085888b); };
            c.w32(8, t170);
            let r171 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r171);
            let Some(t172) = c.ld32(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000858892); };
            c.w32(9, t172);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 222; continue; }
            blk = 197; continue;
        }
        197 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_ne() { blk = 225; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t173) = c.ld64(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008588aa); };
            c.w64(8, t173);
            let Some(t174) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008588ae); };
            c.w64(9, t174);
            let Some(t175) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008588b2); };
            c.logic(t175 & 0x1u64, 8);
            if c.cc_ne() { blk = 201; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t176) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008588b8); };
            c.w64(12, t176);
            let r177 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r177);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 225; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t178) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008588c8); };
            c.w64(7, t178);
            c.w64(0, c.r64(7));
            blk = 224; continue;
        }
        201 => {
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 225; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t179) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008588dd); };
            c.w32(7, t179);
            let r180 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r180);
            let Some(t181) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008588e3); };
            c.w32(7, t181);
            let r182 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r182);
            let r183 = c.r32(7) & 0x3fu64;
            c.w32(7, r183);
            c.logic(r183, 32);
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_l() { blk = 210; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_l() { blk = 207; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_e() { blk = 217; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(7), 0x26u64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(7), 0x22u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_l() { blk = 214; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(7), 0x1fu64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_e() { blk = 217; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(7), 0x1du64, 32);
            if c.cc_e() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            return RegfileExit::fault(0x7000858933);
        }
        217 => {
            c.w32(7, 0x1u64);
            blk = 221; continue;
        }
        218 => {
            c.w32(7, 0x2u64);
            blk = 221; continue;
        }
        219 => {
            c.w32(7, 0x4u64);
            blk = 221; continue;
        }
        220 => {
            c.w32(7, 0x8u64);
            blk = 221; continue;
        }
        221 => {
            let r184 = c.sub(c.r64(8), c.r64(9), 64);
            c.w64(8, r184);
            c.w64(0, c.r64(8));
            c.cqo();
            c.idiv(c.r64(7), 64);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000858959); };
            c.w64(8, t185);
            blk = 224; continue;
        }
        222 => {
            c.w64(10, c.r64(4));
            let r186 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r186);
            let r187 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r187);
            c.logic(r187, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085896a); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000858975); };
            c.w64(0, t188);
            c.w64(10, 0x7000858983u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000858983); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000858987); }
            return c.call(c.r64(0), 0x700085898d, 0x700085898b);
        }
        223 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085898d); }
            let Some(t189) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000858995); };
            c.w64(4, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085899a); };
            c.w64(11, t190);
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008589a3); };
            c.w64(7, t191);
            blk = 227; continue;
        }
        226 => {
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008589ac); };
            c.w64(7, t192);
            blk = 227; continue;
        }
        227 => {
            let Some(t193) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008589b3); };
            c.cmp(t193, c.r32(7), 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008589bc); };
            c.w64(0, t194);
            blk = 229; continue;
        }
        229 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008589c6); }
            return c.ret(0x0, 0x70008589c7);
        }
        230 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008589c8); };
            c.w64(0, t195);
            blk = 229; continue;
        }
        231 => {
            blk = 243; continue;
        }
        232 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 243; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t196) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008589da); };
            c.w32(9, t196);
            let r197 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r197);
            blk = 19; continue;
        }
        234 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008589e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008589ea); }
            blk = 238; continue;
        }
        235 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008589f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008589f4); }
            blk = 238; continue;
        }
        236 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008589fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008589fe); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t198) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000858a08); };
            c.w64(12, t198);
            blk = 84; continue;
        }
        238 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000858a12); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000858a14); }
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x7000858a15); };
            c.w64(3, t199);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000858a26, 0x7000858a21);
        }
        239 => {
            let r200 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r200);
            c.w64(8, c.r64(0));
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000858a2b); };
            c.w64(7, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000858a2f); };
            c.w64(9, t202);
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000858a33); };
            c.w64(15, t203);
            blk = 87; continue;
        }
        240 => {
            return RegfileExit::fault(0x7000858a3f);
        }
        241 => {
            blk = 243; continue;
        }
        242 => {
            return c.call(0x70008a1af4, 0x7000858a47, 0x7000858a42);
        }
        243 => {
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000858a47); };
            if !c.push64(t204) { return RegfileExit::fault(0x7000858a47); }
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000858a4a); };
            if !c.push64(t205) { return RegfileExit::fault(0x7000858a4a); }
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x1f58)) else { return RegfileExit::fault(0x7000858a4d); };
            c.w64(3, t206);
            c.w32(0, 0x2u64);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000858a59); };
            c.w64(6, t207);
            return c.call(0x700086ed74, 0x7000858a62, 0x7000858a5d);
        }
        244 => {
            c.w64(7, c.r64(0));
            blk = 227; continue;
        }
        245 => {
            return RegfileExit::jump(0x7000858a6c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000858a74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858a74); };
            c.w64(7, t0);
            c.w64(2, c.r64(7));
            c.w64(3, c.r64(7));
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_b() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000858a87); };
            c.w64(7, t1);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858a8e); };
            c.w64(2, t2);
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_b() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000858a99); };
            c.w64(3, t3);
            blk = 4; continue;
        }
        4 => {
            if !c.pop64_reg(1) { return RegfileExit::fault(0x7000858a9e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(0).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000858aa4); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000858aa5); }
            return RegfileExit::jump(0x70007f6194);
        }
        5 => {
            return RegfileExit::fault(0x7000858aab);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000858ab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858ab4); };
            c.w64(7, t0);
            c.w64(2, c.r64(7));
            c.w64(3, c.r64(7));
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_b() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000858ac7); };
            c.w64(7, t1);
            c.w64(2, c.r64(7));
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858ad1); };
            c.w64(3, t2);
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_b() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000858adc); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        4 => {
            if !c.pop64_reg(1) { return RegfileExit::fault(0x7000858ae1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(0).wrapping_mul(8)).wrapping_add(0x8));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858ae7); };
            if !c.push64(t4) { return RegfileExit::fault(0x7000858ae7); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000858aee); }
            return RegfileExit::jump(0x70007f8574);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000858b14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ab0)) else { return RegfileExit::fault(0x7000858b14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858b34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ab8)) else { return RegfileExit::fault(0x7000858b34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858b54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ac0)) else { return RegfileExit::fault(0x7000858b54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858b74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ac8)) else { return RegfileExit::fault(0x7000858b74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858b94(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ad0)) else { return RegfileExit::fault(0x7000858b94); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858bb4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ad8)) else { return RegfileExit::fault(0x7000858bb4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858bd4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ae0)) else { return RegfileExit::fault(0x7000858bd4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858bf4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ae8)) else { return RegfileExit::fault(0x7000858bf4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858c14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1af0)) else { return RegfileExit::fault(0x7000858c14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858c34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1af8)) else { return RegfileExit::fault(0x7000858c34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858c54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b00)) else { return RegfileExit::fault(0x7000858c54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858c74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b08)) else { return RegfileExit::fault(0x7000858c74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858c94(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b10)) else { return RegfileExit::fault(0x7000858c94); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000858cb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000858cb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000858cb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000858cb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000858cba); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000858cbf); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(2)) { return RegfileExit::fault(0x7000858cc3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000858cc7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858cd1); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000858cd7); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000858cdd); };
            c.w32(7, t6);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x7000858cef);
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x7000858cf0); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858cf9); };
            c.w64(0, t9);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000858d07); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000858d0b); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858d18); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858d1f); };
            c.w32(8, t12);
            let r13 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r13);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 27; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x7000858d38); };
            c.w64(3, t14);
            return c.call(0x7000870914, 0x7000858d44, 0x7000858d3f);
        }
        6 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858d44); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000858d51); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000858d64); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858d71); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            let Some(t20) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000858d77); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000858d7d); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858d8a); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 115; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 15; continue;
        }
        14 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858d9c); };
            c.w64(0, t23);
            let Some(t24) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858da0); };
            c.w32(1, t24);
            let r25 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r25);
            let Some(t26) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858da6); };
            c.cmp(t26, 0x83eu64, 16);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(7, 0x1u64);
            blk = 18; continue;
        }
        17 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        18 => {
            c.w32(1, 0x1u64);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(7, 0x90u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000858dca); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x7000858dcb); };
            c.w64(3, t27);
            c.w64(0, c.r64(1));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000858dd5); };
            c.w64(6, t28);
            return c.call(0x700086ed74, 0x7000858dde, 0x7000858dd9);
        }
        20 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858dde); };
            c.w64(0, t29);
            c.w32(1, 0x1u64);
            blk = 28; continue;
        }
        21 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858de9); };
            c.w64(0, t30);
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858ded); };
            c.w32(1, t31);
            let r32 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r32);
            let Some(t33) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858df3); };
            c.cmp(t33, 0x83eu64, 16);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(1, 0x92u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000858e00); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x7000858e01); };
            c.w64(3, t34);
            c.w32(0, 0x1u64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000858e0d); };
            c.w64(6, t35);
            return c.call(0x700086ed74, 0x7000858e16, 0x7000858e11);
        }
        23 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858e18); };
            c.w64(0, t36);
            blk = 28; continue;
        }
        24 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858e1e); };
            c.w64(0, t37);
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858e22); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            let Some(t40) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858e28); };
            c.cmp(t40, 0x83eu64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(1, 0x1u64);
            blk = 28; continue;
        }
        26 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        27 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000858e3d); }
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000858e41); };
            c.w64(2, t41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858e45); };
            c.cmp(t42, c.r32(2), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000858e53); };
            c.w64(2, t43);
            blk = 36; continue;
        }
        31 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000858e59); };
            c.w64(7, t44);
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858e5d); };
            c.cmp(t45, c.r32(7), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000858e66); };
            c.w64(2, t46);
            blk = 36; continue;
        }
        33 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000858e6c); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000858e73); };
            c.w64(6, t48);
            return c.call(0x7000870914, 0x7000858e7c, 0x7000858e77);
        }
        34 => {
            let Some(t49) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000858e7c); };
            c.cmp(c.r32(0), t49, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000858e85); };
            c.w64(2, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858e89); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000858e8d); };
            c.w64(1, t52);
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(2)) { return RegfileExit::fault(0x7000858e91); }
            let Some(t53) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000858e95); };
            c.cmp(t53, 0x0u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000858e9f); };
            c.w32(7, t54);
            let r55 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r55);
            let Some(t56) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000858ea5); };
            c.cmp(t56, 0x83eu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000858eb2); };
            c.w64(7, t57);
            blk = 92; continue;
        }
        40 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa58)) else { return RegfileExit::fault(0x7000858ebb); };
            c.w64(3, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000858ec2); };
            c.w64(6, t59);
            return c.call(0x7000870914, 0x7000858ecb, 0x7000858ec6);
        }
        41 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000858ecb); };
            c.w64(7, t60);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000858ecf); }
            let Some(t61) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858ed3); };
            c.cmp(t61, c.r32(7), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(1, c.r64(7));
            blk = 45; continue;
        }
        43 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x700)) else { return RegfileExit::fault(0x7000858ee1); };
            c.w64(3, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000858ee8); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000858eec); };
            c.w64(6, t64);
            return c.call(0x7000870914, 0x7000858ef5, 0x7000858ef0);
        }
        44 => {
            c.w64(1, c.r64(0));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000858ef8); };
            c.w64(0, t65);
            blk = 45; continue;
        }
        45 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000858efc); };
            c.w64(2, t66);
            c.w64(7, c.r64(1));
            blk = 92; continue;
        }
        46 => {
            let Some(t67) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000858f08); };
            c.w32(1, t67);
            let r68 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000858f0e); };
            c.w64(7, t69);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000858f12); }
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000858f16); };
            c.cmp(t70, c.r32(7), 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t71) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000858f23); };
            c.w32(7, t71);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.logic(c.r8(7) & 0xffffffffffffff80u64, 8);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        49 => {
            c.w32(9, 0x1u64);
            blk = 50; continue;
        }
        50 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let r72 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r72);
            blk = 52; continue;
        }
        52 => {
            c.logic(c.r8(7) & 0x2u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let r73 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r73);
            blk = 54; continue;
        }
        54 => {
            c.logic(c.r8(7) & 0x40u64, 8);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let r74 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r74);
            blk = 56; continue;
        }
        56 => {
            c.logic(c.r8(7) & 0x4u64, 8);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let r75 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r75);
            blk = 58; continue;
        }
        58 => {
            c.logic(c.r8(7) & 0x20u64, 8);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let r76 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r76);
            blk = 60; continue;
        }
        60 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            c.w8(6, (c.cc_ne() as u64));
            c.w32(6, c.r8(6));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000858fbd); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let r77 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r77);
            blk = 62; continue;
        }
        62 => {
            c.logic(c.r8(7) & 0x8u64, 8);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000858fd6); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let r78 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r78);
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(1, 0xcu64);
            let r79 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r79);
            if c.cc_o() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000858ffa); };
            c.w64(1, t80);
            let Some(t81) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000859001); };
            c.w64(1, t81);
            let r82 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r82);
            let r83 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r83);
            c.logic(r83, 64);
            c.w64(7, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000859010); };
            c.w64(6, t84);
            let Some(t85) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000859017); };
            c.cmp(t85, c.r64(7), 64);
            if c.cc_be() { blk = 119; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 119; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085902d); };
            c.w64(2, t86);
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x7000859034); }
            let r87 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r87);
            blk = 69; continue;
        }
        69 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700085903b); };
            c.w64(7, t88);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000859042); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000859045); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700085904c); }
            c.w64(9, c.r64(1));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000859053); };
            c.w64(2, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859057); };
            c.w64(1, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085905b); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085905f); };
            c.w64(6, t92);
            blk = 72; continue;
        }
        70 => {
            return RegfileExit::fault(0x7000859065);
        }
        71 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000859066); };
            c.w64(9, t93);
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(8, 0xbu64);
            blk = 75; continue;
        }
        74 => {
            if !c.st8(c.r64(9).wrapping_add(0xb), 0x64u64) { return RegfileExit::fault(0x700085907b); }
            c.w32(8, 0xcu64);
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), 0x67u64) { return RegfileExit::fault(0x700085908c); }
            let r94 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r94);
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), 0x69u64) { return RegfileExit::fault(0x700085909b); }
            let r95 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r95);
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), 0x6cu64) { return RegfileExit::fault(0x70008590aa); }
            let r96 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r96);
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), 0x6du64) { return RegfileExit::fault(0x70008590b8); }
            let r97 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r97);
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), 0x73u64) { return RegfileExit::fault(0x70008590c6); }
            let r98 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r98);
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(11, c.r64(8).wrapping_add(0x1));
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), 0x75u64) { return RegfileExit::fault(0x70008590d8); }
            c.w64(8, c.r64(11));
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), 0x79u64) { return RegfileExit::fault(0x70008590e5); }
            blk = 89; continue;
        }
        89 => {
            c.w64(7, c.r64(9));
            blk = 90; continue;
        }
        90 => {
            c.w64(0, c.r64(1));
            blk = 92; continue;
        }
        91 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008590f2); };
            c.w64(7, t99);
            blk = 92; continue;
        }
        92 => {
            let Some(t100) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008590f6); };
            c.cmp(c.r32(2), t100, 32);
            if c.cc_ne() { blk = 121; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008590ff); };
            c.w64(1, t101);
            let Some(t102) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000859103); };
            c.w32(1, t102);
            let r103 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r103);
            let Some(t104) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000859109); };
            c.w32(8, t104);
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085910e); };
            c.w64(9, t105);
            let Some(t106) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000859115); };
            c.w64(11, t106);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000859124); };
            c.w64(12, t107);
            let Some(t108) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085912b); };
            c.cmp(t108, c.r64(8), 64);
            if c.cc_be() { blk = 166; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(9), c.r64(8)) { return RegfileExit::fault(0x7000859135); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700085913c); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000859140); };
            c.w64(1, t109);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000859147); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700085914b); }
            c.w64(1, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_e() { blk = 98; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000859159); };
            c.w64(9, t110);
            blk = 97; continue;
        }
        97 => {
            let r111 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r111);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x7000859164); }
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_ne() { blk = 97; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085916e); };
            c.cmp(t112, c.r32(0), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 99; continue;
        }
        99 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t113) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085917f); };
            c.w32(1, t113);
            let r114 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r114);
            let Some(t115) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859185); };
            c.cmp(t115, 0x40u64, 16);
            if c.cc_b() { blk = 103; continue; }
            blk = 101; continue;
        }
        101 => {
            blk = 168; continue;
        }
        102 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000859191); };
            c.w64(0, t116);
            blk = 103; continue;
        }
        103 => {
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000859198); };
            c.cmp(t117, c.r32(7), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t118) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008591ab); };
            c.w32(1, t118);
            let r119 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r119);
            let Some(t120) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008591b1); };
            c.cmp(t120, 0x40u64, 16);
            if c.cc_b() { blk = 108; continue; }
            blk = 106; continue;
        }
        106 => {
            blk = 170; continue;
        }
        107 => {
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008591bd); };
            c.w64(7, t121);
            blk = 108; continue;
        }
        108 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008591c4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008591c6); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008591c7); }
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2670)) else { return RegfileExit::fault(0x70008591c8); };
            c.w64(3, t122);
            c.w32(0, 0x3u64);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008591d4); };
            c.w64(6, t123);
            return c.call(0x700086ed74, 0x70008591dd, 0x70008591d8);
        }
        109 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008591dd); };
            c.w64(1, t124);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008591e4); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return c.ret(0x18, 0x70008591eb);
        }
        111 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008591ee); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008591f5); }
            return c.ret(0x0, 0x70008591f7);
        }
        112 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008591f8); };
            c.w64(0, t125);
            blk = 109; continue;
        }
        113 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008591fe); };
            c.w64(3, t126);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085920c, 0x7000859207);
        }
        114 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085920c); };
            c.w64(6, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000859210); };
            c.w64(2, t128);
            blk = 1; continue;
        }
        115 => {
            let Some(t129) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000859219); };
            c.logic(t129 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 116; continue;
        }
        116 => {
            blk = 15; continue;
        }
        117 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700085922b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 15; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 21; continue;
        }
        119 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000859243); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000859247); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700085924b); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x700085924f); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x7000859253); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x7000859257); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x700085925b); }
            return c.call(0x70007febb4, 0x7000859264, 0x700085925f);
        }
        120 => {
            c.w64(1, c.r64(0));
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000859267); };
            c.w64(3, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085926b); };
            c.w64(0, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085926f); };
            c.w64(15, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000859273); };
            c.w64(12, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000859277); };
            c.w64(11, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700085927b); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700085927f); };
            c.w64(9, t136);
            blk = 69; continue;
        }
        121 => {
            let Some(t137) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000859288); };
            c.w32(1, t137);
            let r138 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r138);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085928e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000859292); }
            let Some(t139) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000859296); };
            c.logic(t139 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t140) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008592a0); };
            c.w32(1, t140);
            let r141 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r141);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008592a6); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t142) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008592b3); };
            c.w32(8, t142);
            let r143 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r143);
            let Some(t144) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008592ba); };
            c.cmp(t144, 0xacu64, 16);
            if c.cc_ne() { blk = 164; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008592c7); };
            c.w64(8, t145);
            let Some(t146) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008592cb); };
            c.cmp(t146, c.r32(8), 32);
            if c.cc_ne() { blk = 164; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t147) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008592d5); };
            c.logic(t147 & 0x200000u64, 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008592de); };
            c.w64(12, t148);
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008592e5); };
            c.w64(9, t149);
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008592ec); };
            c.w64(3, t150);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008592f5); };
            c.w64(8, t151);
            blk = 146; continue;
        }
        127 => {
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000859301); };
            c.w64(8, t152);
            let Some(t153) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000859308); };
            c.w64(9, t153);
            c.w64(11, c.r64(9).wrapping_add(0x4c));
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085930f); };
            c.w64(12, t154);
            let Some(t155) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000859316); };
            c.cmp(t155, c.r64(11), 64);
            if c.cc_a() { blk = 130; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w32(2, 0x4cu64);
            return c.call(0x70007febf4, 0x7000859326, 0x7000859321);
        }
        129 => {
            c.w64(9, c.r64(0));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859329); };
            c.w64(7, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085932d); };
            c.w64(0, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000859331); };
            c.w64(1, t158);
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000859335); };
            c.w64(8, t159);
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085933c); };
            c.w64(12, t160);
            blk = 131; continue;
        }
        130 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000859345); }
            let r161 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r161);
            blk = 131; continue;
        }
        131 => {
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x700085934c); };
            c.w64(11, t162);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000859353); }
            c.w32(11, 0x22u64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700085935d); }
            let Some(t163) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000859361); };
            c.w32(11, t163);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            return RegfileExit::fault(0x7000859371);
        }
        133 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000859374); }
            let Some(t164) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000859378); };
            c.w32(11, t164);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_a() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            return RegfileExit::fault(0x7000859388);
        }
        135 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000859389); }
            let Some(t165) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085938d); };
            c.w32(11, t165);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            return RegfileExit::fault(0x700085939d);
        }
        137 => {
            c.w32(11, 0x8u64);
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008593a4); }
            let Some(t166) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008593a8); };
            c.w32(11, t166);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_a() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            return RegfileExit::fault(0x70008593b8);
        }
        139 => {
            c.w32(11, 0x2u64);
            if !c.st32(c.r64(9).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70008593bf); }
            let Some(t167) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008593c3); };
            c.w32(11, t167);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_a() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            return RegfileExit::fault(0x70008593d3);
        }
        141 => {
            if !c.st32(c.r64(9).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x70008593d4); }
            c.w64(11, c.r64(9).wrapping_add(0x4b));
            c.w64(15, c.r64(9).wrapping_add(0x1b));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 145; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008593e5); };
            c.w64(3, t168);
            blk = 143; continue;
        }
        143 => {
            if !c.st32(c.r64(15), c.r32(3)) { return RegfileExit::fault(0x70008593ec); }
            let r169 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r169);
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 143; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 146; continue;
        }
        145 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008593fa); };
            c.w64(3, t170);
            blk = 146; continue;
        }
        146 => {
            let Some(t171) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000859401); };
            c.w32(11, t171);
            c.w64(2, c.r64(11).wrapping_mul(4));
            let Some(t172) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085940e); };
            c.w64(11, t172);
            c.w64(15, c.r64(2).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000859415); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000859419); }
            let Some(t173) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085941d); };
            c.cmp(t173, c.r64(15), 64);
            if c.cc_a() { blk = 149; continue; }
            blk = 147; continue;
        }
        147 => {
            return c.call(0x70007febf4, 0x7000859428, 0x7000859423);
        }
        148 => {
            c.w64(8, c.r64(0));
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085942b); };
            c.w64(7, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085942f); };
            c.w64(0, t175);
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000859433); };
            c.w64(3, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085943a); };
            c.w64(1, t177);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000859440); };
            c.w64(9, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000859444); };
            c.w64(2, t179);
            blk = 150; continue;
        }
        149 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x700085944a); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 150; continue;
        }
        150 => {
            c.w64(11, c.r64(2));
            let r180 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r180);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000859458); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700085945c); }
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000859460); };
            c.w64(9, t181);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000859467); }
            let Some(t182) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085946b); };
            c.w32(9, t182);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700085946f); }
            c.w64(12, c.r64(9));
            let r183 = c.r32(12) & 0xe0000000u64;
            c.w32(12, r183);
            c.logic(r183, 32);
            if c.cc_ne() { blk = 155; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 163; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(15, c.r64(3));
            blk = 153; continue;
        }
        153 => {
            let r184 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r184);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(15)) { return RegfileExit::fault(0x7000859490); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 153; continue; }
            blk = 154; continue;
        }
        154 => {
            blk = 163; continue;
        }
        155 => {
            let r185 = c.sub(c.r32(9), 0x20000000u64, 32);
            c.w32(9, r185);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008594a3); }
            let Some(t186) = c.ld8(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008594a7); };
            c.w32(9, t186);
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_e() { blk = 158; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008594b9); };
            c.w64(15, t187);
            blk = 157; continue;
        }
        157 => {
            let r188 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r188);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(15)) { return RegfileExit::fault(0x70008594c4); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ne() { blk = 157; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_e() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let r189 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r189);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(3)) { return RegfileExit::fault(0x70008594d7); }
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_ne() { blk = 159; continue; }
            blk = 160; continue;
        }
        160 => {
            let r190 = c.add(c.r32(12), 0xe0000000u64, 32);
            c.w32(12, r190);
            if c.cc_ne() { blk = 163; continue; }
            blk = 161; continue;
        }
        161 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008594ea); }
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008594eb); };
            c.w64(3, t191);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008594fc, 0x70008594f7);
        }
        162 => {
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008594fc); };
            c.w64(7, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859500); };
            c.w64(0, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000859504); };
            c.w64(8, t194);
            blk = 163; continue;
        }
        163 => {
            c.w64(0, c.r64(8));
            blk = 165; continue;
        }
        164 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085950d); };
            if !c.push64(t195) { return RegfileExit::fault(0x700085950d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000859510); }
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0x2408)) else { return RegfileExit::fault(0x7000859511); };
            c.w64(3, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000859518); };
            c.w64(6, t197);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000859529, 0x7000859524);
        }
        165 => {
            c.w64(8, c.r64(0));
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085952c); };
            c.w64(7, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859530); };
            c.w64(0, t199);
            blk = 98; continue;
        }
        166 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000859539); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700085953d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000859541); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000859545); }
            return c.call(0x70007febf4, 0x700085954e, 0x7000859549);
        }
        167 => {
            c.w64(8, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859551); };
            c.w64(7, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859555); };
            c.w64(0, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000859559); };
            c.w64(2, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085955d); };
            c.w64(1, t203);
            blk = 95; continue;
        }
        168 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000859566); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085956a); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085956e); };
            c.w64(6, t204);
            return c.call(0x700089c794, 0x7000859577, 0x7000859572);
        }
        169 => {
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859577); };
            c.w64(7, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085957b); };
            c.w64(8, t206);
            blk = 103; continue;
        }
        170 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000859584); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000859588); }
            c.w64(0, c.r64(7));
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085958f); };
            c.w64(6, t207);
            return c.call(0x700089c794, 0x7000859598, 0x7000859593);
        }
        171 => {
            c.w64(7, c.r64(0));
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085959b); };
            c.w64(8, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085959f); };
            c.w64(0, t209);
            blk = 108; continue;
        }
        172 => {
            return RegfileExit::jump(0x70008595ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008595b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b18)) else { return RegfileExit::fault(0x70008595b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008595d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b20)) else { return RegfileExit::fault(0x70008595d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008595f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b28)) else { return RegfileExit::fault(0x70008595f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000859614(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b30)) else { return RegfileExit::fault(0x7000859614); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000859634(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b38)) else { return RegfileExit::fault(0x7000859634); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000859654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000859654); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000859658); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000859659); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085965a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085965f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000859663); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 67; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085966d); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085967a); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859680); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 75; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085968c); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085969a); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008596a1); };
            c.cmp(t9, 0x83eu64, 16);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008596aa); };
            c.w64(8, t10);
            blk = 53; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008596b3); };
            c.w64(8, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008596b7); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(1, 0x142u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008596c5); }
            c.w32(0, 0x1u64);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008596cb); };
            c.w64(3, t13);
            return c.call(0x700086ed74, 0x70008596d7, 0x70008596d2);
        }
        8 => {
            return RegfileExit::fault(0x70008596d7);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008596d8); };
            c.w32(8, t14);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(8) & 0xffffffffffffff80u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        11 => {
            c.w32(0, 0x1u64);
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let r15 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r15);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let r16 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r16);
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(8) & 0x40u64, 8);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let r17 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r17);
            blk = 18; continue;
        }
        18 => {
            c.logic(c.r8(8) & 0x4u64, 8);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let r18 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r18);
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000859758); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let r19 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r19);
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(8) & 0x10u64, 8);
            c.w8(6, (c.cc_ne() as u64));
            c.w32(6, c.r8(6));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000859770); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let r20 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r20);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(8) & 0x8u64, 8);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000859788); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let r21 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r21);
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r22 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r22);
            if c.cc_o() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008597a7); };
            c.w64(7, t23);
            let Some(t24) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008597ae); };
            c.w64(8, t24);
            let r25 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r25);
            let r26 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r26);
            c.logic(r26, 64);
            c.w64(6, c.r64(8).wrapping_add(c.r64(1)));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008597bd); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008597c4); };
            c.cmp(t28, c.r64(6), 64);
            if c.cc_be() { blk = 69; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 69; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x70008597da); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008597e1); };
            c.w64(7, t29);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008597e8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008597eb); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008597f2); }
            c.w64(0, c.r64(1));
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008597f8); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008597fc); };
            c.w64(7, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859800); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859804); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000859808); };
            c.w64(2, t34);
            blk = 34; continue;
        }
        32 => {
            return RegfileExit::fault(0x700085980e);
        }
        33 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700085980f); };
            c.w64(0, t35);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(9, 0xbu64);
            blk = 37; continue;
        }
        36 => {
            if !c.st8(c.r64(0).wrapping_add(0xb), 0x64u64) { return RegfileExit::fault(0x7000859824); }
            c.w32(9, 0xcu64);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(9)), 0x67u64) { return RegfileExit::fault(0x7000859834); }
            let r36 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r36);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(9)), 0x69u64) { return RegfileExit::fault(0x7000859843); }
            let r37 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r37);
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(9)), 0x6cu64) { return RegfileExit::fault(0x7000859852); }
            let r38 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r38);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(9)), 0x6du64) { return RegfileExit::fault(0x7000859860); }
            let r39 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r39);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(9)), 0x73u64) { return RegfileExit::fault(0x700085986e); }
            let r40 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r40);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(11, c.r64(9).wrapping_add(0x1));
            if !c.st8(c.r64(0).wrapping_add(c.r64(9)), 0x75u64) { return RegfileExit::fault(0x7000859880); }
            c.w64(9, c.r64(11));
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(9)), 0x79u64) { return RegfileExit::fault(0x700085988e); }
            blk = 51; continue;
        }
        51 => {
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000859893); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000859899); };
            c.w64(6, t43);
            c.w64(8, c.r64(0));
            blk = 53; continue;
        }
        52 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008598a2); };
            c.w64(8, t44);
            blk = 53; continue;
        }
        53 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008598a6); };
            c.cmp(t45, c.r32(7), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t46) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008598b9); };
            c.w32(9, t46);
            let r47 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r47);
            let Some(t48) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008598c0); };
            c.cmp(t48, 0x40u64, 16);
            if c.cc_b() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 71; continue;
        }
        57 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008598cd); };
            c.w64(7, t49);
            blk = 58; continue;
        }
        58 => {
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008598d4); };
            c.cmp(t50, c.r32(8), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 59; continue;
        }
        59 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t51) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008598e7); };
            c.w32(9, t51);
            let r52 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r52);
            let Some(t53) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008598ee); };
            c.cmp(t53, 0x40u64, 16);
            if c.cc_b() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 73; continue;
        }
        62 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008598fb); };
            c.w64(8, t54);
            blk = 63; continue;
        }
        63 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000859902); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000859903); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000859904); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2670)) else { return RegfileExit::fault(0x7000859906); };
            c.w64(3, t55);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000859917, 0x7000859912);
        }
        64 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000859917); };
            c.w64(1, t56);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085991e); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            return c.ret(0x18, 0x7000859925);
        }
        66 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000859928); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085992f); }
            return c.ret(0x0, 0x7000859931);
        }
        67 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000859934); };
            c.w64(3, t57);
            return c.call(0x700086ed74, 0x7000859940, 0x700085993b);
        }
        68 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000859940); };
            c.w64(6, t58);
            blk = 1; continue;
        }
        69 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000859949); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700085994d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000859951); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000859955); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000859959); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x700085995d); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000859969, 0x7000859964);
        }
        70 => {
            c.w64(1, c.r64(0));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085996c); };
            c.w64(3, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000859970); };
            c.w64(15, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000859974); };
            c.w64(12, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000859978); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700085997c); };
            c.w64(9, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000859980); };
            c.w64(0, t64);
            blk = 31; continue;
        }
        71 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000859989); }
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x7000859995, 0x7000859990);
        }
        72 => {
            c.w64(7, c.r64(0));
            let Some(t65) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000859998); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085999c); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008599a0); };
            c.w64(8, t67);
            blk = 58; continue;
        }
        73 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008599a9); }
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008599b5, 0x70008599b0);
        }
        74 => {
            c.w64(8, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008599b8); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008599bc); };
            c.w64(6, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008599c0); };
            c.w64(7, t70);
            blk = 63; continue;
        }
        75 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008599ce); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008599cf); };
            c.w64(7, t71);
            let Some(t72) = c.ld32(c.r64(7).wrapping_add(0x5f)) else { return RegfileExit::fault(0x70008599d6); };
            c.w32(7, t72);
            let r73 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r73);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008599dc); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008599dd); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008599de); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008599ef, 0x70008599ea);
        }
        76 => {
            return RegfileExit::fault(0x70008599ef);
        }
        77 => {
            return RegfileExit::jump(0x70008599f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000859a14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b40)) else { return RegfileExit::fault(0x7000859a14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000859a34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b48)) else { return RegfileExit::fault(0x7000859a34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000859a54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000859a54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000859a58); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000859a5e); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000859a64); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000859a6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000859a6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000859a72); }
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(7));
            return c.call(0x70008e8b74, 0x7000859a81, 0x7000859a7c);
        }
        1 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000859a86); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859a8a); };
            c.w32(2, t6);
            let r7 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r7);
            c.w32(7, 0x4u64);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000859a94); };
            c.w64(8, t8);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000859a98); }
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000859a9c); };
            c.w64(7, t9);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000859aa0); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r10 = c.r64(9) & c.r64(8);
            c.w64(9, r10);
            c.logic(r10, 64);
            let Some(t11) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859ab4); };
            c.logic(t11 & 0x4u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            let r12 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r12);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000859abd); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r13 = c.r64(9) & c.r64(8);
            c.w64(9, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859ad1); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x7000859adb); }
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x7000859adf); }
            c.w64(0, c.r64(8));
            blk = 7; continue;
        }
        7 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000859ae9); }
            return c.ret(0x0, 0x7000859aea);
        }
        8 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000859aeb); };
            c.w64(0, t15);
            blk = 7; continue;
        }
        9 => {
            c.w32(7, c.r32(7));
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            c.w64(9, 0xfffffffffffc0000u64);
            let r17 = c.r64(9) & c.r64(7);
            c.w64(9, r17);
            c.logic(r17, 64);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859b03); };
            c.logic(t18 & 0x2u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(11, c.r64(8).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000859b0e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000859b0f); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000859b1c, 0x7000859b17);
        }
        11 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000859b1c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000859b1e); }
            blk = 6; continue;
        }
        12 => {
            c.w32(7, c.r32(7));
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            c.w64(9, 0xfffffffffffc0000u64);
            let r20 = c.r64(9) & c.r64(7);
            c.w64(9, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859b30); };
            c.logic(t21 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(11, c.r64(8).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000859b3b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000859b3c); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000859b49, 0x7000859b44);
        }
        14 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000859b49); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000859b4b); }
            blk = 4; continue;
        }
        15 => {
            return RegfileExit::jump(0x7000859b54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000859b74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000859b74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000859b78); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000859b7e); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859b84); };
            c.w32(7, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000859b88); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000859b8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000859b90); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000859b94); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000859b98); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859ba8); };
            c.w32(12, t4);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_b() { blk = 50; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000859bb8); };
            c.w32(15, t5);
            let r6 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r6);
            let Some(t7) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859bbf); };
            c.w32(8, t7);
            let r8 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r8);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000859bc6); }
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 51; continue;
        }
        6 => {
            blk = 9; continue;
        }
        7 => {
            return c.call(0x7000859a54, 0x7000859be8, 0x7000859be3);
        }
        8 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000859beb); }
            return c.ret(0x0, 0x7000859bec);
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(15).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000859bed); };
            c.w32(8, t9);
            c.cmp(c.r32(8), 0x7eu64, 32);
            if c.cc_a() { blk = 64; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000859c05); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 13; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000859c10); }
            blk = 13; continue;
        }
        13 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000859c14); };
            c.w64(2, t10);
            let r11 = c.r32(2) & 0x7u64;
            c.w32(2, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 64; continue;
        }
        18 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 64; continue;
        }
        21 => {
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000859c47); };
            c.w32(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000859c4a); };
            c.cmp(t13, c.r32(2), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000859c57); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000859c5d); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            let Some(t18) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859c63); };
            c.w32(2, t18);
            blk = 12; continue;
        }
        23 => {
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000859c69); };
            c.w32(2, t19);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r20 = c.add(c.r64(7), c.r64(2), 64);
            c.w64(7, r20);
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000859c75); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            let Some(t23) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000859c7b); };
            c.w32(2, t23);
            let r24 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r24);
            let Some(t25) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859c81); };
            c.w32(2, t25);
            blk = 12; continue;
        }
        24 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000859c87); };
            c.w32(3, t26);
            let r27 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r27);
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000859c8d); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000859c93); };
            c.w32(2, t30);
            blk = 12; continue;
        }
        25 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        26 => {
            c.w32(2, 0x1u64);
            blk = 28; continue;
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000859ca7); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 29; continue;
        }
        29 => {
            let r31 = c.add(c.r64(3), 0xbu64, 64);
            c.w64(3, r31);
            blk = 30; continue;
        }
        30 => {
            let r32 = c.add(c.r64(12), c.r64(7), 64);
            c.w64(12, r32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000859cc2); };
            c.w64(2, t33);
            let r34 = c.r32(2) & 0x8u64;
            c.w32(2, r34);
            c.logic(r34, 32);
            let r35 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r35);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let r36 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r36);
            let r37 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r37);
            let Some(t38) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000859cdb); };
            c.w32(3, t38);
            let r39 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r39);
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000859ce2); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t40) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000859cef); };
            c.w32(2, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x7000859cf2); };
            c.w64(7, t41);
            let Some(t42) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000859cf9); };
            c.w64(7, t42);
            c.cmp(c.r32(2), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(3, c.r64(2).wrapping_mul(8));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13));
            let Some(t43) = c.ld64(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000859d0e); };
            c.w64(3, t43);
            blk = 36; continue;
        }
        35 => {
            let r44 = c.add(c.r64(3), 0x3fu64, 64);
            c.w64(3, r44);
            c.w64(2, c.r64(13));
            blk = 36; continue;
        }
        36 => {
            let r45 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r45);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w32(11, c.r32(11));
            c.w64(10, c.r64(4));
            let r46 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r46);
            let r47 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r47);
            c.logic(r47, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x7000859d37); }
            let r48 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r48);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000859d3f); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x7000859d43); };
            c.w64(11, t49);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x7000859d4a); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(8)) { return RegfileExit::fault(0x7000859d4f); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(7)) { return RegfileExit::fault(0x7000859d54); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x7000859d59); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(2)) { return RegfileExit::fault(0x7000859d62); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x7000859d67); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000859d6c); };
            c.w64(1, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000859d70); };
            c.w64(2, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000859d74); };
            c.w64(8, t52);
            c.w64(9, c.r64(12));
            c.w64(10, 0x7000859d82u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000859d82); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000859d86); }
            return c.call(c.r64(3), 0x7000859d8c, 0x7000859d8a);
        }
        37 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000859d8c); }
            let Some(t53) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x7000859d94); };
            c.w64(4, t53);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 56; continue;
        }
        40 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859db1); };
            c.w64(0, t54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000859db5); };
            c.w32(7, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000859db8); };
            c.w64(11, t56);
            let Some(t57) = c.ld32(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000859dbc); };
            c.w32(11, t57);
            let r58 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r58);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r59 = c.sub(c.r32(7), 0x6u64, 32);
            c.w32(7, r59);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_l() { blk = 63; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000859dd9); }
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000859ddd); };
            c.w64(11, t60);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000859de1); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r61 = c.r64(12) & c.r64(0);
            c.w64(12, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859df5); };
            c.logic(t62 & 0x4u64, 8);
            if c.cc_ne() { blk = 69; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000859e01); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r63 = c.r64(12) & c.r64(0);
            c.w64(12, r63);
            c.logic(r63, 64);
            let Some(t64) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859e15); };
            c.logic(t64 & 0x4u64, 8);
            if c.cc_ne() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if c.cc_e() { blk = 49; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(11, 0x13u64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x7000859e2f); };
            c.w64(12, t65);
            blk = 48; continue;
        }
        47 => {
            let r66 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r66);
            c.w64(9, c.r64(15));
            blk = 48; continue;
        }
        48 => {
            c.w64(15, c.r64(9).wrapping_add(0x4));
            let Some(t67) = c.ld32(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000859e43); };
            c.w32(3, t67);
            let r68 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r68);
            if !c.st32(c.r64(0).wrapping_add(c.r64(11)), c.r32(3)) { return RegfileExit::fault(0x7000859e49); }
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_ne() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 8; continue;
        }
        50 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000859e57); };
            c.w64(0, t69);
            blk = 8; continue;
        }
        51 => {
            return RegfileExit::fault(0x7000859e63);
        }
        52 => {
            blk = 64; continue;
        }
        53 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000859e69); };
            c.w64(2, t70);
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t71) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000859e76); };
            c.w64(3, t71);
            blk = 30; continue;
        }
        55 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(7, c.r64(3).wrapping_add(c.r64(7).wrapping_mul(2)));
            let Some(t72) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000859e87); };
            c.w32(3, t72);
            let r73 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r73);
            blk = 32; continue;
        }
        56 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000859e9f); };
            c.w64(0, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000859ea3); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859ea7); };
            c.w64(6, t76);
            blk = 64; continue;
        }
        59 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000859ead); };
            if !c.push64(t77) { return RegfileExit::fault(0x7000859ead); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000859eb0); };
            if !c.push64(t78) { return RegfileExit::fault(0x7000859eb0); }
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000859eb3); };
            if !c.push64(t79) { return RegfileExit::fault(0x7000859eb3); }
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859eb6); };
            if !c.push64(t80) { return RegfileExit::fault(0x7000859eb6); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x7000859eb9); };
            c.w64(3, t81);
            c.w32(0, 0x4u64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859ec5); };
            c.w64(6, t82);
            return c.call(0x700086ed74, 0x7000859ece, 0x7000859ec9);
        }
        60 => {
            blk = 8; continue;
        }
        61 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x7000859ed3); };
            c.w64(3, t83);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859edc); };
            c.w64(6, t84);
            return c.call(0x700086ed74, 0x7000859ee5, 0x7000859ee0);
        }
        62 => {
            return RegfileExit::fault(0x7000859ee5);
        }
        63 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000859ee6); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000859eea); };
            c.w64(1, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000859eee); };
            c.w64(6, t87);
            blk = 64; continue;
        }
        64 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000859ef2); }
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000859ef3); };
            if !c.push64(t88) { return RegfileExit::fault(0x7000859ef3); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000859ef6); }
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000859ef7); };
            if !c.push64(t89) { return RegfileExit::fault(0x7000859ef7); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x7000859efa); };
            c.w64(3, t90);
            c.w32(0, 0x4u64);
            return c.call(0x700086ed74, 0x7000859f0b, 0x7000859f06);
        }
        65 => {
            blk = 8; continue;
        }
        66 => {
            c.w32(11, c.r32(11));
            let r91 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r91);
            c.w64(12, 0xfffffffffffc0000u64);
            let r92 = c.r64(12) & c.r64(11);
            c.w64(12, r92);
            c.logic(r92, 64);
            let Some(t93) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859f20); };
            c.logic(t93 & 0x2u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(15, c.r64(0).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000859f30); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000859f31); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000859f3e, 0x7000859f39);
        }
        68 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000859f3e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000859f40); }
            blk = 45; continue;
        }
        69 => {
            c.w32(11, c.r32(11));
            let r94 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r94);
            c.w64(12, 0xfffffffffffc0000u64);
            let r95 = c.r64(12) & c.r64(11);
            c.w64(12, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000859f56); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(15, c.r64(0).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000859f66); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000859f67); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000859f74, 0x7000859f6f);
        }
        71 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000859f74); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000859f76); }
            blk = 43; continue;
        }
        72 => {
            return RegfileExit::jump(0x7000859f80);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000859f94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1680)) else { return RegfileExit::fault(0x7000859f94); };
            return RegfileExit::jump(t0);
        }
        1 => {
            return RegfileExit::fault(0x7000859f9b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000859fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1688)) else { return RegfileExit::fault(0x7000859fb4); };
            return RegfileExit::jump(t0);
        }
        1 => {
            return RegfileExit::fault(0x7000859fbb);
        }
            _ => unreachable!(),
        }
    }
}
