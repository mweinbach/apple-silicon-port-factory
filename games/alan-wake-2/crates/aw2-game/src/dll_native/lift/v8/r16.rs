#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000875c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000875c54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000875c58); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000875c5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000875c62); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000875c66); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000875c6a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 807; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000875c76); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000875c7c); };
            c.w32(8, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000875c81); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000875c85); }
            c.cmp(c.r16(8), 0x411u64, 16);
            if c.cc_be() { blk = 806; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 387; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000875c9e); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000875ca5); };
            c.w32(9, t6);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000875cb1); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r7);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 388; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 809; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000875d02); };
            c.w32(11, t8);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(8, c.r64(3));
            blk = 27; continue;
        }
        15 => {
            c.w64(10, c.r64(4));
            let r9 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r9);
            let r10 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r10);
            c.logic(r10, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000875d4c); }
            c.w64(2, c.r64(3));
            c.w64(1, c.r64(13));
            c.w64(9, c.r64(1));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x7000875d5a); };
            c.w64(0, t11);
            c.w64(10, 0x7000875d68u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000875d68); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000875d6c); }
            return c.call(c.r64(0), 0x7000875d72, 0x7000875d70);
        }
        16 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000875d72); }
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000875d7a); };
            c.w64(4, t12);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 22; continue; }
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
            c.w64(11, c.r64(7));
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875d98); };
            c.w64(0, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000875d9c); };
            c.w64(3, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000875da0); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000875da4); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000875da8); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000875dac); };
            c.w64(8, t18);
            blk = 388; continue;
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875db5); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000875db9); };
            c.w64(1, t20);
            blk = 808; continue;
        }
        21 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875dc2); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000875dc6); };
            c.w64(1, t22);
            blk = 808; continue;
        }
        22 => {
            c.w64(8, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875dd2); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000875dd6); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000875dda); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000875dde); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000875de2); };
            c.w64(7, t27);
            blk = 27; continue;
        }
        23 => {
            c.w64(8, c.r64(3));
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000875deb); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            c.w64(3, c.r64(8));
            c.w64(8, c.r64(9));
            blk = 27; continue;
        }
        24 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        25 => {
            if let Some(sr) = c.shr(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let r30 = c.r64(11) & 0xffffffu64;
            c.w64(11, r30);
            c.logic(r30, 64);
            blk = 388; continue;
        }
        26 => {
            c.w64(8, c.r64(3));
            blk = 27; continue;
        }
        27 => {
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000875e15); };
            c.w32(9, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000875e19); }
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 311; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t32) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000875e2a); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            c.logic(c.r32(9) & 0xffc00u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 29; continue;
        }
        29 => {
            let r34 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r34);
            let r35 = c.r32(9) & 0x3ffu64;
            c.w32(9, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_be() { blk = 42; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t36) = c.ld16(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000875e53); };
            c.w32(12, (t36 as u16 as i16 as i64 as u64));
            let Some(t37) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000875e58); };
            c.w32(15, t37);
            let r38 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r38);
            let r39 = c.sub(c.r32(12), 0x1u64, 32);
            c.w32(12, r39);
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000875e66); }
            c.w64(3, c.r64(2));
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000875e6f); }
            blk = 32; continue;
        }
        32 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000875e73); };
            c.w64(6, t40);
            let r41 = c.sub(c.r32(6), c.r32(3), 32);
            c.w32(6, r41);
            let r42 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r42);
            let r43 = c.add(c.r32(6), c.r32(3), 32);
            c.w32(6, r43);
            c.w32(2, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000875e83); };
            c.w32(2, t44);
            let r45 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r45);
            let r46 = c.shr(c.r32(2), 0x9u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r46);
            let r47 = c.r32(2) & 0x3ffu64;
            c.w32(2, r47);
            c.logic(r47, 32);
            c.w32(2, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t48) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000875e99); };
            c.w32(2, t48);
            let r49 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000875ea1); };
            c.w32(2, t50);
            let r51 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r51);
            c.cmp(c.r32(15), c.r32(2), 32);
            if c.cc_be() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(3, c.r64(6).wrapping_add(0x1));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000875eaf); };
            c.w64(6, t52);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 37; continue;
        }
        36 => {
            let r53 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r53);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_l() { blk = 182; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000875ecb); };
            c.w32(2, t54);
            let r55 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r55);
            let r56 = c.shr(c.r32(2), 0x9u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r56);
            let r57 = c.r32(2) & 0x3ffu64;
            c.w32(2, r57);
            c.logic(r57, 32);
            c.w32(6, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t58) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000875ee1); };
            c.w32(0, t58);
            let r59 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r59);
            let Some(t60) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000875ee9); };
            c.w32(7, t60);
            let r61 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r61);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_be() { blk = 182; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(9, c.r64(6));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000875f09); };
            c.w64(0, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000875f0d); };
            c.w64(3, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000875f11); };
            c.w64(6, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000875f15); };
            c.w64(7, t65);
            blk = 46; continue;
        }
        42 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r66 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r66);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 182; continue; }
            blk = 43; continue;
        }
        43 => {
            let r67 = c.sub(c.r64(9), 0x3u64, 64);
            c.w64(9, r67);
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000875f30); };
            c.cmp(t68, c.r32(8), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ne() { blk = 43; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 182; continue;
        }
        46 => {
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000875f46); };
            c.w32(12, t69);
            c.w32(15, c.r32(12));
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        48 => {
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000875f5c); };
            c.cmp(t70, c.r32(8), 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000875f69); };
            c.cmp(t71, c.r32(8), 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000875f76); };
            c.cmp(t72, c.r32(8), 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000875f83); };
            c.cmp(t73, c.r32(8), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000875f90); };
            c.cmp(t74, c.r32(8), 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000875f9d); };
            c.cmp(t75, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000875faa); };
            c.cmp(t76, c.r32(8), 32);
            if c.cc_e() { blk = 175; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(1));
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000875fc0); };
            c.cmp(t77, c.r32(2), 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(1, c.r64(2));
            blk = 173; continue;
        }
        58 => {
            c.w64(15, c.r64(0));
            c.w64(1, c.r64(2));
            blk = 808; continue;
        }
        59 => {
            c.w64(8, c.r64(15));
            let r78 = c.r32(8) & 0x380u64;
            c.w32(8, r78);
            c.logic(r78, 32);
            c.cmp(c.r32(8), 0x80u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(8), 0x100u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0x180u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        64 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t79) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087601e); };
            c.w32(9, t79);
            let r80 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r80);
            c.w32(11, 0x4u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(11, 0x2u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            let r81 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r81);
            c.logic(r81, 64);
            let Some(t82) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876046); };
            c.cmp(t82, c.r32(9), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        70 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        71 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        72 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        73 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t83) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876071); };
            c.w32(9, t83);
            let Some(t84) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000876075); };
            c.cmp(t84, c.r32(9), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        76 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        78 => {
            let r85 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r85);
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t86) = c.ld8(c.r64(7).wrapping_add(0x4)) else { return RegfileExit::fault(0x700087609a); };
            c.w32(9, t86);
            let Some(t87) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087609f); };
            c.w32(7, t87);
            let r88 = c.r32(15) & 0x8u64;
            c.w32(15, r88);
            c.logic(r88, 32);
            let r89 = c.r64(12) & 0x3ffu64;
            c.w64(12, r89);
            c.logic(r89, 64);
            c.cmp(c.r32(8), 0x100u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(15), 0x8u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r90 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r90);
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_b() { blk = 127; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(12, c.r64(0));
            let Some(t91) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008760d8); };
            c.w32(15, t91);
            let r92 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r92);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008760e6); };
            c.w64(15, t93);
            blk = 81; continue;
        }
        81 => {
            let r94 = c.sub(c.r64(9), c.r64(7), 64);
            c.w64(9, r94);
            c.w64(7, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x7000876108); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r95 = c.r64(8) & c.r64(15);
            c.w64(8, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087611b); };
            c.logic(t96 & 0x4u64, 8);
            if c.cc_ne() { blk = 991; continue; }
            blk = 85; continue;
        }
        85 => {
            blk = 173; continue;
        }
        86 => {
            let Some(t97) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087612b); };
            c.w32(7, t97);
            let r98 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r98);
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 87; continue;
        }
        87 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t99) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876152); };
            c.w32(8, t99);
            let Some(t100) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000876156); };
            c.cmp(t100, c.r32(8), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        91 => {
            let Some(t101) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876167); };
            c.w32(8, t101);
            let Some(t102) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087616b); };
            c.cmp(t102, c.r32(8), 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876174); }
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876179); }
            blk = 100; continue;
        }
        93 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        94 => {
            let Some(t103) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876188); };
            c.w32(8, t103);
            let Some(t104) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087618c); };
            c.cmp(t104, c.r32(8), 32);
            if c.cc_ne() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(8, c.r64(1));
            let r105 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r105);
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087619b); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 100; continue;
        }
        96 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        97 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t106) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008761bd); };
            c.w32(8, t106);
            let Some(t107) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008761c1); };
            c.cmp(t107, c.r32(8), 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 99; continue;
        }
        99 => {
            let r108 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r108);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008761cc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 100; continue;
        }
        100 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            if c.cc_e() { blk = 108; continue; }
            blk = 102; continue;
        }
        102 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            if c.cc_e() { blk = 107; continue; }
            blk = 104; continue;
        }
        104 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 173; continue; }
            blk = 105; continue;
        }
        105 => {
            if c.cc_ne() { blk = 173; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        107 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        108 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        110 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        111 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        112 => {
            let Some(t109) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087623c); };
            c.w32(7, t109);
            let r110 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r110);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876249); }
            blk = 115; continue;
        }
        114 => {
            c.w64(8, c.r64(1));
            let r111 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r111);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000876265); }
            blk = 173; continue;
        }
        117 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087626f); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            if c.cc_e() { blk = 125; continue; }
            blk = 119; continue;
        }
        119 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            if c.cc_e() { blk = 124; continue; }
            blk = 121; continue;
        }
        121 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 173; continue; }
            blk = 122; continue;
        }
        122 => {
            if c.cc_ne() { blk = 173; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        124 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        125 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(15, c.r64(12));
            blk = 808; continue;
        }
        127 => {
            c.w64(7, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 158; continue; }
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 132; continue; }
            blk = 129; continue;
        }
        129 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x70008762df); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r112 = c.r64(8) & c.r64(0);
            c.w64(8, r112);
            c.logic(r112, 64);
            let Some(t113) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008762f1); };
            c.logic(t113 & 0x4u64, 8);
            if c.cc_ne() { blk = 988; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 173; continue;
        }
        132 => {
            let Some(t114) = c.ld32(c.r64(0).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876301); };
            c.w32(7, t114);
            let r115 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r115);
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 133; continue;
        }
        133 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 140; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t116) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876328); };
            c.w32(8, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087632c); };
            c.cmp(t117, c.r32(8), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        137 => {
            let Some(t118) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087633d); };
            c.w32(8, t118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000876341); };
            c.cmp(t119, c.r32(8), 32);
            if c.cc_ne() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087634a); }
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700087634f); }
            blk = 146; continue;
        }
        139 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        140 => {
            let Some(t120) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087635e); };
            c.w32(8, t120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000876362); };
            c.cmp(t121, c.r32(8), 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w64(8, c.r64(1));
            let r122 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r122);
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876371); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 146; continue;
        }
        142 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        143 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t123) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876393); };
            c.w32(8, t123);
            let Some(t124) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000876397); };
            c.cmp(t124, c.r32(8), 32);
            if c.cc_ne() { blk = 156; continue; }
            blk = 145; continue;
        }
        145 => {
            let r125 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r125);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008763a2); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 146; continue;
        }
        146 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            if c.cc_e() { blk = 154; continue; }
            blk = 148; continue;
        }
        148 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            if c.cc_e() { blk = 153; continue; }
            blk = 150; continue;
        }
        150 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 173; continue; }
            blk = 151; continue;
        }
        151 => {
            if c.cc_ne() { blk = 173; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        153 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        154 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        156 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        157 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        158 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876417); }
            blk = 161; continue;
        }
        160 => {
            c.w64(7, c.r64(1));
            let r126 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r126);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 161; continue;
        }
        161 => {
            let Some(t127) = c.ld32(c.r64(0).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087642b); };
            c.w32(7, t127);
            let r128 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r128);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 162; continue;
        }
        162 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876439); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            if c.cc_e() { blk = 170; continue; }
            blk = 164; continue;
        }
        164 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            if c.cc_e() { blk = 169; continue; }
            blk = 166; continue;
        }
        166 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 172; continue; }
            blk = 167; continue;
        }
        167 => {
            if c.cc_ne() { blk = 172; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        169 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        170 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        172 => {
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000876485); }
            blk = 173; continue;
        }
        173 => {
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        174 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000876490); }
            return c.ret(0x0, 0x7000876491);
        }
        175 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        176 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        177 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        178 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        179 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        180 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        181 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        182 => {
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008764ca); };
            c.cmp(t129, c.r32(8), 32);
            if c.cc_e() { blk = 310; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008764d7); };
            c.cmp(t130, c.r32(8), 32);
            if c.cc_e() { blk = 309; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008764e4); };
            c.cmp(t131, c.r32(8), 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008764f1); };
            c.cmp(t132, c.r32(8), 32);
            if c.cc_e() { blk = 307; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x70008764fe); };
            c.cmp(t133, c.r32(8), 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t134) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x700087650b); };
            c.cmp(t134, c.r32(8), 32);
            if c.cc_e() { blk = 305; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t135) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000876518); };
            c.cmp(t135, c.r32(8), 32);
            if c.cc_e() { blk = 304; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000876525); };
            c.w64(7, t136);
            let Some(t137) = c.ld32(c.r64(7).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000876529); };
            c.w32(7, t137);
            let r138 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r138);
            c.w32(9, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 190; continue;
        }
        190 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 302; continue; }
            blk = 191; continue;
        }
        191 => {
            let r139 = c.r32(9) & 0x3u64;
            c.w32(9, r139);
            c.logic(r139, 32);
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            let r140 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r140);
            c.logic(r140, 64);
            blk = 223; continue;
        }
        193 => {
            let Some(t141) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876558); };
            c.w32(9, t141);
            let Some(t142) = c.ld32(c.r64(13).wrapping_add(0x148)) else { return RegfileExit::fault(0x700087655c); };
            c.cmp(t142, c.r32(9), 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876565); };
            c.w64(15, t143);
            blk = 808; continue;
        }
        195 => {
            let Some(t144) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087656e); };
            c.w32(9, t144);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            c.w8(9, (c.cc_b() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t145) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087658a); };
            c.w32(11, t145);
            let r146 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r146);
            c.w64(2, c.r64(11));
            blk = 198; continue;
        }
        197 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(2), 0x20u64, 32);
            if c.cc_be() { blk = 218; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t147) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008765b0); };
            c.w32(9, t147);
            let r148 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r148);
            blk = 203; continue;
        }
        202 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 203; continue;
        }
        203 => {
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008765bc); };
            c.w32(11, t149);
            let r150 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r150);
            let r151 = c.sub(c.r32(9), 0x1u64, 32);
            c.w32(9, r151);
            c.w64(15, c.r64(9));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 205; continue;
        }
        204 => {
            c.w64(15, c.r64(0));
            blk = 205; continue;
        }
        205 => {
            c.w64(0, c.r64(15));
            let r152 = c.sub(c.r32(0), c.r32(12), 32);
            c.w32(0, r152);
            let r153 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r153);
            let r154 = c.add(c.r32(0), c.r32(12), 32);
            c.w32(0, r154);
            c.w32(3, c.r64(0).wrapping_add(c.r64(0)));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t155) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008765e4); };
            c.w32(3, t155);
            let r156 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r156);
            let Some(t157) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008765eb); };
            c.w32(3, t157);
            let r158 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r158);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_be() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w32(12, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(12), c.r32(0), 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 208; continue;
        }
        208 => {
            blk = 210; continue;
        }
        209 => {
            let r159 = c.add(c.r32(12), 0x1u64, 32);
            c.w32(12, r159);
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_l() { blk = 217; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w32(15, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t160) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000876614); };
            c.w32(0, t160);
            let r161 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r161);
            let Some(t162) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087661c); };
            c.w32(3, t162);
            let r163 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r163);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_ne() { blk = 216; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ne() { blk = 209; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_be() { blk = 215; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(9, c.r64(15).wrapping_add(0x2));
            blk = 222; continue;
        }
        215 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876637); };
            c.w64(15, t164);
            blk = 808; continue;
        }
        216 => {
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876640); };
            c.w64(15, t165);
            blk = 808; continue;
        }
        217 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876649); };
            c.w64(15, t166);
            blk = 808; continue;
        }
        218 => {
            c.w64(9, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x2));
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_e() { blk = 300; continue; }
            blk = 219; continue;
        }
        219 => {
            let r167 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r167);
            let Some(t168) = c.ld32(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087666b); };
            c.cmp(t168, c.r32(8), 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_ne() { blk = 219; continue; }
            blk = 221; continue;
        }
        221 => {
            blk = 300; continue;
        }
        222 => {
            let Some(t169) = c.ld32(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087667d); };
            c.w32(7, t169);
            let r170 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r170);
            let r171 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r171);
            c.logic(r171, 64);
            blk = 223; continue;
        }
        223 => {
            let Some(t172) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000876689); };
            c.w32(9, t172);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700087668d); }
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 299; continue; }
            blk = 224; continue;
        }
        224 => {
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r173 = c.r64(9) & 0x3ffu64;
            c.w64(9, r173);
            c.logic(r173, 64);
            c.w64(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t174) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008766b2); };
            c.w32(11, t174);
            let r175 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r175);
            let Some(t176) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008766b9); };
            c.cmp(t176, c.r32(8), 32);
            if c.cc_ne() { blk = 298; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t177) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008766c4); };
            c.w32(12, t177);
            c.w32(15, c.r32(12));
            c.logic(c.r8(12) & 0x52u64, 8);
            if c.cc_ne() { blk = 297; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t178) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008766d6); };
            c.w32(0, t178);
            let Some(t179) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008766da); };
            c.cmp(t179, c.r32(0), 32);
            if c.cc_ne() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            let Some(t180) = c.ld8(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008766e3); };
            c.logic(t180 & 0x1u64, 8);
            if c.cc_e() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            c.w32(2, 0x1u64);
            c.w64(8, c.r64(2));
            blk = 230; continue;
        }
        229 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 230; continue;
        }
        230 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876707); };
            c.w64(15, t181);
            blk = 808; continue;
        }
        232 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 237; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t182) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000876716); };
            c.cmp(t182, c.r32(1), 32);
            if c.cc_ne() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087671d); };
            c.w64(0, t183);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000876721); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r184 = c.r64(8) & c.r64(0);
            c.w64(8, r184);
            c.logic(r184, 64);
            let Some(t185) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087672e); };
            c.logic(t185 & 0x4u64, 8);
            if c.cc_ne() { blk = 985; continue; }
            blk = 235; continue;
        }
        235 => {
            blk = 296; continue;
        }
        236 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087673e); };
            c.w64(15, t186);
            blk = 808; continue;
        }
        237 => {
            let r187 = c.r32(15) & 0x380u64;
            c.w32(15, r187);
            c.logic(r187, 32);
            c.cmp(c.r32(15), 0x80u64, 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(15), 0x100u64, 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 239; continue;
        }
        239 => {
            c.cmp(c.r32(15), 0x180u64, 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 240; continue;
        }
        240 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087677b); };
            c.w64(15, t188);
            blk = 808; continue;
        }
        242 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 250; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t189) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000876789); };
            c.w32(8, t189);
            let r190 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r190);
            c.w32(9, 0x4u64);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w32(9, 0x2u64);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 246; continue;
        }
        246 => {
            let r191 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r191);
            c.logic(r191, 64);
            let Some(t192) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008767b1); };
            c.cmp(t192, c.r32(8), 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008767b7); };
            c.w64(15, t193);
            blk = 808; continue;
        }
        248 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008767c0); };
            c.w64(15, t194);
            blk = 808; continue;
        }
        249 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008767c9); };
            c.w64(15, t195);
            blk = 808; continue;
        }
        250 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008767d2); };
            c.w64(15, t196);
            blk = 808; continue;
        }
        251 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t197) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008767e0); };
            c.w32(8, t197);
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008767e4); };
            c.cmp(t198, c.r32(8), 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008767ed); };
            c.w64(15, t199);
            blk = 808; continue;
        }
        254 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008767fb); };
            c.w64(15, t200);
            blk = 808; continue;
        }
        256 => {
            let r201 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r201);
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t202) = c.ld8(c.r64(7).wrapping_add(0x4)) else { return RegfileExit::fault(0x700087680b); };
            c.w32(8, t202);
            let Some(t203) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876810); };
            c.w32(9, t203);
            let r204 = c.r64(12) & 0x3ffu64;
            c.w64(12, r204);
            c.logic(r204, 64);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r205 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r205);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_b() { blk = 283; continue; }
            blk = 257; continue;
        }
        257 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 264; continue; }
            blk = 258; continue;
        }
        258 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 260; continue; }
            blk = 259; continue;
        }
        259 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876842); }
            blk = 261; continue;
        }
        260 => {
            c.w64(11, c.r64(1));
            let r206 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r206);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 261; continue;
        }
        261 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000876858); };
            c.w64(11, t207);
            let Some(t208) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700087685f); };
            c.w64(12, t208);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000876867); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700087686e); };
            c.cmp(t210, c.r64(15), 64);
            if c.cc_be() { blk = 812; continue; }
            blk = 262; continue;
        }
        262 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000876877); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 263; continue;
        }
        263 => {
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087687f); };
            c.w64(12, t211);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000876886); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087688a); }
            blk = 265; continue;
        }
        264 => {
            c.w64(11, c.r64(1));
            blk = 265; continue;
        }
        265 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876895); };
            c.w64(12, t212);
            let Some(t213) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876899); };
            c.w32(15, t213);
            let r214 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r214);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008768a1); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 267; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t215) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008768ab); };
            c.w32(0, t215);
            let r216 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r216);
            c.w64(3, c.r64(0));
            let r217 = c.r32(3) & 0x3ffu64;
            c.w32(3, r217);
            c.logic(r217, 32);
            let r218 = c.r32(0) & 0x3ffffc00u64;
            c.w32(0, r218);
            c.logic(r218, 32);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 268; continue;
        }
        267 => {
            let r219 = c.shl(c.r32(15), 0x9u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r219);
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008768d1); };
            c.w64(0, t220);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 268; continue;
        }
        268 => {
            let r221 = c.sub(c.r64(8), c.r64(9), 64);
            c.w64(8, r221);
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_b() { blk = 279; continue; }
            blk = 269; continue;
        }
        269 => {
            c.w64(9, c.r64(3).wrapping_add(0x3));
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008768ea); };
            c.w64(2, t222);
            let Some(t223) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008768f1); };
            c.w64(6, t223);
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.w64(7, c.r64(1).wrapping_add(c.r64(6)));
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000876900); };
            c.w64(11, t224);
            let Some(t225) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000876907); };
            c.cmp(t225, c.r64(7), 64);
            if c.cc_be() { blk = 814; continue; }
            blk = 270; continue;
        }
        270 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x7000876910); }
            c.w64(7, c.r64(6).wrapping_add(0x1));
            blk = 271; continue;
        }
        271 => {
            c.w32(11, c.r32(9));
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x700087691e); };
            c.w64(1, t226);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000876925); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x7000876928); }
            c.w64(3, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_e() { blk = 274; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000876940); };
            c.w64(2, t227);
            blk = 273; continue;
        }
        273 => {
            let r228 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r228);
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(2)) { return RegfileExit::fault(0x700087694b); }
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_ne() { blk = 273; continue; }
            blk = 274; continue;
        }
        274 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 276; continue; }
            blk = 275; continue;
        }
        275 => {
            let r229 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r229);
            let Some(t230) = c.ld32(c.r64(3).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700087695e); };
            c.w32(9, t230);
            if !c.st32(c.r64(3).wrapping_add(c.r64(7)), c.r32(9)) { return RegfileExit::fault(0x7000876962); }
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_ne() { blk = 275; continue; }
            blk = 276; continue;
        }
        276 => {
            let r231 = c.r32(11) | c.r32(15);
            c.w32(11, r231);
            c.logic(r231, 32);
            c.w32(9, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000876973); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000876977); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 277; continue;
        }
        277 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r232 = c.r64(9) & c.r64(12);
            c.w64(9, r232);
            c.logic(r232, 64);
            let Some(t233) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087698c); };
            c.logic(t233 & 0x4u64, 8);
            if c.cc_ne() { blk = 982; continue; }
            blk = 278; continue;
        }
        278 => {
            c.w64(0, c.r64(7));
            blk = 279; continue;
        }
        279 => {
            c.w64(7, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008769a2); };
            c.w64(8, t234);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(8)) { return RegfileExit::fault(0x70008769a6); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r235 = c.r64(9) & c.r64(0);
            c.w64(9, r235);
            c.logic(r235, 64);
            let Some(t236) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008769ba); };
            c.logic(t236 & 0x4u64, 8);
            if c.cc_ne() { blk = 979; continue; }
            blk = 281; continue;
        }
        281 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008769c5); };
            c.w64(7, t237);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008769c9); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r238 = c.r64(8) & c.r64(12);
            c.w64(8, r238);
            c.logic(r238, 64);
            let Some(t239) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008769d8); };
            c.logic(t239 & 0x4u64, 8);
            if c.cc_ne() { blk = 976; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008769e3); };
            c.w64(1, t240);
            blk = 296; continue;
        }
        283 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 288; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008769fa); };
            c.w64(11, t241);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008769fe); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r242 = c.r64(9) & c.r64(11);
            c.w64(9, r242);
            c.logic(r242, 64);
            let Some(t243) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000876a0c); };
            c.logic(t243 & 0x4u64, 8);
            if c.cc_ne() { blk = 973; continue; }
            blk = 285; continue;
        }
        285 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000876a17); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 287; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r244 = c.r64(7) & c.r64(11);
            c.w64(7, r244);
            c.logic(r244, 64);
            let Some(t245) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000876a2a); };
            c.logic(t245 & 0x4u64, 8);
            if c.cc_ne() { blk = 970; continue; }
            blk = 287; continue;
        }
        287 => {
            blk = 296; continue;
        }
        288 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000876a3e); }
            blk = 291; continue;
        }
        290 => {
            c.w64(9, c.r64(1));
            let r246 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r246);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 291; continue;
        }
        291 => {
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000876a54); };
            c.w64(9, t247);
            let Some(t248) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000876a5b); };
            c.w64(11, t248);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000876a62); };
            c.w64(15, t249);
            let Some(t250) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000876a69); };
            c.cmp(t250, c.r64(12), 64);
            if c.cc_be() { blk = 816; continue; }
            blk = 292; continue;
        }
        292 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000876a72); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 293; continue;
        }
        293 => {
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000876a79); };
            c.w64(11, t251);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000876a80); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000876a84); }
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876a8a); };
            c.w64(11, t252);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000876a8e); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r253 = c.r64(12) & c.r64(11);
            c.w64(12, r253);
            c.logic(r253, 64);
            let Some(t254) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000876a9c); };
            c.logic(t254 & 0x4u64, 8);
            if c.cc_ne() { blk = 967; continue; }
            blk = 294; continue;
        }
        294 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x7000876aa8); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r255 = c.r64(7) & c.r64(11);
            c.w64(7, r255);
            c.logic(r255, 64);
            let Some(t256) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000876abc); };
            c.logic(t256 & 0x4u64, 8);
            if c.cc_ne() { blk = 964; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        297 => {
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876ace); };
            c.w64(15, t257);
            blk = 808; continue;
        }
        298 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876ad7); };
            c.w64(15, t258);
            blk = 808; continue;
        }
        299 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876ae0); };
            c.w64(15, t259);
            blk = 808; continue;
        }
        300 => {
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876ae9); };
            c.w64(15, t260);
            blk = 808; continue;
        }
        301 => {
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876af2); };
            c.w64(15, t261);
            blk = 808; continue;
        }
        302 => {
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876afb); };
            c.w64(15, t262);
            blk = 808; continue;
        }
        303 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b04); };
            c.w64(15, t263);
            blk = 808; continue;
        }
        304 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b0d); };
            c.w64(15, t264);
            blk = 808; continue;
        }
        305 => {
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b16); };
            c.w64(15, t265);
            blk = 808; continue;
        }
        306 => {
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b1f); };
            c.w64(15, t266);
            blk = 808; continue;
        }
        307 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b28); };
            c.w64(15, t267);
            blk = 808; continue;
        }
        308 => {
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b31); };
            c.w64(15, t268);
            blk = 808; continue;
        }
        309 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b3a); };
            c.w64(15, t269);
            blk = 808; continue;
        }
        310 => {
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876b43); };
            c.w64(15, t270);
            blk = 808; continue;
        }
        311 => {
            let Some(t271) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876b4c); };
            c.w32(11, t271);
            let r272 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r272);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 313; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000876b59); };
            c.w64(11, t273);
            blk = 313; continue;
        }
        313 => {
            let Some(t274) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000876b60); };
            c.w32(12, t274);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r275 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r275);
            let Some(t276) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876b6e); };
            c.w32(15, t276);
            let r277 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r277);
            let r278 = c.r64(15) & c.r64(12);
            c.w64(15, r278);
            c.logic(r278, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000876b79); }
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 315; continue;
        }
        314 => {
            let r279 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r279);
            let r280 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r280);
            c.w64(0, c.r64(12));
            let r281 = c.r64(0) & c.r64(15);
            c.w64(0, r281);
            c.logic(r281, 64);
            c.w64(15, c.r64(0));
            blk = 315; continue;
        }
        315 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(0, c.r64(3).wrapping_mul(4));
            let Some(t282) = c.ld32(c.r64(11).wrapping_add(c.r64(0)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000876b9e); };
            c.w32(6, t282);
            let Some(t283) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000876ba3); };
            c.cmp(t283, c.r32(6), 32);
            if c.cc_e() { blk = 337; continue; }
            blk = 316; continue;
        }
        316 => {
            c.cmp(c.r32(6), c.r32(8), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 317; continue;
        }
        317 => {
            let Some(t284) = c.ld32(c.r64(11).wrapping_add(c.r64(0)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000876bb5); };
            c.w32(7, t284);
            c.logic(c.r8(7) & 0x2u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876bc0); };
            c.w64(15, t285);
            blk = 808; continue;
        }
        319 => {
            let Some(t286) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000876bc9); };
            c.cmp(t286, c.r32(8), 32);
            if c.cc_e() { blk = 336; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000876bd6); };
            c.cmp(t287, c.r32(8), 32);
            if c.cc_e() { blk = 335; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t288) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000876be3); };
            c.cmp(t288, c.r32(8), 32);
            if c.cc_e() { blk = 334; continue; }
            blk = 322; continue;
        }
        322 => {
            let Some(t289) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000876bf0); };
            c.cmp(t289, c.r32(8), 32);
            if c.cc_e() { blk = 333; continue; }
            blk = 323; continue;
        }
        323 => {
            let Some(t290) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000876bfd); };
            c.cmp(t290, c.r32(8), 32);
            if c.cc_e() { blk = 332; continue; }
            blk = 324; continue;
        }
        324 => {
            let Some(t291) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000876c06); };
            c.cmp(t291, c.r32(8), 32);
            if c.cc_e() { blk = 331; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t292) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000876c0f); };
            c.cmp(t292, c.r32(8), 32);
            if c.cc_e() { blk = 330; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t293) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876c18); };
            c.w32(7, t293);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 818; continue; }
            blk = 327; continue;
        }
        327 => {
            c.w64(7, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x7000876c37); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 329; continue; }
            blk = 328; continue;
        }
        328 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r294 = c.r64(8) & c.r64(11);
            c.w64(8, r294);
            c.logic(r294, 64);
            let Some(t295) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000876c4a); };
            c.logic(t295 & 0x4u64, 8);
            if c.cc_ne() { blk = 961; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        330 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876c5d); };
            c.w64(15, t296);
            blk = 808; continue;
        }
        331 => {
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876c66); };
            c.w64(15, t297);
            blk = 808; continue;
        }
        332 => {
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876c6f); };
            c.w64(15, t298);
            blk = 808; continue;
        }
        333 => {
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876c78); };
            c.w64(15, t299);
            blk = 808; continue;
        }
        334 => {
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876c81); };
            c.w64(15, t300);
            blk = 808; continue;
        }
        335 => {
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876c8a); };
            c.w64(15, t301);
            blk = 808; continue;
        }
        336 => {
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876c93); };
            c.w64(15, t302);
            blk = 808; continue;
        }
        337 => {
            let Some(t303) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000876c9c); };
            c.cmp(t303, 0x82eu64, 16);
            if c.cc_e() { blk = 386; continue; }
            blk = 338; continue;
        }
        338 => {
            let Some(t304) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000876ca8); };
            c.cmp(t304, c.r32(8), 32);
            if c.cc_e() { blk = 385; continue; }
            blk = 339; continue;
        }
        339 => {
            let Some(t305) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000876cb5); };
            c.cmp(t305, c.r32(8), 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t306) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000876cc2); };
            c.cmp(t306, c.r32(8), 32);
            if c.cc_e() { blk = 383; continue; }
            blk = 341; continue;
        }
        341 => {
            let Some(t307) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000876ccf); };
            c.cmp(t307, c.r32(8), 32);
            if c.cc_e() { blk = 382; continue; }
            blk = 342; continue;
        }
        342 => {
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000876cdc); };
            c.cmp(t308, c.r32(8), 32);
            if c.cc_e() { blk = 381; continue; }
            blk = 343; continue;
        }
        343 => {
            let Some(t309) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000876ce9); };
            c.cmp(t309, c.r32(8), 32);
            if c.cc_e() { blk = 380; continue; }
            blk = 344; continue;
        }
        344 => {
            let Some(t310) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000876cf6); };
            c.cmp(t310, c.r32(8), 32);
            if c.cc_e() { blk = 379; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t311) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876d03); };
            c.w32(12, t311);
            let Some(t312) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000876d07); };
            c.cmp(t312, c.r32(12), 32);
            if c.cc_ne() { blk = 349; continue; }
            blk = 346; continue;
        }
        346 => {
            let Some(t313) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000876d10); };
            c.w32(12, t313);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 347; continue;
        }
        347 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_e() { blk = 351; continue; }
            blk = 348; continue;
        }
        348 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876d20); };
            c.w64(15, t314);
            blk = 808; continue;
        }
        349 => {
            c.logic(c.r32(9) & 0x8000000u64, 32);
            if c.cc_ne() { blk = 351; continue; }
            blk = 350; continue;
        }
        350 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876d32); };
            c.w64(15, t315);
            blk = 808; continue;
        }
        351 => {
            c.logic(c.r32(9) & 0x100000u64, 32);
            if c.cc_e() { blk = 355; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t316) = c.ld32(c.r64(7).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000876d44); };
            c.w32(9, t316);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 355; continue; }
            blk = 353; continue;
        }
        353 => {
            c.w64(10, c.r64(4));
            let r317 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r317);
            let r318 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r318);
            c.logic(r318, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000876d59); }
            c.w64(1, c.r64(7));
            let Some(t319) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x7000876d61); };
            c.w64(0, t319);
            c.w64(10, 0x7000876d6fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000876d6f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000876d73); }
            return c.call(c.r64(0), 0x7000876d79, 0x7000876d77);
        }
        354 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000876d79); }
            let Some(t320) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000876d81); };
            c.w64(4, t320);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000876d86); };
            c.w64(1, t321);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000876d8a); };
            c.w64(8, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000876d8e); };
            c.w64(11, t323);
            blk = 355; continue;
        }
        355 => {
            let Some(t324) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876d92); };
            c.w32(7, t324);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t325) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000876d99); };
            c.w32(9, t325);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 819; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t326) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000876daa); };
            c.w32(12, t326);
            let r327 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r327);
            c.w64(15, c.r64(12));
            let r328 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r328);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r329 = c.r64(15) & 0xfffffffffffffffeu64;
            c.w64(15, r329);
            c.logic(r329, 64);
            let r330 = c.add(c.r32(15), c.r32(12), 32);
            c.w32(15, r330);
            c.w32(0, c.r32(9));
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_b() { blk = 378; continue; }
            blk = 357; continue;
        }
        357 => {
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 820; continue; }
            blk = 358; continue;
        }
        358 => {
            let r331 = c.sub(c.r32(0), c.r32(12), 32);
            c.w32(0, r331);
            let r332 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r332);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t333) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000876de0); };
            c.w32(15, t333);
            let r334 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r334);
            c.logic(r334, 64);
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_b() { blk = 378; continue; }
            blk = 359; continue;
        }
        359 => {
            c.cmp(c.r64(7), 0x3u64, 64);
            if c.cc_be() { blk = 821; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t335) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000876dfb); };
            c.w32(7, t335);
            let r336 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r336);
            c.w32(9, c.r64(7).wrapping_add(0x2));
            c.cmp(c.r32(9), 0xfffffeu64, 32);
            if c.cc_a() { blk = 378; continue; }
            blk = 361; continue;
        }
        361 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x7000876e16); }
            let Some(t337) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876e1a); };
            c.w32(9, t337);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 822; continue; }
            blk = 362; continue;
        }
        362 => {
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000876e31); }
            let Some(t338) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000876e35); };
            c.w32(9, t338);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r339 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r339);
            let Some(t340) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876e43); };
            c.w32(12, t340);
            let r341 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r341);
            let r342 = c.r64(12) & c.r64(9);
            c.w64(12, r342);
            c.logic(r342, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 364; continue;
        }
        363 => {
            let r343 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r343);
            let r344 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r344);
            c.w64(0, c.r64(9));
            let r345 = c.r64(0) & c.r64(12);
            c.w64(0, r345);
            c.logic(r345, 64);
            c.w64(12, c.r64(0));
            blk = 364; continue;
        }
        364 => {
            c.w64(0, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(0).wrapping_mul(4));
            c.w64(2, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            let Some(t346) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000876e78); };
            c.w32(6, t346);
            let Some(t347) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000876e7d); };
            c.cmp(t347, c.r32(6), 32);
            if c.cc_e() { blk = 366; continue; }
            blk = 365; continue;
        }
        365 => {
            let Some(t348) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000876e86); };
            c.cmp(t348, c.r32(6), 32);
            if c.cc_ne() { blk = 363; continue; }
            blk = 366; continue;
        }
        366 => {
            let Some(t349) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876e8f); };
            c.w32(9, t349);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_ae() { blk = 823; continue; }
            blk = 367; continue;
        }
        367 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(2)), c.r32(8)) { return RegfileExit::fault(0x7000876ea2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 369; continue; }
            blk = 368; continue;
        }
        368 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r350 = c.r64(9) & c.r64(11);
            c.w64(9, r350);
            c.logic(r350, 64);
            let Some(t351) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000876eb6); };
            c.logic(t351 & 0x4u64, 8);
            if c.cc_ne() { blk = 958; continue; }
            blk = 369; continue;
        }
        369 => {
            let Some(t352) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876ec1); };
            c.w32(9, t352);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(0).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ae() { blk = 824; continue; }
            blk = 370; continue;
        }
        370 => {
            if let Some(sr) = c.shl(c.r64(7), 0x8u64, 64) { c.w64(7, sr); }
            c.w32(9, c.r32(7));
            c.w64(12, c.r64(0).wrapping_mul(4).wrapping_add(0xb));
            c.w64(15, c.r64(1));
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x7000876eea); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 372; continue; }
            blk = 371; continue;
        }
        371 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r353 = c.r64(2) & c.r64(11);
            c.w64(2, r353);
            c.logic(r353, 64);
            let Some(t354) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000876efe); };
            c.logic(t354 & 0x4u64, 8);
            if c.cc_ne() { blk = 955; continue; }
            blk = 372; continue;
        }
        372 => {
            let Some(t355) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000876f08); };
            c.w32(12, t355);
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000876f0c); };
            c.cmp(t356, c.r32(12), 32);
            if c.cc_ne() { blk = 375; continue; }
            blk = 373; continue;
        }
        373 => {
            let Some(t357) = c.ld8(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000876f15); };
            c.logic(t357 & 0x1u64, 8);
            if c.cc_e() { blk = 375; continue; }
            blk = 374; continue;
        }
        374 => {
            let r358 = c.r32(9) | 0x20u64;
            c.w32(9, r358);
            c.logic(r358, 32);
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 376; continue;
        }
        375 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 376; continue;
        }
        376 => {
            let Some(t359) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000876f28); };
            c.w32(8, t359);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(0).wrapping_add(0x2));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 825; continue; }
            blk = 377; continue;
        }
        377 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000876f3f); }
            c.w64(0, c.r64(15));
            blk = 174; continue;
        }
        378 => {
            let Some(t360) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000876f4c); };
            c.w64(5, t360);
            let Some(t361) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x7000876f50); };
            c.w64(3, t361);
            c.w32(0, 0x3u64);
            let Some(t362) = c.ld64(c.r64(4).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000876f5c); };
            c.w64(10, t362);
            let Some(t363) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x7000876f61); };
            if !c.push64(t363) { return RegfileExit::fault(0x7000876f61); }
            let sp364 = c.r64(4);
            let Some(pv365) = c.ld64(sp364) else { return RegfileExit::fault(0x7000876f65); };
            c.set_rsp(sp364.wrapping_add(8));
            let pea366 = c.r64(4).wrapping_add(0x60);
            if !c.st64(pea366, pv365) { c.set_rsp(sp364); return RegfileExit::fault(0x7000876f65); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x7000876f69); }
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(8)) { return RegfileExit::fault(0x7000876f6e); }
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(1)) { return RegfileExit::fault(0x7000876f73); }
            let Some(t367) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000876f78); };
            c.w64(6, t367);
            let r368 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r368);
            return RegfileExit::jump(0x700086ed74);
        }
        379 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876f86); };
            c.w64(15, t369);
            blk = 808; continue;
        }
        380 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876f8f); };
            c.w64(15, t370);
            blk = 808; continue;
        }
        381 => {
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876f98); };
            c.w64(15, t371);
            blk = 808; continue;
        }
        382 => {
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876fa1); };
            c.w64(15, t372);
            blk = 808; continue;
        }
        383 => {
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876faa); };
            c.w64(15, t373);
            blk = 808; continue;
        }
        384 => {
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876fb3); };
            c.w64(15, t374);
            blk = 808; continue;
        }
        385 => {
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876fbc); };
            c.w64(15, t375);
            blk = 808; continue;
        }
        386 => {
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000876fc5); };
            c.w64(15, t376);
            blk = 808; continue;
        }
        387 => {
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 388; continue;
        }
        388 => {
            let Some(t377) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000876fd7); };
            c.w32(9, t377);
            c.w64(12, c.r64(9));
            let r378 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r378);
            c.w64(15, c.r64(0));
            let Some(t379) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000876fe6); };
            c.w32(0, t379);
            let r380 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r380);
            let r381 = c.r32(12) & 0x3fu64;
            c.w32(12, r381);
            c.logic(r381, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000876ff1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000876ff5); }
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_be() { blk = 391; continue; }
            blk = 389; continue;
        }
        389 => {
            c.cmp(c.r32(12), 0x11u64, 32);
            if c.cc_ge() { blk = 808; continue; }
            blk = 390; continue;
        }
        390 => {
            let r382 = c.r32(9) & 0xfcu64;
            c.w32(9, r382);
            c.logic(r382, 32);
            c.cmp(c.r32(9), 0x30u64, 32);
            blk = 808; continue;
        }
        391 => {
            c.cmp(c.r16(8), 0x834u64, 16);
            if c.cc_e() { blk = 394; continue; }
            blk = 392; continue;
        }
        392 => {
            let Some(t383) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000877021); };
            c.w32(8, t383);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 694; continue; }
            blk = 393; continue;
        }
        393 => {
            blk = 808; continue;
        }
        394 => {
            let Some(t384) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000877039); };
            c.w32(8, t384);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 694; continue; }
            blk = 395; continue;
        }
        395 => {
            let Some(t385) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087704c); };
            c.w32(2, t385);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(2));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_be() { blk = 808; continue; }
            blk = 396; continue;
        }
        396 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_e() { blk = 574; continue; }
            blk = 397; continue;
        }
        397 => {
            let Some(t386) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087706a); };
            c.logic(t386 & 0x200000u64, 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t387) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000877077); };
            c.w32(8, t387);
            let r388 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r388);
            let Some(t389) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087707e); };
            c.w32(8, t389);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 808; continue; }
            blk = 399; continue;
        }
        399 => {
            let Some(t390) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087708c); };
            c.w32(8, t390);
            let Some(t391) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000877090); };
            c.cmp(t391, c.r32(8), 32);
            if c.cc_ne() { blk = 510; continue; }
            blk = 400; continue;
        }
        400 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 480; continue; }
            blk = 401; continue;
        }
        401 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_le() { blk = 419; continue; }
            blk = 402; continue;
        }
        402 => {
            let r392 = c.r32(9) & 0xfcu64;
            c.w32(9, r392);
            c.logic(r392, 32);
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 416; continue; }
            blk = 403; continue;
        }
        403 => {
            let Some(t393) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008770c1); };
            c.w32(8, t393);
            let r394 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r394);
            let Some(t395) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008770c8); };
            c.w32(8, t395);
            let r396 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r396);
            let Some(t397) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008770cf); };
            c.w32(9, t397);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 405; continue; }
            blk = 404; continue;
        }
        404 => {
            return RegfileExit::fault(0x70008770df);
        }
        405 => {
            let Some(t398) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008770e0); };
            c.cmp(t398, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 406; continue;
        }
        406 => {
            c.w64(7, c.r64(15));
            let r399 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r399);
            c.logic(r399, 64);
            let Some(t400) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008770f7); };
            c.w64(12, t400);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 413; continue; }
            blk = 407; continue;
        }
        407 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 413; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t401) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087710b); };
            c.w64(3, t401);
            let Some(t402) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000877112); };
            c.w64(3, t402);
            c.w64(2, c.r64(12));
            let r403 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r403);
            c.logic(r403, 64);
            c.w64(6, c.r64(3));
            let r404 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r404);
            c.logic(r404, 64);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 411; continue; }
            blk = 409; continue;
        }
        409 => {
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_e() { blk = 412; continue; }
            blk = 410; continue;
        }
        410 => {
            blk = 413; continue;
        }
        411 => {
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_be() { blk = 413; continue; }
            blk = 412; continue;
        }
        412 => {
            let Some(t405) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087713a); };
            c.w32(7, t405);
            let Some(t406) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700087713e); };
            c.cmp(t406, c.r32(7), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 413; continue;
        }
        413 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 415; continue; }
            blk = 414; continue;
        }
        414 => {
            return RegfileExit::fault(0x7000877151);
        }
        415 => {
            let Some(t407) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000877152); };
            c.w32(7, t407);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000877159); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r408 = c.r64(8) & c.r64(15);
            c.w64(8, r408);
            c.logic(r408, 64);
            let Some(t409) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877167); };
            c.logic(t409 & 0x4u64, 8);
            if c.cc_ne() { blk = 952; continue; }
            blk = 416; continue;
        }
        416 => {
            c.w64(7, c.r64(11).wrapping_add(0x1));
            let r410 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r410);
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000877180); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 418; continue; }
            blk = 417; continue;
        }
        417 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r411 = c.r64(9) & c.r64(0);
            c.w64(9, r411);
            c.logic(r411, 64);
            let Some(t412) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877193); };
            c.logic(t412 & 0x4u64, 8);
            if c.cc_ne() { blk = 949; continue; }
            blk = 418; continue;
        }
        418 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700087719e); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        419 => {
            let Some(t413) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008771aa); };
            c.w32(8, t413);
            let r414 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r414);
            let Some(t415) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008771b1); };
            c.w32(9, t415);
            let Some(t416) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008771b5); };
            c.w32(8, t416);
            let r417 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r417);
            let Some(t418) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008771bc); };
            c.cmp(t418, c.r32(9), 32);
            if c.cc_ne() { blk = 457; continue; }
            blk = 420; continue;
        }
        420 => {
            c.w64(9, c.r64(15));
            let r419 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r419);
            c.logic(r419, 64);
            let Some(t420) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008771d3); };
            c.w64(12, t420);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 428; continue; }
            blk = 421; continue;
        }
        421 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 428; continue; }
            blk = 422; continue;
        }
        422 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t421) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008771e7); };
            c.w64(6, t421);
            let Some(t422) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008771ee); };
            c.w64(6, t422);
            c.w64(1, c.r64(12));
            let r423 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r423);
            c.logic(r423, 64);
            c.w64(11, c.r64(6));
            let r424 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r424);
            c.logic(r424, 64);
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_e() { blk = 425; continue; }
            blk = 423; continue;
        }
        423 => {
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_e() { blk = 426; continue; }
            blk = 424; continue;
        }
        424 => {
            blk = 428; continue;
        }
        425 => {
            c.cmp(c.r64(6), c.r64(12), 64);
            if c.cc_be() { blk = 428; continue; }
            blk = 426; continue;
        }
        426 => {
            let Some(t425) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000877216); };
            c.w32(9, t425);
            let Some(t426) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700087721a); };
            c.cmp(t426, c.r32(9), 32);
            if c.cc_ne() { blk = 428; continue; }
            blk = 427; continue;
        }
        427 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877223); };
            c.w64(1, t427);
            blk = 808; continue;
        }
        428 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087722c); };
            c.w32(9, t428);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x69u64, 64);
            if c.cc_a() { blk = 430; continue; }
            blk = 429; continue;
        }
        429 => {
            return RegfileExit::fault(0x700087723c);
        }
        430 => {
            let Some(t429) = c.ld32(c.r64(8).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x700087723d); };
            c.cmp(t429, c.r32(7), 32);
            if c.cc_ne() { blk = 434; continue; }
            blk = 431; continue;
        }
        431 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 433; continue; }
            blk = 432; continue;
        }
        432 => {
            return RegfileExit::fault(0x700087724c);
        }
        433 => {
            let Some(t430) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x700087724d); };
            c.w32(7, t430);
            blk = 440; continue;
        }
        434 => {
            c.cmp(c.r64(9), 0x6au64, 64);
            if c.cc_a() { blk = 436; continue; }
            blk = 435; continue;
        }
        435 => {
            return RegfileExit::fault(0x700087725c);
        }
        436 => {
            let Some(t431) = c.ld32(c.r64(8).wrapping_add(0x1af)) else { return RegfileExit::fault(0x700087725d); };
            c.cmp(t431, c.r32(7), 32);
            if c.cc_ne() { blk = 456; continue; }
            blk = 437; continue;
        }
        437 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 439; continue; }
            blk = 438; continue;
        }
        438 => {
            return RegfileExit::fault(0x7000877270);
        }
        439 => {
            let Some(t432) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000877271); };
            c.w32(7, t432);
            blk = 440; continue;
        }
        440 => {
            c.cmp(c.r64(3), 0x3fffu64, 64);
            if c.cc_ae() { blk = 455; continue; }
            blk = 441; continue;
        }
        441 => {
            c.cmp(c.r64(3), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 830; continue; }
            blk = 442; continue;
        }
        442 => {
            c.w64(8, c.r64(2).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 826; continue; }
            blk = 443; continue;
        }
        443 => {
            c.w32(9, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008772ab); };
            c.w64(11, t433);
            let Some(t434) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008772b2); };
            c.w64(12, t434);
            c.w64(3, c.r64(8).wrapping_add(c.r64(12)));
            c.w64(1, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            let Some(t435) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008772c1); };
            c.cmp(t435, c.r64(3), 64);
            if c.cc_be() { blk = 828; continue; }
            blk = 444; continue;
        }
        444 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008772cb); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 445; continue;
        }
        445 => {
            let Some(t436) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008772d3); };
            c.w64(11, t436);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008772da); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008772de); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 451; continue; }
            blk = 446; continue;
        }
        446 => {
            c.w64(9, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            blk = 447; continue;
        }
        447 => {
            let r437 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r437);
            let Some(t438) = c.ld32(c.r64(1).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008772f4); };
            c.w32(11, t438);
            let r439 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r439);
            let Some(t440) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008772fc); };
            c.cmp(t440, c.r32(11), 32);
            if c.cc_e() { blk = 449; continue; }
            blk = 448; continue;
        }
        448 => {
            let r441 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r441);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(8)), 0) { return RegfileExit::fault(0x7000877311); }
            blk = 450; continue;
        }
        449 => {
            c.w64(11, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8)), c.r64(11)) { return RegfileExit::fault(0x7000877323); }
            blk = 450; continue;
        }
        450 => {
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 447; continue; }
            blk = 451; continue;
        }
        451 => {
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087732d); };
            c.w64(11, t442);
            c.w64(9, c.r64(11).wrapping_add(0x1));
            let r443 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r443);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000877338); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 453; continue; }
            blk = 452; continue;
        }
        452 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r444 = c.r64(12) & c.r64(15);
            c.w64(12, r444);
            c.logic(r444, 64);
            let Some(t445) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087734c); };
            c.logic(t445 & 0x4u64, 8);
            if c.cc_ne() { blk = 946; continue; }
            blk = 453; continue;
        }
        453 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000877358); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r446 = c.r64(8) & c.r64(15);
            c.w64(8, r446);
            c.logic(r446, 64);
            let Some(t447) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877366); };
            c.logic(t447 & 0x4u64, 8);
            if c.cc_ne() { blk = 943; continue; }
            blk = 454; continue;
        }
        454 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877371); };
            c.w64(1, t448);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000877375); }
            let Some(t449) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087737a); };
            c.w32(7, t449);
            let r450 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r450);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700087738b); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000877392); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        455 => {
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087739e); };
            c.w64(1, t451);
            blk = 808; continue;
        }
        456 => {
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008773a7); };
            c.w64(1, t452);
            blk = 808; continue;
        }
        457 => {
            c.w64(9, c.r64(15));
            let r453 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r453);
            c.logic(r453, 64);
            let Some(t454) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008773ba); };
            c.w64(12, t454);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 464; continue; }
            blk = 458; continue;
        }
        458 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 464; continue; }
            blk = 459; continue;
        }
        459 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t455) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008773ce); };
            c.w64(3, t455);
            let Some(t456) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008773d5); };
            c.w64(3, t456);
            c.w64(2, c.r64(12));
            let r457 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r457);
            c.logic(r457, 64);
            c.w64(6, c.r64(3));
            let r458 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r458);
            c.logic(r458, 64);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 462; continue; }
            blk = 460; continue;
        }
        460 => {
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_e() { blk = 463; continue; }
            blk = 461; continue;
        }
        461 => {
            blk = 464; continue;
        }
        462 => {
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_be() { blk = 464; continue; }
            blk = 463; continue;
        }
        463 => {
            let Some(t459) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008773fd); };
            c.w32(9, t459);
            let Some(t460) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000877401); };
            c.cmp(t460, c.r32(9), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 464; continue;
        }
        464 => {
            let Some(t461) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087740e); };
            c.w32(9, t461);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x69u64, 64);
            if c.cc_a() { blk = 466; continue; }
            blk = 465; continue;
        }
        465 => {
            return RegfileExit::fault(0x700087741e);
        }
        466 => {
            let Some(t462) = c.ld32(c.r64(8).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x700087741f); };
            c.cmp(t462, c.r32(7), 32);
            if c.cc_ne() { blk = 470; continue; }
            blk = 467; continue;
        }
        467 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 469; continue; }
            blk = 468; continue;
        }
        468 => {
            return RegfileExit::fault(0x700087742e);
        }
        469 => {
            let Some(t463) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700087742f); };
            c.w32(7, t463);
            blk = 476; continue;
        }
        470 => {
            c.cmp(c.r64(9), 0x6au64, 64);
            if c.cc_a() { blk = 472; continue; }
            blk = 471; continue;
        }
        471 => {
            return RegfileExit::fault(0x700087743e);
        }
        472 => {
            let Some(t464) = c.ld32(c.r64(8).wrapping_add(0x1af)) else { return RegfileExit::fault(0x700087743f); };
            c.cmp(t464, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 473; continue;
        }
        473 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 475; continue; }
            blk = 474; continue;
        }
        474 => {
            return RegfileExit::fault(0x7000877452);
        }
        475 => {
            let Some(t465) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000877453); };
            c.w32(7, t465);
            blk = 476; continue;
        }
        476 => {
            c.w64(8, c.r64(11).wrapping_add(0x1));
            let r466 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r466);
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000877469); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r467 = c.r64(11) & c.r64(15);
            c.w64(11, r467);
            c.logic(r467, 64);
            let Some(t468) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877477); };
            c.logic(t468 & 0x4u64, 8);
            if c.cc_ne() { blk = 940; continue; }
            blk = 477; continue;
        }
        477 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x7000877482); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 479; continue; }
            blk = 478; continue;
        }
        478 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r469 = c.r64(7) & c.r64(0);
            c.w64(7, r469);
            c.logic(r469, 64);
            let Some(t470) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877495); };
            c.logic(t470 & 0x4u64, 8);
            if c.cc_ne() { blk = 937; continue; }
            blk = 479; continue;
        }
        479 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700087749f); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        480 => {
            let r471 = c.r32(9) & 0xfcu64;
            c.w32(9, r471);
            c.logic(r471, 32);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 509; continue; }
            blk = 481; continue;
        }
        481 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 509; continue; }
            blk = 482; continue;
        }
        482 => {
            let Some(t472) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008774c6); };
            c.w32(8, t472);
            let r473 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r473);
            let Some(t474) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008774cd); };
            c.w32(8, t474);
            let r475 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r475);
            let Some(t476) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008774d4); };
            c.w32(9, t476);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x69u64, 64);
            if c.cc_a() { blk = 484; continue; }
            blk = 483; continue;
        }
        483 => {
            return RegfileExit::fault(0x70008774e4);
        }
        484 => {
            let Some(t477) = c.ld32(c.r64(8).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008774e5); };
            c.cmp(t477, c.r32(7), 32);
            if c.cc_ne() { blk = 496; continue; }
            blk = 485; continue;
        }
        485 => {
            c.w64(7, c.r64(15));
            let r478 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r478);
            c.logic(r478, 64);
            let Some(t479) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008774fc); };
            c.w64(12, t479);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 492; continue; }
            blk = 486; continue;
        }
        486 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 492; continue; }
            blk = 487; continue;
        }
        487 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t480) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877510); };
            c.w64(3, t480);
            let Some(t481) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000877517); };
            c.w64(3, t481);
            c.w64(2, c.r64(12));
            let r482 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r482);
            c.logic(r482, 64);
            c.w64(6, c.r64(3));
            let r483 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r483);
            c.logic(r483, 64);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 490; continue; }
            blk = 488; continue;
        }
        488 => {
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_e() { blk = 491; continue; }
            blk = 489; continue;
        }
        489 => {
            blk = 492; continue;
        }
        490 => {
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_be() { blk = 492; continue; }
            blk = 491; continue;
        }
        491 => {
            let Some(t484) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087753f); };
            c.w32(7, t484);
            let Some(t485) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000877543); };
            c.cmp(t485, c.r32(7), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 492; continue;
        }
        492 => {
            c.cmp(c.r64(9), 0x6au64, 64);
            if c.cc_a() { blk = 494; continue; }
            blk = 493; continue;
        }
        493 => {
            return RegfileExit::fault(0x7000877556);
        }
        494 => {
            let Some(t486) = c.ld32(c.r64(8).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000877557); };
            c.w32(7, t486);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087755e); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r487 = c.r64(8) & c.r64(15);
            c.w64(8, r487);
            c.logic(r487, 64);
            let Some(t488) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087756c); };
            c.logic(t488 & 0x4u64, 8);
            if c.cc_ne() { blk = 934; continue; }
            blk = 495; continue;
        }
        495 => {
            blk = 509; continue;
        }
        496 => {
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 498; continue; }
            blk = 497; continue;
        }
        497 => {
            return RegfileExit::fault(0x7000877582);
        }
        498 => {
            let Some(t489) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000877583); };
            c.cmp(t489, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 499; continue;
        }
        499 => {
            c.w64(7, c.r64(15));
            let r490 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r490);
            c.logic(r490, 64);
            let Some(t491) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087759a); };
            c.w64(12, t491);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 506; continue; }
            blk = 500; continue;
        }
        500 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 506; continue; }
            blk = 501; continue;
        }
        501 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t492) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008775ae); };
            c.w64(3, t492);
            let Some(t493) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008775b5); };
            c.w64(3, t493);
            c.w64(2, c.r64(12));
            let r494 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r494);
            c.logic(r494, 64);
            c.w64(6, c.r64(3));
            let r495 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r495);
            c.logic(r495, 64);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 504; continue; }
            blk = 502; continue;
        }
        502 => {
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_e() { blk = 505; continue; }
            blk = 503; continue;
        }
        503 => {
            blk = 506; continue;
        }
        504 => {
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_be() { blk = 506; continue; }
            blk = 505; continue;
        }
        505 => {
            let Some(t496) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008775dd); };
            c.w32(7, t496);
            let Some(t497) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008775e1); };
            c.cmp(t497, c.r32(7), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 506; continue;
        }
        506 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 508; continue; }
            blk = 507; continue;
        }
        507 => {
            return RegfileExit::fault(0x70008775f4);
        }
        508 => {
            let Some(t498) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x70008775f5); };
            c.w32(7, t498);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008775fc); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r499 = c.r64(8) & c.r64(15);
            c.w64(8, r499);
            c.logic(r499, 64);
            let Some(t500) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087760a); };
            c.logic(t500 & 0x4u64, 8);
            if c.cc_ne() { blk = 931; continue; }
            blk = 509; continue;
        }
        509 => {
            c.w64(7, c.r64(11).wrapping_add(0x1));
            let r501 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r501);
            if !c.st32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700087761b); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000877620); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        510 => {
            let Some(t502) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700087762c); };
            c.cmp(t502, c.r32(8), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 511; continue;
        }
        511 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 558; continue; }
            blk = 512; continue;
        }
        512 => {
            let Some(t503) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000877642); };
            c.w32(8, t503);
            let Some(t504) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000877646); };
            c.cmp(t504, c.r32(8), 32);
            if c.cc_ne() { blk = 514; continue; }
            blk = 513; continue;
        }
        513 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700087764f); }
            blk = 559; continue;
        }
        514 => {
            let Some(t505) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000877659); };
            c.w32(8, t505);
            let r506 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r506);
            c.w64(9, c.r64(15));
            let r507 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r507);
            c.logic(r507, 64);
            let Some(t508) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087766a); };
            c.w32(8, t508);
            let r509 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r509);
            let Some(t510) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877671); };
            c.w64(12, t510);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 522; continue; }
            blk = 515; continue;
        }
        515 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 522; continue; }
            blk = 516; continue;
        }
        516 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t511) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877685); };
            c.w64(6, t511);
            let Some(t512) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700087768c); };
            c.w64(6, t512);
            c.w64(1, c.r64(12));
            let r513 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r513);
            c.logic(r513, 64);
            c.w64(11, c.r64(6));
            let r514 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r514);
            c.logic(r514, 64);
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_e() { blk = 519; continue; }
            blk = 517; continue;
        }
        517 => {
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_e() { blk = 520; continue; }
            blk = 518; continue;
        }
        518 => {
            blk = 522; continue;
        }
        519 => {
            c.cmp(c.r64(6), c.r64(12), 64);
            if c.cc_be() { blk = 522; continue; }
            blk = 520; continue;
        }
        520 => {
            let Some(t515) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008776b4); };
            c.w32(9, t515);
            let Some(t516) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008776b8); };
            c.cmp(t516, c.r32(9), 32);
            if c.cc_ne() { blk = 522; continue; }
            blk = 521; continue;
        }
        521 => {
            let Some(t517) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008776c1); };
            c.w64(1, t517);
            blk = 808; continue;
        }
        522 => {
            let Some(t518) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008776ca); };
            c.w32(9, t518);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6du64, 64);
            if c.cc_a() { blk = 524; continue; }
            blk = 523; continue;
        }
        523 => {
            return RegfileExit::fault(0x70008776da);
        }
        524 => {
            let Some(t519) = c.ld32(c.r64(8).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008776db); };
            c.cmp(t519, c.r32(7), 32);
            if c.cc_ne() { blk = 528; continue; }
            blk = 525; continue;
        }
        525 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 527; continue; }
            blk = 526; continue;
        }
        526 => {
            return RegfileExit::fault(0x70008776ea);
        }
        527 => {
            let Some(t520) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x70008776eb); };
            c.w32(7, t520);
            blk = 534; continue;
        }
        528 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 530; continue; }
            blk = 529; continue;
        }
        529 => {
            return RegfileExit::fault(0x70008776fa);
        }
        530 => {
            let Some(t521) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008776fb); };
            c.cmp(t521, c.r32(7), 32);
            if c.cc_ne() { blk = 557; continue; }
            blk = 531; continue;
        }
        531 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 533; continue; }
            blk = 532; continue;
        }
        532 => {
            return RegfileExit::fault(0x700087770e);
        }
        533 => {
            let Some(t522) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700087770f); };
            c.w32(7, t522);
            blk = 534; continue;
        }
        534 => {
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 556; continue; }
            blk = 535; continue;
        }
        535 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 836; continue; }
            blk = 536; continue;
        }
        536 => {
            c.w64(8, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            let Some(t523) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877738); };
            c.w64(9, t523);
            let Some(t524) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700087773f); };
            c.w64(11, t524);
            c.w64(12, c.r64(8).wrapping_add(c.r64(11)));
            let Some(t525) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000877746); };
            c.w64(1, t525);
            let Some(t526) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700087774d); };
            c.cmp(t526, c.r64(12), 64);
            if c.cc_be() { blk = 832; continue; }
            blk = 537; continue;
        }
        537 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000877756); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 538; continue;
        }
        538 => {
            c.w32(9, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000877761); };
            c.w64(11, t527);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000877768); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700087776c); }
            c.w64(9, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 541; continue; }
            blk = 539; continue;
        }
        539 => {
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087777e); };
            c.w64(11, t528);
            blk = 540; continue;
        }
        540 => {
            let r529 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r529);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000877789); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 540; continue; }
            blk = 541; continue;
        }
        541 => {
            c.w64(9, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 550; continue; }
            blk = 542; continue;
        }
        542 => {
            c.w64(11, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            let Some(t530) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x70008777ad); };
            c.w64(12, t530);
            blk = 544; continue;
        }
        543 => {
            let r531 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r531);
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008777ba); };
            c.w64(0, t532);
            blk = 544; continue;
        }
        544 => {
            let r533 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r533);
            let Some(t534) = c.ld64(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008777c2); };
            c.cmp(t534, c.r64(12), 64);
            if c.cc_e() { blk = 549; continue; }
            blk = 545; continue;
        }
        545 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(0))) { return RegfileExit::fault(0x70008777c8); }
            let Some(t535) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008777ce); };
            c.w64(3, t535);
            let Some(t536) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008777d5); };
            c.w64(2, t536);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            c.w64(6, c.r64(11).wrapping_sub(0x4));
            let Some(t537) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008777e0); };
            c.w64(0, t537);
            let Some(t538) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008777e7); };
            c.cmp(t538, c.r64(1), 64);
            if c.cc_be() { blk = 834; continue; }
            blk = 546; continue;
        }
        546 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008777f0); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 547; continue;
        }
        547 => {
            let Some(t539) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008777f7); };
            c.w64(3, t539);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008777fe); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000877801); }
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x7000877806); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 549; continue; }
            blk = 548; continue;
        }
        548 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r540 = c.r64(3) & c.r64(8);
            c.w64(3, r540);
            c.logic(r540, 64);
            let Some(t541) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877818); };
            c.logic(t541 & 0x4u64, 8);
            if c.cc_ne() { blk = 928; continue; }
            blk = 549; continue;
        }
        549 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 543; continue; }
            blk = 550; continue;
        }
        550 => {
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000877828); };
            c.w64(11, t542);
            c.w64(9, c.r64(11).wrapping_add(0x1));
            let r543 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r543);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000877833); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 552; continue; }
            blk = 551; continue;
        }
        551 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r544 = c.r64(12) & c.r64(15);
            c.w64(12, r544);
            c.logic(r544, 64);
            let Some(t545) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877847); };
            c.logic(t545 & 0x4u64, 8);
            if c.cc_ne() { blk = 925; continue; }
            blk = 552; continue;
        }
        552 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000877853); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r546 = c.r64(8) & c.r64(15);
            c.w64(8, r546);
            c.logic(r546, 64);
            let Some(t547) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877861); };
            c.logic(t547 & 0x4u64, 8);
            if c.cc_ne() { blk = 922; continue; }
            blk = 553; continue;
        }
        553 => {
            let Some(t548) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087786c); };
            c.w32(7, t548);
            let r549 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r549);
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087787b); };
            c.w64(0, t550);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700087787f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 555; continue; }
            blk = 554; continue;
        }
        554 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r551 = c.r64(11) & c.r64(7);
            c.w64(11, r551);
            c.logic(r551, 64);
            let Some(t552) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877891); };
            c.logic(t552 & 0x4u64, 8);
            if c.cc_ne() { blk = 919; continue; }
            blk = 555; continue;
        }
        555 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700087789c); }
            blk = 174; continue;
        }
        556 => {
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008778a5); };
            c.w64(1, t553);
            blk = 808; continue;
        }
        557 => {
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008778ae); };
            c.w64(1, t554);
            blk = 808; continue;
        }
        558 => {
            c.w64(8, c.r64(1));
            let r555 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r555);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 559; continue;
        }
        559 => {
            let r556 = c.r32(9) & 0xfcu64;
            c.w32(9, r556);
            c.logic(r556, 32);
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 573; continue; }
            blk = 560; continue;
        }
        560 => {
            let Some(t557) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008778d7); };
            c.w32(8, t557);
            let r558 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r558);
            let Some(t559) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008778de); };
            c.w32(8, t559);
            let r560 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r560);
            let Some(t561) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008778e5); };
            c.w32(9, t561);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6du64, 64);
            if c.cc_a() { blk = 562; continue; }
            blk = 561; continue;
        }
        561 => {
            return RegfileExit::fault(0x70008778f5);
        }
        562 => {
            let Some(t562) = c.ld32(c.r64(8).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008778f6); };
            c.cmp(t562, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 563; continue;
        }
        563 => {
            c.w64(7, c.r64(15));
            let r563 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r563);
            c.logic(r563, 64);
            let Some(t564) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087790d); };
            c.w64(12, t564);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 570; continue; }
            blk = 564; continue;
        }
        564 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 570; continue; }
            blk = 565; continue;
        }
        565 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t565) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877921); };
            c.w64(3, t565);
            let Some(t566) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000877928); };
            c.w64(3, t566);
            c.w64(2, c.r64(12));
            let r567 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r567);
            c.logic(r567, 64);
            c.w64(6, c.r64(3));
            let r568 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r568);
            c.logic(r568, 64);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 568; continue; }
            blk = 566; continue;
        }
        566 => {
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_e() { blk = 569; continue; }
            blk = 567; continue;
        }
        567 => {
            blk = 570; continue;
        }
        568 => {
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_be() { blk = 570; continue; }
            blk = 569; continue;
        }
        569 => {
            let Some(t569) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000877950); };
            c.w32(7, t569);
            let Some(t570) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000877954); };
            c.cmp(t570, c.r32(7), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 570; continue;
        }
        570 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 572; continue; }
            blk = 571; continue;
        }
        571 => {
            return RegfileExit::fault(0x7000877967);
        }
        572 => {
            let Some(t571) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000877968); };
            c.w32(7, t571);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087796f); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r572 = c.r64(8) & c.r64(15);
            c.w64(8, r572);
            c.logic(r572, 64);
            let Some(t573) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087797d); };
            c.logic(t573 & 0x4u64, 8);
            if c.cc_ne() { blk = 916; continue; }
            blk = 573; continue;
        }
        573 => {
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let r574 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r574);
            c.w64(8, c.r64(0));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700087799b); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008779a2); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        574 => {
            let Some(t575) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008779ae); };
            c.logic(t575 & 0x200000u64, 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 575; continue;
        }
        575 => {
            let Some(t576) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008779bb); };
            c.w32(8, t576);
            let r577 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r577);
            let Some(t578) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008779c2); };
            c.w32(8, t578);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 808; continue; }
            blk = 576; continue;
        }
        576 => {
            let Some(t579) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008779d0); };
            c.w32(8, t579);
            let Some(t580) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008779d4); };
            c.cmp(t580, c.r32(8), 32);
            if c.cc_ne() { blk = 644; continue; }
            blk = 577; continue;
        }
        577 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 643; continue; }
            blk = 578; continue;
        }
        578 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_le() { blk = 582; continue; }
            blk = 579; continue;
        }
        579 => {
            c.w64(7, c.r64(11).wrapping_add(0x1));
            let r581 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r581);
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008779fe); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 581; continue; }
            blk = 580; continue;
        }
        580 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r582 = c.r64(9) & c.r64(0);
            c.w64(9, r582);
            c.logic(r582, 64);
            let Some(t583) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877a11); };
            c.logic(t583 & 0x4u64, 8);
            if c.cc_ne() { blk = 913; continue; }
            blk = 581; continue;
        }
        581 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000877a1c); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        582 => {
            let Some(t584) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000877a28); };
            c.w32(8, t584);
            let r585 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r585);
            let Some(t586) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000877a2f); };
            c.w32(9, t586);
            let Some(t587) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000877a33); };
            c.w32(8, t587);
            let r588 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r588);
            let Some(t589) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000877a3a); };
            c.cmp(t589, c.r32(9), 32);
            if c.cc_ne() { blk = 620; continue; }
            blk = 583; continue;
        }
        583 => {
            c.w64(9, c.r64(15));
            let r590 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r590);
            c.logic(r590, 64);
            let Some(t591) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877a51); };
            c.w64(12, t591);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 591; continue; }
            blk = 584; continue;
        }
        584 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 591; continue; }
            blk = 585; continue;
        }
        585 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t592) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877a65); };
            c.w64(6, t592);
            let Some(t593) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000877a6c); };
            c.w64(6, t593);
            c.w64(1, c.r64(12));
            let r594 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r594);
            c.logic(r594, 64);
            c.w64(11, c.r64(6));
            let r595 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r595);
            c.logic(r595, 64);
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_e() { blk = 588; continue; }
            blk = 586; continue;
        }
        586 => {
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_e() { blk = 589; continue; }
            blk = 587; continue;
        }
        587 => {
            blk = 591; continue;
        }
        588 => {
            c.cmp(c.r64(6), c.r64(12), 64);
            if c.cc_be() { blk = 591; continue; }
            blk = 589; continue;
        }
        589 => {
            let Some(t596) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000877a94); };
            c.w32(9, t596);
            let Some(t597) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000877a98); };
            c.cmp(t597, c.r32(9), 32);
            if c.cc_ne() { blk = 591; continue; }
            blk = 590; continue;
        }
        590 => {
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877aa1); };
            c.w64(1, t598);
            blk = 808; continue;
        }
        591 => {
            let Some(t599) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000877aaa); };
            c.w32(9, t599);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x69u64, 64);
            if c.cc_a() { blk = 593; continue; }
            blk = 592; continue;
        }
        592 => {
            return RegfileExit::fault(0x7000877aba);
        }
        593 => {
            let Some(t600) = c.ld32(c.r64(8).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000877abb); };
            c.cmp(t600, c.r32(7), 32);
            if c.cc_ne() { blk = 597; continue; }
            blk = 594; continue;
        }
        594 => {
            c.cmp(c.r64(9), 0x6du64, 64);
            if c.cc_a() { blk = 596; continue; }
            blk = 595; continue;
        }
        595 => {
            return RegfileExit::fault(0x7000877aca);
        }
        596 => {
            let Some(t601) = c.ld32(c.r64(8).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000877acb); };
            c.w32(7, t601);
            blk = 603; continue;
        }
        597 => {
            c.cmp(c.r64(9), 0x6au64, 64);
            if c.cc_a() { blk = 599; continue; }
            blk = 598; continue;
        }
        598 => {
            return RegfileExit::fault(0x7000877ada);
        }
        599 => {
            let Some(t602) = c.ld32(c.r64(8).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000877adb); };
            c.cmp(t602, c.r32(7), 32);
            if c.cc_ne() { blk = 619; continue; }
            blk = 600; continue;
        }
        600 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 602; continue; }
            blk = 601; continue;
        }
        601 => {
            return RegfileExit::fault(0x7000877aee);
        }
        602 => {
            let Some(t603) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000877aef); };
            c.w32(7, t603);
            blk = 603; continue;
        }
        603 => {
            c.cmp(c.r64(3), 0x3fffu64, 64);
            if c.cc_ae() { blk = 618; continue; }
            blk = 604; continue;
        }
        604 => {
            c.cmp(c.r64(3), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 842; continue; }
            blk = 605; continue;
        }
        605 => {
            c.w64(8, c.r64(2).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 838; continue; }
            blk = 606; continue;
        }
        606 => {
            let Some(t604) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877b25); };
            c.w64(9, t604);
            let Some(t605) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000877b2c); };
            c.w64(11, t605);
            c.w64(12, c.r64(8).wrapping_add(c.r64(11)));
            let Some(t606) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877b33); };
            c.cmp(t606, c.r64(12), 64);
            if c.cc_be() { blk = 840; continue; }
            blk = 607; continue;
        }
        607 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000877b3d); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 608; continue;
        }
        608 => {
            c.w32(9, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t607) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000877b48); };
            c.w64(11, t607);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000877b4f); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000877b53); }
            c.w64(9, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 614; continue; }
            blk = 609; continue;
        }
        609 => {
            c.w64(11, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            blk = 610; continue;
        }
        610 => {
            let r608 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r608);
            let Some(t609) = c.ld32(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x7000877b71); };
            c.w32(12, t609);
            let r610 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r610);
            let Some(t611) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000877b79); };
            c.cmp(t611, c.r32(12), 32);
            if c.cc_e() { blk = 612; continue; }
            blk = 611; continue;
        }
        611 => {
            let r612 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r612);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(c.r64(8)), 0) { return RegfileExit::fault(0x7000877b8e); }
            blk = 613; continue;
        }
        612 => {
            let Some(t613) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x7000877b96); };
            c.w64(12, t613);
            if !c.st64(c.r64(11).wrapping_add(c.r64(8)), c.r64(12)) { return RegfileExit::fault(0x7000877b9d); }
            blk = 613; continue;
        }
        613 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 610; continue; }
            blk = 614; continue;
        }
        614 => {
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000877ba7); };
            c.w64(11, t614);
            c.w64(9, c.r64(11).wrapping_add(0x1));
            let r615 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r615);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000877bb2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 616; continue; }
            blk = 615; continue;
        }
        615 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r616 = c.r64(12) & c.r64(15);
            c.w64(12, r616);
            c.logic(r616, 64);
            let Some(t617) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877bc6); };
            c.logic(t617 & 0x4u64, 8);
            if c.cc_ne() { blk = 910; continue; }
            blk = 616; continue;
        }
        616 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000877bd2); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r618 = c.r64(8) & c.r64(15);
            c.w64(8, r618);
            c.logic(r618, 64);
            let Some(t619) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877be0); };
            c.logic(t619 & 0x4u64, 8);
            if c.cc_ne() { blk = 907; continue; }
            blk = 617; continue;
        }
        617 => {
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877beb); };
            c.w64(1, t620);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000877bef); }
            let Some(t621) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000877bf4); };
            c.w32(7, t621);
            let r622 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r622);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000877c05); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000877c0c); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        618 => {
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877c18); };
            c.w64(1, t623);
            blk = 808; continue;
        }
        619 => {
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877c21); };
            c.w64(1, t624);
            blk = 808; continue;
        }
        620 => {
            c.w64(9, c.r64(15));
            let r625 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r625);
            c.logic(r625, 64);
            let Some(t626) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877c34); };
            c.w64(12, t626);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 627; continue; }
            blk = 621; continue;
        }
        621 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 627; continue; }
            blk = 622; continue;
        }
        622 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t627) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877c48); };
            c.w64(3, t627);
            let Some(t628) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000877c4f); };
            c.w64(3, t628);
            c.w64(2, c.r64(12));
            let r629 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r629);
            c.logic(r629, 64);
            c.w64(6, c.r64(3));
            let r630 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r630);
            c.logic(r630, 64);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 625; continue; }
            blk = 623; continue;
        }
        623 => {
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_e() { blk = 626; continue; }
            blk = 624; continue;
        }
        624 => {
            blk = 627; continue;
        }
        625 => {
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_be() { blk = 627; continue; }
            blk = 626; continue;
        }
        626 => {
            let Some(t631) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000877c77); };
            c.w32(9, t631);
            let Some(t632) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000877c7b); };
            c.cmp(t632, c.r32(9), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 627; continue;
        }
        627 => {
            let Some(t633) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000877c88); };
            c.w32(9, t633);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x69u64, 64);
            if c.cc_a() { blk = 629; continue; }
            blk = 628; continue;
        }
        628 => {
            return RegfileExit::fault(0x7000877c98);
        }
        629 => {
            let Some(t634) = c.ld32(c.r64(8).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000877c99); };
            c.cmp(t634, c.r32(7), 32);
            if c.cc_ne() { blk = 633; continue; }
            blk = 630; continue;
        }
        630 => {
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 632; continue; }
            blk = 631; continue;
        }
        631 => {
            return RegfileExit::fault(0x7000877ca8);
        }
        632 => {
            let Some(t635) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000877ca9); };
            c.w32(7, t635);
            blk = 639; continue;
        }
        633 => {
            c.cmp(c.r64(9), 0x6au64, 64);
            if c.cc_a() { blk = 635; continue; }
            blk = 634; continue;
        }
        634 => {
            return RegfileExit::fault(0x7000877cb8);
        }
        635 => {
            let Some(t636) = c.ld32(c.r64(8).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000877cb9); };
            c.cmp(t636, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 636; continue;
        }
        636 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 638; continue; }
            blk = 637; continue;
        }
        637 => {
            return RegfileExit::fault(0x7000877ccc);
        }
        638 => {
            let Some(t637) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000877ccd); };
            c.w32(7, t637);
            blk = 639; continue;
        }
        639 => {
            c.w64(8, c.r64(11).wrapping_add(0x1));
            let r638 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r638);
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000877ce3); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r639 = c.r64(11) & c.r64(15);
            c.w64(11, r639);
            c.logic(r639, 64);
            let Some(t640) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877cf1); };
            c.logic(t640 & 0x4u64, 8);
            if c.cc_ne() { blk = 904; continue; }
            blk = 640; continue;
        }
        640 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x7000877cfc); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 642; continue; }
            blk = 641; continue;
        }
        641 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r641 = c.r64(7) & c.r64(0);
            c.w64(7, r641);
            c.logic(r641, 64);
            let Some(t642) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877d0f); };
            c.logic(t642 & 0x4u64, 8);
            if c.cc_ne() { blk = 901; continue; }
            blk = 642; continue;
        }
        642 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000877d19); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        643 => {
            c.w64(7, c.r64(11).wrapping_add(0x1));
            let r643 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r643);
            if !c.st32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000877d2b); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000877d30); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        644 => {
            let Some(t644) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000877d3c); };
            c.cmp(t644, c.r32(8), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 645; continue;
        }
        645 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 692; continue; }
            blk = 646; continue;
        }
        646 => {
            let Some(t645) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000877d52); };
            c.w32(8, t645);
            let Some(t646) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000877d56); };
            c.cmp(t646, c.r32(8), 32);
            if c.cc_ne() { blk = 648; continue; }
            blk = 647; continue;
        }
        647 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000877d5f); }
            blk = 693; continue;
        }
        648 => {
            let Some(t647) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000877d69); };
            c.w32(8, t647);
            let r648 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r648);
            c.w64(9, c.r64(15));
            let r649 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r649);
            c.logic(r649, 64);
            let Some(t650) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000877d7a); };
            c.w32(8, t650);
            let r651 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r651);
            let Some(t652) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877d81); };
            c.w64(12, t652);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 656; continue; }
            blk = 649; continue;
        }
        649 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 656; continue; }
            blk = 650; continue;
        }
        650 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t653) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877d95); };
            c.w64(6, t653);
            let Some(t654) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000877d9c); };
            c.w64(6, t654);
            c.w64(1, c.r64(12));
            let r655 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r655);
            c.logic(r655, 64);
            c.w64(11, c.r64(6));
            let r656 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r656);
            c.logic(r656, 64);
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_e() { blk = 653; continue; }
            blk = 651; continue;
        }
        651 => {
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_e() { blk = 654; continue; }
            blk = 652; continue;
        }
        652 => {
            blk = 656; continue;
        }
        653 => {
            c.cmp(c.r64(6), c.r64(12), 64);
            if c.cc_be() { blk = 656; continue; }
            blk = 654; continue;
        }
        654 => {
            let Some(t657) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000877dc4); };
            c.w32(9, t657);
            let Some(t658) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000877dc8); };
            c.cmp(t658, c.r32(9), 32);
            if c.cc_ne() { blk = 656; continue; }
            blk = 655; continue;
        }
        655 => {
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877dd1); };
            c.w64(1, t659);
            blk = 808; continue;
        }
        656 => {
            let Some(t660) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000877dda); };
            c.w32(9, t660);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6du64, 64);
            if c.cc_a() { blk = 658; continue; }
            blk = 657; continue;
        }
        657 => {
            return RegfileExit::fault(0x7000877dea);
        }
        658 => {
            let Some(t661) = c.ld32(c.r64(8).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000877deb); };
            c.cmp(t661, c.r32(7), 32);
            if c.cc_ne() { blk = 662; continue; }
            blk = 659; continue;
        }
        659 => {
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 661; continue; }
            blk = 660; continue;
        }
        660 => {
            return RegfileExit::fault(0x7000877dfa);
        }
        661 => {
            let Some(t662) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000877dfb); };
            c.w32(7, t662);
            blk = 668; continue;
        }
        662 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 664; continue; }
            blk = 663; continue;
        }
        663 => {
            return RegfileExit::fault(0x7000877e0a);
        }
        664 => {
            let Some(t663) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000877e0b); };
            c.cmp(t663, c.r32(7), 32);
            if c.cc_ne() { blk = 691; continue; }
            blk = 665; continue;
        }
        665 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 667; continue; }
            blk = 666; continue;
        }
        666 => {
            return RegfileExit::fault(0x7000877e1e);
        }
        667 => {
            let Some(t664) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000877e1f); };
            c.w32(7, t664);
            blk = 668; continue;
        }
        668 => {
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 690; continue; }
            blk = 669; continue;
        }
        669 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 848; continue; }
            blk = 670; continue;
        }
        670 => {
            c.w32(8, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t665) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877e44); };
            c.w64(9, t665);
            let Some(t666) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000877e4b); };
            c.w64(11, t666);
            c.w64(12, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            c.w64(1, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t667) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000877e6c); };
            c.w64(6, t667);
            let Some(t668) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000877e73); };
            c.cmp(t668, c.r64(3), 64);
            if c.cc_be() { blk = 844; continue; }
            blk = 671; continue;
        }
        671 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x7000877e7c); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 672; continue;
        }
        672 => {
            let Some(t669) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000877e83); };
            c.w64(11, t669);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000877e8a); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000877e8e); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 675; continue; }
            blk = 673; continue;
        }
        673 => {
            c.w64(8, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            let Some(t670) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000877e9f); };
            c.w64(11, t670);
            blk = 674; continue;
        }
        674 => {
            let r671 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r671);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x7000877eaa); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 674; continue; }
            blk = 675; continue;
        }
        675 => {
            c.w64(8, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 684; continue; }
            blk = 676; continue;
        }
        676 => {
            c.w64(11, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            let Some(t672) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x7000877ece); };
            c.w64(12, t672);
            blk = 678; continue;
        }
        677 => {
            let r673 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r673);
            let Some(t674) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000877edb); };
            c.w64(0, t674);
            blk = 678; continue;
        }
        678 => {
            let r675 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r675);
            let Some(t676) = c.ld64(c.r64(8).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x7000877ee3); };
            c.cmp(t676, c.r64(12), 64);
            if c.cc_e() { blk = 683; continue; }
            blk = 679; continue;
        }
        679 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(0))) { return RegfileExit::fault(0x7000877ee9); }
            let Some(t677) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000877eef); };
            c.w64(3, t677);
            let Some(t678) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000877ef6); };
            c.w64(2, t678);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            c.w64(6, c.r64(11).wrapping_sub(0x4));
            let Some(t679) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000877f01); };
            c.w64(0, t679);
            let Some(t680) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000877f08); };
            c.cmp(t680, c.r64(1), 64);
            if c.cc_be() { blk = 846; continue; }
            blk = 680; continue;
        }
        680 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x7000877f11); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 681; continue;
        }
        681 => {
            let Some(t681) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000877f18); };
            c.w64(3, t681);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000877f1f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000877f22); }
            if !c.st32(c.r64(9).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x7000877f27); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 683; continue; }
            blk = 682; continue;
        }
        682 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r682 = c.r64(3) & c.r64(9);
            c.w64(3, r682);
            c.logic(r682, 64);
            let Some(t683) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877f39); };
            c.logic(t683 & 0x4u64, 8);
            if c.cc_ne() { blk = 898; continue; }
            blk = 683; continue;
        }
        683 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 677; continue; }
            blk = 684; continue;
        }
        684 => {
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000877f49); };
            c.w64(11, t684);
            c.w64(8, c.r64(11).wrapping_add(0x1));
            let r685 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r685);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000877f5c); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 686; continue; }
            blk = 685; continue;
        }
        685 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r686 = c.r64(12) & c.r64(15);
            c.w64(12, r686);
            c.logic(r686, 64);
            let Some(t687) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877f70); };
            c.logic(t687 & 0x4u64, 8);
            if c.cc_ne() { blk = 895; continue; }
            blk = 686; continue;
        }
        686 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000877f7c); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r688 = c.r64(9) & c.r64(15);
            c.w64(9, r688);
            c.logic(r688, 64);
            let Some(t689) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877f8a); };
            c.logic(t689 & 0x4u64, 8);
            if c.cc_ne() { blk = 892; continue; }
            blk = 687; continue;
        }
        687 => {
            let Some(t690) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000877f95); };
            c.w32(7, t690);
            let r691 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r691);
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877f9c); };
            c.w64(0, t692);
            if !c.st32(c.r64(7).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x7000877fa0); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 689; continue; }
            blk = 688; continue;
        }
        688 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r693 = c.r64(9) & c.r64(7);
            c.w64(9, r693);
            c.logic(r693, 64);
            let Some(t694) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000877fb2); };
            c.logic(t694 & 0x4u64, 8);
            if c.cc_ne() { blk = 889; continue; }
            blk = 689; continue;
        }
        689 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000877fbd); }
            blk = 174; continue;
        }
        690 => {
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877fc6); };
            c.w64(1, t695);
            blk = 808; continue;
        }
        691 => {
            let Some(t696) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000877fcf); };
            c.w64(1, t696);
            blk = 808; continue;
        }
        692 => {
            c.w64(7, c.r64(1));
            let r697 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r697);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 693; continue;
        }
        693 => {
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let r698 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r698);
            c.w64(8, c.r64(0));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000877ff8); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x7000877fff); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        694 => {
            let Some(t699) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087800b); };
            c.w32(8, t699);
            let Some(t700) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700087800f); };
            c.cmp(t700, c.r32(8), 32);
            if c.cc_ne() { blk = 759; continue; }
            blk = 695; continue;
        }
        695 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 758; continue; }
            blk = 696; continue;
        }
        696 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_le() { blk = 700; continue; }
            blk = 697; continue;
        }
        697 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x7000878033); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 699; continue; }
            blk = 698; continue;
        }
        698 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r701 = c.r64(8) & c.r64(0);
            c.w64(8, r701);
            c.logic(r701, 64);
            let Some(t702) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878045); };
            c.logic(t702 & 0x4u64, 8);
            if c.cc_ne() { blk = 886; continue; }
            blk = 699; continue;
        }
        699 => {
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        700 => {
            let Some(t703) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000878058); };
            c.w32(8, t703);
            let r704 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r704);
            let Some(t705) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087805f); };
            c.w32(9, t705);
            let Some(t706) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000878063); };
            c.w32(8, t706);
            let r707 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r707);
            let Some(t708) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087806a); };
            c.cmp(t708, c.r32(9), 32);
            if c.cc_ne() { blk = 735; continue; }
            blk = 701; continue;
        }
        701 => {
            c.w64(9, c.r64(15));
            let r709 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r709);
            c.logic(r709, 64);
            let Some(t710) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878081); };
            c.w64(12, t710);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 708; continue; }
            blk = 702; continue;
        }
        702 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 708; continue; }
            blk = 703; continue;
        }
        703 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t711) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000878095); };
            c.w64(2, t711);
            let Some(t712) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700087809c); };
            c.w64(2, t712);
            c.w64(3, c.r64(12));
            let r713 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r713);
            c.logic(r713, 64);
            c.w64(6, c.r64(2));
            let r714 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r714);
            c.logic(r714, 64);
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_e() { blk = 706; continue; }
            blk = 704; continue;
        }
        704 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 707; continue; }
            blk = 705; continue;
        }
        705 => {
            blk = 708; continue;
        }
        706 => {
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_be() { blk = 708; continue; }
            blk = 707; continue;
        }
        707 => {
            let Some(t715) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008780c4); };
            c.w32(9, t715);
            let Some(t716) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008780c8); };
            c.cmp(t716, c.r32(9), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 708; continue;
        }
        708 => {
            let Some(t717) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008780d5); };
            c.w32(9, t717);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x69u64, 64);
            if c.cc_a() { blk = 710; continue; }
            blk = 709; continue;
        }
        709 => {
            return RegfileExit::fault(0x70008780e5);
        }
        710 => {
            let Some(t718) = c.ld32(c.r64(8).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008780e6); };
            c.cmp(t718, c.r32(7), 32);
            if c.cc_ne() { blk = 714; continue; }
            blk = 711; continue;
        }
        711 => {
            c.cmp(c.r64(9), 0x6du64, 64);
            if c.cc_a() { blk = 713; continue; }
            blk = 712; continue;
        }
        712 => {
            return RegfileExit::fault(0x70008780f5);
        }
        713 => {
            let Some(t719) = c.ld32(c.r64(8).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008780f6); };
            c.w32(7, t719);
            blk = 720; continue;
        }
        714 => {
            c.cmp(c.r64(9), 0x6au64, 64);
            if c.cc_a() { blk = 716; continue; }
            blk = 715; continue;
        }
        715 => {
            return RegfileExit::fault(0x7000878105);
        }
        716 => {
            let Some(t720) = c.ld32(c.r64(8).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000878106); };
            c.cmp(t720, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 717; continue;
        }
        717 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 719; continue; }
            blk = 718; continue;
        }
        718 => {
            return RegfileExit::fault(0x7000878119);
        }
        719 => {
            let Some(t721) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x700087811a); };
            c.w32(7, t721);
            blk = 720; continue;
        }
        720 => {
            let Some(t722) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000878121); };
            c.w32(8, t722);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x3fffu64, 64);
            if c.cc_ae() { blk = 808; continue; }
            blk = 721; continue;
        }
        721 => {
            c.cmp(c.r64(9), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 854; continue; }
            blk = 722; continue;
        }
        722 => {
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 850; continue; }
            blk = 723; continue;
        }
        723 => {
            let Some(t723) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087815d); };
            c.w64(3, t723);
            let Some(t724) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000878164); };
            c.w64(2, t724);
            c.w64(6, c.r64(12).wrapping_add(c.r64(2)));
            let Some(t725) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087816b); };
            c.cmp(t725, c.r64(6), 64);
            if c.cc_be() { blk = 852; continue; }
            blk = 724; continue;
        }
        724 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000878175); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 725; continue;
        }
        725 => {
            let r726 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r726);
            let Some(t727) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700087817f); };
            c.w64(3, t727);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000878186); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700087818b); }
            c.w64(9, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 731; continue; }
            blk = 726; continue;
        }
        726 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            blk = 727; continue;
        }
        727 => {
            let r728 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r728);
            let Some(t729) = c.ld32(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008781aa); };
            c.w32(3, t729);
            let r730 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r730);
            let Some(t731) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008781b2); };
            c.cmp(t731, c.r32(3), 32);
            if c.cc_e() { blk = 729; continue; }
            blk = 728; continue;
        }
        728 => {
            let r732 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r732);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12)), 0) { return RegfileExit::fault(0x70008781c5); }
            blk = 730; continue;
        }
        729 => {
            let Some(t733) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x70008781cd); };
            c.w64(3, t733);
            if !c.st64(c.r64(8).wrapping_add(c.r64(12)), c.r64(3)) { return RegfileExit::fault(0x70008781d4); }
            blk = 730; continue;
        }
        730 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 727; continue; }
            blk = 731; continue;
        }
        731 => {
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008781de); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 733; continue; }
            blk = 732; continue;
        }
        732 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r734 = c.r64(8) & c.r64(15);
            c.w64(8, r734);
            c.logic(r734, 64);
            let Some(t735) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008781f2); };
            c.logic(t735 & 0x4u64, 8);
            if c.cc_ne() { blk = 883; continue; }
            blk = 733; continue;
        }
        733 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008781fd); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r736 = c.r64(8) & c.r64(15);
            c.w64(8, r736);
            c.logic(r736, 64);
            let Some(t737) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087820b); };
            c.logic(t737 & 0x4u64, 8);
            if c.cc_ne() { blk = 880; continue; }
            blk = 734; continue;
        }
        734 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000878216); }
            let Some(t738) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087821b); };
            c.w32(7, t738);
            let r739 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r739);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700087822c); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        735 => {
            c.w64(9, c.r64(15));
            let r740 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r740);
            c.logic(r740, 64);
            let Some(t741) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878245); };
            c.w64(12, t741);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 742; continue; }
            blk = 736; continue;
        }
        736 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 742; continue; }
            blk = 737; continue;
        }
        737 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t742) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000878259); };
            c.w64(2, t742);
            let Some(t743) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000878260); };
            c.w64(2, t743);
            c.w64(3, c.r64(12));
            let r744 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r744);
            c.logic(r744, 64);
            c.w64(6, c.r64(2));
            let r745 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r745);
            c.logic(r745, 64);
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_e() { blk = 740; continue; }
            blk = 738; continue;
        }
        738 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 741; continue; }
            blk = 739; continue;
        }
        739 => {
            blk = 742; continue;
        }
        740 => {
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_be() { blk = 742; continue; }
            blk = 741; continue;
        }
        741 => {
            let Some(t746) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000878288); };
            c.w32(9, t746);
            let Some(t747) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700087828c); };
            c.cmp(t747, c.r32(9), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 742; continue;
        }
        742 => {
            let Some(t748) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000878299); };
            c.w32(9, t748);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x69u64, 64);
            if c.cc_a() { blk = 744; continue; }
            blk = 743; continue;
        }
        743 => {
            return RegfileExit::fault(0x70008782a9);
        }
        744 => {
            let Some(t749) = c.ld32(c.r64(8).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008782aa); };
            c.cmp(t749, c.r32(7), 32);
            if c.cc_ne() { blk = 748; continue; }
            blk = 745; continue;
        }
        745 => {
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 747; continue; }
            blk = 746; continue;
        }
        746 => {
            return RegfileExit::fault(0x70008782b9);
        }
        747 => {
            let Some(t750) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008782ba); };
            c.w32(7, t750);
            blk = 754; continue;
        }
        748 => {
            c.cmp(c.r64(9), 0x6au64, 64);
            if c.cc_a() { blk = 750; continue; }
            blk = 749; continue;
        }
        749 => {
            return RegfileExit::fault(0x70008782c9);
        }
        750 => {
            let Some(t751) = c.ld32(c.r64(8).wrapping_add(0x1af)) else { return RegfileExit::fault(0x70008782ca); };
            c.cmp(t751, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 751; continue;
        }
        751 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 753; continue; }
            blk = 752; continue;
        }
        752 => {
            return RegfileExit::fault(0x70008782dd);
        }
        753 => {
            let Some(t752) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x70008782de); };
            c.w32(7, t752);
            blk = 754; continue;
        }
        754 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008782ed); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r753 = c.r64(9) & c.r64(15);
            c.w64(9, r753);
            c.logic(r753, 64);
            let Some(t754) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008782fb); };
            c.logic(t754 & 0x4u64, 8);
            if c.cc_ne() { blk = 877; continue; }
            blk = 755; continue;
        }
        755 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000878306); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 757; continue; }
            blk = 756; continue;
        }
        756 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r755 = c.r64(7) & c.r64(0);
            c.w64(7, r755);
            c.logic(r755, 64);
            let Some(t756) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878319); };
            c.logic(t756 & 0x4u64, 8);
            if c.cc_ne() { blk = 874; continue; }
            blk = 757; continue;
        }
        757 => {
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        758 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700087832b); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        759 => {
            let Some(t757) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000878338); };
            c.cmp(t757, c.r32(8), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 760; continue;
        }
        760 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 804; continue; }
            blk = 761; continue;
        }
        761 => {
            let Some(t758) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087834e); };
            c.w32(8, t758);
            let Some(t759) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000878352); };
            c.cmp(t759, c.r32(8), 32);
            if c.cc_ne() { blk = 763; continue; }
            blk = 762; continue;
        }
        762 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700087835b); }
            blk = 805; continue;
        }
        763 => {
            let Some(t760) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000878365); };
            c.w32(8, t760);
            let r761 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r761);
            c.w64(9, c.r64(15));
            let r762 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r762);
            c.logic(r762, 64);
            let Some(t763) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000878376); };
            c.w32(8, t763);
            let r764 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r764);
            let Some(t765) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087837d); };
            c.w64(12, t765);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 770; continue; }
            blk = 764; continue;
        }
        764 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 770; continue; }
            blk = 765; continue;
        }
        765 => {
            c.w64(12, c.r64(15).wrapping_add(0x13));
            let Some(t766) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000878391); };
            c.w64(2, t766);
            let Some(t767) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000878398); };
            c.w64(2, t767);
            c.w64(3, c.r64(12));
            let r768 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r768);
            c.logic(r768, 64);
            c.w64(6, c.r64(2));
            let r769 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r769);
            c.logic(r769, 64);
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_e() { blk = 768; continue; }
            blk = 766; continue;
        }
        766 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 769; continue; }
            blk = 767; continue;
        }
        767 => {
            blk = 770; continue;
        }
        768 => {
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_be() { blk = 770; continue; }
            blk = 769; continue;
        }
        769 => {
            let Some(t770) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008783c0); };
            c.w32(9, t770);
            let Some(t771) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008783c4); };
            c.cmp(t771, c.r32(9), 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 770; continue;
        }
        770 => {
            let Some(t772) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008783d1); };
            c.w32(9, t772);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6du64, 64);
            if c.cc_a() { blk = 772; continue; }
            blk = 771; continue;
        }
        771 => {
            return RegfileExit::fault(0x70008783e1);
        }
        772 => {
            let Some(t773) = c.ld32(c.r64(8).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008783e2); };
            c.cmp(t773, c.r32(7), 32);
            if c.cc_ne() { blk = 776; continue; }
            blk = 773; continue;
        }
        773 => {
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 775; continue; }
            blk = 774; continue;
        }
        774 => {
            return RegfileExit::fault(0x70008783f1);
        }
        775 => {
            let Some(t774) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008783f2); };
            c.w32(7, t774);
            blk = 782; continue;
        }
        776 => {
            c.cmp(c.r64(9), 0x6eu64, 64);
            if c.cc_a() { blk = 778; continue; }
            blk = 777; continue;
        }
        777 => {
            return RegfileExit::fault(0x7000878401);
        }
        778 => {
            let Some(t775) = c.ld32(c.r64(8).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000878402); };
            c.cmp(t775, c.r32(7), 32);
            if c.cc_ne() { blk = 808; continue; }
            blk = 779; continue;
        }
        779 => {
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 781; continue; }
            blk = 780; continue;
        }
        780 => {
            return RegfileExit::fault(0x7000878415);
        }
        781 => {
            let Some(t776) = c.ld32(c.r64(8).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000878416); };
            c.w32(7, t776);
            blk = 782; continue;
        }
        782 => {
            let Some(t777) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087841d); };
            c.w32(8, t777);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700087842a); }
            c.cmp(c.r64(9), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 808; continue; }
            blk = 783; continue;
        }
        783 => {
            c.cmp(c.r64(9), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 860; continue; }
            blk = 784; continue;
        }
        784 => {
            let Some(t778) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000878448); };
            c.w64(12, t778);
            let Some(t779) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087844f); };
            c.w64(3, t779);
            c.w64(2, c.r64(8).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t780) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087845f); };
            c.w64(1, t780);
            let Some(t781) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000878466); };
            c.cmp(t781, c.r64(6), 64);
            if c.cc_be() { blk = 856; continue; }
            blk = 785; continue;
        }
        785 => {
            if !c.st64(c.r64(12), c.r64(6)) { return RegfileExit::fault(0x700087846f); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            blk = 786; continue;
        }
        786 => {
            let r782 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r782);
            let Some(t783) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700087847a); };
            c.w64(3, t783);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000878481); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000878486); }
            c.w64(9, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 789; continue; }
            blk = 787; continue;
        }
        787 => {
            c.w64(3, c.r64(9));
            let Some(t784) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087849c); };
            c.w64(2, t784);
            blk = 788; continue;
        }
        788 => {
            let r785 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r785);
            if !c.st32(c.r64(3).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70008784a7); }
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_ne() { blk = 788; continue; }
            blk = 789; continue;
        }
        789 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 798; continue; }
            blk = 790; continue;
        }
        790 => {
            let Some(t786) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x70008784c3); };
            c.w64(3, t786);
            blk = 792; continue;
        }
        791 => {
            let r787 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r787);
            let Some(t788) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008784d0); };
            c.w64(0, t788);
            blk = 792; continue;
        }
        792 => {
            let r789 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r789);
            let Some(t790) = c.ld64(c.r64(8).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008784d8); };
            c.cmp(t790, c.r64(3), 64);
            if c.cc_e() { blk = 797; continue; }
            blk = 793; continue;
        }
        793 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(0))) { return RegfileExit::fault(0x70008784de); }
            let Some(t791) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008784e4); };
            c.w64(2, t791);
            let Some(t792) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008784eb); };
            c.w64(1, t792);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            c.w64(0, c.r64(9).wrapping_sub(0x4));
            let Some(t793) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008784f6); };
            c.w64(7, t793);
            let Some(t794) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008784fd); };
            c.cmp(t794, c.r64(6), 64);
            if c.cc_be() { blk = 858; continue; }
            blk = 794; continue;
        }
        794 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000878506); }
            c.w64(7, c.r64(1).wrapping_add(0x1));
            blk = 795; continue;
        }
        795 => {
            let Some(t795) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087850d); };
            c.w64(2, t795);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000878514); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000878517); }
            if !c.st32(c.r64(12).wrapping_add(c.r64(0)), c.r32(7)) { return RegfileExit::fault(0x700087851c); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 797; continue; }
            blk = 796; continue;
        }
        796 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r796 = c.r64(2) & c.r64(12);
            c.w64(2, r796);
            c.logic(r796, 64);
            let Some(t797) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878530); };
            c.logic(t797 & 0x4u64, 8);
            if c.cc_ne() { blk = 871; continue; }
            blk = 797; continue;
        }
        797 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 791; continue; }
            blk = 798; continue;
        }
        798 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000878548); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 800; continue; }
            blk = 799; continue;
        }
        799 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r798 = c.r64(8) & c.r64(15);
            c.w64(8, r798);
            c.logic(r798, 64);
            let Some(t799) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087855c); };
            c.logic(t799 & 0x4u64, 8);
            if c.cc_ne() { blk = 868; continue; }
            blk = 800; continue;
        }
        800 => {
            let Some(t800) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000878567); };
            c.w64(8, t800);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087856b); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r801 = c.r64(9) & c.r64(15);
            c.w64(9, r801);
            c.logic(r801, 64);
            let Some(t802) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878579); };
            c.logic(t802 & 0x4u64, 8);
            if c.cc_ne() { blk = 865; continue; }
            blk = 801; continue;
        }
        801 => {
            let Some(t803) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000878584); };
            c.w32(8, t803);
            let r804 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r804);
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087858b); };
            c.w64(0, t805);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700087858f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 803; continue; }
            blk = 802; continue;
        }
        802 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r806 = c.r64(9) & c.r64(8);
            c.w64(9, r806);
            c.logic(r806, 64);
            let Some(t807) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008785a1); };
            c.logic(t807 & 0x4u64, 8);
            if c.cc_ne() { blk = 862; continue; }
            blk = 803; continue;
        }
        803 => {
            blk = 174; continue;
        }
        804 => {
            c.w64(7, c.r64(1));
            let r808 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r808);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 805; continue;
        }
        805 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            c.w64(7, c.r64(0));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x70008785cb); }
            c.w64(0, c.r64(1));
            blk = 174; continue;
        }
        806 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        807 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        808 => {
            let Some(t809) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008785e2); };
            c.w64(5, t809);
            let Some(t810) = c.ld64(c.r64(13).wrapping_add(0x2488)) else { return RegfileExit::fault(0x70008785e6); };
            c.w64(3, t810);
            c.w32(0, 0x3u64);
            let Some(t811) = c.ld64(c.r64(4).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008785f2); };
            c.w64(10, t811);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x70008785f7); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(15)) { return RegfileExit::fault(0x70008785fc); }
            let Some(t812) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x7000878601); };
            c.w64(10, t812);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000878606); }
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(1)) { return RegfileExit::fault(0x700087860b); }
            let Some(t813) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000878610); };
            c.w64(6, t813);
            let r814 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r814);
            return RegfileExit::jump(0x700086ed74);
        }
        809 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 811; continue; }
            blk = 810; continue;
        }
        810 => {
            c.w64(15, c.r64(0));
            blk = 808; continue;
        }
        811 => {
            let Some(t815) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087862a); };
            c.w32(8, t815);
            let r816 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r816);
            blk = 27; continue;
        }
        812 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x700087863b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700087863f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000878643); }
            return c.call(0x70007febf4, 0x700087864d, 0x7000878648);
        }
        813 => {
            c.w64(11, c.r64(0));
            let Some(t817) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000878650); };
            c.w64(1, t817);
            let Some(t818) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000878654); };
            c.w64(7, t818);
            let Some(t819) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000878658); };
            c.w64(8, t819);
            let Some(t820) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087865c); };
            c.w64(9, t820);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000878660); }
            blk = 263; continue;
        }
        814 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x700087866a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700087866e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000878672); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000878676); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x700087867a); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000878686, 0x7000878681);
        }
        815 => {
            c.w64(7, c.r64(0));
            let Some(t821) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000878689); };
            c.w64(12, t821);
            let Some(t822) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087868d); };
            c.w64(8, t822);
            let Some(t823) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000878691); };
            c.w64(0, t823);
            let Some(t824) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000878695); };
            c.w64(15, t824);
            let Some(t825) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000878699); };
            c.w64(9, t825);
            let Some(t826) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087869d); };
            c.w64(3, t826);
            blk = 271; continue;
        }
        816 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70008786ab); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008786af); }
            return c.call(0x70007febf4, 0x70008786b9, 0x70008786b4);
        }
        817 => {
            c.w64(9, c.r64(0));
            let Some(t827) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008786bc); };
            c.w64(1, t827);
            let Some(t828) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008786c0); };
            c.w64(8, t828);
            let Some(t829) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008786c4); };
            c.w64(7, t829);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008786c8); }
            blk = 293; continue;
        }
        818 => {
            return RegfileExit::fault(0x70008786d2);
        }
        819 => {
            return RegfileExit::fault(0x70008786d3);
        }
        820 => {
            return RegfileExit::fault(0x70008786d4);
        }
        821 => {
            return RegfileExit::fault(0x70008786d5);
        }
        822 => {
            return RegfileExit::fault(0x70008786d6);
        }
        823 => {
            return RegfileExit::fault(0x70008786d7);
        }
        824 => {
            return RegfileExit::fault(0x70008786d8);
        }
        825 => {
            return RegfileExit::fault(0x70008786d9);
        }
        826 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t830) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008786dc); };
            c.w64(3, t830);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008786ea, 0x70008786e5);
        }
        827 => {
            return RegfileExit::fault(0x70008786ea);
        }
        828 => {
            let r831 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r831);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008786ee); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008786f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008786f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008786f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008786fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008786ff); }
            let Some(t832) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000878703); };
            c.w64(3, t832);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000878717, 0x7000878712);
        }
        829 => {
            c.w64(8, c.r64(0));
            let Some(t833) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087871a); };
            c.w64(15, t833);
            let Some(t834) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087871e); };
            c.w64(0, t834);
            let Some(t835) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000878722); };
            c.w64(2, t835);
            let Some(t836) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000878726); };
            c.w64(7, t836);
            let Some(t837) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087872a); };
            c.w64(1, t837);
            let Some(t838) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087872e); };
            c.w64(9, t838);
            blk = 445; continue;
        }
        830 => {
            let Some(t839) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000878737); };
            c.w64(3, t839);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000878747, 0x7000878742);
        }
        831 => {
            return RegfileExit::fault(0x7000878747);
        }
        832 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000878748); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x700087874c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000878750); }
            c.w64(2, c.r64(8));
            return c.call(0x70007febf4, 0x700087875c, 0x7000878757);
        }
        833 => {
            c.w64(8, c.r64(0));
            let Some(t840) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087875f); };
            c.w64(15, t840);
            let Some(t841) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000878763); };
            c.w64(0, t841);
            let Some(t842) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000878767); };
            c.w64(2, t842);
            let Some(t843) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087876b); };
            c.w64(3, t843);
            let Some(t844) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087876f); };
            c.w64(7, t844);
            blk = 538; continue;
        }
        834 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700087877d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x7000878781); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000878785); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000878789); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700087878d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000878791); }
            return c.call(0x70007febf4, 0x700087879b, 0x7000878796);
        }
        835 => {
            let Some(t845) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x700087879b); };
            c.w64(12, t845);
            let Some(t846) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008787a2); };
            c.w64(15, t846);
            let Some(t847) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008787a6); };
            c.w64(7, t847);
            let Some(t848) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008787aa); };
            c.w64(8, t848);
            let Some(t849) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008787ae); };
            c.w64(11, t849);
            let Some(t850) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008787b2); };
            c.w64(9, t850);
            let Some(t851) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008787b6); };
            c.w64(6, t851);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008787ba); }
            blk = 547; continue;
        }
        836 => {
            let Some(t852) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008787c4); };
            c.w64(3, t852);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008787d4, 0x70008787cf);
        }
        837 => {
            return RegfileExit::fault(0x70008787d4);
        }
        838 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t853) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008787d7); };
            c.w64(3, t853);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008787e5, 0x70008787e0);
        }
        839 => {
            return RegfileExit::fault(0x70008787e5);
        }
        840 => {
            let r854 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r854);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008787e9); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008787ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008787ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008787f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008787f6); }
            c.w32(0, 0x2u64);
            let Some(t855) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008787ff); };
            c.w64(3, t855);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000878811, 0x700087880c);
        }
        841 => {
            c.w64(8, c.r64(0));
            let Some(t856) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000878814); };
            c.w64(15, t856);
            let Some(t857) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000878818); };
            c.w64(0, t857);
            let Some(t858) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087881c); };
            c.w64(2, t858);
            let Some(t859) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000878820); };
            c.w64(3, t859);
            let Some(t860) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000878824); };
            c.w64(7, t860);
            blk = 608; continue;
        }
        842 => {
            let Some(t861) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700087882d); };
            c.w64(3, t861);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087883d, 0x7000878838);
        }
        843 => {
            return RegfileExit::fault(0x700087883d);
        }
        844 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700087883e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000878842); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000878846); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700087884a); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febf4, 0x7000878856, 0x7000878851);
        }
        845 => {
            c.w64(9, c.r64(0));
            let Some(t862) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000878859); };
            c.w64(15, t862);
            let Some(t863) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087885d); };
            c.w64(0, t863);
            let Some(t864) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000878861); };
            c.w64(7, t864);
            let Some(t865) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000878865); };
            c.w64(2, t865);
            let Some(t866) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000878869); };
            c.w64(1, t866);
            let Some(t867) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087886d); };
            c.w64(8, t867);
            blk = 672; continue;
        }
        846 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700087887b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700087887f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000878883); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000878887); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700087888b); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700087888f); }
            return c.call(0x70007febf4, 0x7000878899, 0x7000878894);
        }
        847 => {
            let Some(t868) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x7000878899); };
            c.w64(12, t868);
            let Some(t869) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008788a0); };
            c.w64(15, t869);
            let Some(t870) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008788a4); };
            c.w64(7, t870);
            let Some(t871) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008788a8); };
            c.w64(9, t871);
            let Some(t872) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008788ac); };
            c.w64(11, t872);
            let Some(t873) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008788b0); };
            c.w64(8, t873);
            let Some(t874) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008788b4); };
            c.w64(6, t874);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008788b8); }
            blk = 681; continue;
        }
        848 => {
            let Some(t875) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008788c2); };
            c.w64(3, t875);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008788d2, 0x70008788cd);
        }
        849 => {
            return RegfileExit::fault(0x70008788d2);
        }
        850 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t876) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008788d5); };
            c.w64(3, t876);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008788e3, 0x70008788de);
        }
        851 => {
            return RegfileExit::fault(0x70008788e3);
        }
        852 => {
            let r877 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r877);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008788e7); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008788eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008788ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008788f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008788f4); }
            let Some(t878) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008788f8); };
            c.w64(3, t878);
            c.w32(0, 0x2u64);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700087890c, 0x7000878907);
        }
        853 => {
            c.w64(12, c.r64(0));
            let Some(t879) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087890f); };
            c.w64(15, t879);
            let Some(t880) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000878913); };
            c.w64(1, t880);
            let Some(t881) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000878917); };
            c.w64(0, t881);
            let Some(t882) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087891b); };
            c.w64(11, t882);
            let Some(t883) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087891f); };
            c.w64(7, t883);
            let Some(t884) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000878923); };
            c.w64(8, t884);
            let Some(t885) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000878927); };
            c.w64(9, t885);
            blk = 725; continue;
        }
        854 => {
            let Some(t886) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000878930); };
            c.w64(3, t886);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000878940, 0x700087893b);
        }
        855 => {
            return RegfileExit::fault(0x7000878940);
        }
        856 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000878941); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000878945); }
            return c.call(0x70007febf4, 0x700087894e, 0x7000878949);
        }
        857 => {
            c.w64(12, c.r64(0));
            let Some(t887) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000878951); };
            c.w64(15, t887);
            let Some(t888) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000878955); };
            c.w64(0, t888);
            let Some(t889) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000878959); };
            c.w64(11, t889);
            let Some(t890) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087895d); };
            c.w64(7, t890);
            let Some(t891) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000878961); };
            c.w64(8, t891);
            let Some(t892) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000878965); };
            c.w64(9, t892);
            blk = 786; continue;
        }
        858 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x7000878973); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000878977); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700087897b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700087897f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000878983); }
            return c.call(0x70007febf4, 0x700087898d, 0x7000878988);
        }
        859 => {
            c.w64(7, c.r64(0));
            let Some(t893) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000878990); };
            c.w64(15, t893);
            let Some(t894) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000878994); };
            c.w64(11, t894);
            let Some(t895) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000878998); };
            c.w64(12, t895);
            let Some(t896) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087899c); };
            c.w64(9, t896);
            let Some(t897) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008789a0); };
            c.w64(8, t897);
            let Some(t898) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008789a4); };
            c.w64(0, t898);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008789a8); }
            let Some(t899) = c.ld64(0x700087731bu64) else { return RegfileExit::fault(0x70008789ad); };
            c.w64(3, t899);
            blk = 795; continue;
        }
        860 => {
            let Some(t900) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008789b9); };
            c.w64(3, t900);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008789c9, 0x70008789c4);
        }
        861 => {
            return RegfileExit::fault(0x70008789c9);
        }
        862 => {
            c.w32(0, c.r32(0));
            let r901 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r901);
            c.w64(9, 0xfffffffffffc0000u64);
            let r902 = c.r64(9) & c.r64(0);
            c.w64(9, r902);
            c.logic(r902, 64);
            let Some(t903) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008789d9); };
            c.logic(t903 & 0x2u64, 8);
            if c.cc_e() { blk = 803; continue; }
            blk = 863; continue;
        }
        863 => {
            c.w64(11, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008789e8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008789e9); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008789f6, 0x70008789f1);
        }
        864 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008789f6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008789f8); }
            blk = 803; continue;
        }
        865 => {
            c.w32(8, c.r32(8));
            let r904 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r904);
            c.w64(9, 0xfffffffffffc0000u64);
            let r905 = c.r64(9) & c.r64(8);
            c.w64(9, r905);
            c.logic(r905, 64);
            let Some(t906) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878a0e); };
            c.logic(t906 & 0x2u64, 8);
            if c.cc_e() { blk = 801; continue; }
            blk = 866; continue;
        }
        866 => {
            c.w64(11, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878a1d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878a1e); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000878a2b, 0x7000878a26);
        }
        867 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878a2b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878a2d); }
            blk = 801; continue;
        }
        868 => {
            c.w32(12, c.r32(12));
            let r907 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r907);
            c.w64(8, 0xfffffffffffc0000u64);
            let r908 = c.r64(8) & c.r64(12);
            c.w64(8, r908);
            c.logic(r908, 64);
            let Some(t909) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878a43); };
            c.logic(t909 & 0x2u64, 8);
            if c.cc_e() { blk = 800; continue; }
            blk = 869; continue;
        }
        869 => {
            c.w64(9, c.r64(15).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878a52); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878a53); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000878a60, 0x7000878a5b);
        }
        870 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878a60); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878a62); }
            blk = 800; continue;
        }
        871 => {
            c.w32(7, c.r32(7));
            let r910 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r910);
            c.w64(2, 0xfffffffffffc0000u64);
            let r911 = c.r64(2) & c.r64(7);
            c.w64(2, r911);
            c.logic(r911, 64);
            let Some(t912) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878a77); };
            c.logic(t912 & 0x2u64, 8);
            if c.cc_e() { blk = 797; continue; }
            blk = 872; continue;
        }
        872 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878a85); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878a86); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x7000878a93, 0x7000878a8e);
        }
        873 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878a93); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878a95); }
            blk = 797; continue;
        }
        874 => {
            c.w32(1, c.r32(1));
            let r913 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r913);
            c.w64(7, 0xfffffffffffc0000u64);
            let r914 = c.r64(7) & c.r64(1);
            c.w64(7, r914);
            c.logic(r914, 64);
            let Some(t915) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878aaa); };
            c.logic(t915 & 0x2u64, 8);
            if c.cc_e() { blk = 757; continue; }
            blk = 875; continue;
        }
        875 => {
            c.w64(9, c.r64(0).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878ab8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878ab9); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000878ac6, 0x7000878ac1);
        }
        876 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878ac6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878ac8); }
            blk = 757; continue;
        }
        877 => {
            c.w32(7, c.r32(7));
            let r916 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r916);
            c.w64(9, 0xfffffffffffc0000u64);
            let r917 = c.r64(9) & c.r64(7);
            c.w64(9, r917);
            c.logic(r917, 64);
            let Some(t918) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878add); };
            c.logic(t918 & 0x2u64, 8);
            if c.cc_e() { blk = 755; continue; }
            blk = 878; continue;
        }
        878 => {
            c.w64(11, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878aec); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878aed); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000878afa, 0x7000878af5);
        }
        879 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878afa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878afc); }
            blk = 755; continue;
        }
        880 => {
            c.w32(7, c.r32(7));
            let r919 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r919);
            c.w64(8, 0xfffffffffffc0000u64);
            let r920 = c.r64(8) & c.r64(7);
            c.w64(8, r920);
            c.logic(r920, 64);
            let Some(t921) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878b11); };
            c.logic(t921 & 0x2u64, 8);
            if c.cc_e() { blk = 734; continue; }
            blk = 881; continue;
        }
        881 => {
            c.w64(9, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878b20); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878b21); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000878b2e, 0x7000878b29);
        }
        882 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878b2e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878b30); }
            blk = 734; continue;
        }
        883 => {
            c.w32(12, c.r32(12));
            let r922 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r922);
            c.w64(8, 0xfffffffffffc0000u64);
            let r923 = c.r64(8) & c.r64(12);
            c.w64(8, r923);
            c.logic(r923, 64);
            let Some(t924) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878b46); };
            c.logic(t924 & 0x2u64, 8);
            if c.cc_e() { blk = 733; continue; }
            blk = 884; continue;
        }
        884 => {
            c.w64(9, c.r64(15).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878b55); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878b56); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000878b63, 0x7000878b5e);
        }
        885 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878b63); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878b65); }
            blk = 733; continue;
        }
        886 => {
            c.w32(1, c.r32(1));
            let r925 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r925);
            c.w64(8, 0xfffffffffffc0000u64);
            let r926 = c.r64(8) & c.r64(1);
            c.w64(8, r926);
            c.logic(r926, 64);
            let Some(t927) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878b7a); };
            c.logic(t927 & 0x2u64, 8);
            if c.cc_e() { blk = 699; continue; }
            blk = 887; continue;
        }
        887 => {
            c.w64(9, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878b89); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878b8a); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000878b97, 0x7000878b92);
        }
        888 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878b97); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878b99); }
            blk = 699; continue;
        }
        889 => {
            c.w32(0, c.r32(0));
            let r928 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r928);
            c.w64(9, 0xfffffffffffc0000u64);
            let r929 = c.r64(9) & c.r64(0);
            c.w64(9, r929);
            c.logic(r929, 64);
            let Some(t930) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878bae); };
            c.logic(t930 & 0x2u64, 8);
            if c.cc_e() { blk = 689; continue; }
            blk = 890; continue;
        }
        890 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878bbd); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000878bc7, 0x7000878bc2);
        }
        891 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878bc7); }
            blk = 689; continue;
        }
        892 => {
            c.w32(7, c.r32(7));
            let r931 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r931);
            c.w64(9, 0xfffffffffffc0000u64);
            let r932 = c.r64(9) & c.r64(7);
            c.w64(9, r932);
            c.logic(r932, 64);
            let Some(t933) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878bdd); };
            c.logic(t933 & 0x2u64, 8);
            if c.cc_e() { blk = 687; continue; }
            blk = 893; continue;
        }
        893 => {
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878bec); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878bed); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000878bfa, 0x7000878bf5);
        }
        894 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878bfa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878bfc); }
            blk = 687; continue;
        }
        895 => {
            c.w32(9, c.r32(9));
            let r934 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r934);
            c.w64(12, 0xfffffffffffc0000u64);
            let r935 = c.r64(12) & c.r64(9);
            c.w64(12, r935);
            c.logic(r935, 64);
            let Some(t936) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878c12); };
            c.logic(t936 & 0x2u64, 8);
            if c.cc_e() { blk = 686; continue; }
            blk = 896; continue;
        }
        896 => {
            c.w64(0, c.r64(15).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878c22); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878c23); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000878c30, 0x7000878c2b);
        }
        897 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878c30); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878c32); }
            blk = 686; continue;
        }
        898 => {
            c.w32(0, c.r32(0));
            let r937 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r937);
            c.w64(3, 0xfffffffffffc0000u64);
            let r938 = c.r64(3) & c.r64(0);
            c.w64(3, r938);
            c.logic(r938, 64);
            let Some(t939) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878c47); };
            c.logic(t939 & 0x2u64, 8);
            if c.cc_e() { blk = 683; continue; }
            blk = 899; continue;
        }
        899 => {
            c.w64(2, c.r64(9).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878c55); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878c56); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000878c63, 0x7000878c5e);
        }
        900 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878c63); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878c65); }
            blk = 683; continue;
        }
        901 => {
            c.w32(1, c.r32(1));
            let r940 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r940);
            c.w64(7, 0xfffffffffffc0000u64);
            let r941 = c.r64(7) & c.r64(1);
            c.w64(7, r941);
            c.logic(r941, 64);
            let Some(t942) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878c7a); };
            c.logic(t942 & 0x2u64, 8);
            if c.cc_e() { blk = 642; continue; }
            blk = 902; continue;
        }
        902 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878c88); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878c89); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000878c96, 0x7000878c91);
        }
        903 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878c96); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878c98); }
            blk = 642; continue;
        }
        904 => {
            c.w32(7, c.r32(7));
            let r943 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r943);
            c.w64(11, 0xfffffffffffc0000u64);
            let r944 = c.r64(11) & c.r64(7);
            c.w64(11, r944);
            c.logic(r944, 64);
            let Some(t945) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878cad); };
            c.logic(t945 & 0x2u64, 8);
            if c.cc_e() { blk = 640; continue; }
            blk = 905; continue;
        }
        905 => {
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878cbc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878cbd); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000878cca, 0x7000878cc5);
        }
        906 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878cca); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878ccc); }
            blk = 640; continue;
        }
        907 => {
            c.w32(7, c.r32(7));
            let r946 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r946);
            c.w64(8, 0xfffffffffffc0000u64);
            let r947 = c.r64(8) & c.r64(7);
            c.w64(8, r947);
            c.logic(r947, 64);
            let Some(t948) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878ce1); };
            c.logic(t948 & 0x2u64, 8);
            if c.cc_e() { blk = 617; continue; }
            blk = 908; continue;
        }
        908 => {
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878cf0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878cf1); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000878cfe, 0x7000878cf9);
        }
        909 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878cfe); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878d00); }
            blk = 617; continue;
        }
        910 => {
            c.w32(8, c.r32(8));
            let r949 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r949);
            c.w64(12, 0xfffffffffffc0000u64);
            let r950 = c.r64(12) & c.r64(8);
            c.w64(12, r950);
            c.logic(r950, 64);
            let Some(t951) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878d16); };
            c.logic(t951 & 0x2u64, 8);
            if c.cc_e() { blk = 616; continue; }
            blk = 911; continue;
        }
        911 => {
            c.w64(0, c.r64(15).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878d26); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878d27); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000878d34, 0x7000878d2f);
        }
        912 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878d34); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878d36); }
            blk = 616; continue;
        }
        913 => {
            c.w32(1, c.r32(1));
            let r952 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r952);
            c.w64(9, 0xfffffffffffc0000u64);
            let r953 = c.r64(9) & c.r64(1);
            c.w64(9, r953);
            c.logic(r953, 64);
            let Some(t954) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878d4b); };
            c.logic(t954 & 0x2u64, 8);
            if c.cc_e() { blk = 581; continue; }
            blk = 914; continue;
        }
        914 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878d5a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878d5b); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000878d68, 0x7000878d63);
        }
        915 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878d68); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878d6a); }
            blk = 581; continue;
        }
        916 => {
            c.w32(7, c.r32(7));
            let r955 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r955);
            c.w64(8, 0xfffffffffffc0000u64);
            let r956 = c.r64(8) & c.r64(7);
            c.w64(8, r956);
            c.logic(r956, 64);
            let Some(t957) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878d7f); };
            c.logic(t957 & 0x2u64, 8);
            if c.cc_e() { blk = 573; continue; }
            blk = 917; continue;
        }
        917 => {
            c.w64(9, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878d8e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878d8f); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000878d9c, 0x7000878d97);
        }
        918 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878d9c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878d9e); }
            blk = 573; continue;
        }
        919 => {
            c.w32(0, c.r32(0));
            let r958 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r958);
            c.w64(11, 0xfffffffffffc0000u64);
            let r959 = c.r64(11) & c.r64(0);
            c.w64(11, r959);
            c.logic(r959, 64);
            let Some(t960) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878db3); };
            c.logic(t960 & 0x2u64, 8);
            if c.cc_e() { blk = 555; continue; }
            blk = 920; continue;
        }
        920 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878dc2); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000878dcc, 0x7000878dc7);
        }
        921 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878dcc); }
            blk = 555; continue;
        }
        922 => {
            c.w32(7, c.r32(7));
            let r961 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r961);
            c.w64(8, 0xfffffffffffc0000u64);
            let r962 = c.r64(8) & c.r64(7);
            c.w64(8, r962);
            c.logic(r962, 64);
            let Some(t963) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878de2); };
            c.logic(t963 & 0x2u64, 8);
            if c.cc_e() { blk = 553; continue; }
            blk = 923; continue;
        }
        923 => {
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878df1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878df2); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000878dff, 0x7000878dfa);
        }
        924 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878dff); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878e01); }
            blk = 553; continue;
        }
        925 => {
            c.w32(8, c.r32(8));
            let r964 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r964);
            c.w64(12, 0xfffffffffffc0000u64);
            let r965 = c.r64(12) & c.r64(8);
            c.w64(12, r965);
            c.logic(r965, 64);
            let Some(t966) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878e17); };
            c.logic(t966 & 0x2u64, 8);
            if c.cc_e() { blk = 552; continue; }
            blk = 926; continue;
        }
        926 => {
            c.w64(0, c.r64(15).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878e27); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878e28); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000878e35, 0x7000878e30);
        }
        927 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878e35); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878e37); }
            blk = 552; continue;
        }
        928 => {
            c.w32(0, c.r32(0));
            let r967 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r967);
            c.w64(3, 0xfffffffffffc0000u64);
            let r968 = c.r64(3) & c.r64(0);
            c.w64(3, r968);
            c.logic(r968, 64);
            let Some(t969) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878e4c); };
            c.logic(t969 & 0x2u64, 8);
            if c.cc_e() { blk = 549; continue; }
            blk = 929; continue;
        }
        929 => {
            c.w64(2, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878e5a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878e5b); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000878e68, 0x7000878e63);
        }
        930 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878e68); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878e6a); }
            blk = 549; continue;
        }
        931 => {
            c.w32(7, c.r32(7));
            let r970 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r970);
            c.w64(8, 0xfffffffffffc0000u64);
            let r971 = c.r64(8) & c.r64(7);
            c.w64(8, r971);
            c.logic(r971, 64);
            let Some(t972) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878e7f); };
            c.logic(t972 & 0x2u64, 8);
            if c.cc_e() { blk = 509; continue; }
            blk = 932; continue;
        }
        932 => {
            c.w64(9, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878e8e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878e8f); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000878e9c, 0x7000878e97);
        }
        933 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878e9c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878e9e); }
            blk = 509; continue;
        }
        934 => {
            c.w32(7, c.r32(7));
            let r973 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r973);
            c.w64(8, 0xfffffffffffc0000u64);
            let r974 = c.r64(8) & c.r64(7);
            c.w64(8, r974);
            c.logic(r974, 64);
            let Some(t975) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878eb3); };
            c.logic(t975 & 0x2u64, 8);
            if c.cc_e() { blk = 495; continue; }
            blk = 935; continue;
        }
        935 => {
            c.w64(9, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878ec2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878ec3); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000878ed0, 0x7000878ecb);
        }
        936 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878ed0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878ed2); }
            blk = 495; continue;
        }
        937 => {
            c.w32(1, c.r32(1));
            let r976 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r976);
            c.w64(7, 0xfffffffffffc0000u64);
            let r977 = c.r64(7) & c.r64(1);
            c.w64(7, r977);
            c.logic(r977, 64);
            let Some(t978) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878ee7); };
            c.logic(t978 & 0x2u64, 8);
            if c.cc_e() { blk = 479; continue; }
            blk = 938; continue;
        }
        938 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878ef5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878ef6); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000878f03, 0x7000878efe);
        }
        939 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878f03); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878f05); }
            blk = 479; continue;
        }
        940 => {
            c.w32(7, c.r32(7));
            let r979 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r979);
            c.w64(11, 0xfffffffffffc0000u64);
            let r980 = c.r64(11) & c.r64(7);
            c.w64(11, r980);
            c.logic(r980, 64);
            let Some(t981) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878f1a); };
            c.logic(t981 & 0x2u64, 8);
            if c.cc_e() { blk = 477; continue; }
            blk = 941; continue;
        }
        941 => {
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878f29); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878f2a); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000878f37, 0x7000878f32);
        }
        942 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878f37); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878f39); }
            blk = 477; continue;
        }
        943 => {
            c.w32(7, c.r32(7));
            let r982 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r982);
            c.w64(8, 0xfffffffffffc0000u64);
            let r983 = c.r64(8) & c.r64(7);
            c.w64(8, r983);
            c.logic(r983, 64);
            let Some(t984) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878f4e); };
            c.logic(t984 & 0x2u64, 8);
            if c.cc_e() { blk = 454; continue; }
            blk = 944; continue;
        }
        944 => {
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878f5d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878f5e); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000878f6b, 0x7000878f66);
        }
        945 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878f6b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878f6d); }
            blk = 454; continue;
        }
        946 => {
            c.w32(8, c.r32(8));
            let r985 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r985);
            c.w64(12, 0xfffffffffffc0000u64);
            let r986 = c.r64(12) & c.r64(8);
            c.w64(12, r986);
            c.logic(r986, 64);
            let Some(t987) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878f83); };
            c.logic(t987 & 0x2u64, 8);
            if c.cc_e() { blk = 453; continue; }
            blk = 947; continue;
        }
        947 => {
            c.w64(0, c.r64(15).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878f93); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878f94); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000878fa1, 0x7000878f9c);
        }
        948 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878fa1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878fa3); }
            blk = 453; continue;
        }
        949 => {
            c.w32(1, c.r32(1));
            let r988 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r988);
            c.w64(9, 0xfffffffffffc0000u64);
            let r989 = c.r64(9) & c.r64(1);
            c.w64(9, r989);
            c.logic(r989, 64);
            let Some(t990) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878fb8); };
            c.logic(t990 & 0x2u64, 8);
            if c.cc_e() { blk = 418; continue; }
            blk = 950; continue;
        }
        950 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878fc7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878fc8); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000878fd5, 0x7000878fd0);
        }
        951 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000878fd5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000878fd7); }
            blk = 418; continue;
        }
        952 => {
            c.w32(7, c.r32(7));
            let r991 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r991);
            c.w64(8, 0xfffffffffffc0000u64);
            let r992 = c.r64(8) & c.r64(7);
            c.w64(8, r992);
            c.logic(r992, 64);
            let Some(t993) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000878fec); };
            c.logic(t993 & 0x2u64, 8);
            if c.cc_e() { blk = 416; continue; }
            blk = 953; continue;
        }
        953 => {
            c.w64(9, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000878ffb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000878ffc); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000879009, 0x7000879004);
        }
        954 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000879009); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087900b); }
            blk = 416; continue;
        }
        955 => {
            c.w32(15, c.r32(15));
            let r994 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r994);
            c.w64(2, 0xfffffffffffc0000u64);
            let r995 = c.r64(2) & c.r64(15);
            c.w64(2, r995);
            c.logic(r995, 64);
            let Some(t996) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879021); };
            c.logic(t996 & 0x2u64, 8);
            if c.cc_e() { blk = 372; continue; }
            blk = 956; continue;
        }
        956 => {
            c.w64(1, c.r64(11).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087902f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879030); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x700087903d, 0x7000879038);
        }
        957 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087903d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087903f); }
            blk = 372; continue;
        }
        958 => {
            c.w32(8, c.r32(8));
            let r997 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r997);
            c.w64(9, 0xfffffffffffc0000u64);
            let r998 = c.r64(9) & c.r64(8);
            c.w64(9, r998);
            c.logic(r998, 64);
            let Some(t999) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879055); };
            c.logic(t999 & 0x2u64, 8);
            if c.cc_e() { blk = 369; continue; }
            blk = 959; continue;
        }
        959 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879064); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879065); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000879072, 0x700087906d);
        }
        960 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000879072); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000879074); }
            blk = 369; continue;
        }
        961 => {
            c.w32(1, c.r32(1));
            let r1000 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1000);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1001 = c.r64(8) & c.r64(1);
            c.w64(8, r1001);
            c.logic(r1001, 64);
            let Some(t1002) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879089); };
            c.logic(t1002 & 0x2u64, 8);
            if c.cc_e() { blk = 329; continue; }
            blk = 962; continue;
        }
        962 => {
            c.w64(9, c.r64(11).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879098); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879099); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008790a6, 0x70008790a1);
        }
        963 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008790a6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008790a8); }
            blk = 329; continue;
        }
        964 => {
            c.w32(9, c.r32(9));
            let r1003 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1003);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1004 = c.r64(7) & c.r64(9);
            c.w64(7, r1004);
            c.logic(r1004, 64);
            let Some(t1005) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008790be); };
            c.logic(t1005 & 0x2u64, 8);
            if c.cc_e() { blk = 296; continue; }
            blk = 965; continue;
        }
        965 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008790cc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008790cd); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008790da, 0x70008790d5);
        }
        966 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008790da); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008790dc); }
            blk = 296; continue;
        }
        967 => {
            c.w32(7, c.r32(7));
            let r1006 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1006);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1007 = c.r64(12) & c.r64(7);
            c.w64(12, r1007);
            c.logic(r1007, 64);
            let Some(t1008) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008790f1); };
            c.logic(t1008 & 0x2u64, 8);
            if c.cc_e() { blk = 294; continue; }
            blk = 968; continue;
        }
        968 => {
            c.w64(15, c.r64(11).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879101); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879102); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x700087910f, 0x700087910a);
        }
        969 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087910f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000879111); }
            blk = 294; continue;
        }
        970 => {
            c.w32(1, c.r32(1));
            let r1009 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1009);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1010 = c.r64(7) & c.r64(1);
            c.w64(7, r1010);
            c.logic(r1010, 64);
            let Some(t1011) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879126); };
            c.logic(t1011 & 0x2u64, 8);
            if c.cc_e() { blk = 287; continue; }
            blk = 971; continue;
        }
        971 => {
            c.w64(9, c.r64(11).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879134); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879135); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000879142, 0x700087913d);
        }
        972 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000879142); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000879144); }
            blk = 287; continue;
        }
        973 => {
            c.w32(7, c.r32(7));
            let r1012 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1012);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1013 = c.r64(9) & c.r64(7);
            c.w64(9, r1013);
            c.logic(r1013, 64);
            let Some(t1014) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879159); };
            c.logic(t1014 & 0x2u64, 8);
            if c.cc_e() { blk = 285; continue; }
            blk = 974; continue;
        }
        974 => {
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879168); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879169); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000879176, 0x7000879171);
        }
        975 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000879176); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000879178); }
            blk = 285; continue;
        }
        976 => {
            c.w32(7, c.r32(7));
            let r1015 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1015);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1016 = c.r64(8) & c.r64(7);
            c.w64(8, r1016);
            c.logic(r1016, 64);
            let Some(t1017) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087918d); };
            c.logic(t1017 & 0x2u64, 8);
            if c.cc_e() { blk = 282; continue; }
            blk = 977; continue;
        }
        977 => {
            c.w64(9, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087919d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087919e); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x70008791ab, 0x70008791a6);
        }
        978 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008791ab); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008791ad); }
            blk = 282; continue;
        }
        979 => {
            c.w32(8, c.r32(8));
            let r1018 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1018);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1019 = c.r64(9) & c.r64(8);
            c.w64(9, r1019);
            c.logic(r1019, 64);
            let Some(t1020) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008791c3); };
            c.logic(t1020 & 0x2u64, 8);
            if c.cc_e() { blk = 281; continue; }
            blk = 980; continue;
        }
        980 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008791d2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008791d3); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008791e0, 0x70008791db);
        }
        981 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008791e0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008791e2); }
            blk = 281; continue;
        }
        982 => {
            c.w32(7, c.r32(7));
            let r1021 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1021);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1022 = c.r64(9) & c.r64(7);
            c.w64(9, r1022);
            c.logic(r1022, 64);
            let Some(t1023) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008791f7); };
            c.logic(t1023 & 0x2u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 983; continue;
        }
        983 => {
            c.w64(11, c.r64(12).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879207); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879208); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000879215, 0x7000879210);
        }
        984 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000879215); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000879217); }
            blk = 278; continue;
        }
        985 => {
            c.w32(7, c.r32(7));
            let r1024 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1024);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1025 = c.r64(8) & c.r64(7);
            c.w64(8, r1025);
            c.logic(r1025, 64);
            let Some(t1026) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087922c); };
            c.logic(t1026 & 0x2u64, 8);
            if c.cc_e() { blk = 235; continue; }
            blk = 986; continue;
        }
        986 => {
            c.w64(9, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087923b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087923c); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000879249, 0x7000879244);
        }
        987 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000879249); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087924b); }
            blk = 235; continue;
        }
        988 => {
            c.w32(1, c.r32(1));
            let r1027 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1027);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1028 = c.r64(8) & c.r64(1);
            c.w64(8, r1028);
            c.logic(r1028, 64);
            let Some(t1029) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879260); };
            c.logic(t1029 & 0x2u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 989; continue;
        }
        989 => {
            c.w64(9, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087926f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000879270); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700087927d, 0x7000879278);
        }
        990 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087927d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087927f); }
            blk = 131; continue;
        }
        991 => {
            c.w32(1, c.r32(1));
            let r1030 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1030);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1031 = c.r64(8) & c.r64(1);
            c.w64(8, r1031);
            c.logic(r1031, 64);
            let Some(t1032) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879294); };
            c.logic(t1032 & 0x2u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 992; continue;
        }
        992 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008792a3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008792a4); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008792b1, 0x70008792ac);
        }
        993 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008792b1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008792b3); }
            blk = 85; continue;
        }
        994 => {
            return RegfileExit::jump(0x70008792bc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008792d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return c.call(0x7000824274, 0x70008792d9, 0x70008792d4);
        }
        1 => {
            return RegfileExit::fault(0x70008792d9);
        }
        2 => {
            return RegfileExit::fault(0x70008792da);
        }
        3 => {
            return RegfileExit::fault(0x70008792db);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008792f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return c.call(0x7000824274, 0x70008792f9, 0x70008792f4);
        }
        1 => {
            return RegfileExit::fault(0x70008792f9);
        }
        2 => {
            return RegfileExit::fault(0x70008792fa);
        }
        3 => {
            return RegfileExit::fault(0x70008792fb);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000879314(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c08)) else { return RegfileExit::fault(0x7000879314); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000879334(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c10)) else { return RegfileExit::fault(0x7000879334); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000879354(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c18)) else { return RegfileExit::fault(0x7000879354); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000879374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000879374); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000879378); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879379); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087937a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700087937f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000879383); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000879397); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700087939f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008793a3); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008793ad); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008793b3); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008793c4); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008793cb); };
            c.cmp(t7, 0x82bu64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008793d8); };
            c.w32(9, t8);
            c.cmp(c.r32(9), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0xfffffffffffffffeu64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x1b), c.r32(9)) { return RegfileExit::fault(0x70008793f3); }
            c.w64(0, c.r64(1));
            c.w64(2, c.r64(7));
            return c.call(0x70007fa8f4, 0x7000879402, 0x70008793fd);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000879402); };
            c.w64(1, t9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000879406); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879409); };
            c.w64(8, t11);
            let r12 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000879411); }
            c.cmp(c.r32(7), 0xfffffffffffffffcu64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087941d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087941e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000879424); }
            return c.ret(0x0, 0x7000879426);
        }
        11 => {
            c.w64(7, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(1).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700087942e); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000879431); };
            c.w64(3, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000879438); };
            c.w64(6, t14);
            return c.call(0x700082faf4, 0x7000879441, 0x700087943c);
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879441); };
            c.w64(1, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000879448); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000879449); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087944f); }
            return c.ret(0x0, 0x7000879451);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000879452); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000879456); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000879467, 0x7000879462);
        }
        14 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000879467); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087946b); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        15 => {
            c.w64(1, 0xfffffffffffffffeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087947b); };
            c.w64(7, t19);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700087947f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000879482); }
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000879488); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087948f); };
            c.w64(6, t21);
            return c.call(0x700086ed74, 0x7000879498, 0x7000879493);
        }
        16 => {
            return RegfileExit::fault(0x7000879498);
        }
        17 => {
            c.w32(1, 0x70u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700087949e); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700087949f); };
            c.w64(3, t22);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008794b0, 0x70008794ab);
        }
        18 => {
            return RegfileExit::fault(0x70008794b0);
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008794b1); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008794b8); };
            c.w64(3, t24);
            return c.call(0x700082faf4, 0x70008794c4, 0x70008794bf);
        }
        20 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008794c4); };
            c.w64(8, t25);
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008794cf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008794d0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008794d6); }
            return c.ret(0x0, 0x70008794d8);
        }
        21 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008794de); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008794df); };
            c.w64(1, t26);
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0x10b)) else { return RegfileExit::fault(0x70008794e6); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008794ef); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008794f0); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008794f1); };
            c.w64(3, t29);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000879502, 0x70008794fd);
        }
        22 => {
            return RegfileExit::fault(0x7000879502);
        }
        23 => {
            return RegfileExit::jump(0x7000879504);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000879514(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c20)) else { return RegfileExit::fault(0x7000879514); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000879534(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c28)) else { return RegfileExit::fault(0x7000879534); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000879554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000879554); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000879558); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879559); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087955a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700087955f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000879563); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000879570); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(8, c.r64(5));
            let r2 = c.add(c.r64(8), 0x18u64, 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879581); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000879587); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(5));
            let r5 = c.add(c.r64(9), 0x18u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld64(c.r64(9).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087959b); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008795a1); };
            c.w64(9, t7);
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008795a8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008795a9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008795aa); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008795ac); }
            c.w32(0, 0x4u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x25e8)) else { return RegfileExit::fault(0x70008795b3); };
            c.w64(3, t8);
            return c.call(0x700086ed74, 0x70008795bf, 0x70008795ba);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008795bf); }
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008795c3); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(2, c.r64(5));
            let Some(t10) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008795cd); };
            c.w64(2, t10);
            blk = 11; continue;
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008795d3); };
            c.w64(2, t11);
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(2));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008795dd); };
            c.w64(6, t12);
            return c.call(0x7000845874, 0x70008795e6, 0x70008795e1);
        }
        12 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008795e6); };
            if !c.push64(t13) { return RegfileExit::fault(0x70008795e6); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x6c8)) else { return RegfileExit::fault(0x70008795e9); };
            c.w64(2, t14);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008795f0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008795f1); }
            c.w32(2, 0x4u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008795f7); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2480)) else { return RegfileExit::fault(0x70008795f8); };
            c.w64(3, t15);
            c.w32(0, 0x4u64);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879604); };
            c.w64(6, t16);
            return c.call(0x700086ed74, 0x700087960d, 0x7000879608);
        }
        13 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087960d); };
            c.w64(2, t17);
            let r18 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000879615); };
            c.w64(0, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087961c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087961d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000879623); }
            return c.ret(0x0, 0x7000879625);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000879626); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700087962a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700087962e); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000879632); };
            c.w64(3, t20);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000879643, 0x700087963e);
        }
        15 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879643); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000879647); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087964b); };
            c.w64(2, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087964f); };
            c.w64(0, t24);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x700087965c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000879674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000879674); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000879678); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879679); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087967a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700087967f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000879683); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087968d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879699); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087969f); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 61; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008796b3); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008796bc); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008796c2); };
            c.cmp(t8, 0x834u64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008796ca); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            blk = 27; continue;
        }
        6 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        7 => {
            c.w32(7, 0x1u64);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008796e3); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008796e9); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008796ef); };
            c.w32(8, t15);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008796f6); };
            c.w32(9, t16);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xb0u64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x7000879706);
        }
        11 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000879707); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087971d); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            let Some(t20) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000879723); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000879729); };
            c.w32(7, t22);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t23) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087972f); };
            c.w32(8, t23);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x44u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x700087973c);
        }
        15 => {
            let Some(t24) = c.ld32(c.r64(1).wrapping_add(0x117)) else { return RegfileExit::fault(0x700087973d); };
            c.cmp(t24, c.r32(8), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(7), 0xaeu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x700087974f);
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(1).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x7000879750); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(7), 0xafu64, 64);
            if c.cc_a() { blk = 64; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x7000879766);
        }
        21 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879767); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a17f4, 0x700087977a, 0x7000879775);
        }
        24 => {
            c.w64(1, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087977d); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000879781); };
            c.w64(6, t29);
            blk = 27; continue;
        }
        25 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ge() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        27 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087978e); };
            c.w64(7, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000879792); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x70008797a4, 0x700087979f);
        }
        29 => {
            c.w64(7, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008797a7); };
            c.w64(0, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008797ab); };
            c.w64(6, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008797af); };
            c.w64(1, t33);
            blk = 30; continue;
        }
        30 => {
            c.w32(8, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008797c8); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 38; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 34; continue;
        }
        33 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ge() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(12, c.r32(1));
            let r34 = c.add(c.r32(8), c.r32(12), 32);
            c.w32(8, r34);
            if c.cc_o() { blk = 67; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087981e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 58; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 42; continue;
        }
        41 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879841); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087984c); }
            blk = 48; continue;
        }
        45 => {
            c.w64(1, c.r64(7));
            let r35 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r35);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 48; continue;
        }
        46 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            let r36 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r36);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087986a); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 48; continue;
        }
        48 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 51; continue;
        }
        50 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_le() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 53; continue;
        }
        52 => {
            c.w32(1, 0x1u64);
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(3, c.r64(7));
            return c.call(0x7000870914, 0x7000879899, 0x7000879894);
        }
        55 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000879899); };
            c.w64(1, t37);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008798a0); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return c.ret(0x10, 0x70008798a7);
        }
        57 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008798aa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008798b1); }
            return c.ret(0x0, 0x70008798b3);
        }
        58 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008798b4); };
            c.w64(0, t38);
            blk = 55; continue;
        }
        59 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008798bf); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x70008798cb, 0x70008798c6);
        }
        60 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008798cb); };
            c.w64(6, t40);
            blk = 1; continue;
        }
        61 => {
            return c.call(0x70008a2054, 0x70008798d9, 0x70008798d4);
        }
        62 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008798d9); };
            c.w64(6, t41);
            blk = 3; continue;
        }
        63 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008798e2); };
            c.w32(1, t42);
            let r43 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r43);
            blk = 22; continue;
        }
        64 => {
            let Some(t44) = c.ld32(c.r64(1).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x70008798ed); };
            c.cmp(t44, c.r32(8), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008798fa); };
            c.w64(3, t45);
            return c.call(0x7000870914, 0x7000879906, 0x7000879901);
        }
        66 => {
            c.w64(1, c.r64(0));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000879909); };
            c.w64(0, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087990d); };
            c.w64(6, t47);
            blk = 22; continue;
        }
        67 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087991c); }
            blk = 70; continue;
        }
        69 => {
            let r48 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r48);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879933); }
            blk = 73; continue;
        }
        72 => {
            c.w64(7, c.r64(1));
            let r49 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r49);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 73; continue;
        }
        73 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000879957); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 79; continue; }
            blk = 74; continue;
        }
        74 => {
            if c.cc_ne() { blk = 79; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(8, c.r64(7));
            let r50 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r50);
            if c.cc_o() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 38; continue;
        }
        79 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000879988); };
            c.w64(7, t51);
            let Some(t52) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087998f); };
            c.w64(8, t52);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000879996); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700087999d); };
            c.cmp(t54, c.r64(9), 64);
            if c.cc_a() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008799ac, 0x70008799a7);
        }
        81 => {
            c.w64(7, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008799af); };
            c.w64(0, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008799b3); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008799b7); };
            c.w64(1, t57);
            blk = 83; continue;
        }
        82 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008799bd); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 83; continue;
        }
        83 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008799c4); };
            c.w64(8, t58);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008799cb); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008799cf); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008799d4); }
            blk = 38; continue;
        }
        84 => {
            return RegfileExit::jump(0x70008799e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008799f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008799f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008799f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008799f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008799fa); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008799ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 51; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000879a13); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000879a27); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(3, 0x1u64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 20; continue;
        }
        3 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xfb8)) else { return RegfileExit::fault(0x7000879a4b); };
            c.w64(15, t3);
            let Some(t4) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879a52); };
            c.cmp(t4, c.r32(12), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xfc0)) else { return RegfileExit::fault(0x7000879a5c); };
            c.w64(15, t5);
            let Some(t6) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879a63); };
            c.cmp(t6, c.r32(12), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879a6d); };
            c.w32(15, t7);
            let r8 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r8);
            let Some(t9) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000879a74); };
            c.cmp(t9, 0x834u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, 0x1u64);
            blk = 9; continue;
        }
        8 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        9 => {
            c.w32(3, 0x1u64);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000879a94); };
            c.w32(2, t10);
            let r11 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r11);
            let r12 = c.r32(2) & 0x3fu64;
            c.w32(2, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_a() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879aa4); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            let Some(t15) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000879aaa); };
            c.w32(2, t15);
            let r16 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000879ab0); };
            c.w32(7, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4cu64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000879abf);
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000879ac0); };
            c.w32(2, t18);
            let Some(t19) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000879ac6); };
            c.cmp(t19, c.r32(2), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000879acc); };
            c.w64(15, t20);
            let Some(t21) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879ad3); };
            c.cmp(t21, c.r32(12), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008276b4, 0x7000879ae1, 0x7000879adc);
        }
        16 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879ae1); };
            c.w64(9, t22);
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000879aec); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000879aed); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000879af3); }
            return c.ret(0x0, 0x7000879af5);
        }
        17 => {
            c.w32(3, 0x1u64);
            blk = 20; continue;
        }
        18 => {
            c.w32(3, 0x1u64);
            blk = 20; continue;
        }
        19 => {
            c.w32(3, 0x1u64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xfb8)) else { return RegfileExit::fault(0x7000879b16); };
            c.w64(11, t23);
            let Some(t24) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879b1d); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xfc0)) else { return RegfileExit::fault(0x7000879b27); };
            c.w64(11, t25);
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879b2e); };
            c.cmp(t26, c.r32(12), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879b38); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            let Some(t29) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000879b3f); };
            c.cmp(t29, 0x834u64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, c.r64(3));
            blk = 26; continue;
        }
        25 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t30) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000879b5a); };
            c.w32(15, t30);
            let r31 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r31);
            let r32 = c.r32(15) & 0x3fu64;
            c.w32(15, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_a() { blk = 50; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t33) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879b71); };
            c.w32(15, t33);
            let r34 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r34);
            let Some(t35) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000879b78); };
            c.w32(15, t35);
            let r36 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r36);
            let Some(t37) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000879b7f); };
            c.w32(2, t37);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x7000879b8f);
        }
        30 => {
            let Some(t38) = c.ld32(c.r64(15).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000879b90); };
            c.w32(15, t38);
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000879b97); };
            c.cmp(t39, c.r32(15), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000879ba1); };
            c.w64(11, t40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879ba8); };
            c.cmp(t41, c.r32(12), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879bb2); };
            c.w32(8, t42);
            let r43 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r43);
            c.w32(11, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879bc2); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 50; continue; }
            blk = 34; continue;
        }
        34 => {
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 50; continue;
        }
        36 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_ne() { blk = 50; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000879bf7); };
            c.w64(8, t44);
            blk = 41; continue;
        }
        40 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000879bfd); };
            c.w64(8, t45);
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879c0a); };
            c.w32(11, t46);
            let r47 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r47);
            let Some(t48) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000879c11); };
            c.cmp(t48, 0x834u64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(12, c.r64(3));
            blk = 45; continue;
        }
        44 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t49) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000879c28); };
            c.w32(12, t49);
            let r50 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            let r51 = c.r32(12) & 0x3fu64;
            c.w32(12, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_a() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t52) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000879c3b); };
            c.cmp(t52, c.r32(15), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008276b4, 0x7000879c49, 0x7000879c44);
        }
        49 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879c49); };
            c.w64(1, t53);
            let r54 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r54);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000879c54); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000879c55); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000879c5b); }
            return c.ret(0x0, 0x7000879c5d);
        }
        50 => {
            let Some(t55) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000879c5e); };
            c.w64(1, t55);
            let Some(t56) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000879c62); };
            c.w64(5, t56);
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000879c66); };
            c.w64(2, t57);
            c.w64(7, c.r64(1));
            let r58 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r58);
            return RegfileExit::jump(0x7000825b14);
        }
        51 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000879c79); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000879c7d); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000879c83); };
            c.w64(3, t59);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000879c92, 0x7000879c8d);
        }
        52 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000879c92); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000879c96); };
            c.w64(6, t61);
            blk = 1; continue;
        }
        53 => {
            return RegfileExit::jump(0x7000879ca0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000879cb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000879cb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000879cb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000879cb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000879cba); }
            let r0 = c.sub(c.r64(4), 0x70u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(6)) { return RegfileExit::fault(0x7000879cbf); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000879cc3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000879cc7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 270; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(8, c.r64(7).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000879cdf); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000879ce4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000879ce8); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 272; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879cf6); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000879cfd); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 272; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000879d16); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879d20); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000879d27); };
            c.cmp(t8, 0x834u64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879d31); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            blk = 27; continue;
        }
        6 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        7 => {
            c.w32(12, 0x1u64);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879d4e); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            let Some(t13) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000879d55); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000879d5d); };
            c.w32(15, t15);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t16) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879d65); };
            c.w32(3, t16);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xb0u64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x7000879d75);
        }
        11 => {
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000879d76); };
            c.cmp(t17, c.r32(3), 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 276; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879d8e); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000879d95); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000879d9c); };
            c.w32(12, t22);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t23) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000879da3); };
            c.w32(15, t23);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x44u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x7000879db0);
        }
        15 => {
            let Some(t24) = c.ld32(c.r64(11).wrapping_add(0x117)) else { return RegfileExit::fault(0x7000879db1); };
            c.cmp(t24, c.r32(15), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(12), 0xaeu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x7000879dc3);
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x7000879dc4); };
            c.cmp(t25, c.r32(15), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(12), 0xafu64, 64);
            if c.cc_a() { blk = 275; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x7000879dda);
        }
        21 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000879ddb); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a17f4, 0x7000879df0, 0x7000879deb);
        }
        24 => {
            c.w64(11, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000879df3); };
            c.w64(9, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000879df7); };
            c.w64(6, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879dfb); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000879dff); };
            c.w64(7, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000879e03); };
            c.w64(0, t32);
            blk = 27; continue;
        }
        25 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ge() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        27 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000879e15); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000879e19); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t33) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000879e23); };
            c.w64(7, t33);
            c.w64(15, c.r64(7));
            blk = 30; continue;
        }
        29 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000879e2c); };
            c.w64(15, t34);
            blk = 30; continue;
        }
        30 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000879e33); };
            c.w64(7, t35);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(0, c.r64(15));
            blk = 34; continue;
        }
        32 => {
            c.w64(0, c.r64(15));
            return c.call(0x7000899a14, 0x7000879e4d, 0x7000879e48);
        }
        33 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000879e4d); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000879e51); };
            c.w64(12, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000879e55); };
            c.w64(6, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000879e59); };
            c.w64(11, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000879e5d); };
            c.w64(7, t40);
            blk = 34; continue;
        }
        34 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.w32(3, c.r32(11));
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(15), 0x0u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000879e89); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x7000879e8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(9)) { return RegfileExit::fault(0x7000879e91); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879e9d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 59; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 39; continue;
        }
        37 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 59; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0x1u64);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879ec7); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879ed2); }
            blk = 45; continue;
        }
        42 => {
            c.w64(2, c.r64(11));
            let r41 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r41);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 45; continue;
        }
        43 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(2, c.r64(0));
            let r42 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r42);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879ef4); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 45; continue;
        }
        45 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 58; continue; }
            blk = 46; continue;
        }
        46 => {
            blk = 48; continue;
        }
        47 => {
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_le() { blk = 58; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879f19); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879f24); }
            blk = 54; continue;
        }
        51 => {
            c.w64(2, c.r64(0));
            let r43 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r43);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 54; continue;
        }
        52 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(2, c.r64(11));
            let r44 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r44);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879f44); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 54; continue;
        }
        54 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 74; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 57; continue;
        }
        56 => {
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_le() { blk = 74; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000879f66); };
            c.w64(0, t45);
            blk = 74; continue;
        }
        58 => {
            c.w64(0, c.r64(11));
            blk = 74; continue;
        }
        59 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 61; continue;
        }
        61 => {
            let r46 = c.add(c.r32(2), c.r32(3), 32);
            c.w32(2, r46);
            if c.cc_o() { blk = 278; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 63; continue;
        }
        63 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879f9c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 74; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 68; continue;
        }
        66 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 74; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w32(2, 0x1u64);
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000879fbe); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 73; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 72; continue;
        }
        71 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_le() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000879fd4); };
            c.w64(0, t47);
            blk = 74; continue;
        }
        73 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 74; continue;
        }
        74 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x7000879fdf); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t48) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000879fe9); };
            c.w64(2, t48);
            blk = 77; continue;
        }
        76 => {
            c.w64(2, c.r64(7));
            blk = 77; continue;
        }
        77 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x7000879fff, 0x7000879ffa);
        }
        79 => {
            c.w64(2, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a002); };
            c.w64(12, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a006); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087a00a); };
            c.w64(11, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087a00e); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087a012); };
            c.w64(3, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087a016); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087a01d); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087a024); };
            c.w64(8, t56);
            blk = 80; continue;
        }
        80 => {
            c.w32(1, c.r32(2));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a038); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 107; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 85; continue;
        }
        83 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 107; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(1, 0x1u64);
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a062); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a06d); }
            blk = 91; continue;
        }
        88 => {
            c.w64(1, c.r64(11));
            let r57 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r57);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 91; continue;
        }
        89 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(1, c.r64(2));
            let r58 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r58);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a08f); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 91; continue;
        }
        91 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 105; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 94; continue;
        }
        93 => {
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_le() { blk = 105; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 95; continue;
        }
        95 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a0b4); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a0bf); }
            blk = 100; continue;
        }
        97 => {
            c.w64(0, c.r64(2));
            let r59 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r59);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 100; continue;
        }
        98 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 102; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(0, c.r64(11));
            let r60 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r60);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a0df); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 100; continue;
        }
        100 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 104; continue; }
            blk = 101; continue;
        }
        101 => {
            blk = 103; continue;
        }
        102 => {
            c.cmp(c.r32(2), c.r32(11), 32);
            if c.cc_le() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700087a0f9); };
            c.w64(0, t61);
            c.w64(2, c.r64(0));
            blk = 106; continue;
        }
        104 => {
            c.w64(0, c.r64(2));
            blk = 106; continue;
        }
        105 => {
            c.w64(0, c.r64(11));
            blk = 106; continue;
        }
        106 => {
            c.w64(2, c.r64(0));
            blk = 122; continue;
        }
        107 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 295; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 295; continue; }
            blk = 109; continue;
        }
        109 => {
            let r62 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r62);
            if c.cc_o() { blk = 295; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 111; continue;
        }
        111 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a138); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 122; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 116; continue;
        }
        114 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ge() { blk = 122; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w32(0, 0x1u64);
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a15a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 121; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 120; continue;
        }
        119 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_le() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700087a170); };
            c.w64(2, t63);
            blk = 122; continue;
        }
        121 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            c.w8(0, (c.cc_be() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x700087a185); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t64) = c.ld64(c.r64(8).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087a18e); };
            c.w64(1, t64);
            blk = 125; continue;
        }
        124 => {
            c.w64(1, c.r64(7));
            blk = 125; continue;
        }
        125 => {
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087a197); };
            c.cmp(t65, c.r32(1), 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(7, c.r64(11));
            blk = 134; continue;
        }
        127 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t66) = c.ld64(c.r64(8).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087a1aa); };
            c.w64(7, t66);
            blk = 129; continue;
        }
        129 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(0, c.r64(7));
            blk = 133; continue;
        }
        131 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x700087a1c1, 0x700087a1bc);
        }
        132 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087a1c1); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a1c8); };
            c.w64(12, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a1cc); };
            c.w64(6, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087a1d0); };
            c.w64(2, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087a1d4); };
            c.w64(11, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087a1d8); };
            c.w64(15, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087a1dc); };
            c.w64(3, t73);
            blk = 133; continue;
        }
        133 => {
            c.w64(7, c.r64(0));
            blk = 134; continue;
        }
        134 => {
            c.w32(8, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 135; continue;
        }
        135 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a1f5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 159; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 139; continue;
        }
        137 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 159; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w32(8, 0x1u64);
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 140; continue;
        }
        140 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a222); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a22d); }
            blk = 145; continue;
        }
        142 => {
            c.w64(8, c.r64(11));
            let r74 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r74);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 145; continue;
        }
        143 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(8, c.r64(7));
            let r75 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r75);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a252); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 145; continue;
        }
        145 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 158; continue; }
            blk = 146; continue;
        }
        146 => {
            blk = 148; continue;
        }
        147 => {
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_le() { blk = 158; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 149; continue;
        }
        149 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a278); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a283); }
            blk = 154; continue;
        }
        151 => {
            c.w64(8, c.r64(7));
            let r76 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r76);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 154; continue;
        }
        152 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 156; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(8, c.r64(11));
            let r77 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r77);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a2a6); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 154; continue;
        }
        154 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 174; continue; }
            blk = 155; continue;
        }
        155 => {
            blk = 157; continue;
        }
        156 => {
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_le() { blk = 174; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700087a2c9); };
            c.w64(7, t78);
            blk = 174; continue;
        }
        158 => {
            c.w64(7, c.r64(11));
            blk = 174; continue;
        }
        159 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 313; continue; }
            blk = 161; continue;
        }
        161 => {
            let r79 = c.add(c.r32(8), c.r32(3), 32);
            c.w32(8, r79);
            if c.cc_o() { blk = 313; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 163; continue;
        }
        163 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 166; continue; }
            blk = 164; continue;
        }
        164 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a302); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 174; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 168; continue;
        }
        166 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ge() { blk = 174; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w32(8, 0x1u64);
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 171; continue; }
            blk = 169; continue;
        }
        169 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a327); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 173; continue; }
            blk = 170; continue;
        }
        170 => {
            blk = 172; continue;
        }
        171 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_le() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700087a33d); };
            c.w64(7, t80);
            blk = 174; continue;
        }
        173 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 174; continue;
        }
        174 => {
            c.w32(8, c.r32(2));
            c.w32(0, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x700087a362); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 331; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 331; continue; }
            blk = 176; continue;
        }
        176 => {
            let r81 = c.sub(c.r32(0), c.r32(8), 32);
            c.w32(0, r81);
            if c.cc_o() { blk = 331; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 178; continue;
        }
        178 => {
            let Some(t82) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a385); };
            c.w32(0, t82);
            let Some(t83) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a388); };
            c.logic(t83 & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t84) = c.ld32(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087a392); };
            c.cmp(t84, 0x0u64, 32);
            if c.cc_ne() { blk = 349; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 349; continue; }
            blk = 180; continue;
        }
        180 => {
            let r85 = c.sub(c.r32(3), c.r32(0), 32);
            c.w32(3, r85);
            if c.cc_o() { blk = 349; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 182; continue;
        }
        182 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 183; continue;
        }
        183 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a3ce); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a3d9); }
            blk = 188; continue;
        }
        185 => {
            c.w64(8, c.r64(11));
            let r86 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r86);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 188; continue;
        }
        186 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 190; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w64(8, c.r64(7));
            let r87 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r87);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a3fe); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 188; continue;
        }
        188 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 202; continue; }
            blk = 189; continue;
        }
        189 => {
            blk = 191; continue;
        }
        190 => {
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_le() { blk = 202; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 192; continue;
        }
        192 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a424); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a42f); }
            blk = 197; continue;
        }
        194 => {
            c.w64(8, c.r64(7));
            let r88 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r88);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 197; continue;
        }
        195 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 199; continue; }
            blk = 196; continue;
        }
        196 => {
            let r89 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r89);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a44f); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 197; continue;
        }
        197 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 201; continue; }
            blk = 198; continue;
        }
        198 => {
            blk = 200; continue;
        }
        199 => {
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_le() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700087a46a); };
            c.w64(11, t90);
            blk = 202; continue;
        }
        201 => {
            c.w64(11, c.r64(7));
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 203; continue;
        }
        203 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a47c); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a486); };
            c.w64(7, t91);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a48a); }
            blk = 208; continue;
        }
        205 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a491); };
            c.w64(7, t92);
            let r93 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r93);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a49f); };
            c.w64(7, t94);
            blk = 208; continue;
        }
        206 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 210; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w64(8, c.r64(2));
            let r95 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a4b0); };
            c.w64(7, t96);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a4b4); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 208; continue;
        }
        208 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 213; continue; }
            blk = 209; continue;
        }
        209 => {
            blk = 230; continue;
        }
        210 => {
            let Some(t97) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a4cd); };
            c.cmp(c.r32(2), t97, 32);
            if c.cc_l() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a4d2); };
            c.w64(7, t98);
            blk = 230; continue;
        }
        212 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a4db); };
            c.w64(7, t99);
            blk = 213; continue;
        }
        213 => {
            c.w32(8, c.r32(11));
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 366; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 366; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700087a502); };
            c.w64(3, t100);
            let r101 = c.add(c.r32(3), c.r32(8), 32);
            c.w32(3, r101);
            if c.cc_o() { blk = 366; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 217; continue;
        }
        217 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 218; continue;
        }
        218 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a522); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a52d); }
            blk = 223; continue;
        }
        220 => {
            let r102 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r102);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 223; continue;
        }
        221 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w64(12, c.r64(7));
            let r103 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r103);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a54c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 223; continue;
        }
        223 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 227; continue; }
            blk = 224; continue;
        }
        224 => {
            blk = 226; continue;
        }
        225 => {
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_l() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 228; continue;
        }
        227 => {
            c.w32(12, 0x1u64);
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 231; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a577); };
            c.w64(12, t104);
            blk = 230; continue;
        }
        230 => {
            c.w32(3, 0x2u64);
            blk = 246; continue;
        }
        231 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700087a5a4); };
            c.w64(3, t105);
            let r106 = c.add(c.r32(3), c.r32(8), 32);
            c.w32(3, r106);
            if c.cc_o() { blk = 384; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 235; continue;
        }
        235 => {
            c.w32(3, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 401; continue; }
            blk = 236; continue;
        }
        236 => {
            let r107 = c.sub(c.r32(3), 0x2u64, 32);
            c.w32(3, r107);
            if c.cc_o() { blk = 401; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 416; continue; }
            blk = 239; continue;
        }
        239 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 416; continue; }
            blk = 240; continue;
        }
        240 => {
            let r108 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r108);
            if c.cc_o() { blk = 416; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 242; continue;
        }
        242 => {
            c.w32(8, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 434; continue; }
            blk = 243; continue;
        }
        243 => {
            let r109 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r109);
            if c.cc_o() { blk = 434; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 245; continue;
        }
        245 => {
            c.w64(3, 0xfffffffffffffffeu64);
            c.w64(8, c.r64(3));
            blk = 246; continue;
        }
        246 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700087a626); }
            blk = 248; continue;
        }
        247 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a62c); };
            c.w64(6, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087a630); };
            c.w64(3, t111);
            blk = 248; continue;
        }
        248 => {
            c.w32(8, c.r32(11));
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700087a643); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700087a647); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 249; continue;
        }
        249 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a651); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 252; continue; }
            blk = 250; continue;
        }
        250 => {
            blk = 269; continue;
        }
        251 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_le() { blk = 269; continue; }
            blk = 252; continue;
        }
        252 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700087a670); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x700087a674); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x700087a678); }
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087a67c); };
            c.w64(0, t112);
            c.w64(3, c.r64(2));
            return c.call(0x7000820354, 0x700087a688, 0x700087a683);
        }
        253 => {
            let Some(t113) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700087a688); };
            c.cmp(t113, c.r32(0), 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087a691); };
            c.w64(0, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a695); };
            c.w64(3, t115);
            c.w32(1, 0x2u64);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a69e); };
            c.w64(6, t116);
            return c.call(0x7000820e54, 0x700087a6a7, 0x700087a6a2);
        }
        255 => {
            blk = 258; continue;
        }
        256 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087a6a9); };
            c.w64(0, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087a6ad); };
            c.w64(3, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a6b1); };
            c.w64(6, t119);
            return c.call(0x7000870914, 0x700087a6ba, 0x700087a6b5);
        }
        257 => {
            c.w64(1, c.r64(0));
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087a6bd); };
            c.w64(0, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a6c1); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a6c5); };
            c.w64(6, t122);
            return c.call(0x7000871cb4, 0x700087a6ce, 0x700087a6c9);
        }
        258 => {
            let Some(t123) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087a6ce); };
            c.w32(7, t123);
            let Some(t124) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087a6d1); };
            c.w32(8, t124);
            let Some(t125) = c.ld8(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087a6d5); };
            c.logic(t125 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t126) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087a6e1); };
            c.logic(t126 & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 449; continue; }
            blk = 259; continue;
        }
        259 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 449; continue; }
            blk = 260; continue;
        }
        260 => {
            let r127 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r127);
            if c.cc_o() { blk = 449; continue; }
            blk = 261; continue;
        }
        261 => {
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 262; continue;
        }
        262 => {
            let Some(t128) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a70d); };
            c.w32(8, t128);
            let Some(t129) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087a711); };
            c.logic(t129 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 467; continue; }
            blk = 263; continue;
        }
        263 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 467; continue; }
            blk = 264; continue;
        }
        264 => {
            let r130 = c.add(c.r32(7), c.r32(8), 32);
            c.w32(7, r130);
            if c.cc_o() { blk = 467; continue; }
            blk = 265; continue;
        }
        265 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 266; continue;
        }
        266 => {
            let Some(t131) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087a73d); };
            c.cmp(t131, 0x0u64, 32);
            if c.cc_e() { blk = 485; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087a747); };
            c.w64(8, t132);
            let r133 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r133);
            if c.cc_o() { blk = 485; continue; }
            blk = 268; continue;
        }
        268 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 247; continue;
        }
        269 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a75d); };
            c.w64(7, t134);
            let r135 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087a765); };
            c.w64(0, t136);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087a76c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087a76d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087a773); }
            return c.ret(0x0, 0x700087a775);
        }
        270 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087a778); };
            c.w64(3, t137);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x700087a787, 0x700087a782);
        }
        271 => {
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a787); };
            c.w64(6, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a78b); };
            c.w64(0, t139);
            blk = 1; continue;
        }
        272 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2054, 0x700087a79c, 0x700087a797);
        }
        273 => {
            c.w64(9, c.r64(0));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a79f); };
            c.w64(6, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087a7a3); };
            c.w64(8, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087a7a7); };
            c.w64(7, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a7ab); };
            c.w64(0, t143);
            blk = 3; continue;
        }
        274 => {
            let Some(t144) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087a7b4); };
            c.w32(11, t144);
            let r145 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r145);
            blk = 22; continue;
        }
        275 => {
            let Some(t146) = c.ld32(c.r64(11).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700087a7c0); };
            c.cmp(t146, c.r32(15), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700087a7cd); };
            c.w64(3, t147);
            c.w64(0, c.r64(9));
            return c.call(0x7000870914, 0x700087a7dc, 0x700087a7d7);
        }
        277 => {
            c.w64(11, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087a7df); };
            c.w64(9, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a7e3); };
            c.w64(6, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087a7e7); };
            c.w64(8, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087a7eb); };
            c.w64(7, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a7ef); };
            c.w64(0, t152);
            blk = 22; continue;
        }
        278 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 280; continue; }
            blk = 279; continue;
        }
        279 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a7fe); }
            blk = 281; continue;
        }
        280 => {
            c.w64(2, c.r64(11));
            let r153 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r153);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 281; continue;
        }
        281 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 283; continue; }
            blk = 282; continue;
        }
        282 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a818); }
            blk = 284; continue;
        }
        283 => {
            let r154 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r154);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 284; continue;
        }
        284 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(0, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700087a839); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 290; continue; }
            blk = 285; continue;
        }
        285 => {
            if c.cc_ne() { blk = 290; continue; }
            blk = 286; continue;
        }
        286 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 288; continue; }
            blk = 287; continue;
        }
        287 => {
            c.movq_rx(2, 0);
            if let Some(sr) = c.shr(c.r64(2), 0x20u64, 64) { c.w64(2, sr); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 290; continue; }
            blk = 288; continue;
        }
        288 => {
            c.w64(2, c.r64(0));
            let r155 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r155);
            if c.cc_o() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 63; continue;
        }
        290 => {
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087a868); };
            c.w64(0, t156);
            let Some(t157) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700087a86f); };
            c.w64(2, t157);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700087a876); }
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087a87a); };
            c.w64(2, t158);
            let Some(t159) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700087a881); };
            c.cmp(t159, c.r64(1), 64);
            if c.cc_a() { blk = 293; continue; }
            blk = 291; continue;
        }
        291 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087a890, 0x700087a88b);
        }
        292 => {
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087a890); };
            c.w64(8, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a894); };
            c.w64(12, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a898); };
            c.w64(6, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087a89c); };
            c.w64(11, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087a8a0); };
            c.w64(15, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087a8a4); };
            c.w64(3, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087a8a8); };
            c.w64(9, t166);
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087a8af); };
            c.w64(7, t167);
            blk = 294; continue;
        }
        293 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700087a8b8); }
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087a8bb); };
            c.w64(0, t168);
            let r169 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r169);
            blk = 294; continue;
        }
        294 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087a8c3); };
            c.w64(2, t170);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700087a8ca); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700087a8cd); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087a8d2); }
            blk = 63; continue;
        }
        295 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 297; continue; }
            blk = 296; continue;
        }
        296 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a8e2); }
            blk = 298; continue;
        }
        297 => {
            c.w64(1, c.r64(11));
            let r171 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r171);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 298; continue;
        }
        298 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 300; continue; }
            blk = 299; continue;
        }
        299 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a8fc); }
            blk = 301; continue;
        }
        300 => {
            let r172 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r172);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 301; continue;
        }
        301 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(0, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700087a91d); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 307; continue; }
            blk = 302; continue;
        }
        302 => {
            if c.cc_ne() { blk = 307; continue; }
            blk = 303; continue;
        }
        303 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 305; continue; }
            blk = 304; continue;
        }
        304 => {
            c.movq_rx(2, 0);
            if let Some(sr) = c.shr(c.r64(2), 0x20u64, 64) { c.w64(2, sr); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 307; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w64(2, c.r64(0));
            let r173 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r173);
            if c.cc_o() { blk = 307; continue; }
            blk = 306; continue;
        }
        306 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(2));
            blk = 312; continue;
        }
        307 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087a94c); };
            c.w64(0, t174);
            let Some(t175) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700087a953); };
            c.w64(2, t175);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700087a95a); }
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087a95e); };
            c.w64(2, t176);
            let Some(t177) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700087a965); };
            c.cmp(t177, c.r64(1), 64);
            if c.cc_a() { blk = 310; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087a974, 0x700087a96f);
        }
        309 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087a974); };
            c.w64(8, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087a978); };
            c.w64(12, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087a97c); };
            c.w64(6, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087a980); };
            c.w64(11, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087a984); };
            c.w64(15, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087a988); };
            c.w64(3, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087a98c); };
            c.w64(9, t184);
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087a993); };
            c.w64(7, t185);
            blk = 311; continue;
        }
        310 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700087a99c); }
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087a99f); };
            c.w64(0, t186);
            let r187 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r187);
            blk = 311; continue;
        }
        311 => {
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087a9a7); };
            c.w64(2, t188);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700087a9ae); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700087a9b1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087a9b6); }
            blk = 312; continue;
        }
        312 => {
            c.w64(2, c.r64(0));
            blk = 111; continue;
        }
        313 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 315; continue; }
            blk = 314; continue;
        }
        314 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a9c9); }
            blk = 316; continue;
        }
        315 => {
            c.w64(8, c.r64(11));
            let r189 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r189);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 316; continue;
        }
        316 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 318; continue; }
            blk = 317; continue;
        }
        317 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087a9e5); }
            blk = 319; continue;
        }
        318 => {
            let r190 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r190);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 319; continue;
        }
        319 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700087aa06); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 320; continue;
        }
        320 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 321; continue;
        }
        321 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 323; continue; }
            blk = 322; continue;
        }
        322 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 325; continue; }
            blk = 323; continue;
        }
        323 => {
            c.w64(8, c.r64(7));
            let r191 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r191);
            if c.cc_o() { blk = 325; continue; }
            blk = 324; continue;
        }
        324 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 330; continue;
        }
        325 => {
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087aa37); };
            c.w64(7, t192);
            let Some(t193) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087aa3e); };
            c.w64(8, t193);
            c.w64(0, c.r64(8).wrapping_add(0xc));
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087aa45); };
            c.w64(1, t194);
            let Some(t195) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700087aa4c); };
            c.cmp(t195, c.r64(0), 64);
            if c.cc_a() { blk = 328; continue; }
            blk = 326; continue;
        }
        326 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700087aa5e, 0x700087aa59);
        }
        327 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087aa5e); };
            c.w64(9, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087aa65); };
            c.w64(12, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087aa69); };
            c.w64(6, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087aa6d); };
            c.w64(2, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087aa71); };
            c.w64(11, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087aa75); };
            c.w64(15, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087aa79); };
            c.w64(3, t202);
            blk = 329; continue;
        }
        328 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x700087aa7f); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 329; continue;
        }
        329 => {
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087aa89); };
            c.w64(7, t203);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087aa90); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700087aa93); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087aa98); }
            blk = 330; continue;
        }
        330 => {
            c.w64(7, c.r64(0));
            blk = 163; continue;
        }
        331 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x700087aaa5); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087aaaf); }
            blk = 334; continue;
        }
        333 => {
            c.w64(0, c.r64(2));
            let r204 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r204);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 334; continue;
        }
        334 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 336; continue; }
            blk = 335; continue;
        }
        335 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087aac8); }
            blk = 337; continue;
        }
        336 => {
            let r205 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r205);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 337; continue;
        }
        337 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(7, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x700087aae9); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 343; continue; }
            blk = 338; continue;
        }
        338 => {
            if c.cc_ne() { blk = 343; continue; }
            blk = 339; continue;
        }
        339 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 341; continue; }
            blk = 340; continue;
        }
        340 => {
            c.movq_rx(0, 1);
            if let Some(sr) = c.shr(c.r64(0), 0x20u64, 64) { c.w64(0, sr); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 343; continue; }
            blk = 341; continue;
        }
        341 => {
            c.w64(0, c.r64(7));
            let r206 = c.add(c.r32(0), c.r32(7), 32);
            c.w32(0, r206);
            if c.cc_o() { blk = 343; continue; }
            blk = 342; continue;
        }
        342 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 348; continue;
        }
        343 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ab18); };
            c.w64(7, t207);
            let Some(t208) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087ab1f); };
            c.w64(0, t208);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700087ab26); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ab2a); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700087ab31); };
            c.cmp(t210, c.r64(1), 64);
            if c.cc_a() { blk = 346; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700087ab43, 0x700087ab3e);
        }
        345 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087ab43); };
            c.w64(3, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ab47); };
            c.w64(12, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087ab4b); };
            c.w64(6, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087ab4f); };
            c.w64(2, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087ab53); };
            c.w64(9, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700087ab57); };
            c.w64(8, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087ab5b); };
            c.w64(11, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ab5f); };
            c.w64(15, t218);
            blk = 347; continue;
        }
        346 => {
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x700087ab65); }
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ab68); };
            c.w64(7, t219);
            let r220 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r220);
            c.w64(0, c.r64(7));
            blk = 347; continue;
        }
        347 => {
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087ab73); };
            c.w64(7, t221);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087ab7a); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700087ab7d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087ab82); }
            blk = 348; continue;
        }
        348 => {
            c.w64(7, c.r64(0));
            blk = 178; continue;
        }
        349 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700087ab8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700087ab93); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x700087ab97); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700087ab9b); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 351; continue; }
            blk = 350; continue;
        }
        350 => {
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087aba4); };
            c.w64(3, t222);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700087aba8); }
            blk = 352; continue;
        }
        351 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087abaf); };
            c.w64(3, t223);
            let r224 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r224);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 352; continue;
        }
        352 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 354; continue; }
            blk = 353; continue;
        }
        353 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087abc3); }
            blk = 355; continue;
        }
        354 => {
            let r225 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r225);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 355; continue;
        }
        355 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(11, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 1) { return RegfileExit::fault(0x700087abe9); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 361; continue; }
            blk = 356; continue;
        }
        356 => {
            if c.cc_ne() { blk = 361; continue; }
            blk = 357; continue;
        }
        357 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 359; continue; }
            blk = 358; continue;
        }
        358 => {
            c.movq_rx(15, 1);
            if let Some(sr) = c.shr(c.r64(15), 0x20u64, 64) { c.w64(15, sr); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_l() { blk = 361; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(15, c.r64(11));
            let r226 = c.add(c.r32(15), c.r32(11), 32);
            c.w32(15, r226);
            if c.cc_o() { blk = 361; continue; }
            blk = 360; continue;
        }
        360 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            blk = 182; continue;
        }
        361 => {
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ac1b); };
            c.w64(11, t227);
            let Some(t228) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700087ac22); };
            c.w64(15, t228);
            c.w64(3, c.r64(15).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700087ac29); }
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ac2d); };
            c.w64(15, t229);
            let Some(t230) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700087ac34); };
            c.cmp(t230, c.r64(3), 64);
            if c.cc_a() { blk = 364; continue; }
            blk = 362; continue;
        }
        362 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700087ac46, 0x700087ac41);
        }
        363 => {
            c.w64(11, c.r64(0));
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ac49); };
            c.w64(12, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087ac4d); };
            c.w64(6, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087ac51); };
            c.w64(2, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ac55); };
            c.w64(1, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087ac59); };
            c.w64(0, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087ac5d); };
            c.w64(9, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700087ac61); };
            c.w64(8, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ac65); };
            c.w64(7, t238);
            blk = 365; continue;
        }
        364 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700087ac6b); }
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ac6e); };
            c.w64(11, t239);
            let r240 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r240);
            blk = 365; continue;
        }
        365 => {
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087ac76); };
            c.w64(15, t241);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700087ac7d); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700087ac81); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087ac86); }
            blk = 182; continue;
        }
        366 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700087ac91); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700087ac95); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700087ac99); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700087ac9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700087aca1); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x700087aca5); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 368; continue; }
            blk = 367; continue;
        }
        367 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087acaf); }
            blk = 369; continue;
        }
        368 => {
            c.w64(3, c.r64(11));
            let r242 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r242);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 369; continue;
        }
        369 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 371; continue; }
            blk = 370; continue;
        }
        370 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087acca); }
            blk = 372; continue;
        }
        371 => {
            c.w64(3, c.r64(2));
            let r243 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r243);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 372; continue;
        }
        372 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x700087acee); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 378; continue; }
            blk = 373; continue;
        }
        373 => {
            if c.cc_ne() { blk = 378; continue; }
            blk = 374; continue;
        }
        374 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 376; continue; }
            blk = 375; continue;
        }
        375 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 378; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(7, c.r64(3));
            let r244 = c.add(c.r32(7), c.r32(3), 32);
            c.w32(7, r244);
            if c.cc_o() { blk = 378; continue; }
            blk = 377; continue;
        }
        377 => {
            c.w64(3, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(3));
            blk = 383; continue;
        }
        378 => {
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ad20); };
            c.w64(7, t245);
            let Some(t246) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087ad27); };
            c.w64(3, t246);
            c.w64(7, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(3)) { return RegfileExit::fault(0x700087ad2e); }
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ad35); };
            c.w64(3, t247);
            let Some(t248) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700087ad3c); };
            c.cmp(t248, c.r64(7), 64);
            if c.cc_a() { blk = 381; continue; }
            blk = 379; continue;
        }
        379 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700087ad4e, 0x700087ad49);
        }
        380 => {
            c.w64(7, c.r64(0));
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ad51); };
            c.w64(12, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087ad55); };
            c.w64(6, t250);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087ad59); };
            c.w64(11, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087ad5d); };
            c.w64(2, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ad61); };
            c.w64(1, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ad65); };
            c.w64(15, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087ad69); };
            c.w64(8, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087ad6d); };
            c.w64(0, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087ad71); };
            c.w64(9, t257);
            blk = 382; continue;
        }
        381 => {
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ad77); };
            c.w64(3, t258);
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x700087ad7e); }
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087ad81); };
            c.w64(7, t259);
            let r260 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r260);
            blk = 382; continue;
        }
        382 => {
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087ad8c); };
            c.w64(3, t261);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700087ad93); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x700087ad96); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087ad9b); }
            blk = 383; continue;
        }
        383 => {
            c.w64(3, c.r64(7));
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087ada3); };
            c.w64(7, t262);
            blk = 217; continue;
        }
        384 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700087adac); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700087adb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700087adb4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700087adb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700087adbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700087adc0); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 386; continue; }
            blk = 385; continue;
        }
        385 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087adca); }
            blk = 387; continue;
        }
        386 => {
            c.w64(3, c.r64(11));
            let r263 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r263);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 387; continue;
        }
        387 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 389; continue; }
            blk = 388; continue;
        }
        388 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087ade5); }
            blk = 390; continue;
        }
        389 => {
            let r264 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r264);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 390; continue;
        }
        390 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700087ae08); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 396; continue; }
            blk = 391; continue;
        }
        391 => {
            if c.cc_ne() { blk = 396; continue; }
            blk = 392; continue;
        }
        392 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 394; continue; }
            blk = 393; continue;
        }
        393 => {
            c.movq_rx(3, 0);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 396; continue; }
            blk = 394; continue;
        }
        394 => {
            c.w64(3, c.r64(9));
            let r265 = c.add(c.r32(3), c.r32(9), 32);
            c.w32(3, r265);
            if c.cc_o() { blk = 396; continue; }
            blk = 395; continue;
        }
        395 => {
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 235; continue;
        }
        396 => {
            let Some(t266) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ae39); };
            c.w64(9, t266);
            let Some(t267) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700087ae40); };
            c.w64(3, t267);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x700087ae47); }
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ae4b); };
            c.w64(3, t268);
            let Some(t269) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700087ae52); };
            c.cmp(t269, c.r64(2), 64);
            if c.cc_a() { blk = 399; continue; }
            blk = 397; continue;
        }
        397 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087ae61, 0x700087ae5c);
        }
        398 => {
            blk = 400; continue;
        }
        399 => {
            if !c.st64(c.r64(9), c.r64(2)) { return RegfileExit::fault(0x700087ae63); }
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087ae66); };
            c.w64(9, t270);
            let r271 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r271);
            c.w64(0, c.r64(9));
            blk = 400; continue;
        }
        400 => {
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087ae71); };
            c.w64(7, t272);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087ae78); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700087ae7b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087ae80); }
            c.w64(9, c.r64(0));
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087ae88); };
            c.w64(6, t273);
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087ae8c); };
            c.w64(11, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087ae90); };
            c.w64(7, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ae94); };
            c.w64(1, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ae98); };
            c.w64(15, t277);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087ae9c); };
            c.w64(8, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087aea0); };
            c.w64(0, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087aea4); };
            c.w64(12, t280);
            blk = 235; continue;
        }
        401 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700087aead); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700087aeb1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700087aeb5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700087aeb9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700087aebd); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700087aec1); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 403; continue; }
            blk = 402; continue;
        }
        402 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700087aeca); }
            blk = 404; continue;
        }
        403 => {
            let r281 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r281);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 404; continue;
        }
        404 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700087aefe); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 410; continue; }
            blk = 405; continue;
        }
        405 => {
            if c.cc_ne() { blk = 410; continue; }
            blk = 406; continue;
        }
        406 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 408; continue; }
            blk = 407; continue;
        }
        407 => {
            c.movq_rx(3, 0);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 410; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w64(3, c.r64(9));
            let r282 = c.add(c.r32(3), c.r32(9), 32);
            c.w32(3, r282);
            if c.cc_o() { blk = 410; continue; }
            blk = 409; continue;
        }
        409 => {
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(2));
            blk = 415; continue;
        }
        410 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087af2f); };
            c.w64(9, t283);
            let Some(t284) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700087af36); };
            c.w64(3, t284);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x700087af3d); }
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087af41); };
            c.w64(3, t285);
            let Some(t286) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700087af48); };
            c.cmp(t286, c.r64(2), 64);
            if c.cc_a() { blk = 413; continue; }
            blk = 411; continue;
        }
        411 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087af57, 0x700087af52);
        }
        412 => {
            c.w64(9, c.r64(0));
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087af5a); };
            c.w64(6, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087af5e); };
            c.w64(11, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087af62); };
            c.w64(7, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087af66); };
            c.w64(1, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087af6a); };
            c.w64(15, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087af6e); };
            c.w64(8, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087af72); };
            c.w64(0, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087af76); };
            c.w64(12, t294);
            blk = 414; continue;
        }
        413 => {
            if !c.st64(c.r64(9), c.r64(2)) { return RegfileExit::fault(0x700087af7c); }
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087af7f); };
            c.w64(9, t295);
            let r296 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r296);
            blk = 414; continue;
        }
        414 => {
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087af87); };
            c.w64(3, t297);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700087af8e); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700087af92); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087af97); }
            blk = 415; continue;
        }
        415 => {
            c.w64(2, c.r64(9));
            blk = 238; continue;
        }
        416 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700087afa5); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x700087afa9); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 418; continue; }
            blk = 417; continue;
        }
        417 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087afb3); }
            blk = 419; continue;
        }
        418 => {
            c.w64(8, c.r64(11));
            let r298 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r298);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 419; continue;
        }
        419 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 421; continue; }
            blk = 420; continue;
        }
        420 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087afcf); }
            blk = 422; continue;
        }
        421 => {
            let r299 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r299);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 422; continue;
        }
        422 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700087aff0); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 428; continue; }
            blk = 423; continue;
        }
        423 => {
            if c.cc_ne() { blk = 428; continue; }
            blk = 424; continue;
        }
        424 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 426; continue; }
            blk = 425; continue;
        }
        425 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 428; continue; }
            blk = 426; continue;
        }
        426 => {
            c.w64(8, c.r64(7));
            let r300 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r300);
            if c.cc_o() { blk = 428; continue; }
            blk = 427; continue;
        }
        427 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 433; continue;
        }
        428 => {
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087b021); };
            c.w64(7, t301);
            let Some(t302) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087b028); };
            c.w64(8, t302);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087b02f); };
            c.w64(12, t303);
            let Some(t304) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087b036); };
            c.cmp(t304, c.r64(9), 64);
            if c.cc_a() { blk = 431; continue; }
            blk = 429; continue;
        }
        429 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700087b049, 0x700087b044);
        }
        430 => {
            blk = 432; continue;
        }
        431 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700087b04b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 432; continue;
        }
        432 => {
            let Some(t305) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b055); };
            c.w64(7, t305);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087b05c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700087b05f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087b064); }
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087b069); };
            c.w64(2, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087b06d); };
            c.w64(6, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087b071); };
            c.w64(11, t308);
            blk = 433; continue;
        }
        433 => {
            c.w64(7, c.r64(0));
            blk = 242; continue;
        }
        434 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700087b07d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x700087b081); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 436; continue; }
            blk = 435; continue;
        }
        435 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b08b); }
            blk = 437; continue;
        }
        436 => {
            let r309 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r309);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 437; continue;
        }
        437 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700087b0ba); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 443; continue; }
            blk = 438; continue;
        }
        438 => {
            if c.cc_ne() { blk = 443; continue; }
            blk = 439; continue;
        }
        439 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 441; continue; }
            blk = 440; continue;
        }
        440 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 443; continue; }
            blk = 441; continue;
        }
        441 => {
            c.w64(8, c.r64(7));
            let r310 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r310);
            if c.cc_o() { blk = 443; continue; }
            blk = 442; continue;
        }
        442 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 448; continue;
        }
        443 => {
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087b0eb); };
            c.w64(7, t311);
            let Some(t312) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087b0f2); };
            c.w64(8, t312);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087b0f9); };
            c.w64(12, t313);
            let Some(t314) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087b100); };
            c.cmp(t314, c.r64(9), 64);
            if c.cc_a() { blk = 446; continue; }
            blk = 444; continue;
        }
        444 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700087b113, 0x700087b10e);
        }
        445 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087b113); };
            c.w64(2, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087b117); };
            c.w64(6, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087b11b); };
            c.w64(11, t317);
            blk = 447; continue;
        }
        446 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700087b121); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 447; continue;
        }
        447 => {
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b12b); };
            c.w64(7, t318);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087b132); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700087b135); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087b13a); }
            blk = 448; continue;
        }
        448 => {
            c.w64(7, c.r64(0));
            blk = 245; continue;
        }
        449 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700087b147); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700087b14b); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 451; continue; }
            blk = 450; continue;
        }
        450 => {
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087b155); };
            c.w64(8, t319);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b159); }
            blk = 452; continue;
        }
        451 => {
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087b161); };
            c.w64(8, t320);
            let r321 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r321);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 452; continue;
        }
        452 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 454; continue; }
            blk = 453; continue;
        }
        453 => {
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087b177); };
            c.w64(2, t322);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b17b); }
            blk = 455; continue;
        }
        454 => {
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087b182); };
            c.w64(2, t323);
            let r324 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r324);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 455; continue;
        }
        455 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700087b1a2); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 461; continue; }
            blk = 456; continue;
        }
        456 => {
            if c.cc_ne() { blk = 461; continue; }
            blk = 457; continue;
        }
        457 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 459; continue; }
            blk = 458; continue;
        }
        458 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 461; continue; }
            blk = 459; continue;
        }
        459 => {
            c.w64(9, c.r64(8));
            let r325 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r325);
            if c.cc_o() { blk = 461; continue; }
            blk = 460; continue;
        }
        460 => {
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(2));
            blk = 466; continue;
        }
        461 => {
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087b1d4); };
            c.w64(8, t326);
            let Some(t327) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087b1db); };
            c.w64(9, t327);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087b1e2); };
            c.w64(15, t328);
            let Some(t329) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700087b1e9); };
            c.cmp(t329, c.r64(12), 64);
            if c.cc_a() { blk = 464; continue; }
            blk = 462; continue;
        }
        462 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087b1f8, 0x700087b1f3);
        }
        463 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087b1f8); };
            c.w64(7, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087b1fc); };
            c.w64(11, t331);
            blk = 465; continue;
        }
        464 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700087b202); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 465; continue;
        }
        465 => {
            let Some(t332) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b20c); };
            c.w64(8, t332);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087b213); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700087b217); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087b21c); }
            blk = 466; continue;
        }
        466 => {
            c.w64(2, c.r64(0));
            blk = 262; continue;
        }
        467 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x700087b229); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 469; continue; }
            blk = 468; continue;
        }
        468 => {
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087b233); };
            c.w64(8, t333);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b237); }
            blk = 470; continue;
        }
        469 => {
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087b23f); };
            c.w64(8, t334);
            let r335 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r335);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 470; continue;
        }
        470 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 472; continue; }
            blk = 471; continue;
        }
        471 => {
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087b255); };
            c.w64(7, t336);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b259); }
            blk = 473; continue;
        }
        472 => {
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087b260); };
            c.w64(7, t337);
            let r338 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r338);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 473; continue;
        }
        473 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700087b27e); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 479; continue; }
            blk = 474; continue;
        }
        474 => {
            if c.cc_ne() { blk = 479; continue; }
            blk = 475; continue;
        }
        475 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 477; continue; }
            blk = 476; continue;
        }
        476 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 479; continue; }
            blk = 477; continue;
        }
        477 => {
            c.w64(8, c.r64(7));
            let r339 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r339);
            if c.cc_o() { blk = 479; continue; }
            blk = 478; continue;
        }
        478 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 484; continue;
        }
        479 => {
            let Some(t340) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087b2af); };
            c.w64(7, t340);
            let Some(t341) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087b2b6); };
            c.w64(8, t341);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087b2bd); };
            c.w64(11, t342);
            let Some(t343) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700087b2c4); };
            c.cmp(t343, c.r64(9), 64);
            if c.cc_a() { blk = 482; continue; }
            blk = 480; continue;
        }
        480 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700087b2d6, 0x700087b2d1);
        }
        481 => {
            blk = 483; continue;
        }
        482 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700087b2d8); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 483; continue;
        }
        483 => {
            let Some(t344) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b2e2); };
            c.w64(7, t344);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700087b2e9); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700087b2ec); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087b2f1); }
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087b2f6); };
            c.w64(2, t345);
            blk = 484; continue;
        }
        484 => {
            c.w64(7, c.r64(0));
            blk = 266; continue;
        }
        485 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x700087b302); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x700087b306); }
            let Some(t346) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087b30a); };
            c.cmp(t346, 0x0u64, 32);
            if c.cc_ne() { blk = 487; continue; }
            blk = 486; continue;
        }
        486 => {
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087b310); };
            c.w64(11, t347);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b314); }
            blk = 488; continue;
        }
        487 => {
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087b31c); };
            c.w64(11, t348);
            let r349 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r349);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 488; continue;
        }
        488 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700087b34c); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 494; continue; }
            blk = 489; continue;
        }
        489 => {
            if c.cc_ne() { blk = 494; continue; }
            blk = 490; continue;
        }
        490 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 492; continue; }
            blk = 491; continue;
        }
        491 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 494; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(9, c.r64(8));
            let r350 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r350);
            if c.cc_o() { blk = 494; continue; }
            blk = 493; continue;
        }
        493 => {
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 499; continue;
        }
        494 => {
            let Some(t351) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087b37e); };
            c.w64(8, t351);
            let Some(t352) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087b385); };
            c.w64(9, t352);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087b38c); };
            c.w64(12, t353);
            let Some(t354) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087b393); };
            c.cmp(t354, c.r64(11), 64);
            if c.cc_a() { blk = 497; continue; }
            blk = 495; continue;
        }
        495 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700087b3a6, 0x700087b3a1);
        }
        496 => {
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087b3a6); };
            c.w64(2, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087b3aa); };
            c.w64(7, t356);
            blk = 498; continue;
        }
        497 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087b3b0); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 498; continue;
        }
        498 => {
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b3ba); };
            c.w64(8, t357);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087b3c1); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700087b3c5); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087b3ca); }
            blk = 499; continue;
        }
        499 => {
            c.w64(11, c.r64(0));
            blk = 247; continue;
        }
        500 => {
            return RegfileExit::jump(0x700087b3d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087b3f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087b3f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087b3f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087b3f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087b3fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087b3ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087b409); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b417); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087b41e); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087b437); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b444); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087b44b); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087b452); };
            c.w64(8, t11);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b45c); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b460); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087b469); };
            c.w64(9, t14);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b473); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b477); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087b480); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700087b482); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087b484); };
            c.w64(2, t17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700087b48b); }
            c.w64(0, c.r64(7));
            let Some(t18) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087b48f); };
            c.w64(1, t18);
            return c.call(0x700087b774, 0x700087b498, 0x700087b493);
        }
        10 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087b498); };
            c.w64(1, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087b49f); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700087b4a6);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087b4a9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087b4b0); }
            return c.ret(0x0, 0x700087b4b2);
        }
        13 => {
            return RegfileExit::fault(0x700087b4b3);
        }
        14 => {
            return RegfileExit::fault(0x700087b4b4);
        }
        15 => {
            return RegfileExit::fault(0x700087b4b5);
        }
        16 => {
            return RegfileExit::fault(0x700087b4b6);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087b4b7); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087b4bb); };
            c.w64(3, t20);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087b4c9, 0x700087b4c4);
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087b4c9); };
            c.w64(6, t21);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x700087b4d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087b4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087b4f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087b4f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087b4f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087b4fa); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087b4ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087b509); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b517); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087b51e); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 31; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087b537); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b544); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087b54b); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087b556); };
            c.w64(8, t11);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(11, c.r32(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b57b); };
            c.w32(15, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b57f); };
            c.cmp(t13, c.r32(15), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087b58c); };
            c.w64(15, t14);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b596); };
            c.w32(0, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b59a); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700087b5a7); };
            c.w64(0, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087b5ab); };
            c.cmp(t18, c.r32(0), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087b5ba); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b5c3); };
            c.w32(2, t20);
            let r21 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r21);
            let Some(t22) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087b5c9); };
            c.logic(t22 & 0x10u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b5cf); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t24) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087b5dc); };
            c.cmp(t24, 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 17; continue;
        }
        16 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, 0x1u64);
            blk = 19; continue;
        }
        18 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 21; continue;
        }
        21 => {
            let r25 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r25);
            if c.cc_o() { blk = 38; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087b613); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700087b615); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087b617); };
            c.w64(2, t26);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700087b61e); }
            c.w64(0, c.r64(7));
            let Some(t27) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087b622); };
            c.w64(1, t27);
            return c.call(0x700087b774, 0x700087b62b, 0x700087b626);
        }
        24 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087b62b); };
            c.w64(1, t28);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087b632); }
            c.cmp(c.r64(1), 0x5u64, 64);
            if c.cc_g() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return c.ret(0x30, 0x700087b639);
        }
        26 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087b63c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087b643); }
            return c.ret(0x0, 0x700087b645);
        }
        27 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087b646); };
            c.w64(0, t29);
            blk = 24; continue;
        }
        28 => {
            return RegfileExit::fault(0x700087b64f);
        }
        29 => {
            return RegfileExit::fault(0x700087b650);
        }
        30 => {
            return RegfileExit::fault(0x700087b651);
        }
        31 => {
            return RegfileExit::fault(0x700087b652);
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087b653); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087b659); };
            c.w64(3, t30);
            return c.call(0x700086ed74, 0x700087b665, 0x700087b660);
        }
        33 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087b665); };
            c.w64(6, t31);
            blk = 1; continue;
        }
        34 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087b66e); };
            c.logic(t32 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 17; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b680); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 17; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 18; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087b698); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b6a2); }
            blk = 41; continue;
        }
        40 => {
            let r33 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r33);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 41; continue;
        }
        41 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700087b6d6); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 47; continue; }
            blk = 42; continue;
        }
        42 => {
            if c.cc_ne() { blk = 47; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(9, c.r64(8));
            let r34 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r34);
            if c.cc_o() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 52; continue;
        }
        47 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087b708); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087b70f); };
            c.w64(9, t36);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087b716); };
            c.w64(12, t37);
            let Some(t38) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087b71d); };
            c.cmp(t38, c.r64(11), 64);
            if c.cc_a() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087b72d, 0x700087b728);
        }
        49 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087b72d); };
            c.w64(6, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087b731); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087b735); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087b739); };
            c.w64(15, t42);
            blk = 51; continue;
        }
        50 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087b73f); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 51; continue;
        }
        51 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087b749); };
            c.w64(8, t43);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087b750); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700087b754); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087b759); }
            blk = 52; continue;
        }
        52 => {
            c.w64(8, c.r64(0));
            blk = 23; continue;
        }
        53 => {
            return RegfileExit::jump(0x700087b768);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087b774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087b774); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087b778); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087b77e); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700087b78e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700087b792); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700087b796); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700087b79a); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700087b79e); }
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087b7a2); };
            c.w64(0, t2);
            blk = 2; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087b7a8); };
            c.w64(8, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087b7ac); };
            c.w64(7, t4);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700087b7bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700087b7c0); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b7ca); }
            let Some(t5) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087b7cf); };
            c.cmp(t5, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b7d5); }
            blk = 8; continue;
        }
        5 => {
            c.w64(3, c.r64(8));
            let r6 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r6);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087b7ec); };
            c.cmp(t7, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(3, c.r64(0));
            let r8 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r8);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b7f7); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 259; continue;
        }
        10 => {
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ge() { blk = 259; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700087b819); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 285; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b827); };
            c.w32(3, t9);
            let r10 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r10);
            let Some(t11) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087b82d); };
            c.w32(8, t11);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 285; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 278; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b854); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087b85b); };
            c.w32(9, t14);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087b867); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(2) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(1, 0x1fffffffffffffu64);
            let r15 = c.add(c.r64(1), c.r64(2), 64);
            c.w64(1, r15);
            c.w64(6, 0x3ffffffffffffeu64);
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087b8a2); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087b8a6); };
            c.w64(6, t17);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 264; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087b8bc); };
            c.w32(2, t18);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 285; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 285; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(9, c.r64(0));
            blk = 30; continue;
        }
        27 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087b8ed); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            blk = 30; continue;
        }
        28 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r21 = c.r64(2) & 0xffffffu64;
            c.w64(2, r21);
            c.logic(r21, 64);
            blk = 107; continue;
        }
        29 => {
            c.w64(9, c.r64(0));
            blk = 30; continue;
        }
        30 => {
            c.w64(2, c.r64(7));
            blk = 32; continue;
        }
        31 => {
            let Some(t22) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087b90e); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087b914); };
            c.w32(8, t24);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087b926); };
            c.w32(1, t25);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087b935); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            c.logic(c.r32(1) & 0xffc00u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 35; continue;
        }
        35 => {
            let r28 = c.shr(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r28);
            let r29 = c.r32(1) & 0x3ffu64;
            c.w32(1, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_be() { blk = 48; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t30) = c.ld16(c.r64(2).wrapping_add(0x5)) else { return RegfileExit::fault(0x700087b959); };
            c.w32(6, (t30 as u16 as i16 as i64 as u64));
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087b95d); };
            c.w32(7, t31);
            let r32 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r32);
            let r33 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r33);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700087b967); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700087b970); }
            blk = 38; continue;
        }
        38 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087b974); };
            c.w64(0, t34);
            let r35 = c.sub(c.r32(0), c.r32(15), 32);
            c.w32(0, r35);
            let r36 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r36);
            let r37 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r37);
            c.w32(12, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087b987); };
            c.w32(12, t38);
            let r39 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r39);
            let r40 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.r32(12) & 0x3ffu64;
            c.w32(12, r41);
            c.logic(r41, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t42) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087b9a1); };
            c.w32(12, t42);
            let r43 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r43);
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087b9a9); };
            c.w32(12, t44);
            let r45 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r45);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_be() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(15, c.r64(0).wrapping_add(0x1));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087b9bb); };
            c.w64(0, t46);
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 43; continue;
        }
        42 => {
            let r47 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r47);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_l() { blk = 70; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w32(12, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087b9da); };
            c.w32(12, t48);
            let r49 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r49);
            let r50 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            let r51 = c.r32(12) & 0x3ffu64;
            c.w32(12, r51);
            c.logic(r51, 32);
            c.w32(0, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t52) = c.ld32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087b9f4); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087b9fb); };
            c.w32(11, t54);
            let r55 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r55);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_be() { blk = 70; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 240; continue;
        }
        48 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let r56 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r56);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 70; continue; }
            blk = 49; continue;
        }
        49 => {
            let r57 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r57);
            let Some(t58) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087ba34); };
            c.cmp(t58, c.r32(9), 32);
            if c.cc_e() { blk = 240; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 49; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 70; continue;
        }
        52 => {
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087ba4a); };
            c.w32(11, t59);
            let r60 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r60);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700087ba57); };
            c.w64(11, t61);
            blk = 54; continue;
        }
        54 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087ba5e); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r63 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r63);
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087ba6c); };
            c.w32(15, t64);
            let r65 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r65);
            let r66 = c.r64(15) & c.r64(12);
            c.w64(15, r66);
            c.logic(r66, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r67 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r67);
            let r68 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r68);
            c.w64(2, c.r64(12));
            let r69 = c.r64(2) & c.r64(15);
            c.w64(2, r69);
            c.logic(r69, 64);
            c.w64(15, c.r64(2));
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087ba90); };
            c.w32(2, t70);
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087ba95); };
            c.cmp(t71, c.r32(2), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_e() { blk = 240; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 55; continue;
        }
        59 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 272; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t72) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087baba); };
            c.logic(t72 & 0x24u64, 8);
            if c.cc_ne() { blk = 277; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t73) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087bac4); };
            c.w32(11, t73);
            let r74 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r74);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700087bad1); };
            c.w64(11, t75);
            blk = 63; continue;
        }
        63 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087bad8); };
            c.w32(12, t76);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r77 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r77);
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087bae6); };
            c.w32(15, t78);
            let r79 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r79);
            let r80 = c.r64(15) & c.r64(12);
            c.w64(15, r80);
            c.logic(r80, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        64 => {
            let r81 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r81);
            let r82 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r82);
            c.w64(2, c.r64(12));
            let r83 = c.r64(2) & c.r64(15);
            c.w64(2, r83);
            c.logic(r83, 64);
            c.w64(15, c.r64(2));
            blk = 65; continue;
        }
        65 => {
            c.w64(2, c.r64(15).wrapping_add(0x5));
            let Some(t84) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bb09); };
            c.w32(1, t84);
            let r85 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r85);
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087bb11); };
            c.cmp(t86, c.r32(1), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087bb1a); };
            c.cmp(t87, c.r32(1), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t88) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087bb23); };
            c.cmp(t88, c.r32(9), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t89) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087bb29); };
            c.w32(11, t89);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 305; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t90) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087bb3c); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087bb40); };
            c.cmp(t91, c.r32(11), 32);
            if c.cc_ne() { blk = 240; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t92) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087bb5a); };
            c.w32(8, t92);
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            let Some(t94) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bb61); };
            c.w32(8, t94);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 101; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bb71); };
            c.w32(11, t95);
            let r96 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r96);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 101; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 76; continue;
        }
        76 => {
            c.w64(15, c.r64(11));
            let r97 = c.r32(15) & 0x7u64;
            c.w32(15, r97);
            c.logic(r97, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 266; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 266; continue;
        }
        84 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087bbde); };
            c.w32(11, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087bbe3); };
            c.cmp(t99, c.r32(11), 32);
            if c.cc_ne() { blk = 267; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087bbf0); };
            c.w32(12, t100);
            let r101 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r101);
            let Some(t102) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087bbf8); };
            c.w32(11, t102);
            let r103 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r103);
            let Some(t104) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bc00); };
            c.w32(11, t104);
            blk = 76; continue;
        }
        86 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087bc07); };
            c.w32(11, t105);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r106 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r106);
            let Some(t107) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087bc15); };
            c.w32(12, t107);
            let r108 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087bc1d); };
            c.w32(11, t109);
            let r110 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r110);
            let Some(t111) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bc25); };
            c.w32(11, t111);
            blk = 76; continue;
        }
        87 => {
            let Some(t112) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087bc2f); };
            c.w32(12, t112);
            let r113 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r113);
            let Some(t114) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087bc37); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bc3f); };
            c.w32(11, t116);
            blk = 76; continue;
        }
        88 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 91; continue;
        }
        89 => {
            c.w32(15, 0x1u64);
            blk = 91; continue;
        }
        90 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 268; continue; }
            blk = 92; continue;
        }
        92 => {
            let r117 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r117);
            blk = 93; continue;
        }
        93 => {
            let r118 = c.r32(11) & 0x8u64;
            c.w32(11, r118);
            c.logic(r118, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t119) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x700087bc79); };
            c.w32(8, t119);
            blk = 96; continue;
        }
        95 => {
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700087bc80); };
            c.w32(8, t120);
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 276; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 275; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 273; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087bcb3); };
            c.w64(0, t121);
            let Some(t122) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087bcb7); };
            c.w32(8, t122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700087bcbb); };
            c.cmp(t123, c.r32(8), 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t124) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bcc4); };
            c.logic(t124 & 0x1u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t125) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087bcce); };
            c.w32(2, t125);
            let r126 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087bcd4); };
            c.cmp(t127, c.r32(2), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 104; continue;
        }
        104 => {
            blk = 31; continue;
        }
        105 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(2, c.r64(9));
            blk = 106; continue;
        }
        106 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087bcef); };
            c.w64(6, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087bcf3); };
            c.w64(1, t129);
            blk = 107; continue;
        }
        107 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700087bcf7); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 109; continue;
        }
        108 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087bd03); };
            c.w32(3, t130);
            let r131 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r131);
            let Some(t132) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bd0a); };
            c.w32(9, t132);
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 285; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t133) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087bd1c); };
            c.w32(9, t133);
            let r134 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r134);
            let r135 = c.r32(9) & 0x3fu64;
            c.w32(9, r135);
            c.logic(r135, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 145; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 129; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 121; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 118; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x28u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 117; continue;
        }
        117 => {
            blk = 285; continue;
        }
        118 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 285; continue;
        }
        121 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_l() { blk = 126; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 125; continue;
        }
        125 => {
            blk = 285; continue;
        }
        126 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 285; continue;
        }
        129 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 138; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 135; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 134; continue;
        }
        134 => {
            blk = 285; continue;
        }
        135 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 137; continue;
        }
        137 => {
            blk = 285; continue;
        }
        138 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 285; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 285; continue;
        }
        145 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_l() { blk = 163; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_l() { blk = 155; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_l() { blk = 152; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 285; continue;
        }
        152 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 154; continue;
        }
        154 => {
            blk = 285; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_l() { blk = 160; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(9), 0xfu64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 285; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 162; continue;
        }
        162 => {
            blk = 285; continue;
        }
        163 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 172; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 169; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 285; continue;
        }
        169 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 171; continue;
        }
        171 => {
            blk = 285; continue;
        }
        172 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 176; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 175; continue;
        }
        175 => {
            blk = 285; continue;
        }
        176 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 285; continue;
        }
        179 => {
            blk = 184; continue;
        }
        180 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bf45); };
            c.w32(8, t136);
            let r137 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r137);
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087bf4c); };
            c.w32(9, t138);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_ae() { blk = 185; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t139) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bf65); };
            c.cmp(t139, c.r64(9), 64);
            if c.cc_e() { blk = 203; continue; }
            blk = 182; continue;
        }
        182 => {
            blk = 240; continue;
        }
        183 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087bf75); };
            c.w32(9, t140);
            let r141 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r141);
            let Some(t142) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bf7c); };
            c.w32(9, t142);
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 240; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bf89); };
            c.w32(8, t143);
            let r144 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087bf90); };
            c.w32(9, t145);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 187; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 285; continue; }
            blk = 186; continue;
        }
        186 => {
            blk = 203; continue;
        }
        187 => {
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bfb3); };
            c.w32(8, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087bfb8); };
            c.cmp(t147, c.r32(8), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 188; continue;
        }
        188 => {
            blk = 240; continue;
        }
        189 => {
            let Some(t148) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087bfca); };
            c.w32(9, t148);
            let r149 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r149);
            let Some(t150) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bfd1); };
            c.w32(9, t150);
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 240; continue; }
            blk = 190; continue;
        }
        190 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700087bfde); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 285; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087bff1); };
            c.w32(8, t151);
            let r152 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r152);
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087bff8); };
            c.w32(9, t153);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r154 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r154);
            c.w32(12, c.r32(2));
            c.w64(10, c.r64(4));
            let r155 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r155);
            let r156 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r156);
            c.logic(r156, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700087c014); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700087c019); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700087c01d); }
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(13));
            c.w64(15, c.r64(1));
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700087c02a); };
            c.w64(0, t157);
            c.w64(10, 0x700087c038u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700087c038); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700087c03c); }
            return c.call(c.r64(0), 0x700087c042, 0x700087c040);
        }
        192 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700087c042); }
            let Some(t158) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087c04a); };
            c.w64(4, t158);
            c.w32(8, c.r32(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087c052); };
            let r160 = c.r64(8) & t159;
            c.w64(8, r160);
            c.logic(r160, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087c059); };
            c.w64(11, t161);
            blk = 194; continue;
        }
        193 => {
            let r162 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r162);
            let r163 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087c066); };
            c.w64(12, t164);
            let r165 = c.r64(12) & c.r64(8);
            c.w64(12, r165);
            c.logic(r165, 64);
            c.w64(8, c.r64(12));
            blk = 194; continue;
        }
        194 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t166) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c075); };
            c.w32(12, t166);
            let r167 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087c07d); };
            c.cmp(t168, c.r32(12), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 195; continue;
        }
        195 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087c08c); };
            c.cmp(t169, c.r32(12), 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 197; continue;
        }
        197 => {
            c.xorpd_rr(0, 0);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087c099); };
            c.cvtsi2sd(0, (t170 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700087c09f); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 193; continue; }
            blk = 198; continue;
        }
        198 => {
            if c.cc_e() { blk = 240; continue; }
            blk = 199; continue;
        }
        199 => {
            blk = 193; continue;
        }
        200 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087c0ba); };
            c.cmp(c.r64(12), t171, 64);
            if c.cc_e() { blk = 240; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 193; continue;
        }
        202 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c0c6); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087c0ca); };
            c.w64(3, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087c0ce); };
            c.w64(2, t174);
            blk = 203; continue;
        }
        203 => {
            let Some(t175) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087c0d2); };
            c.w32(8, t175);
            let r176 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087c0d9); };
            c.w64(1, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c0dd); };
            c.w64(7, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087c0e1); };
            c.w64(6, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087c0e5); };
            c.cmp(t180, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 204; continue;
        }
        204 => {
            blk = 108; continue;
        }
        205 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c0f7); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700087c0fe); };
            c.logic(t183 & 0x4u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(9, c.r64(2));
            let Some(t184) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700087c10c); };
            c.cmp(t184, c.r64(9), 64);
            if c.cc_a() { blk = 240; continue; }
            blk = 207; continue;
        }
        207 => {
            blk = 241; continue;
        }
        208 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c11b); };
            c.w32(11, t185);
            let r186 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r186);
            let Some(t187) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700087c122); };
            c.w32(12, t187);
            c.w64(9, c.r64(2));
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 237; continue; }
            blk = 209; continue;
        }
        209 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t188) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c13d); };
            c.w64(11, t188);
            let Some(t189) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087c141); };
            c.w64(12, t189);
            let Some(t190) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700087c145); };
            c.logic(t190 & 0x1u64, 8);
            if c.cc_ne() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t191) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087c14c); };
            c.w64(15, t191);
            let r192 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r192);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 241; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t193) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700087c15c); };
            c.w64(8, t193);
            blk = 239; continue;
        }
        213 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 241; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t194) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c16e); };
            c.w32(8, t194);
            let r195 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r195);
            let Some(t196) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087c175); };
            c.w32(8, t196);
            let r197 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r197);
            let r198 = c.r32(8) & 0x3fu64;
            c.w32(8, r198);
            c.logic(r198, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 224; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 220; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 219; continue;
        }
        219 => {
            blk = 261; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 222; continue;
        }
        222 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 223; continue;
        }
        223 => {
            blk = 261; continue;
        }
        224 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 229; continue; }
            blk = 225; continue;
        }
        225 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 228; continue;
        }
        228 => {
            blk = 261; continue;
        }
        229 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            blk = 261; continue;
        }
        232 => {
            c.w32(8, 0x1u64);
            blk = 236; continue;
        }
        233 => {
            c.w32(8, 0x2u64);
            blk = 236; continue;
        }
        234 => {
            c.w32(8, 0x4u64);
            blk = 236; continue;
        }
        235 => {
            c.w32(8, 0x8u64);
            blk = 236; continue;
        }
        236 => {
            let r199 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r199);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(8), 64);
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(11));
            blk = 239; continue;
        }
        237 => {
            c.w64(10, c.r64(4));
            let r200 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r200);
            let r201 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r201);
            c.logic(r201, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700087c238); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700087c243); };
            c.w64(0, t202);
            c.w64(10, 0x700087c251u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700087c251); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700087c255); }
            return c.call(c.r64(0), 0x700087c25b, 0x700087c259);
        }
        238 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700087c25b); }
            let Some(t203) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087c263); };
            c.w64(4, t203);
            c.w64(8, c.r64(0));
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c26b); };
            c.w64(0, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087c26f); };
            c.w64(9, t205);
            blk = 239; continue;
        }
        239 => {
            c.w64(2, c.r64(9));
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ae() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700087c27b); };
            c.w64(8, t206);
            blk = 242; continue;
        }
        241 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087c284); };
            c.w64(8, t207);
            blk = 242; continue;
        }
        242 => {
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700087c28b); };
            c.cmp(t208, c.r32(8), 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c298); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c29c); };
            c.w64(3, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087c2a0); };
            c.w64(6, t211);
            return c.call(0x7000870914, 0x700087c2a9, 0x700087c2a4);
        }
        244 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c2a9); };
            if !c.push64(t212) { return RegfileExit::fault(0x700087c2a9); }
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c2ac); };
            if !c.push64(t213) { return RegfileExit::fault(0x700087c2ac); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087c2af); }
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087c2b0); };
            if !c.push64(t214) { return RegfileExit::fault(0x700087c2b0); }
            c.w32(0, 0x3u64);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087c2b8); };
            c.w64(7, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087c2bc); };
            c.w64(6, t216);
            return c.call(0x70007f2274, 0x700087c2c5, 0x700087c2c0);
        }
        245 => {
            let Some(t217) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087c2c5); };
            c.cmp(t217, c.r32(0), 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 252; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087c2d5); };
            c.cmp(t218, c.r32(0), 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t219) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c2de); };
            c.w32(8, t219);
            let r220 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r220);
            let Some(t221) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087c2e5); };
            c.logic(t221 & 0x10u64, 8);
            if c.cc_ne() { blk = 254; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t222) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087c2ec); };
            c.cmp(t222, c.r32(8), 32);
            if c.cc_e() { blk = 289; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t223) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c2f9); };
            c.cmp(t223, 0x41u64, 16);
            if c.cc_e() { blk = 287; continue; }
            blk = 251; continue;
        }
        251 => {
            blk = 253; continue;
        }
        252 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 253; continue;
        }
        253 => {
            c.w32(8, 0x1u64);
            blk = 255; continue;
        }
        254 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t224) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087c31e); };
            c.cmp(t224, 0x0u64, 32);
            if c.cc_e() { blk = 291; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087c328); };
            c.w64(8, t225);
            let r226 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r226);
            if c.cc_o() { blk = 291; continue; }
            blk = 258; continue;
        }
        258 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        259 => {
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700087c33e); };
            c.w64(0, t227);
            blk = 260; continue;
        }
        260 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087c348); }
            return c.ret(0x18, 0x700087c349);
        }
        261 => {
            return RegfileExit::fault(0x700087c34c);
        }
        262 => {
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087c34d); };
            c.w64(0, t228);
            blk = 260; continue;
        }
        263 => {
            blk = 285; continue;
        }
        264 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 285; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t229) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c366); };
            c.w32(9, t229);
            let r230 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r230);
            blk = 30; continue;
        }
        266 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700087c372); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700087c376); }
            blk = 270; continue;
        }
        267 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700087c37c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700087c380); }
            blk = 270; continue;
        }
        268 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700087c386); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700087c38a); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 270; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t231) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087c394); };
            c.w64(12, t231);
            blk = 93; continue;
        }
        270 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700087c39e); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087c3a0); }
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700087c3a1); };
            c.w64(3, t232);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700087c3b5, 0x700087c3b0);
        }
        271 => {
            let r233 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r233);
            c.w64(8, c.r64(0));
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087c3ba); };
            c.w64(3, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087c3be); };
            c.w64(9, t235);
            blk = 96; continue;
        }
        272 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c3c7); };
            c.w64(0, t236);
            blk = 285; continue;
        }
        273 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c3cd); };
            c.w64(0, t237);
            blk = 285; continue;
        }
        274 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c3d3); };
            c.w64(0, t238);
            blk = 285; continue;
        }
        275 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c3d9); };
            c.w64(0, t239);
            blk = 285; continue;
        }
        276 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c3df); };
            c.w64(0, t240);
            blk = 285; continue;
        }
        277 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c3e5); };
            c.w64(0, t241);
            blk = 285; continue;
        }
        278 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087c3eb); };
            c.w64(6, t242);
            return c.call(0x70008a1af4, 0x700087c3f4, 0x700087c3ef);
        }
        279 => {
            let Some(t243) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c3f4); };
            c.w32(8, t243);
            let Some(t244) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700087c3f8); };
            c.cmp(t244, c.r32(8), 32);
            if c.cc_ne() { blk = 282; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t245) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c401); };
            c.logic(t245 & 0x1u64, 8);
            if c.cc_e() { blk = 283; continue; }
            blk = 281; continue;
        }
        281 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c407); };
            c.w64(0, t246);
            blk = 241; continue;
        }
        282 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c410); };
            c.w64(3, t247);
            blk = 283; continue;
        }
        283 => {
            c.w64(3, c.r64(0));
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c417); };
            c.w64(0, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087c41b); };
            c.w64(6, t249);
            return c.call(0x70008bced4, 0x700087c424, 0x700087c41f);
        }
        284 => {
            c.w64(8, c.r64(0));
            blk = 242; continue;
        }
        285 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c42c); };
            if !c.push64(t250) { return RegfileExit::fault(0x700087c42c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087c42f); }
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700087c430); };
            c.w64(3, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087c437); };
            c.w64(6, t252);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700087c448, 0x700087c443);
        }
        286 => {
            c.w64(8, c.r64(0));
            blk = 242; continue;
        }
        287 => {
            let Some(t253) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087c450); };
            c.logic(t253 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 288; continue;
        }
        288 => {
            blk = 253; continue;
        }
        289 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087c462); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 253; continue; }
            blk = 290; continue;
        }
        290 => {
            blk = 254; continue;
        }
        291 => {
            let Some(t254) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087c47a); };
            c.cmp(t254, 0x0u64, 32);
            if c.cc_ne() { blk = 293; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c480); };
            c.w64(0, t255);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087c484); }
            blk = 294; continue;
        }
        293 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087c48b); };
            c.w64(0, t256);
            let r257 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r257);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 294; continue;
        }
        294 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x700087c4b9); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 300; continue; }
            blk = 295; continue;
        }
        295 => {
            if c.cc_ne() { blk = 300; continue; }
            blk = 296; continue;
        }
        296 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 298; continue; }
            blk = 297; continue;
        }
        297 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 300; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(9, c.r64(8));
            let r258 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r258);
            if c.cc_o() { blk = 300; continue; }
            blk = 299; continue;
        }
        299 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        300 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087c4eb); };
            c.w64(8, t259);
            let Some(t260) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087c4f2); };
            c.w64(9, t260);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087c4f9); };
            c.w64(12, t261);
            let Some(t262) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087c500); };
            c.cmp(t262, c.r64(11), 64);
            if c.cc_a() { blk = 303; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087c510, 0x700087c50b);
        }
        302 => {
            blk = 304; continue;
        }
        303 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087c512); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 304; continue;
        }
        304 => {
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087c51c); };
            c.w64(8, t263);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087c523); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700087c527); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087c52c); }
            blk = 1; continue;
        }
        305 => {
            return RegfileExit::fault(0x700087c536);
        }
        306 => {
            return RegfileExit::jump(0x700087c538);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087c554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087c554); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087c558); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087c559); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087c55a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087c55f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700087c563); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087c567); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087c57f); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087c584); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087c593); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700087c5a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700087c5a6); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c5bd); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c5c4); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 93; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700087c5de); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c5e8); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c5ef); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c5f8); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c618); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087c61f); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087c626); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c62d); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700087c63d);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700087c63e); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c655); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087c65c); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087c664); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c66c); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x700087c679);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x700087c67a); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0xaeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700087c68d);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700087c68e); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 96; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x700087c6a5);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c6a6); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a17f4, 0x700087c6bb, 0x700087c6b6);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087c6be); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087c6c2); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c6c6); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087c6ca); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087c6ce); };
            c.w64(9, t35);
            blk = 29; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        28 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(3, (c.cc_be() as u64));
            c.w32(3, c.r8(3));
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700087c6f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700087c6f6); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 105; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087c709); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087c712); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087c719); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700087c720); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 106; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c72d); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087c733); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 106; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(9));
            let Some(t42) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087c746); };
            c.w64(8, t42);
            blk = 38; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700087c74f); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 87; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c75d); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700087c764); }
            let Some(t45) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c768); };
            c.cmp(t45, 0x834u64, 16);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, 0x1u64);
            blk = 42; continue;
        }
        41 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087c783); };
            c.w32(3, t46);
            c.w64(7, c.r64(3));
            let r47 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r47);
            let r48 = c.r32(7) & 0x3fu64;
            c.w32(7, r48);
            c.logic(r48, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700087c791); }
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 87; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c79e); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087c7a4); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087c7aa); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700087c7ba);
        }
        46 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700087c7bb); };
            c.w32(8, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087c7c2); };
            c.cmp(t55, c.r32(8), 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087c7cc); };
            c.w64(8, t56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c7d3); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(7, c.r64(12));
            blk = 50; continue;
        }
        49 => {
            c.w32(8, c.r64(7).wrapping_add(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087c7e9); };
            c.w64(9, t58);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087c7ed); };
            c.w64(8, t59);
            blk = 50; continue;
        }
        50 => {
            let Some(t60) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087c7f4); };
            c.cmp(c.r32(7), t60, 32);
            if c.cc_ge() { blk = 79; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t61) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c7fd); };
            c.cmp(t61, c.r32(9), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c807); };
            c.cmp(t62, c.r32(12), 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087c811); };
            c.w32(8, t63);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_le() { blk = 85; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t64) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087c81e); };
            c.w32(8, t64);
            let r65 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r65);
            let r66 = c.r32(8) & 0x3fu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            let r67 = c.shr(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r67);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c834); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c83b); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r71);
            let Some(t72) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c842); };
            c.w32(9, t72);
            let r73 = c.sub(c.r32(9), 0x77u64, 32);
            c.w32(9, r73);
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_a() { blk = 81; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t74) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087c855); };
            c.w32(9, t74);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 80; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t75) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c865); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087c86d); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700087c87a); }
            blk = 65; continue;
        }
        59 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c883); };
            c.w32(8, t78);
            let r79 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r79);
            let Some(t80) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c88a); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700087c88e); };
            c.cmp(t81, c.r32(9), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087c89b); };
            c.w32(9, t82);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 82; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700087c8ab); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t83) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c8bc); };
            c.cmp(t83, c.r64(11), 64);
            if c.cc_e() { blk = 78; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087c8c7); };
            c.w64(8, t84);
            let Some(t85) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087c8ce); };
            c.w64(9, t85);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700087c8d5); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087c8d9); };
            c.w64(12, t86);
            let Some(t87) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087c8e0); };
            c.cmp(t87, c.r64(11), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087c8ea); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087c8f1); };
            c.w64(9, t88);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700087c8f8); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087c8fc); }
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c902); };
            c.w64(11, t89);
            blk = 65; continue;
        }
        65 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700087c906); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087c908); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087c909); }
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087c90b); };
            if !c.push64(t90) { return RegfileExit::fault(0x700087c90b); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x700087c91e, 0x700087c919);
        }
        66 => {
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087c91e); };
            c.cmp(t91, c.r32(0), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087c92e); };
            c.cmp(t92, c.r32(0), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t93) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087c937); };
            c.w32(8, t93);
            let r94 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r94);
            let Some(t95) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087c93e); };
            c.logic(t95 & 0x10u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087c945); };
            c.cmp(t96, c.r32(8), 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t97) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087c952); };
            c.cmp(t97, 0x41u64, 16);
            if c.cc_e() { blk = 101; continue; }
            blk = 72; continue;
        }
        72 => {
            blk = 74; continue;
        }
        73 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(8, 0x1u64);
            blk = 76; continue;
        }
        75 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087c977); };
            c.w64(6, t98);
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087c97b); };
            c.w64(2, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087c982); };
            c.w64(0, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c986); };
            c.w64(11, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087c98a); };
            c.w64(1, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087c98e); };
            c.w64(7, t103);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 49; continue;
        }
        78 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087c99a); };
            c.w64(11, t104);
            blk = 49; continue;
        }
        79 => {
            c.w64(8, c.r64(0));
            let r105 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r105);
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700087c9aa); };
            c.w64(0, t106);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087c9b4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087c9b5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087c9bb); }
            return c.ret(0x0, 0x700087c9bd);
        }
        80 => {
            return RegfileExit::fault(0x700087c9be);
        }
        81 => {
            return RegfileExit::fault(0x700087c9bf);
        }
        82 => {
            return RegfileExit::fault(0x700087c9c0);
        }
        83 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087c9c1); };
            c.w64(8, t107);
            let r108 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r108);
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087c9c9); };
            c.w64(0, t109);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087c9d3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087c9d4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087c9da); }
            return c.ret(0x0, 0x700087c9dc);
        }
        84 => {
            return RegfileExit::fault(0x700087c9dd);
        }
        85 => {
            c.w64(12, c.r64(7));
            blk = 87; continue;
        }
        86 => {
            c.w64(12, c.r64(7));
            blk = 87; continue;
        }
        87 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700087c9e6); }
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087c9e8); };
            if !c.push64(t110) { return RegfileExit::fault(0x700087c9e8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700087c9eb); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(1));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087c9f2); };
            c.w64(1, t111);
            c.w64(7, c.r64(11));
            return c.call(0x700087b774, 0x700087c9fe, 0x700087c9f9);
        }
        88 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087c9fe); };
            c.w64(8, t112);
            let r113 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r113);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087ca09); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087ca0a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087ca10); }
            return c.ret(0x0, 0x700087ca12);
        }
        89 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087ca19); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700087ca1b); };
            c.w64(8, t114);
            let Some(t115) = c.ld32(c.r64(8).wrapping_add(0x10f)) else { return RegfileExit::fault(0x700087ca22); };
            c.w32(8, t115);
            let r116 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r116);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087ca2c); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700087ca2e); };
            c.w64(3, t117);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700087ca3f, 0x700087ca3a);
        }
        90 => {
            return RegfileExit::fault(0x700087ca3f);
        }
        91 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087ca40); };
            c.w64(3, t118);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700087ca51, 0x700087ca4c);
        }
        92 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ca51); };
            c.w64(6, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ca55); };
            c.w64(0, t120);
            blk = 1; continue;
        }
        93 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x700087ca66, 0x700087ca61);
        }
        94 => {
            c.w64(11, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ca69); };
            c.w64(6, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ca6d); };
            c.w64(8, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ca71); };
            c.w64(9, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ca75); };
            c.w64(0, t124);
            blk = 4; continue;
        }
        95 => {
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087ca7e); };
            c.w32(12, t125);
            let r126 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r126);
            blk = 23; continue;
        }
        96 => {
            let Some(t127) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700087ca8a); };
            c.cmp(t127, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700087ca98); };
            c.w64(3, t128);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x700087caa7, 0x700087caa2);
        }
        98 => {
            c.w64(12, c.r64(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087caaa); };
            c.w64(6, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087caae); };
            c.w64(8, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087cab2); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087cab6); };
            c.w64(9, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087caba); };
            c.w64(0, t133);
            blk = 23; continue;
        }
        99 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700087cac8); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700087cad5, 0x700087cad0);
        }
        100 => {
            c.w64(8, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087cad8); };
            c.w64(6, t134);
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087cadc); };
            c.w64(2, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087cae3); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087cae7); };
            c.w64(1, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087caeb); };
            c.w64(7, t138);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700087caef); }
            blk = 64; continue;
        }
        101 => {
            let Some(t139) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087caf9); };
            c.logic(t139 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 74; continue;
        }
        103 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087cb0b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 74; continue; }
            blk = 104; continue;
        }
        104 => {
            blk = 75; continue;
        }
        105 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087cb23); };
            c.w64(2, t140);
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t141) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087cb2f); };
            c.w64(8, t141);
            c.w64(2, c.r64(8));
            blk = 108; continue;
        }
        108 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087cb3c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700087cb3e); }
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700087cb3f); };
            c.w64(3, t142);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700087cb50, 0x700087cb4b);
        }
        109 => {
            return RegfileExit::fault(0x700087cb50);
        }
        110 => {
            return RegfileExit::jump(0x700087cb54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087cb74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087cb74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087cb78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087cb79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087cb7a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087cb7f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087cb89); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cb97); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087cb9e); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 22; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087cbb7); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cbc4); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087cbcb); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087cbd6); };
            c.w64(2, t11);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cbe3); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087cbea); };
            c.w32(8, t14);
            let r15 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r15);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087cc03); };
            c.w64(8, t16);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cc0d); };
            c.w32(9, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087cc11); };
            c.cmp(t18, c.r32(9), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x40)) else { return RegfileExit::fault(0x700087cc1a); };
            c.w64(9, t19);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cc24); };
            c.w32(11, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087cc28); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700087cc31); };
            c.w64(11, t22);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cc3b); };
            c.w32(12, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087cc3f); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087cc48); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700087cc4a); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700087cc4c); }
            c.w64(0, c.r64(7));
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087cc51); };
            c.w64(1, t25);
            return c.call(0x700087d0b4, 0x700087cc5a, 0x700087cc55);
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087cc5a); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087cc61); }
            c.cmp(c.r64(1), 0x6u64, 64);
            if c.cc_g() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return c.ret(0x38, 0x700087cc68);
        }
        16 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087cc6b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087cc72); }
            return c.ret(0x0, 0x700087cc74);
        }
        17 => {
            return RegfileExit::fault(0x700087cc75);
        }
        18 => {
            return RegfileExit::fault(0x700087cc76);
        }
        19 => {
            return RegfileExit::fault(0x700087cc77);
        }
        20 => {
            return RegfileExit::fault(0x700087cc78);
        }
        21 => {
            return RegfileExit::fault(0x700087cc79);
        }
        22 => {
            return RegfileExit::fault(0x700087cc7a);
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087cc7b); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087cc7f); };
            c.w64(3, t27);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087cc8d, 0x700087cc88);
        }
        24 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087cc8d); };
            c.w64(6, t28);
            blk = 1; continue;
        }
        25 => {
            return RegfileExit::jump(0x700087cc98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087ccb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087ccb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087ccb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087ccb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087ccba); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700087ccbf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087ccc3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087cccd); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087ccdb); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087cce2); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 38; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087ccfb); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cd08); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087cd0f); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087cd1a); };
            c.w64(0, t11);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cd26); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087cd2d); };
            c.w32(8, t14);
            let r15 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r15);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087cd46); };
            c.w64(8, t16);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(11, c.r32(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700087cd65); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700087cd69); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700087cd6d); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cd77); };
            c.w32(15, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087cd7b); };
            c.cmp(t18, c.r32(15), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x48)) else { return RegfileExit::fault(0x700087cd88); };
            c.w64(15, t19);
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.w32(1, c.r32(15));
            c.cmp(c.r32(2), 0x0u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700087cda4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700087cda8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700087cdac); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cdb6); };
            c.w32(8, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087cdba); };
            c.cmp(t21, c.r32(8), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700087cdc7); };
            c.w64(8, t22);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087cdd1); };
            c.w32(9, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087cdd5); };
            c.cmp(t24, c.r32(9), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x50)) else { return RegfileExit::fault(0x700087cde2); };
            c.w64(9, t25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087cde6); };
            c.cmp(t26, c.r32(9), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(8, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087cdfc); };
            c.cmp(t27, c.r32(9), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087ce05); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087ce0c); };
            c.logic(t30 & 0x10u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087ce13); };
            c.cmp(t31, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t32) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087ce20); };
            c.cmp(t32, 0x41u64, 16);
            if c.cc_e() { blk = 41; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 21; continue;
        }
        20 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_add(0x40)) else { return RegfileExit::fault(0x700087ce34); };
            c.w64(1, t33);
            c.w64(3, c.r64(15));
            return c.call(0x7000899bb4, 0x700087ce40, 0x700087ce3b);
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ce40); };
            c.cmp(t34, 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087ce4a); };
            c.w64(8, t35);
            let r36 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r36);
            if c.cc_o() { blk = 45; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087ce5b); };
            c.w64(12, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087ce5f); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087ce63); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087ce67); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ce6b); };
            c.w64(6, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ce6f); };
            c.w64(11, t42);
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 27; continue;
        }
        27 => {
            let r43 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r43);
            if c.cc_o() { blk = 60; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 29; continue;
        }
        29 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087ce8a); }
            let Some(t44) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700087ce8c); };
            if !c.push64(t44) { return RegfileExit::fault(0x700087ce8c); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700087ce8f); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(7));
            let Some(t45) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087ce97); };
            c.w64(1, t45);
            return c.call(0x700087d0b4, 0x700087cea0, 0x700087ce9b);
        }
        30 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087cea0); };
            c.w64(1, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087cea7); }
            c.cmp(c.r64(1), 0x8u64, 64);
            if c.cc_g() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return c.ret(0x48, 0x700087ceae);
        }
        32 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087ceb1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087ceb8); }
            return c.ret(0x0, 0x700087ceba);
        }
        33 => {
            return RegfileExit::fault(0x700087cebb);
        }
        34 => {
            return RegfileExit::fault(0x700087cebc);
        }
        35 => {
            return RegfileExit::fault(0x700087cebd);
        }
        36 => {
            return RegfileExit::fault(0x700087cebe);
        }
        37 => {
            return RegfileExit::fault(0x700087cebf);
        }
        38 => {
            return RegfileExit::fault(0x700087cec0);
        }
        39 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087cec1); };
            c.w64(3, t47);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087cecf, 0x700087ceca);
        }
        40 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087cecf); };
            c.w64(6, t48);
            blk = 1; continue;
        }
        41 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087ced8); };
            c.logic(t49 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 21; continue;
        }
        43 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700087cee7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 21; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 26; continue;
        }
        45 => {
            let Some(t50) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087cf00); };
            c.cmp(t50, 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_add(0x48)) else { return RegfileExit::fault(0x700087cf06); };
            c.w64(15, t51);
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x700087cf0a); }
            blk = 48; continue;
        }
        47 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_add(0x48)) else { return RegfileExit::fault(0x700087cf12); };
            c.w64(15, t52);
            let r53 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r53);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 48; continue;
        }
        48 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700087cf42); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 54; continue; }
            blk = 49; continue;
        }
        49 => {
            if c.cc_ne() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, c.r64(8));
            let r54 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r54);
            if c.cc_o() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            blk = 59; continue;
        }
        54 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087cf74); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087cf7b); };
            c.w64(9, t56);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087cf82); };
            c.w64(12, t57);
            let Some(t58) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087cf89); };
            c.cmp(t58, c.r64(11), 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087cf99, 0x700087cf94);
        }
        56 => {
            blk = 58; continue;
        }
        57 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087cf9b); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 58; continue;
        }
        58 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087cfa5); };
            c.w64(8, t59);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087cfac); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700087cfb0); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087cfb5); }
            blk = 59; continue;
        }
        59 => {
            c.w64(15, c.r64(0));
            blk = 25; continue;
        }
        60 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700087cfc2); }
            let Some(t60) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087cfc6); };
            c.cmp(t60, 0x0u64, 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087cfcc); };
            c.w64(8, t61);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087cfd0); }
            blk = 63; continue;
        }
        62 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087cfd8); };
            c.w64(8, t62);
            let r63 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r63);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 63; continue;
        }
        63 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x700087d008); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 69; continue; }
            blk = 64; continue;
        }
        64 => {
            if c.cc_ne() { blk = 69; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(9, c.r64(8));
            let r64 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r64);
            if c.cc_o() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 29; continue;
        }
        69 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087d03a); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087d041); };
            c.w64(9, t66);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087d048); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087d04f); };
            c.cmp(t68, c.r64(11), 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087d05f, 0x700087d05a);
        }
        71 => {
            c.w64(8, c.r64(0));
            let Some(t69) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087d062); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087d066); };
            c.w64(3, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087d06a); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087d06e); };
            c.w64(15, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087d072); };
            c.w64(6, t73);
            blk = 73; continue;
        }
        72 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087d078); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 73; continue;
        }
        73 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087d07f); };
            c.w64(9, t74);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700087d086); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700087d08a); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087d08f); }
            blk = 29; continue;
        }
        74 => {
            return RegfileExit::jump(0x700087d09c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087d0b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087d0b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087d0b8); }
            let r0 = c.sub(c.r64(4), 0x70u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087d0be); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700087d0ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700087d0d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x700087d0d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700087d0da); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x700087d0de); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x700087d0e2); }
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087d0e6); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087d0ea); };
            c.w64(10, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(10)) { return RegfileExit::fault(0x700087d0ee); }
            blk = 2; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087d0f4); };
            c.w64(8, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700087d0f8); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087d0fc); };
            c.w64(7, t5);
            blk = 2; continue;
        }
        2 => {
            c.w32(11, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700087d10b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700087d10f); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087d118); }
            let Some(t6) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087d11d); };
            c.cmp(t6, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087d123); }
            blk = 8; continue;
        }
        5 => {
            c.w64(2, c.r64(8));
            let r7 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r7);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 8; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087d13a); };
            c.cmp(t8, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(2, c.r64(0));
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087d145); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 264; continue;
        }
        10 => {
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ge() { blk = 264; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700087d167); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 288; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d175); };
            c.w32(2, t10);
            let r11 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r11);
            let Some(t12) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d17b); };
            c.w32(8, t12);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 288; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 281; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d1a1); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d1a8); };
            c.w32(9, t15);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087d1b4); }
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
            c.w64(6, 0x1fffffffffffffu64);
            let r16 = c.add(c.r64(6), c.r64(1), 64);
            c.w64(6, r16);
            c.w64(7, 0x3ffffffffffffeu64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087d1ef); };
            c.w64(7, t17);
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087d1f3); };
            c.w64(6, t18);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 267; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d209); };
            c.w32(1, t19);
            c.logic(c.r32(1) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(1) & 0x2u64, 8);
            if c.cc_e() { blk = 288; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 288; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(9, c.r64(0));
            blk = 31; continue;
        }
        28 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d23a); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            blk = 31; continue;
        }
        29 => {
            if let Some(sr) = c.shr(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r22 = c.r64(1) & 0xffffffu64;
            c.w64(1, r22);
            c.logic(r22, 64);
            blk = 109; continue;
        }
        30 => {
            c.w64(9, c.r64(0));
            blk = 31; continue;
        }
        31 => {
            c.w64(1, c.r64(7));
            blk = 33; continue;
        }
        32 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d25b); };
            c.w32(2, t23);
            let r24 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r24);
            let Some(t25) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d261); };
            c.w32(8, t25);
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d273); };
            c.w32(6, t26);
            c.logic(c.r32(6) & 0x200000u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t27) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087d282); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            c.logic(c.r32(6) & 0xffc00u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 36; continue;
        }
        36 => {
            let r29 = c.shr(c.r32(6), 0xau64, 32).unwrap_or(c.r32(6));
            c.w32(6, r29);
            let r30 = c.r32(6) & 0x3ffu64;
            c.w32(6, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(6), 0x20u64, 32);
            if c.cc_be() { blk = 49; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t31) = c.ld16(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x700087d2a6); };
            c.w32(7, (t31 as u16 as i16 as i64 as u64));
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d2aa); };
            c.w32(11, t32);
            let r33 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r33);
            let r34 = c.sub(c.r32(7), 0x1u64, 32);
            c.w32(7, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700087d2b5); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 39; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700087d2bd); }
            blk = 39; continue;
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087d2c1); };
            c.w64(0, t35);
            let r36 = c.sub(c.r32(0), c.r32(3), 32);
            c.w32(0, r36);
            let r37 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r37);
            let r38 = c.add(c.r32(0), c.r32(3), 32);
            c.w32(0, r38);
            c.w32(15, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087d2d2); };
            c.w32(15, t39);
            let r40 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r40);
            let r41 = c.shr(c.r32(15), 0x9u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r41);
            let r42 = c.r32(15) & 0x3ffu64;
            c.w32(15, r42);
            c.logic(r42, 32);
            c.w32(15, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d2ec); };
            c.w32(15, t43);
            let r44 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r44);
            let Some(t45) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d2f4); };
            c.w32(15, t45);
            let r46 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r46);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_be() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, c.r64(0).wrapping_add(0x1));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087d304); };
            c.w64(0, t47);
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 44; continue;
        }
        43 => {
            let r48 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r48);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_l() { blk = 71; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(15, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087d320); };
            c.w32(15, t49);
            let r50 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r50);
            let r51 = c.shr(c.r32(15), 0x9u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r51);
            let r52 = c.r32(15) & 0x3ffu64;
            c.w32(15, r52);
            c.logic(r52, 32);
            c.w32(0, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t53) = c.ld32(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d33a); };
            c.w32(0, t53);
            let r54 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d341); };
            c.w32(12, t55);
            let r56 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r56);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_be() { blk = 71; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 242; continue;
        }
        49 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            let r57 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r57);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 50; continue;
        }
        50 => {
            let r58 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r58);
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d37a); };
            c.cmp(t59, c.r32(9), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 50; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 71; continue;
        }
        53 => {
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d390); };
            c.w32(11, t60);
            let r61 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r61);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700087d39d); };
            c.w64(11, t62);
            blk = 55; continue;
        }
        55 => {
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d3a4); };
            c.w32(12, t63);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r64 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r64);
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d3b2); };
            c.w32(15, t65);
            let r66 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r66);
            let r67 = c.r64(15) & c.r64(12);
            c.w64(15, r67);
            c.logic(r67, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 57; continue;
        }
        56 => {
            let r68 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r68);
            let r69 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r69);
            c.w64(3, c.r64(12));
            let r70 = c.r64(3) & c.r64(15);
            c.w64(3, r70);
            c.logic(r70, 64);
            c.w64(15, c.r64(3));
            blk = 57; continue;
        }
        57 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d3d6); };
            c.w32(3, t71);
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087d3db); };
            c.cmp(t72, c.r32(3), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 56; continue;
        }
        60 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 275; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t73) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087d400); };
            c.logic(t73 & 0x24u64, 8);
            if c.cc_ne() { blk = 280; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t74) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d40a); };
            c.w32(11, t74);
            let r75 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r75);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700087d417); };
            c.w64(11, t76);
            blk = 64; continue;
        }
        64 => {
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d41e); };
            c.w32(12, t77);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r78 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r78);
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d42c); };
            c.w32(15, t79);
            let r80 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r80);
            let r81 = c.r64(15) & c.r64(12);
            c.w64(15, r81);
            c.logic(r81, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 66; continue;
        }
        65 => {
            let r82 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r82);
            let r83 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r83);
            c.w64(3, c.r64(12));
            let r84 = c.r64(3) & c.r64(15);
            c.w64(3, r84);
            c.logic(r84, 64);
            c.w64(15, c.r64(3));
            blk = 66; continue;
        }
        66 => {
            c.w64(3, c.r64(15).wrapping_add(0x5));
            let Some(t85) = c.ld32(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d44f); };
            c.w32(1, t85);
            let r86 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r86);
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087d457); };
            c.cmp(t87, c.r32(1), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t88) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087d460); };
            c.cmp(t88, c.r32(1), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t89) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d469); };
            c.cmp(t89, c.r32(9), 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t90) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d46f); };
            c.w32(11, t90);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 323; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t91) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d482); };
            c.w32(11, t91);
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087d486); };
            c.cmp(t92, c.r32(11), 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 102; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t93) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d4a0); };
            c.w32(8, t93);
            let r94 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r94);
            let Some(t95) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d4a7); };
            c.w32(8, t95);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 102; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t96) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d4b7); };
            c.w32(11, t96);
            let r97 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r97);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 102; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 77; continue;
        }
        77 => {
            c.w64(15, c.r64(11));
            let r98 = c.r32(15) & 0x7u64;
            c.w32(15, r98);
            c.logic(r98, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 82; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 81; continue;
        }
        81 => {
            blk = 269; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 269; continue;
        }
        85 => {
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d524); };
            c.w32(11, t99);
            let Some(t100) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087d529); };
            c.cmp(t100, c.r32(11), 32);
            if c.cc_ne() { blk = 270; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t101) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d536); };
            c.w32(12, t101);
            let r102 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r102);
            let Some(t103) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d53e); };
            c.w32(11, t103);
            let r104 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r104);
            let Some(t105) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d546); };
            c.w32(11, t105);
            blk = 77; continue;
        }
        87 => {
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d54d); };
            c.w32(11, t106);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r107 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r107);
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d55b); };
            c.w32(12, t108);
            let r109 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r109);
            let Some(t110) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d563); };
            c.w32(11, t110);
            let r111 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r111);
            let Some(t112) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d56b); };
            c.w32(11, t112);
            blk = 77; continue;
        }
        88 => {
            let Some(t113) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d575); };
            c.w32(12, t113);
            let r114 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r114);
            let Some(t115) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d57d); };
            c.w32(11, t115);
            let r116 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r116);
            let Some(t117) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d585); };
            c.w32(11, t117);
            blk = 77; continue;
        }
        89 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 92; continue;
        }
        90 => {
            c.w32(15, 0x1u64);
            blk = 92; continue;
        }
        91 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 271; continue; }
            blk = 93; continue;
        }
        93 => {
            let r118 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r118);
            blk = 94; continue;
        }
        94 => {
            let r119 = c.r32(11) & 0x8u64;
            c.w32(11, r119);
            c.logic(r119, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t120) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x700087d5bf); };
            c.w32(8, t120);
            blk = 97; continue;
        }
        96 => {
            let Some(t121) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700087d5c6); };
            c.w32(8, t121);
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 279; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 276; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087d5f9); };
            c.w64(0, t122);
            let Some(t123) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d5fd); };
            c.w32(8, t123);
            let Some(t124) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700087d601); };
            c.cmp(t124, c.r32(8), 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t125) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d60a); };
            c.logic(t125 & 0x1u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t126) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d614); };
            c.w32(1, t126);
            let r127 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r127);
            let Some(t128) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087d61a); };
            c.cmp(t128, c.r32(1), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 32; continue;
        }
        106 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087d62c); };
            c.w64(7, t129);
            blk = 108; continue;
        }
        107 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(1, c.r64(9));
            blk = 108; continue;
        }
        108 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087d63b); };
            c.w64(6, t130);
            blk = 109; continue;
        }
        109 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700087d63f); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 111; continue;
        }
        110 => {
            let Some(t131) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087d64b); };
            c.w32(2, t131);
            let r132 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r132);
            let Some(t133) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d652); };
            c.w32(9, t133);
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 288; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t134) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087d664); };
            c.w32(9, t134);
            let r135 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r135);
            let r136 = c.r32(9) & 0x3fu64;
            c.w32(9, r136);
            c.logic(r136, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 147; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 131; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 123; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 120; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(9), 0x28u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 288; continue;
        }
        120 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 122; continue;
        }
        122 => {
            blk = 288; continue;
        }
        123 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_l() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 127; continue;
        }
        127 => {
            blk = 288; continue;
        }
        128 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 130; continue;
        }
        130 => {
            blk = 288; continue;
        }
        131 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 140; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 137; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 136; continue;
        }
        136 => {
            blk = 288; continue;
        }
        137 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 288; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 143; continue;
        }
        143 => {
            blk = 288; continue;
        }
        144 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 146; continue;
        }
        146 => {
            blk = 288; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_l() { blk = 165; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_l() { blk = 157; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_l() { blk = 154; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 288; continue;
        }
        154 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 156; continue;
        }
        156 => {
            blk = 288; continue;
        }
        157 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_l() { blk = 162; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0xfu64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 161; continue;
        }
        161 => {
            blk = 288; continue;
        }
        162 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 164; continue;
        }
        164 => {
            blk = 288; continue;
        }
        165 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 174; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 171; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 170; continue;
        }
        170 => {
            blk = 288; continue;
        }
        171 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 173; continue;
        }
        173 => {
            blk = 288; continue;
        }
        174 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 178; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 177; continue;
        }
        177 => {
            blk = 288; continue;
        }
        178 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 180; continue;
        }
        180 => {
            blk = 288; continue;
        }
        181 => {
            blk = 186; continue;
        }
        182 => {
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d88d); };
            c.w32(8, t137);
            let r138 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r138);
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d894); };
            c.w32(9, t139);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_ae() { blk = 187; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t140) = c.ld64(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d8ad); };
            c.cmp(t140, c.r64(9), 64);
            if c.cc_e() { blk = 205; continue; }
            blk = 184; continue;
        }
        184 => {
            blk = 242; continue;
        }
        185 => {
            let Some(t141) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d8bd); };
            c.w32(9, t141);
            let r142 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r142);
            let Some(t143) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d8c4); };
            c.w32(9, t143);
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_b() { blk = 242; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t144) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d8d1); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087d8d8); };
            c.w32(9, t146);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_b() { blk = 189; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_b() { blk = 288; continue; }
            blk = 188; continue;
        }
        188 => {
            blk = 205; continue;
        }
        189 => {
            let Some(t147) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d8fb); };
            c.w32(8, t147);
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087d900); };
            c.cmp(t148, c.r32(8), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 190; continue;
        }
        190 => {
            blk = 242; continue;
        }
        191 => {
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087d912); };
            c.w32(9, t149);
            let r150 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r150);
            let Some(t151) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d919); };
            c.w32(9, t151);
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_b() { blk = 242; continue; }
            blk = 192; continue;
        }
        192 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x700087d926); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_b() { blk = 288; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d939); };
            c.w32(8, t152);
            let r153 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r153);
            let Some(t154) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087d940); };
            c.w32(9, t154);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r155 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r155);
            c.w32(15, c.r32(1));
            c.w64(10, c.r64(4));
            let r156 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r156);
            let r157 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r157);
            c.logic(r157, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700087d95c); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x700087d961); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700087d965); }
            c.w64(2, c.r64(15));
            c.w64(1, c.r64(13));
            c.w64(3, c.r64(1));
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700087d972); };
            c.w64(0, t158);
            c.w64(10, 0x700087d980u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700087d980); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700087d984); }
            return c.call(c.r64(0), 0x700087d98a, 0x700087d988);
        }
        194 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700087d98a); }
            let Some(t159) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087d992); };
            c.w64(4, t159);
            c.w32(8, c.r32(0));
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087d99a); };
            let r161 = c.r64(8) & t160;
            c.w64(8, r161);
            c.logic(r161, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087d9a1); };
            c.w64(11, t162);
            blk = 196; continue;
        }
        195 => {
            let r163 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r163);
            let r164 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087d9ae); };
            c.w64(12, t165);
            let r166 = c.r64(12) & c.r64(8);
            c.w64(12, r166);
            c.logic(r166, 64);
            c.w64(8, c.r64(12));
            blk = 196; continue;
        }
        196 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t167) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087d9bd); };
            c.w32(12, t167);
            let r168 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087d9c5); };
            c.cmp(t169, c.r32(12), 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 197; continue;
        }
        197 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t170) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087d9d4); };
            c.cmp(t170, c.r32(12), 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 199; continue;
        }
        199 => {
            c.xorpd_rr(0, 0);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087d9e1); };
            c.cvtsi2sd(0, (t171 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700087d9e7); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 195; continue; }
            blk = 200; continue;
        }
        200 => {
            if c.cc_e() { blk = 242; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 195; continue;
        }
        202 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087da02); };
            c.cmp(c.r64(12), t172, 64);
            if c.cc_e() { blk = 242; continue; }
            blk = 203; continue;
        }
        203 => {
            blk = 195; continue;
        }
        204 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087da0e); };
            c.w64(0, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087da12); };
            c.w64(2, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087da16); };
            c.w64(1, t175);
            blk = 205; continue;
        }
        205 => {
            let Some(t176) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087da1a); };
            c.w32(8, t176);
            let r177 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087da21); };
            c.w64(6, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087da25); };
            c.w64(7, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087da29); };
            c.cmp(t180, c.r32(8), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 110; continue;
        }
        207 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087da3b); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700087da42); };
            c.logic(t183 & 0x4u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t184) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700087da4d); };
            c.cmp(t184, c.r64(1), 64);
            if c.cc_a() { blk = 242; continue; }
            blk = 209; continue;
        }
        209 => {
            blk = 243; continue;
        }
        210 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087da5c); };
            c.w32(9, t185);
            let r186 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r186);
            let Some(t187) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700087da63); };
            c.w32(11, t187);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 211; continue;
        }
        211 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t188) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087da7b); };
            c.w64(9, t188);
            let Some(t189) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087da7f); };
            c.w64(11, t189);
            let Some(t190) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700087da83); };
            c.logic(t190 & 0x1u64, 8);
            if c.cc_ne() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t191) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087da8a); };
            c.w64(12, t191);
            let r192 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r192);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t193) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700087da9a); };
            c.w64(8, t193);
            blk = 241; continue;
        }
        215 => {
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t194) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087daac); };
            c.w32(8, t194);
            let r195 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r195);
            let Some(t196) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087dab3); };
            c.w32(8, t196);
            let r197 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r197);
            let r198 = c.r32(8) & 0x3fu64;
            c.w32(8, r198);
            c.logic(r198, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 226; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 222; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 221; continue;
        }
        221 => {
            blk = 265; continue;
        }
        222 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 225; continue;
        }
        225 => {
            blk = 265; continue;
        }
        226 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 231; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 229; continue;
        }
        229 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 230; continue;
        }
        230 => {
            blk = 265; continue;
        }
        231 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            blk = 265; continue;
        }
        234 => {
            c.w32(8, 0x1u64);
            blk = 238; continue;
        }
        235 => {
            c.w32(8, 0x2u64);
            blk = 238; continue;
        }
        236 => {
            c.w32(8, 0x4u64);
            blk = 238; continue;
        }
        237 => {
            c.w32(8, 0x8u64);
            blk = 238; continue;
        }
        238 => {
            let r199 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r199);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(10));
            c.cqo();
            c.idiv(c.r64(8), 64);
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(9));
            blk = 241; continue;
        }
        239 => {
            c.w64(10, c.r64(4));
            let r200 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r200);
            let r201 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r201);
            c.logic(r201, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700087db76); }
            c.w64(2, c.r64(8));
            c.w64(1, c.r64(13));
            c.w64(9, c.r64(1));
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700087db84); };
            c.w64(0, t202);
            c.w64(10, 0x700087db92u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700087db92); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700087db96); }
            return c.call(c.r64(0), 0x700087db9c, 0x700087db9a);
        }
        240 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700087db9c); }
            let Some(t203) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087dba4); };
            c.w64(4, t203);
            c.w64(8, c.r64(0));
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dbac); };
            c.w64(0, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087dbb0); };
            c.w64(1, t205);
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_ae() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700087dbb9); };
            c.w64(8, t206);
            blk = 244; continue;
        }
        243 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087dbc2); };
            c.w64(8, t207);
            blk = 244; continue;
        }
        244 => {
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700087dbc9); };
            c.cmp(t208, c.r32(8), 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087dbd2); };
            c.w64(11, t209);
            blk = 261; continue;
        }
        246 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087dbdb); };
            c.w64(0, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dbdf); };
            c.w64(3, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087dbe3); };
            c.w64(6, t212);
            return c.call(0x7000870914, 0x700087dbec, 0x700087dbe7);
        }
        247 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087dbec); };
            if !c.push64(t213) { return RegfileExit::fault(0x700087dbec); }
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dbef); };
            if !c.push64(t214) { return RegfileExit::fault(0x700087dbef); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087dbf2); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087dbf3); };
            if !c.push64(t215) { return RegfileExit::fault(0x700087dbf3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700087dbf6); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087dbfa); };
            c.w64(6, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087dbfe); };
            c.w64(7, t217);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x700087dc0f, 0x700087dc0a);
        }
        248 => {
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087dc0f); };
            c.cmp(t218, c.r32(0), 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 249; continue;
        }
        249 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087dc23); };
            c.cmp(t219, c.r32(0), 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t220) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087dc30); };
            c.w32(8, t220);
            let r221 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r221);
            let Some(t222) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087dc37); };
            c.logic(t222 & 0x10u64, 8);
            if c.cc_ne() { blk = 260; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t223) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087dc3e); };
            c.cmp(t223, c.r32(8), 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t224) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087dc4b); };
            c.cmp(t224, 0x41u64, 16);
            if c.cc_e() { blk = 290; continue; }
            blk = 254; continue;
        }
        254 => {
            blk = 256; continue;
        }
        255 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t225) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087dc5f); };
            c.w32(8, t225);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700087dc63); }
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087dc67); };
            c.w64(0, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087dc6b); };
            c.w64(3, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087dc6f); };
            c.w64(1, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087dc73); };
            c.w64(6, t229);
            return c.call(0x7000899bb4, 0x700087dc7c, 0x700087dc77);
        }
        257 => {
            let Some(t230) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087dc7c); };
            c.logic(t230 & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 294; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087dc92); };
            c.w64(9, t231);
            let r232 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r232);
            if c.cc_o() { blk = 294; continue; }
            blk = 259; continue;
        }
        259 => {
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 261; continue;
        }
        260 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087dca5); };
            c.w64(11, t233);
            blk = 261; continue;
        }
        261 => {
            let Some(t234) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087dca9); };
            c.cmp(t234, 0x0u64, 32);
            if c.cc_e() { blk = 309; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087dcb3); };
            c.w64(8, t235);
            let r236 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r236);
            if c.cc_o() { blk = 309; continue; }
            blk = 263; continue;
        }
        263 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        264 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087dcc9); };
            c.w64(0, t237);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087dcd0); }
            return c.ret(0x18, 0x700087dcd1);
        }
        265 => {
            return RegfileExit::fault(0x700087dcd4);
        }
        266 => {
            blk = 288; continue;
        }
        267 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 288; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t238) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087dce5); };
            c.w32(9, t238);
            let r239 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r239);
            blk = 31; continue;
        }
        269 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x700087dcf1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700087dcf5); }
            blk = 273; continue;
        }
        270 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x700087dcfb); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700087dcff); }
            blk = 273; continue;
        }
        271 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x700087dd05); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700087dd09); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 273; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t240) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087dd13); };
            c.w64(12, t240);
            blk = 94; continue;
        }
        273 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700087dd1d); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087dd1f); }
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700087dd20); };
            c.w64(3, t241);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700087dd31, 0x700087dd2c);
        }
        274 => {
            let r242 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r242);
            c.w64(8, c.r64(0));
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087dd36); };
            c.w64(2, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087dd3a); };
            c.w64(9, t244);
            blk = 97; continue;
        }
        275 => {
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd43); };
            c.w64(0, t245);
            blk = 288; continue;
        }
        276 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd49); };
            c.w64(0, t246);
            blk = 288; continue;
        }
        277 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd4f); };
            c.w64(0, t247);
            blk = 288; continue;
        }
        278 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd55); };
            c.w64(0, t248);
            blk = 288; continue;
        }
        279 => {
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd5b); };
            c.w64(0, t249);
            blk = 288; continue;
        }
        280 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd61); };
            c.w64(0, t250);
            blk = 288; continue;
        }
        281 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087dd67); };
            c.w64(6, t251);
            return c.call(0x70008a1af4, 0x700087dd70, 0x700087dd6b);
        }
        282 => {
            let Some(t252) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087dd70); };
            c.w32(8, t252);
            let Some(t253) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700087dd74); };
            c.cmp(t253, c.r32(8), 32);
            if c.cc_ne() { blk = 285; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t254) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087dd7d); };
            c.logic(t254 & 0x1u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd83); };
            c.w64(0, t255);
            blk = 243; continue;
        }
        285 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dd8c); };
            c.w64(3, t256);
            blk = 286; continue;
        }
        286 => {
            c.w64(3, c.r64(0));
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087dd93); };
            c.w64(0, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087dd97); };
            c.w64(6, t258);
            return c.call(0x70008bced4, 0x700087dda0, 0x700087dd9b);
        }
        287 => {
            c.w64(8, c.r64(0));
            blk = 244; continue;
        }
        288 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087dda8); };
            if !c.push64(t259) { return RegfileExit::fault(0x700087dda8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087ddab); }
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700087ddac); };
            c.w64(3, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087ddb3); };
            c.w64(6, t261);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700087ddc4, 0x700087ddbf);
        }
        289 => {
            c.w64(8, c.r64(0));
            blk = 244; continue;
        }
        290 => {
            let Some(t262) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087ddcc); };
            c.logic(t262 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 291; continue;
        }
        291 => {
            blk = 256; continue;
        }
        292 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087ddde); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 256; continue; }
            blk = 293; continue;
        }
        293 => {
            blk = 260; continue;
        }
        294 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ddfc); };
            c.w64(11, t263);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700087de00); }
            blk = 297; continue;
        }
        296 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087de08); };
            c.w64(11, t264);
            let r265 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r265);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 297; continue;
        }
        297 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x700087de38); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 303; continue; }
            blk = 298; continue;
        }
        298 => {
            if c.cc_ne() { blk = 303; continue; }
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 303; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(9, c.r64(8));
            let r266 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r266);
            if c.cc_o() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 308; continue;
        }
        303 => {
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087de6a); };
            c.w64(8, t267);
            let Some(t268) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087de71); };
            c.w64(9, t268);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087de78); };
            c.w64(12, t269);
            let Some(t270) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087de7f); };
            c.cmp(t270, c.r64(11), 64);
            if c.cc_a() { blk = 306; continue; }
            blk = 304; continue;
        }
        304 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087de8f, 0x700087de8a);
        }
        305 => {
            blk = 307; continue;
        }
        306 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087de91); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 307; continue;
        }
        307 => {
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087de9b); };
            c.w64(8, t271);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087dea2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700087dea6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087deab); }
            blk = 308; continue;
        }
        308 => {
            c.w64(11, c.r64(0));
            blk = 261; continue;
        }
        309 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700087deb8); }
            let Some(t272) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087debc); };
            c.cmp(t272, 0x0u64, 32);
            if c.cc_ne() { blk = 311; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087dec2); };
            c.w64(0, t273);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087dec6); }
            blk = 312; continue;
        }
        311 => {
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087decd); };
            c.w64(0, t274);
            let r275 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r275);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 312; continue;
        }
        312 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 1) { return RegfileExit::fault(0x700087defb); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 318; continue; }
            blk = 313; continue;
        }
        313 => {
            if c.cc_ne() { blk = 318; continue; }
            blk = 314; continue;
        }
        314 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 316; continue; }
            blk = 315; continue;
        }
        315 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 318; continue; }
            blk = 316; continue;
        }
        316 => {
            c.w64(9, c.r64(8));
            let r276 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r276);
            if c.cc_o() { blk = 318; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        318 => {
            let Some(t277) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087df2d); };
            c.w64(8, t277);
            let Some(t278) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087df34); };
            c.w64(9, t278);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t279) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087df3b); };
            c.w64(15, t279);
            let Some(t280) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700087df42); };
            c.cmp(t280, c.r64(12), 64);
            if c.cc_a() { blk = 321; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087df51, 0x700087df4c);
        }
        320 => {
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087df51); };
            c.w64(11, t281);
            blk = 322; continue;
        }
        321 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700087df57); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 322; continue;
        }
        322 => {
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087df61); };
            c.w64(8, t282);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700087df68); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x700087df6c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087df71); }
            blk = 1; continue;
        }
        323 => {
            return RegfileExit::fault(0x700087df7b);
        }
        324 => {
            return RegfileExit::jump(0x700087df80);
        }
            _ => unreachable!(),
        }
    }
}
