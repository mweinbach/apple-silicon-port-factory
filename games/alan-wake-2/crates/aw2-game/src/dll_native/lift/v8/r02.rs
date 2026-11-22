#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000800df4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000800df4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000800df8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000800dfe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000800e02); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000800e06); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000800e0a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 959; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800e17); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800e1e); };
            c.w32(9, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000800e23); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000800e27); }
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_be() { blk = 958; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 501; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800e40); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800e47); };
            c.w32(11, t6);
            c.cmp(c.r16(11), 0x42u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000800e53); }
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
            c.w64(3, 0x3ffffffffffffeu64);
            c.cmp(c.r64(15), c.r64(3), 64);
            if c.cc_be() { blk = 502; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_ae() { blk = 961; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800ea4); };
            c.w32(12, t8);
            c.logic(c.r32(12) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
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
            c.w64(9, c.r64(1));
            blk = 27; continue;
        }
        15 => {
            c.w64(10, c.r64(4));
            let r9 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r9);
            let r10 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r10);
            c.logic(r10, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000800eee); }
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(13));
            c.w64(11, c.r64(1));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x7000800efc); };
            c.w64(0, t11);
            c.w64(10, 0x7000800f0au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000800f0a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000800f0e); }
            return c.call(c.r64(0), 0x7000800f14, 0x7000800f12);
        }
        16 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000800f14); }
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000800f1c); };
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
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800f3a); };
            c.w64(2, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800f3e); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800f42); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000800f46); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000800f4a); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000800f4e); };
            c.w64(9, t18);
            blk = 502; continue;
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800f57); };
            c.w64(3, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800f5b); };
            c.w64(0, t20);
            blk = 960; continue;
        }
        21 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800f64); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800f68); };
            c.w64(0, t22);
            blk = 960; continue;
        }
        22 => {
            c.w64(9, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800f74); };
            c.w64(2, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800f78); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800f7c); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000800f80); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000800f84); };
            c.w64(8, t27);
            blk = 27; continue;
        }
        23 => {
            c.w64(9, c.r64(1));
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800f8d); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            c.w64(1, c.r64(9));
            c.w64(9, c.r64(11));
            blk = 27; continue;
        }
        24 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        25 => {
            if let Some(sr) = c.shr(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let r30 = c.r64(12) & 0xffffffu64;
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 502; continue;
        }
        26 => {
            c.w64(9, c.r64(1));
            blk = 27; continue;
        }
        27 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800fb7); };
            c.w32(11, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000800fbb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000800fbf); }
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_ne() { blk = 325; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000800fd0); };
            c.w32(12, t32);
            let r33 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r33);
            c.logic(c.r32(11) & 0xffc00u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 29; continue;
        }
        29 => {
            let r34 = c.shr(c.r32(11), 0xau64, 32).unwrap_or(c.r32(11));
            c.w32(11, r34);
            let r35 = c.r32(11) & 0x3ffu64;
            c.w32(11, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_be() { blk = 42; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t36) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000800ff9); };
            c.w32(15, (t36 as u16 as i16 as i64 as u64));
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800fff); };
            c.w32(3, t37);
            let r38 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r38);
            let r39 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r39);
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700080100c); }
            c.w64(7, c.r64(1));
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000801015); }
            blk = 32; continue;
        }
        32 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000801019); };
            c.w64(6, t40);
            let r41 = c.sub(c.r32(6), c.r32(1), 32);
            c.w32(6, r41);
            let r42 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r42);
            let r43 = c.add(c.r32(6), c.r32(1), 32);
            c.w32(6, r43);
            c.w32(7, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000801029); };
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
            let Some(t48) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080103f); };
            c.w32(7, t48);
            let r49 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r49);
            let Some(t50) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801047); };
            c.w32(7, t50);
            let r51 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r51);
            c.cmp(c.r32(3), c.r32(7), 32);
            if c.cc_be() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(1, c.r64(6).wrapping_add(0x1));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000801054); };
            c.w64(6, t52);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 37; continue;
        }
        36 => {
            let r53 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r53);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(15), c.r32(1), 32);
            if c.cc_l() { blk = 193; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(6, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000801070); };
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
            let Some(t58) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000801086); };
            c.w32(0, t58);
            let r59 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r59);
            let Some(t60) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080108e); };
            c.w32(2, t60);
            let r61 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r61);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(11), c.r32(6), 32);
            if c.cc_be() { blk = 193; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008010aa); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008010ae); };
            c.w64(2, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008010b2); };
            c.w64(1, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008010b6); };
            c.w64(0, t65);
            blk = 46; continue;
        }
        42 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r66 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(7, r66);
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 193; continue; }
            blk = 43; continue;
        }
        43 => {
            let r67 = c.sub(c.r64(7), 0x3u64, 64);
            c.w64(7, r67);
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008010d1); };
            c.cmp(t68, c.r32(9), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 43; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 193; continue;
        }
        46 => {
            c.w64(11, c.r64(7).wrapping_mul(4));
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008010eb); };
            c.w32(15, t69);
            c.w32(3, c.r32(15));
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 447; continue; }
            blk = 47; continue;
        }
        47 => {
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 48; continue;
        }
        48 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080110d); };
            c.w32(8, t70);
            let r71 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r71);
            blk = 438; continue;
        }
        50 => {
            let r72 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r72);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t73) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000801121); };
            c.w32(11, t73);
            let r74 = c.r64(15) & 0x3ffu64;
            c.w64(15, r74);
            c.logic(r74, 64);
            let r75 = c.r32(3) & 0x380u64;
            c.w32(3, r75);
            c.logic(r75, 32);
            let r76 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r76);
            c.cmp(c.r32(3), 0x100u64, 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let Some(t77) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801144); };
            c.w32(8, t77);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t78) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080114e); };
            c.w32(15, t78);
            let r79 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r79);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700080115b); };
            c.w64(15, t80);
            blk = 53; continue;
        }
        53 => {
            let r81 = c.sub(c.r64(11), c.r64(8), 64);
            c.w64(11, r81);
            let Some(t82) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000801165); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 438; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000801177); }
            blk = 57; continue;
        }
        55 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t84) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000801185); };
            c.w32(8, t84);
            let r85 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r85);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080118d); }
            blk = 57; continue;
        }
        57 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000801193); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700080119a); };
            c.w64(11, t87);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008011a1); };
            c.w64(15, t88);
            let Some(t89) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008011a8); };
            c.cmp(t89, c.r64(12), 64);
            if c.cc_be() { blk = 964; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008011b1); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 59; continue;
        }
        59 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008011b8); };
            c.w64(11, t90);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008011bf); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008011c3); }
            blk = 438; continue;
        }
        60 => {
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008011ce); };
            c.w32(8, t91);
            let r92 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r92);
            blk = 438; continue;
        }
        61 => {
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008011db); };
            c.cmp(t93, c.r32(9), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008011e8); };
            c.cmp(t94, c.r32(9), 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008011f5); };
            c.cmp(t95, c.r32(9), 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000801202); };
            c.cmp(t96, c.r32(9), 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t97) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x700080120f); };
            c.cmp(t97, c.r32(9), 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x700080121c); };
            c.cmp(t98, c.r32(9), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000801229); };
            c.cmp(t99, c.r32(9), 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 68; continue;
        }
        68 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080123c); };
            c.cmp(t100, c.r32(0), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        71 => {
            c.w64(9, c.r64(3));
            let r101 = c.r32(9) & 0x380u64;
            c.w32(9, r101);
            c.logic(r101, 32);
            c.cmp(c.r32(9), 0x80u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(9), 0x100u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(9), 0x180u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        76 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000801292); };
            c.w32(11, t102);
            let r103 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r103);
            c.w32(12, 0x4u64);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(12, 0x2u64);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            let r104 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008012ba); };
            c.cmp(t105, c.r32(11), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        82 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        83 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        84 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        85 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t106) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008012e4); };
            c.w32(11, t106);
            let Some(t107) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008012e8); };
            c.cmp(t107, c.r32(11), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        88 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        90 => {
            let r108 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r108);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t109) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x700080130c); };
            c.w32(11, t109);
            let Some(t110) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801311); };
            c.w32(8, t110);
            let r111 = c.r32(3) & 0x8u64;
            c.w32(3, r111);
            c.logic(r111, 32);
            let r112 = c.r64(15) & 0x3ffu64;
            c.w64(15, r112);
            c.logic(r112, 64);
            c.cmp(c.r32(9), 0x100u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(3), 0x8u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r113 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r113);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 139; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t114) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801346); };
            c.w32(15, t114);
            let r115 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r115);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000801353); };
            c.w64(15, t116);
            blk = 93; continue;
        }
        93 => {
            let r117 = c.sub(c.r64(11), c.r64(8), 64);
            c.w64(11, r117);
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 95; continue;
        }
        95 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000801375); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r118 = c.r64(9) & c.r64(15);
            c.w64(9, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801387); };
            c.logic(t119 & 0x4u64, 8);
            if c.cc_ne() { blk = 1151; continue; }
            blk = 97; continue;
        }
        97 => {
            blk = 185; continue;
        }
        98 => {
            let Some(t120) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000801397); };
            c.w32(8, t120);
            let r121 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r121);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 99; continue;
        }
        99 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t122) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008013be); };
            c.w32(9, t122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008013c2); };
            c.cmp(t123, c.r32(9), 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        103 => {
            let Some(t124) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008013d3); };
            c.w32(9, t124);
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008013d7); };
            c.cmp(t125, c.r32(9), 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008013e0); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008013e6); }
            blk = 112; continue;
        }
        105 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        106 => {
            let Some(t126) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008013f5); };
            c.w32(9, t126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008013f9); };
            c.cmp(t127, c.r32(9), 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(9, c.r64(0));
            let r128 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r128);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000801408); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 112; continue;
        }
        108 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t129) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080142b); };
            c.w32(9, t129);
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080142f); };
            c.cmp(t130, c.r32(9), 32);
            if c.cc_ne() { blk = 122; continue; }
            blk = 111; continue;
        }
        111 => {
            let r131 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r131);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080143b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 112; continue;
        }
        112 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            if c.cc_e() { blk = 120; continue; }
            blk = 114; continue;
        }
        114 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            if c.cc_e() { blk = 119; continue; }
            blk = 116; continue;
        }
        116 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 185; continue; }
            blk = 117; continue;
        }
        117 => {
            if c.cc_ne() { blk = 185; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        119 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        120 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        122 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        123 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        124 => {
            let Some(t132) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008014ac); };
            c.w32(8, t132);
            let r133 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r133);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008014b8); }
            blk = 127; continue;
        }
        126 => {
            c.w64(9, c.r64(0));
            let r134 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r134);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008014d4); }
            blk = 185; continue;
        }
        129 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008014df); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            if c.cc_e() { blk = 137; continue; }
            blk = 131; continue;
        }
        131 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            if c.cc_e() { blk = 136; continue; }
            blk = 133; continue;
        }
        133 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 185; continue; }
            blk = 134; continue;
        }
        134 => {
            if c.cc_ne() { blk = 185; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        136 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        137 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        139 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000801550); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r135 = c.r64(9) & c.r64(2);
            c.w64(9, r135);
            c.logic(r135, 64);
            let Some(t136) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801562); };
            c.logic(t136 & 0x4u64, 8);
            if c.cc_ne() { blk = 1148; continue; }
            blk = 143; continue;
        }
        143 => {
            blk = 185; continue;
        }
        144 => {
            let Some(t137) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000801572); };
            c.w32(8, t137);
            let r138 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r138);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 145; continue;
        }
        145 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t139) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000801599); };
            c.w32(9, t139);
            let Some(t140) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080159d); };
            c.cmp(t140, c.r32(9), 32);
            if c.cc_e() { blk = 149; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        149 => {
            let Some(t141) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008015ae); };
            c.w32(9, t141);
            let Some(t142) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008015b2); };
            c.cmp(t142, c.r32(9), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008015bb); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008015c1); }
            blk = 158; continue;
        }
        151 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        152 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008015d0); };
            c.w32(9, t143);
            let Some(t144) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008015d4); };
            c.cmp(t144, c.r32(9), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(9, c.r64(0));
            let r145 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r145);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008015e3); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 158; continue;
        }
        154 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t146) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000801606); };
            c.w32(9, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080160a); };
            c.cmp(t147, c.r32(9), 32);
            if c.cc_ne() { blk = 168; continue; }
            blk = 157; continue;
        }
        157 => {
            let r148 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r148);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000801616); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 158; continue;
        }
        158 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            if c.cc_e() { blk = 166; continue; }
            blk = 160; continue;
        }
        160 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            if c.cc_e() { blk = 165; continue; }
            blk = 162; continue;
        }
        162 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 185; continue; }
            blk = 163; continue;
        }
        163 => {
            if c.cc_ne() { blk = 185; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        165 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        166 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        168 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        169 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        170 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080168b); }
            blk = 173; continue;
        }
        172 => {
            c.w64(8, c.r64(0));
            let r149 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r149);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 173; continue;
        }
        173 => {
            let Some(t150) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008016a1); };
            c.w32(8, t150);
            let r151 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r151);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 174; continue;
        }
        174 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008016af); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            if c.cc_e() { blk = 182; continue; }
            blk = 176; continue;
        }
        176 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            if c.cc_e() { blk = 181; continue; }
            blk = 178; continue;
        }
        178 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 184; continue; }
            blk = 179; continue;
        }
        179 => {
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        181 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        182 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        184 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008016fc); }
            blk = 185; continue;
        }
        185 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000801705); }
            return c.ret(0x0, 0x7000801706);
        }
        186 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        187 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        188 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        189 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        190 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        191 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        192 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        193 => {
            let Some(t152) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x700080173f); };
            c.cmp(t152, c.r32(9), 32);
            if c.cc_e() { blk = 324; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x700080174c); };
            c.cmp(t153, c.r32(9), 32);
            if c.cc_e() { blk = 323; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000801759); };
            c.cmp(t154, c.r32(9), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000801766); };
            c.cmp(t155, c.r32(9), 32);
            if c.cc_e() { blk = 321; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t156) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000801773); };
            c.cmp(t156, c.r32(9), 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t157) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000801780); };
            c.cmp(t157, c.r32(9), 32);
            if c.cc_e() { blk = 319; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x700080178d); };
            c.cmp(t158, c.r32(9), 32);
            if c.cc_e() { blk = 318; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(0x23)) else { return RegfileExit::fault(0x700080179a); };
            c.w32(8, t159);
            let r160 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r160);
            c.w32(11, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 317; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 316; continue; }
            blk = 202; continue;
        }
        202 => {
            let r161 = c.r32(11) & 0x3u64;
            c.w32(11, r161);
            c.logic(r161, 32);
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            let r162 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r162);
            c.logic(r162, 64);
            blk = 234; continue;
        }
        204 => {
            let Some(t163) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008017cb); };
            c.w32(11, t163);
            let Some(t164) = c.ld32(c.r64(13).wrapping_add(0x148)) else { return RegfileExit::fault(0x70008017cf); };
            c.cmp(t164, c.r32(11), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008017d8); };
            c.w64(3, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008017dc); };
            c.w64(0, t166);
            blk = 960; continue;
        }
        206 => {
            let Some(t167) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008017e5); };
            c.w32(11, t167);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            c.w8(11, (c.cc_b() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t168) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000801801); };
            c.w32(12, t168);
            let r169 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r169);
            c.w64(7, c.r64(12));
            blk = 209; continue;
        }
        208 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 315; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_be() { blk = 229; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t170) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000801827); };
            c.w32(11, t170);
            let r171 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r171);
            blk = 214; continue;
        }
        213 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 214; continue;
        }
        214 => {
            let Some(t172) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801833); };
            c.w32(12, t172);
            let r173 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r173);
            let r174 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r174);
            c.w64(0, c.r64(11));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 216; continue;
        }
        215 => {
            c.w64(0, c.r64(3));
            blk = 216; continue;
        }
        216 => {
            c.w64(3, c.r64(0));
            let r175 = c.sub(c.r32(3), c.r32(15), 32);
            c.w32(3, r175);
            let r176 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r176);
            let r177 = c.add(c.r32(3), c.r32(15), 32);
            c.w32(3, r177);
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080185b); };
            c.w32(2, t178);
            let r179 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r179);
            let Some(t180) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801863); };
            c.w32(2, t180);
            let r181 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r181);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_be() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w32(15, c.r64(3).wrapping_add(0x1));
            c.w64(3, c.r64(0));
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ne() { blk = 215; continue; }
            blk = 219; continue;
        }
        219 => {
            blk = 221; continue;
        }
        220 => {
            let r182 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r182);
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_l() { blk = 228; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(0, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t183) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080188c); };
            c.w32(3, t183);
            let r184 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r184);
            let Some(t185) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801894); };
            c.w32(2, t185);
            let r186 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r186);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r32(7), c.r32(15), 32);
            if c.cc_be() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w64(11, c.r64(0).wrapping_add(0x2));
            blk = 233; continue;
        }
        226 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008018af); };
            c.w64(3, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008018b3); };
            c.w64(0, t188);
            blk = 960; continue;
        }
        227 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008018bc); };
            c.w64(3, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008018c0); };
            c.w64(0, t190);
            blk = 960; continue;
        }
        228 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008018c9); };
            c.w64(3, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008018cd); };
            c.w64(0, t192);
            blk = 960; continue;
        }
        229 => {
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x2));
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 314; continue; }
            blk = 230; continue;
        }
        230 => {
            let r193 = c.sub(c.r64(11), 0x2u64, 64);
            c.w64(11, r193);
            let Some(t194) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008018ef); };
            c.cmp(t194, c.r32(9), 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_ne() { blk = 230; continue; }
            blk = 232; continue;
        }
        232 => {
            blk = 314; continue;
        }
        233 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000801901); };
            c.w32(8, t195);
            let r196 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r196);
            let r197 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r197);
            c.logic(r197, 64);
            blk = 234; continue;
        }
        234 => {
            let Some(t198) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080190d); };
            c.w32(11, t198);
            let r199 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r199);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000801914); }
            c.w32(12, 0);
            c.logic(0, 32);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t200) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801920); };
            c.cmp(t200, c.r32(12), 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801926); };
            c.w64(3, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080192a); };
            c.w64(0, t202);
            blk = 960; continue;
        }
        237 => {
            let Some(t203) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000801933); };
            c.w32(11, t203);
            c.logic(c.r32(11) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 313; continue; }
            blk = 238; continue;
        }
        238 => {
            if let Some(sr) = c.shr(c.r64(11), 0xau64, 64) { c.w64(11, sr); }
            let r204 = c.r64(11) & 0x3ffu64;
            c.w64(11, r204);
            c.logic(r204, 64);
            c.w64(11, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t205) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000801958); };
            c.w32(12, t205);
            let r206 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r206);
            let Some(t207) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080195f); };
            c.cmp(t207, c.r32(9), 32);
            if c.cc_ne() { blk = 312; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t208) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700080196a); };
            c.w32(15, t208);
            c.w32(0, c.r32(15));
            c.logic(c.r8(15) & 0x52u64, 8);
            if c.cc_ne() { blk = 311; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t209) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080197c); };
            c.w32(3, t209);
            let Some(t210) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000801980); };
            c.cmp(t210, c.r32(3), 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t211) = c.ld8(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000801989); };
            c.logic(t211 & 0x1u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            c.w32(3, 0x1u64);
            c.w64(7, c.r64(3));
            blk = 244; continue;
        }
        243 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 244; continue;
        }
        244 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008019ad); };
            c.w64(3, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008019b1); };
            c.w64(0, t213);
            blk = 960; continue;
        }
        246 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008019c0); };
            c.w64(2, t214);
            let Some(t215) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008019c4); };
            c.cmp(t215, c.r32(2), 32);
            if c.cc_ne() { blk = 250; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008019cb); };
            c.w64(1, t216);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008019cf); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r217 = c.r64(9) & c.r64(1);
            c.w64(9, r217);
            c.logic(r217, 64);
            let Some(t218) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008019dd); };
            c.logic(t218 & 0x4u64, 8);
            if c.cc_ne() { blk = 1145; continue; }
            blk = 249; continue;
        }
        249 => {
            blk = 310; continue;
        }
        250 => {
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008019ed); };
            c.w64(3, t219);
            c.w64(0, c.r64(2));
            blk = 960; continue;
        }
        251 => {
            let r220 = c.r32(0) & 0x380u64;
            c.w32(0, r220);
            c.logic(r220, 32);
            c.cmp(c.r32(0), 0x80u64, 32);
            if c.cc_e() { blk = 268; continue; }
            blk = 252; continue;
        }
        252 => {
            c.cmp(c.r32(0), 0x100u64, 32);
            if c.cc_e() { blk = 265; continue; }
            blk = 253; continue;
        }
        253 => {
            c.cmp(c.r32(0), 0x180u64, 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 254; continue;
        }
        254 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 270; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801a24); };
            c.w64(3, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801a28); };
            c.w64(0, t222);
            blk = 960; continue;
        }
        256 => {
            let Some(t223) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801a31); };
            c.logic(t223 & 0x1u64, 8);
            if c.cc_e() { blk = 264; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t224) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000801a37); };
            c.w32(9, t224);
            let r225 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r225);
            c.w32(11, 0x4u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 258; continue;
        }
        258 => {
            c.w32(11, 0x2u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 270; continue; }
            blk = 259; continue;
        }
        259 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 260; continue;
        }
        260 => {
            let r226 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r226);
            c.logic(r226, 64);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801a63); };
            c.w64(12, t227);
            let Some(t228) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000801a67); };
            c.cmp(t228, c.r32(9), 32);
            if c.cc_e() { blk = 270; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801a6e); };
            c.w64(3, t229);
            c.w64(0, c.r64(12));
            blk = 960; continue;
        }
        262 => {
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801a7a); };
            c.w64(3, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801a7e); };
            c.w64(0, t231);
            blk = 960; continue;
        }
        263 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801a87); };
            c.w64(3, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801a8b); };
            c.w64(0, t233);
            blk = 960; continue;
        }
        264 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801a94); };
            c.w64(3, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801a98); };
            c.w64(0, t235);
            blk = 960; continue;
        }
        265 => {
            let Some(t236) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801aa1); };
            c.logic(t236 & 0x1u64, 8);
            if c.cc_e() { blk = 270; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801aa7); };
            c.w64(12, t237);
            let Some(t238) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000801aab); };
            c.w32(9, t238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000801ab0); };
            c.cmp(t239, c.r32(9), 32);
            if c.cc_e() { blk = 270; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801ab9); };
            c.w64(3, t240);
            c.w64(0, c.r64(12));
            blk = 960; continue;
        }
        268 => {
            let Some(t241) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801ac5); };
            c.logic(t241 & 0x1u64, 8);
            if c.cc_e() { blk = 270; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801acb); };
            c.w64(3, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801acf); };
            c.w64(0, t243);
            blk = 960; continue;
        }
        270 => {
            let r244 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r244);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t245) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000801adf); };
            c.w32(9, t245);
            let Some(t246) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801ae4); };
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
            if c.cc_b() { blk = 297; continue; }
            blk = 271; continue;
        }
        271 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t249) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801b0f); };
            c.logic(t249 & 0x1u64, 8);
            if c.cc_e() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801b15); };
            c.w64(0, t250);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000801b19); }
            blk = 275; continue;
        }
        274 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801b20); };
            c.w64(0, t251);
            let r252 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r252);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 275; continue;
        }
        275 => {
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000801b2e); };
            c.w64(12, t253);
            let Some(t254) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000801b35); };
            c.w64(15, t254);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000801b3d); };
            c.w64(3, t255);
            let Some(t256) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000801b44); };
            c.cmp(t256, c.r64(0), 64);
            if c.cc_be() { blk = 966; continue; }
            blk = 276; continue;
        }
        276 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000801b4d); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 277; continue;
        }
        277 => {
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000801b55); };
            c.w64(15, t257);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000801b5c); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000801b61); }
            c.w64(0, c.r64(12));
            blk = 279; continue;
        }
        278 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801b6d); };
            c.w64(0, t258);
            blk = 279; continue;
        }
        279 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801b71); };
            c.w64(15, t259);
            let Some(t260) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801b75); };
            c.w32(12, t260);
            let r261 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r261);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000801b7c); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t262) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801b86); };
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
            blk = 282; continue;
        }
        281 => {
            let r266 = c.shl(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r266);
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000801bae); };
            c.w64(3, t267);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 282; continue;
        }
        282 => {
            let r268 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r268);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 293; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(11, c.r64(2).wrapping_add(0x3));
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000801bc7); };
            c.w64(1, t269);
            let Some(t270) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000801bce); };
            c.w64(6, t270);
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(7).wrapping_add(c.r64(6)));
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000801bdd); };
            c.w64(0, t271);
            let Some(t272) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000801be4); };
            c.cmp(t272, c.r64(8), 64);
            if c.cc_be() { blk = 968; continue; }
            blk = 284; continue;
        }
        284 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x7000801bed); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 285; continue;
        }
        285 => {
            c.w32(0, c.r32(11));
            c.w32(1, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x7000801bfb); };
            c.w64(6, t273);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000801c02); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000801c06); }
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_e() { blk = 288; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000801c1f); };
            c.w64(1, t274);
            blk = 287; continue;
        }
        287 => {
            let r275 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r275);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000801c2a); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ne() { blk = 287; continue; }
            blk = 288; continue;
        }
        288 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            let r276 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r276);
            let Some(t277) = c.ld32(c.r64(2).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000801c3d); };
            c.w32(11, t277);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000801c41); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 289; continue; }
            blk = 290; continue;
        }
        290 => {
            let r278 = c.r32(0) | c.r32(12);
            c.w32(0, r278);
            c.logic(r278, 32);
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000801c52); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000801c56); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 292; continue; }
            blk = 291; continue;
        }
        291 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r279 = c.r64(11) & c.r64(15);
            c.w64(11, r279);
            c.logic(r279, 64);
            let Some(t280) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801c6a); };
            c.logic(t280 & 0x4u64, 8);
            if c.cc_ne() { blk = 1142; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w64(3, c.r64(8));
            blk = 293; continue;
        }
        293 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000801c80); };
            c.w64(0, t281);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000801c84); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 295; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r282 = c.r64(9) & c.r64(3);
            c.w64(9, r282);
            c.logic(r282, 64);
            let Some(t283) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801c96); };
            c.logic(t283 & 0x4u64, 8);
            if c.cc_ne() { blk = 1139; continue; }
            blk = 295; continue;
        }
        295 => {
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000801ca1); };
            c.w64(8, t284);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000801ca5); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r285 = c.r64(9) & c.r64(15);
            c.w64(9, r285);
            c.logic(r285, 64);
            let Some(t286) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801cb3); };
            c.logic(t286 & 0x4u64, 8);
            if c.cc_ne() { blk = 1136; continue; }
            blk = 296; continue;
        }
        296 => {
            blk = 310; continue;
        }
        297 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 302; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801cd1); };
            c.w64(12, t287);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000801cd5); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r288 = c.r64(11) & c.r64(12);
            c.w64(11, r288);
            c.logic(r288, 64);
            let Some(t289) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801ce4); };
            c.logic(t289 & 0x4u64, 8);
            if c.cc_ne() { blk = 1133; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801cef); };
            c.w64(0, t290);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000801cf3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r291 = c.r64(8) & c.r64(12);
            c.w64(8, r291);
            c.logic(r291, 64);
            let Some(t292) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801d05); };
            c.logic(t292 & 0x4u64, 8);
            if c.cc_ne() { blk = 1130; continue; }
            blk = 301; continue;
        }
        301 => {
            blk = 310; continue;
        }
        302 => {
            let Some(t293) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801d15); };
            c.logic(t293 & 0x1u64, 8);
            if c.cc_e() { blk = 304; continue; }
            blk = 303; continue;
        }
        303 => {
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801d1b); };
            c.w64(0, t294);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000801d1f); }
            blk = 305; continue;
        }
        304 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801d26); };
            c.w64(0, t295);
            let r296 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r296);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 305; continue;
        }
        305 => {
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000801d34); };
            c.w64(11, t297);
            let Some(t298) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000801d3b); };
            c.w64(12, t298);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000801d43); };
            c.w64(0, t299);
            let Some(t300) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000801d4a); };
            c.cmp(t300, c.r64(15), 64);
            if c.cc_be() { blk = 970; continue; }
            blk = 306; continue;
        }
        306 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000801d53); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 307; continue;
        }
        307 => {
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000801d5b); };
            c.w64(12, t301);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000801d62); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000801d66); }
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801d6c); };
            c.w64(12, t302);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000801d70); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r303 = c.r64(15) & c.r64(12);
            c.w64(15, r303);
            c.logic(r303, 64);
            let Some(t304) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801d7f); };
            c.logic(t304 & 0x4u64, 8);
            if c.cc_ne() { blk = 1127; continue; }
            blk = 308; continue;
        }
        308 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x7000801d8a); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 310; continue; }
            blk = 309; continue;
        }
        309 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r305 = c.r64(8) & c.r64(12);
            c.w64(8, r305);
            c.logic(r305, 64);
            let Some(t306) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801d9e); };
            c.logic(t306 & 0x4u64, 8);
            if c.cc_ne() { blk = 1124; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801da9); };
            c.w64(0, t307);
            blk = 185; continue;
        }
        311 => {
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801db2); };
            c.w64(3, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801db6); };
            c.w64(0, t309);
            blk = 960; continue;
        }
        312 => {
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801dbf); };
            c.w64(3, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801dc3); };
            c.w64(0, t311);
            blk = 960; continue;
        }
        313 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801dcc); };
            c.w64(3, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801dd0); };
            c.w64(0, t313);
            blk = 960; continue;
        }
        314 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801dd9); };
            c.w64(3, t314);
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801ddd); };
            c.w64(0, t315);
            blk = 960; continue;
        }
        315 => {
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801de6); };
            c.w64(3, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801dea); };
            c.w64(0, t317);
            blk = 960; continue;
        }
        316 => {
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801df3); };
            c.w64(3, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801df7); };
            c.w64(0, t319);
            blk = 960; continue;
        }
        317 => {
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e00); };
            c.w64(3, t320);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e04); };
            c.w64(0, t321);
            blk = 960; continue;
        }
        318 => {
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e0d); };
            c.w64(3, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e11); };
            c.w64(0, t323);
            blk = 960; continue;
        }
        319 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e1a); };
            c.w64(3, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e1e); };
            c.w64(0, t325);
            blk = 960; continue;
        }
        320 => {
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e27); };
            c.w64(3, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e2b); };
            c.w64(0, t327);
            blk = 960; continue;
        }
        321 => {
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e34); };
            c.w64(3, t328);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e38); };
            c.w64(0, t329);
            blk = 960; continue;
        }
        322 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e41); };
            c.w64(3, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e45); };
            c.w64(0, t331);
            blk = 960; continue;
        }
        323 => {
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e4e); };
            c.w64(3, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e52); };
            c.w64(0, t333);
            blk = 960; continue;
        }
        324 => {
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801e5b); };
            c.w64(3, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000801e5f); };
            c.w64(0, t335);
            blk = 960; continue;
        }
        325 => {
            let Some(t336) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801e68); };
            c.w32(12, t336);
            let r337 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r337);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 327; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t338) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000801e75); };
            c.w64(12, t338);
            blk = 327; continue;
        }
        327 => {
            let Some(t339) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000801e7c); };
            c.w32(15, t339);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t340) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801e84); };
            c.w32(3, t340);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r341 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r341);
            let r342 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r342);
            c.w64(7, c.r64(3));
            let r343 = c.r64(7) & c.r64(15);
            c.w64(7, r343);
            c.logic(r343, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000801e98); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x7000801e9c); }
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 329; continue;
        }
        328 => {
            let r344 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r344);
            c.w64(3, c.r64(1).wrapping_add(c.r64(7)));
            c.w64(2, c.r64(15));
            let r345 = c.r64(2) & c.r64(3);
            c.w64(2, r345);
            c.logic(r345, 64);
            c.w64(7, c.r64(2));
            blk = 329; continue;
        }
        329 => {
            c.w64(6, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(6).wrapping_mul(4));
            let Some(t346) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000801ec2); };
            c.w32(3, t346);
            let Some(t347) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000801ec7); };
            c.cmp(t347, c.r32(3), 32);
            if c.cc_e() { blk = 353; continue; }
            blk = 330; continue;
        }
        330 => {
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_ne() { blk = 328; continue; }
            blk = 331; continue;
        }
        331 => {
            let Some(t348) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000801ed9); };
            c.w32(8, t348);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 447; continue; }
            blk = 332; continue;
        }
        332 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 335; continue; }
            blk = 333; continue;
        }
        333 => {
            c.w64(8, c.r64(6).wrapping_add(0x1));
            let Some(t349) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801ef2); };
            c.w32(11, t349);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 972; continue; }
            blk = 334; continue;
        }
        334 => {
            let Some(t350) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000801f06); };
            c.w32(8, t350);
            let r351 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r351);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801f0e); };
            c.w64(2, t352);
            blk = 438; continue;
        }
        335 => {
            let Some(t353) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000801f17); };
            c.cmp(t353, c.r32(9), 32);
            if c.cc_e() { blk = 352; continue; }
            blk = 336; continue;
        }
        336 => {
            let Some(t354) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000801f24); };
            c.cmp(t354, c.r32(9), 32);
            if c.cc_e() { blk = 351; continue; }
            blk = 337; continue;
        }
        337 => {
            let Some(t355) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000801f31); };
            c.cmp(t355, c.r32(9), 32);
            if c.cc_e() { blk = 350; continue; }
            blk = 338; continue;
        }
        338 => {
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000801f3e); };
            c.cmp(t356, c.r32(9), 32);
            if c.cc_e() { blk = 349; continue; }
            blk = 339; continue;
        }
        339 => {
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000801f4b); };
            c.cmp(t357, c.r32(9), 32);
            if c.cc_e() { blk = 348; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t358) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000801f54); };
            c.cmp(t358, c.r32(9), 32);
            if c.cc_e() { blk = 347; continue; }
            blk = 341; continue;
        }
        341 => {
            let Some(t359) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000801f5d); };
            c.cmp(t359, c.r32(9), 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 342; continue;
        }
        342 => {
            let Some(t360) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000801f66); };
            c.w32(8, t360);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(6).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 973; continue; }
            blk = 343; continue;
        }
        343 => {
            c.w64(8, c.r64(6).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000801f86); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 345; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r361 = c.r64(9) & c.r64(12);
            c.w64(9, r361);
            c.logic(r361, 64);
            let Some(t362) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000801f98); };
            c.logic(t362 & 0x4u64, 8);
            if c.cc_ne() { blk = 1121; continue; }
            blk = 345; continue;
        }
        345 => {
            blk = 185; continue;
        }
        346 => {
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801fa8); };
            c.w64(3, t363);
            blk = 960; continue;
        }
        347 => {
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801fb1); };
            c.w64(3, t364);
            blk = 960; continue;
        }
        348 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801fba); };
            c.w64(3, t365);
            blk = 960; continue;
        }
        349 => {
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801fc3); };
            c.w64(3, t366);
            blk = 960; continue;
        }
        350 => {
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801fcc); };
            c.w64(3, t367);
            blk = 960; continue;
        }
        351 => {
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801fd5); };
            c.w64(3, t368);
            blk = 960; continue;
        }
        352 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000801fde); };
            c.w64(3, t369);
            blk = 960; continue;
        }
        353 => {
            let Some(t370) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000801fe7); };
            c.cmp(t370, 0x82eu64, 16);
            if c.cc_e() { blk = 500; continue; }
            blk = 354; continue;
        }
        354 => {
            let Some(t371) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000801ff4); };
            c.cmp(t371, c.r32(9), 32);
            if c.cc_e() { blk = 499; continue; }
            blk = 355; continue;
        }
        355 => {
            let Some(t372) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000802001); };
            c.cmp(t372, c.r32(9), 32);
            if c.cc_e() { blk = 498; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t373) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x700080200e); };
            c.cmp(t373, c.r32(9), 32);
            if c.cc_e() { blk = 497; continue; }
            blk = 357; continue;
        }
        357 => {
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x700080201b); };
            c.cmp(t374, c.r32(9), 32);
            if c.cc_e() { blk = 496; continue; }
            blk = 358; continue;
        }
        358 => {
            let Some(t375) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000802028); };
            c.cmp(t375, c.r32(9), 32);
            if c.cc_e() { blk = 495; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t376) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000802035); };
            c.cmp(t376, c.r32(9), 32);
            if c.cc_e() { blk = 494; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t377) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000802042); };
            c.cmp(t377, c.r32(9), 32);
            if c.cc_e() { blk = 493; continue; }
            blk = 361; continue;
        }
        361 => {
            let Some(t378) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080204f); };
            c.w32(15, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000802053); };
            c.cmp(t379, c.r32(15), 32);
            if c.cc_ne() { blk = 365; continue; }
            blk = 362; continue;
        }
        362 => {
            let Some(t380) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080205c); };
            c.w32(15, t380);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 365; continue; }
            blk = 363; continue;
        }
        363 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_e() { blk = 367; continue; }
            blk = 364; continue;
        }
        364 => {
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080206c); };
            c.w64(3, t381);
            blk = 960; continue;
        }
        365 => {
            c.logic(c.r32(11) & 0x8000000u64, 32);
            if c.cc_ne() { blk = 367; continue; }
            blk = 366; continue;
        }
        366 => {
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080207e); };
            c.w64(3, t382);
            blk = 960; continue;
        }
        367 => {
            let Some(t383) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802087); };
            c.w32(15, t383);
            let r384 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r384);
            let Some(t385) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080208e); };
            c.w32(3, t385);
            let r386 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000802095); };
            c.w64(2, t387);
            blk = 369; continue;
        }
        368 => {
            let Some(t388) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080209b); };
            c.w32(3, t388);
            let r389 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r389);
            blk = 369; continue;
        }
        369 => {
            let Some(t390) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008020a2); };
            c.cmp(t390, c.r32(15), 32);
            if c.cc_e() { blk = 465; continue; }
            blk = 370; continue;
        }
        370 => {
            let Some(t391) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008020af); };
            c.w32(1, t391);
            c.cmp(c.r16(1), 0x410u64, 16);
            if c.cc_be() { blk = 399; continue; }
            blk = 371; continue;
        }
        371 => {
            let Some(t392) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008020be); };
            c.w32(6, t392);
            c.logic(c.r32(6) & 0x200000u64, 32);
            if c.cc_ne() { blk = 392; continue; }
            blk = 372; continue;
        }
        372 => {
            let Some(t393) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008020cd); };
            c.w32(7, t393);
            let r394 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r394);
            c.logic(c.r32(6) & 0xffc00u64, 32);
            if c.cc_e() { blk = 410; continue; }
            blk = 373; continue;
        }
        373 => {
            let r395 = c.shr(c.r32(6), 0xau64, 32).unwrap_or(c.r32(6));
            c.w32(6, r395);
            let r396 = c.r32(6) & 0x3ffu64;
            c.w32(6, r396);
            c.logic(r396, 32);
            c.cmp(c.r32(6), 0x20u64, 32);
            if c.cc_be() { blk = 388; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t397) = c.ld16(c.r64(7).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008020f1); };
            c.w32(12, (t397 as u16 as i16 as i64 as u64));
            let r398 = c.sub(c.r32(12), 0x1u64, 32);
            c.w32(12, r398);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008020fa); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 376; continue;
        }
        375 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000802103); }
            blk = 376; continue;
        }
        376 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000802107); };
            c.w64(11, t399);
            let r400 = c.sub(c.r32(11), c.r32(8), 32);
            c.w32(11, r400);
            let r401 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r401);
            let r402 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r402);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t403) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700080211b); };
            c.w32(0, t403);
            let r404 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r404);
            let r405 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r405);
            let r406 = c.r32(0) & 0x3ffu64;
            c.w32(0, r406);
            c.logic(r406, 32);
            c.w32(0, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t407) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080212f); };
            c.w32(0, t407);
            let r408 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r408);
            let Some(t409) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802136); };
            c.w32(0, t409);
            let r410 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r410);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_be() { blk = 378; continue; }
            blk = 377; continue;
        }
        377 => {
            c.w32(8, c.r64(11).wrapping_add(0x1));
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000802144); };
            c.w64(11, t411);
            blk = 378; continue;
        }
        378 => {
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_ne() { blk = 375; continue; }
            blk = 379; continue;
        }
        379 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700080214d); }
            blk = 381; continue;
        }
        380 => {
            let r412 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r412);
            blk = 381; continue;
        }
        381 => {
            c.cmp(c.r32(12), c.r32(8), 32);
            if c.cc_l() { blk = 387; continue; }
            blk = 382; continue;
        }
        382 => {
            c.w32(11, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t413) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000802163); };
            c.w32(11, t413);
            let r414 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r414);
            let r415 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r415);
            let r416 = c.r32(11) & 0x3ffu64;
            c.w32(11, r416);
            c.logic(r416, 32);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t417) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080217d); };
            c.w32(3, t417);
            let r418 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r418);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000802184); }
            let Some(t419) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802188); };
            c.w32(0, t419);
            let r420 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r420);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 386; continue; }
            blk = 383; continue;
        }
        383 => {
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_ne() { blk = 380; continue; }
            blk = 384; continue;
        }
        384 => {
            c.cmp(c.r32(6), c.r32(11), 32);
            if c.cc_a() { blk = 413; continue; }
            blk = 385; continue;
        }
        385 => {
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008021a0); };
            c.w64(3, t421);
            blk = 410; continue;
        }
        386 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008021a9); };
            c.w64(3, t422);
            blk = 410; continue;
        }
        387 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008021b2); };
            c.w64(3, t423);
            blk = 410; continue;
        }
        388 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            let r424 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(0, r424);
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 410; continue; }
            blk = 389; continue;
        }
        389 => {
            let r425 = c.sub(c.r64(0), 0x3u64, 64);
            c.w64(0, r425);
            let Some(t426) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008021d0); };
            c.cmp(t426, c.r32(9), 32);
            if c.cc_e() { blk = 414; continue; }
            blk = 390; continue;
        }
        390 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ne() { blk = 389; continue; }
            blk = 391; continue;
        }
        391 => {
            blk = 410; continue;
        }
        392 => {
            let Some(t427) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008021e6); };
            c.w32(8, t427);
            let r428 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r428);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 394; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008021f3); };
            c.w64(8, t429);
            blk = 394; continue;
        }
        394 => {
            let Some(t430) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008021fa); };
            c.w32(11, t430);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r431 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r431);
            c.w64(12, c.r64(2));
            let r432 = c.r64(12) & c.r64(11);
            c.w64(12, r432);
            c.logic(r432, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 396; continue;
        }
        395 => {
            let r433 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r433);
            let r434 = c.add(c.r64(12), c.r64(0), 64);
            c.w64(12, r434);
            c.w64(6, c.r64(11));
            let r435 = c.r64(6) & c.r64(12);
            c.w64(6, r435);
            c.logic(r435, 64);
            c.w64(12, c.r64(6));
            blk = 396; continue;
        }
        396 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(7, c.r64(6).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700080222f); }
            let Some(t436) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000802233); };
            c.w32(6, t436);
            let Some(t437) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000802238); };
            c.cmp(t437, c.r32(6), 32);
            if c.cc_e() { blk = 410; continue; }
            blk = 397; continue;
        }
        397 => {
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_e() { blk = 430; continue; }
            blk = 398; continue;
        }
        398 => {
            blk = 395; continue;
        }
        399 => {
            c.cmp(c.r16(1), 0xbdu64, 16);
            if c.cc_ne() { blk = 434; continue; }
            blk = 400; continue;
        }
        400 => {
            let Some(t438) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x700080225b); };
            c.logic(t438 & 0x24u64, 8);
            if c.cc_ne() { blk = 464; continue; }
            blk = 401; continue;
        }
        401 => {
            let Some(t439) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802265); };
            c.w32(8, t439);
            let r440 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r440);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700080226c); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 403; continue; }
            blk = 402; continue;
        }
        402 => {
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000802276); };
            c.w64(8, t441);
            blk = 403; continue;
        }
        403 => {
            let Some(t442) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080227d); };
            c.w32(11, t442);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r443 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r443);
            c.w64(12, c.r64(2));
            let r444 = c.r64(12) & c.r64(11);
            c.w64(12, r444);
            c.logic(r444, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 405; continue;
        }
        404 => {
            let r445 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r445);
            let r446 = c.add(c.r64(12), c.r64(0), 64);
            c.w64(12, r446);
            c.w64(15, c.r64(11));
            let r447 = c.r64(15) & c.r64(12);
            c.w64(15, r447);
            c.logic(r447, 64);
            c.w64(12, c.r64(15));
            blk = 405; continue;
        }
        405 => {
            c.w64(6, c.r64(12).wrapping_add(0x5));
            c.w64(7, c.r64(6).wrapping_mul(4));
            let Some(t448) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008022b2); };
            c.w32(15, t448);
            let r449 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r449);
            let Some(t450) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008022ba); };
            c.cmp(t450, c.r32(15), 32);
            if c.cc_e() { blk = 410; continue; }
            blk = 406; continue;
        }
        406 => {
            let Some(t451) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008022c3); };
            c.cmp(t451, c.r32(15), 32);
            if c.cc_e() { blk = 404; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t452) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008022cc); };
            c.cmp(t452, c.r32(9), 32);
            if c.cc_ne() { blk = 404; continue; }
            blk = 408; continue;
        }
        408 => {
            let Some(t453) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008022d2); };
            c.w32(11, t453);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(6).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 977; continue; }
            blk = 409; continue;
        }
        409 => {
            let Some(t454) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008022e9); };
            c.w32(8, t454);
            let r455 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r455);
            let Some(t456) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008022f1); };
            c.w32(11, t456);
            let r457 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r457);
            let Some(t458) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008022f8); };
            c.cmp(t458, c.r32(11), 32);
            if c.cc_ne() { blk = 435; continue; }
            blk = 410; continue;
        }
        410 => {
            c.cmp(c.r16(1), 0x82eu64, 16);
            if c.cc_e() { blk = 463; continue; }
            blk = 411; continue;
        }
        411 => {
            let Some(t459) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802310); };
            c.w32(15, t459);
            let r460 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r460);
            let Some(t461) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000802317); };
            c.cmp(t461, c.r32(15), 32);
            if c.cc_e() { blk = 465; continue; }
            blk = 412; continue;
        }
        412 => {
            blk = 368; continue;
        }
        413 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000802329); };
            c.w64(0, t462);
            let Some(t463) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080232d); };
            c.w64(3, t463);
            blk = 414; continue;
        }
        414 => {
            c.w64(8, c.r64(0).wrapping_mul(4));
            let Some(t464) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000802339); };
            c.w32(11, t464);
            c.w32(12, c.r32(11));
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 447; continue; }
            blk = 415; continue;
        }
        415 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 465; continue; }
            blk = 416; continue;
        }
        416 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 418; continue; }
            blk = 417; continue;
        }
        417 => {
            let Some(t465) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080235b); };
            c.w32(8, t465);
            let r466 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r466);
            blk = 429; continue;
        }
        418 => {
            let r467 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r467);
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t468) = c.ld8(c.r64(3).wrapping_add(0x4)) else { return RegfileExit::fault(0x700080236f); };
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
            let Some(t472) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802394); };
            c.w32(12, t472);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 423; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t473) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080239e); };
            c.w32(0, t473);
            let r474 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r474);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 421; continue; }
            blk = 420; continue;
        }
        420 => {
            let Some(t475) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008023a9); };
            c.w64(0, t475);
            blk = 421; continue;
        }
        421 => {
            let r476 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r476);
            let Some(t477) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008023b3); };
            c.w32(8, t477);
            let r478 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r478);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 429; continue; }
            blk = 422; continue;
        }
        422 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008023c1); }
            blk = 425; continue;
        }
        423 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 428; continue; }
            blk = 424; continue;
        }
        424 => {
            let Some(t479) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008023cf); };
            c.w32(8, t479);
            let r480 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r480);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008023d7); }
            blk = 425; continue;
        }
        425 => {
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008023dd); };
            c.w64(8, t481);
            let Some(t482) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008023e4); };
            c.w64(11, t482);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t483) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008023eb); };
            c.w64(0, t483);
            let Some(t484) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008023f2); };
            c.cmp(t484, c.r64(12), 64);
            if c.cc_be() { blk = 974; continue; }
            blk = 426; continue;
        }
        426 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008023fb); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 427; continue;
        }
        427 => {
            let Some(t485) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000802402); };
            c.w64(11, t485);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000802409); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080240d); }
            blk = 429; continue;
        }
        428 => {
            let Some(t486) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802415); };
            c.w32(8, t486);
            let r487 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r487);
            blk = 429; continue;
        }
        429 => {
            c.w64(2, c.r64(15));
            blk = 438; continue;
        }
        430 => {
            let Some(t488) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802422); };
            c.w32(11, t488);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 447; continue; }
            blk = 431; continue;
        }
        431 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 465; continue; }
            blk = 432; continue;
        }
        432 => {
            let Some(t489) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080243b); };
            c.w32(11, t489);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000802445); };
            c.w64(12, t490);
            let r491 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r491);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 976; continue; }
            blk = 433; continue;
        }
        433 => {
            let Some(t492) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000802456); };
            c.w32(8, t492);
            let r493 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r493);
            c.w64(2, c.r64(15));
            blk = 438; continue;
        }
        434 => {
            let Some(t494) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000802463); };
            c.w64(3, t494);
            let Some(t495) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000802467); };
            c.w64(0, t495);
            blk = 960; continue;
        }
        435 => {
            let Some(t496) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000802470); };
            c.w32(8, t496);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 447; continue; }
            blk = 436; continue;
        }
        436 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 465; continue; }
            blk = 437; continue;
        }
        437 => {
            let Some(t497) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000802488); };
            c.w64(2, t497);
            c.w64(8, c.r64(11));
            blk = 438; continue;
        }
        438 => {
            let Some(t498) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080248f); };
            c.w32(11, t498);
            let Some(t499) = c.ld32(c.r64(13).wrapping_add(0xd60)) else { return RegfileExit::fault(0x7000802493); };
            c.cmp(t499, c.r32(11), 32);
            if c.cc_ne() { blk = 440; continue; }
            blk = 439; continue;
        }
        439 => {
            let Some(t500) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080249c); };
            c.w64(3, t500);
            let Some(t501) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008024a0); };
            c.w64(0, t501);
            blk = 960; continue;
        }
        440 => {
            let Some(t502) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008024a9); };
            c.w32(7, t502);
            let r503 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r503);
            let Some(t504) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008024b0); };
            c.w32(8, t504);
            let r505 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r505);
            let Some(t506) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008024b7); };
            c.cmp(t506, c.r32(8), 32);
            if c.cc_e() { blk = 446; continue; }
            blk = 441; continue;
        }
        441 => {
            let Some(t507) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008024c0); };
            c.logic(t507 & 0x2u64, 8);
            if c.cc_ne() { blk = 444; continue; }
            blk = 442; continue;
        }
        442 => {
            c.w32(8, 0xbcu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008024cd); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008024cf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008024d1); }
            let Some(t508) = c.ld64(c.r64(13).wrapping_add(0x2258)) else { return RegfileExit::fault(0x70008024d2); };
            c.w64(3, t508);
            c.w32(0, 0x3u64);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008024de); };
            c.w64(6, t509);
            return c.call(0x700086ed74, 0x70008024e7, 0x70008024e2);
        }
        443 => {
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008024e7); };
            c.w64(0, t510);
            blk = 185; continue;
        }
        444 => {
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008024f0); };
            if !c.push64(t511) { return RegfileExit::fault(0x70008024f0); }
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008024f3); };
            if !c.push64(t512) { return RegfileExit::fault(0x70008024f3); }
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008024f6); };
            c.w64(6, t513);
            c.w32(0, 0x1u64);
            return c.call(0x70007f21d4, 0x7000802504, 0x70008024ff);
        }
        445 => {
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000802504); };
            c.w64(0, t514);
            blk = 185; continue;
        }
        446 => {
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080250d); };
            c.w64(3, t515);
            let Some(t516) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000802511); };
            c.w64(0, t516);
            blk = 960; continue;
        }
        447 => {
            let Some(t517) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080251a); };
            c.w64(8, t517);
            let Some(t518) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080251e); };
            c.w32(11, t518);
            let r519 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r519);
            let Some(t520) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000802525); };
            c.cmp(t520, c.r32(11), 32);
            if c.cc_e() { blk = 978; continue; }
            blk = 448; continue;
        }
        448 => {
            let Some(t521) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000802532); };
            c.w32(12, t521);
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 460; continue; }
            blk = 449; continue;
        }
        449 => {
            let Some(t522) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000802542); };
            c.w32(11, t522);
            let r523 = c.r32(11) & 0x12u64;
            c.w32(11, r523);
            c.logic(r523, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 459; continue; }
            blk = 450; continue;
        }
        450 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 452; continue; }
            blk = 451; continue;
        }
        451 => {
            let Some(t524) = c.ld64(c.r64(13).wrapping_add(0xb30)) else { return RegfileExit::fault(0x7000802557); };
            c.w64(11, t524);
            blk = 461; continue;
        }
        452 => {
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 458; continue; }
            blk = 453; continue;
        }
        453 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 457; continue; }
            blk = 454; continue;
        }
        454 => {
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 456; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t525) = c.ld64(c.r64(13).wrapping_add(0xaa8)) else { return RegfileExit::fault(0x7000802576); };
            c.w64(11, t525);
            blk = 461; continue;
        }
        456 => {
            let Some(t526) = c.ld64(c.r64(13).wrapping_add(0x550)) else { return RegfileExit::fault(0x700080257f); };
            c.w64(11, t526);
            blk = 461; continue;
        }
        457 => {
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x7000802588); };
            c.w64(11, t527);
            blk = 461; continue;
        }
        458 => {
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x918)) else { return RegfileExit::fault(0x7000802591); };
            c.w64(11, t528);
            blk = 461; continue;
        }
        459 => {
            let Some(t529) = c.ld64(c.r64(13).wrapping_add(0x730)) else { return RegfileExit::fault(0x700080259a); };
            c.w64(11, t529);
            blk = 461; continue;
        }
        460 => {
            let Some(t530) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008025a3); };
            c.w32(11, t530);
            let r531 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r531);
            blk = 461; continue;
        }
        461 => {
            c.w32(12, 0x15au64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008025b0); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008025b2); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008025b4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008025b6); }
            let Some(t532) = c.ld64(c.r64(13).wrapping_add(0x2258)) else { return RegfileExit::fault(0x70008025b8); };
            c.w64(3, t532);
            c.w32(0, 0x4u64);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008025c4); };
            c.w64(6, t533);
            return c.call(0x700086ed74, 0x70008025cd, 0x70008025c8);
        }
        462 => {
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008025cd); };
            c.w64(0, t534);
            blk = 185; continue;
        }
        463 => {
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008025d6); };
            c.w64(3, t535);
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008025da); };
            c.w64(0, t536);
            blk = 960; continue;
        }
        464 => {
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008025e3); };
            c.w64(3, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008025e7); };
            c.w64(0, t538);
            blk = 960; continue;
        }
        465 => {
            let Some(t539) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008025f0); };
            c.logic(t539 & 0x100000u64, 32);
            if c.cc_e() { blk = 469; continue; }
            blk = 466; continue;
        }
        466 => {
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008025f9); };
            c.w64(1, t540);
            let Some(t541) = c.ld32(c.r64(1).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008025fd); };
            c.w32(8, t541);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 469; continue; }
            blk = 467; continue;
        }
        467 => {
            c.w64(10, c.r64(4));
            let r542 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r542);
            let r543 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r543);
            c.logic(r543, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000802612); }
            let Some(t544) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x7000802617); };
            c.w64(0, t544);
            c.w64(10, 0x7000802625u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000802625); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000802629); }
            return c.call(c.r64(0), 0x700080262f, 0x700080262d);
        }
        468 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080262f); }
            let Some(t545) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000802637); };
            c.w64(4, t545);
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080263c); };
            c.w64(9, t546);
            blk = 469; continue;
        }
        469 => {
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000802640); };
            c.w64(12, t547);
            let Some(t548) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802644); };
            c.w32(8, t548);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t549) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080264c); };
            c.w32(11, t549);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 979; continue; }
            blk = 470; continue;
        }
        470 => {
            let Some(t550) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080265e); };
            c.w32(15, t550);
            let r551 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r551);
            c.w64(0, c.r64(15));
            let r552 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r552);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r553 = c.r64(0) & 0xfffffffffffffffeu64;
            c.w64(0, r553);
            c.logic(r553, 64);
            let r554 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r554);
            c.w32(3, c.r32(11));
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_b() { blk = 492; continue; }
            blk = 471; continue;
        }
        471 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 980; continue; }
            blk = 472; continue;
        }
        472 => {
            let r555 = c.sub(c.r32(3), c.r32(15), 32);
            c.w32(3, r555);
            let r556 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r556);
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t557) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000802694); };
            c.w32(0, t557);
            let r558 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r558);
            c.logic(r558, 64);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_b() { blk = 492; continue; }
            blk = 473; continue;
        }
        473 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 981; continue; }
            blk = 474; continue;
        }
        474 => {
            let Some(t559) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008026b0); };
            c.w32(8, t559);
            let r560 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r560);
            c.w32(11, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(11), 0xfffffeu64, 32);
            if c.cc_a() { blk = 492; continue; }
            blk = 475; continue;
        }
        475 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70008026cc); }
            let Some(t561) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008026d1); };
            c.w32(11, t561);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 982; continue; }
            blk = 476; continue;
        }
        476 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008026e9); }
            let Some(t562) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008026ee); };
            c.w32(11, t562);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r563 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r563);
            let Some(t564) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008026fd); };
            c.w32(15, t564);
            let r565 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r565);
            let r566 = c.r64(15) & c.r64(11);
            c.w64(15, r566);
            c.logic(r566, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 478; continue;
        }
        477 => {
            let r567 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r567);
            let r568 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r568);
            c.w64(3, c.r64(11));
            let r569 = c.r64(3) & c.r64(15);
            c.w64(3, r569);
            c.logic(r569, 64);
            c.w64(15, c.r64(3));
            blk = 478; continue;
        }
        478 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            c.w64(1, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t570) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000802731); };
            c.w32(6, t570);
            let Some(t571) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000802736); };
            c.cmp(t571, c.r32(6), 32);
            if c.cc_e() { blk = 480; continue; }
            blk = 479; continue;
        }
        479 => {
            let Some(t572) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080273f); };
            c.cmp(t572, c.r32(6), 32);
            if c.cc_ne() { blk = 477; continue; }
            blk = 480; continue;
        }
        480 => {
            let Some(t573) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802748); };
            c.w32(11, t573);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 983; continue; }
            blk = 481; continue;
        }
        481 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x700080275c); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 483; continue; }
            blk = 482; continue;
        }
        482 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r574 = c.r64(11) & c.r64(12);
            c.w64(11, r574);
            c.logic(r574, 64);
            let Some(t575) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802770); };
            c.logic(t575 & 0x4u64, 8);
            if c.cc_ne() { blk = 1118; continue; }
            blk = 483; continue;
        }
        483 => {
            let Some(t576) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080277b); };
            c.w32(11, t576);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 484; continue;
        }
        484 => {
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            c.w32(11, c.r32(8));
            c.w64(15, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008027a2); };
            c.w64(0, t577);
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008027a6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 486; continue; }
            blk = 485; continue;
        }
        485 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r578 = c.r64(1) & c.r64(12);
            c.w64(1, r578);
            c.logic(r578, 64);
            let Some(t579) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008027b8); };
            c.logic(t579 & 0x4u64, 8);
            if c.cc_ne() { blk = 1115; continue; }
            blk = 486; continue;
        }
        486 => {
            let Some(t580) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008027c2); };
            c.w32(15, t580);
            let Some(t581) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008027c6); };
            c.cmp(t581, c.r32(15), 32);
            if c.cc_ne() { blk = 489; continue; }
            blk = 487; continue;
        }
        487 => {
            let Some(t582) = c.ld8(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008027cf); };
            c.logic(t582 & 0x1u64, 8);
            if c.cc_e() { blk = 489; continue; }
            blk = 488; continue;
        }
        488 => {
            let r583 = c.r32(11) | 0x20u64;
            c.w32(11, r583);
            c.logic(r583, 32);
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 490; continue;
        }
        489 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 490; continue;
        }
        490 => {
            let Some(t584) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008027e2); };
            c.w32(9, t584);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(3).wrapping_add(0x2));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 985; continue; }
            blk = 491; continue;
        }
        491 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008027fa); }
            blk = 185; continue;
        }
        492 => {
            let Some(t585) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000802804); };
            c.w64(5, t585);
            let Some(t586) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x7000802808); };
            c.w64(3, t586);
            c.w32(0, 0x3u64);
            let Some(t587) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x7000802814); };
            c.w64(10, t587);
            let Some(t588) = c.ld64(c.r64(4).wrapping_add(0x58)) else { return RegfileExit::fault(0x7000802819); };
            if !c.push64(t588) { return RegfileExit::fault(0x7000802819); }
            let sp589 = c.r64(4);
            let Some(pv590) = c.ld64(sp589) else { return RegfileExit::fault(0x700080281d); };
            c.set_rsp(sp589.wrapping_add(8));
            let pea591 = c.r64(4).wrapping_add(0x70);
            if !c.st64(pea591, pv590) { c.set_rsp(sp589); return RegfileExit::fault(0x700080281d); }
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000802821); }
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(9)) { return RegfileExit::fault(0x7000802826); }
            let Some(t592) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x700080282b); };
            c.w64(10, t592);
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(10)) { return RegfileExit::fault(0x7000802830); }
            let Some(t593) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x7000802835); };
            c.w64(6, t593);
            let r594 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r594);
            return RegfileExit::jump(0x700086ed74);
        }
        493 => {
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000802843); };
            c.w64(3, t595);
            blk = 960; continue;
        }
        494 => {
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080284c); };
            c.w64(3, t596);
            blk = 960; continue;
        }
        495 => {
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000802855); };
            c.w64(3, t597);
            blk = 960; continue;
        }
        496 => {
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080285e); };
            c.w64(3, t598);
            blk = 960; continue;
        }
        497 => {
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000802867); };
            c.w64(3, t599);
            blk = 960; continue;
        }
        498 => {
            let Some(t600) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000802870); };
            c.w64(3, t600);
            blk = 960; continue;
        }
        499 => {
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000802879); };
            c.w64(3, t601);
            blk = 960; continue;
        }
        500 => {
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000802882); };
            c.w64(3, t602);
            blk = 960; continue;
        }
        501 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11));
            blk = 502; continue;
        }
        502 => {
            let Some(t603) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000802894); };
            c.w32(11, t603);
            c.w64(15, c.r64(11));
            let r604 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r604);
            c.w64(3, c.r64(2));
            let Some(t605) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008028a3); };
            c.w32(2, t605);
            let r606 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r606);
            let r607 = c.r32(15) & 0x3fu64;
            c.w32(15, r607);
            c.logic(r607, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008028ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008028b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008028b5); }
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_be() { blk = 505; continue; }
            blk = 503; continue;
        }
        503 => {
            c.cmp(c.r32(15), 0x11u64, 32);
            if c.cc_ge() { blk = 960; continue; }
            blk = 504; continue;
        }
        504 => {
            let r608 = c.r32(11) & 0xfcu64;
            c.w32(11, r608);
            c.logic(r608, 32);
            c.cmp(c.r32(11), 0x30u64, 32);
            blk = 960; continue;
        }
        505 => {
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 508; continue; }
            blk = 506; continue;
        }
        506 => {
            let Some(t609) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008028e1); };
            c.w32(9, t609);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 832; continue; }
            blk = 507; continue;
        }
        507 => {
            blk = 960; continue;
        }
        508 => {
            let Some(t610) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008028f9); };
            c.w32(9, t610);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 832; continue; }
            blk = 509; continue;
        }
        509 => {
            let Some(t611) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080290c); };
            c.w32(7, t611);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(7));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000802918); }
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_be() { blk = 960; continue; }
            blk = 510; continue;
        }
        510 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 700; continue; }
            blk = 511; continue;
        }
        511 => {
            let Some(t612) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080292e); };
            c.logic(t612 & 0x200000u64, 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 512; continue;
        }
        512 => {
            let Some(t613) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080293c); };
            c.w32(9, t613);
            let r614 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r614);
            let Some(t615) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000802943); };
            c.w32(9, t615);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 960; continue; }
            blk = 513; continue;
        }
        513 => {
            let Some(t616) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802951); };
            c.w32(9, t616);
            let Some(t617) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000802955); };
            c.cmp(t617, c.r32(9), 32);
            if c.cc_ne() { blk = 630; continue; }
            blk = 514; continue;
        }
        514 => {
            c.w64(9, c.r64(8));
            blk = 515; continue;
        }
        515 => {
            let Some(t618) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802965); };
            c.w32(9, t618);
            let r619 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r619);
            let Some(t620) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700080296c); };
            c.cmp(t620, c.r32(9), 32);
            if c.cc_e() { blk = 520; continue; }
            blk = 516; continue;
        }
        516 => {
            let Some(t621) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802975); };
            c.w32(9, t621);
            let r622 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r622);
            let Some(t623) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080297c); };
            c.cmp(t623, 0x411u64, 16);
            if c.cc_be() { blk = 960; continue; }
            blk = 517; continue;
        }
        517 => {
            let Some(t624) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000802989); };
            c.w32(11, t624);
            c.w64(7, c.r64(11));
            let r625 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r625);
            let r626 = c.r32(7) & 0x3fu64;
            c.w32(7, r626);
            c.logic(r626, 32);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_be() { blk = 515; continue; }
            blk = 518; continue;
        }
        518 => {
            let r627 = c.r32(11) & 0xfcu64;
            c.w32(11, r627);
            c.logic(r627, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 515; continue; }
            blk = 519; continue;
        }
        519 => {
            blk = 960; continue;
        }
        520 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 600; continue; }
            blk = 521; continue;
        }
        521 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_le() { blk = 539; continue; }
            blk = 522; continue;
        }
        522 => {
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008029c3); };
            c.w64(11, t628);
            let r629 = c.r32(11) & 0xfcu64;
            c.w32(11, r629);
            c.logic(r629, 32);
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_e() { blk = 536; continue; }
            blk = 523; continue;
        }
        523 => {
            let Some(t630) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008029d8); };
            c.w32(9, t630);
            let r631 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r631);
            let Some(t632) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008029df); };
            c.w32(9, t632);
            let r633 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r633);
            let Some(t634) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008029e6); };
            c.w32(11, t634);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 525; continue; }
            blk = 524; continue;
        }
        524 => {
            return RegfileExit::fault(0x70008029f6);
        }
        525 => {
            let Some(t635) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008029f7); };
            c.cmp(t635, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 526; continue;
        }
        526 => {
            c.w64(8, c.r64(3));
            let r636 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r636);
            c.logic(r636, 64);
            let Some(t637) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802a0e); };
            c.w64(15, t637);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 533; continue; }
            blk = 527; continue;
        }
        527 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 533; continue; }
            blk = 528; continue;
        }
        528 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t638) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000802a22); };
            c.w64(1, t638);
            let Some(t639) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000802a29); };
            c.w64(1, t639);
            c.w64(7, c.r64(15));
            let r640 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r640);
            c.logic(r640, 64);
            c.w64(6, c.r64(1));
            let r641 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r641);
            c.logic(r641, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 531; continue; }
            blk = 529; continue;
        }
        529 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 532; continue; }
            blk = 530; continue;
        }
        530 => {
            blk = 533; continue;
        }
        531 => {
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_be() { blk = 533; continue; }
            blk = 532; continue;
        }
        532 => {
            let Some(t642) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802a51); };
            c.w32(8, t642);
            let Some(t643) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000802a55); };
            c.cmp(t643, c.r32(8), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 533; continue;
        }
        533 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 535; continue; }
            blk = 534; continue;
        }
        534 => {
            return RegfileExit::fault(0x7000802a68);
        }
        535 => {
            let Some(t644) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000802a69); };
            c.w32(8, t644);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000802a70); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r645 = c.r64(9) & c.r64(3);
            c.w64(9, r645);
            c.logic(r645, 64);
            let Some(t646) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802a7e); };
            c.logic(t646 & 0x4u64, 8);
            if c.cc_ne() { blk = 1112; continue; }
            blk = 536; continue;
        }
        536 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(9, c.r64(12).wrapping_add(0x1));
            let r647 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r647);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000802a99); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 538; continue; }
            blk = 537; continue;
        }
        537 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r648 = c.r64(11) & c.r64(2);
            c.w64(11, r648);
            c.logic(r648, 64);
            let Some(t649) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802aab); };
            c.logic(t649 & 0x4u64, 8);
            if c.cc_ne() { blk = 1109; continue; }
            blk = 538; continue;
        }
        538 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000802ab6); }
            blk = 185; continue;
        }
        539 => {
            let Some(t650) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802abf); };
            c.w32(9, t650);
            let r651 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r651);
            let Some(t652) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802ac6); };
            c.w32(11, t652);
            let Some(t653) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000802aca); };
            c.w32(9, t653);
            let r654 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r654);
            let Some(t655) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000802ad1); };
            c.cmp(t655, c.r32(11), 32);
            if c.cc_ne() { blk = 577; continue; }
            blk = 540; continue;
        }
        540 => {
            c.w64(11, c.r64(3));
            let r656 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r656);
            c.logic(r656, 64);
            let Some(t657) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802ae8); };
            c.w64(15, t657);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 548; continue; }
            blk = 541; continue;
        }
        541 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 548; continue; }
            blk = 542; continue;
        }
        542 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t658) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000802afc); };
            c.w64(7, t658);
            let Some(t659) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000802b03); };
            c.w64(7, t659);
            c.w64(6, c.r64(15));
            let r660 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r660);
            c.logic(r660, 64);
            c.w64(0, c.r64(7));
            let r661 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r661);
            c.logic(r661, 64);
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_e() { blk = 545; continue; }
            blk = 543; continue;
        }
        543 => {
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_e() { blk = 546; continue; }
            blk = 544; continue;
        }
        544 => {
            blk = 548; continue;
        }
        545 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 548; continue; }
            blk = 546; continue;
        }
        546 => {
            let Some(t662) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802b2a); };
            c.w32(11, t662);
            let Some(t663) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000802b2e); };
            c.cmp(t663, c.r32(11), 32);
            if c.cc_ne() { blk = 548; continue; }
            blk = 547; continue;
        }
        547 => {
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000802b37); };
            c.w64(0, t664);
            blk = 960; continue;
        }
        548 => {
            let Some(t665) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802b40); };
            c.w32(11, t665);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 550; continue; }
            blk = 549; continue;
        }
        549 => {
            return RegfileExit::fault(0x7000802b50);
        }
        550 => {
            let Some(t666) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000802b51); };
            c.cmp(t666, c.r32(8), 32);
            if c.cc_ne() { blk = 554; continue; }
            blk = 551; continue;
        }
        551 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 553; continue; }
            blk = 552; continue;
        }
        552 => {
            return RegfileExit::fault(0x7000802b60);
        }
        553 => {
            let Some(t667) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000802b61); };
            c.w32(8, t667);
            blk = 560; continue;
        }
        554 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 556; continue; }
            blk = 555; continue;
        }
        555 => {
            return RegfileExit::fault(0x7000802b70);
        }
        556 => {
            let Some(t668) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000802b71); };
            c.cmp(t668, c.r32(8), 32);
            if c.cc_ne() { blk = 576; continue; }
            blk = 557; continue;
        }
        557 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 559; continue; }
            blk = 558; continue;
        }
        558 => {
            return RegfileExit::fault(0x7000802b84);
        }
        559 => {
            let Some(t669) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000802b85); };
            c.w32(8, t669);
            blk = 560; continue;
        }
        560 => {
            c.cmp(c.r64(1), 0x3fffu64, 64);
            if c.cc_ae() { blk = 575; continue; }
            blk = 561; continue;
        }
        561 => {
            c.cmp(c.r64(1), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 990; continue; }
            blk = 562; continue;
        }
        562 => {
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000802ba6); };
            c.w64(11, t670);
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 986; continue; }
            blk = 563; continue;
        }
        563 => {
            c.w32(15, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t671) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000802bc3); };
            c.w64(0, t671);
            let Some(t672) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000802bca); };
            c.w64(1, t672);
            c.w64(6, c.r64(9).wrapping_add(c.r64(1)));
            c.w64(7, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            let Some(t673) = c.ld64(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802bd9); };
            c.cmp(t673, c.r64(6), 64);
            if c.cc_be() { blk = 988; continue; }
            blk = 564; continue;
        }
        564 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x7000802be3); }
            c.w64(9, c.r64(1).wrapping_add(0x1));
            blk = 565; continue;
        }
        565 => {
            let Some(t674) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000802bea); };
            c.w64(0, t674);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000802bf1); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000802bf5); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 571; continue; }
            blk = 566; continue;
        }
        566 => {
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            blk = 567; continue;
        }
        567 => {
            let r675 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r675);
            let Some(t676) = c.ld32(c.r64(7).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000802c0b); };
            c.w32(15, t676);
            let r677 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r677);
            let Some(t678) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000802c13); };
            c.cmp(t678, c.r32(15), 32);
            if c.cc_e() { blk = 569; continue; }
            blk = 568; continue;
        }
        568 => {
            let r679 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r679);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(c.r64(9)), 0) { return RegfileExit::fault(0x7000802c28); }
            blk = 570; continue;
        }
        569 => {
            c.w64(15, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(11).wrapping_add(c.r64(9)), c.r64(15)) { return RegfileExit::fault(0x7000802c3a); }
            blk = 570; continue;
        }
        570 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 567; continue; }
            blk = 571; continue;
        }
        571 => {
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r680 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r680);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000802c4c); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 573; continue; }
            blk = 572; continue;
        }
        572 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r681 = c.r64(15) & c.r64(3);
            c.w64(15, r681);
            c.logic(r681, 64);
            let Some(t682) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802c60); };
            c.logic(t682 & 0x4u64, 8);
            if c.cc_ne() { blk = 1106; continue; }
            blk = 573; continue;
        }
        573 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000802c6b); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r683 = c.r64(9) & c.r64(3);
            c.w64(9, r683);
            c.logic(r683, 64);
            let Some(t684) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802c79); };
            c.logic(t684 & 0x4u64, 8);
            if c.cc_ne() { blk = 1103; continue; }
            blk = 574; continue;
        }
        574 => {
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000802c84); };
            c.w64(0, t685);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000802c88); }
            let Some(t686) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000802c8d); };
            c.w32(8, t686);
            let r687 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r687);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000802c9e); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000802ca5); }
            blk = 185; continue;
        }
        575 => {
            let Some(t688) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000802cae); };
            c.w64(0, t688);
            blk = 960; continue;
        }
        576 => {
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000802cb7); };
            c.w64(0, t689);
            blk = 960; continue;
        }
        577 => {
            c.w64(11, c.r64(3));
            let r690 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r690);
            c.logic(r690, 64);
            let Some(t691) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802cca); };
            c.w64(15, t691);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 584; continue; }
            blk = 578; continue;
        }
        578 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 584; continue; }
            blk = 579; continue;
        }
        579 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t692) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000802cde); };
            c.w64(1, t692);
            let Some(t693) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000802ce5); };
            c.w64(1, t693);
            c.w64(7, c.r64(15));
            let r694 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r694);
            c.logic(r694, 64);
            c.w64(6, c.r64(1));
            let r695 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r695);
            c.logic(r695, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 582; continue; }
            blk = 580; continue;
        }
        580 => {
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 583; continue; }
            blk = 581; continue;
        }
        581 => {
            blk = 584; continue;
        }
        582 => {
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_be() { blk = 584; continue; }
            blk = 583; continue;
        }
        583 => {
            let Some(t696) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802d0d); };
            c.w32(11, t696);
            let Some(t697) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000802d11); };
            c.cmp(t697, c.r32(11), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 584; continue;
        }
        584 => {
            let Some(t698) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802d1e); };
            c.w32(11, t698);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 586; continue; }
            blk = 585; continue;
        }
        585 => {
            return RegfileExit::fault(0x7000802d2e);
        }
        586 => {
            let Some(t699) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000802d2f); };
            c.cmp(t699, c.r32(8), 32);
            if c.cc_ne() { blk = 590; continue; }
            blk = 587; continue;
        }
        587 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 589; continue; }
            blk = 588; continue;
        }
        588 => {
            return RegfileExit::fault(0x7000802d3e);
        }
        589 => {
            let Some(t700) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000802d3f); };
            c.w32(8, t700);
            blk = 596; continue;
        }
        590 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 592; continue; }
            blk = 591; continue;
        }
        591 => {
            return RegfileExit::fault(0x7000802d4e);
        }
        592 => {
            let Some(t701) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000802d4f); };
            c.cmp(t701, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 593; continue;
        }
        593 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 595; continue; }
            blk = 594; continue;
        }
        594 => {
            return RegfileExit::fault(0x7000802d62);
        }
        595 => {
            let Some(t702) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000802d63); };
            c.w32(8, t702);
            blk = 596; continue;
        }
        596 => {
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r703 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r703);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000802d7a); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r704 = c.r64(12) & c.r64(3);
            c.w64(12, r704);
            c.logic(r704, 64);
            let Some(t705) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802d88); };
            c.logic(t705 & 0x4u64, 8);
            if c.cc_ne() { blk = 1100; continue; }
            blk = 597; continue;
        }
        597 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000802d94); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 599; continue; }
            blk = 598; continue;
        }
        598 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r706 = c.r64(8) & c.r64(2);
            c.w64(8, r706);
            c.logic(r706, 64);
            let Some(t707) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802da6); };
            c.logic(t707 & 0x4u64, 8);
            if c.cc_ne() { blk = 1097; continue; }
            blk = 599; continue;
        }
        599 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000802db1); }
            blk = 185; continue;
        }
        600 => {
            let Some(t708) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000802dba); };
            c.w64(11, t708);
            let r709 = c.r32(11) & 0xfcu64;
            c.w32(11, r709);
            c.logic(r709, 32);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 629; continue; }
            blk = 601; continue;
        }
        601 => {
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_e() { blk = 629; continue; }
            blk = 602; continue;
        }
        602 => {
            let Some(t710) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802dd9); };
            c.w32(9, t710);
            let r711 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r711);
            let Some(t712) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000802de0); };
            c.w32(9, t712);
            let r713 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r713);
            let Some(t714) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000802de7); };
            c.w32(11, t714);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 604; continue; }
            blk = 603; continue;
        }
        603 => {
            return RegfileExit::fault(0x7000802df7);
        }
        604 => {
            let Some(t715) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000802df8); };
            c.cmp(t715, c.r32(8), 32);
            if c.cc_ne() { blk = 616; continue; }
            blk = 605; continue;
        }
        605 => {
            c.w64(8, c.r64(3));
            let r716 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r716);
            c.logic(r716, 64);
            let Some(t717) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802e0f); };
            c.w64(15, t717);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 612; continue; }
            blk = 606; continue;
        }
        606 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 612; continue; }
            blk = 607; continue;
        }
        607 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t718) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000802e23); };
            c.w64(1, t718);
            let Some(t719) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000802e2a); };
            c.w64(1, t719);
            c.w64(7, c.r64(15));
            let r720 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r720);
            c.logic(r720, 64);
            c.w64(6, c.r64(1));
            let r721 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r721);
            c.logic(r721, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 610; continue; }
            blk = 608; continue;
        }
        608 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 611; continue; }
            blk = 609; continue;
        }
        609 => {
            blk = 612; continue;
        }
        610 => {
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_be() { blk = 612; continue; }
            blk = 611; continue;
        }
        611 => {
            let Some(t722) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802e52); };
            c.w32(8, t722);
            let Some(t723) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000802e56); };
            c.cmp(t723, c.r32(8), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 612; continue;
        }
        612 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 614; continue; }
            blk = 613; continue;
        }
        613 => {
            return RegfileExit::fault(0x7000802e69);
        }
        614 => {
            let Some(t724) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000802e6a); };
            c.w32(8, t724);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000802e71); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r725 = c.r64(9) & c.r64(3);
            c.w64(9, r725);
            c.logic(r725, 64);
            let Some(t726) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802e7f); };
            c.logic(t726 & 0x4u64, 8);
            if c.cc_ne() { blk = 1094; continue; }
            blk = 615; continue;
        }
        615 => {
            blk = 629; continue;
        }
        616 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 618; continue; }
            blk = 617; continue;
        }
        617 => {
            return RegfileExit::fault(0x7000802e95);
        }
        618 => {
            let Some(t727) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000802e96); };
            c.cmp(t727, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 619; continue;
        }
        619 => {
            c.w64(8, c.r64(3));
            let r728 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r728);
            c.logic(r728, 64);
            let Some(t729) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802ead); };
            c.w64(15, t729);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 626; continue; }
            blk = 620; continue;
        }
        620 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 626; continue; }
            blk = 621; continue;
        }
        621 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t730) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000802ec1); };
            c.w64(1, t730);
            let Some(t731) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000802ec8); };
            c.w64(1, t731);
            c.w64(7, c.r64(15));
            let r732 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r732);
            c.logic(r732, 64);
            c.w64(6, c.r64(1));
            let r733 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r733);
            c.logic(r733, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 624; continue; }
            blk = 622; continue;
        }
        622 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 625; continue; }
            blk = 623; continue;
        }
        623 => {
            blk = 626; continue;
        }
        624 => {
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_be() { blk = 626; continue; }
            blk = 625; continue;
        }
        625 => {
            let Some(t734) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802ef0); };
            c.w32(8, t734);
            let Some(t735) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000802ef4); };
            c.cmp(t735, c.r32(8), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 626; continue;
        }
        626 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 628; continue; }
            blk = 627; continue;
        }
        627 => {
            return RegfileExit::fault(0x7000802f07);
        }
        628 => {
            let Some(t736) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000802f08); };
            c.w32(8, t736);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000802f0f); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r737 = c.r64(9) & c.r64(3);
            c.w64(9, r737);
            c.logic(r737, 64);
            let Some(t738) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802f1d); };
            c.logic(t738 & 0x4u64, 8);
            if c.cc_ne() { blk = 1091; continue; }
            blk = 629; continue;
        }
        629 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            let r739 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r739);
            if !c.st32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000802f30); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000802f35); }
            blk = 185; continue;
        }
        630 => {
            let Some(t740) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000802f3e); };
            c.cmp(t740, c.r32(9), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 631; continue;
        }
        631 => {
            c.w64(9, c.r64(8));
            blk = 632; continue;
        }
        632 => {
            let Some(t741) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000802f4e); };
            c.w32(9, t741);
            let r742 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r742);
            let Some(t743) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000802f55); };
            c.cmp(t743, c.r32(9), 32);
            if c.cc_e() { blk = 637; continue; }
            blk = 633; continue;
        }
        633 => {
            let Some(t744) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802f5e); };
            c.w32(9, t744);
            let r745 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r745);
            let Some(t746) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000802f65); };
            c.cmp(t746, 0x411u64, 16);
            if c.cc_be() { blk = 960; continue; }
            blk = 634; continue;
        }
        634 => {
            let Some(t747) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000802f72); };
            c.w32(15, t747);
            c.w64(11, c.r64(15));
            let r748 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r748);
            let r749 = c.r32(11) & 0x3fu64;
            c.w32(11, r749);
            c.logic(r749, 32);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_be() { blk = 632; continue; }
            blk = 635; continue;
        }
        635 => {
            let r750 = c.r32(15) & 0xfcu64;
            c.w32(15, r750);
            c.logic(r750, 32);
            c.cmp(c.r32(15), 0xa0u64, 32);
            if c.cc_e() { blk = 632; continue; }
            blk = 636; continue;
        }
        636 => {
            blk = 960; continue;
        }
        637 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 684; continue; }
            blk = 638; continue;
        }
        638 => {
            let Some(t751) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802fa5); };
            c.w32(9, t751);
            let Some(t752) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000802fa9); };
            c.cmp(t752, c.r32(9), 32);
            if c.cc_ne() { blk = 640; continue; }
            blk = 639; continue;
        }
        639 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000802fb2); }
            blk = 685; continue;
        }
        640 => {
            let Some(t753) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000802fbc); };
            c.w32(9, t753);
            let r754 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r754);
            c.w64(11, c.r64(3));
            let r755 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r755);
            c.logic(r755, 64);
            let Some(t756) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000802fcd); };
            c.w32(9, t756);
            let r757 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r757);
            let Some(t758) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000802fd4); };
            c.w64(15, t758);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 648; continue; }
            blk = 641; continue;
        }
        641 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 648; continue; }
            blk = 642; continue;
        }
        642 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t759) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000802fe8); };
            c.w64(6, t759);
            let Some(t760) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000802fef); };
            c.w64(6, t760);
            c.w64(0, c.r64(15));
            let r761 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r761);
            c.logic(r761, 64);
            c.w64(12, c.r64(6));
            let r762 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r762);
            c.logic(r762, 64);
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_e() { blk = 645; continue; }
            blk = 643; continue;
        }
        643 => {
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_e() { blk = 646; continue; }
            blk = 644; continue;
        }
        644 => {
            blk = 648; continue;
        }
        645 => {
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_be() { blk = 648; continue; }
            blk = 646; continue;
        }
        646 => {
            let Some(t763) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803016); };
            c.w32(11, t763);
            let Some(t764) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700080301a); };
            c.cmp(t764, c.r32(11), 32);
            if c.cc_ne() { blk = 648; continue; }
            blk = 647; continue;
        }
        647 => {
            let Some(t765) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000803023); };
            c.w64(0, t765);
            blk = 960; continue;
        }
        648 => {
            let Some(t766) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080302c); };
            c.w32(11, t766);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 650; continue; }
            blk = 649; continue;
        }
        649 => {
            return RegfileExit::fault(0x700080303c);
        }
        650 => {
            let Some(t767) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x700080303d); };
            c.cmp(t767, c.r32(8), 32);
            if c.cc_ne() { blk = 654; continue; }
            blk = 651; continue;
        }
        651 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 653; continue; }
            blk = 652; continue;
        }
        652 => {
            return RegfileExit::fault(0x700080304c);
        }
        653 => {
            let Some(t768) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700080304d); };
            c.w32(8, t768);
            blk = 660; continue;
        }
        654 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 656; continue; }
            blk = 655; continue;
        }
        655 => {
            return RegfileExit::fault(0x700080305c);
        }
        656 => {
            let Some(t769) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x700080305d); };
            c.cmp(t769, c.r32(8), 32);
            if c.cc_ne() { blk = 683; continue; }
            blk = 657; continue;
        }
        657 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 659; continue; }
            blk = 658; continue;
        }
        658 => {
            return RegfileExit::fault(0x7000803070);
        }
        659 => {
            let Some(t770) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000803071); };
            c.w32(8, t770);
            blk = 660; continue;
        }
        660 => {
            c.cmp(c.r64(1), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 682; continue; }
            blk = 661; continue;
        }
        661 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 996; continue; }
            blk = 662; continue;
        }
        662 => {
            c.w64(9, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            let Some(t771) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080309a); };
            c.w64(11, t771);
            let Some(t772) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008030a1); };
            c.w64(12, t772);
            c.w64(15, c.r64(9).wrapping_add(c.r64(12)));
            let Some(t773) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008030a8); };
            c.w64(0, t773);
            let Some(t774) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008030af); };
            c.cmp(t774, c.r64(15), 64);
            if c.cc_be() { blk = 992; continue; }
            blk = 663; continue;
        }
        663 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008030b8); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 664; continue;
        }
        664 => {
            c.w32(11, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t775) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008030c4); };
            c.w64(12, t775);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008030cb); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008030cf); }
            c.w64(11, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 667; continue; }
            blk = 665; continue;
        }
        665 => {
            let Some(t776) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008030e1); };
            c.w64(12, t776);
            blk = 666; continue;
        }
        666 => {
            let r777 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r777);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x70008030ec); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 666; continue; }
            blk = 667; continue;
        }
        667 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 676; continue; }
            blk = 668; continue;
        }
        668 => {
            c.w64(12, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t778) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x7000803110); };
            c.w64(15, t778);
            blk = 670; continue;
        }
        669 => {
            let r779 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r779);
            let Some(t780) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080311d); };
            c.w64(2, t780);
            blk = 670; continue;
        }
        670 => {
            let r781 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r781);
            let Some(t782) = c.ld64(c.r64(11).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000803125); };
            c.cmp(t782, c.r64(15), 64);
            if c.cc_e() { blk = 675; continue; }
            blk = 671; continue;
        }
        671 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(2))) { return RegfileExit::fault(0x700080312b); }
            let Some(t783) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803131); };
            c.w64(0, t783);
            let Some(t784) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000803138); };
            c.w64(1, t784);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            c.w64(7, c.r64(12).wrapping_sub(0x4));
            let Some(t785) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000803144); };
            c.w64(2, t785);
            let Some(t786) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700080314b); };
            c.cmp(t786, c.r64(6), 64);
            if c.cc_be() { blk = 994; continue; }
            blk = 672; continue;
        }
        672 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x7000803154); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 673; continue;
        }
        673 => {
            let Some(t787) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080315b); };
            c.w64(2, t787);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000803162); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000803165); }
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700080316a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 675; continue; }
            blk = 674; continue;
        }
        674 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r788 = c.r64(2) & c.r64(9);
            c.w64(2, r788);
            c.logic(r788, 64);
            let Some(t789) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080317c); };
            c.logic(t789 & 0x4u64, 8);
            if c.cc_ne() { blk = 1088; continue; }
            blk = 675; continue;
        }
        675 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 669; continue; }
            blk = 676; continue;
        }
        676 => {
            let Some(t790) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080318c); };
            c.w64(12, t790);
            c.w64(11, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let r791 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r791);
            let r792 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r792);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700080319f); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 678; continue; }
            blk = 677; continue;
        }
        677 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r793 = c.r64(15) & c.r64(3);
            c.w64(15, r793);
            c.logic(r793, 64);
            let Some(t794) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008031b3); };
            c.logic(t794 & 0x4u64, 8);
            if c.cc_ne() { blk = 1085; continue; }
            blk = 678; continue;
        }
        678 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008031be); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r795 = c.r64(9) & c.r64(3);
            c.w64(9, r795);
            c.logic(r795, 64);
            let Some(t796) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008031cc); };
            c.logic(t796 & 0x4u64, 8);
            if c.cc_ne() { blk = 1082; continue; }
            blk = 679; continue;
        }
        679 => {
            let Some(t797) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008031d7); };
            c.w32(8, t797);
            let r798 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r798);
            let Some(t799) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008031de); };
            c.w64(0, t799);
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008031e2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 681; continue; }
            blk = 680; continue;
        }
        680 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r800 = c.r64(9) & c.r64(8);
            c.w64(9, r800);
            c.logic(r800, 64);
            let Some(t801) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008031f4); };
            c.logic(t801 & 0x4u64, 8);
            if c.cc_ne() { blk = 1079; continue; }
            blk = 681; continue;
        }
        681 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008031ff); }
            blk = 185; continue;
        }
        682 => {
            let Some(t802) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000803208); };
            c.w64(0, t802);
            blk = 960; continue;
        }
        683 => {
            let Some(t803) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000803211); };
            c.w64(0, t803);
            blk = 960; continue;
        }
        684 => {
            c.w64(9, c.r64(0));
            let r804 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r804);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 685; continue;
        }
        685 => {
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000803229); };
            c.w64(11, t805);
            let r806 = c.r32(11) & 0xfcu64;
            c.w32(11, r806);
            c.logic(r806, 32);
            c.cmp(c.r32(11), 0x14u64, 32);
            if c.cc_e() { blk = 699; continue; }
            blk = 686; continue;
        }
        686 => {
            let Some(t807) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080323e); };
            c.w32(9, t807);
            let r808 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r808);
            let Some(t809) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000803245); };
            c.w32(9, t809);
            let r810 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r810);
            let Some(t811) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080324c); };
            c.w32(11, t811);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 688; continue; }
            blk = 687; continue;
        }
        687 => {
            return RegfileExit::fault(0x700080325c);
        }
        688 => {
            let Some(t812) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x700080325d); };
            c.cmp(t812, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 689; continue;
        }
        689 => {
            c.w64(8, c.r64(3));
            let r813 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r813);
            c.logic(r813, 64);
            let Some(t814) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803274); };
            c.w64(15, t814);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 696; continue; }
            blk = 690; continue;
        }
        690 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 696; continue; }
            blk = 691; continue;
        }
        691 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t815) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803288); };
            c.w64(1, t815);
            let Some(t816) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700080328f); };
            c.w64(1, t816);
            c.w64(7, c.r64(15));
            let r817 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r817);
            c.logic(r817, 64);
            c.w64(6, c.r64(1));
            let r818 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r818);
            c.logic(r818, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 694; continue; }
            blk = 692; continue;
        }
        692 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 695; continue; }
            blk = 693; continue;
        }
        693 => {
            blk = 696; continue;
        }
        694 => {
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_be() { blk = 696; continue; }
            blk = 695; continue;
        }
        695 => {
            let Some(t819) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008032b7); };
            c.w32(8, t819);
            let Some(t820) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008032bb); };
            c.cmp(t820, c.r32(8), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 696; continue;
        }
        696 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 698; continue; }
            blk = 697; continue;
        }
        697 => {
            return RegfileExit::fault(0x70008032ce);
        }
        698 => {
            let Some(t821) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008032cf); };
            c.w32(8, t821);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008032d6); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r822 = c.r64(9) & c.r64(3);
            c.w64(9, r822);
            c.logic(r822, 64);
            let Some(t823) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008032e4); };
            c.logic(t823 & 0x4u64, 8);
            if c.cc_ne() { blk = 1076; continue; }
            blk = 699; continue;
        }
        699 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let r824 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r824);
            c.w64(9, c.r64(2));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000803304); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700080330b); }
            blk = 185; continue;
        }
        700 => {
            let Some(t825) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000803314); };
            c.logic(t825 & 0x200000u64, 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 701; continue;
        }
        701 => {
            let Some(t826) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000803322); };
            c.w32(9, t826);
            let r827 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r827);
            let Some(t828) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000803329); };
            c.w32(9, t828);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 960; continue; }
            blk = 702; continue;
        }
        702 => {
            let Some(t829) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803337); };
            c.w32(9, t829);
            let Some(t830) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080333b); };
            c.cmp(t830, c.r32(9), 32);
            if c.cc_ne() { blk = 776; continue; }
            blk = 703; continue;
        }
        703 => {
            c.w64(9, c.r64(8));
            blk = 704; continue;
        }
        704 => {
            let Some(t831) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080334b); };
            c.w32(9, t831);
            let r832 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r832);
            let Some(t833) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000803352); };
            c.cmp(t833, c.r32(9), 32);
            if c.cc_e() { blk = 709; continue; }
            blk = 705; continue;
        }
        705 => {
            let Some(t834) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080335b); };
            c.w32(9, t834);
            let r835 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r835);
            let Some(t836) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000803362); };
            c.cmp(t836, 0x411u64, 16);
            if c.cc_be() { blk = 960; continue; }
            blk = 706; continue;
        }
        706 => {
            let Some(t837) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080336f); };
            c.w32(11, t837);
            c.w64(7, c.r64(11));
            let r838 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r838);
            let r839 = c.r32(7) & 0x3fu64;
            c.w32(7, r839);
            c.logic(r839, 32);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_be() { blk = 704; continue; }
            blk = 707; continue;
        }
        707 => {
            let r840 = c.r32(11) & 0xfcu64;
            c.w32(11, r840);
            c.logic(r840, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 704; continue; }
            blk = 708; continue;
        }
        708 => {
            blk = 960; continue;
        }
        709 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 775; continue; }
            blk = 710; continue;
        }
        710 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_le() { blk = 714; continue; }
            blk = 711; continue;
        }
        711 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(9, c.r64(12).wrapping_add(0x1));
            let r841 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r841);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008033b5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 713; continue; }
            blk = 712; continue;
        }
        712 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r842 = c.r64(11) & c.r64(2);
            c.w64(11, r842);
            c.logic(r842, 64);
            let Some(t843) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008033c7); };
            c.logic(t843 & 0x4u64, 8);
            if c.cc_ne() { blk = 1073; continue; }
            blk = 713; continue;
        }
        713 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008033d2); }
            blk = 185; continue;
        }
        714 => {
            let Some(t844) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008033db); };
            c.w32(9, t844);
            let r845 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r845);
            let Some(t846) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008033e2); };
            c.w32(11, t846);
            let Some(t847) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008033e6); };
            c.w32(9, t847);
            let r848 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r848);
            let Some(t849) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008033ed); };
            c.cmp(t849, c.r32(11), 32);
            if c.cc_ne() { blk = 752; continue; }
            blk = 715; continue;
        }
        715 => {
            c.w64(11, c.r64(3));
            let r850 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r850);
            c.logic(r850, 64);
            let Some(t851) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803404); };
            c.w64(15, t851);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 723; continue; }
            blk = 716; continue;
        }
        716 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 723; continue; }
            blk = 717; continue;
        }
        717 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t852) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803418); };
            c.w64(7, t852);
            let Some(t853) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080341f); };
            c.w64(7, t853);
            c.w64(6, c.r64(15));
            let r854 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r854);
            c.logic(r854, 64);
            c.w64(0, c.r64(7));
            let r855 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r855);
            c.logic(r855, 64);
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_e() { blk = 720; continue; }
            blk = 718; continue;
        }
        718 => {
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_e() { blk = 721; continue; }
            blk = 719; continue;
        }
        719 => {
            blk = 723; continue;
        }
        720 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 723; continue; }
            blk = 721; continue;
        }
        721 => {
            let Some(t856) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803446); };
            c.w32(11, t856);
            let Some(t857) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700080344a); };
            c.cmp(t857, c.r32(11), 32);
            if c.cc_ne() { blk = 723; continue; }
            blk = 722; continue;
        }
        722 => {
            let Some(t858) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000803453); };
            c.w64(0, t858);
            blk = 960; continue;
        }
        723 => {
            let Some(t859) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080345c); };
            c.w32(11, t859);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 725; continue; }
            blk = 724; continue;
        }
        724 => {
            return RegfileExit::fault(0x700080346c);
        }
        725 => {
            let Some(t860) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x700080346d); };
            c.cmp(t860, c.r32(8), 32);
            if c.cc_ne() { blk = 729; continue; }
            blk = 726; continue;
        }
        726 => {
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 728; continue; }
            blk = 727; continue;
        }
        727 => {
            return RegfileExit::fault(0x700080347c);
        }
        728 => {
            let Some(t861) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x700080347d); };
            c.w32(8, t861);
            blk = 735; continue;
        }
        729 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 731; continue; }
            blk = 730; continue;
        }
        730 => {
            return RegfileExit::fault(0x700080348c);
        }
        731 => {
            let Some(t862) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x700080348d); };
            c.cmp(t862, c.r32(8), 32);
            if c.cc_ne() { blk = 751; continue; }
            blk = 732; continue;
        }
        732 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 734; continue; }
            blk = 733; continue;
        }
        733 => {
            return RegfileExit::fault(0x70008034a0);
        }
        734 => {
            let Some(t863) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008034a1); };
            c.w32(8, t863);
            blk = 735; continue;
        }
        735 => {
            c.cmp(c.r64(1), 0x3fffu64, 64);
            if c.cc_ae() { blk = 750; continue; }
            blk = 736; continue;
        }
        736 => {
            c.cmp(c.r64(1), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 1002; continue; }
            blk = 737; continue;
        }
        737 => {
            let Some(t864) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008034c2); };
            c.w64(11, t864);
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 998; continue; }
            blk = 738; continue;
        }
        738 => {
            let Some(t865) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008034db); };
            c.w64(15, t865);
            let Some(t866) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008034e2); };
            c.w64(0, t866);
            c.w64(6, c.r64(9).wrapping_add(c.r64(0)));
            let Some(t867) = c.ld64(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008034e9); };
            c.cmp(t867, c.r64(6), 64);
            if c.cc_be() { blk = 1000; continue; }
            blk = 739; continue;
        }
        739 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x70008034f3); }
            c.w64(9, c.r64(0).wrapping_add(0x1));
            blk = 740; continue;
        }
        740 => {
            c.w32(15, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t868) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008034fe); };
            c.w64(0, t868);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000803505); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000803509); }
            c.w64(15, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 746; continue; }
            blk = 741; continue;
        }
        741 => {
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            blk = 742; continue;
        }
        742 => {
            let r869 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r869);
            let Some(t870) = c.ld32(c.r64(15).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000803527); };
            c.w32(0, t870);
            let r871 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r871);
            let Some(t872) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080352f); };
            c.cmp(t872, c.r32(0), 32);
            if c.cc_e() { blk = 744; continue; }
            blk = 743; continue;
        }
        743 => {
            let r873 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r873);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(c.r64(9)), 0) { return RegfileExit::fault(0x7000803542); }
            blk = 745; continue;
        }
        744 => {
            let Some(t874) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x700080354a); };
            c.w64(0, t874);
            if !c.st64(c.r64(11).wrapping_add(c.r64(9)), c.r64(0)) { return RegfileExit::fault(0x7000803551); }
            blk = 745; continue;
        }
        745 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 742; continue; }
            blk = 746; continue;
        }
        746 => {
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r875 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r875);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000803563); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 748; continue; }
            blk = 747; continue;
        }
        747 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r876 = c.r64(15) & c.r64(3);
            c.w64(15, r876);
            c.logic(r876, 64);
            let Some(t877) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803577); };
            c.logic(t877 & 0x4u64, 8);
            if c.cc_ne() { blk = 1070; continue; }
            blk = 748; continue;
        }
        748 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000803582); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r878 = c.r64(9) & c.r64(3);
            c.w64(9, r878);
            c.logic(r878, 64);
            let Some(t879) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803590); };
            c.logic(t879 & 0x4u64, 8);
            if c.cc_ne() { blk = 1067; continue; }
            blk = 749; continue;
        }
        749 => {
            let Some(t880) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080359b); };
            c.w64(0, t880);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080359f); }
            let Some(t881) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008035a4); };
            c.w32(8, t881);
            let r882 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r882);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x70008035b5); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008035bc); }
            blk = 185; continue;
        }
        750 => {
            let Some(t883) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008035c5); };
            c.w64(0, t883);
            blk = 960; continue;
        }
        751 => {
            let Some(t884) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008035ce); };
            c.w64(0, t884);
            blk = 960; continue;
        }
        752 => {
            c.w64(11, c.r64(3));
            let r885 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r885);
            c.logic(r885, 64);
            let Some(t886) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008035e1); };
            c.w64(15, t886);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 759; continue; }
            blk = 753; continue;
        }
        753 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 759; continue; }
            blk = 754; continue;
        }
        754 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t887) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008035f5); };
            c.w64(1, t887);
            let Some(t888) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008035fc); };
            c.w64(1, t888);
            c.w64(7, c.r64(15));
            let r889 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r889);
            c.logic(r889, 64);
            c.w64(6, c.r64(1));
            let r890 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r890);
            c.logic(r890, 64);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_e() { blk = 757; continue; }
            blk = 755; continue;
        }
        755 => {
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 758; continue; }
            blk = 756; continue;
        }
        756 => {
            blk = 759; continue;
        }
        757 => {
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_be() { blk = 759; continue; }
            blk = 758; continue;
        }
        758 => {
            let Some(t891) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803624); };
            c.w32(11, t891);
            let Some(t892) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000803628); };
            c.cmp(t892, c.r32(11), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 759; continue;
        }
        759 => {
            let Some(t893) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000803635); };
            c.w32(11, t893);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 761; continue; }
            blk = 760; continue;
        }
        760 => {
            return RegfileExit::fault(0x7000803645);
        }
        761 => {
            let Some(t894) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000803646); };
            c.cmp(t894, c.r32(8), 32);
            if c.cc_ne() { blk = 765; continue; }
            blk = 762; continue;
        }
        762 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 764; continue; }
            blk = 763; continue;
        }
        763 => {
            return RegfileExit::fault(0x7000803655);
        }
        764 => {
            let Some(t895) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000803656); };
            c.w32(8, t895);
            blk = 771; continue;
        }
        765 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 767; continue; }
            blk = 766; continue;
        }
        766 => {
            return RegfileExit::fault(0x7000803665);
        }
        767 => {
            let Some(t896) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000803666); };
            c.cmp(t896, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 768; continue;
        }
        768 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 770; continue; }
            blk = 769; continue;
        }
        769 => {
            return RegfileExit::fault(0x7000803679);
        }
        770 => {
            let Some(t897) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700080367a); };
            c.w32(8, t897);
            blk = 771; continue;
        }
        771 => {
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_add(0x1));
            let r898 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r898);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000803691); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r899 = c.r64(12) & c.r64(3);
            c.w64(12, r899);
            c.logic(r899, 64);
            let Some(t900) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080369f); };
            c.logic(t900 & 0x4u64, 8);
            if c.cc_ne() { blk = 1064; continue; }
            blk = 772; continue;
        }
        772 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x70008036ab); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 774; continue; }
            blk = 773; continue;
        }
        773 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r901 = c.r64(8) & c.r64(2);
            c.w64(8, r901);
            c.logic(r901, 64);
            let Some(t902) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008036bd); };
            c.logic(t902 & 0x4u64, 8);
            if c.cc_ne() { blk = 1061; continue; }
            blk = 774; continue;
        }
        774 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008036c8); }
            blk = 185; continue;
        }
        775 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            let r903 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r903);
            if !c.st32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008036d9); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008036de); }
            blk = 185; continue;
        }
        776 => {
            let Some(t904) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008036e7); };
            c.cmp(t904, c.r32(9), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 777; continue;
        }
        777 => {
            c.w64(9, c.r64(8));
            blk = 778; continue;
        }
        778 => {
            let Some(t905) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008036f7); };
            c.w32(9, t905);
            let r906 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r906);
            let Some(t907) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008036fe); };
            c.cmp(t907, c.r32(9), 32);
            if c.cc_e() { blk = 783; continue; }
            blk = 779; continue;
        }
        779 => {
            let Some(t908) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803707); };
            c.w32(9, t908);
            let r909 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r909);
            let Some(t910) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080370e); };
            c.cmp(t910, 0x411u64, 16);
            if c.cc_be() { blk = 960; continue; }
            blk = 780; continue;
        }
        780 => {
            let Some(t911) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080371b); };
            c.w32(11, t911);
            c.w64(15, c.r64(11));
            let r912 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r912);
            let r913 = c.r32(15) & 0x3fu64;
            c.w32(15, r913);
            c.logic(r913, 32);
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_be() { blk = 778; continue; }
            blk = 781; continue;
        }
        781 => {
            let r914 = c.r32(11) & 0xfcu64;
            c.w32(11, r914);
            c.logic(r914, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 778; continue; }
            blk = 782; continue;
        }
        782 => {
            blk = 960; continue;
        }
        783 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 830; continue; }
            blk = 784; continue;
        }
        784 => {
            let Some(t915) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080374e); };
            c.w32(9, t915);
            let Some(t916) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000803752); };
            c.cmp(t916, c.r32(9), 32);
            if c.cc_ne() { blk = 786; continue; }
            blk = 785; continue;
        }
        785 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080375b); }
            blk = 831; continue;
        }
        786 => {
            let Some(t917) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803765); };
            c.w32(9, t917);
            let r918 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r918);
            c.w64(11, c.r64(3));
            let r919 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r919);
            c.logic(r919, 64);
            let Some(t920) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000803776); };
            c.w32(9, t920);
            let r921 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r921);
            let Some(t922) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080377d); };
            c.w64(15, t922);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 794; continue; }
            blk = 787; continue;
        }
        787 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 794; continue; }
            blk = 788; continue;
        }
        788 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t923) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803791); };
            c.w64(6, t923);
            let Some(t924) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000803798); };
            c.w64(6, t924);
            c.w64(0, c.r64(15));
            let r925 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r925);
            c.logic(r925, 64);
            c.w64(12, c.r64(6));
            let r926 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r926);
            c.logic(r926, 64);
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_e() { blk = 791; continue; }
            blk = 789; continue;
        }
        789 => {
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_e() { blk = 792; continue; }
            blk = 790; continue;
        }
        790 => {
            blk = 794; continue;
        }
        791 => {
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_be() { blk = 794; continue; }
            blk = 792; continue;
        }
        792 => {
            let Some(t927) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008037bf); };
            c.w32(11, t927);
            let Some(t928) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008037c3); };
            c.cmp(t928, c.r32(11), 32);
            if c.cc_ne() { blk = 794; continue; }
            blk = 793; continue;
        }
        793 => {
            let Some(t929) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008037cc); };
            c.w64(0, t929);
            blk = 960; continue;
        }
        794 => {
            let Some(t930) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008037d5); };
            c.w32(11, t930);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 796; continue; }
            blk = 795; continue;
        }
        795 => {
            return RegfileExit::fault(0x70008037e5);
        }
        796 => {
            let Some(t931) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008037e6); };
            c.cmp(t931, c.r32(8), 32);
            if c.cc_ne() { blk = 800; continue; }
            blk = 797; continue;
        }
        797 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 799; continue; }
            blk = 798; continue;
        }
        798 => {
            return RegfileExit::fault(0x70008037f5);
        }
        799 => {
            let Some(t932) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008037f6); };
            c.w32(8, t932);
            blk = 806; continue;
        }
        800 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 802; continue; }
            blk = 801; continue;
        }
        801 => {
            return RegfileExit::fault(0x7000803805);
        }
        802 => {
            let Some(t933) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000803806); };
            c.cmp(t933, c.r32(8), 32);
            if c.cc_ne() { blk = 829; continue; }
            blk = 803; continue;
        }
        803 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 805; continue; }
            blk = 804; continue;
        }
        804 => {
            return RegfileExit::fault(0x7000803819);
        }
        805 => {
            let Some(t934) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700080381a); };
            c.w32(8, t934);
            blk = 806; continue;
        }
        806 => {
            c.cmp(c.r64(1), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 828; continue; }
            blk = 807; continue;
        }
        807 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1008; continue; }
            blk = 808; continue;
        }
        808 => {
            c.w32(9, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t935) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080383f); };
            c.w64(11, t935);
            let Some(t936) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000803846); };
            c.w64(12, t936);
            c.w64(15, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(15).wrapping_add(c.r64(12)));
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t937) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000803867); };
            c.w64(6, t937);
            let Some(t938) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700080386e); };
            c.cmp(t938, c.r64(0), 64);
            if c.cc_be() { blk = 1004; continue; }
            blk = 809; continue;
        }
        809 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x7000803877); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 810; continue;
        }
        810 => {
            let Some(t939) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080387f); };
            c.w64(12, t939);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000803886); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700080388a); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 813; continue; }
            blk = 811; continue;
        }
        811 => {
            c.w64(9, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t940) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080389b); };
            c.w64(12, t940);
            blk = 812; continue;
        }
        812 => {
            let r941 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r941);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008038a6); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 812; continue; }
            blk = 813; continue;
        }
        813 => {
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 822; continue; }
            blk = 814; continue;
        }
        814 => {
            c.w64(12, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t942) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x70008038ca); };
            c.w64(15, t942);
            blk = 816; continue;
        }
        815 => {
            let r943 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r943);
            let Some(t944) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008038d7); };
            c.w64(2, t944);
            blk = 816; continue;
        }
        816 => {
            let r945 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r945);
            let Some(t946) = c.ld64(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008038df); };
            c.cmp(t946, c.r64(15), 64);
            if c.cc_e() { blk = 821; continue; }
            blk = 817; continue;
        }
        817 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(2))) { return RegfileExit::fault(0x70008038e5); }
            let Some(t947) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008038eb); };
            c.w64(0, t947);
            let Some(t948) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008038f2); };
            c.w64(1, t948);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            c.w64(7, c.r64(12).wrapping_sub(0x4));
            let Some(t949) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008038fe); };
            c.w64(2, t949);
            let Some(t950) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000803905); };
            c.cmp(t950, c.r64(6), 64);
            if c.cc_be() { blk = 1006; continue; }
            blk = 818; continue;
        }
        818 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x700080390e); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 819; continue;
        }
        819 => {
            let Some(t951) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000803915); };
            c.w64(2, t951);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700080391c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080391f); }
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x7000803924); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 821; continue; }
            blk = 820; continue;
        }
        820 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r952 = c.r64(2) & c.r64(11);
            c.w64(2, r952);
            c.logic(r952, 64);
            let Some(t953) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803936); };
            c.logic(t953 & 0x4u64, 8);
            if c.cc_ne() { blk = 1058; continue; }
            blk = 821; continue;
        }
        821 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 815; continue; }
            blk = 822; continue;
        }
        822 => {
            let Some(t954) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000803946); };
            c.w64(12, t954);
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let r955 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r955);
            let r956 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r956);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000803959); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 824; continue; }
            blk = 823; continue;
        }
        823 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r957 = c.r64(15) & c.r64(3);
            c.w64(15, r957);
            c.logic(r957, 64);
            let Some(t958) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080396d); };
            c.logic(t958 & 0x4u64, 8);
            if c.cc_ne() { blk = 1055; continue; }
            blk = 824; continue;
        }
        824 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000803978); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r959 = c.r64(11) & c.r64(3);
            c.w64(11, r959);
            c.logic(r959, 64);
            let Some(t960) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803986); };
            c.logic(t960 & 0x4u64, 8);
            if c.cc_ne() { blk = 1052; continue; }
            blk = 825; continue;
        }
        825 => {
            let Some(t961) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000803991); };
            c.w32(8, t961);
            let r962 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r962);
            let Some(t963) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000803998); };
            c.w64(0, t963);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x700080399c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 827; continue; }
            blk = 826; continue;
        }
        826 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r964 = c.r64(11) & c.r64(8);
            c.w64(11, r964);
            c.logic(r964, 64);
            let Some(t965) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008039ae); };
            c.logic(t965 & 0x4u64, 8);
            if c.cc_ne() { blk = 1049; continue; }
            blk = 827; continue;
        }
        827 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008039b9); }
            blk = 185; continue;
        }
        828 => {
            let Some(t966) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008039c2); };
            c.w64(0, t966);
            blk = 960; continue;
        }
        829 => {
            let Some(t967) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008039cb); };
            c.w64(0, t967);
            blk = 960; continue;
        }
        830 => {
            c.w64(8, c.r64(0));
            let r968 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r968);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 831; continue;
        }
        831 => {
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let r969 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r969);
            c.w64(9, c.r64(2));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x70008039f8); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008039ff); }
            blk = 185; continue;
        }
        832 => {
            let Some(t970) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803a08); };
            c.w32(9, t970);
            let Some(t971) = c.ld32(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000803a0c); };
            c.cmp(t971, c.r32(9), 32);
            if c.cc_ne() { blk = 904; continue; }
            blk = 833; continue;
        }
        833 => {
            let Some(t972) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000803a19); };
            c.w32(9, t972);
            let Some(t973) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000803a1e); };
            c.cmp(t973, c.r32(9), 32);
            if c.cc_ne() { blk = 840; continue; }
            blk = 834; continue;
        }
        834 => {
            c.w64(9, c.r64(8));
            blk = 835; continue;
        }
        835 => {
            let Some(t974) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803a2a); };
            c.w32(9, t974);
            let r975 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r975);
            let Some(t976) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000803a31); };
            c.cmp(t976, c.r32(9), 32);
            if c.cc_e() { blk = 840; continue; }
            blk = 836; continue;
        }
        836 => {
            let Some(t977) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803a3a); };
            c.w32(9, t977);
            let r978 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r978);
            let Some(t979) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000803a41); };
            c.cmp(t979, 0x411u64, 16);
            if c.cc_be() { blk = 960; continue; }
            blk = 837; continue;
        }
        837 => {
            let Some(t980) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000803a4e); };
            c.w32(11, t980);
            c.w64(7, c.r64(11));
            let r981 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r981);
            let r982 = c.r32(7) & 0x3fu64;
            c.w32(7, r982);
            c.logic(r982, 32);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_be() { blk = 835; continue; }
            blk = 838; continue;
        }
        838 => {
            let r983 = c.r32(11) & 0xfcu64;
            c.w32(11, r983);
            c.logic(r983, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 835; continue; }
            blk = 839; continue;
        }
        839 => {
            blk = 960; continue;
        }
        840 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 903; continue; }
            blk = 841; continue;
        }
        841 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_le() { blk = 845; continue; }
            blk = 842; continue;
        }
        842 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000803a8c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 844; continue; }
            blk = 843; continue;
        }
        843 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r984 = c.r64(9) & c.r64(2);
            c.w64(9, r984);
            c.logic(r984, 64);
            let Some(t985) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803a9e); };
            c.logic(t985 & 0x4u64, 8);
            if c.cc_ne() { blk = 1046; continue; }
            blk = 844; continue;
        }
        844 => {
            blk = 185; continue;
        }
        845 => {
            let Some(t986) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803aae); };
            c.w32(9, t986);
            let r987 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r987);
            let Some(t988) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803ab5); };
            c.w32(11, t988);
            let Some(t989) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000803ab9); };
            c.w32(9, t989);
            let r990 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r990);
            let Some(t991) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000803ac0); };
            c.cmp(t991, c.r32(11), 32);
            if c.cc_ne() { blk = 880; continue; }
            blk = 846; continue;
        }
        846 => {
            c.w64(11, c.r64(3));
            let r992 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r992);
            c.logic(r992, 64);
            let Some(t993) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803ad7); };
            c.w64(15, t993);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 853; continue; }
            blk = 847; continue;
        }
        847 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 853; continue; }
            blk = 848; continue;
        }
        848 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t994) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803aeb); };
            c.w64(7, t994);
            let Some(t995) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000803af2); };
            c.w64(7, t995);
            c.w64(1, c.r64(15));
            let r996 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r996);
            c.logic(r996, 64);
            c.w64(6, c.r64(7));
            let r997 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r997);
            c.logic(r997, 64);
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_e() { blk = 851; continue; }
            blk = 849; continue;
        }
        849 => {
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_e() { blk = 852; continue; }
            blk = 850; continue;
        }
        850 => {
            blk = 853; continue;
        }
        851 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 853; continue; }
            blk = 852; continue;
        }
        852 => {
            let Some(t998) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803b1a); };
            c.w32(11, t998);
            let Some(t999) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000803b1e); };
            c.cmp(t999, c.r32(11), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 853; continue;
        }
        853 => {
            let Some(t1000) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000803b2b); };
            c.w32(11, t1000);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 855; continue; }
            blk = 854; continue;
        }
        854 => {
            return RegfileExit::fault(0x7000803b3b);
        }
        855 => {
            let Some(t1001) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000803b3c); };
            c.cmp(t1001, c.r32(8), 32);
            if c.cc_ne() { blk = 859; continue; }
            blk = 856; continue;
        }
        856 => {
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 858; continue; }
            blk = 857; continue;
        }
        857 => {
            return RegfileExit::fault(0x7000803b4b);
        }
        858 => {
            let Some(t1002) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000803b4c); };
            c.w32(8, t1002);
            blk = 865; continue;
        }
        859 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 861; continue; }
            blk = 860; continue;
        }
        860 => {
            return RegfileExit::fault(0x7000803b5b);
        }
        861 => {
            let Some(t1003) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000803b5c); };
            c.cmp(t1003, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 862; continue;
        }
        862 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 864; continue; }
            blk = 863; continue;
        }
        863 => {
            return RegfileExit::fault(0x7000803b6f);
        }
        864 => {
            let Some(t1004) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000803b70); };
            c.w32(8, t1004);
            blk = 865; continue;
        }
        865 => {
            let Some(t1005) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000803b77); };
            c.w32(9, t1005);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3fffu64, 64);
            if c.cc_ae() { blk = 960; continue; }
            blk = 866; continue;
        }
        866 => {
            c.cmp(c.r64(11), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 1014; continue; }
            blk = 867; continue;
        }
        867 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 1010; continue; }
            blk = 868; continue;
        }
        868 => {
            let Some(t1006) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803bb3); };
            c.w64(1, t1006);
            let Some(t1007) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000803bba); };
            c.w64(6, t1007);
            c.w64(7, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t1008) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803bc1); };
            c.cmp(t1008, c.r64(7), 64);
            if c.cc_be() { blk = 1012; continue; }
            blk = 869; continue;
        }
        869 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000803bcb); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 870; continue;
        }
        870 => {
            let r1009 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r1009);
            let Some(t1010) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000803bd5); };
            c.w64(1, t1010);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000803bdc); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000803be0); }
            c.w64(11, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 876; continue; }
            blk = 871; continue;
        }
        871 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            blk = 872; continue;
        }
        872 => {
            let r1011 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r1011);
            let Some(t1012) = c.ld32(c.r64(11).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000803bfe); };
            c.w32(1, t1012);
            let r1013 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r1013);
            let Some(t1014) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000803c06); };
            c.cmp(t1014, c.r32(1), 32);
            if c.cc_e() { blk = 874; continue; }
            blk = 873; continue;
        }
        873 => {
            let r1015 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r1015);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x7000803c19); }
            blk = 875; continue;
        }
        874 => {
            let Some(t1016) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x7000803c21); };
            c.w64(1, t1016);
            if !c.st64(c.r64(9).wrapping_add(c.r64(15)), c.r64(1)) { return RegfileExit::fault(0x7000803c28); }
            blk = 875; continue;
        }
        875 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 872; continue; }
            blk = 876; continue;
        }
        876 => {
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000803c32); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 878; continue; }
            blk = 877; continue;
        }
        877 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r1017 = c.r64(9) & c.r64(3);
            c.w64(9, r1017);
            c.logic(r1017, 64);
            let Some(t1018) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803c46); };
            c.logic(t1018 & 0x4u64, 8);
            if c.cc_ne() { blk = 1043; continue; }
            blk = 878; continue;
        }
        878 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000803c51); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r1019 = c.r64(9) & c.r64(3);
            c.w64(9, r1019);
            c.logic(r1019, 64);
            let Some(t1020) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803c5f); };
            c.logic(t1020 & 0x4u64, 8);
            if c.cc_ne() { blk = 1040; continue; }
            blk = 879; continue;
        }
        879 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000803c6a); }
            let Some(t1021) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000803c6f); };
            c.w32(8, t1021);
            let r1022 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1022);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000803c80); }
            blk = 185; continue;
        }
        880 => {
            c.w64(11, c.r64(3));
            let r1023 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r1023);
            c.logic(r1023, 64);
            let Some(t1024) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803c96); };
            c.w64(15, t1024);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 887; continue; }
            blk = 881; continue;
        }
        881 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 887; continue; }
            blk = 882; continue;
        }
        882 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t1025) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803caa); };
            c.w64(7, t1025);
            let Some(t1026) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000803cb1); };
            c.w64(7, t1026);
            c.w64(1, c.r64(15));
            let r1027 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r1027);
            c.logic(r1027, 64);
            c.w64(6, c.r64(7));
            let r1028 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r1028);
            c.logic(r1028, 64);
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_e() { blk = 885; continue; }
            blk = 883; continue;
        }
        883 => {
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_e() { blk = 886; continue; }
            blk = 884; continue;
        }
        884 => {
            blk = 887; continue;
        }
        885 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 887; continue; }
            blk = 886; continue;
        }
        886 => {
            let Some(t1029) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803cd9); };
            c.w32(11, t1029);
            let Some(t1030) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000803cdd); };
            c.cmp(t1030, c.r32(11), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 887; continue;
        }
        887 => {
            let Some(t1031) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000803cea); };
            c.w32(11, t1031);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 889; continue; }
            blk = 888; continue;
        }
        888 => {
            return RegfileExit::fault(0x7000803cfa);
        }
        889 => {
            let Some(t1032) = c.ld32(c.r64(9).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000803cfb); };
            c.cmp(t1032, c.r32(8), 32);
            if c.cc_ne() { blk = 893; continue; }
            blk = 890; continue;
        }
        890 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 892; continue; }
            blk = 891; continue;
        }
        891 => {
            return RegfileExit::fault(0x7000803d0a);
        }
        892 => {
            let Some(t1033) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000803d0b); };
            c.w32(8, t1033);
            blk = 899; continue;
        }
        893 => {
            c.cmp(c.r64(11), 0x6au64, 64);
            if c.cc_a() { blk = 895; continue; }
            blk = 894; continue;
        }
        894 => {
            return RegfileExit::fault(0x7000803d1a);
        }
        895 => {
            let Some(t1034) = c.ld32(c.r64(9).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000803d1b); };
            c.cmp(t1034, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 896; continue;
        }
        896 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 898; continue; }
            blk = 897; continue;
        }
        897 => {
            return RegfileExit::fault(0x7000803d2e);
        }
        898 => {
            let Some(t1035) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000803d2f); };
            c.w32(8, t1035);
            blk = 899; continue;
        }
        899 => {
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000803d3e); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r1036 = c.r64(11) & c.r64(3);
            c.w64(11, r1036);
            c.logic(r1036, 64);
            let Some(t1037) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803d4c); };
            c.logic(t1037 & 0x4u64, 8);
            if c.cc_ne() { blk = 1037; continue; }
            blk = 900; continue;
        }
        900 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000803d57); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 902; continue; }
            blk = 901; continue;
        }
        901 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r1038 = c.r64(8) & c.r64(2);
            c.w64(8, r1038);
            c.logic(r1038, 64);
            let Some(t1039) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803d69); };
            c.logic(t1039 & 0x4u64, 8);
            if c.cc_ne() { blk = 1034; continue; }
            blk = 902; continue;
        }
        902 => {
            blk = 185; continue;
        }
        903 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000803d79); }
            blk = 185; continue;
        }
        904 => {
            let Some(t1040) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000803d83); };
            c.cmp(t1040, c.r32(9), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 905; continue;
        }
        905 => {
            let Some(t1041) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x7000803d90); };
            c.w64(9, t1041);
            let Some(t1042) = c.ld64(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000803d97); };
            c.cmp(t1042, c.r64(9), 64);
            if c.cc_ne() { blk = 912; continue; }
            blk = 906; continue;
        }
        906 => {
            c.w64(9, c.r64(8));
            blk = 907; continue;
        }
        907 => {
            let Some(t1043) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803da1); };
            c.w32(9, t1043);
            let r1044 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1044);
            let Some(t1045) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000803da8); };
            c.cmp(t1045, c.r32(9), 32);
            if c.cc_e() { blk = 912; continue; }
            blk = 908; continue;
        }
        908 => {
            let Some(t1046) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803db1); };
            c.w32(9, t1046);
            let r1047 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1047);
            let Some(t1048) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000803db8); };
            c.cmp(t1048, 0x411u64, 16);
            if c.cc_be() { blk = 960; continue; }
            blk = 909; continue;
        }
        909 => {
            let Some(t1049) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000803dc5); };
            c.w32(11, t1049);
            c.w64(15, c.r64(11));
            let r1050 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r1050);
            let r1051 = c.r32(15) & 0x3fu64;
            c.w32(15, r1051);
            c.logic(r1051, 32);
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_be() { blk = 907; continue; }
            blk = 910; continue;
        }
        910 => {
            let r1052 = c.r32(11) & 0xfcu64;
            c.w32(11, r1052);
            c.logic(r1052, 32);
            c.cmp(c.r32(11), 0xa0u64, 32);
            if c.cc_e() { blk = 907; continue; }
            blk = 911; continue;
        }
        911 => {
            blk = 960; continue;
        }
        912 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 956; continue; }
            blk = 913; continue;
        }
        913 => {
            let Some(t1053) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803df8); };
            c.w32(9, t1053);
            let Some(t1054) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000803dfc); };
            c.cmp(t1054, c.r32(9), 32);
            if c.cc_ne() { blk = 915; continue; }
            blk = 914; continue;
        }
        914 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000803e05); }
            blk = 957; continue;
        }
        915 => {
            let Some(t1055) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000803e0f); };
            c.w32(9, t1055);
            let r1056 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1056);
            c.w64(11, c.r64(3));
            let r1057 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r1057);
            c.logic(r1057, 64);
            let Some(t1058) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000803e20); };
            c.w32(9, t1058);
            let r1059 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1059);
            let Some(t1060) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803e27); };
            c.w64(15, t1060);
            c.logic(c.r8(15) & 0x18u64, 8);
            if c.cc_e() { blk = 922; continue; }
            blk = 916; continue;
        }
        916 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 922; continue; }
            blk = 917; continue;
        }
        917 => {
            c.w64(15, c.r64(3).wrapping_add(0x13));
            let Some(t1061) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803e3b); };
            c.w64(7, t1061);
            let Some(t1062) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000803e42); };
            c.w64(7, t1062);
            c.w64(1, c.r64(15));
            let r1063 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r1063);
            c.logic(r1063, 64);
            c.w64(6, c.r64(7));
            let r1064 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r1064);
            c.logic(r1064, 64);
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_e() { blk = 920; continue; }
            blk = 918; continue;
        }
        918 => {
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_e() { blk = 921; continue; }
            blk = 919; continue;
        }
        919 => {
            blk = 922; continue;
        }
        920 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 922; continue; }
            blk = 921; continue;
        }
        921 => {
            let Some(t1065) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000803e6a); };
            c.w32(11, t1065);
            let Some(t1066) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000803e6e); };
            c.cmp(t1066, c.r32(11), 32);
            if c.cc_e() { blk = 960; continue; }
            blk = 922; continue;
        }
        922 => {
            let Some(t1067) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000803e7b); };
            c.w32(11, t1067);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6du64, 64);
            if c.cc_a() { blk = 924; continue; }
            blk = 923; continue;
        }
        923 => {
            return RegfileExit::fault(0x7000803e8b);
        }
        924 => {
            let Some(t1068) = c.ld32(c.r64(9).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x7000803e8c); };
            c.cmp(t1068, c.r32(8), 32);
            if c.cc_ne() { blk = 928; continue; }
            blk = 925; continue;
        }
        925 => {
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 927; continue; }
            blk = 926; continue;
        }
        926 => {
            return RegfileExit::fault(0x7000803e9b);
        }
        927 => {
            let Some(t1069) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000803e9c); };
            c.w32(8, t1069);
            blk = 934; continue;
        }
        928 => {
            c.cmp(c.r64(11), 0x6eu64, 64);
            if c.cc_a() { blk = 930; continue; }
            blk = 929; continue;
        }
        929 => {
            return RegfileExit::fault(0x7000803eab);
        }
        930 => {
            let Some(t1070) = c.ld32(c.r64(9).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000803eac); };
            c.cmp(t1070, c.r32(8), 32);
            if c.cc_ne() { blk = 960; continue; }
            blk = 931; continue;
        }
        931 => {
            c.cmp(c.r64(11), 0x6cu64, 64);
            if c.cc_a() { blk = 933; continue; }
            blk = 932; continue;
        }
        932 => {
            return RegfileExit::fault(0x7000803ebf);
        }
        933 => {
            let Some(t1071) = c.ld32(c.r64(9).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000803ec0); };
            c.w32(8, t1071);
            blk = 934; continue;
        }
        934 => {
            let Some(t1072) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000803ec7); };
            c.w32(9, t1072);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000803ed4); }
            c.cmp(c.r64(11), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 960; continue; }
            blk = 935; continue;
        }
        935 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1020; continue; }
            blk = 936; continue;
        }
        936 => {
            let Some(t1073) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803ef2); };
            c.w64(15, t1073);
            let Some(t1074) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000803ef9); };
            c.w64(1, t1074);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(1)));
            let Some(t1075) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000803f08); };
            c.w64(0, t1075);
            let Some(t1076) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000803f0f); };
            c.cmp(t1076, c.r64(7), 64);
            if c.cc_be() { blk = 1016; continue; }
            blk = 937; continue;
        }
        937 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x7000803f18); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 938; continue;
        }
        938 => {
            let r1077 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r1077);
            let Some(t1078) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000803f22); };
            c.w64(0, t1078);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000803f29); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000803f2d); }
            c.w64(11, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 941; continue; }
            blk = 939; continue;
        }
        939 => {
            c.w64(0, c.r64(11));
            let Some(t1079) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000803f42); };
            c.w64(1, t1079);
            blk = 940; continue;
        }
        940 => {
            let r1080 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r1080);
            if !c.st32(c.r64(0).wrapping_add(c.r64(15)), c.r32(1)) { return RegfileExit::fault(0x7000803f4d); }
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_ne() { blk = 940; continue; }
            blk = 941; continue;
        }
        941 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 950; continue; }
            blk = 942; continue;
        }
        942 => {
            let Some(t1081) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x7000803f69); };
            c.w64(0, t1081);
            blk = 944; continue;
        }
        943 => {
            let r1082 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r1082);
            let Some(t1083) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000803f76); };
            c.w64(2, t1083);
            blk = 944; continue;
        }
        944 => {
            let r1084 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r1084);
            let Some(t1085) = c.ld64(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000803f7e); };
            c.cmp(t1085, c.r64(0), 64);
            if c.cc_e() { blk = 949; continue; }
            blk = 945; continue;
        }
        945 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(2))) { return RegfileExit::fault(0x7000803f84); }
            let Some(t1086) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000803f8a); };
            c.w64(1, t1086);
            let Some(t1087) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000803f91); };
            c.w64(6, t1087);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            c.w64(2, c.r64(11).wrapping_sub(0x4));
            let Some(t1088) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000803f9c); };
            c.w64(8, t1088);
            let Some(t1089) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000803fa3); };
            c.cmp(t1089, c.r64(7), 64);
            if c.cc_be() { blk = 1018; continue; }
            blk = 946; continue;
        }
        946 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000803fac); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 947; continue;
        }
        947 => {
            let Some(t1090) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000803fb3); };
            c.w64(1, t1090);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000803fba); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000803fbe); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(2)), c.r32(8)) { return RegfileExit::fault(0x7000803fc4); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 949; continue; }
            blk = 948; continue;
        }
        948 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r1091 = c.r64(1) & c.r64(15);
            c.w64(1, r1091);
            c.logic(r1091, 64);
            let Some(t1092) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000803fd8); };
            c.logic(t1092 & 0x4u64, 8);
            if c.cc_ne() { blk = 1031; continue; }
            blk = 949; continue;
        }
        949 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 943; continue; }
            blk = 950; continue;
        }
        950 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000803ff0); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 952; continue; }
            blk = 951; continue;
        }
        951 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r1093 = c.r64(9) & c.r64(3);
            c.w64(9, r1093);
            c.logic(r1093, 64);
            let Some(t1094) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804004); };
            c.logic(t1094 & 0x4u64, 8);
            if c.cc_ne() { blk = 1028; continue; }
            blk = 952; continue;
        }
        952 => {
            let Some(t1095) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080400f); };
            c.w64(9, t1095);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000804013); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r1096 = c.r64(11) & c.r64(3);
            c.w64(11, r1096);
            c.logic(r1096, 64);
            let Some(t1097) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804021); };
            c.logic(t1097 & 0x4u64, 8);
            if c.cc_ne() { blk = 1025; continue; }
            blk = 953; continue;
        }
        953 => {
            let Some(t1098) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080402c); };
            c.w32(9, t1098);
            let r1099 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1099);
            let Some(t1100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000804033); };
            c.w64(0, t1100);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000804037); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 955; continue; }
            blk = 954; continue;
        }
        954 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r1101 = c.r64(11) & c.r64(9);
            c.w64(11, r1101);
            c.logic(r1101, 64);
            let Some(t1102) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804049); };
            c.logic(t1102 & 0x4u64, 8);
            if c.cc_ne() { blk = 1022; continue; }
            blk = 955; continue;
        }
        955 => {
            blk = 185; continue;
        }
        956 => {
            c.w64(8, c.r64(0));
            let r1103 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r1103);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 957; continue;
        }
        957 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            c.w64(8, c.r64(2));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000804075); }
            blk = 185; continue;
        }
        958 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        959 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        960 => {
            let Some(t1104) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000804089); };
            c.w64(5, t1104);
            let Some(t1105) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x700080408d); };
            c.w64(10, t1105);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000804092); }
            let Some(t1106) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x7000804097); };
            c.w64(6, t1106);
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(3)) { return RegfileExit::fault(0x700080409c); }
            let Some(t1107) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008040a1); };
            c.w64(10, t1107);
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(10)) { return RegfileExit::fault(0x70008040a6); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(0)) { return RegfileExit::fault(0x70008040ab); }
            let Some(t1108) = c.ld64(c.r64(13).wrapping_add(0x2470)) else { return RegfileExit::fault(0x70008040b0); };
            c.w64(3, t1108);
            c.w32(0, 0x3u64);
            let r1109 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r1109);
            return RegfileExit::jump(0x700086ed74);
        }
        961 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 963; continue; }
            blk = 962; continue;
        }
        962 => {
            c.w64(3, c.r64(2));
            blk = 960; continue;
        }
        963 => {
            let Some(t1110) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008040d1); };
            c.w32(9, t1110);
            let r1111 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1111);
            blk = 27; continue;
        }
        964 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008040e2); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008040ef, 0x70008040ea);
        }
        965 => {
            c.w64(8, c.r64(0));
            let Some(t1112) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008040f2); };
            c.w64(2, t1112);
            let Some(t1113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008040f6); };
            c.w64(1, t1113);
            let Some(t1114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008040fa); };
            c.w64(0, t1114);
            let Some(t1115) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008040fe); };
            c.w64(6, t1115);
            let Some(t1116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000804102); };
            c.w64(9, t1116);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000804106); }
            blk = 59; continue;
        }
        966 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000804115); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000804119); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700080411d); }
            return c.call(0x70007febf4, 0x7000804127, 0x7000804122);
        }
        967 => {
            c.w64(12, c.r64(0));
            let Some(t1117) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080412a); };
            c.w64(8, t1117);
            let Some(t1118) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080412e); };
            c.w64(9, t1118);
            let Some(t1119) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000804132); };
            c.w64(11, t1119);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000804136); }
            blk = 277; continue;
        }
        968 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000804140); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000804144); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000804148); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700080414c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000804150); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febf4, 0x700080415c, 0x7000804157);
        }
        969 => {
            c.w64(8, c.r64(0));
            let Some(t1120) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080415f); };
            c.w64(15, t1120);
            let Some(t1121) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000804163); };
            c.w64(9, t1121);
            let Some(t1122) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000804167); };
            c.w64(3, t1122);
            let Some(t1123) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080416b); };
            c.w64(12, t1123);
            let Some(t1124) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080416f); };
            c.w64(11, t1124);
            let Some(t1125) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000804173); };
            c.w64(2, t1125);
            blk = 285; continue;
        }
        970 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000804181); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000804185); }
            return c.call(0x70007febf4, 0x700080418f, 0x700080418a);
        }
        971 => {
            c.w64(11, c.r64(0));
            let Some(t1126) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000804192); };
            c.w64(9, t1126);
            let Some(t1127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000804196); };
            c.w64(8, t1127);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700080419a); }
            blk = 307; continue;
        }
        972 => {
            return RegfileExit::fault(0x70008041a4);
        }
        973 => {
            return RegfileExit::fault(0x70008041a5);
        }
        974 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008041ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008041b0); }
            return c.call(0x70007febf4, 0x70008041b9, 0x70008041b4);
        }
        975 => {
            c.w64(8, c.r64(0));
            let Some(t1128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008041bc); };
            c.w64(9, t1128);
            let Some(t1129) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008041c0); };
            c.w64(15, t1129);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008041c4); }
            blk = 427; continue;
        }
        976 => {
            return RegfileExit::fault(0x70008041ce);
        }
        977 => {
            return RegfileExit::fault(0x70008041cf);
        }
        978 => {
            let Some(t1130) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x70008041d0); };
            c.w64(11, t1130);
            blk = 461; continue;
        }
        979 => {
            return RegfileExit::fault(0x70008041dc);
        }
        980 => {
            return RegfileExit::fault(0x70008041dd);
        }
        981 => {
            return RegfileExit::fault(0x70008041de);
        }
        982 => {
            return RegfileExit::fault(0x70008041df);
        }
        983 => {
            return RegfileExit::fault(0x70008041e0);
        }
        984 => {
            return RegfileExit::fault(0x70008041e1);
        }
        985 => {
            return RegfileExit::fault(0x70008041e2);
        }
        986 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1131) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008041e5); };
            c.w64(3, t1131);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008041f3, 0x70008041ee);
        }
        987 => {
            return RegfileExit::fault(0x70008041f3);
        }
        988 => {
            let r1132 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r1132);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008041f7); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008041fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008041fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000804200); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x7000804204); }
            let Some(t1133) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000804208); };
            c.w64(3, t1133);
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700080421c, 0x7000804217);
        }
        989 => {
            c.w64(9, c.r64(0));
            let Some(t1134) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080421f); };
            c.w64(3, t1134);
            let Some(t1135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000804223); };
            c.w64(2, t1135);
            let Some(t1136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000804227); };
            c.w64(12, t1136);
            let Some(t1137) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080422b); };
            c.w64(11, t1137);
            let Some(t1138) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080422f); };
            c.w64(8, t1138);
            let Some(t1139) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000804233); };
            c.w64(7, t1139);
            let Some(t1140) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000804237); };
            c.w64(15, t1140);
            blk = 565; continue;
        }
        990 => {
            let Some(t1141) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000804240); };
            c.w64(3, t1141);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000804250, 0x700080424b);
        }
        991 => {
            return RegfileExit::fault(0x7000804250);
        }
        992 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x7000804251); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000804255); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febf4, 0x7000804261, 0x700080425c);
        }
        993 => {
            c.w64(9, c.r64(0));
            let Some(t1142) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000804264); };
            c.w64(3, t1142);
            let Some(t1143) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000804268); };
            c.w64(2, t1143);
            let Some(t1144) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080426c); };
            c.w64(7, t1144);
            let Some(t1145) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000804270); };
            c.w64(1, t1145);
            let Some(t1146) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000804274); };
            c.w64(8, t1146);
            blk = 664; continue;
        }
        994 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000804282); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000804286); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700080428a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700080428e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000804292); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000804296); }
            return c.call(0x70007febf4, 0x70008042a0, 0x700080429b);
        }
        995 => {
            let Some(t1147) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x70008042a0); };
            c.w64(15, t1147);
            let Some(t1148) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008042a7); };
            c.w64(3, t1148);
            let Some(t1149) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008042ab); };
            c.w64(8, t1149);
            let Some(t1150) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008042af); };
            c.w64(9, t1150);
            let Some(t1151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008042b3); };
            c.w64(12, t1151);
            let Some(t1152) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008042b7); };
            c.w64(11, t1152);
            let Some(t1153) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008042bb); };
            c.w64(7, t1153);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008042bf); }
            blk = 673; continue;
        }
        996 => {
            let Some(t1154) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008042c9); };
            c.w64(3, t1154);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008042d9, 0x70008042d4);
        }
        997 => {
            return RegfileExit::fault(0x70008042d9);
        }
        998 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1155) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008042dc); };
            c.w64(3, t1155);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008042ea, 0x70008042e5);
        }
        999 => {
            return RegfileExit::fault(0x70008042ea);
        }
        1000 => {
            let r1156 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r1156);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008042ee); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008042f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008042f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008042f7); }
            let Some(t1157) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008042fb); };
            c.w64(3, t1157);
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700080430f, 0x700080430a);
        }
        1001 => {
            c.w64(9, c.r64(0));
            let Some(t1158) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000804312); };
            c.w64(3, t1158);
            let Some(t1159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000804316); };
            c.w64(2, t1159);
            let Some(t1160) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080431a); };
            c.w64(12, t1160);
            let Some(t1161) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080431e); };
            c.w64(11, t1161);
            let Some(t1162) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000804322); };
            c.w64(1, t1162);
            let Some(t1163) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000804326); };
            c.w64(8, t1163);
            blk = 740; continue;
        }
        1002 => {
            let Some(t1164) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080432f); };
            c.w64(3, t1164);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080433f, 0x700080433a);
        }
        1003 => {
            return RegfileExit::fault(0x700080433f);
        }
        1004 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000804340); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000804344); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000804348); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000804354, 0x700080434f);
        }
        1005 => {
            c.w64(11, c.r64(0));
            let Some(t1165) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000804357); };
            c.w64(3, t1165);
            let Some(t1166) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080435b); };
            c.w64(2, t1166);
            let Some(t1167) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080435f); };
            c.w64(8, t1167);
            let Some(t1168) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000804363); };
            c.w64(7, t1168);
            let Some(t1169) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000804367); };
            c.w64(1, t1169);
            let Some(t1170) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080436b); };
            c.w64(9, t1170);
            blk = 810; continue;
        }
        1006 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000804379); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080437d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000804381); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000804385); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000804389); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700080438d); }
            return c.call(0x70007febf4, 0x7000804397, 0x7000804392);
        }
        1007 => {
            let Some(t1171) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x7000804397); };
            c.w64(15, t1171);
            let Some(t1172) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080439e); };
            c.w64(3, t1172);
            let Some(t1173) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008043a2); };
            c.w64(8, t1173);
            let Some(t1174) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008043a6); };
            c.w64(11, t1174);
            let Some(t1175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008043aa); };
            c.w64(12, t1175);
            let Some(t1176) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008043ae); };
            c.w64(9, t1176);
            let Some(t1177) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008043b2); };
            c.w64(7, t1177);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008043b6); }
            blk = 819; continue;
        }
        1008 => {
            let Some(t1178) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008043c0); };
            c.w64(3, t1178);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008043d0, 0x70008043cb);
        }
        1009 => {
            return RegfileExit::fault(0x70008043d0);
        }
        1010 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1179) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008043d3); };
            c.w64(3, t1179);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008043e1, 0x70008043dc);
        }
        1011 => {
            return RegfileExit::fault(0x70008043e1);
        }
        1012 => {
            let r1180 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r1180);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008043e5); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008043e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008043ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008043ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008043f2); }
            c.w32(0, 0x2u64);
            let Some(t1181) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008043fb); };
            c.w64(3, t1181);
            c.w64(15, c.r64(3));
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700080440d, 0x7000804408);
        }
        1013 => {
            c.w64(15, c.r64(0));
            let Some(t1182) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000804410); };
            c.w64(3, t1182);
            let Some(t1183) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000804414); };
            c.w64(0, t1183);
            let Some(t1184) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000804418); };
            c.w64(2, t1184);
            let Some(t1185) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080441c); };
            c.w64(12, t1185);
            let Some(t1186) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000804420); };
            c.w64(8, t1186);
            let Some(t1187) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000804424); };
            c.w64(9, t1187);
            let Some(t1188) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000804428); };
            c.w64(11, t1188);
            blk = 870; continue;
        }
        1014 => {
            let Some(t1189) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000804431); };
            c.w64(3, t1189);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000804441, 0x700080443c);
        }
        1015 => {
            return RegfileExit::fault(0x7000804441);
        }
        1016 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000804442); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000804446); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000804452, 0x700080444d);
        }
        1017 => {
            c.w64(15, c.r64(0));
            let Some(t1190) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000804455); };
            c.w64(3, t1190);
            let Some(t1191) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000804459); };
            c.w64(2, t1191);
            let Some(t1192) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080445d); };
            c.w64(12, t1192);
            let Some(t1193) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000804461); };
            c.w64(8, t1193);
            let Some(t1194) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000804465); };
            c.w64(9, t1194);
            let Some(t1195) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000804469); };
            c.w64(11, t1195);
            blk = 938; continue;
        }
        1018 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000804472); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700080447b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700080447f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000804483); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000804487); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000804494, 0x700080448f);
        }
        1019 => {
            c.w64(8, c.r64(0));
            let Some(t1196) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000804497); };
            c.w64(3, t1196);
            let Some(t1197) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080449b); };
            c.w64(12, t1197);
            let Some(t1198) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080449f); };
            c.w64(15, t1198);
            let Some(t1199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008044a3); };
            c.w64(11, t1199);
            let Some(t1200) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008044a7); };
            c.w64(9, t1200);
            let Some(t1201) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008044ab); };
            c.w64(2, t1201);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008044af); }
            let Some(t1202) = c.ld64(0x7000802c32u64) else { return RegfileExit::fault(0x70008044b4); };
            c.w64(0, t1202);
            blk = 947; continue;
        }
        1020 => {
            let Some(t1203) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008044c0); };
            c.w64(3, t1203);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008044d0, 0x70008044cb);
        }
        1021 => {
            return RegfileExit::fault(0x70008044d0);
        }
        1022 => {
            c.w32(0, c.r32(0));
            let r1204 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1204);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1205 = c.r64(11) & c.r64(0);
            c.w64(11, r1205);
            c.logic(r1205, 64);
            let Some(t1206) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008044e0); };
            c.logic(t1206 & 0x2u64, 8);
            if c.cc_e() { blk = 955; continue; }
            blk = 1023; continue;
        }
        1023 => {
            c.w64(12, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008044ef); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008044f0); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008044fd, 0x70008044f8);
        }
        1024 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008044fd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008044ff); }
            blk = 955; continue;
        }
        1025 => {
            c.w32(9, c.r32(9));
            let r1207 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1207);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1208 = c.r64(11) & c.r64(9);
            c.w64(11, r1208);
            c.logic(r1208, 64);
            let Some(t1209) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804515); };
            c.logic(t1209 & 0x2u64, 8);
            if c.cc_e() { blk = 953; continue; }
            blk = 1026; continue;
        }
        1026 => {
            c.w64(12, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804524); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804525); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000804532, 0x700080452d);
        }
        1027 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804532); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804534); }
            blk = 953; continue;
        }
        1028 => {
            c.w32(15, c.r32(15));
            let r1210 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1210);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1211 = c.r64(9) & c.r64(15);
            c.w64(9, r1211);
            c.logic(r1211, 64);
            let Some(t1212) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080454a); };
            c.logic(t1212 & 0x2u64, 8);
            if c.cc_e() { blk = 952; continue; }
            blk = 1029; continue;
        }
        1029 => {
            c.w64(11, c.r64(3).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804559); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080455a); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000804567, 0x7000804562);
        }
        1030 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804567); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804569); }
            blk = 952; continue;
        }
        1031 => {
            c.w32(8, c.r32(8));
            let r1213 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1213);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1214 = c.r64(1) & c.r64(8);
            c.w64(1, r1214);
            c.logic(r1214, 64);
            let Some(t1215) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080457f); };
            c.logic(t1215 & 0x2u64, 8);
            if c.cc_e() { blk = 949; continue; }
            blk = 1032; continue;
        }
        1032 => {
            c.w64(6, c.r64(15).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080458d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080458e); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x700080459b, 0x7000804596);
        }
        1033 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080459b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080459d); }
            blk = 949; continue;
        }
        1034 => {
            c.w32(0, c.r32(0));
            let r1216 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1216);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1217 = c.r64(8) & c.r64(0);
            c.w64(8, r1217);
            c.logic(r1217, 64);
            let Some(t1218) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008045b2); };
            c.logic(t1218 & 0x2u64, 8);
            if c.cc_e() { blk = 902; continue; }
            blk = 1035; continue;
        }
        1035 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008045c1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008045c2); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008045cf, 0x70008045ca);
        }
        1036 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008045cf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008045d1); }
            blk = 902; continue;
        }
        1037 => {
            c.w32(8, c.r32(8));
            let r1219 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1219);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1220 = c.r64(11) & c.r64(8);
            c.w64(11, r1220);
            c.logic(r1220, 64);
            let Some(t1221) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008045e7); };
            c.logic(t1221 & 0x2u64, 8);
            if c.cc_e() { blk = 900; continue; }
            blk = 1038; continue;
        }
        1038 => {
            c.w64(12, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008045f6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008045f7); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x7000804604, 0x70008045ff);
        }
        1039 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804604); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804606); }
            blk = 900; continue;
        }
        1040 => {
            c.w32(8, c.r32(8));
            let r1222 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1222);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1223 = c.r64(9) & c.r64(8);
            c.w64(9, r1223);
            c.logic(r1223, 64);
            let Some(t1224) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080461c); };
            c.logic(t1224 & 0x2u64, 8);
            if c.cc_e() { blk = 879; continue; }
            blk = 1041; continue;
        }
        1041 => {
            c.w64(11, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080462b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080462c); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000804639, 0x7000804634);
        }
        1042 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804639); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080463b); }
            blk = 879; continue;
        }
        1043 => {
            c.w32(15, c.r32(15));
            let r1225 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1225);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1226 = c.r64(9) & c.r64(15);
            c.w64(9, r1226);
            c.logic(r1226, 64);
            let Some(t1227) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804651); };
            c.logic(t1227 & 0x2u64, 8);
            if c.cc_e() { blk = 878; continue; }
            blk = 1044; continue;
        }
        1044 => {
            c.w64(11, c.r64(3).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804660); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804661); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700080466e, 0x7000804669);
        }
        1045 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080466e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804670); }
            blk = 878; continue;
        }
        1046 => {
            c.w32(0, c.r32(0));
            let r1228 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1228);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1229 = c.r64(9) & c.r64(0);
            c.w64(9, r1229);
            c.logic(r1229, 64);
            let Some(t1230) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804685); };
            c.logic(t1230 & 0x2u64, 8);
            if c.cc_e() { blk = 844; continue; }
            blk = 1047; continue;
        }
        1047 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804694); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804695); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008046a2, 0x700080469d);
        }
        1048 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008046a2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008046a4); }
            blk = 844; continue;
        }
        1049 => {
            c.w32(0, c.r32(0));
            let r1231 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1231);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1232 = c.r64(11) & c.r64(0);
            c.w64(11, r1232);
            c.logic(r1232, 64);
            let Some(t1233) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008046b9); };
            c.logic(t1233 & 0x2u64, 8);
            if c.cc_e() { blk = 827; continue; }
            blk = 1050; continue;
        }
        1050 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008046c8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008046c9); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008046d6, 0x70008046d1);
        }
        1051 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008046d6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008046d8); }
            blk = 827; continue;
        }
        1052 => {
            c.w32(8, c.r32(8));
            let r1234 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1234);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1235 = c.r64(11) & c.r64(8);
            c.w64(11, r1235);
            c.logic(r1235, 64);
            let Some(t1236) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008046ee); };
            c.logic(t1236 & 0x2u64, 8);
            if c.cc_e() { blk = 825; continue; }
            blk = 1053; continue;
        }
        1053 => {
            c.w64(15, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008046fd); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008046fe); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x700080470b, 0x7000804706);
        }
        1054 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080470b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080470d); }
            blk = 825; continue;
        }
        1055 => {
            c.w32(11, c.r32(11));
            let r1237 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1237);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1238 = c.r64(15) & c.r64(11);
            c.w64(15, r1238);
            c.logic(r1238, 64);
            let Some(t1239) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804723); };
            c.logic(t1239 & 0x2u64, 8);
            if c.cc_e() { blk = 824; continue; }
            blk = 1056; continue;
        }
        1056 => {
            c.w64(0, c.r64(3).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804732); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804733); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000804740, 0x700080473b);
        }
        1057 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804740); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804742); }
            blk = 824; continue;
        }
        1058 => {
            c.w32(0, c.r32(0));
            let r1240 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1240);
            c.w64(2, 0xfffffffffffc0000u64);
            let r1241 = c.r64(2) & c.r64(0);
            c.w64(2, r1241);
            c.logic(r1241, 64);
            let Some(t1242) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804757); };
            c.logic(t1242 & 0x2u64, 8);
            if c.cc_e() { blk = 821; continue; }
            blk = 1059; continue;
        }
        1059 => {
            c.w64(1, c.r64(11).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804765); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804766); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x7000804773, 0x700080476e);
        }
        1060 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804773); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804775); }
            blk = 821; continue;
        }
        1061 => {
            c.w32(0, c.r32(0));
            let r1243 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1243);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1244 = c.r64(8) & c.r64(0);
            c.w64(8, r1244);
            c.logic(r1244, 64);
            let Some(t1245) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080478a); };
            c.logic(t1245 & 0x2u64, 8);
            if c.cc_e() { blk = 774; continue; }
            blk = 1062; continue;
        }
        1062 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804799); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080479a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008047a7, 0x70008047a2);
        }
        1063 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008047a7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008047a9); }
            blk = 774; continue;
        }
        1064 => {
            c.w32(8, c.r32(8));
            let r1246 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1246);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1247 = c.r64(12) & c.r64(8);
            c.w64(12, r1247);
            c.logic(r1247, 64);
            let Some(t1248) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008047bf); };
            c.logic(t1248 & 0x2u64, 8);
            if c.cc_e() { blk = 772; continue; }
            blk = 1065; continue;
        }
        1065 => {
            c.w64(15, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008047cf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008047d0); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x70008047dd, 0x70008047d8);
        }
        1066 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008047dd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008047df); }
            blk = 772; continue;
        }
        1067 => {
            c.w32(8, c.r32(8));
            let r1249 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1249);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1250 = c.r64(9) & c.r64(8);
            c.w64(9, r1250);
            c.logic(r1250, 64);
            let Some(t1251) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008047f5); };
            c.logic(t1251 & 0x2u64, 8);
            if c.cc_e() { blk = 749; continue; }
            blk = 1068; continue;
        }
        1068 => {
            c.w64(15, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804804); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804805); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000804812, 0x700080480d);
        }
        1069 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804812); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804814); }
            blk = 749; continue;
        }
        1070 => {
            c.w32(9, c.r32(9));
            let r1252 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1252);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1253 = c.r64(15) & c.r64(9);
            c.w64(15, r1253);
            c.logic(r1253, 64);
            let Some(t1254) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080482a); };
            c.logic(t1254 & 0x2u64, 8);
            if c.cc_e() { blk = 748; continue; }
            blk = 1071; continue;
        }
        1071 => {
            c.w64(0, c.r64(3).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804839); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080483a); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000804847, 0x7000804842);
        }
        1072 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804847); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804849); }
            blk = 748; continue;
        }
        1073 => {
            c.w32(0, c.r32(0));
            let r1255 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1255);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1256 = c.r64(11) & c.r64(0);
            c.w64(11, r1256);
            c.logic(r1256, 64);
            let Some(t1257) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080485e); };
            c.logic(t1257 & 0x2u64, 8);
            if c.cc_e() { blk = 713; continue; }
            blk = 1074; continue;
        }
        1074 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080486d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080486e); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700080487b, 0x7000804876);
        }
        1075 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080487b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080487d); }
            blk = 713; continue;
        }
        1076 => {
            c.w32(8, c.r32(8));
            let r1258 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1258);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1259 = c.r64(9) & c.r64(8);
            c.w64(9, r1259);
            c.logic(r1259, 64);
            let Some(t1260) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804893); };
            c.logic(t1260 & 0x2u64, 8);
            if c.cc_e() { blk = 699; continue; }
            blk = 1077; continue;
        }
        1077 => {
            c.w64(11, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008048a2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008048a3); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x70008048b0, 0x70008048ab);
        }
        1078 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008048b0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008048b2); }
            blk = 699; continue;
        }
        1079 => {
            c.w32(0, c.r32(0));
            let r1261 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1261);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1262 = c.r64(9) & c.r64(0);
            c.w64(9, r1262);
            c.logic(r1262, 64);
            let Some(t1263) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008048c7); };
            c.logic(t1263 & 0x2u64, 8);
            if c.cc_e() { blk = 681; continue; }
            blk = 1080; continue;
        }
        1080 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008048d6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008048d7); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008048e4, 0x70008048df);
        }
        1081 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008048e4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008048e6); }
            blk = 681; continue;
        }
        1082 => {
            c.w32(8, c.r32(8));
            let r1264 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1264);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1265 = c.r64(9) & c.r64(8);
            c.w64(9, r1265);
            c.logic(r1265, 64);
            let Some(t1266) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008048fc); };
            c.logic(t1266 & 0x2u64, 8);
            if c.cc_e() { blk = 679; continue; }
            blk = 1083; continue;
        }
        1083 => {
            c.w64(15, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080490b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080490c); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000804919, 0x7000804914);
        }
        1084 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804919); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080491b); }
            blk = 679; continue;
        }
        1085 => {
            c.w32(9, c.r32(9));
            let r1267 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1267);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1268 = c.r64(15) & c.r64(9);
            c.w64(15, r1268);
            c.logic(r1268, 64);
            let Some(t1269) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804931); };
            c.logic(t1269 & 0x2u64, 8);
            if c.cc_e() { blk = 678; continue; }
            blk = 1086; continue;
        }
        1086 => {
            c.w64(0, c.r64(3).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804940); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804941); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700080494e, 0x7000804949);
        }
        1087 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080494e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804950); }
            blk = 678; continue;
        }
        1088 => {
            c.w32(0, c.r32(0));
            let r1270 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1270);
            c.w64(2, 0xfffffffffffc0000u64);
            let r1271 = c.r64(2) & c.r64(0);
            c.w64(2, r1271);
            c.logic(r1271, 64);
            let Some(t1272) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804965); };
            c.logic(t1272 & 0x2u64, 8);
            if c.cc_e() { blk = 675; continue; }
            blk = 1089; continue;
        }
        1089 => {
            c.w64(1, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804973); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804974); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x7000804981, 0x700080497c);
        }
        1090 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804981); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804983); }
            blk = 675; continue;
        }
        1091 => {
            c.w32(8, c.r32(8));
            let r1273 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1273);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1274 = c.r64(9) & c.r64(8);
            c.w64(9, r1274);
            c.logic(r1274, 64);
            let Some(t1275) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804999); };
            c.logic(t1275 & 0x2u64, 8);
            if c.cc_e() { blk = 629; continue; }
            blk = 1092; continue;
        }
        1092 => {
            c.w64(11, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008049a8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008049a9); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x70008049b6, 0x70008049b1);
        }
        1093 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008049b6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008049b8); }
            blk = 629; continue;
        }
        1094 => {
            c.w32(8, c.r32(8));
            let r1276 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1276);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1277 = c.r64(9) & c.r64(8);
            c.w64(9, r1277);
            c.logic(r1277, 64);
            let Some(t1278) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008049ce); };
            c.logic(t1278 & 0x2u64, 8);
            if c.cc_e() { blk = 615; continue; }
            blk = 1095; continue;
        }
        1095 => {
            c.w64(11, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008049dd); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008049de); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x70008049eb, 0x70008049e6);
        }
        1096 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008049eb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008049ed); }
            blk = 615; continue;
        }
        1097 => {
            c.w32(0, c.r32(0));
            let r1279 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1279);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1280 = c.r64(8) & c.r64(0);
            c.w64(8, r1280);
            c.logic(r1280, 64);
            let Some(t1281) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804a02); };
            c.logic(t1281 & 0x2u64, 8);
            if c.cc_e() { blk = 599; continue; }
            blk = 1098; continue;
        }
        1098 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804a11); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804a12); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000804a1f, 0x7000804a1a);
        }
        1099 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804a1f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804a21); }
            blk = 599; continue;
        }
        1100 => {
            c.w32(8, c.r32(8));
            let r1282 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1282);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1283 = c.r64(12) & c.r64(8);
            c.w64(12, r1283);
            c.logic(r1283, 64);
            let Some(t1284) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804a37); };
            c.logic(t1284 & 0x2u64, 8);
            if c.cc_e() { blk = 597; continue; }
            blk = 1101; continue;
        }
        1101 => {
            c.w64(15, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804a47); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804a48); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000804a55, 0x7000804a50);
        }
        1102 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804a55); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804a57); }
            blk = 597; continue;
        }
        1103 => {
            c.w32(8, c.r32(8));
            let r1285 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1285);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1286 = c.r64(9) & c.r64(8);
            c.w64(9, r1286);
            c.logic(r1286, 64);
            let Some(t1287) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804a6d); };
            c.logic(t1287 & 0x2u64, 8);
            if c.cc_e() { blk = 574; continue; }
            blk = 1104; continue;
        }
        1104 => {
            c.w64(15, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804a7c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804a7d); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000804a8a, 0x7000804a85);
        }
        1105 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804a8a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804a8c); }
            blk = 574; continue;
        }
        1106 => {
            c.w32(9, c.r32(9));
            let r1288 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1288);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1289 = c.r64(15) & c.r64(9);
            c.w64(15, r1289);
            c.logic(r1289, 64);
            let Some(t1290) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804aa2); };
            c.logic(t1290 & 0x2u64, 8);
            if c.cc_e() { blk = 573; continue; }
            blk = 1107; continue;
        }
        1107 => {
            c.w64(0, c.r64(3).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804ab1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804ab2); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000804abf, 0x7000804aba);
        }
        1108 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804abf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804ac1); }
            blk = 573; continue;
        }
        1109 => {
            c.w32(0, c.r32(0));
            let r1291 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1291);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1292 = c.r64(11) & c.r64(0);
            c.w64(11, r1292);
            c.logic(r1292, 64);
            let Some(t1293) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804ad6); };
            c.logic(t1293 & 0x2u64, 8);
            if c.cc_e() { blk = 538; continue; }
            blk = 1110; continue;
        }
        1110 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804ae5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804ae6); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000804af3, 0x7000804aee);
        }
        1111 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804af3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804af5); }
            blk = 538; continue;
        }
        1112 => {
            c.w32(8, c.r32(8));
            let r1294 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1294);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1295 = c.r64(9) & c.r64(8);
            c.w64(9, r1295);
            c.logic(r1295, 64);
            let Some(t1296) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804b0b); };
            c.logic(t1296 & 0x2u64, 8);
            if c.cc_e() { blk = 536; continue; }
            blk = 1113; continue;
        }
        1113 => {
            c.w64(11, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804b1a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804b1b); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000804b28, 0x7000804b23);
        }
        1114 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804b28); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804b2a); }
            blk = 536; continue;
        }
        1115 => {
            c.w32(0, c.r32(0));
            let r1297 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1297);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1298 = c.r64(1) & c.r64(0);
            c.w64(1, r1298);
            c.logic(r1298, 64);
            let Some(t1299) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804b3f); };
            c.logic(t1299 & 0x2u64, 8);
            if c.cc_e() { blk = 486; continue; }
            blk = 1116; continue;
        }
        1116 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804b4d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804b4e); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x7000804b5b, 0x7000804b56);
        }
        1117 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804b5b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804b5d); }
            blk = 486; continue;
        }
        1118 => {
            c.w32(9, c.r32(9));
            let r1300 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1300);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1301 = c.r64(11) & c.r64(9);
            c.w64(11, r1301);
            c.logic(r1301, 64);
            let Some(t1302) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804b73); };
            c.logic(t1302 & 0x2u64, 8);
            if c.cc_e() { blk = 483; continue; }
            blk = 1119; continue;
        }
        1119 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804b82); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804b83); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000804b90, 0x7000804b8b);
        }
        1120 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804b90); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804b92); }
            blk = 483; continue;
        }
        1121 => {
            c.w32(0, c.r32(0));
            let r1303 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1303);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1304 = c.r64(9) & c.r64(0);
            c.w64(9, r1304);
            c.logic(r1304, 64);
            let Some(t1305) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804ba7); };
            c.logic(t1305 & 0x2u64, 8);
            if c.cc_e() { blk = 345; continue; }
            blk = 1122; continue;
        }
        1122 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804bb6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804bb7); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000804bc4, 0x7000804bbf);
        }
        1123 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804bc4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804bc6); }
            blk = 345; continue;
        }
        1124 => {
            c.w32(11, c.r32(11));
            let r1306 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1306);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1307 = c.r64(8) & c.r64(11);
            c.w64(8, r1307);
            c.logic(r1307, 64);
            let Some(t1308) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804bdc); };
            c.logic(t1308 & 0x2u64, 8);
            if c.cc_e() { blk = 310; continue; }
            blk = 1125; continue;
        }
        1125 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804beb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804bec); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000804bf9, 0x7000804bf4);
        }
        1126 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804bf9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804bfb); }
            blk = 310; continue;
        }
        1127 => {
            c.w32(8, c.r32(8));
            let r1309 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1309);
            c.w64(15, 0xfffffffffffc0000u64);
            let r1310 = c.r64(15) & c.r64(8);
            c.w64(15, r1310);
            c.logic(r1310, 64);
            let Some(t1311) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804c11); };
            c.logic(t1311 & 0x2u64, 8);
            if c.cc_e() { blk = 308; continue; }
            blk = 1128; continue;
        }
        1128 => {
            c.w64(0, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804c21); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804c22); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x7000804c2f, 0x7000804c2a);
        }
        1129 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804c2f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804c31); }
            blk = 308; continue;
        }
        1130 => {
            c.w32(0, c.r32(0));
            let r1312 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1312);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1313 = c.r64(8) & c.r64(0);
            c.w64(8, r1313);
            c.logic(r1313, 64);
            let Some(t1314) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804c46); };
            c.logic(t1314 & 0x2u64, 8);
            if c.cc_e() { blk = 301; continue; }
            blk = 1131; continue;
        }
        1131 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804c55); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804c56); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000804c63, 0x7000804c5e);
        }
        1132 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804c63); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804c65); }
            blk = 301; continue;
        }
        1133 => {
            c.w32(8, c.r32(8));
            let r1315 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1315);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1316 = c.r64(11) & c.r64(8);
            c.w64(11, r1316);
            c.logic(r1316, 64);
            let Some(t1317) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804c7b); };
            c.logic(t1317 & 0x2u64, 8);
            if c.cc_e() { blk = 299; continue; }
            blk = 1134; continue;
        }
        1134 => {
            c.w64(15, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804c8b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804c8c); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x7000804c99, 0x7000804c94);
        }
        1135 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804c99); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804c9b); }
            blk = 299; continue;
        }
        1136 => {
            c.w32(8, c.r32(8));
            let r1318 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1318);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1319 = c.r64(9) & c.r64(8);
            c.w64(9, r1319);
            c.logic(r1319, 64);
            let Some(t1320) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804cb1); };
            c.logic(t1320 & 0x2u64, 8);
            if c.cc_e() { blk = 296; continue; }
            blk = 1137; continue;
        }
        1137 => {
            c.w64(11, c.r64(15).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804cc0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804cc1); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000804cce, 0x7000804cc9);
        }
        1138 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804cce); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804cd0); }
            blk = 296; continue;
        }
        1139 => {
            c.w32(0, c.r32(0));
            let r1321 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1321);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1322 = c.r64(9) & c.r64(0);
            c.w64(9, r1322);
            c.logic(r1322, 64);
            let Some(t1323) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804ce5); };
            c.logic(t1323 & 0x2u64, 8);
            if c.cc_e() { blk = 295; continue; }
            blk = 1140; continue;
        }
        1140 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804cf4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804cf5); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000804d02, 0x7000804cfd);
        }
        1141 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804d02); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804d04); }
            blk = 295; continue;
        }
        1142 => {
            c.w32(8, c.r32(8));
            let r1324 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1324);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1325 = c.r64(11) & c.r64(8);
            c.w64(11, r1325);
            c.logic(r1325, 64);
            let Some(t1326) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804d1a); };
            c.logic(t1326 & 0x2u64, 8);
            if c.cc_e() { blk = 292; continue; }
            blk = 1143; continue;
        }
        1143 => {
            c.w64(12, c.r64(15).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804d29); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804d2a); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000804d37, 0x7000804d32);
        }
        1144 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804d37); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804d39); }
            blk = 292; continue;
        }
        1145 => {
            c.w32(8, c.r32(8));
            let r1327 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1327);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1328 = c.r64(9) & c.r64(8);
            c.w64(9, r1328);
            c.logic(r1328, 64);
            let Some(t1329) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804d4f); };
            c.logic(t1329 & 0x2u64, 8);
            if c.cc_e() { blk = 249; continue; }
            blk = 1146; continue;
        }
        1146 => {
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804d5e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804d5f); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000804d6c, 0x7000804d67);
        }
        1147 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804d6c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804d6e); }
            blk = 249; continue;
        }
        1148 => {
            c.w32(0, c.r32(0));
            let r1330 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1330);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1331 = c.r64(9) & c.r64(0);
            c.w64(9, r1331);
            c.logic(r1331, 64);
            let Some(t1332) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804d83); };
            c.logic(t1332 & 0x2u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 1149; continue;
        }
        1149 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804d92); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804d93); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000804da0, 0x7000804d9b);
        }
        1150 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804da0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804da2); }
            blk = 143; continue;
        }
        1151 => {
            c.w32(0, c.r32(0));
            let r1333 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1333);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1334 = c.r64(9) & c.r64(0);
            c.w64(9, r1334);
            c.logic(r1334, 64);
            let Some(t1335) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000804db7); };
            c.logic(t1335 & 0x2u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 1152; continue;
        }
        1152 => {
            c.w64(11, c.r64(15).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000804dc6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000804dc7); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000804dd4, 0x7000804dcf);
        }
        1153 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000804dd4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000804dd6); }
            blk = 97; continue;
        }
        1154 => {
            return RegfileExit::jump(0x7000804de0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000804df4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000804df4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000804df8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000804dfe); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000804e04); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000804e0b); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000804e11); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000804e15); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000804e19); }
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000804e1d); };
            c.cmp(t7, 0x40u64, 16);
            if c.cc_ae() { blk = 21; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000804e29); };
            c.w32(8, t8);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xa7u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x7000804e3c);
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x7000804e3d); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000804e47); };
            c.w32(9, t11);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 24; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000804e5b); };
            c.w32(11, t12);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, 0xffffffffffffffffu64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            let r13 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r13);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_le() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(3, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_ae() { blk = 25; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000804e8a); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000804e92); };
            c.w32(2, t16);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000804ea1); };
            c.w32(0, t17);
            let r18 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r18);
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000804ea7); };
            c.w32(6, t19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(7, c.r64(15));
            blk = 10; continue;
        }
        9 => {
            let r20 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r20);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_ge() { blk = 5; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_ae() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000804ec3); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::fault(0x7000804ecb);
        }
        15 => {
            return RegfileExit::fault(0x7000804ecc);
        }
        16 => {
            let r22 = c.add(c.r64(7), 0x2u64, 64);
            c.w64(7, r22);
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_b() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x7000804ed6);
        }
        18 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000804ed7); };
            c.w32(0, t23);
            let r24 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000804ede); };
            c.cmp(t25, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000804eea); }
            return c.ret(0x0, 0x7000804eeb);
        }
        20 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000804eec); };
            c.w64(0, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000804ef0); };
            c.w64(6, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000804ef4); };
            c.w64(7, t28);
            blk = 21; continue;
        }
        21 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000804ef8); };
            c.w32(8, t29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x7000804f08);
        }
        23 => {
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000804f09); };
            c.w32(2, t30);
            let r31 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r31);
            let Some(t32) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000804f0f); };
            c.w64(5, t32);
            let r33 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r33);
            return RegfileExit::jump(0x7000804ff4);
        }
        24 => {
            return RegfileExit::fault(0x7000804f1c);
        }
        25 => {
            return RegfileExit::fault(0x7000804f1d);
        }
        26 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000804f1e); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000804f1f); };
            c.w64(3, t34);
            c.w32(0, 0x1u64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000804f2b); };
            c.w64(6, t35);
            return c.call(0x700086ed74, 0x7000804f34, 0x7000804f2f);
        }
        27 => {
            blk = 19; continue;
        }
        28 => {
            return RegfileExit::jump(0x7000804f38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000804f54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000804f54); };
            c.w32(7, t0);
            let r1 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000804f5a); };
            c.cmp(t2, c.r32(7), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000804f63); };
            c.w32(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000804f66); };
            c.cmp(t4, c.r32(2), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x7000804f72);
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000804f73); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            return c.ret(0x0, 0x7000804f79);
        }
        4 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000804f7a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000804f7b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000804f7c); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000804f7d); };
            if !c.push64(t7) { return RegfileExit::fault(0x7000804f7d); }
            c.w32(0, 0x4u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2050)) else { return RegfileExit::fault(0x7000804f86); };
            c.w64(3, t8);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x7000804f8d); }
            return RegfileExit::jump(0x700086ed74);
        }
        5 => {
            return RegfileExit::jump(0x7000804f98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000804fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000804fb4); };
            c.w32(3, t0);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x7000804fba);
        }
        1 => {
            return RegfileExit::jump(0x7000804fbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000804fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000804fd4); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000804fda); };
            c.w32(3, t2);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x7000804fe0);
        }
        1 => {
            return RegfileExit::jump(0x7000804fe4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000804ff4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000804ffd); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805004); };
            c.w32(9, t2);
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 218; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805015); };
            c.w32(11, t3);
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700080502b); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000805038); };
            c.w32(12, t5);
            c.w64(15, c.r64(12));
            let r6 = c.r32(15) & 0xc0u64;
            c.w32(15, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(15), 0xc0u64, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805050); };
            c.w32(15, t7);
            let r8 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r8);
            let Some(t9) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000805057); };
            c.w32(15, t9);
            let r10 = c.r32(15) & 0x1fu64;
            c.w32(15, r10);
            c.logic(r10, 32);
            let r11 = c.sub(c.r32(15), 0x10u64, 32);
            c.w32(15, r11);
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_a() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700080506f); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000805076); };
            c.cmp(t14, c.r32(8), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080507f); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000805083); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x700080508f);
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805090); };
            c.w32(0, t17);
            let r18 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r18);
            return c.ret(0x0, 0x7000805097);
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000805098); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700080509c); }
            let r19 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r19);
            blk = 16; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008050a4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008050a8); }
            let r20 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r20);
            blk = 16; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008050b0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008050b4); }
            let r21 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r21);
            blk = 16; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008050bc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008050c0); }
            let r22 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008050c6); }
            blk = 220; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008050cf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008050d3); }
            let r23 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r23);
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008050d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008050dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008050e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008050e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008050e9); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 213; continue; }
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t24) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000805106); };
            c.w32(12, t24);
            let r25 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r25);
            c.logic(c.r32(9) & 0xffc00u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 19; continue;
        }
        19 => {
            let r26 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r26);
            let r27 = c.r32(9) & 0x3ffu64;
            c.w32(9, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_be() { blk = 33; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t28) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x700080512f); };
            c.w32(15, (t28 as u16 as i16 as i64 as u64));
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805135); };
            c.w32(3, t29);
            let r30 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r30);
            let r31 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r31);
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000805141); }
            c.w64(11, c.r64(7));
            blk = 21; continue;
        }
        21 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000805148); };
            c.w64(0, t32);
            let r33 = c.sub(c.r32(0), c.r32(11), 32);
            c.w32(0, r33);
            let r34 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r34);
            let r35 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r35);
            c.w32(6, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700080515a); };
            c.w32(6, t36);
            let r37 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r37);
            let r38 = c.shr(c.r32(6), 0x9u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r38);
            let r39 = c.r32(6) & 0x3ffu64;
            c.w32(6, r39);
            c.logic(r39, 32);
            c.w32(6, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805170); };
            c.w32(6, t40);
            let r41 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r41);
            let Some(t42) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805178); };
            c.w32(6, t42);
            let r43 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r43);
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_be() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(11, c.r64(0).wrapping_add(0x1));
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000805186); };
            c.w64(0, t44);
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700080518f); }
            blk = 21; continue;
        }
        25 => {
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_l() { blk = 103; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t45) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008051a5); };
            c.w32(0, t45);
            let r46 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r46);
            let r47 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r47);
            let r48 = c.r32(0) & 0x3ffu64;
            c.w32(0, r48);
            c.logic(r48, 32);
            c.w32(6, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008051ba); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008051c2); };
            c.w32(2, t51);
            let r52 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r52);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let r53 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r53);
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008051d6); };
            c.w64(2, t54);
            blk = 25; continue;
        }
        29 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_be() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(9, c.r64(6));
            blk = 37; continue;
        }
        31 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008051e6); };
            c.w64(2, t55);
            blk = 103; continue;
        }
        32 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008051ef); };
            c.w64(2, t56);
            blk = 103; continue;
        }
        33 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r57 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r57);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 103; continue; }
            blk = 34; continue;
        }
        34 => {
            let r58 = c.sub(c.r64(9), 0x3u64, 64);
            c.w64(9, r58);
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080520d); };
            c.cmp(t59, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ne() { blk = 34; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 103; continue;
        }
        37 => {
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t60) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000805223); };
            c.w32(11, t60);
            c.w32(15, c.r32(11));
            c.w64(0, c.r64(11));
            let r61 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000805230); }
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t62) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080523a); };
            c.w32(8, t62);
            let r63 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000805242); };
            c.w64(12, t64);
            blk = 52; continue;
        }
        39 => {
            c.w64(9, c.r64(0));
            if let Some(sr) = c.shr(c.r64(9), 0x13u64, 64) { c.w64(9, sr); }
            let Some(t65) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000805252); };
            c.w32(11, t65);
            let r66 = c.r64(9) & 0x3ffu64;
            c.w64(9, r66);
            c.logic(r66, 64);
            let r67 = c.r32(15) & 0x380u64;
            c.w32(15, r67);
            c.logic(r67, 32);
            let r68 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r68);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t69) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805277); };
            c.w32(8, t69);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 44; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000805281); };
            c.w64(12, t70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805285); };
            c.w32(15, t71);
            let r72 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r72);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000805293); };
            c.w64(15, t73);
            blk = 42; continue;
        }
        42 => {
            let r74 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r74);
            let Some(t75) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080529d); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008052af); }
            blk = 46; continue;
        }
        44 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008052bd); };
            c.w64(8, t77);
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008052c1); };
            c.w32(9, t78);
            let r79 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r79);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008052c9); }
            c.w64(12, c.r64(8));
            blk = 46; continue;
        }
        46 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008052d2); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008052d9); };
            c.w64(9, t81);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008052e0); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008052e5); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008052ec); };
            c.cmp(t83, c.r64(11), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008052fb, 0x70008052f6);
        }
        48 => {
            c.w64(8, c.r64(0));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008052fe); };
            c.w64(12, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000805302); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000805306); };
            c.w64(0, t86);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700080530c); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 50; continue;
        }
        50 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000805313); };
            c.w64(9, t87);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080531a); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700080531e); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000805323); }
            blk = 52; continue;
        }
        51 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080532b); };
            c.w64(2, t88);
            let Some(t89) = c.ld32(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080532f); };
            c.w32(8, t89);
            let r90 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r90);
            c.w64(12, c.r64(2));
            blk = 52; continue;
        }
        52 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080533a); };
            c.w64(6, t91);
            c.w64(2, c.r64(12));
            blk = 62; continue;
        }
        53 => {
            let Some(t92) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805346); };
            c.w32(9, t92);
            let r93 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r93);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000805353); };
            c.w64(9, t94);
            blk = 55; continue;
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080535a); };
            c.w32(12, t95);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r96 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r96);
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805368); };
            c.w32(15, t97);
            let r98 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r98);
            let r99 = c.r64(15) & c.r64(12);
            c.w64(15, r99);
            c.logic(r99, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(11, c.r64(7).wrapping_mul(4));
            let Some(t100) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805382); };
            c.w32(0, t100);
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000805387); };
            c.cmp(t101, c.r32(0), 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            let r102 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r102);
            c.w64(11, c.r64(3).wrapping_add(c.r64(15)));
            c.w64(15, c.r64(12));
            let r103 = c.r64(15) & c.r64(11);
            c.w64(15, r103);
            c.logic(r103, 64);
            blk = 56; continue;
        }
        59 => {
            let Some(t104) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008053a8); };
            c.w32(8, t104);
            let Some(t105) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008053ac); };
            c.w32(12, t105);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_b() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            return RegfileExit::fault(0x70008053c0);
        }
        61 => {
            let Some(t106) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008053c1); };
            c.w32(8, t106);
            let r107 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r107);
            let r108 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r108);
            c.w64(0, c.r64(12));
            blk = 62; continue;
        }
        62 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 102; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t109) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008053d7); };
            c.w32(9, t109);
            let Some(t110) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008053db); };
            c.cmp(t110, c.r32(9), 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008053e8); };
            c.w32(8, t111);
            let r112 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r112);
            let Some(t113) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008053ef); };
            c.w32(9, t113);
            let r114 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r114);
            let Some(t115) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008053f6); };
            c.logic(t115 & 0x2u64, 8);
            if c.cc_ne() { blk = 83; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000805401); };
            c.cmp(t116, c.r32(9), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700080540a); };
            c.w64(0, t117);
            blk = 67; continue;
        }
        67 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000805414); }
            return c.ret(0x0, 0x7000805415);
        }
        68 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000805416); };
            c.w64(9, t118);
            let Some(t119) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x700080541d); };
            c.cmp(t119, 0x20u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000805427); };
            c.w32(9, t120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080542b); };
            c.cmp(t121, c.r32(9), 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t122) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805438); };
            c.w32(9, t122);
            let r123 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r123);
            let Some(t124) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080543f); };
            c.w32(11, t124);
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000805443); };
            c.cmp(t125, c.r32(11), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t126) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700080544c); };
            c.w32(9, t126);
            let r127 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r127);
            let Some(t128) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805453); };
            c.w32(11, t128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000805457); };
            c.cmp(t129, c.r32(11), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 72; continue;
        }
        72 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 220; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t130) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080546a); };
            c.w32(11, t130);
            let r131 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r131);
            let Some(t132) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805471); };
            c.w32(12, t132);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 78; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000805491); };
            c.cmp(t133, c.r32(11), 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(9, c.r64(2));
            blk = 78; continue;
        }
        77 => {
            let Some(t134) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008054a3); };
            c.w32(9, t134);
            let r135 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r135);
            blk = 78; continue;
        }
        78 => {
            let Some(t136) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008054aa); };
            c.w32(9, t136);
            let r137 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r137);
            let Some(t138) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008054b1); };
            c.w32(9, t138);
            let r139 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r139);
            let Some(t140) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008054b8); };
            c.w32(11, t140);
            let r141 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r141);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 220; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t142) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008054ce); };
            c.w32(6, t142);
            let r143 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r143);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008054d5); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007f6fb4, 0x70008054e0, 0x70008054db);
        }
        80 => {
            blk = 67; continue;
        }
        81 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008054e5); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008054e6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008054e7); }
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008054e8); };
            c.w64(3, t144);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008054f9, 0x70008054f4);
        }
        82 => {
            blk = 67; continue;
        }
        83 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008054fe); }
            c.w64(7, c.r64(8));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000805509, 0x7000805504);
        }
        84 => {
            blk = 67; continue;
        }
        85 => {
            let Some(t145) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080550e); };
            c.w32(9, t145);
            let r146 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r146);
            let Some(t147) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805515); };
            c.w32(11, t147);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 100; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 92; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 220; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t148) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080553f); };
            c.w32(8, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000805543); };
            c.cmp(t149, c.r32(8), 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t150) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805550); };
            c.w32(8, t150);
            let r151 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r151);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 220; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805561); };
            c.w32(9, t152);
            let r153 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r153);
            let Some(t154) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805568); };
            c.cmp(t154, 0x40u64, 16);
            if c.cc_ae() { blk = 220; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t155) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805574); };
            c.w32(8, t155);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 67; continue;
        }
        92 => {
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805581); };
            c.w32(8, t156);
            let Some(t157) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000805585); };
            c.cmp(t157, c.r32(8), 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t158) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000805592); };
            c.w32(8, t158);
            c.w64(9, c.r64(8));
            let r159 = c.r32(9) & 0xc0u64;
            c.w32(9, r159);
            c.logic(r159, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t160) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008055aa); };
            c.w32(9, t160);
            let r161 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r161);
            let Some(t162) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008055b1); };
            c.w32(9, t162);
            let r163 = c.r32(9) & 0x1fu64;
            c.w32(9, r163);
            c.logic(r163, 32);
            let r164 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r164);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_a() { blk = 220; continue; }
            blk = 95; continue;
        }
        95 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 220; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t165) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008055d1); };
            c.w32(0, t165);
            let r166 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r166);
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008055d7); };
            c.cmp(t167, c.r32(0), 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t168) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008055e4); };
            c.w32(8, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008055e8); };
            c.cmp(t169, c.r32(8), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            blk = 67; continue;
        }
        99 => {
            let Some(t170) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008055f6); };
            c.w32(0, t170);
            let r171 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r171);
            blk = 67; continue;
        }
        100 => {
            let Some(t172) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805601); };
            c.w32(8, t172);
            let Some(t173) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000805605); };
            c.cmp(t173, c.r32(8), 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t174) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805612); };
            c.w32(0, t174);
            let r175 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r175);
            blk = 67; continue;
        }
        102 => {
            c.w64(0, c.r64(8));
            blk = 67; continue;
        }
        103 => {
            let Some(t176) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805625); };
            c.w32(9, t176);
            let Some(t177) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000805629); };
            c.cmp(t177, c.r32(9), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t178) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805632); };
            c.w32(9, t178);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 105; continue;
        }
        105 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_e() { blk = 211; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000805646); };
            c.w64(6, t179);
            blk = 220; continue;
        }
        107 => {
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080564f); };
            c.w64(11, t180);
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(11), 0x82eu64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805660); };
            c.w32(8, t181);
            let r182 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r182);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000805667); }
            let Some(t183) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700080566b); };
            c.cmp(t183, c.r32(8), 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805678); };
            c.w32(9, t184);
            let r185 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r185);
            let Some(t186) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080567f); };
            c.w32(11, t186);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000805684); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 137; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t187) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805694); };
            c.w32(12, t187);
            c.logic(c.r32(12) & 0x200000u64, 32);
            if c.cc_ne() { blk = 131; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008056a5); };
            c.w32(15, t188);
            let r189 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r189);
            c.logic(c.r32(12) & 0xffc00u64, 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 113; continue;
        }
        113 => {
            let r190 = c.shr(c.r32(12), 0xau64, 32).unwrap_or(c.r32(12));
            c.w32(12, r190);
            let r191 = c.r32(12) & 0x3ffu64;
            c.w32(12, r191);
            c.logic(r191, 32);
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_be() { blk = 127; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t192) = c.ld16(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008056ce); };
            c.w32(0, (t192 as u16 as i16 as i64 as u64));
            let Some(t193) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008056d3); };
            c.w32(3, t193);
            let r194 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r194);
            let r195 = c.sub(c.r32(0), 0x1u64, 32);
            c.w32(0, r195);
            c.w64(7, c.r64(0));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 115; continue;
        }
        115 => {
            c.w64(2, c.r64(7));
            let r196 = c.sub(c.r32(2), c.r32(6), 32);
            c.w32(2, r196);
            let r197 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r197);
            let r198 = c.add(c.r32(2), c.r32(6), 32);
            c.w32(2, r198);
            c.w32(9, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t199) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008056f1); };
            c.w32(9, t199);
            let r200 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r200);
            let r201 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r201);
            let r202 = c.r32(9) & 0x3ffu64;
            c.w32(9, r202);
            c.logic(r202, 32);
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t203) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080570b); };
            c.w32(9, t203);
            let r204 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r204);
            let Some(t205) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805713); };
            c.w32(9, t205);
            let r206 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r206);
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_be() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(6, c.r64(2).wrapping_add(0x1));
            c.w64(2, c.r64(7));
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(7, c.r64(2));
            blk = 115; continue;
        }
        119 => {
            c.cmp(c.r32(0), c.r32(6), 32);
            if c.cc_l() { blk = 126; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w32(9, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t207) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700080573a); };
            c.w32(9, t207);
            let r208 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r208);
            let r209 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r209);
            let r210 = c.r32(9) & 0x3ffu64;
            c.w32(9, r210);
            c.logic(r210, 32);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t211) = c.ld32(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805754); };
            c.w32(7, t211);
            let r212 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r212);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700080575c); }
            let Some(t213) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805760); };
            c.w32(2, t213);
            let r214 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r214);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            let r215 = c.add(c.r32(6), 0x1u64, 32);
            c.w32(6, r215);
            blk = 119; continue;
        }
        123 => {
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_a() { blk = 149; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080577c); };
            c.w64(2, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000805780); };
            c.w64(9, t217);
            blk = 148; continue;
        }
        125 => {
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000805789); };
            c.w64(2, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080578d); };
            c.w64(9, t219);
            blk = 148; continue;
        }
        126 => {
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000805796); };
            c.w64(2, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080579a); };
            c.w64(9, t221);
            blk = 148; continue;
        }
        127 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r222 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r222);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 148; continue; }
            blk = 128; continue;
        }
        128 => {
            let r223 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r223);
            let Some(t224) = c.ld32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008057b8); };
            c.cmp(t224, c.r32(1), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 128; continue; }
            blk = 130; continue;
        }
        130 => {
            blk = 148; continue;
        }
        131 => {
            let Some(t225) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008057ce); };
            c.w32(12, t225);
            let r226 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r226);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008057db); };
            c.w64(12, t227);
            blk = 133; continue;
        }
        133 => {
            let Some(t228) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008057e2); };
            c.w32(15, t228);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r229 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r229);
            let Some(t230) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008057f1); };
            c.w32(0, t230);
            let r231 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r231);
            let r232 = c.r64(0) & c.r64(15);
            c.w64(0, r232);
            c.logic(r232, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 134; continue;
        }
        134 => {
            c.w64(6, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(7, c.r64(6).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000805809); }
            let Some(t233) = c.ld32(c.r64(12).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080580d); };
            c.w32(6, t233);
            let Some(t234) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000805812); };
            c.cmp(t234, c.r32(6), 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 136; continue;
        }
        136 => {
            let r235 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r235);
            let r236 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r236);
            c.w64(6, c.r64(15));
            let r237 = c.r64(6) & c.r64(0);
            c.w64(6, r237);
            c.logic(r237, 64);
            c.w64(0, c.r64(6));
            blk = 134; continue;
        }
        137 => {
            c.cmp(c.r16(11), 0xbdu64, 16);
            if c.cc_ne() { blk = 210; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t238) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000805845); };
            c.logic(t238 & 0x24u64, 8);
            if c.cc_ne() { blk = 210; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t239) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805850); };
            c.w32(12, t239);
            let r240 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r240);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700080585d); };
            c.w64(12, t241);
            blk = 141; continue;
        }
        141 => {
            let Some(t242) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805864); };
            c.w32(15, t242);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r243 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r243);
            let Some(t244) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805873); };
            c.w32(0, t244);
            let r245 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r245);
            let r246 = c.r64(0) & c.r64(15);
            c.w64(0, r246);
            c.logic(r246, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 142; continue;
        }
        142 => {
            c.w64(6, c.r64(0).wrapping_add(0x5));
            let Some(t247) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805882); };
            c.w32(7, t247);
            let r248 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700080588a); };
            c.cmp(t249, c.r32(7), 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t250) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000805893); };
            c.cmp(t250, c.r32(7), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t251) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080589c); };
            c.cmp(t251, c.r32(1), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            let r252 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r252);
            let r253 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r253);
            c.w64(6, c.r64(15));
            let r254 = c.r64(6) & c.r64(0);
            c.w64(6, r254);
            c.logic(r254, 64);
            c.w64(0, c.r64(6));
            blk = 142; continue;
        }
        146 => {
            let Some(t255) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008058b3); };
            c.w32(12, t255);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(6), c.r64(12), 64);
            if c.cc_ae() { blk = 168; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t256) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008058c7); };
            c.w32(12, t256);
            let r257 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r257);
            let Some(t258) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008058ce); };
            c.cmp(t258, c.r32(12), 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(8, c.r64(9));
            blk = 108; continue;
        }
        149 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008058e3); };
            c.w64(12, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008058e7); };
            c.w64(2, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008058eb); };
            c.w64(9, t261);
            blk = 150; continue;
        }
        150 => {
            c.w64(11, c.r64(12).wrapping_mul(4));
            let Some(t262) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008058f7); };
            c.w32(12, t262);
            c.w32(0, c.r32(12));
            c.w64(3, c.r64(12));
            let r263 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r263);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000805904); }
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t264) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080590e); };
            c.w32(12, t264);
            let r265 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r265);
            c.w64(9, c.r64(12));
            blk = 170; continue;
        }
        152 => {
            c.w64(11, c.r64(3));
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t266) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000805925); };
            c.w32(12, t266);
            let r267 = c.r64(11) & 0x3ffu64;
            c.w64(11, r267);
            c.logic(r267, 64);
            let r268 = c.r32(0) & 0x380u64;
            c.w32(0, r268);
            c.logic(r268, 32);
            let r269 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r269);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let Some(t270) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805946); };
            c.w32(9, t270);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 157; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t271) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805950); };
            c.w32(15, t271);
            let r272 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r272);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700080595d); };
            c.w64(15, t273);
            blk = 155; continue;
        }
        155 => {
            let r274 = c.sub(c.r64(11), c.r64(9), 64);
            c.w64(11, r274);
            let Some(t275) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805967); };
            c.w32(9, t275);
            let r276 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r276);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 156; continue;
        }
        156 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000805979); }
            blk = 159; continue;
        }
        157 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 164; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t277) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805987); };
            c.w32(9, t277);
            let r278 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r278);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700080598f); }
            blk = 159; continue;
        }
        159 => {
            let Some(t279) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000805995); };
            c.w64(9, t279);
            let Some(t280) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700080599c); };
            c.w64(11, t280);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008059a3); }
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008059a8); };
            c.w64(15, t281);
            let Some(t282) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008059af); };
            c.cmp(t282, c.r64(12), 64);
            if c.cc_a() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x70008059c1, 0x70008059bc);
        }
        161 => {
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008059c1); };
            c.w64(3, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008059c5); };
            c.w64(2, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008059c9); };
            c.w64(1, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008059cd); };
            c.w64(8, t286);
            blk = 163; continue;
        }
        162 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008059d3); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 163; continue;
        }
        163 => {
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008059dd); };
            c.w64(9, t287);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008059e4); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008059e8); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008059ed); }
            c.w64(9, c.r64(0));
            blk = 170; continue;
        }
        164 => {
            let Some(t288) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008059f7); };
            c.w32(12, t288);
            let r289 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r289);
            c.w64(9, c.r64(12));
            blk = 170; continue;
        }
        165 => {
            let Some(t290) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805a04); };
            c.w32(9, t290);
            let Some(t291) = c.ld32(c.r64(12).wrapping_add(c.r64(7)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805a09); };
            c.w32(11, t291);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000805a14); };
            c.w64(15, t292);
            let r293 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r293);
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            return RegfileExit::fault(0x7000805a21);
        }
        167 => {
            let Some(t294) = c.ld32(c.r64(12).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805a22); };
            c.w32(12, t294);
            let r295 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r295);
            let r296 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r296);
            c.w64(9, c.r64(12));
            c.w64(3, c.r64(11));
            blk = 170; continue;
        }
        168 => {
            return RegfileExit::fault(0x7000805a35);
        }
        169 => {
            let Some(t297) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805a36); };
            c.w32(9, t297);
            let r298 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r298);
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(12));
            blk = 170; continue;
        }
        170 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 209; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t299) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805a4c); };
            c.w32(11, t299);
            let Some(t300) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000805a50); };
            c.cmp(t300, c.r32(11), 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t301) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805a5d); };
            c.w32(9, t301);
            let r302 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r302);
            let Some(t303) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805a64); };
            c.w32(11, t303);
            let r304 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r304);
            let Some(t305) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000805a6b); };
            c.logic(t305 & 0x2u64, 8);
            if c.cc_ne() { blk = 191; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t306) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000805a76); };
            c.cmp(t306, c.r32(11), 32);
            if c.cc_e() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000805a7f); };
            c.w64(0, t307);
            blk = 67; continue;
        }
        175 => {
            let Some(t308) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000805a8b); };
            c.w64(11, t308);
            let Some(t309) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000805a92); };
            c.cmp(t309, 0x20u64, 8);
            if c.cc_e() { blk = 189; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t310) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000805a9c); };
            c.w32(11, t310);
            let Some(t311) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000805aa0); };
            c.cmp(t311, c.r32(11), 32);
            if c.cc_ne() { blk = 210; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t312) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805aad); };
            c.w32(11, t312);
            let r313 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r313);
            let Some(t314) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805ab4); };
            c.w32(12, t314);
            let Some(t315) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000805ab8); };
            c.cmp(t315, c.r32(12), 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t316) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000805ac1); };
            c.w32(11, t316);
            let r317 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r317);
            let Some(t318) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805ac8); };
            c.w32(12, t318);
            let Some(t319) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000805acc); };
            c.cmp(t319, c.r32(12), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 179; continue;
        }
        179 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t320) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805adf); };
            c.w32(12, t320);
            let r321 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r321);
            let Some(t322) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805ae6); };
            c.w32(15, t322);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 185; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t323) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000805b05); };
            c.cmp(t323, c.r32(12), 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 183; continue;
        }
        183 => {
            blk = 186; continue;
        }
        184 => {
            let Some(t324) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805b14); };
            c.w32(8, t324);
            let r325 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r325);
            blk = 186; continue;
        }
        185 => {
            c.w64(8, c.r64(11));
            blk = 186; continue;
        }
        186 => {
            let Some(t326) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805b20); };
            c.w32(8, t326);
            let r327 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r327);
            let Some(t328) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805b27); };
            c.w32(8, t328);
            let r329 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r329);
            let Some(t330) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805b2e); };
            c.w32(11, t330);
            let r331 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r331);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 210; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t332) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000805b44); };
            c.w32(6, t332);
            let r333 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r333);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000805b4b); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x7000805b56, 0x7000805b51);
        }
        188 => {
            blk = 67; continue;
        }
        189 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000805b5b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000805b5d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000805b5e); }
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000805b5f); };
            c.w64(3, t334);
            c.w32(0, 0x3u64);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000805b6b); };
            c.w64(6, t335);
            return c.call(0x700086ed74, 0x7000805b74, 0x7000805b6f);
        }
        190 => {
            blk = 67; continue;
        }
        191 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000805b79); }
            c.w64(7, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000805b7f); };
            c.w64(6, t336);
            return c.call(0x70007f2274, 0x7000805b88, 0x7000805b83);
        }
        192 => {
            blk = 67; continue;
        }
        193 => {
            let Some(t337) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805b8d); };
            c.w32(11, t337);
            let r338 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r338);
            let Some(t339) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805b94); };
            c.w32(12, t339);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 207; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 200; continue; }
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 210; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t340) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805bbf); };
            c.w32(9, t340);
            let Some(t341) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000805bc3); };
            c.cmp(t341, c.r32(9), 32);
            if c.cc_ne() { blk = 210; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t342) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805bd0); };
            c.w32(8, t342);
            let r343 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r343);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t344) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805be1); };
            c.w32(9, t344);
            let r345 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r345);
            let Some(t346) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805be8); };
            c.cmp(t346, 0x40u64, 16);
            if c.cc_ae() { blk = 210; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t347) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805bf4); };
            c.w32(8, t347);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 67; continue;
        }
        200 => {
            let Some(t348) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805c01); };
            c.w32(9, t348);
            let Some(t349) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000805c05); };
            c.cmp(t349, c.r32(9), 32);
            if c.cc_ne() { blk = 210; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t350) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000805c12); };
            c.w32(9, t350);
            c.w64(11, c.r64(9));
            let r351 = c.r32(11) & 0xc0u64;
            c.w32(11, r351);
            c.logic(r351, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t352) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805c2a); };
            c.w32(11, t352);
            let r353 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r353);
            let Some(t354) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000805c31); };
            c.w32(11, t354);
            let r355 = c.r32(11) & 0x1fu64;
            c.w32(11, r355);
            c.logic(r355, 32);
            let r356 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r356);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 210; continue; }
            blk = 203; continue;
        }
        203 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 210; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t357) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000805c49); };
            c.w32(0, t357);
            let r358 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r358);
            let Some(t359) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000805c50); };
            c.cmp(t359, c.r32(0), 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t360) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805c59); };
            c.w32(8, t360);
            let Some(t361) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000805c5d); };
            c.cmp(t361, c.r32(8), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t362) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805c6a); };
            c.w32(0, t362);
            let r363 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r363);
            blk = 67; continue;
        }
        207 => {
            let Some(t364) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805c75); };
            c.w32(9, t364);
            let Some(t365) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000805c79); };
            c.cmp(t365, c.r32(9), 32);
            if c.cc_ne() { blk = 210; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t366) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805c82); };
            c.w32(0, t366);
            let r367 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r367);
            blk = 67; continue;
        }
        209 => {
            c.w64(0, c.r64(9));
            blk = 67; continue;
        }
        210 => {
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000805c96); };
            c.w64(6, t368);
            blk = 220; continue;
        }
        211 => {
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000805c9f); };
            c.w64(0, t369);
            blk = 67; continue;
        }
        212 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000805cab); };
            c.w64(6, t370);
            blk = 220; continue;
        }
        213 => {
            c.cmp(c.r16(11), 0xbcu64, 16);
            if c.cc_ne() { blk = 220; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(3, c.r64(1));
            let Some(t371) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805cbc); };
            c.w32(8, t371);
            let Some(t372) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000805cc0); };
            c.cmp(t372, c.r32(8), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t373) = c.ld8(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805cc9); };
            c.logic(t373 & 0x1u64, 8);
            if c.cc_e() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(1, c.r64(3));
            blk = 220; continue;
        }
        217 => {
            let Some(t374) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000805cd4); };
            c.w64(5, t374);
            c.w64(0, c.r64(2));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            let r375 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r375);
            return RegfileExit::jump(0x70008bc174);
        }
        218 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000805cec); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000805cf0); }
            let r376 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r376);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000805cf6); }
            blk = 220; continue;
        }
        219 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000805cfc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000805d00); }
            let r377 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r377);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000805d06); }
            blk = 220; continue;
        }
        220 => {
            let Some(t378) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000805d0a); };
            c.w64(5, t378);
            let Some(t379) = c.ld64(c.r64(13).wrapping_add(0x2058)) else { return RegfileExit::fault(0x7000805d0e); };
            c.w64(3, t379);
            c.w32(0, 0x3u64);
            let Some(t380) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x7000805d1a); };
            c.w64(10, t380);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x7000805d1f); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(2)) { return RegfileExit::fault(0x7000805d24); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(1)) { return RegfileExit::fault(0x7000805d29); }
            let Some(t381) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000805d2e); };
            c.w64(10, t381);
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(10)) { return RegfileExit::fault(0x7000805d33); }
            let r382 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r382);
            return RegfileExit::jump(0x700086ed74);
        }
        221 => {
            return RegfileExit::jump(0x7000805d44);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000805d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000805d54); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000805d55); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000805d56); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000805d57); }
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000805d58); };
            if !c.push64(t0) { return RegfileExit::fault(0x7000805d58); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x7000805d5c); }
            c.w32(0, 0x5u64);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2090)) else { return RegfileExit::fault(0x7000805d66); };
            c.w64(3, t1);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x7000805d74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000805d94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000805d94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000805d98); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000805d9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000805da2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000805da6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000805daa); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000805dae); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 405; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805dbb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805dc2); };
            c.w32(9, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000805dc7); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000805dcb); }
            c.cmp(c.r16(9), 0x410u64, 16);
            if c.cc_be() { blk = 405; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000805ddb); };
            c.w32(11, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000805ddf); }
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_ne() { blk = 249; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000805df0); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            c.logic(c.r32(11) & 0xffc00u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 4; continue;
        }
        4 => {
            let r7 = c.shr(c.r32(11), 0xau64, 32).unwrap_or(c.r32(11));
            c.w32(11, r7);
            let r8 = c.r32(11) & 0x3ffu64;
            c.w32(11, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_be() { blk = 17; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000805e19); };
            c.w32(15, (t9 as u16 as i16 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805e1f); };
            c.w32(3, t10);
            let r11 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let r12 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000805e29); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000805e31); }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000805e35); };
            c.w64(6, t13);
            let r14 = c.sub(c.r32(6), c.r32(7), 32);
            c.w32(6, r14);
            let r15 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r15);
            let r16 = c.add(c.r32(6), c.r32(7), 32);
            c.w32(6, r16);
            c.w32(9, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000805e46); };
            c.w32(9, t17);
            let r18 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r18);
            let r19 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r19);
            let r20 = c.r32(9) & 0x3ffu64;
            c.w32(9, r20);
            c.logic(r20, 32);
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805e60); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805e68); };
            c.w32(9, t23);
            let r24 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r24);
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, c.r64(6).wrapping_add(0x1));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000805e78); };
            c.w64(6, t25);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(7), c.r32(6), 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            let r26 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r26);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(9, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000805e95); };
            c.w32(9, t27);
            let r28 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r28);
            let r29 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r29);
            let r30 = c.r32(9) & 0x3ffu64;
            c.w32(9, r30);
            c.logic(r30, 32);
            c.w32(6, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805eaf); };
            c.w32(0, t31);
            let r32 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r32);
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805eb7); };
            c.w32(2, t33);
            let r34 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r34);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_be() { blk = 142; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(6));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000805ed5); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000805ed9); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000805edd); };
            c.w64(2, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000805ee1); };
            c.w64(6, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000805ee5); };
            c.w64(9, t39);
            blk = 21; continue;
        }
        17 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r40 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r40);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 142; continue; }
            blk = 18; continue;
        }
        18 => {
            let r41 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r41);
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000805f00); };
            c.cmp(t42, c.r32(1), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 142; continue;
        }
        21 => {
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t43) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000805f16); };
            c.w32(15, t43);
            c.w32(3, c.r32(15));
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 361; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000805f38); };
            c.w32(8, t44);
            let r45 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r45);
            blk = 353; continue;
        }
        25 => {
            let r46 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r46);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t47) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000805f4c); };
            c.w32(9, t47);
            let r48 = c.r64(15) & 0x3ffu64;
            c.w64(15, r48);
            c.logic(r48, 64);
            let r49 = c.r32(3) & 0x380u64;
            c.w32(3, r49);
            c.logic(r49, 32);
            let r50 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r50);
            c.cmp(c.r32(3), 0x100u64, 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t51) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805f6f); };
            c.w32(8, t51);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 30; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t52) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000805f79); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000805f86); };
            c.w64(12, t54);
            blk = 28; continue;
        }
        28 => {
            let r55 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000805f90); };
            c.w32(8, t56);
            let r57 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r57);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 353; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000805fa2); }
            blk = 32; continue;
        }
        30 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t58) = c.ld32(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805fb0); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000805fb8); }
            blk = 32; continue;
        }
        32 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000805fbe); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000805fc5); };
            c.w64(9, t61);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000805fcc); };
            c.w64(12, t62);
            let Some(t63) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000805fd3); };
            c.cmp(t63, c.r64(11), 64);
            if c.cc_be() { blk = 406; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000805fdd); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 34; continue;
        }
        34 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000805fe4); };
            c.w64(9, t64);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000805feb); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000805fef); }
            blk = 353; continue;
        }
        35 => {
            let Some(t65) = c.ld32(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000805ffa); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            blk = 353; continue;
        }
        36 => {
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000806007); };
            c.cmp(t67, c.r32(1), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t68) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000806014); };
            c.cmp(t68, c.r32(1), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000806021); };
            c.cmp(t69, c.r32(1), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x700080602e); };
            c.cmp(t70, c.r32(1), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x700080603b); };
            c.cmp(t71, c.r32(1), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000806048); };
            c.cmp(t72, c.r32(1), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000806055); };
            c.cmp(t73, c.r32(1), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t74) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000806068); };
            c.cmp(t74, c.r32(0), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 141; continue;
        }
        46 => {
            c.w64(9, c.r64(3));
            let r75 = c.r32(9) & 0x380u64;
            c.w32(9, r75);
            c.logic(r75, 32);
            c.cmp(c.r32(9), 0x80u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(9), 0x100u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(9), 0x180u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 61; continue;
        }
        51 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 405; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t76) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008060b5); };
            c.w32(11, t76);
            let r77 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r77);
            c.w32(12, 0x4u64);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(12, 0x2u64);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 55; continue;
        }
        55 => {
            let r78 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r78);
            c.logic(r78, 64);
            let Some(t79) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008060e5); };
            c.cmp(t79, c.r32(11), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 405; continue;
        }
        57 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t80) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008060f4); };
            c.w32(11, t80);
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008060f8); };
            c.cmp(t81, c.r32(11), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 61; continue;
        }
        60 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 405; continue; }
            blk = 61; continue;
        }
        61 => {
            let r82 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r82);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t83) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000806116); };
            c.w32(11, t83);
            let Some(t84) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080611b); };
            c.w32(8, t84);
            let r85 = c.r32(3) & 0x8u64;
            c.w32(3, r85);
            c.logic(r85, 32);
            let r86 = c.r64(15) & 0x3ffu64;
            c.w64(15, r86);
            c.logic(r86, 64);
            c.cmp(c.r32(9), 0x100u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(3), 0x8u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r87 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r87);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 103; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t88) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806150); };
            c.w32(15, t88);
            let r89 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r89);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700080615d); };
            c.w64(15, t90);
            blk = 64; continue;
        }
        64 => {
            let r91 = c.sub(c.r64(11), c.r64(8), 64);
            c.w64(11, r91);
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700080617f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r92 = c.r64(9) & c.r64(15);
            c.w64(9, r92);
            c.logic(r92, 64);
            let Some(t93) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000806191); };
            c.logic(t93 & 0x4u64, 8);
            if c.cc_ne() { blk = 512; continue; }
            blk = 68; continue;
        }
        68 => {
            blk = 141; continue;
        }
        69 => {
            let Some(t94) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008061a1); };
            c.w32(8, t94);
            let r95 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r95);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 70; continue;
        }
        70 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t96) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008061c8); };
            c.w32(9, t96);
            let Some(t97) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008061cc); };
            c.cmp(t97, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t98) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008061d9); };
            c.w32(9, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008061dd); };
            c.cmp(t99, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 74; continue;
        }
        74 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008061ea); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008061f0); }
            blk = 80; continue;
        }
        75 => {
            let Some(t100) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008061f7); };
            c.w32(9, t100);
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008061fb); };
            c.cmp(t101, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(9, c.r64(0));
            let r102 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r102);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080620e); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 80; continue;
        }
        77 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t103) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806229); };
            c.w32(9, t103);
            let Some(t104) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080622d); };
            c.cmp(t104, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 79; continue;
        }
        79 => {
            let r105 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r105);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080623d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 80; continue;
        }
        80 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            if c.cc_e() { blk = 87; continue; }
            blk = 82; continue;
        }
        82 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 84; continue;
        }
        84 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 141; continue; }
            blk = 85; continue;
        }
        85 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 86; continue;
        }
        86 => {
            blk = 141; continue;
        }
        87 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 88; continue;
        }
        88 => {
            blk = 405; continue;
        }
        89 => {
            let Some(t106) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806294); };
            c.w32(8, t106);
            let r107 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r107);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008062a0); }
            blk = 92; continue;
        }
        91 => {
            c.w64(9, c.r64(0));
            let r108 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r108);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008062bc); }
            blk = 141; continue;
        }
        94 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008062c7); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            if c.cc_e() { blk = 101; continue; }
            blk = 96; continue;
        }
        96 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 98; continue;
        }
        98 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 141; continue; }
            blk = 99; continue;
        }
        99 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 100; continue;
        }
        100 => {
            blk = 141; continue;
        }
        101 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 405; continue;
        }
        103 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700080632e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r109 = c.r64(9) & c.r64(2);
            c.w64(9, r109);
            c.logic(r109, 64);
            let Some(t110) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000806340); };
            c.logic(t110 & 0x4u64, 8);
            if c.cc_ne() { blk = 509; continue; }
            blk = 107; continue;
        }
        107 => {
            blk = 141; continue;
        }
        108 => {
            let Some(t111) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806350); };
            c.w32(8, t111);
            let r112 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r112);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 109; continue;
        }
        109 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 114; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t113) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806377); };
            c.w32(9, t113);
            let Some(t114) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080637b); };
            c.cmp(t114, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t115) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806388); };
            c.w32(9, t115);
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080638c); };
            c.cmp(t116, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 113; continue;
        }
        113 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000806399); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080639f); }
            blk = 119; continue;
        }
        114 => {
            let Some(t117) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008063a6); };
            c.w32(9, t117);
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008063aa); };
            c.cmp(t118, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(9, c.r64(0));
            let r119 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r119);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008063bd); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 119; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t120) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008063d8); };
            c.w32(9, t120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008063dc); };
            c.cmp(t121, c.r32(9), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 118; continue;
        }
        118 => {
            let r122 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r122);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008063ec); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 119; continue;
        }
        119 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            if c.cc_e() { blk = 126; continue; }
            blk = 121; continue;
        }
        121 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 123; continue;
        }
        123 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 141; continue; }
            blk = 124; continue;
        }
        124 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 125; continue;
        }
        125 => {
            blk = 141; continue;
        }
        126 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 127; continue;
        }
        127 => {
            blk = 405; continue;
        }
        128 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000806447); }
            blk = 131; continue;
        }
        130 => {
            c.w64(8, c.r64(0));
            let r123 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r123);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 131; continue;
        }
        131 => {
            let Some(t124) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080645d); };
            c.w32(8, t124);
            let r125 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r125);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 140; continue; }
            blk = 132; continue;
        }
        132 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080646b); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            if c.cc_e() { blk = 139; continue; }
            blk = 134; continue;
        }
        134 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 136; continue;
        }
        136 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 140; continue; }
            blk = 137; continue;
        }
        137 => {
            if c.cc_e() { blk = 405; continue; }
            blk = 138; continue;
        }
        138 => {
            blk = 140; continue;
        }
        139 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 405; continue; }
            blk = 140; continue;
        }
        140 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008064ae); }
            blk = 141; continue;
        }
        141 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008064b7); }
            return c.ret(0x0, 0x70008064b8);
        }
        142 => {
            let Some(t126) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008064b9); };
            c.cmp(t126, c.r32(1), 32);
            if c.cc_e() { blk = 440; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008064c6); };
            c.cmp(t127, c.r32(1), 32);
            if c.cc_e() { blk = 439; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t128) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008064d3); };
            c.cmp(t128, c.r32(1), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008064e0); };
            c.cmp(t129, c.r32(1), 32);
            if c.cc_e() { blk = 437; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x70008064ed); };
            c.cmp(t130, c.r32(1), 32);
            if c.cc_e() { blk = 436; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008064fa); };
            c.cmp(t131, c.r32(1), 32);
            if c.cc_e() { blk = 435; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000806507); };
            c.cmp(t132, c.r32(1), 32);
            if c.cc_e() { blk = 434; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t133) = c.ld32(c.r64(8).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000806514); };
            c.w32(8, t133);
            let r134 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r134);
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 433; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 432; continue; }
            blk = 151; continue;
        }
        151 => {
            let r135 = c.r32(9) & 0x3u64;
            c.w32(9, r135);
            c.logic(r135, 32);
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 152; continue;
        }
        152 => {
            let r136 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r136);
            c.logic(r136, 64);
            blk = 179; continue;
        }
        153 => {
            let Some(t137) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806545); };
            c.w32(9, t137);
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0x148)) else { return RegfileExit::fault(0x7000806549); };
            c.cmp(t138, c.r32(9), 32);
            if c.cc_ne() { blk = 408; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806556); };
            c.w32(9, t139);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            c.w8(9, (c.cc_b() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000806572); };
            c.w32(11, t140);
            let r141 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r141);
            blk = 157; continue;
        }
        156 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 431; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_be() { blk = 174; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000806598); };
            c.w32(9, t142);
            let r143 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r143);
            blk = 162; continue;
        }
        161 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 162; continue;
        }
        162 => {
            let Some(t144) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008065a4); };
            c.w32(12, t144);
            let r145 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r145);
            let r146 = c.sub(c.r32(9), 0x1u64, 32);
            c.w32(9, r146);
            c.w64(0, c.r64(9));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 164; continue;
        }
        163 => {
            c.w64(0, c.r64(3));
            blk = 164; continue;
        }
        164 => {
            c.w64(3, c.r64(0));
            let r147 = c.sub(c.r32(3), c.r32(15), 32);
            c.w32(3, r147);
            let r148 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r148);
            let r149 = c.add(c.r32(3), c.r32(15), 32);
            c.w32(3, r149);
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t150) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008065cc); };
            c.w32(2, t150);
            let r151 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r151);
            let Some(t152) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008065d4); };
            c.w32(2, t152);
            let r153 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r153);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_be() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w32(15, c.r64(3).wrapping_add(0x1));
            c.w64(3, c.r64(0));
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 167; continue;
        }
        167 => {
            blk = 169; continue;
        }
        168 => {
            let r154 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r154);
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_l() { blk = 411; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w32(0, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t155) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806601); };
            c.w32(3, t155);
            let r156 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r156);
            let Some(t157) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806609); };
            c.w32(2, t157);
            let r158 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r158);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_ne() { blk = 410; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_ne() { blk = 168; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_be() { blk = 409; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(9, c.r64(0).wrapping_add(0x2));
            blk = 178; continue;
        }
        174 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x2));
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_e() { blk = 430; continue; }
            blk = 175; continue;
        }
        175 => {
            let r159 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r159);
            let Some(t160) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806644); };
            c.cmp(t160, c.r32(1), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_ne() { blk = 175; continue; }
            blk = 177; continue;
        }
        177 => {
            blk = 430; continue;
        }
        178 => {
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000806656); };
            c.w32(8, t161);
            let r162 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r162);
            let r163 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r163);
            c.logic(r163, 64);
            blk = 179; continue;
        }
        179 => {
            let Some(t164) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000806662); };
            c.w32(9, t164);
            let r165 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r165);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000806669); }
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t166) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806675); };
            c.cmp(t166, c.r32(11), 32);
            if c.cc_ne() { blk = 412; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t167) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080667f); };
            c.w32(9, t167);
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 429; continue; }
            blk = 182; continue;
        }
        182 => {
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r168 = c.r64(9) & 0x3ffu64;
            c.w64(9, r168);
            c.logic(r168, 64);
            c.w64(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t169) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008066a4); };
            c.w32(11, t169);
            let r170 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r170);
            let Some(t171) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008066ab); };
            c.cmp(t171, c.r32(1), 32);
            if c.cc_ne() { blk = 428; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t172) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008066b6); };
            c.w32(12, t172);
            c.w32(15, c.r32(12));
            c.logic(c.r8(12) & 0x52u64, 8);
            if c.cc_ne() { blk = 427; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t173) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008066c8); };
            c.w32(0, t173);
            let Some(t174) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008066cb); };
            c.cmp(t174, c.r32(0), 32);
            if c.cc_ne() { blk = 187; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t175) = c.ld8(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008066d4); };
            c.logic(t175 & 0x1u64, 8);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w32(0, 0x1u64);
            blk = 188; continue;
        }
        187 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 188; continue;
        }
        188 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 413; continue; }
            blk = 189; continue;
        }
        189 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 193; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008066fb); };
            c.w64(3, t176);
            let Some(t177) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008066ff); };
            c.cmp(t177, c.r32(3), 32);
            if c.cc_ne() { blk = 414; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080670a); };
            c.w64(2, t178);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700080670e); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r179 = c.r64(9) & c.r64(2);
            c.w64(9, r179);
            c.logic(r179, 64);
            let Some(t180) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080671c); };
            c.logic(t180 & 0x4u64, 8);
            if c.cc_ne() { blk = 506; continue; }
            blk = 192; continue;
        }
        192 => {
            blk = 248; continue;
        }
        193 => {
            let r181 = c.r32(15) & 0x380u64;
            c.w32(15, r181);
            c.logic(r181, 32);
            c.cmp(c.r32(15), 0x80u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r32(15), 0x100u64, 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(15), 0x180u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 196; continue;
        }
        196 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 415; continue; }
            blk = 197; continue;
        }
        197 => {
            blk = 208; continue;
        }
        198 => {
            let Some(t182) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000806761); };
            c.logic(t182 & 0x1u64, 8);
            if c.cc_e() { blk = 419; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t183) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080676b); };
            c.w32(9, t183);
            let r184 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r184);
            c.w32(11, 0x4u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 418; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w32(11, 0x2u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 417; continue; }
            blk = 202; continue;
        }
        202 => {
            let r185 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r185);
            c.logic(r185, 64);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080679b); };
            c.w64(3, t186);
            let Some(t187) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080679f); };
            c.cmp(t187, c.r32(9), 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 203; continue;
        }
        203 => {
            blk = 416; continue;
        }
        204 => {
            let Some(t188) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008067aa); };
            c.logic(t188 & 0x1u64, 8);
            if c.cc_e() { blk = 208; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008067b0); };
            c.w64(3, t189);
            let Some(t190) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008067b4); };
            c.w32(9, t190);
            let Some(t191) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008067b8); };
            c.cmp(t191, c.r32(9), 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(0, c.r64(3));
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008067c4); };
            c.w64(7, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008067c8); };
            c.w64(2, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008067cc); };
            c.w64(6, t194);
            blk = 405; continue;
        }
        207 => {
            let Some(t195) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008067d5); };
            c.logic(t195 & 0x1u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 208; continue;
        }
        208 => {
            let r196 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r196);
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t197) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008067e6); };
            c.w32(9, t197);
            let Some(t198) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008067eb); };
            c.w32(11, t198);
            let r199 = c.r64(12) & 0x3ffu64;
            c.w64(12, r199);
            c.logic(r199, 64);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let r200 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r200);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 235; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 216; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t201) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000806818); };
            c.logic(t201 & 0x1u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080681e); };
            c.w64(12, t202);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000806822); }
            blk = 213; continue;
        }
        212 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080682b); };
            c.w64(12, t203);
            let r204 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r204);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 213; continue;
        }
        213 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080683b); };
            c.w64(12, t205);
            let Some(t206) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000806842); };
            c.w64(15, t206);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080684a); };
            c.w64(3, t207);
            let Some(t208) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000806851); };
            c.cmp(t208, c.r64(0), 64);
            if c.cc_be() { blk = 421; continue; }
            blk = 214; continue;
        }
        214 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700080685a); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 215; continue;
        }
        215 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000806862); };
            c.w64(15, t209);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000806869); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080686e); }
            blk = 217; continue;
        }
        216 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000806877); };
            c.w64(12, t210);
            blk = 217; continue;
        }
        217 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080687b); };
            c.w64(3, t211);
            let Some(t212) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080687f); };
            c.w32(15, t212);
            let r213 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r213);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000806886); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t214) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806890); };
            c.w32(0, t214);
            let r215 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r215);
            c.w64(2, c.r64(0));
            let r216 = c.r32(2) & 0x3ffu64;
            c.w32(2, r216);
            c.logic(r216, 32);
            let r217 = c.r32(0) & 0x3ffffc00u64;
            c.w32(0, r217);
            c.logic(r217, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 220; continue;
        }
        219 => {
            let r218 = c.shl(c.r32(15), 0x9u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r218);
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008068b6); };
            c.w64(0, t219);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 220; continue;
        }
        220 => {
            let r220 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r220);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 231; continue; }
            blk = 221; continue;
        }
        221 => {
            c.w64(11, c.r64(2).wrapping_add(0x3));
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008068cf); };
            c.w64(1, t221);
            let Some(t222) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008068d6); };
            c.w64(6, t222);
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(7).wrapping_add(c.r64(6)));
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008068e5); };
            c.w64(12, t223);
            let Some(t224) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008068ec); };
            c.cmp(t224, c.r64(8), 64);
            if c.cc_be() { blk = 423; continue; }
            blk = 222; continue;
        }
        222 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008068f6); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 223; continue;
        }
        223 => {
            c.w32(12, c.r32(11));
            c.w32(1, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x7000806904); };
            c.w64(6, t225);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700080690b); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700080690f); }
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_e() { blk = 226; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000806928); };
            c.w64(1, t226);
            blk = 225; continue;
        }
        225 => {
            let r227 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r227);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000806933); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ne() { blk = 225; continue; }
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            let r228 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r228);
            let Some(t229) = c.ld32(c.r64(2).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x7000806946); };
            c.w32(11, t229);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x700080694a); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 227; continue; }
            blk = 228; continue;
        }
        228 => {
            let r230 = c.r32(12) | c.r32(15);
            c.w32(12, r230);
            c.logic(r230, 32);
            c.w32(11, c.r64(12).wrapping_add(c.r64(12)));
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700080695b); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700080695f); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r231 = c.r64(11) & c.r64(3);
            c.w64(11, r231);
            c.logic(r231, 64);
            let Some(t232) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000806973); };
            c.logic(t232 & 0x4u64, 8);
            if c.cc_ne() { blk = 503; continue; }
            blk = 230; continue;
        }
        230 => {
            c.w64(0, c.r64(8));
            blk = 231; continue;
        }
        231 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000806989); };
            c.w64(9, t233);
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x700080698d); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r234 = c.r64(11) & c.r64(0);
            c.w64(11, r234);
            c.logic(r234, 64);
            let Some(t235) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008069a1); };
            c.logic(t235 & 0x4u64, 8);
            if c.cc_ne() { blk = 500; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008069ac); };
            c.w64(8, t236);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008069b0); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r237 = c.r64(9) & c.r64(3);
            c.w64(9, r237);
            c.logic(r237, 64);
            let Some(t238) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008069be); };
            c.logic(t238 & 0x4u64, 8);
            if c.cc_ne() { blk = 497; continue; }
            blk = 234; continue;
        }
        234 => {
            blk = 248; continue;
        }
        235 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 240; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008069dc); };
            c.w64(12, t239);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008069e0); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r240 = c.r64(11) & c.r64(12);
            c.w64(11, r240);
            c.logic(r240, 64);
            let Some(t241) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008069ef); };
            c.logic(t241 & 0x4u64, 8);
            if c.cc_ne() { blk = 494; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008069fa); };
            c.w64(15, t242);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x70008069fe); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r243 = c.r64(8) & c.r64(12);
            c.w64(8, r243);
            c.logic(r243, 64);
            let Some(t244) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000806a12); };
            c.logic(t244 & 0x4u64, 8);
            if c.cc_ne() { blk = 491; continue; }
            blk = 239; continue;
        }
        239 => {
            blk = 248; continue;
        }
        240 => {
            let Some(t245) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000806a22); };
            c.logic(t245 & 0x1u64, 8);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000806a28); };
            c.w64(12, t246);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000806a2c); }
            blk = 243; continue;
        }
        242 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000806a35); };
            c.w64(12, t247);
            let r248 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r248);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 243; continue;
        }
        243 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000806a45); };
            c.w64(11, t249);
            let Some(t250) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000806a4c); };
            c.w64(12, t250);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000806a54); };
            c.w64(0, t251);
            let Some(t252) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000806a5b); };
            c.cmp(t252, c.r64(15), 64);
            if c.cc_be() { blk = 425; continue; }
            blk = 244; continue;
        }
        244 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000806a64); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 245; continue;
        }
        245 => {
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000806a6c); };
            c.w64(12, t253);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000806a73); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000806a77); }
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000806a7d); };
            c.w64(12, t254);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000806a81); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r255 = c.r64(15) & c.r64(12);
            c.w64(15, r255);
            c.logic(r255, 64);
            let Some(t256) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000806a90); };
            c.logic(t256 & 0x4u64, 8);
            if c.cc_ne() { blk = 488; continue; }
            blk = 246; continue;
        }
        246 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x7000806a9b); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r257 = c.r64(8) & c.r64(12);
            c.w64(8, r257);
            c.logic(r257, 64);
            let Some(t258) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000806aaf); };
            c.logic(t258 & 0x4u64, 8);
            if c.cc_ne() { blk = 485; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000806aba); };
            c.w64(0, t259);
            blk = 141; continue;
        }
        249 => {
            let Some(t260) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806ac3); };
            c.w32(12, t260);
            let r261 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r261);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000806ad0); };
            c.w64(12, t262);
            blk = 251; continue;
        }
        251 => {
            let Some(t263) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806ad7); };
            c.w32(15, t263);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t264) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806adf); };
            c.w32(3, t264);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r265 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r265);
            let r266 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r266);
            c.w64(7, c.r64(3));
            let r267 = c.r64(7) & c.r64(15);
            c.w64(7, r267);
            c.logic(r267, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000806af2); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(3)) { return RegfileExit::fault(0x7000806af6); }
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 253; continue;
        }
        252 => {
            let r268 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r268);
            c.w64(8, c.r64(6).wrapping_add(c.r64(7)));
            c.w64(3, c.r64(15));
            let r269 = c.r64(3) & c.r64(8);
            c.w64(3, r269);
            c.logic(r269, 64);
            c.w64(7, c.r64(3));
            blk = 253; continue;
        }
        253 => {
            c.w64(2, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(2).wrapping_mul(4));
            let Some(t270) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806b1c); };
            c.w32(8, t270);
            let Some(t271) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000806b21); };
            c.cmp(t271, c.r32(8), 32);
            if c.cc_e() { blk = 270; continue; }
            blk = 254; continue;
        }
        254 => {
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t272) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806b33); };
            c.w32(8, t272);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 361; continue; }
            blk = 256; continue;
        }
        256 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 257; continue;
        }
        257 => {
            c.w64(8, c.r64(2).wrapping_add(0x1));
            let Some(t273) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806b4c); };
            c.w32(9, t273);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 441; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t274) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000806b60); };
            c.w32(8, t274);
            let r275 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000806b68); };
            c.w64(2, t276);
            blk = 353; continue;
        }
        259 => {
            let Some(t277) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000806b71); };
            c.cmp(t277, c.r32(1), 32);
            if c.cc_e() { blk = 449; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t278) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000806b7e); };
            c.cmp(t278, c.r32(1), 32);
            if c.cc_e() { blk = 448; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t279) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000806b8b); };
            c.cmp(t279, c.r32(1), 32);
            if c.cc_e() { blk = 447; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t280) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000806b98); };
            c.cmp(t280, c.r32(1), 32);
            if c.cc_e() { blk = 446; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t281) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000806ba5); };
            c.cmp(t281, c.r32(1), 32);
            if c.cc_e() { blk = 445; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t282) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000806bb2); };
            c.cmp(t282, c.r32(1), 32);
            if c.cc_e() { blk = 444; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t283) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000806bbf); };
            c.cmp(t283, c.r32(1), 32);
            if c.cc_e() { blk = 443; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t284) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806bcc); };
            c.w32(8, t284);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 442; continue; }
            blk = 267; continue;
        }
        267 => {
            c.w64(8, c.r64(2).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000806bec); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r285 = c.r64(9) & c.r64(12);
            c.w64(9, r285);
            c.logic(r285, 64);
            let Some(t286) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000806bfe); };
            c.logic(t286 & 0x4u64, 8);
            if c.cc_ne() { blk = 482; continue; }
            blk = 269; continue;
        }
        269 => {
            blk = 141; continue;
        }
        270 => {
            c.w32(8, c.r16(9));
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_e() { blk = 475; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000806c1f); };
            c.cmp(t287, c.r32(1), 32);
            if c.cc_e() { blk = 474; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t288) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000806c2c); };
            c.cmp(t288, c.r32(1), 32);
            if c.cc_e() { blk = 473; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t289) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000806c39); };
            c.cmp(t289, c.r32(1), 32);
            if c.cc_e() { blk = 472; continue; }
            blk = 274; continue;
        }
        274 => {
            let Some(t290) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000806c46); };
            c.cmp(t290, c.r32(1), 32);
            if c.cc_e() { blk = 471; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t291) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000806c53); };
            c.cmp(t291, c.r32(1), 32);
            if c.cc_e() { blk = 470; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t292) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000806c60); };
            c.cmp(t292, c.r32(1), 32);
            if c.cc_e() { blk = 469; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t293) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000806c6d); };
            c.cmp(t293, c.r32(1), 32);
            if c.cc_e() { blk = 468; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t294) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806c7a); };
            c.w32(8, t294);
            let Some(t295) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000806c7e); };
            c.cmp(t295, c.r32(8), 32);
            if c.cc_ne() { blk = 282; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t296) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806c87); };
            c.w32(8, t296);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 282; continue; }
            blk = 280; continue;
        }
        280 => {
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 450; continue; }
            blk = 281; continue;
        }
        281 => {
            blk = 283; continue;
        }
        282 => {
            c.logic(c.r32(11) & 0x8000000u64, 32);
            if c.cc_e() { blk = 451; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000806caa); };
            c.w64(8, t297);
            let Some(t298) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806cae); };
            c.w32(15, t298);
            let r299 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r299);
            let Some(t300) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806cb5); };
            c.w32(3, t300);
            let r301 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000806cbc); };
            c.w64(2, t302);
            blk = 285; continue;
        }
        284 => {
            let Some(t303) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806cc2); };
            c.w32(3, t303);
            let r304 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r304);
            blk = 285; continue;
        }
        285 => {
            let Some(t305) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000806cc9); };
            c.cmp(t305, c.r32(15), 32);
            if c.cc_e() { blk = 377; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t306) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806cd6); };
            c.w32(6, t306);
            c.cmp(c.r16(6), 0x410u64, 16);
            if c.cc_be() { blk = 315; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t307) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000806ce5); };
            c.w32(7, t307);
            c.logic(c.r32(7) & 0x200000u64, 32);
            if c.cc_ne() { blk = 308; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t308) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000806cf4); };
            c.w32(12, t308);
            let r309 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r309);
            c.logic(c.r32(7) & 0xffc00u64, 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 289; continue;
        }
        289 => {
            let r310 = c.shr(c.r32(7), 0xau64, 32).unwrap_or(c.r32(7));
            c.w32(7, r310);
            let r311 = c.r32(7) & 0x3ffu64;
            c.w32(7, r311);
            c.logic(r311, 32);
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_be() { blk = 304; continue; }
            blk = 290; continue;
        }
        290 => {
            let Some(t312) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000806d19); };
            c.w32(11, (t312 as u16 as i16 as i64 as u64));
            let r313 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r313);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x7000806d23); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 292; continue;
        }
        291 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000806d2c); }
            blk = 292; continue;
        }
        292 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000806d30); };
            c.w64(8, t314);
            let r315 = c.sub(c.r32(8), c.r32(9), 32);
            c.w32(8, r315);
            let r316 = c.shr(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r316);
            let r317 = c.add(c.r32(8), c.r32(9), 32);
            c.w32(8, r317);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t318) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000806d44); };
            c.w32(0, t318);
            let r319 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r319);
            let r320 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r320);
            let r321 = c.r32(0) & 0x3ffu64;
            c.w32(0, r321);
            c.logic(r321, 32);
            c.w32(0, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t322) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806d59); };
            c.w32(0, t322);
            let r323 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r323);
            let Some(t324) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806d61); };
            c.w32(0, t324);
            let r325 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r325);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_be() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            c.w32(9, c.r64(8).wrapping_add(0x1));
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000806d6f); };
            c.w64(8, t326);
            blk = 294; continue;
        }
        294 => {
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_ne() { blk = 291; continue; }
            blk = 295; continue;
        }
        295 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x7000806d78); }
            blk = 297; continue;
        }
        296 => {
            let r327 = c.add(c.r32(9), 0x1u64, 32);
            c.w32(9, r327);
            blk = 297; continue;
        }
        297 => {
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_l() { blk = 303; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w32(8, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t328) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000806d8e); };
            c.w32(8, t328);
            let r329 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r329);
            let r330 = c.shr(c.r32(8), 0x9u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r330);
            let r331 = c.r32(8) & 0x3ffu64;
            c.w32(8, r331);
            c.logic(r331, 32);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t332) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806da8); };
            c.w32(3, t332);
            let r333 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r333);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000806db0); }
            let Some(t334) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806db4); };
            c.w32(0, t334);
            let r335 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r335);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 302; continue; }
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_ne() { blk = 296; continue; }
            blk = 300; continue;
        }
        300 => {
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_a() { blk = 329; continue; }
            blk = 301; continue;
        }
        301 => {
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000806dcb); };
            c.w64(3, t336);
            blk = 326; continue;
        }
        302 => {
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000806dd4); };
            c.w64(3, t337);
            blk = 326; continue;
        }
        303 => {
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000806ddd); };
            c.w64(3, t338);
            blk = 326; continue;
        }
        304 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            let r339 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(0, r339);
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 326; continue; }
            blk = 305; continue;
        }
        305 => {
            let r340 = c.sub(c.r64(0), 0x3u64, 64);
            c.w64(0, r340);
            let Some(t341) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806dfb); };
            c.cmp(t341, c.r32(1), 32);
            if c.cc_e() { blk = 330; continue; }
            blk = 306; continue;
        }
        306 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ne() { blk = 305; continue; }
            blk = 307; continue;
        }
        307 => {
            blk = 326; continue;
        }
        308 => {
            let Some(t342) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806e11); };
            c.w32(8, t342);
            let r343 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r343);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 310; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t344) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000806e1e); };
            c.w64(8, t344);
            blk = 310; continue;
        }
        310 => {
            let Some(t345) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806e25); };
            c.w32(9, t345);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r346 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r346);
            c.w64(11, c.r64(2));
            let r347 = c.r64(11) & c.r64(9);
            c.w64(11, r347);
            c.logic(r347, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 312; continue;
        }
        311 => {
            let r348 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r348);
            let r349 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r349);
            c.w64(0, c.r64(9));
            let r350 = c.r64(0) & c.r64(11);
            c.w64(0, r350);
            c.logic(r350, 64);
            c.w64(11, c.r64(0));
            blk = 312; continue;
        }
        312 => {
            c.w64(0, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(7, c.r64(0).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000806e5b); }
            let Some(t351) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806e5f); };
            c.w32(0, t351);
            let Some(t352) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000806e64); };
            c.cmp(t352, c.r32(0), 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 313; continue;
        }
        313 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 314; continue;
        }
        314 => {
            blk = 311; continue;
        }
        315 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000806e7b); }
            c.cmp(c.r16(6), 0xbdu64, 16);
            if c.cc_ne() { blk = 455; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t353) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000806e8a); };
            c.logic(t353 & 0x24u64, 8);
            if c.cc_ne() { blk = 460; continue; }
            blk = 317; continue;
        }
        317 => {
            let Some(t354) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806e94); };
            c.w32(8, t354);
            let r355 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r355);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000806ea1); };
            c.w64(8, t356);
            blk = 319; continue;
        }
        319 => {
            let Some(t357) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806ea8); };
            c.w32(9, t357);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r358 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r358);
            c.w64(11, c.r64(2));
            let r359 = c.r64(11) & c.r64(9);
            c.w64(11, r359);
            c.logic(r359, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 321; continue;
        }
        320 => {
            let r360 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r360);
            let r361 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r361);
            c.w64(15, c.r64(9));
            let r362 = c.r64(15) & c.r64(11);
            c.w64(15, r362);
            c.logic(r362, 64);
            c.w64(11, c.r64(15));
            blk = 321; continue;
        }
        321 => {
            c.w64(0, c.r64(11).wrapping_add(0x5));
            c.w64(7, c.r64(0).wrapping_mul(4));
            let Some(t363) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806edd); };
            c.w32(15, t363);
            let r364 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r364);
            let Some(t365) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000806ee5); };
            c.cmp(t365, c.r32(15), 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 322; continue;
        }
        322 => {
            let Some(t366) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000806eee); };
            c.cmp(t366, c.r32(15), 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 323; continue;
        }
        323 => {
            let Some(t367) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806ef7); };
            c.cmp(t367, c.r32(1), 32);
            if c.cc_ne() { blk = 320; continue; }
            blk = 324; continue;
        }
        324 => {
            let Some(t368) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806efd); };
            c.w32(9, t368);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 456; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t369) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000806f14); };
            c.w32(8, t369);
            let r370 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r370);
            let Some(t371) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000806f1c); };
            c.w32(9, t371);
            let r372 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r372);
            let Some(t373) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000806f23); };
            c.cmp(t373, c.r32(9), 32);
            if c.cc_ne() { blk = 350; continue; }
            blk = 326; continue;
        }
        326 => {
            c.cmp(c.r16(6), 0x82eu64, 16);
            if c.cc_e() { blk = 459; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t374) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000806f3b); };
            c.w32(15, t374);
            let r375 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r375);
            let Some(t376) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000806f42); };
            c.cmp(t376, c.r32(15), 32);
            if c.cc_e() { blk = 377; continue; }
            blk = 328; continue;
        }
        328 => {
            blk = 284; continue;
        }
        329 => {
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000806f54); };
            c.w64(0, t377);
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000806f58); };
            c.w64(3, t378);
            blk = 330; continue;
        }
        330 => {
            c.w64(8, c.r64(0).wrapping_mul(4));
            let Some(t379) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000806f64); };
            c.w32(9, t379);
            c.w32(11, c.r32(9));
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 361; continue; }
            blk = 331; continue;
        }
        331 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 332; continue;
        }
        332 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 334; continue; }
            blk = 333; continue;
        }
        333 => {
            let Some(t380) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000806f86); };
            c.w32(8, t380);
            let r381 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r381);
            blk = 345; continue;
        }
        334 => {
            let r382 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r382);
            if let Some(sr) = c.shr(c.r64(9), 0x13u64, 64) { c.w64(9, sr); }
            let Some(t383) = c.ld8(c.r64(3).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000806f9a); };
            c.w32(8, t383);
            let r384 = c.r64(9) & 0x3ffu64;
            c.w64(9, r384);
            c.logic(r384, 64);
            let r385 = c.r32(11) & 0x380u64;
            c.w32(11, r385);
            c.logic(r385, 32);
            let r386 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r386);
            c.cmp(c.r32(11), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t387) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806fbf); };
            c.w32(11, t387);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 339; continue; }
            blk = 335; continue;
        }
        335 => {
            let Some(t388) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000806fc9); };
            c.w32(12, t388);
            let r389 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r389);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 337; continue; }
            blk = 336; continue;
        }
        336 => {
            let Some(t390) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000806fd6); };
            c.w64(12, t390);
            blk = 337; continue;
        }
        337 => {
            let r391 = c.sub(c.r64(8), c.r64(11), 64);
            c.w64(8, r391);
            let Some(t392) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000806fe0); };
            c.w32(8, t392);
            let r393 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r393);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 345; continue; }
            blk = 338; continue;
        }
        338 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000806fee); }
            blk = 341; continue;
        }
        339 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 344; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t394) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000806ffc); };
            c.w32(8, t394);
            let r395 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r395);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000807004); }
            blk = 341; continue;
        }
        341 => {
            let Some(t396) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080700a); };
            c.w64(8, t396);
            let Some(t397) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000807011); };
            c.w64(9, t397);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t398) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000807018); };
            c.w64(12, t398);
            let Some(t399) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700080701f); };
            c.cmp(t399, c.r64(11), 64);
            if c.cc_be() { blk = 452; continue; }
            blk = 342; continue;
        }
        342 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000807029); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 343; continue;
        }
        343 => {
            let Some(t400) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000807030); };
            c.w64(9, t400);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000807037); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080703b); }
            blk = 345; continue;
        }
        344 => {
            let Some(t401) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807043); };
            c.w32(8, t401);
            let r402 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r402);
            blk = 345; continue;
        }
        345 => {
            c.w64(2, c.r64(15));
            blk = 353; continue;
        }
        346 => {
            let Some(t403) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000807050); };
            c.w32(9, t403);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 361; continue; }
            blk = 347; continue;
        }
        347 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 348; continue;
        }
        348 => {
            let Some(t404) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807069); };
            c.w32(9, t404);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000807073); };
            c.w64(11, t405);
            let r406 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r406);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 454; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t407) = c.ld32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000807084); };
            c.w32(8, t407);
            let r408 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r408);
            c.w64(2, c.r64(15));
            blk = 353; continue;
        }
        350 => {
            let Some(t409) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807091); };
            c.w32(8, t409);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 361; continue; }
            blk = 351; continue;
        }
        351 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008070a9); };
            c.w64(2, t410);
            c.w64(8, c.r64(9));
            blk = 353; continue;
        }
        353 => {
            let Some(t411) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008070b0); };
            c.w32(9, t411);
            let Some(t412) = c.ld32(c.r64(13).wrapping_add(0xd60)) else { return RegfileExit::fault(0x70008070b4); };
            c.cmp(t412, c.r32(9), 32);
            if c.cc_ne() { blk = 355; continue; }
            blk = 354; continue;
        }
        354 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008070bd); };
            c.w64(0, t413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008070c1); };
            c.w64(7, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008070c5); };
            c.w64(2, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008070c9); };
            c.w64(6, t416);
            blk = 405; continue;
        }
        355 => {
            let Some(t417) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008070d2); };
            c.w32(7, t417);
            let r418 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r418);
            let Some(t419) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008070d9); };
            c.w32(8, t419);
            let r420 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r420);
            let Some(t421) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008070e0); };
            c.cmp(t421, c.r32(8), 32);
            if c.cc_e() { blk = 457; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t422) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008070ed); };
            c.logic(t422 & 0x2u64, 8);
            if c.cc_ne() { blk = 359; continue; }
            blk = 357; continue;
        }
        357 => {
            c.w32(8, 0xbcu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008070fa); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008070fc); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008070fd); }
            let Some(t423) = c.ld64(c.r64(13).wrapping_add(0x2258)) else { return RegfileExit::fault(0x70008070fe); };
            c.w64(3, t423);
            c.w32(0, 0x3u64);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080710a); };
            c.w64(6, t424);
            return c.call(0x700086ed74, 0x7000807113, 0x700080710e);
        }
        358 => {
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807113); };
            c.w64(0, t425);
            blk = 141; continue;
        }
        359 => {
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080711c); };
            if !c.push64(t426) { return RegfileExit::fault(0x700080711c); }
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080711f); };
            if !c.push64(t427) { return RegfileExit::fault(0x700080711f); }
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807122); };
            c.w64(6, t428);
            c.w32(0, 0x1u64);
            return c.call(0x70007f21d4, 0x7000807130, 0x700080712b);
        }
        360 => {
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807130); };
            c.w64(0, t429);
            blk = 141; continue;
        }
        361 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000807139); };
            c.w64(8, t430);
            let Some(t431) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080713d); };
            c.cmp(t431, c.r32(8), 32);
            if c.cc_e() { blk = 458; continue; }
            blk = 362; continue;
        }
        362 => {
            let Some(t432) = c.ld16(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080714a); };
            c.w32(9, t432);
            c.cmp(c.r32(9), 0x43u64, 32);
            if c.cc_e() { blk = 374; continue; }
            blk = 363; continue;
        }
        363 => {
            let Some(t433) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000807159); };
            c.w32(8, t433);
            let r434 = c.r32(8) & 0x12u64;
            c.w32(8, r434);
            c.logic(r434, 32);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 373; continue; }
            blk = 364; continue;
        }
        364 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 366; continue; }
            blk = 365; continue;
        }
        365 => {
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0xb30)) else { return RegfileExit::fault(0x700080716e); };
            c.w64(8, t435);
            blk = 375; continue;
        }
        366 => {
            let Some(t436) = c.ld16(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000807177); };
            c.w32(8, t436);
            c.cmp(c.r32(8), 0xbau64, 32);
            if c.cc_ge() { blk = 372; continue; }
            blk = 367; continue;
        }
        367 => {
            c.cmp(c.r32(8), 0x40u64, 32);
            if c.cc_l() { blk = 371; continue; }
            blk = 368; continue;
        }
        368 => {
            c.cmp(c.r32(8), 0x41u64, 32);
            if c.cc_e() { blk = 370; continue; }
            blk = 369; continue;
        }
        369 => {
            let Some(t437) = c.ld64(c.r64(13).wrapping_add(0xaa8)) else { return RegfileExit::fault(0x7000807191); };
            c.w64(8, t437);
            blk = 375; continue;
        }
        370 => {
            let Some(t438) = c.ld64(c.r64(13).wrapping_add(0x550)) else { return RegfileExit::fault(0x700080719a); };
            c.w64(8, t438);
            blk = 375; continue;
        }
        371 => {
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x70008071a3); };
            c.w64(8, t439);
            blk = 375; continue;
        }
        372 => {
            let Some(t440) = c.ld64(c.r64(13).wrapping_add(0x918)) else { return RegfileExit::fault(0x70008071ac); };
            c.w64(8, t440);
            blk = 375; continue;
        }
        373 => {
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x730)) else { return RegfileExit::fault(0x70008071b5); };
            c.w64(8, t441);
            blk = 375; continue;
        }
        374 => {
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008071be); };
            c.w64(9, t442);
            let Some(t443) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008071c2); };
            c.w32(8, t443);
            let r444 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r444);
            blk = 375; continue;
        }
        375 => {
            c.w32(9, 0x15au64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008071cf); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008071d1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008071d2); }
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008071d4); };
            if !c.push64(t445) { return RegfileExit::fault(0x70008071d4); }
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0x2258)) else { return RegfileExit::fault(0x70008071d7); };
            c.w64(3, t446);
            c.w32(0, 0x4u64);
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008071e3); };
            c.w64(6, t447);
            return c.call(0x700086ed74, 0x70008071ec, 0x70008071e7);
        }
        376 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008071ec); };
            c.w64(0, t448);
            blk = 141; continue;
        }
        377 => {
            let Some(t449) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008071f5); };
            c.logic(t449 & 0x100000u64, 32);
            if c.cc_e() { blk = 381; continue; }
            blk = 378; continue;
        }
        378 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008071fe); };
            c.w64(8, t450);
            let Some(t451) = c.ld32(c.r64(8).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000807202); };
            c.w32(9, t451);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 381; continue; }
            blk = 379; continue;
        }
        379 => {
            c.w64(10, c.r64(4));
            let r452 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r452);
            let r453 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r453);
            c.logic(r453, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000807217); }
            c.w64(1, c.r64(8));
            let Some(t454) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x700080721f); };
            c.w64(0, t454);
            c.w64(10, 0x700080722du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080722d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000807231); }
            return c.call(c.r64(0), 0x7000807237, 0x7000807235);
        }
        380 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000807237); }
            let Some(t455) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080723f); };
            c.w64(4, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000807244); };
            c.w64(1, t456);
            blk = 381; continue;
        }
        381 => {
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000807248); };
            c.w64(12, t457);
            let Some(t458) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080724c); };
            c.w32(8, t458);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t459) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000807254); };
            c.w32(9, t459);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 461; continue; }
            blk = 382; continue;
        }
        382 => {
            let Some(t460) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807266); };
            c.w32(11, t460);
            let r461 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r461);
            c.w64(15, c.r64(11));
            let r462 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r462);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r463 = c.r64(15) & 0xfffffffffffffffeu64;
            c.w64(15, r463);
            c.logic(r463, 64);
            let r464 = c.add(c.r32(15), c.r32(11), 32);
            c.w32(15, r464);
            c.w32(0, c.r32(9));
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_b() { blk = 404; continue; }
            blk = 383; continue;
        }
        383 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 462; continue; }
            blk = 384; continue;
        }
        384 => {
            let r465 = c.sub(c.r32(0), c.r32(11), 32);
            c.w32(0, r465);
            let r466 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r466);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t467) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080729d); };
            c.w32(15, t467);
            let r468 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r468);
            c.logic(r468, 64);
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_b() { blk = 404; continue; }
            blk = 385; continue;
        }
        385 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 463; continue; }
            blk = 386; continue;
        }
        386 => {
            let Some(t469) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008072b9); };
            c.w32(8, t469);
            let r470 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r470);
            c.w32(9, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(9), 0xfffffeu64, 32);
            if c.cc_a() { blk = 404; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008072d5); }
            let Some(t471) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008072da); };
            c.w32(9, t471);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 464; continue; }
            blk = 388; continue;
        }
        388 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008072f2); }
            let Some(t472) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008072f7); };
            c.w32(9, t472);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r473 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r473);
            c.w64(11, c.r64(1));
            let Some(t474) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807309); };
            c.w32(15, t474);
            let r475 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r475);
            let r476 = c.r64(15) & c.r64(9);
            c.w64(15, r476);
            c.logic(r476, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 390; continue;
        }
        389 => {
            let r477 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r477);
            let r478 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r478);
            c.w64(3, c.r64(9));
            let r479 = c.r64(3) & c.r64(15);
            c.w64(3, r479);
            c.logic(r479, 64);
            c.w64(15, c.r64(3));
            blk = 390; continue;
        }
        390 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            c.w64(1, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t480) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080733d); };
            c.w32(6, t480);
            let Some(t481) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000807342); };
            c.cmp(t481, c.r32(6), 32);
            if c.cc_e() { blk = 392; continue; }
            blk = 391; continue;
        }
        391 => {
            let Some(t482) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080734b); };
            c.cmp(t482, c.r32(6), 32);
            if c.cc_ne() { blk = 389; continue; }
            blk = 392; continue;
        }
        392 => {
            let Some(t483) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807354); };
            c.w32(9, t483);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_ae() { blk = 465; continue; }
            blk = 393; continue;
        }
        393 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x7000807368); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 395; continue; }
            blk = 394; continue;
        }
        394 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r484 = c.r64(9) & c.r64(12);
            c.w64(9, r484);
            c.logic(r484, 64);
            let Some(t485) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080737c); };
            c.logic(t485 & 0x4u64, 8);
            if c.cc_ne() { blk = 479; continue; }
            blk = 395; continue;
        }
        395 => {
            let Some(t486) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807387); };
            c.w32(9, t486);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ae() { blk = 466; continue; }
            blk = 396; continue;
        }
        396 => {
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            c.w32(9, c.r32(8));
            c.w64(15, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            let Some(t487) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008073ae); };
            c.w64(0, t487);
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008073b2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 398; continue; }
            blk = 397; continue;
        }
        397 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r488 = c.r64(1) & c.r64(12);
            c.w64(1, r488);
            c.logic(r488, 64);
            let Some(t489) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008073c4); };
            c.logic(t489 & 0x4u64, 8);
            if c.cc_ne() { blk = 476; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t490) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008073ce); };
            c.w32(15, t490);
            let Some(t491) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008073d2); };
            c.cmp(t491, c.r32(15), 32);
            if c.cc_ne() { blk = 401; continue; }
            blk = 399; continue;
        }
        399 => {
            let Some(t492) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008073db); };
            c.logic(t492 & 0x1u64, 8);
            if c.cc_e() { blk = 401; continue; }
            blk = 400; continue;
        }
        400 => {
            let r493 = c.r32(9) | 0x20u64;
            c.w32(9, r493);
            c.logic(r493, 32);
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 402; continue;
        }
        401 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 402; continue;
        }
        402 => {
            let Some(t494) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008073ee); };
            c.w32(9, t494);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(3).wrapping_add(0x2));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 467; continue; }
            blk = 403; continue;
        }
        403 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(2)).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x7000807406); }
            blk = 141; continue;
        }
        404 => {
            let Some(t495) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000807410); };
            c.w64(5, t495);
            let Some(t496) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x7000807414); };
            c.w64(3, t496);
            c.w32(0, 0x3u64);
            let Some(t497) = c.ld64(c.r64(4).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000807420); };
            c.w64(10, t497);
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(10)) { return RegfileExit::fault(0x7000807425); }
            let Some(t498) = c.ld64(c.r64(4).wrapping_add(0x78)) else { return RegfileExit::fault(0x700080742a); };
            c.w64(10, t498);
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(10)) { return RegfileExit::fault(0x700080742f); }
            let Some(t499) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x7000807434); };
            c.w64(10, t499);
            if !c.st64(c.r64(4).wrapping_add(0x78), c.r64(10)) { return RegfileExit::fault(0x7000807439); }
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(1)) { return RegfileExit::fault(0x700080743e); }
            let Some(t500) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x7000807443); };
            c.w64(6, t500);
            let r501 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r501);
            return RegfileExit::jump(0x700086ed74);
        }
        405 => {
            let Some(t502) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000807451); };
            c.w64(5, t502);
            let r503 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r503);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000807459); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700080745a); }
            let Some(t504) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080745b); };
            if !c.push64(t504) { return RegfileExit::fault(0x700080745b); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700080745f); }
            c.w32(0, 0x5u64);
            let Some(t505) = c.ld64(c.r64(13).wrapping_add(0x2098)) else { return RegfileExit::fault(0x7000807469); };
            c.w64(3, t505);
            let Some(t506) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000807470); };
            c.w64(10, t506);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(10)) { return RegfileExit::fault(0x7000807477); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(7)) { return RegfileExit::fault(0x700080747c); }
            return RegfileExit::jump(0x700086ed74);
        }
        406 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700080748b); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000807498, 0x7000807493);
        }
        407 => {
            c.w64(8, c.r64(0));
            let Some(t507) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080749b); };
            c.w64(0, t507);
            let Some(t508) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080749f); };
            c.w64(7, t508);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008074a3); };
            c.w64(2, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008074a7); };
            c.w64(1, t510);
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008074ab); };
            c.w64(6, t511);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008074af); }
            blk = 34; continue;
        }
        408 => {
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008074b9); };
            c.w64(0, t512);
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008074bd); };
            c.w64(7, t513);
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008074c1); };
            c.w64(2, t514);
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008074c5); };
            c.w64(6, t515);
            blk = 405; continue;
        }
        409 => {
            let Some(t516) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008074cb); };
            c.w64(0, t516);
            let Some(t517) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008074cf); };
            c.w64(7, t517);
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008074d3); };
            c.w64(2, t518);
            let Some(t519) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008074d7); };
            c.w64(6, t519);
            blk = 405; continue;
        }
        410 => {
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008074e0); };
            c.w64(0, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008074e4); };
            c.w64(7, t521);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008074e8); };
            c.w64(2, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008074ec); };
            c.w64(6, t523);
            blk = 405; continue;
        }
        411 => {
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008074f5); };
            c.w64(0, t524);
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008074f9); };
            c.w64(7, t525);
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008074fd); };
            c.w64(2, t526);
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807501); };
            c.w64(6, t527);
            blk = 405; continue;
        }
        412 => {
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080750a); };
            c.w64(0, t528);
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080750e); };
            c.w64(7, t529);
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807512); };
            c.w64(2, t530);
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807516); };
            c.w64(6, t531);
            blk = 405; continue;
        }
        413 => {
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080751f); };
            c.w64(0, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807523); };
            c.w64(7, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807527); };
            c.w64(2, t534);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080752b); };
            c.w64(6, t535);
            blk = 405; continue;
        }
        414 => {
            c.w64(0, c.r64(3));
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807537); };
            c.w64(7, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080753b); };
            c.w64(2, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080753f); };
            c.w64(6, t538);
            blk = 405; continue;
        }
        415 => {
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807548); };
            c.w64(0, t539);
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080754c); };
            c.w64(7, t540);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807550); };
            c.w64(2, t541);
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807554); };
            c.w64(6, t542);
            blk = 405; continue;
        }
        416 => {
            c.w64(0, c.r64(3));
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807560); };
            c.w64(7, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807564); };
            c.w64(2, t544);
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807568); };
            c.w64(6, t545);
            blk = 405; continue;
        }
        417 => {
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807571); };
            c.w64(0, t546);
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807575); };
            c.w64(7, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807579); };
            c.w64(2, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080757d); };
            c.w64(6, t549);
            blk = 405; continue;
        }
        418 => {
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807586); };
            c.w64(0, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080758a); };
            c.w64(7, t551);
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080758e); };
            c.w64(2, t552);
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807592); };
            c.w64(6, t553);
            blk = 405; continue;
        }
        419 => {
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080759b); };
            c.w64(0, t554);
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080759f); };
            c.w64(7, t555);
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008075a3); };
            c.w64(2, t556);
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008075a7); };
            c.w64(6, t557);
            blk = 405; continue;
        }
        420 => {
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008075b0); };
            c.w64(0, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008075b4); };
            c.w64(7, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008075b8); };
            c.w64(2, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008075bc); };
            c.w64(6, t561);
            blk = 405; continue;
        }
        421 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008075ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008075ce); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008075d2); }
            return c.call(0x70007febf4, 0x70008075dc, 0x70008075d7);
        }
        422 => {
            c.w64(12, c.r64(0));
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008075df); };
            c.w64(8, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008075e3); };
            c.w64(9, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008075e7); };
            c.w64(11, t564);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008075eb); }
            blk = 215; continue;
        }
        423 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008075f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008075f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008075fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000807601); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000807605); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febf4, 0x7000807611, 0x700080760c);
        }
        424 => {
            c.w64(8, c.r64(0));
            let Some(t565) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807614); };
            c.w64(3, t565);
            let Some(t566) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807618); };
            c.w64(9, t566);
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080761c); };
            c.w64(0, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000807620); };
            c.w64(15, t568);
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000807624); };
            c.w64(11, t569);
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000807628); };
            c.w64(2, t570);
            blk = 223; continue;
        }
        425 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000807636); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700080763a); }
            return c.call(0x70007febf4, 0x7000807644, 0x700080763f);
        }
        426 => {
            c.w64(11, c.r64(0));
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807647); };
            c.w64(9, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080764b); };
            c.w64(8, t572);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700080764f); }
            blk = 245; continue;
        }
        427 => {
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807659); };
            c.w64(0, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080765d); };
            c.w64(7, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807661); };
            c.w64(2, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807665); };
            c.w64(6, t576);
            blk = 405; continue;
        }
        428 => {
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080766e); };
            c.w64(0, t577);
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807672); };
            c.w64(7, t578);
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807676); };
            c.w64(2, t579);
            let Some(t580) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080767a); };
            c.w64(6, t580);
            blk = 405; continue;
        }
        429 => {
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807683); };
            c.w64(0, t581);
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807687); };
            c.w64(7, t582);
            let Some(t583) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080768b); };
            c.w64(2, t583);
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080768f); };
            c.w64(6, t584);
            blk = 405; continue;
        }
        430 => {
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807698); };
            c.w64(0, t585);
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080769c); };
            c.w64(7, t586);
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008076a0); };
            c.w64(2, t587);
            let Some(t588) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008076a4); };
            c.w64(6, t588);
            blk = 405; continue;
        }
        431 => {
            let Some(t589) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008076ad); };
            c.w64(0, t589);
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008076b1); };
            c.w64(7, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008076b5); };
            c.w64(2, t591);
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008076b9); };
            c.w64(6, t592);
            blk = 405; continue;
        }
        432 => {
            let Some(t593) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008076c2); };
            c.w64(0, t593);
            let Some(t594) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008076c6); };
            c.w64(7, t594);
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008076ca); };
            c.w64(2, t595);
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008076ce); };
            c.w64(6, t596);
            blk = 405; continue;
        }
        433 => {
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008076d7); };
            c.w64(0, t597);
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008076db); };
            c.w64(7, t598);
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008076df); };
            c.w64(2, t599);
            let Some(t600) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008076e3); };
            c.w64(6, t600);
            blk = 405; continue;
        }
        434 => {
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008076ec); };
            c.w64(0, t601);
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008076f0); };
            c.w64(7, t602);
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008076f4); };
            c.w64(2, t603);
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008076f8); };
            c.w64(6, t604);
            blk = 405; continue;
        }
        435 => {
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807701); };
            c.w64(0, t605);
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807705); };
            c.w64(7, t606);
            let Some(t607) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807709); };
            c.w64(2, t607);
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080770d); };
            c.w64(6, t608);
            blk = 405; continue;
        }
        436 => {
            let Some(t609) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807716); };
            c.w64(0, t609);
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080771a); };
            c.w64(7, t610);
            let Some(t611) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080771e); };
            c.w64(2, t611);
            let Some(t612) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807722); };
            c.w64(6, t612);
            blk = 405; continue;
        }
        437 => {
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080772b); };
            c.w64(0, t613);
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080772f); };
            c.w64(7, t614);
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807733); };
            c.w64(2, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807737); };
            c.w64(6, t616);
            blk = 405; continue;
        }
        438 => {
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807740); };
            c.w64(0, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807744); };
            c.w64(7, t618);
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807748); };
            c.w64(2, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080774c); };
            c.w64(6, t620);
            blk = 405; continue;
        }
        439 => {
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807755); };
            c.w64(0, t621);
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807759); };
            c.w64(7, t622);
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080775d); };
            c.w64(2, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807761); };
            c.w64(6, t624);
            blk = 405; continue;
        }
        440 => {
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080776a); };
            c.w64(0, t625);
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080776e); };
            c.w64(7, t626);
            let Some(t627) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807772); };
            c.w64(2, t627);
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807776); };
            c.w64(6, t628);
            blk = 405; continue;
        }
        441 => {
            return RegfileExit::fault(0x700080777f);
        }
        442 => {
            return RegfileExit::fault(0x7000807780);
        }
        443 => {
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807781); };
            c.w64(7, t629);
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807785); };
            c.w64(2, t630);
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807789); };
            c.w64(6, t631);
            blk = 405; continue;
        }
        444 => {
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807792); };
            c.w64(7, t632);
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807796); };
            c.w64(2, t633);
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080779a); };
            c.w64(6, t634);
            blk = 405; continue;
        }
        445 => {
            let Some(t635) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008077a3); };
            c.w64(7, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008077a7); };
            c.w64(2, t636);
            let Some(t637) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008077ab); };
            c.w64(6, t637);
            blk = 405; continue;
        }
        446 => {
            let Some(t638) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008077b4); };
            c.w64(7, t638);
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008077b8); };
            c.w64(2, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008077bc); };
            c.w64(6, t640);
            blk = 405; continue;
        }
        447 => {
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008077c5); };
            c.w64(7, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008077c9); };
            c.w64(2, t642);
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008077cd); };
            c.w64(6, t643);
            blk = 405; continue;
        }
        448 => {
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008077d6); };
            c.w64(7, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008077da); };
            c.w64(2, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008077de); };
            c.w64(6, t646);
            blk = 405; continue;
        }
        449 => {
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008077e7); };
            c.w64(7, t647);
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008077eb); };
            c.w64(2, t648);
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008077ef); };
            c.w64(6, t649);
            blk = 405; continue;
        }
        450 => {
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008077f8); };
            c.w64(7, t650);
            let Some(t651) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008077fc); };
            c.w64(2, t651);
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807800); };
            c.w64(6, t652);
            blk = 405; continue;
        }
        451 => {
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807809); };
            c.w64(7, t653);
            let Some(t654) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080780d); };
            c.w64(2, t654);
            let Some(t655) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807811); };
            c.w64(6, t655);
            blk = 405; continue;
        }
        452 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700080781f); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000807824); }
            return c.call(0x70007febf4, 0x700080782d, 0x7000807828);
        }
        453 => {
            c.w64(8, c.r64(0));
            let Some(t656) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000807830); };
            c.w64(1, t656);
            let Some(t657) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000807834); };
            c.w64(15, t657);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000807838); }
            blk = 343; continue;
        }
        454 => {
            return RegfileExit::fault(0x7000807842);
        }
        455 => {
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807843); };
            c.w64(0, t658);
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807847); };
            c.w64(7, t659);
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080784b); };
            c.w64(2, t660);
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080784f); };
            c.w64(6, t661);
            blk = 405; continue;
        }
        456 => {
            return RegfileExit::fault(0x7000807858);
        }
        457 => {
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000807859); };
            c.w64(0, t662);
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080785d); };
            c.w64(7, t663);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807861); };
            c.w64(2, t664);
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807865); };
            c.w64(6, t665);
            blk = 405; continue;
        }
        458 => {
            let Some(t666) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x700080786e); };
            c.w64(8, t666);
            blk = 375; continue;
        }
        459 => {
            let Some(t667) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080787a); };
            c.w64(0, t667);
            let Some(t668) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080787e); };
            c.w64(7, t668);
            let Some(t669) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807882); };
            c.w64(2, t669);
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807886); };
            c.w64(6, t670);
            blk = 405; continue;
        }
        460 => {
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080788f); };
            c.w64(0, t671);
            let Some(t672) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807893); };
            c.w64(7, t672);
            let Some(t673) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807897); };
            c.w64(2, t673);
            let Some(t674) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080789b); };
            c.w64(6, t674);
            blk = 405; continue;
        }
        461 => {
            return RegfileExit::fault(0x70008078a4);
        }
        462 => {
            return RegfileExit::fault(0x70008078a5);
        }
        463 => {
            return RegfileExit::fault(0x70008078a6);
        }
        464 => {
            return RegfileExit::fault(0x70008078a7);
        }
        465 => {
            return RegfileExit::fault(0x70008078a8);
        }
        466 => {
            return RegfileExit::fault(0x70008078a9);
        }
        467 => {
            return RegfileExit::fault(0x70008078aa);
        }
        468 => {
            let Some(t675) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008078ab); };
            c.w64(7, t675);
            let Some(t676) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008078af); };
            c.w64(2, t676);
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008078b3); };
            c.w64(6, t677);
            blk = 405; continue;
        }
        469 => {
            let Some(t678) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008078bc); };
            c.w64(7, t678);
            let Some(t679) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008078c0); };
            c.w64(2, t679);
            let Some(t680) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008078c4); };
            c.w64(6, t680);
            blk = 405; continue;
        }
        470 => {
            let Some(t681) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008078cd); };
            c.w64(7, t681);
            let Some(t682) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008078d1); };
            c.w64(2, t682);
            let Some(t683) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008078d5); };
            c.w64(6, t683);
            blk = 405; continue;
        }
        471 => {
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008078de); };
            c.w64(7, t684);
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008078e2); };
            c.w64(2, t685);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008078e6); };
            c.w64(6, t686);
            blk = 405; continue;
        }
        472 => {
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008078ef); };
            c.w64(7, t687);
            let Some(t688) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008078f3); };
            c.w64(2, t688);
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008078f7); };
            c.w64(6, t689);
            blk = 405; continue;
        }
        473 => {
            let Some(t690) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807900); };
            c.w64(7, t690);
            let Some(t691) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807904); };
            c.w64(2, t691);
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807908); };
            c.w64(6, t692);
            blk = 405; continue;
        }
        474 => {
            let Some(t693) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807911); };
            c.w64(7, t693);
            let Some(t694) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807915); };
            c.w64(2, t694);
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000807919); };
            c.w64(6, t695);
            blk = 405; continue;
        }
        475 => {
            let Some(t696) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000807922); };
            c.w64(7, t696);
            let Some(t697) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000807926); };
            c.w64(2, t697);
            let Some(t698) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080792a); };
            c.w64(6, t698);
            blk = 405; continue;
        }
        476 => {
            c.w32(0, c.r32(0));
            let r699 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r699);
            c.w64(1, 0xfffffffffffc0000u64);
            let r700 = c.r64(1) & c.r64(0);
            c.w64(1, r700);
            c.logic(r700, 64);
            let Some(t701) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807942); };
            c.logic(t701 & 0x2u64, 8);
            if c.cc_e() { blk = 398; continue; }
            blk = 477; continue;
        }
        477 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807950); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807951); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x700080795e, 0x7000807959);
        }
        478 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080795e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807960); }
            blk = 398; continue;
        }
        479 => {
            c.w32(11, c.r32(11));
            let r702 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r702);
            c.w64(9, 0xfffffffffffc0000u64);
            let r703 = c.r64(9) & c.r64(11);
            c.w64(9, r703);
            c.logic(r703, 64);
            let Some(t704) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807976); };
            c.logic(t704 & 0x2u64, 8);
            if c.cc_e() { blk = 395; continue; }
            blk = 480; continue;
        }
        480 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807985); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807986); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000807993, 0x700080798e);
        }
        481 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807993); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807995); }
            blk = 395; continue;
        }
        482 => {
            c.w32(0, c.r32(0));
            let r705 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r705);
            c.w64(9, 0xfffffffffffc0000u64);
            let r706 = c.r64(9) & c.r64(0);
            c.w64(9, r706);
            c.logic(r706, 64);
            let Some(t707) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008079aa); };
            c.logic(t707 & 0x2u64, 8);
            if c.cc_e() { blk = 269; continue; }
            blk = 483; continue;
        }
        483 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008079b9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008079ba); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008079c7, 0x70008079c2);
        }
        484 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008079c7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008079c9); }
            blk = 269; continue;
        }
        485 => {
            c.w32(11, c.r32(11));
            let r708 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r708);
            c.w64(8, 0xfffffffffffc0000u64);
            let r709 = c.r64(8) & c.r64(11);
            c.w64(8, r709);
            c.logic(r709, 64);
            let Some(t710) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008079df); };
            c.logic(t710 & 0x2u64, 8);
            if c.cc_e() { blk = 248; continue; }
            blk = 486; continue;
        }
        486 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008079ee); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008079ef); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008079fc, 0x70008079f7);
        }
        487 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008079fc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008079fe); }
            blk = 248; continue;
        }
        488 => {
            c.w32(8, c.r32(8));
            let r711 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r711);
            c.w64(15, 0xfffffffffffc0000u64);
            let r712 = c.r64(15) & c.r64(8);
            c.w64(15, r712);
            c.logic(r712, 64);
            let Some(t713) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807a14); };
            c.logic(t713 & 0x2u64, 8);
            if c.cc_e() { blk = 246; continue; }
            blk = 489; continue;
        }
        489 => {
            c.w64(0, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807a24); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807a25); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x7000807a32, 0x7000807a2d);
        }
        490 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807a32); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807a34); }
            blk = 246; continue;
        }
        491 => {
            c.w32(15, c.r32(15));
            let r714 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r714);
            c.w64(8, 0xfffffffffffc0000u64);
            let r715 = c.r64(8) & c.r64(15);
            c.w64(8, r715);
            c.logic(r715, 64);
            let Some(t716) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807a4a); };
            c.logic(t716 & 0x2u64, 8);
            if c.cc_e() { blk = 239; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807a59); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807a5a); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000807a67, 0x7000807a62);
        }
        493 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807a67); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807a69); }
            blk = 239; continue;
        }
        494 => {
            c.w32(8, c.r32(8));
            let r717 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r717);
            c.w64(11, 0xfffffffffffc0000u64);
            let r718 = c.r64(11) & c.r64(8);
            c.w64(11, r718);
            c.logic(r718, 64);
            let Some(t719) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807a7f); };
            c.logic(t719 & 0x2u64, 8);
            if c.cc_e() { blk = 237; continue; }
            blk = 495; continue;
        }
        495 => {
            c.w64(0, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807a8f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807a90); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x7000807a9d, 0x7000807a98);
        }
        496 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807a9d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807a9f); }
            blk = 237; continue;
        }
        497 => {
            c.w32(8, c.r32(8));
            let r720 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r720);
            c.w64(9, 0xfffffffffffc0000u64);
            let r721 = c.r64(9) & c.r64(8);
            c.w64(9, r721);
            c.logic(r721, 64);
            let Some(t722) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807ab5); };
            c.logic(t722 & 0x2u64, 8);
            if c.cc_e() { blk = 234; continue; }
            blk = 498; continue;
        }
        498 => {
            c.w64(11, c.r64(3).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807ac4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807ac5); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000807ad2, 0x7000807acd);
        }
        499 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807ad2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807ad4); }
            blk = 234; continue;
        }
        500 => {
            c.w32(9, c.r32(9));
            let r723 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r723);
            c.w64(11, 0xfffffffffffc0000u64);
            let r724 = c.r64(11) & c.r64(9);
            c.w64(11, r724);
            c.logic(r724, 64);
            let Some(t725) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807aea); };
            c.logic(t725 & 0x2u64, 8);
            if c.cc_e() { blk = 233; continue; }
            blk = 501; continue;
        }
        501 => {
            c.w64(12, c.r64(0).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807af9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807afa); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000807b07, 0x7000807b02);
        }
        502 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807b07); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807b09); }
            blk = 233; continue;
        }
        503 => {
            c.w32(8, c.r32(8));
            let r726 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r726);
            c.w64(11, 0xfffffffffffc0000u64);
            let r727 = c.r64(11) & c.r64(8);
            c.w64(11, r727);
            c.logic(r727, 64);
            let Some(t728) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807b1f); };
            c.logic(t728 & 0x2u64, 8);
            if c.cc_e() { blk = 230; continue; }
            blk = 504; continue;
        }
        504 => {
            c.w64(12, c.r64(3).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807b2e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807b2f); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000807b3c, 0x7000807b37);
        }
        505 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807b3c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807b3e); }
            blk = 230; continue;
        }
        506 => {
            c.w32(8, c.r32(8));
            let r729 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r729);
            c.w64(9, 0xfffffffffffc0000u64);
            let r730 = c.r64(9) & c.r64(8);
            c.w64(9, r730);
            c.logic(r730, 64);
            let Some(t731) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807b54); };
            c.logic(t731 & 0x2u64, 8);
            if c.cc_e() { blk = 192; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807b63); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807b64); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000807b71, 0x7000807b6c);
        }
        508 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807b71); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807b73); }
            blk = 192; continue;
        }
        509 => {
            c.w32(0, c.r32(0));
            let r732 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r732);
            c.w64(9, 0xfffffffffffc0000u64);
            let r733 = c.r64(9) & c.r64(0);
            c.w64(9, r733);
            c.logic(r733, 64);
            let Some(t734) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807b88); };
            c.logic(t734 & 0x2u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 510; continue;
        }
        510 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807b97); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807b98); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000807ba5, 0x7000807ba0);
        }
        511 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807ba5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807ba7); }
            blk = 107; continue;
        }
        512 => {
            c.w32(0, c.r32(0));
            let r735 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r735);
            c.w64(9, 0xfffffffffffc0000u64);
            let r736 = c.r64(9) & c.r64(0);
            c.w64(9, r736);
            c.logic(r736, 64);
            let Some(t737) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807bbc); };
            c.logic(t737 & 0x2u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 513; continue;
        }
        513 => {
            c.w64(11, c.r64(15).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807bcb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000807bcc); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000807bd9, 0x7000807bd4);
        }
        514 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000807bd9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807bdb); }
            blk = 68; continue;
        }
        515 => {
            return RegfileExit::jump(0x7000807be4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000807bf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807bfd); };
            c.w32(7, t0);
            let r1 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1);
            let Some(t2) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807c03); };
            c.w32(8, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x7000807c07); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807c14); };
            c.w32(8, t4);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_be() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_b() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000807c22);
        }
        5 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000807c26); };
            c.w32(8, t5);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807c31); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807c37); };
            c.w32(1, t8);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_b() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x7000807c3f);
        }
        8 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            blk = 15; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000807c4d); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807c53); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807c5a); };
            c.cmp(t13, 0x7cu64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807c62); };
            c.w32(8, t14);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_be() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000807c70);
        }
        13 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807c78); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000807c7d); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(1, c.r64(8).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(7));
            blk = 15; continue;
        }
        15 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807c91); };
            c.w32(0, t17);
            let r18 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r18);
            return c.ret(0x0, 0x7000807c98);
        }
        16 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000807c99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000807c9a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000807c9b); }
            let Some(t19) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000807c9c); };
            if !c.push64(t19) { return RegfileExit::fault(0x7000807c9c); }
            c.w32(0, 0x4u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2018)) else { return RegfileExit::fault(0x7000807ca5); };
            c.w64(3, t20);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x7000807cac); }
            return RegfileExit::jump(0x700086ed74);
        }
        17 => {
            return RegfileExit::jump(0x7000807cb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000807cd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r32(1) & 0x80000001u64, 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000807cdc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000807cdd); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000807cde); }
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000807cdf); };
            if !c.push64(t0) { return RegfileExit::fault(0x7000807cdf); }
            c.w32(0, 0x4u64);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2018)) else { return RegfileExit::fault(0x7000807ce8); };
            c.w64(3, t1);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x7000807cef); }
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000807cf9); }
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807cfa); };
            if !c.push64(t2) { return RegfileExit::fault(0x7000807cfa); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2038)) else { return RegfileExit::fault(0x7000807cfe); };
            c.w64(3, t3);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x7000807d0a); }
            return RegfileExit::jump(0x700086ed74);
        }
        3 => {
            return RegfileExit::jump(0x7000807d18);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000807d34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807d3d); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807d44); };
            c.w32(9, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x7000807d48); };
            c.cmp(t3, c.r32(9), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807d55); };
            c.w32(9, t4);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000807d63);
        }
        5 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000807d67); };
            c.w32(9, t5);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807d72); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807d79); };
            c.w32(1, t8);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_b() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x7000807d81);
        }
        8 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            blk = 15; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000807d8f); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807d96); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807d9d); };
            c.cmp(t13, 0x7cu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807da5); };
            c.w32(9, t14);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000807db3);
        }
        13 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807dbb); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000807dc0); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(1, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(8));
            blk = 15; continue;
        }
        15 => {
            let r17 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000807dd8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r18 = c.r64(7) & c.r64(2);
            c.w64(7, r18);
            c.logic(r18, 64);
            let Some(t19) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807de9); };
            c.logic(t19 & 0x4u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x0, 0x7000807def);
        }
        18 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807df0); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000807df1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000807df2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000807df3); }
            let Some(t20) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000807df4); };
            if !c.push64(t20) { return RegfileExit::fault(0x7000807df4); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x7000807df8); }
            c.w32(0, 0x5u64);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x7000807e02); };
            c.w64(3, t21);
            return RegfileExit::jump(0x700086ed74);
        }
        19 => {
            c.w32(0, c.r32(0));
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            c.w64(7, 0xfffffffffffc0000u64);
            let r23 = c.r64(7) & c.r64(0);
            c.w64(7, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807e1d); };
            c.logic(t24 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807e27); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f1594, 0x7000807e30, 0x7000807e2b);
        }
        21 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807e30); }
            blk = 17; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000807e34);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000807e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807e5d); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807e64); };
            c.w32(9, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x7000807e68); };
            c.cmp(t3, c.r32(9), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807e75); };
            c.w32(9, t4);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000807e83);
        }
        5 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000807e87); };
            c.w32(9, t5);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807e92); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807e99); };
            c.w32(1, t8);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_b() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x7000807ea1);
        }
        8 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            blk = 15; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000807eaf); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807eb6); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807ebd); };
            c.cmp(t13, 0x7cu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807ec5); };
            c.w32(9, t14);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000807ed3);
        }
        13 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807edb); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000807ee0); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(1, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(8));
            blk = 15; continue;
        }
        15 => {
            let r17 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000807ef8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r18 = c.r64(7) & c.r64(2);
            c.w64(7, r18);
            c.logic(r18, 64);
            let Some(t19) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807f09); };
            c.logic(t19 & 0x4u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x0, 0x7000807f0f);
        }
        18 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807f10); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000807f11); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000807f12); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000807f13); }
            let Some(t20) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000807f14); };
            if !c.push64(t20) { return RegfileExit::fault(0x7000807f14); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x7000807f18); }
            c.w32(0, 0x5u64);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x7000807f22); };
            c.w64(3, t21);
            return RegfileExit::jump(0x700086ed74);
        }
        19 => {
            c.w32(0, c.r32(0));
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            c.w64(7, 0xfffffffffffc0000u64);
            let r23 = c.r64(7) & c.r64(0);
            c.w64(7, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000807f3d); };
            c.logic(t24 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000807f47); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f1594, 0x7000807f50, 0x7000807f4b);
        }
        21 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000807f50); }
            blk = 17; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000807f54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000807f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807f7d); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807f84); };
            c.w32(9, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x7000807f88); };
            c.cmp(t3, c.r32(9), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807f95); };
            c.w32(9, t4);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000807fa3);
        }
        5 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000807fa7); };
            c.w32(9, t5);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807fb2); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807fb9); };
            c.w32(1, t8);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_b() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x7000807fc1);
        }
        8 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            blk = 15; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000807fcf); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000807fd6); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807fdd); };
            c.cmp(t13, 0x7cu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000807fe5); };
            c.w32(9, t14);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000807ff3);
        }
        13 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000807ffb); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000808000); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(1, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(8));
            blk = 15; continue;
        }
        15 => {
            let r17 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000808018); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r18 = c.r64(7) & c.r64(2);
            c.w64(7, r18);
            c.logic(r18, 64);
            let Some(t19) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000808029); };
            c.logic(t19 & 0x4u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x0, 0x700080802f);
        }
        18 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000808030); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000808031); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000808032); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000808033); }
            let Some(t20) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000808034); };
            if !c.push64(t20) { return RegfileExit::fault(0x7000808034); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x7000808038); }
            c.w32(0, 0x5u64);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x7000808042); };
            c.w64(3, t21);
            return RegfileExit::jump(0x700086ed74);
        }
        19 => {
            c.w32(0, c.r32(0));
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            c.w64(7, 0xfffffffffffc0000u64);
            let r23 = c.r64(7) & c.r64(0);
            c.w64(7, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080805d); };
            c.logic(t24 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000808067); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f1594, 0x7000808070, 0x700080806b);
        }
        21 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000808070); }
            blk = 17; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000808074);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000808094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080809d); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008080a4); };
            c.w32(9, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x70008080a8); };
            c.cmp(t3, c.r32(9), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008080b5); };
            c.w32(9, t4);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008080c3);
        }
        5 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008080c7); };
            c.w32(9, t5);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008080d2); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008080d9); };
            c.w32(1, t8);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_b() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008080e1);
        }
        8 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            blk = 15; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008080ef); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008080f6); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008080fd); };
            c.cmp(t13, 0x7cu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808105); };
            c.w32(9, t14);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000808113);
        }
        13 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080811b); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000808120); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(1, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(8));
            blk = 15; continue;
        }
        15 => {
            let r17 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000808138); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r18 = c.r64(7) & c.r64(2);
            c.w64(7, r18);
            c.logic(r18, 64);
            let Some(t19) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000808149); };
            c.logic(t19 & 0x4u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x0, 0x700080814f);
        }
        18 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000808150); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000808151); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000808152); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000808153); }
            let Some(t20) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000808154); };
            if !c.push64(t20) { return RegfileExit::fault(0x7000808154); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x7000808158); }
            c.w32(0, 0x5u64);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x7000808162); };
            c.w64(3, t21);
            return RegfileExit::jump(0x700086ed74);
        }
        19 => {
            c.w32(0, c.r32(0));
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            c.w64(7, 0xfffffffffffc0000u64);
            let r23 = c.r64(7) & c.r64(0);
            c.w64(7, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080817d); };
            c.logic(t24 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000808187); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f1594, 0x7000808190, 0x700080818b);
        }
        21 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000808190); }
            blk = 17; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000808194);
        }
            _ => unreachable!(),
        }
    }
}
