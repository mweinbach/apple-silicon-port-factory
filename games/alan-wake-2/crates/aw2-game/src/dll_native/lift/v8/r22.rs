#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008a72b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a72b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a72b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a72b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a72ba); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a72bf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a72c3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a72db); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008a72e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008a72e7); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a72f1); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a72f7); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7304); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a730a); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008a7311); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a731b); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7321); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008a7328); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7336); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a733d); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 62; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a734a); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a7369, 0x70008a7364);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a7369); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a736d); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a7371); };
            c.w64(9, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a7375); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a7379); };
            c.w64(7, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 28; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a738e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 56; continue;
        }
        27 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008a73f5); }
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a73f9); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(11, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a740a); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7413); };
            c.w32(11, t21);
            let r22 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r22);
            let Some(t23) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a741a); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a7421); };
            c.cmp(t24, c.r32(11), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a742e); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 66; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a744b); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            c.w32(0, 0x1u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008a7457); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008a745b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7469); };
            c.w32(12, t28);
            let r29 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r29);
            let Some(t30) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a7470); };
            c.cmp(t30, 0x41u64, 16);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a7479); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008a747b); };
            c.w64(3, t31);
            return c.call(0x700086ed74, 0x70008a7487, 0x70008a7482);
        }
        41 => {
            c.w64(9, c.r64(0));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a748a); };
            c.w64(6, t32);
            c.w32(0, 0x1u64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a7493); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a7497); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a749b); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a749f); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a74a3); };
            c.w64(7, t37);
            blk = 42; continue;
        }
        42 => {
            let Some(t38) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a74a7); };
            c.logic(t38 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t39) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a74b2); };
            c.w64(12, t39);
            c.w64(15, c.r64(2).wrapping_add(0x8));
            let Some(t40) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a74ba); };
            c.cmp(t40, c.r64(15), 64);
            if c.cc_b() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t41) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a74c4); };
            c.w32(7, t41);
            c.logic(c.r32(7) & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(9));
            blk = 47; continue;
        }
        46 => {
            let Some(t42) = c.ld64(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a74d8); };
            c.w64(9, t42);
            c.w64(15, c.r64(9));
            if let Some(sr) = c.shr(c.r64(15), 0x20u64, 64) { c.w64(15, sr); }
            c.w32(9, c.r32(9));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 47; continue;
        }
        47 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let r43 = c.neg(c.r32(9), 32);
            c.w32(9, r43);
            blk = 51; continue;
        }
        50 => {
            c.w32(7, 0xffffffffu64);
            let r44 = c.sub(c.r32(7), c.r32(9), 32);
            c.w32(7, r44);
            c.w64(9, c.r64(7));
            blk = 51; continue;
        }
        51 => {
            let r45 = c.neg(c.r32(15), 32);
            c.w32(15, r45);
            blk = 52; continue;
        }
        52 => {
            let Some(t46) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a750e); };
            c.w64(7, t46);
            c.w64(11, c.r64(9));
            let r47 = c.shr(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r47);
            c.w64(0, c.r64(9));
            let r48 = c.shr(c.r32(0), 0x10u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r48);
            c.w64(3, c.r64(9));
            let r49 = c.shr(c.r32(3), 0x8u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r49);
            c.w64(6, c.r64(15));
            let r50 = c.shr(c.r32(6), 0x18u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r50);
            c.w64(8, c.r64(15));
            let r51 = c.shr(c.r32(8), 0x10u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008a7532); }
            let r52 = c.shr(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r52);
            let r53 = c.add(c.r64(2), c.r64(12), 64);
            c.w64(2, r53);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(11)) { return RegfileExit::fault(0x70008a7542); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(0)) { return RegfileExit::fault(0x70008a7546); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(3)) { return RegfileExit::fault(0x70008a754a); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(9)) { return RegfileExit::fault(0x70008a754e); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x4), c.r8(6)) { return RegfileExit::fault(0x70008a7553); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x5), c.r8(8)) { return RegfileExit::fault(0x70008a7558); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x6), c.r8(15)) { return RegfileExit::fault(0x70008a755d); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a7562); };
            c.w64(1, t54);
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x7), c.r8(1)) { return RegfileExit::fault(0x70008a7566); }
            blk = 55; continue;
        }
        54 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a756c); };
            c.w64(1, t55);
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008a7570); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(15)) { return RegfileExit::fault(0x70008a7573); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(8)) { return RegfileExit::fault(0x70008a7578); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(6)) { return RegfileExit::fault(0x70008a757d); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x4), c.r8(9)) { return RegfileExit::fault(0x70008a7582); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x5), c.r8(3)) { return RegfileExit::fault(0x70008a7587); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x6), c.r8(0)) { return RegfileExit::fault(0x70008a758b); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x7), c.r8(11)) { return RegfileExit::fault(0x70008a758f); }
            blk = 55; continue;
        }
        55 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a7594); };
            c.w64(2, t56);
            let r57 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a759c); };
            c.w64(0, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a75a6); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a75a7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a75ad); }
            return c.ret(0x0, 0x70008a75af);
        }
        56 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a75b5); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a75b6); };
            c.w64(3, t59);
            return c.call(0x700086ed74, 0x70008a75c2, 0x70008a75bd);
        }
        57 => {
            return RegfileExit::fault(0x70008a75c2);
        }
        58 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a75c8); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a75c9); };
            c.w64(2, t60);
            let Some(t61) = c.ld32(c.r64(2).wrapping_add(0x1ff)) else { return RegfileExit::fault(0x70008a75d0); };
            c.w32(2, t61);
            let r62 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r62);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a75d9); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a75da); };
            c.w64(3, t63);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a75eb, 0x70008a75e6);
        }
        59 => {
            return RegfileExit::fault(0x70008a75eb);
        }
        60 => {
            c.w32(2, 0x28u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a75f1); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a75f2); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a75f4); };
            c.w64(3, t64);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a7605, 0x70008a7600);
        }
        61 => {
            return RegfileExit::fault(0x70008a7605);
        }
        62 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a760b); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a760c); };
            c.w64(2, t65);
            let Some(t66) = c.ld32(c.r64(2).wrapping_add(0x1ff)) else { return RegfileExit::fault(0x70008a7613); };
            c.w32(2, t66);
            let r67 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r67);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a761c); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a761d); };
            c.w64(3, t68);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a762e, 0x70008a7629);
        }
        63 => {
            return RegfileExit::fault(0x70008a762e);
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a762f); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a7635); };
            c.w64(3, t69);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a7644, 0x70008a763f);
        }
        65 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a7644); };
            c.w64(6, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a7648); };
            c.w64(0, t71);
            blk = 1; continue;
        }
        66 => {
            let Some(t72) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a7651); };
            c.logic(t72 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 67; continue;
        }
        67 => {
            blk = 36; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a7663); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 37; continue;
        }
        70 => {
            return RegfileExit::jump(0x70008a767c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a7694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a7694); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a7698); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7699); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a769a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a769f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a76a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a76ad); };
            c.cmp(t2, c.r32(2), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a76ba); };
            c.w64(1, t3);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a76c7); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a76ce); };
            c.logic(t6 & 0x2u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a76d9); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a76e0); };
            c.logic(t9 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a76e7); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a76ee); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008a76f2); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a76fb); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, c.r64(8));
            blk = 9; continue;
        }
        8 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7706); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a7707); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a7708); };
            c.w64(7, t15);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a770f); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x70008a7710); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008a7721, 0x70008a771c);
        }
        9 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a7721); };
            c.logic(t17 & 0x200000u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a772a); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a7731); };
            c.w64(7, t19);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a773a); };
            c.w64(12, t20);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a7741); };
            c.w64(1, t21);
            blk = 23; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a774d); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a7754); };
            c.w64(7, t23);
            c.w64(8, c.r64(7).wrapping_add(0x4c));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a775b); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a7762); };
            c.cmp(t25, c.r64(8), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a776b); }
            let r26 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r26);
            blk = 13; continue;
        }
        13 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008a7772); };
            c.w64(8, t27);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008a7779); }
            c.w32(8, 0x22u64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008a7783); }
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a7787); };
            c.w32(8, t28);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70008a779d); }
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a77a0); };
            c.w32(8, t29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x70008a77b4); }
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a77b7); };
            c.w32(8, t30);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(8, 0x8u64);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008a77d1); }
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a77d5); };
            c.w32(8, t31);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(8, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x70008a77ef); }
            let Some(t32) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a77f3); };
            c.w32(8, t32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x70008a7807); }
            c.w64(8, c.r64(7).wrapping_add(0x4b));
            c.w64(11, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7817); };
            c.w64(12, t33);
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(11), c.r32(12)) { return RegfileExit::fault(0x70008a781e); }
            let r34 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r34);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 20; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 23; continue;
        }
        22 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a782c); };
            c.w64(12, t35);
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a7833); };
            c.w32(8, t36);
            c.w64(2, c.r64(8).wrapping_mul(4));
            let Some(t37) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a7840); };
            c.w64(8, t37);
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t38) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a7847); };
            c.cmp(t38, c.r64(11), 64);
            if c.cc_be() { blk = 51; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x70008a7850); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008a785b); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008a785e); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a7861); };
            c.w64(7, t39);
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008a7868); }
            let Some(t40) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a786b); };
            c.w32(7, t40);
            c.w64(9, c.r64(7));
            let r41 = c.r32(9) & 0xe0000000u64;
            c.w32(9, r41);
            c.logic(r41, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            let r42 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r42);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70008a7884); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 27; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 35; continue;
        }
        29 => {
            let r43 = c.sub(c.r32(7), 0x20000000u64, 32);
            c.w32(7, r43);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008a7896); }
            let Some(t44) = c.ld8(c.r64(0).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008a7899); };
            c.w32(7, t44);
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008a78aa); };
            c.w64(11, t45);
            blk = 31; continue;
        }
        31 => {
            let r46 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r46);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70008a78b5); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 31; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let r47 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r47);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70008a78c8); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            let r48 = c.add(c.r32(9), 0xe0000000u64, 32);
            c.w32(9, r48);
            if c.cc_e() { blk = 53; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a78df); };
            c.w64(7, t49);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008a78e3); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a78e6); };
            c.w64(7, t50);
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008a78ea); }
            if !c.st32(c.r64(1).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x70008a78ed); }
            c.w64(0, c.r64(1));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a78f3); };
            c.w64(1, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a78fa); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return c.ret(0x10, 0x70008a7901);
        }
        37 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a7904); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a790b); }
            return c.ret(0x0, 0x70008a790d);
        }
        38 => {
            c.w32(1, 0x318u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a7913); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a7914); };
            c.w64(3, t52);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a7925, 0x70008a7920);
        }
        39 => {
            return RegfileExit::fault(0x70008a7925);
        }
        40 => {
            c.w32(1, 0x52u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a792b); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x6f8)) else { return RegfileExit::fault(0x70008a792c); };
            c.w64(1, t53);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a7933); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a7934); };
            c.w64(3, t54);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a7945, 0x70008a7940);
        }
        41 => {
            return RegfileExit::fault(0x70008a7945);
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008a7946); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008a794a); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a794e); };
            c.w64(3, t55);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a795c, 0x70008a7957);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a795c); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a7960); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a7964); };
            c.w64(2, t58);
            blk = 1; continue;
        }
        44 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008a7972); }
            return c.call(0x70007febf4, 0x70008a797b, 0x70008a7976);
        }
        45 => {
            c.w64(7, c.r64(0));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a797e); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a7982); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a7989); };
            c.w64(9, t61);
            blk = 13; continue;
        }
        46 => {
            return RegfileExit::fault(0x70008a7995);
        }
        47 => {
            return RegfileExit::fault(0x70008a7996);
        }
        48 => {
            return RegfileExit::fault(0x70008a7997);
        }
        49 => {
            return RegfileExit::fault(0x70008a7998);
        }
        50 => {
            return RegfileExit::fault(0x70008a7999);
        }
        51 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008a799a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a799e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008a79a2); }
            return c.call(0x70007febf4, 0x70008a79ab, 0x70008a79a6);
        }
        52 => {
            c.w64(1, c.r64(0));
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a79b0); };
            c.w64(0, t62);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a79b4); };
            c.w64(12, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a79bb); };
            c.w64(7, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a79bf); };
            c.w64(2, t65);
            blk = 25; continue;
        }
        53 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a79c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008a79c9); }
            c.w32(0, 0x1u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008a79d2); };
            c.w64(3, t66);
            return c.call(0x700086ed74, 0x70008a79de, 0x70008a79d9);
        }
        54 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a79de); };
            c.w64(1, t67);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        55 => {
            return RegfileExit::jump(0x70008a79ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a79f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a79f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a79f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a79f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a79fa); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a79ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a7a17); };
            c.w64(8, t2);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7a26); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a7a2d); };
            c.cmp(t5, 0x83bu64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008a7a3d); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a7a47); };
            c.w64(1, t6);
            blk = 6; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7a4d); };
            c.w64(1, t7);
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7a5d); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a7a64); };
            c.w32(11, t10);
            let r11 = c.sub(c.r32(11), 0xbau64, 32);
            c.w32(11, r11);
            c.cmp(c.r32(11), 0x78eu64, 32);
            if c.cc_a() { blk = 44; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7a83); };
            c.w64(11, t12);
            blk = 11; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7a89); };
            c.w64(11, t13);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t14) = c.ld64(c.r64(7).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a7a9f); };
            c.w64(7, t14);
            blk = 15; continue;
        }
        14 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7aa5); };
            c.w64(7, t15);
            blk = 15; continue;
        }
        15 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7aac); };
            c.w64(0, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7ab3); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, c.r64(0));
            blk = 19; continue;
        }
        17 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7acb); };
            c.w32(12, t18);
            let r19 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r19);
            let Some(t20) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a7ad2); };
            c.w32(12, t20);
            let r21 = c.sub(c.r32(12), 0xbau64, 32);
            c.w32(12, r21);
            c.cmp(c.r32(12), 0x78eu64, 32);
            if c.cc_a() { blk = 40; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70008a7aec); };
            c.w64(12, t22);
            let Some(t23) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008a7af3); };
            c.w64(15, t23);
            c.w64(3, c.r64(15).wrapping_add(0x24));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70008a7afb); };
            c.w64(2, t24);
            let Some(t25) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008a7b02); };
            c.cmp(t25, c.r64(3), 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008a7b0b); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 21; continue;
        }
        21 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2e8)) else { return RegfileExit::fault(0x70008a7b13); };
            c.w64(15, t26);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008a7b1a); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008a7b1f); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r27 = c.r64(15) & c.r64(12);
            c.w64(15, r27);
            c.logic(r27, 64);
            let Some(t28) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7b34); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 70; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008a7b3f); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r29 = c.r64(15) & c.r64(12);
            c.w64(15, r29);
            c.logic(r29, 64);
            let Some(t30) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7b53); };
            c.logic(t30 & 0x4u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008a7b5e); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r31 = c.r64(1) & c.r64(12);
            c.w64(1, r31);
            c.logic(r31, 64);
            let Some(t32) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7b73); };
            c.logic(t32 & 0x4u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008a7b7d); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r33 = c.r64(1) & c.r64(12);
            c.w64(1, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7b92); };
            c.logic(t34 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x70008a7b9c); }
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x70008a7ba1); }
            if !c.st32(c.r64(12).wrapping_add(0x1b), c.r32(0)) { return RegfileExit::fault(0x70008a7ba6); }
            if !c.st32(c.r64(12).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008a7bab); }
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a7bb0); };
            c.w32(1, t35);
            let r36 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r36);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70008a7bb7); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r37 = c.r64(11) & c.r64(12);
            c.w64(11, r37);
            c.logic(r37, 64);
            let Some(t38) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7bcb); };
            c.logic(t38 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7bd6); };
            c.cmp(t39, c.r32(1), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st32(c.r64(1).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008a7bdf); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r40 = c.r64(11) & c.r64(1);
            c.w64(11, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7bf3); };
            c.logic(t41 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008a7bfe); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r42 = c.r64(1) & c.r64(8);
            c.w64(1, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7c12); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 52; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7c1c); };
            c.cmp(t44, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7c25); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008a7c27); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2cb0)) else { return RegfileExit::fault(0x70008a7c29); };
            c.w64(3, t45);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a7c3d, 0x70008a7c38);
        }
        38 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7c3d); };
            c.w64(0, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a7c44); };
            c.w64(9, t47);
            blk = 39; continue;
        }
        39 => {
            c.w64(1, c.r64(9));
            let r48 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a7c52); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a7c53); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a7c59); }
            return c.ret(0x0, 0x70008a7c5b);
        }
        40 => {
            c.w32(1, 0x316u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a7c61); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7c62); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a7c63); };
            c.w64(3, t49);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a7c74, 0x70008a7c6f);
        }
        41 => {
            return RegfileExit::fault(0x70008a7c74);
        }
        42 => {
            c.w32(1, 0x31cu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a7c7a); }
            c.w32(0, 0x1u64);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a7c80); };
            c.w64(3, t50);
            return c.call(0x700086ed74, 0x70008a7c8c, 0x70008a7c87);
        }
        43 => {
            return RegfileExit::fault(0x70008a7c8c);
        }
        44 => {
            c.w32(1, 0x31au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a7c92); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a7c93); };
            c.w64(3, t51);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a7ca4, 0x70008a7c9f);
        }
        45 => {
            return RegfileExit::fault(0x70008a7ca4);
        }
        46 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a7caa); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a7cab); };
            c.w64(1, t52);
            let Some(t53) = c.ld32(c.r64(1).wrapping_add(0x203)) else { return RegfileExit::fault(0x70008a7cb2); };
            c.w32(1, t53);
            let r54 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r54);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a7cbb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7cbc); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a7cbe); };
            c.w64(3, t55);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008a7ccf, 0x70008a7cca);
        }
        47 => {
            return RegfileExit::fault(0x70008a7ccf);
        }
        48 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008a7cd0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008a7cd4); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a7cd8); };
            c.w64(3, t56);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a7ce9, 0x70008a7ce4);
        }
        49 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a7ce9); };
            c.w64(6, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a7ced); };
            c.w64(0, t58);
            blk = 1; continue;
        }
        50 => {
            c.w32(2, 0x24u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008a7cfb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008a7cff); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a7d03); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008a7d07); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008a7d0b); }
            return c.call(0x70007fec34, 0x70008a7d14, 0x70008a7d0f);
        }
        51 => {
            c.w64(12, c.r64(0));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a7d17); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a7d1b); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7d1f); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a7d26); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a7d2a); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a7d2e); };
            c.w64(11, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a7d32); };
            c.w64(1, t65);
            blk = 21; continue;
        }
        52 => {
            c.w32(12, c.r32(12));
            let r66 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r66);
            c.w64(1, 0xfffffffffffc0000u64);
            let r67 = c.r64(1) & c.r64(12);
            c.w64(1, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7d4b); };
            c.logic(t68 & 0x2u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(8).wrapping_add(0x13));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7d59); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7d5a); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008a7d67, 0x70008a7d62);
        }
        54 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a7d67); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a7d69); }
            blk = 36; continue;
        }
        55 => {
            c.w32(12, c.r32(12));
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            c.w64(11, 0xfffffffffffc0000u64);
            let r70 = c.r64(11) & c.r64(12);
            c.w64(11, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7d7f); };
            c.logic(t71 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(15, c.r64(1).wrapping_add(0x13));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7d8e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7d8f); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008a7d9c, 0x70008a7d97);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a7d9c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a7d9e); }
            blk = 34; continue;
        }
        58 => {
            c.w32(1, c.r32(1));
            let r72 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r72);
            c.w64(11, 0xfffffffffffc0000u64);
            let r73 = c.r64(11) & c.r64(1);
            c.w64(11, r73);
            c.logic(r73, 64);
            let Some(t74) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7db3); };
            c.logic(t74 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(15, c.r64(12).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7dc3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7dc4); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008a7dd1, 0x70008a7dcc);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a7dd1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a7dd3); }
            blk = 31; continue;
        }
        61 => {
            c.w32(11, c.r32(11));
            let r75 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r75);
            c.w64(1, 0xfffffffffffc0000u64);
            let r76 = c.r64(1) & c.r64(11);
            c.w64(1, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7de9); };
            c.logic(t77 & 0x2u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(15, c.r64(12).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7df8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7df9); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008a7e06, 0x70008a7e01);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a7e06); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a7e08); }
            blk = 29; continue;
        }
        64 => {
            c.w32(7, c.r32(7));
            let r78 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r78);
            c.w64(1, 0xfffffffffffc0000u64);
            let r79 = c.r64(1) & c.r64(7);
            c.w64(1, r79);
            c.logic(r79, 64);
            let Some(t80) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7e1d); };
            c.logic(t80 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(15, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7e2c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7e2d); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008a7e3a, 0x70008a7e35);
        }
        66 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a7e3a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a7e3c); }
            blk = 27; continue;
        }
        67 => {
            c.w32(1, c.r32(1));
            let r81 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r81);
            c.w64(15, 0xfffffffffffc0000u64);
            let r82 = c.r64(15) & c.r64(1);
            c.w64(15, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7e51); };
            c.logic(t83 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(3, c.r64(12).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7e61); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7e62); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008a7e6f, 0x70008a7e6a);
        }
        69 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a7e6f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a7e71); }
            blk = 25; continue;
        }
        70 => {
            c.w32(8, c.r32(8));
            let r84 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r84);
            c.w64(15, 0xfffffffffffc0000u64);
            let r85 = c.r64(15) & c.r64(8);
            c.w64(15, r85);
            c.logic(r85, 64);
            let Some(t86) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7e87); };
            c.logic(t86 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(3, c.r64(12).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7e97); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a7e98); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008a7ea5, 0x70008a7ea0);
        }
        72 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a7ea5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a7ea7); }
            blk = 23; continue;
        }
        73 => {
            return RegfileExit::jump(0x70008a7eb0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a7eb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a7eb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a7eb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a7eb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a7eba); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008a7ebf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a7ec3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(5));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a7ed7); };
            c.w64(11, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008a7edc); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7eea); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a7ef1); };
            c.cmp(t5, 0x83bu64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008a7f0e); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a7f18); };
            c.w64(0, t6);
            c.w64(3, c.r64(0));
            blk = 6; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7f21); };
            c.w64(3, t7);
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7f28); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7f2f); };
            c.cmp(t9, c.r32(3), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a7f38); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            c.w64(3, c.r64(9));
            blk = 13; continue;
        }
        8 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a7f4a); };
            c.w64(3, t12);
            blk = 11; continue;
        }
        10 => {
            c.w64(3, c.r64(0));
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7f5c); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            let Some(t15) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a7f62); };
            c.logic(t15 & 0x2u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008a7f6c); }
            blk = 15; continue;
        }
        14 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a7f72); };
            c.w64(11, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7f76); };
            c.w64(0, t17);
            blk = 15; continue;
        }
        15 => {
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a7f7d); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008a7f84); }
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7f88); };
            c.cmp(t20, c.r32(8), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, c.r64(0));
            blk = 25; continue;
        }
        17 => {
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a7f99); };
            c.w32(15, t21);
            let r22 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r22);
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x70008a7fa0); }
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7fa4); };
            c.cmp(t23, c.r32(15), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x70008a7fad); }
            blk = 19; continue;
        }
        19 => {
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008a7fb1); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r24 = c.r64(2) & c.r64(11);
            c.w64(2, r24);
            c.logic(r24, 64);
            let Some(t25) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a7fc5); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 41; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a7fcf); };
            c.w32(15, t26);
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a7fd3); };
            c.cmp(t27, c.r32(15), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, c.r64(4));
            let r28 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r28);
            let r29 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r29);
            c.logic(r29, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008a7fe7); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1678)) else { return RegfileExit::fault(0x70008a7ff2); };
            c.w64(0, t30);
            c.w64(10, 0x70008a8000u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008a8000); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008a8004); }
            return c.call(c.r64(0), 0x70008a800a, 0x70008a8008);
        }
        23 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008a800a); }
            let Some(t31) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008a8012); };
            c.w64(4, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a8017); };
            c.w64(11, t32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a801b); };
            c.w64(0, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a8022); };
            c.w64(8, t34);
            blk = 24; continue;
        }
        24 => {
            c.w64(9, c.r64(8));
            blk = 25; continue;
        }
        25 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a8029); };
            c.cmp(t35, c.r32(9), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t36) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a8032); };
            c.w32(9, t36);
            let r37 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r37);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a8039); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a803b); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a803c); };
            c.w64(7, t38);
            c.w32(0, 0x1u64);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a8045); };
            c.w64(6, t39);
            return c.call(0x70007f2134, 0x70008a804e, 0x70008a8049);
        }
        27 => {
            blk = 14; continue;
        }
        28 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008a8053); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2cc0)) else { return RegfileExit::fault(0x70008a8055); };
            c.w64(3, t40);
            c.w32(0, 0x1u64);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a8061); };
            c.w64(6, t41);
            return c.call(0x700086ed74, 0x70008a806a, 0x70008a8065);
        }
        29 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a806a); };
            c.w64(9, t42);
            let r43 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r43);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a8072); };
            c.w64(0, t44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a807c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a807d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a8083); }
            return c.ret(0x0, 0x70008a8085);
        }
        30 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t45) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a808c); };
            c.w64(9, t45);
            c.w64(0, c.r64(9));
            blk = 32; continue;
        }
        32 => {
            c.w32(9, 0x318u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a8099); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a809b); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a809c); };
            c.w64(3, t46);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a80ad, 0x70008a80a8);
        }
        33 => {
            return RegfileExit::fault(0x70008a80ad);
        }
        34 => {
            c.w32(9, 0x80u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a80b4); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a80b6); };
            c.w64(9, t47);
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0x207)) else { return RegfileExit::fault(0x70008a80bd); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a80c7); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008a80c9); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a80cb); };
            c.w64(3, t50);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008a80dc, 0x70008a80d7);
        }
        35 => {
            return RegfileExit::fault(0x70008a80dc);
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008a80dd); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a80e1); };
            c.w64(3, t51);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70008a80f2, 0x70008a80ed);
        }
        37 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a80f2); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a80f6); };
            c.w64(0, t53);
            blk = 1; continue;
        }
        38 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a80ff); };
            if !c.push64(t54) { return RegfileExit::fault(0x70008a80ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008a8102); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2cc0)) else { return RegfileExit::fault(0x70008a8106); };
            c.w64(3, t55);
            c.w32(0, 0x1u64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a8112); };
            c.w64(6, t56);
            return c.call(0x700086ed74, 0x70008a811b, 0x70008a8116);
        }
        39 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a811b); };
            if !c.push64(t57) { return RegfileExit::fault(0x70008a811b); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x70008a811e); };
            c.w64(3, t58);
            c.w32(0, 0x1u64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a812a); };
            c.w64(6, t59);
            return c.call(0x700086ed74, 0x70008a8133, 0x70008a812e);
        }
        40 => {
            return RegfileExit::fault(0x70008a8133);
        }
        41 => {
            c.w32(15, c.r32(15));
            let r60 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r60);
            c.w64(2, 0xfffffffffffc0000u64);
            let r61 = c.r64(2) & c.r64(15);
            c.w64(2, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8144); };
            c.logic(t62 & 0x2u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(1, c.r64(11).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a8152); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a8153); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008a8160, 0x70008a815b);
        }
        43 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a8160); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a8162); }
            blk = 21; continue;
        }
        44 => {
            return RegfileExit::jump(0x70008a816c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a8174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8174); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a8178); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a8179); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a817a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a817f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 21; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a8189); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a8196); };
            c.w64(1, t3);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a81a3); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a81a9); };
            c.w32(8, t6);
            let r7 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r7);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 28; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a81bf); };
            c.w32(7, t8);
            c.w64(8, c.r64(7));
            let r9 = c.r32(8) & 0xc0u64;
            c.w32(8, r9);
            c.logic(r9, 32);
            c.cmp(c.r32(8), 0xc0u64, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a81d6); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a81dd); };
            c.w32(8, t12);
            let r13 = c.r32(8) & 0x1fu64;
            c.w32(8, r13);
            c.logic(r13, 32);
            let r14 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r14);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_a() { blk = 28; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a81fd); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8203); };
            c.w32(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008a8207); };
            c.cmp(t18, c.r32(8), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008a8210); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 11; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a821f); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8225); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            blk = 13; continue;
        }
        12 => {
            let Some(t24) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a822d); };
            c.w32(1, t24);
            let r25 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r25);
            blk = 13; continue;
        }
        13 => {
            let Some(t26) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8233); };
            c.w32(8, t26);
            c.cmp(c.r16(8), 0x410u64, 16);
            if c.cc_be() { blk = 23; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a8240); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a8246); };
            c.cmp(t29, c.r32(1), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a8253); };
            c.w64(0, t30);
            blk = 17; continue;
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a825a); };
            c.w64(1, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a8261); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.ret(0x10, 0x70008a8268);
        }
        19 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a826b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a8272); }
            return c.ret(0x0, 0x70008a8274);
        }
        20 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a8275); };
            c.w64(0, t32);
            blk = 17; continue;
        }
        21 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a827e); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a8282); };
            c.w64(3, t33);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a8290, 0x70008a828b);
        }
        22 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a8290); };
            c.w64(6, t34);
            blk = 1; continue;
        }
        23 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a82a1); };
            c.logic(t35 & 0x24u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a82a7); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a82a8); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x23b0)) else { return RegfileExit::fault(0x70008a82a9); };
            c.w64(3, t36);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a82ba, 0x70008a82b5);
        }
        26 => {
            blk = 17; continue;
        }
        27 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a82bc); };
            c.w64(1, t37);
            blk = 28; continue;
        }
        28 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a82c0); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a82c1); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2178)) else { return RegfileExit::fault(0x70008a82c2); };
            c.w64(3, t38);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a82d3, 0x70008a82ce);
        }
        29 => {
            blk = 17; continue;
        }
        30 => {
            return RegfileExit::jump(0x70008a82d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a82f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a82f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a82f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a82f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a82fa); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a82ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a8317); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008a831c); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8326); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a832d); };
            c.w32(11, t5);
            let r6 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r6);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        4 => {
            c.w32(11, 0x1u64);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8354); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a835b); };
            c.cmp(t9, 0x80bu64, 16);
            if c.cc_ne() { blk = 56; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8368); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a836f); };
            c.w32(12, t12);
            c.logic(c.r32(12) & 0x200000u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 8; continue;
        }
        8 => {
            let r13 = c.shr(c.r32(12), 0xau64, 32).unwrap_or(c.r32(12));
            c.w32(12, r13);
            let r14 = c.r32(12) & 0x3ffu64;
            c.w32(12, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 56; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a8395); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a839c); };
            c.w32(15, t17);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008a83a8);
        }
        11 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a83a9); };
            c.w32(3, t18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a83ae); };
            c.w32(7, t19);
            let r20 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008a83b6); };
            c.cmp(t21, c.r32(3), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(3, c.r32(7));
            let r22 = c.r32(3) & 0x3u64;
            c.w32(3, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a83db); };
            c.w32(3, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xd60)) else { return RegfileExit::fault(0x70008a83de); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(15, c.r16(15));
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008a83f5);
        }
        17 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a83f6); };
            c.w32(15, t25);
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008a83fb); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x890)) else { return RegfileExit::fault(0x70008a8403); };
            c.cmp(t28, c.r32(15), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(15, c.r32(12));
            let r29 = c.r32(15) & 0x3u64;
            c.w32(15, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a842b); };
            c.w32(12, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0xd60)) else { return RegfileExit::fault(0x70008a8430); };
            c.cmp(t31, c.r32(12), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t32) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a843d); };
            c.logic(t32 & 0x40u64, 8);
            if c.cc_ne() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a8444); };
            c.w32(12, t33);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x35u64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008a8454);
        }
        24 => {
            let Some(t34) = c.ld32(c.r64(6).wrapping_add(0xdb)) else { return RegfileExit::fault(0x70008a8455); };
            c.w32(12, t34);
            let r35 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r35);
            blk = 28; continue;
        }
        25 => {
            let Some(t36) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a8461); };
            c.w32(12, t36);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x34u64, 64);
            if c.cc_a() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            return RegfileExit::fault(0x70008a8471);
        }
        27 => {
            let Some(t37) = c.ld32(c.r64(6).wrapping_add(0xd7)) else { return RegfileExit::fault(0x70008a8472); };
            c.w32(12, t37);
            let r38 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r38);
            blk = 28; continue;
        }
        28 => {
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a847c); };
            c.w32(15, t39);
            let Some(t40) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a8481); };
            c.cmp(t40, c.r32(15), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_le() { blk = 49; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(15, c.r64(11));
            let r41 = c.add(c.r64(15), 0xffffffffffffffffu64, 64);
            c.w64(15, r41);
            if c.cc_e() { blk = 48; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 61; continue; }
            blk = 32; continue;
        }
        32 => {
            let r42 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r42);
            c.w64(0, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(3, c.r32(3));
            let r43 = c.imul2(c.r32(3), 0x4u64, 32);
            c.w32(3, r43);
            if c.cc_o() { blk = 47; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r44 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r44);
            if c.cc_o() { blk = 46; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a84da); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008a84e1); };
            c.w64(6, t46);
            c.w64(7, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a84e8); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a84ef); };
            c.cmp(t48, c.r64(7), 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 59; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a8505); };
            c.w64(9, t49);
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x70008a850c); }
            c.w64(9, c.r64(6).wrapping_add(0x1));
            blk = 37; continue;
        }
        37 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008a8513); };
            c.w64(3, t50);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008a851a); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008a851e); }
            c.w64(15, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.w32(0, 0x1u64);
            blk = 39; continue;
        }
        38 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008a8531); }
            let r51 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r51);
            let r52 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r52);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_e() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_be() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t53) = c.ld64(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008a854d); };
            c.w64(3, t53);
            blk = 38; continue;
        }
        43 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a8553); };
            c.w64(3, t54);
            blk = 38; continue;
        }
        44 => {
            return RegfileExit::fault(0x70008a855c);
        }
        45 => {
            blk = 50; continue;
        }
        46 => {
            return RegfileExit::fault(0x70008a855f);
        }
        47 => {
            return RegfileExit::fault(0x70008a8560);
        }
        48 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a8561); };
            c.w64(0, t55);
            c.w64(9, c.r64(0));
            blk = 50; continue;
        }
        49 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a856d); };
            c.w64(0, t56);
            c.w64(9, c.r64(0));
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t57) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a857d); };
            c.w64(1, t57);
            blk = 53; continue;
        }
        52 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a8583); };
            c.w64(1, t58);
            blk = 53; continue;
        }
        53 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a858a); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008a8591); };
            c.w64(15, t60);
            c.w64(0, c.r64(15).wrapping_add(0x18));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a8598); };
            c.w64(3, t61);
            let Some(t62) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008a859f); };
            c.cmp(t62, c.r64(0), 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008a85a8); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008a85af); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a85b3); };
            c.w64(12, t63);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008a85ba); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008a85be); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a85c2); };
            c.w64(12, t64);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008a85c6); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008a85ca); }
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008a85ce); }
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a85dc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a85dd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a85e3); }
            return c.ret(0x0, 0x70008a85e5);
        }
        56 => {
            let Some(t65) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a85e6); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008a85ea); };
            c.w64(5, t66);
            c.w64(7, c.r64(1));
            let r67 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r67);
            return RegfileExit::jump(0x700082fa74);
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008a85fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a85fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008a8602); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a8608); };
            c.w64(3, t68);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a8617, 0x70008a8612);
        }
        58 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a8617); };
            c.w64(2, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a861b); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a861f); };
            c.w64(6, t71);
            blk = 1; continue;
        }
        59 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008a8628); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008a862c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008a8630); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008a8634); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a8638); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008a863c); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008a8648, 0x70008a8643);
        }
        60 => {
            c.w64(9, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a864b); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a864f); };
            c.w64(11, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a8653); };
            c.w64(12, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a8657); };
            c.w64(8, t75);
            c.w32(2, 0x8u64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a8660); };
            c.w64(0, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a8664); };
            c.w64(15, t77);
            blk = 37; continue;
        }
        61 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008a866d); };
            c.w64(3, t78);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a867d, 0x70008a8678);
        }
        62 => {
            return RegfileExit::fault(0x70008a867d);
        }
        63 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008a8683); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008a8687); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a868b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008a868f); }
            return c.call(0x70007febb4, 0x70008a8698, 0x70008a8693);
        }
        64 => {
            c.w64(8, c.r64(0));
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a869b); };
            c.w64(11, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a869f); };
            c.w64(9, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a86a3); };
            c.w64(1, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a86a7); };
            c.w64(12, t82);
            blk = 55; continue;
        }
        65 => {
            return RegfileExit::jump(0x70008a86b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a86d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a86d4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a86da); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a86e0); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xdc0)) else { return RegfileExit::fault(0x70008a86e3); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a86ec); };
            c.w32(1, t6);
            c.logic(c.r8(1) & 0x8u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008a86f4); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a86fd); };
            c.w64(1, (t9 as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(3));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_b() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x70008a870c);
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a870d); };
            c.w32(1, t10);
            let r11 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r11);
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008a8714); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a8718); };
            c.w64(0, t12);
            return c.ret(0x0, 0x70008a871f);
        }
        5 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a8720); };
            c.w64(0, t13);
            return c.ret(0x0, 0x70008a8727);
        }
        6 => {
            return RegfileExit::jump(0x70008a872c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a8734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8734); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008a8737); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008a8740); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            c.w32(8, c.r32(7));
            let r4 = c.r32(8) & 0x3u64;
            c.w32(8, r4);
            c.logic(r4, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a875a); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8761); };
            c.cmp(t7, 0x834u64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70008a876d);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a876e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8772); }
            let r8 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r8);
            blk = 9; continue;
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a877a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a877e); }
            let r9 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r9);
            blk = 9; continue;
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8786); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a878a); }
            let r10 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r10);
            blk = 9; continue;
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8792); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8796); }
            let r11 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r11);
            blk = 9; continue;
        }
        9 => {
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008a879f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a87a0); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a87a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008a87a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008a87a6); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2130)) else { return RegfileExit::fault(0x70008a87aa); };
            c.w64(3, t12);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008a87bb, 0x70008a87b6);
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a87bb); };
            c.w64(2, t13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a87bf); };
            c.w32(1, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008a87c2); };
            c.cmp(t15, c.r32(1), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a87cb); };
            c.w64(1, t16);
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008a87d7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r17 = c.r64(7) & c.r64(2);
            c.w64(7, r17);
            c.logic(r17, 64);
            let Some(t18) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a87e8); };
            c.logic(t18 & 0x4u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a87f1); }
            return c.ret(0x0, 0x70008a87f2);
        }
        14 => {
            blk = 13; continue;
        }
        15 => {
            c.w32(0, c.r32(0));
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            c.w64(7, 0xfffffffffffc0000u64);
            let r20 = c.r64(7) & c.r64(0);
            c.w64(7, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8804); };
            c.logic(t21 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a880e); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x70008a8817, 0x70008a8812);
        }
        17 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a8817); }
            blk = 13; continue;
        }
        18 => {
            return RegfileExit::jump(0x70008a881c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a8834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008a8834); };
            if !c.push64(t0) { return RegfileExit::fault(0x70008a8834); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2150)) else { return RegfileExit::fault(0x70008a8837); };
            c.w64(3, t1);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008a883e); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008a8850);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a8854(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a8854); };
            c.w32(8, t0);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008a885d);
        }
        2 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t1) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8861); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8868); };
            c.cmp(t3, c.r32(2), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a886d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8871); }
            let Some(t4) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a8873); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008a8877); };
            c.w32(7, t5);
            c.w32(8, c.r32(7));
            let r6 = c.r32(8) | 0xeu64;
            c.w32(8, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008a888a); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a888e); }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000857fb4, 0x70008a889d, 0x70008a8898);
        }
        6 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a88a0); }
            return c.ret(0x8, 0x70008a88a1);
        }
        7 => {
            c.w64(0, c.r64(1));
            return c.ret(0x8, 0x70008a88a7);
        }
        8 => {
            return RegfileExit::jump(0x70008a88ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a88b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008a88b7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a88bb); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a88c1); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a88c7); };
            c.w32(7, t5);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7au64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008a88d6);
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x1ef)) else { return RegfileExit::fault(0x70008a88d7); };
            c.w32(1, t6);
            c.w64(7, c.r64(2));
            blk = 4; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a88e2); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a88e8); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a88ed); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008a88fd);
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a88fe); };
            c.w32(1, t11);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008a890d);
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a890e); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a8914); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008a891a); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x0, 0x70008a8923);
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8924); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8928); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x22a0)) else { return RegfileExit::fault(0x70008a892a); };
            c.w64(3, t17);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a893b, 0x70008a8936);
        }
        12 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a893e); }
            return c.ret(0x0, 0x70008a893f);
        }
        13 => {
            return RegfileExit::jump(0x70008a8944);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a8954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8954); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8958); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008a895e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008a8962); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008a8966); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008a896a); }
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a896e); };
            c.cmp(t1, c.r32(2), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008a8977); };
            c.w64(3, t2);
            c.w64(7, c.r64(3));
            return c.call(0x7000870914, 0x70008a8986, 0x70008a8981);
        }
        2 => {
            blk = 4; continue;
        }
        3 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(10));
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008a899a); };
            c.w64(1, t3);
            return c.call(0x70008308f4, 0x70008a89a6, 0x70008a89a1);
        }
        4 => {
            c.w64(3, c.r64(0));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a89a9); };
            c.w64(0, t4);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a89ad); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a89b1); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a89b5); };
            c.w64(6, t7);
            return c.call(0x70008a8b74, 0x70008a89be, 0x70008a89b9);
        }
        5 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a89c1); }
            return c.ret(0x0, 0x70008a89c2);
        }
        6 => {
            return RegfileExit::jump(0x70008a89c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a89d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a89d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a89d8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008a89de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008a89e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a89e6); };
            c.w64(7, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008a89ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008a89ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a89f2); }
            c.w64(6, c.r64(8));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008a89f9); }
            c.w64(2, c.r64(0));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008a8a00); };
            c.w64(1, t4);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(7));
            c.w64(9, c.r64(1));
            return c.call(0x70008308f4, 0x70008a8a15, 0x70008a8a10);
        }
        1 => {
            c.w64(3, c.r64(0));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a8a18); };
            c.w64(0, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a8a1c); };
            c.w64(1, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a8a20); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a8a24); };
            c.w64(6, t8);
            return c.call(0x70008a8b74, 0x70008a8a2d, 0x70008a8a28);
        }
        2 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a8a30); }
            return c.ret(0x0, 0x70008a8a31);
        }
        3 => {
            return RegfileExit::jump(0x70008a8a34);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a8a54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8a54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8a58); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008a8a5e); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008a8a62); };
            c.w64(1, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008a8a66); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008a8a6a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8a76); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8a7c); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008a8a88); };
            c.w64(3, t6);
            c.w64(6, c.r64(1));
            return c.call(0x7000870914, 0x70008a8a97, 0x70008a8a92);
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a8a97); };
            c.w64(6, t7);
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8a9b); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8aa1); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a8aa7); };
            c.w32(7, t12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x2du64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008a8ab6);
        }
        5 => {
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0xbb)) else { return RegfileExit::fault(0x70008a8ab7); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a8ac0); };
            c.w32(7, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a8ac4); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008a8acb); };
            c.w64(9, t17);
            c.w64(2, c.r64(7).wrapping_mul(4));
            c.w64(7, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a8ada); };
            c.w64(11, t18);
            let Some(t19) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a8ae1); };
            c.cmp(t19, c.r64(7), 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(8), c.r64(7)) { return RegfileExit::fault(0x70008a8ae6); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a8aed); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a8af0); };
            c.w64(1, t20);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008a8af7); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008a8afa); }
            c.w64(1, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a8b07); };
            c.w64(8, t21);
            blk = 9; continue;
        }
        9 => {
            let r22 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r22);
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(8)) { return RegfileExit::fault(0x70008a8b12); }
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a8b1c); };
            c.w64(1, t23);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008a8b20); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008a8b23); }
            c.w64(0, c.r64(7));
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a8b2c); }
            return c.ret(0x0, 0x70008a8b2d);
        }
        12 => {
            blk = 15; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008a8b30); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008a8b34); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008a8b38); }
            return c.call(0x70007febf4, 0x70008a8b41, 0x70008a8b3c);
        }
        14 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a8b44); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a8b48); };
            c.w64(2, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a8b4c); };
            c.w64(1, t26);
            blk = 7; continue;
        }
        15 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x70008a8b52); };
            c.w64(3, t27);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x70008a8b63, 0x70008a8b5e);
        }
        16 => {
            blk = 11; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008a8b68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a8b74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8b74); };
            c.w32(8, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008a8b78); };
            c.cmp(t1, c.r32(8), 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x1f));
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008a8b94); };
            c.w32(9, t2);
            let r3 = c.add(c.r32(9), 0x8u64, 32);
            c.w32(9, r3);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70008a8ba0); }
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008a8ba5); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            c.w32(11, c.r32(9));
            let r6 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008a8bbd); };
            c.cmp(t7, c.r32(9), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008a8bca); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008a8be1); };
            c.w32(11, t9);
            c.w64(12, c.r64(9));
            let r10 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r10);
            c.logic(r10, 64);
            c.w32(15, c.r32(11));
            let r11 = c.r32(15) & 0x3u64;
            c.w32(15, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8bfa); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8bfe); }
            blk = 30; continue;
        }
        7 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8c1b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8c1f); }
            blk = 30; continue;
        }
        10 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8c26); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8c2d); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a8c34); };
            c.w32(15, t16);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008a8c44);
        }
        12 => {
            c.w32(15, c.r32(3));
            let r17 = c.r32(15) & 0x1u64;
            c.w32(15, r17);
            c.logic(r17, 32);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008a8c4c); };
            c.cmp(t18, c.r32(3), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8c52); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8c56); }
            blk = 30; continue;
        }
        14 => {
            c.w32(9, c.r32(9));
            let r19 = c.r32(9) & 0xfffffffffffffffdu64;
            c.w32(9, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008a8c6d); };
            c.w32(9, t20);
            c.w32(11, c.r32(9));
            let r21 = c.r32(11) & 0x3u64;
            c.w32(11, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r22 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r22);
            c.logic(r22, 64);
            let r23 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r23);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70008a8c92); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a8c97); }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(9, c.r64(3));
            blk = 21; continue;
        }
        20 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a8cad); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            blk = 21; continue;
        }
        21 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8cb4); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8cbb); };
            c.w32(11, t28);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8cc8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8ccc); }
            c.w32(11, c.r16(11));
            let r29 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r29);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 59; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a8ce3); };
            c.w32(9, t30);
            let r31 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r31);
            let Some(t32) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8cea); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8cf1); };
            c.w32(9, t34);
            let r35 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8cf8); };
            c.w32(11, t36);
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8cfc); };
            c.cmp(t37, c.r32(11), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(8, c.r64(3));
            let r38 = c.r64(8) | 0x3u64;
            c.w64(8, r38);
            c.logic(r38, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x70008a8d0d); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r39 = c.r64(9) & c.r64(2);
            c.w64(9, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8d21); };
            c.logic(t40 & 0x4u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a8d2c); }
            blk = 63; continue;
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8d38); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8d3c); }
            blk = 59; continue;
        }
        28 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8d43); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8d47); }
            blk = 63; continue;
        }
        29 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8d4e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8d52); }
            blk = 30; continue;
        }
        30 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t41) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8d5d); };
            c.w32(9, t41);
            let r42 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r42);
            let Some(t43) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8d64); };
            c.w32(9, t43);
            let r44 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r44);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 59; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t45) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8d7a); };
            c.w32(9, t45);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t46) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8d87); };
            c.w32(11, t46);
            let r47 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r47);
            let Some(t48) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8d8f); };
            c.w32(11, t48);
            let r49 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r49);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 59; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8da5); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 35; continue;
        }
        35 => {
            let r52 = c.r64(11) | 0x3u64;
            c.w64(11, r52);
            c.logic(r52, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70008a8dba); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r53 = c.r64(8) & c.r64(2);
            c.w64(8, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8dce); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 72; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a8dd9); }
            blk = 63; continue;
        }
        38 => {
            let Some(t55) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008a8de5); };
            c.w32(9, t55);
            c.w32(11, c.r32(9));
            let r56 = c.r32(11) & 0x3u64;
            c.w32(11, r56);
            c.logic(r56, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r57 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let r58 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70008a8e0a); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a8e0f); }
            blk = 41; continue;
        }
        41 => {
            let Some(t59) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8e16); };
            c.w32(9, t59);
            let r60 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r60);
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8e1d); };
            c.w32(9, t61);
            let r62 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r62);
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a8e24); };
            c.w32(11, t63);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xfu64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::fault(0x70008a8e34);
        }
        43 => {
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x43)) else { return RegfileExit::fault(0x70008a8e35); };
            c.cmp(t64, c.r32(3), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(9, c.r64(3));
            blk = 49; continue;
        }
        45 => {
            let Some(t65) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008a8e40); };
            c.w32(9, t65);
            c.w32(11, c.r32(9));
            let r66 = c.r32(11) & 0x3u64;
            c.w32(11, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r67 = c.r64(9) | 0x2u64;
            c.w64(9, r67);
            c.logic(r67, 64);
            let r68 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r68);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70008a8e65); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a8e6a); }
            blk = 48; continue;
        }
        48 => {
            c.w64(9, c.r64(0));
            blk = 49; continue;
        }
        49 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(11, c.r64(9));
            blk = 52; continue;
        }
        51 => {
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a8e83); };
            c.w32(11, t69);
            let r70 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r70);
            blk = 52; continue;
        }
        52 => {
            let Some(t71) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8e8a); };
            c.w32(12, t71);
            let r72 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r72);
            let Some(t73) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a8e91); };
            c.w32(12, t73);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 51; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8e9f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8ea3); }
            c.w32(12, c.r16(12));
            let r74 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r74);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 59; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t75) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a8eb6); };
            c.w32(11, t75);
            let r76 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r76);
            let Some(t77) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8ebd); };
            c.w32(12, t77);
            let r78 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r78);
            let Some(t79) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8ec4); };
            c.w32(11, t79);
            let r80 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8ecb); };
            c.w32(12, t81);
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a8ed0); };
            c.cmp(t82, c.r32(12), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let r83 = c.r64(9) | 0x3u64;
            c.w64(9, r83);
            c.logic(r83, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x70008a8eda); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r84 = c.r64(8) & c.r64(2);
            c.w64(8, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8eee); };
            c.logic(t85 & 0x4u64, 8);
            if c.cc_ne() { blk = 69; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a8ef9); }
            blk = 63; continue;
        }
        58 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8f02); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8f06); }
            blk = 59; continue;
        }
        59 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70008a8f08); };
            c.w64(1, t86);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70008a8f0f); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008a8f14); }
            blk = 63; continue;
        }
        60 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8f1d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8f21); }
            blk = 63; continue;
        }
        61 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8f25); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8f29); }
            blk = 63; continue;
        }
        62 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a8f2d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a8f31); }
            blk = 63; continue;
        }
        63 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t87) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a8f38); };
            c.w32(2, t87);
            let r88 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r88);
            let Some(t89) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a8f3e); };
            c.logic(t89 & 0x2u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a8f44); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            return c.call(0x70007f2274, 0x70008a8f4f, 0x70008a8f4a);
        }
        66 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a8f52); }
            return c.ret(0x0, 0x70008a8f53);
        }
        67 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a8f54); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x70008a8f55); };
            c.w64(3, t90);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a8f66, 0x70008a8f61);
        }
        68 => {
            return RegfileExit::fault(0x70008a8f66);
        }
        69 => {
            c.w32(9, c.r32(9));
            let r91 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r91);
            c.w64(8, 0xfffffffffffc0000u64);
            let r92 = c.r64(8) & c.r64(9);
            c.w64(8, r92);
            c.logic(r92, 64);
            let Some(t93) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8f77); };
            c.logic(t93 & 0x2u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a8f86); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a8f87); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008a8f94, 0x70008a8f8f);
        }
        71 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a8f94); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a8f96); }
            blk = 57; continue;
        }
        72 => {
            c.w32(11, c.r32(11));
            let r94 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r94);
            c.w64(8, 0xfffffffffffc0000u64);
            let r95 = c.r64(8) & c.r64(11);
            c.w64(8, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8fac); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a8fbb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a8fbc); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008a8fc9, 0x70008a8fc4);
        }
        74 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a8fc9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a8fcb); }
            blk = 37; continue;
        }
        75 => {
            c.w32(8, c.r32(8));
            let r97 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r97);
            c.w64(9, 0xfffffffffffc0000u64);
            let r98 = c.r64(9) & c.r64(8);
            c.w64(9, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a8fe1); };
            c.logic(t99 & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a8ff0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a8ff1); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008a8ffe, 0x70008a8ff9);
        }
        77 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a8ffe); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a9000); }
            blk = 26; continue;
        }
        78 => {
            return RegfileExit::jump(0x70008a9008);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a901a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a901f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a9023); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 21; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a902d); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x70008a9038, 0x70008a9033);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9038); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9042); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9045); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 15; continue;
        }
        7 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let r6 = c.sub(c.r32(7), c.r32(1), 32);
            c.w32(7, r6);
            if c.cc_o() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 10; continue;
        }
        10 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a905e); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9065); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x10, 0x70008a906c);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a906f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9076); }
            return c.ret(0x0, 0x70008a9078);
        }
        13 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a9079); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0x20b)) else { return RegfileExit::fault(0x70008a9080); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            blk = 10; continue;
        }
        14 => {
            blk = 10; continue;
        }
        15 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a908d); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x11c8)) else { return RegfileExit::fault(0x70008a9092); };
            c.w64(10, t11);
            if !c.and128_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008a9099); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            if c.cc_ne() { blk = 23; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(7, c.r64(1));
            let r12 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r12);
            if c.cc_o() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 10; continue;
        }
        21 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a90d2); };
            c.w64(3, t13);
            return c.call(0x700086ed74, 0x70008a90de, 0x70008a90d9);
        }
        22 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a90de); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        23 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a90e7); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a90ee); };
            c.w64(7, t16);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008a90f5); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a90fa); };
            c.w64(9, t17);
            let Some(t18) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a9101); };
            c.cmp(t18, c.r64(8), 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a9110, 0x70008a910b);
        }
        25 => {
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a9112); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 27; continue;
        }
        27 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a911c); };
            c.w64(1, t19);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a9123); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a9126); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a912b); }
            blk = 10; continue;
        }
        28 => {
            return RegfileExit::jump(0x70008a9138);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9154); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9158); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9159); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a915a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a915f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a9163); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 26; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a916d); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x70008a9178, 0x70008a9173);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9178); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9180); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9183); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 10; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a918e); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9195); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x10, 0x70008a919c);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a919f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a91a6); }
            return c.ret(0x0, 0x70008a91a8);
        }
        10 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a91a9); }
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
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a91db); };
            c.w64(10, t7);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008a91e2); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(1, 10);
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_be() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(0));
            blk = 15; continue;
        }
        15 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a9218); };
            c.w64(10, t8);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008a921f); }
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        16 => {
            let Some(t9) = c.ld64(0x70008a91e8u64) else { return RegfileExit::fault(0x70008a9228); };
            c.w64(10, t9);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        19 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(2));
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 28; continue; }
            blk = 21; continue;
        }
        21 => {
            if c.cc_ne() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(7, c.r64(1));
            let r10 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r10);
            if c.cc_o() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 7; continue;
        }
        26 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a9298); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a92a6, 0x70008a92a1);
        }
        27 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a92a6); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        28 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a92af); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a92b6); };
            c.w64(7, t14);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008a92bd); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a92c2); };
            c.w64(9, t15);
            let Some(t16) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a92c9); };
            c.cmp(t16, c.r64(8), 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a92d8, 0x70008a92d3);
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a92da); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 32; continue;
        }
        32 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a92e4); };
            c.w64(1, t17);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a92eb); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a92ee); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a92f3); }
            blk = 7; continue;
        }
        33 => {
            return RegfileExit::jump(0x70008a9300);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9314); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9318); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9319); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a931a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a931f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a9323); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 26; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a932d); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x70008a9338, 0x70008a9333);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9338); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9340); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9343); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 10; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a934e); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9355); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x10, 0x70008a935c);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a935f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9366); }
            return c.ret(0x0, 0x70008a9368);
        }
        10 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9369); }
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
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a939b); };
            c.w64(10, t7);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008a93a2); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(1, 10);
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_be() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(2));
            c.mov128_rr(2, 0);
            blk = 15; continue;
        }
        15 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a93db); };
            c.w64(10, t8);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008a93e2); }
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        16 => {
            let Some(t9) = c.ld64(0x70008a93a8u64) else { return RegfileExit::fault(0x70008a93eb); };
            c.w64(10, t9);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        19 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(0));
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 28; continue; }
            blk = 21; continue;
        }
        21 => {
            if c.cc_ne() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(7, c.r64(1));
            let r10 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r10);
            if c.cc_o() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 7; continue;
        }
        26 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a945e); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a946c, 0x70008a9467);
        }
        27 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a946c); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        28 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a9475); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a947c); };
            c.w64(7, t14);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008a9483); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a9488); };
            c.w64(9, t15);
            let Some(t16) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a948f); };
            c.cmp(t16, c.r64(8), 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a949e, 0x70008a9499);
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a94a0); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 32; continue;
        }
        32 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a94aa); };
            c.w64(1, t17);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a94b1); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a94b4); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a94b9); }
            blk = 7; continue;
        }
        33 => {
            return RegfileExit::jump(0x70008a94c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a94d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a94d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a94d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a94d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a94da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a94df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a94e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 31; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a94ed); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x70008a94f8, 0x70008a94f3);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a94f8); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9500); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9503); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 10; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a950e); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9515); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x10, 0x70008a951c);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a951f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9526); }
            return c.ret(0x0, 0x70008a9528);
        }
        10 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9529); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.mov128_rr(1, 0);
            blk = 23; continue;
        }
        14 => {
            c.mov128_rr(1, 0);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a9562); };
            c.w64(10, t7);
            if !c.xorpd_rm(1, c.r64(10)) { return RegfileExit::fault(0x70008a9569); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(2, 10);
            c.mov128_rr(3, 1);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(1));
            if c.cc_be() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(1));
            blk = 16; continue;
        }
        16 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a959f); };
            c.w64(10, t8);
            if !c.xorpd_rm(3, c.r64(10)) { return RegfileExit::fault(0x70008a95a6); }
            c.mov128_rr(1, 3);
            blk = 23; continue;
        }
        17 => {
            c.mov128_rr(1, 0);
            blk = 23; continue;
        }
        18 => {
            let Some(t9) = c.ld64(0x70008a956fu64) else { return RegfileExit::fault(0x70008a95b4); };
            c.w64(10, t9);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.mov128_rr(1, 2);
            blk = 23; continue;
        }
        21 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(2));
            blk = 23; continue;
        }
        22 => {
            c.mov128_rr(1, 0);
            blk = 23; continue;
        }
        23 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x37);
            c.psrlq(2, 0x2);
            c.mov128_rr(3, 1);
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(3));
            if c.cc_ae() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 1, c.xmm_u64(0));
            blk = 25; continue;
        }
        25 => {
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            if c.cc_ne() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 33; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(7, c.r64(1));
            let r10 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r10);
            if c.cc_o() { blk = 33; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 7; continue;
        }
        31 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a9656); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a9664, 0x70008a965f);
        }
        32 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9664); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        33 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a966d); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a9674); };
            c.w64(7, t14);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x70008a967b); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a9680); };
            c.w64(9, t15);
            let Some(t16) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a9687); };
            c.cmp(t16, c.r64(8), 64);
            if c.cc_a() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a9696, 0x70008a9691);
        }
        35 => {
            blk = 37; continue;
        }
        36 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a9698); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 37; continue;
        }
        37 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a96a2); };
            c.w64(1, t17);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a96a9); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a96ac); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008a96b1); }
            blk = 7; continue;
        }
        38 => {
            return RegfileExit::jump(0x70008a96bc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a96d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a96d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a96d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a96d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a96da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a96df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a96e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 26; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a96ed); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x70008a96f8, 0x70008a96f3);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a96f8); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9700); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9703); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 10; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a970e); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9715); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x10, 0x70008a971c);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a971f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9726); }
            return c.ret(0x0, 0x70008a9728);
        }
        10 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9729); }
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
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a975b); };
            c.w64(10, t7);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008a9762); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(1, 10);
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_be() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(0));
            blk = 15; continue;
        }
        15 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a9798); };
            c.w64(10, t8);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008a979f); }
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        16 => {
            let Some(t9) = c.ld64(0x70008a9768u64) else { return RegfileExit::fault(0x70008a97a8); };
            c.w64(10, t9);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        19 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(0));
            c.mov128_rr(0, 2);
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 28; continue; }
            blk = 21; continue;
        }
        21 => {
            if c.cc_ne() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(7, c.r64(1));
            let r10 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r10);
            if c.cc_o() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 7; continue;
        }
        26 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a981b); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a9829, 0x70008a9824);
        }
        27 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9829); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        28 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a9832); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a9839); };
            c.w64(7, t14);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008a9840); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a9845); };
            c.w64(9, t15);
            let Some(t16) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a984c); };
            c.cmp(t16, c.r64(8), 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a985b, 0x70008a9856);
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a985d); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 32; continue;
        }
        32 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9867); };
            c.w64(1, t17);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a986e); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a9871); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a9876); }
            blk = 7; continue;
        }
        33 => {
            return RegfileExit::jump(0x70008a9884);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9894); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9898); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9899); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a989a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008a989f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a98a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a98ad); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x70008a98b8, 0x70008a98b3);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a98b8); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a98c0); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a98c3); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a98cc); }
            blk = 8; continue;
        }
        7 => {
            let r6 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r6);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 2) { return RegfileExit::fault(0x70008a98dd); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008a98e2); };
            c.w64(0, t7);
            blk = 11; continue;
        }
        9 => {
            return c.call(0x70008a1014, 0x70008a98ed, 0x70008a98e8);
        }
        10 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a98ed); };
            c.w64(6, t8);
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a98f5); };
            c.w32(1, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a98f8); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9901); }
            blk = 15; continue;
        }
        14 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a9912); }
            c.mov128_rr(1, 2);
            c.w64(10, c.r64(4));
            let r12 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r12);
            let r13 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r13);
            c.logic(r13, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008a9925); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70008a992a); };
            c.w64(0, t14);
            c.w64(10, 0x70008a9938u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008a9938); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008a993c); }
            return c.call(c.r64(0), 0x70008a9942, 0x70008a9940);
        }
        16 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008a9942); }
            let Some(t15) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008a994a); };
            c.w64(4, t15);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 27; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_ne() { blk = 27; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 27; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(1));
            let r16 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r16);
            if c.cc_o() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        22 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a9980); };
            c.w64(1, t17);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9987); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return c.ret(0x18, 0x70008a998e);
        }
        24 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a9991); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9998); }
            return c.ret(0x0, 0x70008a999a);
        }
        25 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a999b); };
            c.w64(3, t18);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a99a9, 0x70008a99a4);
        }
        26 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a99a9); };
            c.w64(6, t19);
            blk = 1; continue;
        }
        27 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a99b2); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a99b9); };
            c.w64(7, t21);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008a99c0); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a99c5); };
            c.w64(9, t22);
            let Some(t23) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a99cc); };
            c.cmp(t23, c.r64(8), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a99db, 0x70008a99d6);
        }
        29 => {
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a99dd); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 31; continue;
        }
        31 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a99e7); };
            c.w64(1, t24);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a99ee); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a99f1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a99f6); }
            blk = 22; continue;
        }
        32 => {
            return RegfileExit::jump(0x70008a9a00);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9a14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9a14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9a18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9a19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a9a1a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008a9a1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a9a23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 24; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a9a30); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x34);
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        2 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 31; continue; }
            blk = 3; continue;
        }
        3 => {
            if c.cc_a() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if c.cc_b() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let m = c.movmskpd(0); c.w32(10, m);
            c.logic(c.r8(10) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.movsd_rr(0, 1);
            blk = 7; continue;
        }
        7 => {
            let r2 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r2);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ge() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008a9a67); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a9a6c); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(8, c.r64(5));
            let r3 = c.add(c.r64(8), 0x18u64, 64);
            c.w64(8, r3);
            let Some(t4) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008a9a7c); };
            c.w64(8, t4);
            blk = 14; continue;
        }
        11 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a9a82); };
            c.w64(8, t5);
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a1014, 0x70008a9a93, 0x70008a9a8e);
        }
        13 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008a9a96); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9a9b); };
            c.w64(1, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a9a9f); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a9aa3); };
            c.w64(6, t8);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9aad); };
            c.w32(9, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9ab1); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9aba); }
            blk = 2; continue;
        }
        17 => {
            let r11 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r11);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 2; continue;
        }
        18 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            let r12 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r12);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            if c.cc_ne() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(7));
            let r13 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r13);
            if c.cc_o() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9b10); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a9b11); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9b17); }
            return c.ret(0x0, 0x70008a9b19);
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008a9b1a); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a9b1e); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a9b2f, 0x70008a9b2a);
        }
        25 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a9b2f); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9b33); };
            c.w64(0, t16);
            blk = 1; continue;
        }
        26 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a9b3c); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a9b43); };
            c.w64(8, t18);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a9b4a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008a9b4e); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a9b53); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a9b5a); };
            c.cmp(t20, c.r64(9), 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a9b69, 0x70008a9b64);
        }
        28 => {
            blk = 30; continue;
        }
        29 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008a9b6b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 30; continue;
        }
        30 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9b75); };
            c.w64(1, t21);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a9b7c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008a9b7f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a9b84); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9b89); };
            c.w64(1, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9b90); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a9b91); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9b97); }
            return c.ret(0x0, 0x70008a9b99);
        }
        31 => {
            c.xorpd_rr(0, 0);
            c.arith_sd(3, 0, c.xmm_u64(0));
            blk = 7; continue;
        }
        32 => {
            return RegfileExit::jump(0x70008a9ba8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9bb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9bb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9bb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9bb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a9bba); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008a9bbf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a9bc3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 24; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a9bd0); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x35);
            c.psrlq(0, 0x1);
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        2 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 31; continue; }
            blk = 3; continue;
        }
        3 => {
            if c.cc_b() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let m = c.movmskpd(1); c.w32(10, m);
            c.logic(c.r8(10) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.movsd_rr(0, 1);
            blk = 7; continue;
        }
        7 => {
            let r2 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r2);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ge() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008a9c0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a9c11); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(8, c.r64(5));
            let r3 = c.add(c.r64(8), 0x18u64, 64);
            c.w64(8, r3);
            let Some(t4) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008a9c21); };
            c.w64(8, t4);
            blk = 14; continue;
        }
        11 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a9c27); };
            c.w64(8, t5);
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a1014, 0x70008a9c38, 0x70008a9c33);
        }
        13 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008a9c3b); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9c40); };
            c.w64(1, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a9c44); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a9c48); };
            c.w64(6, t8);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9c52); };
            c.w32(9, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9c56); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9c5f); }
            blk = 2; continue;
        }
        17 => {
            let r11 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r11);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 2; continue;
        }
        18 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            let r12 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r12);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            if c.cc_ne() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(7));
            let r13 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r13);
            if c.cc_o() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9cb5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a9cb6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9cbc); }
            return c.ret(0x0, 0x70008a9cbe);
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008a9cbf); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a9cc3); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a9cd4, 0x70008a9ccf);
        }
        25 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a9cd4); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9cd8); };
            c.w64(0, t16);
            blk = 1; continue;
        }
        26 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a9ce1); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a9ce8); };
            c.w64(8, t18);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a9cef); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008a9cf3); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a9cf8); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a9cff); };
            c.cmp(t20, c.r64(9), 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a9d0e, 0x70008a9d09);
        }
        28 => {
            blk = 30; continue;
        }
        29 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008a9d10); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 30; continue;
        }
        30 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9d1a); };
            c.w64(1, t21);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a9d21); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008a9d24); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a9d29); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9d2e); };
            c.w64(1, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9d35); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a9d36); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9d3c); }
            return c.ret(0x0, 0x70008a9d3e);
        }
        31 => {
            c.xorpd_rr(0, 0);
            c.arith_sd(3, 0, c.xmm_u64(0));
            blk = 7; continue;
        }
        32 => {
            return RegfileExit::jump(0x70008a9d50);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9d54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9d58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9d59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a9d5a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a9d5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a9d69); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9d79); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008a9d98); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x12e0)) else { return RegfileExit::fault(0x70008a9d9d); };
            c.w64(0, t6);
            c.w64(10, 0x70008a9dabu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008a9dab); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008a9daf); }
            return c.call(c.r64(0), 0x70008a9db5, 0x70008a9db3);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008a9db5); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008a9dbd); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a9df3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9dfa); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008a9e01);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a9e04); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9e0b); }
            return c.ret(0x0, 0x70008a9e0d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a9e0e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a9e12); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a9e20, 0x70008a9e1b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9e20); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9e29); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9e2c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008a9e3e, 0x70008a9e39);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a9e43); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a9e4a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008a9e51); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a9e56); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a9e5d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a9e6c, 0x70008a9e67);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a9e6e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9e78); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a9e7f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a9e82); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a9e87); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008a9e94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a9eb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a9eb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a9eb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a9eb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a9eba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a9ebf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a9ec9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a9ed9); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008a9ef8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x12e8)) else { return RegfileExit::fault(0x70008a9efd); };
            c.w64(0, t6);
            c.w64(10, 0x70008a9f0bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008a9f0b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008a9f0f); }
            return c.call(c.r64(0), 0x70008a9f15, 0x70008a9f13);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008a9f15); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008a9f1d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a9f53); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a9f5a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008a9f61);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a9f64); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a9f6b); }
            return c.ret(0x0, 0x70008a9f6d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a9f6e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a9f72); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a9f80, 0x70008a9f7b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a9f80); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a9f89); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9f8c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008a9f9e, 0x70008a9f99);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a9fa3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a9faa); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008a9fb1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a9fb6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a9fbd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a9fcc, 0x70008a9fc7);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a9fce); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a9fd8); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a9fdf); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008a9fe2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a9fe7); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008a9ff4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa01a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa01f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa029); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa039); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aa058); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x12f0)) else { return RegfileExit::fault(0x70008aa05d); };
            c.w64(0, t6);
            c.w64(10, 0x70008aa06bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aa06b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aa06f); }
            return c.call(c.r64(0), 0x70008aa075, 0x70008aa073);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aa075); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aa07d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aa0b3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aa0ba); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aa0c1);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aa0c4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aa0cb); }
            return c.ret(0x0, 0x70008aa0cd);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aa0ce); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aa0d2); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aa0e0, 0x70008aa0db);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aa0e0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa0e9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa0ec); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aa0fe, 0x70008aa0f9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aa103); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aa10a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aa111); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aa116); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aa11d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aa12c, 0x70008aa127);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aa12e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa138); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aa13f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aa142); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aa147); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aa154);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa174); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa178); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa179); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa17a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa17f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa189); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa199); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aa1b8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x12f8)) else { return RegfileExit::fault(0x70008aa1bd); };
            c.w64(0, t6);
            c.w64(10, 0x70008aa1cbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aa1cb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aa1cf); }
            return c.call(c.r64(0), 0x70008aa1d5, 0x70008aa1d3);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aa1d5); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aa1dd); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aa213); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aa21a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aa221);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aa224); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aa22b); }
            return c.ret(0x0, 0x70008aa22d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aa22e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aa232); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aa240, 0x70008aa23b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aa240); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa249); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa24c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aa25e, 0x70008aa259);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aa263); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aa26a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aa271); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aa276); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aa27d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aa28c, 0x70008aa287);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aa28e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa298); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aa29f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aa2a2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aa2a7); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aa2b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa2d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa2d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa2d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa2d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa2da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa2df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa2e9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa2f9); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aa318); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1300)) else { return RegfileExit::fault(0x70008aa31d); };
            c.w64(0, t6);
            c.w64(10, 0x70008aa32bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aa32b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aa32f); }
            return c.call(c.r64(0), 0x70008aa335, 0x70008aa333);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aa335); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aa33d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aa373); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aa37a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aa381);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aa384); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aa38b); }
            return c.ret(0x0, 0x70008aa38d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aa38e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aa392); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aa3a0, 0x70008aa39b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aa3a0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa3a9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa3ac); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aa3be, 0x70008aa3b9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aa3c3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aa3ca); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aa3d1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aa3d6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aa3dd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aa3ec, 0x70008aa3e7);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aa3ee); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa3f8); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aa3ff); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aa402); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aa407); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aa414);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa434); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa438); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa439); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa43a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa43f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa449); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa459); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aa46a); };
            c.w64(0, t4);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.movsd_rm(3, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa47a); }
            blk = 9; continue;
        }
        8 => {
            let r5 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r5);
            c.xorpd_rr(3, 3);
            c.cvtsi2sd(3, (c.r32(0) as u32 as i32 as i64));
            blk = 9; continue;
        }
        9 => {
            c.mov128_rr(0, 2);
            c.mov128_rr(1, 3);
            c.w64(10, c.r64(4));
            let r6 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r6);
            let r7 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r7);
            c.logic(r7, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aa49c); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1308)) else { return RegfileExit::fault(0x70008aa4a1); };
            c.w64(0, t8);
            c.w64(10, 0x70008aa4afu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aa4af); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aa4b3); }
            return c.call(c.r64(0), 0x70008aa4b9, 0x70008aa4b7);
        }
        10 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aa4b9); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aa4c1); };
            c.w64(4, t9);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 27; continue; }
            blk = 11; continue;
        }
        11 => {
            if c.cc_ne() { blk = 27; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 27; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(7, c.r64(1));
            let r10 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r10);
            if c.cc_o() { blk = 27; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 16; continue;
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aa503); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aa50a); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x18, 0x70008aa511);
        }
        18 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aa514); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aa51b); }
            return c.ret(0x0, 0x70008aa51d);
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008aa51e); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aa522); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aa530, 0x70008aa52b);
        }
        20 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008aa530); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        21 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa539); };
            c.w32(1, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008aa53c); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa540); };
            c.cmp(t15, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 22; continue;
        }
        22 => {
            return c.call(0x70008a1014, 0x70008aa552, 0x70008aa54d);
        }
        23 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008aa552); };
            c.w64(6, t16);
            blk = 2; continue;
        }
        24 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa55b); };
            c.w32(1, t17);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 2) { return RegfileExit::fault(0x70008aa55e); }
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa563); };
            c.cmp(t18, c.r32(1), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 25; continue;
        }
        25 => {
            return c.call(0x70008a1014, 0x70008aa575, 0x70008aa570);
        }
        26 => {
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aa575); }
            blk = 6; continue;
        }
        27 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aa57f); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aa586); };
            c.w64(7, t20);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aa58d); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aa592); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aa599); };
            c.cmp(t22, c.r64(8), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aa5a8, 0x70008aa5a3);
        }
        29 => {
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aa5aa); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 31; continue;
        }
        31 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa5b4); };
            c.w64(1, t23);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aa5bb); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aa5be); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aa5c3); }
            blk = 16; continue;
        }
        32 => {
            return RegfileExit::jump(0x70008aa5d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa5d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa5d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa5d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa5d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa5da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa5df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa5e9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa5f9); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aa618); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1310)) else { return RegfileExit::fault(0x70008aa61d); };
            c.w64(0, t6);
            c.w64(10, 0x70008aa62bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aa62b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aa62f); }
            return c.call(c.r64(0), 0x70008aa635, 0x70008aa633);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aa635); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aa63d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aa673); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aa67a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aa681);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aa684); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aa68b); }
            return c.ret(0x0, 0x70008aa68d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aa68e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aa692); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aa6a0, 0x70008aa69b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aa6a0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa6a9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa6ac); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aa6be, 0x70008aa6b9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aa6c3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aa6ca); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aa6d1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aa6d6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aa6dd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aa6ec, 0x70008aa6e7);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aa6ee); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa6f8); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aa6ff); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aa702); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aa707); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aa714);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa734); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa738); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa739); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa73a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa73f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa749); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa759); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aa778); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1318)) else { return RegfileExit::fault(0x70008aa77d); };
            c.w64(0, t6);
            c.w64(10, 0x70008aa78bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aa78b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aa78f); }
            return c.call(c.r64(0), 0x70008aa795, 0x70008aa793);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aa795); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aa79d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aa7d3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aa7da); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aa7e1);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aa7e4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aa7eb); }
            return c.ret(0x0, 0x70008aa7ed);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aa7ee); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aa7f2); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aa800, 0x70008aa7fb);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aa800); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa809); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa80c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aa81e, 0x70008aa819);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aa823); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aa82a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aa831); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aa836); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aa83d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aa84c, 0x70008aa847);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aa84e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa858); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aa85f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aa862); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aa867); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aa874);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa894); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa898); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa899); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa89a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa89f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa8a5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            blk = 6; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa8b5); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, c.r32(0));
            blk = 6; continue;
        }
        6 => {
            c.bsr(1, c.r32(0), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(1, 0x3fu64);
            blk = 8; continue;
        }
        8 => {
            let r4 = c.r32(1) ^ 0x1fu64;
            c.w32(1, r4);
            c.logic(r4, 32);
            c.w64(7, c.r64(1));
            let r5 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r5);
            if c.cc_o() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 10; continue;
        }
        10 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aa8e2); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aa8e9); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x10, 0x70008aa8f0);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aa8f3); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aa8fa); }
            return c.ret(0x0, 0x70008aa8fc);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aa8fd); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aa901); };
            c.w64(3, t7);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aa90f, 0x70008aa90a);
        }
        14 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aa90f); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aa915); };
            c.w32(1, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa918); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 16; continue;
        }
        16 => {
            return c.call(0x70008a1014, 0x70008aa926, 0x70008aa921);
        }
        17 => {
            blk = 2; continue;
        }
        18 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aa928); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008aa92f); };
            c.w64(8, t12);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008aa936); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aa93a); };
            c.w64(11, t13);
            let Some(t14) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008aa941); };
            c.cmp(t14, c.r64(9), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aa950, 0x70008aa94b);
        }
        20 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aa950); };
            c.w64(1, t15);
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008aa956); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 22; continue;
        }
        22 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aa960); };
            c.w64(7, t16);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008aa967); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aa972); }
            blk = 10; continue;
        }
        23 => {
            let r17 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r17);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008aa980); }
            return c.call(0x70008708b4, 0x70008aa98a, 0x70008aa985);
        }
        24 => {
            let Some(t18) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008aa98a); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r19);
            blk = 5; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aa998);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aa9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aa9b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aa9b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aa9b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aa9ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aa9bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aa9c9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aa9d9); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aa9f8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1320)) else { return RegfileExit::fault(0x70008aa9fd); };
            c.w64(0, t6);
            c.w64(10, 0x70008aaa0bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aaa0b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aaa0f); }
            return c.call(c.r64(0), 0x70008aaa15, 0x70008aaa13);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aaa15); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aaa1d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aaa53); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aaa5a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aaa61);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aaa64); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aaa6b); }
            return c.ret(0x0, 0x70008aaa6d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aaa6e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aaa72); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aaa80, 0x70008aaa7b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aaa80); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aaa89); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aaa8c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aaa9e, 0x70008aaa99);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aaaa3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aaaaa); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aaab1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aaab6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aaabd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aaacc, 0x70008aaac7);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aaace); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aaad8); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aaadf); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aaae2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aaae7); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aaaf4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aab14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aab14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aab18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aab19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aab1a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aab1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aab29); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aab39); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aab58); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1328)) else { return RegfileExit::fault(0x70008aab5d); };
            c.w64(0, t6);
            c.w64(10, 0x70008aab6bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aab6b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aab6f); }
            return c.call(c.r64(0), 0x70008aab75, 0x70008aab73);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aab75); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aab7d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aabb3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aabba); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aabc1);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aabc4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aabcb); }
            return c.ret(0x0, 0x70008aabcd);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aabce); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aabd2); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aabe0, 0x70008aabdb);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aabe0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aabe9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aabec); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aabfe, 0x70008aabf9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aac03); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aac0a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aac11); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aac16); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aac1d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aac2c, 0x70008aac27);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aac2e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aac38); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aac3f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aac42); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aac47); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aac54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aac74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aac74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aac78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aac79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aac7a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aac7f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aac89); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aac99); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aacb8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1330)) else { return RegfileExit::fault(0x70008aacbd); };
            c.w64(0, t6);
            c.w64(10, 0x70008aaccbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aaccb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aaccf); }
            return c.call(c.r64(0), 0x70008aacd5, 0x70008aacd3);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aacd5); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aacdd); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aad13); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aad1a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aad21);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aad24); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aad2b); }
            return c.ret(0x0, 0x70008aad2d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aad2e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aad32); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aad40, 0x70008aad3b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aad40); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aad49); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aad4c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aad5e, 0x70008aad59);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aad63); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aad6a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aad71); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aad76); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aad7d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aad8c, 0x70008aad87);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aad8e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aad98); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aad9f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aada2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aada7); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aadb4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aadd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aadd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aadd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aadd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aadda); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aaddf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aade9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aadf9); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008aae18); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1338)) else { return RegfileExit::fault(0x70008aae1d); };
            c.w64(0, t6);
            c.w64(10, 0x70008aae2bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008aae2b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008aae2f); }
            return c.call(c.r64(0), 0x70008aae35, 0x70008aae33);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008aae35); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008aae3d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aae73); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aae7a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aae81);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aae84); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aae8b); }
            return c.ret(0x0, 0x70008aae8d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aae8e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aae92); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aaea0, 0x70008aae9b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aaea0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aaea9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aaeac); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aaebe, 0x70008aaeb9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aaec3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aaeca); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aaed1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aaed6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aaedd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aaeec, 0x70008aaee7);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aaeee); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aaef8); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aaeff); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008aaf02); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aaf07); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008aaf14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aaf34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aaf34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008aaf38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aaf39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aaf3a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008aaf3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008aaf49); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aaf59); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.cvtsd2ss(0, c.xmm_u64(0));
            c.cvtss2sd(0, c.xmm_u32(0));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 19; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(7, c.r64(1));
            let r4 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r4);
            if c.cc_o() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 11; continue;
        }
        11 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aafa3); };
            c.w64(1, t5);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aafaa); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return c.ret(0x10, 0x70008aafb1);
        }
        13 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aafb4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aafbb); }
            return c.ret(0x0, 0x70008aafbd);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aafbe); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aafc2); };
            c.w64(3, t6);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aafd0, 0x70008aafcb);
        }
        15 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aafd0); };
            c.w64(6, t7);
            blk = 1; continue;
        }
        16 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aafd9); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aafdc); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.call(0x70008a1014, 0x70008aafee, 0x70008aafe9);
        }
        18 => {
            blk = 2; continue;
        }
        19 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aaff3); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aaffa); };
            c.w64(7, t11);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008ab001); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ab006); };
            c.w64(9, t12);
            let Some(t13) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ab00d); };
            c.cmp(t13, c.r64(8), 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ab01c, 0x70008ab017);
        }
        21 => {
            blk = 23; continue;
        }
        22 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ab01e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 23; continue;
        }
        23 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab028); };
            c.w64(1, t14);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ab02f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008ab032); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ab037); }
            blk = 11; continue;
        }
        24 => {
            return RegfileExit::jump(0x70008ab044);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab054); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab058); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab059); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab05a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab05f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab069); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            blk = 6; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab07d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 31; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, c.r32(0));
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ab093); };
            c.w64(1, t4);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let r5 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r5);
            blk = 11; continue;
        }
        9 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab0a9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 29; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(1, c.r32(1));
            blk = 11; continue;
        }
        11 => {
            let r6 = c.imul2(c.r32(1), c.r32(0), 32);
            c.w32(1, r6);
            c.w64(7, c.r64(1));
            let r7 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r7);
            if c.cc_o() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        13 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ab0d0); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ab0d7); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return c.ret(0x18, 0x70008ab0de);
        }
        15 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ab0e1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ab0e8); }
            return c.ret(0x0, 0x70008ab0ea);
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab0eb); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ab0ef); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ab0fd, 0x70008ab0f8);
        }
        17 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab0fd); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        18 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab106); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab109); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab10d); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 19; continue;
        }
        19 => {
            return c.call(0x70008a1014, 0x70008ab11f, 0x70008ab11a);
        }
        20 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab11f); };
            c.w64(6, t13);
            blk = 2; continue;
        }
        21 => {
            let Some(t14) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab128); };
            c.w32(7, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008ab12b); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab12f); };
            c.cmp(t15, c.r32(7), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008ab144, 0x70008ab13f);
        }
        23 => {
            c.w64(1, c.r64(0));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ab147); };
            c.w64(0, t16);
            blk = 7; continue;
        }
        24 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ab150); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ab157); };
            c.w64(8, t18);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008ab15e); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ab162); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ab169); };
            c.cmp(t20, c.r64(9), 64);
            if c.cc_a() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ab178, 0x70008ab173);
        }
        26 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab178); };
            c.w64(1, t21);
            blk = 28; continue;
        }
        27 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008ab17e); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 28; continue;
        }
        28 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab188); };
            c.w64(7, t22);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ab18f); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ab19a); }
            blk = 13; continue;
        }
        29 => {
            let r23 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r23);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008ab1a8); }
            return c.call(0x70008708b4, 0x70008ab1b2, 0x70008ab1ad);
        }
        30 => {
            let Some(t24) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008ab1b2); };
            c.w32(1, t24);
            let r25 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r25);
            blk = 10; continue;
        }
        31 => {
            let r26 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r26);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008ab1c2); }
            return c.call(0x70008708b4, 0x70008ab1cc, 0x70008ab1c7);
        }
        32 => {
            let Some(t27) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008ab1cc); };
            c.w32(0, t27);
            let r28 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r28);
            blk = 5; continue;
        }
        33 => {
            return RegfileExit::jump(0x70008ab1dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab1f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab1f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab1f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab1f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab1fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab1ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab209); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab219); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ab238); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1340)) else { return RegfileExit::fault(0x70008ab23d); };
            c.w64(0, t6);
            c.w64(10, 0x70008ab24bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ab24b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ab24f); }
            return c.call(c.r64(0), 0x70008ab255, 0x70008ab253);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ab255); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ab25d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ab293); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ab29a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008ab2a1);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ab2a4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ab2ab); }
            return c.ret(0x0, 0x70008ab2ad);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab2ae); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ab2b2); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ab2c0, 0x70008ab2bb);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab2c0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab2c9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab2cc); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008ab2de, 0x70008ab2d9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ab2e3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ab2ea); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008ab2f1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ab2f6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ab2fd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ab30c, 0x70008ab307);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ab30e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab318); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ab31f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008ab322); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ab327); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008ab334);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab354); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab358); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab359); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab35a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab35f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab369); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab379); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ab398); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1350)) else { return RegfileExit::fault(0x70008ab39d); };
            c.w64(0, t6);
            c.w64(10, 0x70008ab3abu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ab3ab); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ab3af); }
            return c.call(c.r64(0), 0x70008ab3b5, 0x70008ab3b3);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ab3b5); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ab3bd); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ab3f3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ab3fa); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008ab401);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ab404); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ab40b); }
            return c.ret(0x0, 0x70008ab40d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab40e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ab412); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ab420, 0x70008ab41b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab420); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab429); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab42c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008ab43e, 0x70008ab439);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ab443); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ab44a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008ab451); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ab456); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ab45d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ab46c, 0x70008ab467);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ab46e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab478); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ab47f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008ab482); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ab487); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008ab494);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab4b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab4b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab4b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab4b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab4ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab4bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab4c9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab4d9); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ab4f8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1348)) else { return RegfileExit::fault(0x70008ab4fd); };
            c.w64(0, t6);
            c.w64(10, 0x70008ab50bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ab50b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ab50f); }
            return c.call(c.r64(0), 0x70008ab515, 0x70008ab513);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ab515); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ab51d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ab553); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ab55a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008ab561);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ab564); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ab56b); }
            return c.ret(0x0, 0x70008ab56d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab56e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ab572); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ab580, 0x70008ab57b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab580); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab589); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab58c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008ab59e, 0x70008ab599);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ab5a3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ab5aa); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008ab5b1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ab5b6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ab5bd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ab5cc, 0x70008ab5c7);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ab5ce); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab5d8); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ab5df); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008ab5e2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ab5e7); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008ab5f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab614); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab618); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab619); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab61a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab61f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab629); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab639); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ab658); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1358)) else { return RegfileExit::fault(0x70008ab65d); };
            c.w64(0, t6);
            c.w64(10, 0x70008ab66bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ab66b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ab66f); }
            return c.call(c.r64(0), 0x70008ab675, 0x70008ab673);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ab675); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ab67d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ab6b3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ab6ba); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008ab6c1);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ab6c4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ab6cb); }
            return c.ret(0x0, 0x70008ab6cd);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab6ce); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ab6d2); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ab6e0, 0x70008ab6db);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab6e0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab6e9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab6ec); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008ab6fe, 0x70008ab6f9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ab703); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ab70a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008ab711); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ab716); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ab71d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ab72c, 0x70008ab727);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ab72e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab738); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ab73f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008ab742); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ab747); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008ab754);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab774); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab778); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab779); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab77a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab77f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab789); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab799); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ab7b8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1368)) else { return RegfileExit::fault(0x70008ab7bd); };
            c.w64(0, t6);
            c.w64(10, 0x70008ab7cbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ab7cb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ab7cf); }
            return c.call(c.r64(0), 0x70008ab7d5, 0x70008ab7d3);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ab7d5); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ab7dd); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ab813); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ab81a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008ab821);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ab824); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ab82b); }
            return c.ret(0x0, 0x70008ab82d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab82e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ab832); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ab840, 0x70008ab83b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab840); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab849); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab84c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008ab85e, 0x70008ab859);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ab863); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ab86a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008ab871); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ab876); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ab87d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ab88c, 0x70008ab887);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ab88e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab898); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ab89f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008ab8a2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ab8a7); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008ab8b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab8d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab8d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab8d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab8d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab8da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab8df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab8e5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab8f1); }
            blk = 5; continue;
        }
        4 => {
            c.w64(2, c.r64(0));
            let r3 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ab915); };
            c.w64(1, t4);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ab91c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x10, 0x70008ab923);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ab926); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ab92d); }
            return c.ret(0x0, 0x70008ab92f);
        }
        10 => {
            c.w32(0, 0x2u64);
            blk = 7; continue;
        }
        11 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 7; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ab940); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ab944); };
            c.w64(3, t5);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ab952, 0x70008ab94d);
        }
        13 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ab952); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        14 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ab958); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ab95b); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 15; continue;
        }
        15 => {
            return c.call(0x70008a1014, 0x70008ab969, 0x70008ab964);
        }
        16 => {
            blk = 2; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008ab96c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ab974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ab974); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ab978); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ab979); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ab97a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ab97f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ab989); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ab999); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ab9b8); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1370)) else { return RegfileExit::fault(0x70008ab9bd); };
            c.w64(0, t6);
            c.w64(10, 0x70008ab9cbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ab9cb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ab9cf); }
            return c.call(c.r64(0), 0x70008ab9d5, 0x70008ab9d3);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ab9d5); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ab9dd); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aba13); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aba1a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008aba21);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008aba24); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008aba2b); }
            return c.ret(0x0, 0x70008aba2d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aba2e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008aba32); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008aba40, 0x70008aba3b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aba40); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aba49); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aba4c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008aba5e, 0x70008aba59);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aba63); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aba6a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008aba71); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aba76); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008aba7d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aba8c, 0x70008aba87);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008aba8e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aba98); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008aba9f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008abaa2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008abaa7); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008abab4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008abad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008abad4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008abad8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008abad9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008abada); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008abadf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008abae9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008abaf9); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.sqrtsd(0, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 19; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(7, c.r64(1));
            let r4 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r4);
            if c.cc_o() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 11; continue;
        }
        11 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008abb3f); };
            c.w64(1, t5);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008abb46); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return c.ret(0x10, 0x70008abb4d);
        }
        13 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008abb50); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008abb57); }
            return c.ret(0x0, 0x70008abb59);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008abb5a); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008abb5e); };
            c.w64(3, t6);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008abb6c, 0x70008abb67);
        }
        15 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008abb6c); };
            c.w64(6, t7);
            blk = 1; continue;
        }
        16 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008abb75); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008abb78); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.call(0x70008a1014, 0x70008abb8a, 0x70008abb85);
        }
        18 => {
            blk = 2; continue;
        }
        19 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008abb8f); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008abb96); };
            c.w64(7, t11);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008abb9d); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008abba2); };
            c.w64(9, t12);
            let Some(t13) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008abba9); };
            c.cmp(t13, c.r64(8), 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008abbb8, 0x70008abbb3);
        }
        21 => {
            blk = 23; continue;
        }
        22 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008abbba); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 23; continue;
        }
        23 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008abbc4); };
            c.w64(1, t14);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008abbcb); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008abbce); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008abbd3); }
            blk = 11; continue;
        }
        24 => {
            return RegfileExit::jump(0x70008abbe0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008abbf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008abbf4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008abbf8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008abbf9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008abbfa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008abbff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008abc09); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008abc19); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008abc38); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1378)) else { return RegfileExit::fault(0x70008abc3d); };
            c.w64(0, t6);
            c.w64(10, 0x70008abc4bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008abc4b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008abc4f); }
            return c.call(c.r64(0), 0x70008abc55, 0x70008abc53);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008abc55); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008abc5d); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008abc93); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008abc9a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008abca1);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008abca4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008abcab); }
            return c.ret(0x0, 0x70008abcad);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008abcae); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008abcb2); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008abcc0, 0x70008abcbb);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008abcc0); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008abcc9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008abccc); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008abcde, 0x70008abcd9);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008abce3); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008abcea); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008abcf1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008abcf6); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008abcfd); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008abd0c, 0x70008abd07);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008abd0e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008abd18); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008abd1f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008abd22); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008abd27); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008abd34);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008abd54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008abd54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008abd58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008abd59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008abd5a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008abd5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008abd69); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008abd79); }
            blk = 5; continue;
        }
        4 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 5; continue;
        }
        5 => {
            c.mov128_rr(0, 1);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008abd98); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1380)) else { return RegfileExit::fault(0x70008abd9d); };
            c.w64(0, t6);
            c.w64(10, 0x70008abdabu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008abdab); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008abdaf); }
            return c.call(c.r64(0), 0x70008abdb5, 0x70008abdb3);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008abdb5); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008abdbd); };
            c.w64(4, t7);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008abdf3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008abdfa); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x10, 0x70008abe01);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008abe04); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008abe0b); }
            return c.ret(0x0, 0x70008abe0d);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008abe0e); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008abe12); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008abe20, 0x70008abe1b);
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008abe20); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008abe29); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008abe2c); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.call(0x70008a1014, 0x70008abe3e, 0x70008abe39);
        }
        19 => {
            blk = 2; continue;
        }
        20 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008abe43); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008abe4a); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008abe51); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008abe56); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008abe5d); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008abe6c, 0x70008abe67);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008abe6e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        24 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008abe78); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008abe7f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008abe82); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008abe87); }
            blk = 12; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008abe94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008abeb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008abeb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008abeb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008abeb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008abeba); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008abebf); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008abec3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008abec7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(5));
            let r2 = c.add(c.r64(9), 0x18u64, 64);
            c.w64(9, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008abedb); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008abedf); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(7), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 63; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(8, c.r64(7).wrapping_mul(8));
            c.w64(11, c.r64(8).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008abf06); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008abf17); };
            c.w64(12, t3);
            let Some(t4) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008abf1e); };
            c.w64(15, t4);
            let Some(t5) = c.ld64(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008abf22); };
            c.w64(3, t5);
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let r6 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r6);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008abf33); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008abf3b); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008abf3d); };
            c.w64(3, t7);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008abf53, 0x70008abf4e);
        }
        6 => {
            blk = 9; continue;
        }
        7 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(15)));
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x70008abf62); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008abf6d); };
            c.w64(7, t8);
            let r9 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008abf73); };
            c.w64(9, t10);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008abf7a); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008abf7e); }
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r11 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            let r12 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r12);
            c.logic(r12, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008abf90); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008abf95); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008abf9b); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008abf9f); };
            c.w64(0, t14);
            c.w64(10, 0x70008abfadu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008abfad); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008abfb1); }
            return c.call(c.r64(0), 0x70008abfb7, 0x70008abfb5);
        }
        10 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008abfb7); }
            let Some(t15) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008abfbf); };
            c.w64(4, t15);
            c.xorpd_rr(0, 0);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008abfc8); };
            c.w64(11, t16);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008abfd1); };
            c.w64(9, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008abfd5); };
            c.w64(0, t18);
            c.mov128_rr(1, 0);
            blk = 12; continue;
        }
        11 => {
            let r19 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r19);
            c.w64(15, c.r64(12));
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ge() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld64(c.r64(11).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008abff3); };
            c.w64(12, t20);
            blk = 16; continue;
        }
        15 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008abff9); };
            c.w64(12, t21);
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(2, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac010); }
            blk = 20; continue;
        }
        19 => {
            let r22 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r22);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(12) as u32 as i32 as i64));
            blk = 20; continue;
        }
        20 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(2));
            if c.cc_p() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(12, 0x1u64);
            blk = 11; continue;
        }
        23 => {
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ac035); };
            c.w32(12, t23);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ae() { blk = 47; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x11c8)) else { return RegfileExit::fault(0x70008ac048); };
            c.w64(10, t24);
            if !c.and128_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008ac04f); }
            c.w64(12, c.r64(7).wrapping_mul(8));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_be() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.mov128_rr(1, 2);
            blk = 26; continue;
        }
        26 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 2, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0x7), 2) { return RegfileExit::fault(0x70008ac06e); }
            c.w64(12, c.r64(15));
            blk = 11; continue;
        }
        27 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x35);
            c.psrlq(2, 0x1);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            if c.cc_e() { blk = 46; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 30; continue;
        }
        30 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            if c.cc_e() { blk = 44; continue; }
            blk = 32; continue;
        }
        32 => {
            c.mov128_rr(2, 0);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        33 => {
            if !c.movsd_rm(3, c.r64(0).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x70008ac0b5); }
            c.arith_sd(3, 3, c.xmm_u64(1));
            c.mov128_rr(3, 3);
            let r25 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r25);
            c.mov128_rr(4, 3);
            c.arith_sd(2, 4, c.xmm_u64(3));
            c.arith_sd(1, 4, c.xmm_u64(2));
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(4));
            c.mov128_rr(3, 2);
            c.arith_sd(1, 3, c.xmm_u64(0));
            c.arith_sd(1, 3, c.xmm_u64(4));
            c.mov128_rr(0, 2);
            c.mov128_rr(2, 3);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ge() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ac0ef); };
            c.w32(7, t26);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_b() { blk = 33; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 43; continue;
        }
        37 => {
            c.sqrtsd(0, c.xmm_u64(0));
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.arith_sd(2, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(9, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            if c.cc_ne() { blk = 58; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 58; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, c.r64(9));
            let r27 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r27);
            if c.cc_o() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac154); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac155); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac15b); }
            return c.ret(0x0, 0x70008ac15d);
        }
        43 => {
            return RegfileExit::fault(0x70008ac15e);
        }
        44 => {
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac168); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac169); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac16f); }
            return c.ret(0x0, 0x70008ac171);
        }
        45 => {
            c.w64(7, c.r64(9).wrapping_add(0x1));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008ac176); };
            c.w64(0, t28);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac180); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac181); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac187); }
            return c.ret(0x0, 0x70008ac189);
        }
        46 => {
            c.w64(7, c.r64(9).wrapping_add(0x1));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ac18e); };
            c.w64(0, t29);
            let Some(t30) = c.ld32(c.r64(0).wrapping_add(0x20f)) else { return RegfileExit::fault(0x70008ac195); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac1a1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac1a2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac1a8); }
            return c.ret(0x0, 0x70008ac1aa);
        }
        47 => {
            return RegfileExit::fault(0x70008ac1ab);
        }
        48 => {
            let r32 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac1b5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac1b6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac1bc); }
            return c.ret(0x0, 0x70008ac1be);
        }
        49 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ac1bf); };
            c.w64(3, t33);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008ac1d0, 0x70008ac1cb);
        }
        50 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ac1d0); };
            c.w64(6, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ac1d4); };
            c.w64(0, t35);
            blk = 1; continue;
        }
        51 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008ac1e1); };
            c.w64(3, t36);
            c.w64(12, c.r64(6));
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x70008ac1f3, 0x70008ac1ee);
        }
        52 => {
            return RegfileExit::fault(0x70008ac1f3);
        }
        53 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008ac1f7); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008ac1ff); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008ac201); };
            c.w64(3, t38);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008ac217, 0x70008ac212);
        }
        54 => {
            blk = 9; continue;
        }
        55 => {
            let Some(t39) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ac21c); };
            c.w32(3, t39);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x70008ac221); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008ac226); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008ac22a); }
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ac22e); };
            c.cmp(t40, c.r32(3), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(0, c.r64(12));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ac23e); };
            c.w64(6, t41);
            return c.call(0x70008a1014, 0x70008ac247, 0x70008ac242);
        }
        57 => {
            c.w64(12, c.r64(0));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ac24a); };
            c.w64(9, t42);
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008ac24e); }
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ac253); };
            c.w64(0, t43);
            c.xorpd_rr(0, 0);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ac25b); };
            c.w64(15, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ac25f); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ac263); };
            c.w64(11, t46);
            blk = 17; continue;
        }
        58 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ac26c); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ac273); };
            c.w64(11, t48);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008ac27a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x70008ac27e); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ac283); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008ac28a); };
            c.cmp(t50, c.r64(12), 64);
            if c.cc_a() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ac299, 0x70008ac294);
        }
        60 => {
            blk = 62; continue;
        }
        61 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008ac29b); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 62; continue;
        }
        62 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ac2a5); };
            c.w64(7, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ac2ac); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008ac2af); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008ac2b4); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ac2b9); };
            c.w64(7, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac2c0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac2c1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac2c7); }
            return c.ret(0x0, 0x70008ac2c9);
        }
        63 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008ac2ca); };
            c.w64(3, t53);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ac2da, 0x70008ac2d5);
        }
        64 => {
            return RegfileExit::fault(0x70008ac2da);
        }
        65 => {
            return RegfileExit::jump(0x70008ac2dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ac2f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ac2f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ac2f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ac2f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ac2fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ac2ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ac303); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ac30d); };
            c.w32(7, t2);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x86u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008ac31f);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0x21f)) else { return RegfileExit::fault(0x70008ac320); };
            c.w32(7, t3);
            c.w32(8, c.r32(7));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(7));
            blk = 7; continue;
        }
        5 => {
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ac33f); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(6));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1428)) else { return RegfileExit::fault(0x70008ac34a); };
            c.w64(0, t6);
            c.w64(10, 0x70008ac358u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ac358); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ac35c); }
            return c.call(c.r64(0), 0x70008ac362, 0x70008ac360);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ac362); }
            let Some(t7) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ac36a); };
            c.w64(4, t7);
            blk = 7; continue;
        }
        7 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ac36f); };
            c.w64(7, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ac373); };
            c.w32(8, t9);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x86u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008ac386);
        }
        9 => {
            c.w32(9, c.r64(0).wrapping_sub(0x2));
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(0x21f), c.r32(11)) { return RegfileExit::fault(0x70008ac38e); }
            c.cmp(c.r64(8), 0x88u64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008ac39e);
        }
        11 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x227)) else { return RegfileExit::fault(0x70008ac39f); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ac3a8); };
            c.w32(8, t12);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008ac3b1);
        }
        13 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x70008ac3b2); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ac3b9); };
            c.w64(7, t13);
            let Some(t14) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ac3c0); };
            c.w64(8, t14);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ac3c7); };
            c.w64(11, t15);
            let Some(t16) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ac3ce); };
            c.cmp(t16, c.r64(9), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008ac3d3); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ac3da); };
            c.w64(8, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008ac3e1); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ac3e5); }
            c.w64(0, c.r64(7));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ac3ed); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac3f4); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return c.ret(0x8, 0x70008ac3fb);
        }
        17 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac3fe); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac405); }
            return c.ret(0x0, 0x70008ac407);
        }
        18 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ac408); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ac416, 0x70008ac411);
        }
        19 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ac416); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        20 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008ac424); }
            return c.call(0x70007febf4, 0x70008ac42e, 0x70008ac429);
        }
        21 => {
            c.w64(7, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008ac431); }
            blk = 15; continue;
        }
        22 => {
            return RegfileExit::jump(0x70008ac43c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ac454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ac454); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ac458); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ac459); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ac45a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ac45f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ac463); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 149; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ac477); };
            c.w64(7, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ac47e); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008ac490); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ac49a); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ac4a1); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ac4aa); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ac4ba); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ac4c4); };
            c.w64(1, t9);
            blk = 9; continue;
        }
        8 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ac4ca); };
            c.w64(1, t10);
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ac4d1); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            blk = 15; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008ac4e4); }
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008ac4f0, 0x70008ac4eb);
        }
        13 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ac4f0); };
            c.w64(8, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ac4f4); };
            c.w64(6, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ac4f8); };
            c.w64(9, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ac4fc); };
            c.w64(7, t15);
            blk = 15; continue;
        }
        14 => {
            c.w32(0, 0x14u64);
            c.w64(1, c.r64(0));
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac517); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x3f);
            c.psrlq(1, 0x1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 146; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 20; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x4u64, 32);
            if c.cc_l() { blk = 146; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(12, 0x1u64);
            blk = 20; continue;
        }
        20 => {
            c.w32(11, 0x1u64);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac554); }
            c.w64(10, 0x4042000000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            c.cmp(c.r32(0), 0x48u64, 32);
            if c.cc_g() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        25 => {
            c.w64(12, c.r64(11));
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac58c); }
            c.w64(10, 0x4024000000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 32; continue; }
            blk = 29; continue;
        }
        29 => {
            if c.cc_e() { blk = 100; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.cmp(c.r32(0), 0x14u64, 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac5c1); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.xorpd_rr(1, 1);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008ac5ce); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            if c.cc_e() { blk = 53; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac5e4); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 52; continue; }
            blk = 37; continue;
        }
        37 => {
            if c.cc_ne() { blk = 52; continue; }
            blk = 38; continue;
        }
        38 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac5f9); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ac5fe); };
            c.w64(8, t16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x213)) else { return RegfileExit::fault(0x70008ac605); };
            c.w32(8, t17);
            let r18 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r18);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac615); }
            blk = 41; continue;
        }
        40 => {
            let r19 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r19);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 41; continue;
        }
        41 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac631); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ac636); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x217)) else { return RegfileExit::fault(0x70008ac63d); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac64d); }
            blk = 46; continue;
        }
        45 => {
            let r23 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r23);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 46; continue;
        }
        46 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            if c.cc_e() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ac669); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ac66a); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2110)) else { return RegfileExit::fault(0x70008ac66b); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ac67c, 0x70008ac677);
        }
        49 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ac67c); };
            c.w64(1, t25);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac683); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac684); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac68a); }
            return c.ret(0x0, 0x70008ac68c);
        }
        50 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x878)) else { return RegfileExit::fault(0x70008ac68d); };
            c.w64(0, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac697); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac698); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac69e); }
            return c.ret(0x0, 0x70008ac6a0);
        }
        51 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x7c0)) else { return RegfileExit::fault(0x70008ac6a1); };
            c.w64(0, t27);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac6ab); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac6ac); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac6b2); }
            return c.ret(0x0, 0x70008ac6b4);
        }
        52 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x898)) else { return RegfileExit::fault(0x70008ac6b5); };
            c.w64(0, t28);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac6bf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac6c0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac6c6); }
            return c.ret(0x0, 0x70008ac6c8);
        }
        53 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xb90)) else { return RegfileExit::fault(0x70008ac6c9); };
            c.w64(0, t29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac6d3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac6d4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac6da); }
            return c.ret(0x0, 0x70008ac6dc);
        }
        54 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(8, c.r64(7));
            let r30 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r30);
            blk = 58; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac6eb); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_o() { blk = 205; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w32(8, c.r32(8));
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(1, c.r64(0));
            let r31 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r31);
            blk = 62; continue;
        }
        60 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac70e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 203; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w32(1, c.r32(1));
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            c.w8(12, (c.cc_l() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(15, 0x80000000u64);
            c.cmp(c.r32(15), c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(7, c.r64(8));
            let r32 = c.neg(c.r32(7), 32);
            c.w32(7, r32);
            blk = 69; continue;
        }
        65 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ac748); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ac749); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2110)) else { return RegfileExit::fault(0x70008ac74a); };
            c.w64(3, t33);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ac75b, 0x70008ac756);
        }
        66 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ac75b); };
            c.w64(9, t34);
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac766); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac767); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac76d); }
            return c.ret(0x0, 0x70008ac76f);
        }
        67 => {
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_l() { blk = 94; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(7, c.r64(8));
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 71; continue;
        }
        71 => {
            c.w64(15, c.r64(7));
            blk = 73; continue;
        }
        72 => {
            c.w64(0, c.r64(15));
            c.cdq();
            c.idiv(c.r32(1), 32);
            let r35 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r35);
            c.w64(15, c.r64(0));
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_g() { blk = 72; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(0, 0xcu64);
            let r36 = c.add(c.r64(0), c.r64(15), 64);
            c.w64(0, r36);
            if c.cc_o() { blk = 93; continue; }
            blk = 75; continue;
        }
        75 => {
            let r37 = c.add(c.r64(0), 0x3u64, 64);
            c.w64(0, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ac7b2); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ac7b9); };
            c.w64(3, t39);
            let r40 = c.r64(0) & 0xfffffffffffffffcu64;
            c.w64(0, r40);
            c.logic(r40, 64);
            c.w64(2, c.r64(3).wrapping_add(c.r64(0)));
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ac7c4); };
            c.w64(6, t41);
            let Some(t42) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008ac7cb); };
            c.cmp(t42, c.r64(2), 64);
            if c.cc_be() { blk = 153; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 153; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ac7e0); };
            c.w64(0, t43);
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008ac7e7); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 78; continue;
        }
        78 => {
            c.w64(3, c.r64(0));
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008ac7f1); };
            c.w64(0, t44);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008ac7f8); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008ac7fb); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008ac802); }
            let r45 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r45);
            blk = 80; continue;
        }
        79 => {
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(7));
            c.cdq();
            c.idiv(c.r32(1), 32);
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0xb), c.r8(6)) { return RegfileExit::fault(0x70008ac815); }
            let r46 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r46);
            c.w64(7, c.r64(0));
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_le() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t47) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ac826); };
            c.w64(0, (t47 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ae() { blk = 92; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w64(0, c.r64(7));
            c.cdq();
            c.idiv(c.r32(1), 32);
            c.cmp(c.r32(2), 0xau64, 32);
            if c.cc_l() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w32(0, c.r64(2).wrapping_add(0x57));
            blk = 79; continue;
        }
        84 => {
            c.w32(0, c.r64(2).wrapping_add(0x30));
            blk = 79; continue;
        }
        85 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(1), 0xau64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 87; continue;
        }
        87 => {
            let r48 = c.shl(c.r32(11), 0x1au64, 32).unwrap_or(c.r32(11));
            c.w32(11, r48);
            c.w32(1, c.r64(8).wrapping_mul(4));
            let r49 = c.r32(11) | c.r32(1);
            c.w32(11, r49);
            c.logic(r49, 32);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008ac85e); }
            blk = 91; continue;
        }
        88 => {
            let Some(t50) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ac864); };
            c.w64(1, (t50 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            return RegfileExit::fault(0x70008ac86e);
        }
        90 => {
            if !c.st8(c.r64(3).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x70008ac86f); }
            blk = 91; continue;
        }
        91 => {
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac87d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac87e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac884); }
            return c.ret(0x0, 0x70008ac886);
        }
        92 => {
            return RegfileExit::fault(0x70008ac887);
        }
        93 => {
            return RegfileExit::fault(0x70008ac888);
        }
        94 => {
            c.cmp(c.r32(8), 0xau64, 32);
            if c.cc_l() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(1, c.r64(8).wrapping_add(0x57));
            blk = 97; continue;
        }
        96 => {
            c.w32(1, c.r64(8).wrapping_add(0x30));
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 155; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008ac8ad); };
            c.w64(8, t51);
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ac8b4); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ac8bc); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac8d0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac8d1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac8d7); }
            return c.ret(0x0, 0x70008ac8d9);
        }
        100 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x70008ac8da); };
            c.w64(1, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ac8e1); };
            c.w32(12, t56);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(12));
            let r57 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r57);
            let r58 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r58);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008ac8f5); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ac8ff); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 169; continue; }
            blk = 102; continue;
        }
        102 => {
            if c.cc_ne() { blk = 169; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.movq_rx(0, 0);
            if let Some(sr) = c.shr(c.r64(0), 0x20u64, 64) { c.w64(0, sr); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 169; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w64(0, c.r64(8));
            let r59 = c.add(c.r32(0), c.r32(8), 32);
            c.w32(0, r59);
            if c.cc_o() { blk = 169; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 108; continue;
        }
        107 => {
            c.w64(8, c.r64(7));
            blk = 108; continue;
        }
        108 => {
            c.w64(0, c.r64(8));
            let r60 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r60);
            c.w32(3, c.r32(8));
            let r61 = c.r32(15) & c.r32(0);
            c.w32(15, r61);
            c.logic(r61, 32);
            let r62 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r62);
            c.w64(6, c.r64(15).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008ac964); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008ac968); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008ac96c); }
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ac970); };
            c.cmp(t63, c.r32(8), 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t64) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ac977); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ac985); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ac986); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ac98c); }
            return c.ret(0x0, 0x70008ac98e);
        }
        110 => {
            c.cmp(c.r64(12), 0x8000u64, 64);
            if c.cc_l() { blk = 192; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            c.w8(12, (c.cc_l() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(2, 0x80000000u64);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 113; continue;
        }
        113 => {
            let r66 = c.neg(c.r32(0), 32);
            c.w32(0, r66);
            blk = 115; continue;
        }
        114 => {
            c.cmp(c.r32(0), 0xau64, 32);
            if c.cc_l() { blk = 139; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(7, c.r64(0));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 117; continue;
        }
        117 => {
            c.w64(0, c.r64(7));
            c.w32(2, 0x66666667u64);
            blk = 119; continue;
        }
        118 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008ac9dd); }
            c.w64(8, c.r64(2));
            c.imul1(c.r32(8), 32);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ac9e7); };
            c.w64(0, t67);
            let r68 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r68);
            let r69 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r69);
            let r70 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r70);
            let r71 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r71);
            c.w64(2, c.r64(8));
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_g() { blk = 118; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(2));
            c.w32(2, 0xcu64);
            let r72 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r72);
            if c.cc_o() { blk = 138; continue; }
            blk = 121; continue;
        }
        121 => {
            let r73 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aca17); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008aca1e); };
            c.w64(6, t75);
            let r76 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r76);
            c.logic(r76, 64);
            c.w64(15, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aca29); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008aca30); };
            c.cmp(t78, c.r64(15), 64);
            if c.cc_be() { blk = 174; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 174; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aca46); };
            c.w64(1, t79);
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008aca4d); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008aca54); };
            c.w64(15, t80);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008aca5b); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008aca5f); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008aca66); }
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            blk = 126; continue;
        }
        125 => {
            if !c.st8(c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x70008aca70); }
            let r81 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r81);
            c.w64(7, c.r64(0));
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_le() { blk = 131; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aca81); };
            c.w64(0, (t82 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ae() { blk = 137; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(0, c.r64(8));
            c.imul1(c.r32(7), 32);
            c.w64(0, c.r64(7));
            let r83 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r83);
            let r84 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r84);
            let r85 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r85);
            let r86 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(2, r86);
            let r87 = c.sub(c.r32(7), c.r32(2), 32);
            c.w32(7, r87);
            c.cmp(c.r32(7), 0xau64, 32);
            if c.cc_l() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            let r88 = c.add(c.r32(7), 0x57u64, 32);
            c.w32(7, r88);
            blk = 125; continue;
        }
        130 => {
            let r89 = c.add(c.r32(7), 0x30u64, 32);
            c.w32(7, r89);
            blk = 125; continue;
        }
        131 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            let r90 = c.shl(c.r32(11), 0x1au64, 32).unwrap_or(c.r32(11));
            c.w32(11, r90);
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let r91 = c.r32(11) | c.r32(7);
            c.w32(11, r91);
            c.logic(r91, 32);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008acabe); }
            blk = 136; continue;
        }
        133 => {
            let Some(t92) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008acac4); };
            c.w64(7, (t92 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            return RegfileExit::fault(0x70008acace);
        }
        135 => {
            if !c.st8(c.r64(1).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x70008acacf); }
            blk = 136; continue;
        }
        136 => {
            c.w64(0, c.r64(1));
            blk = 141; continue;
        }
        137 => {
            return RegfileExit::fault(0x70008acad8);
        }
        138 => {
            return RegfileExit::fault(0x70008acad9);
        }
        139 => {
            c.w32(7, c.r64(0).wrapping_add(0x30));
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 176; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008acaf1); };
            c.w64(12, t93);
            let Some(t94) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008acaf8); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r95);
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008acb00); };
            c.cmp(t96, c.r32(0), 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008acb0d); };
            c.w64(1, t97);
            let Some(t98) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008acb11); };
            c.w32(7, t98);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008acb1a); };
            c.w64(8, t99);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 190; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008acb27); };
            c.w64(7, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008acb2b); };
            c.w64(11, t101);
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008acb2f); }
            let Some(t102) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008acb34); };
            c.w32(7, t102);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r103 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r103);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 191; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(7, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008acb52); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r104 = c.r64(8) & c.r64(1);
            c.w64(8, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008acb63); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 200; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(1, c.r64(9));
            let r106 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r106);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008acb78); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008acb79); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008acb7f); }
            return c.ret(0x0, 0x70008acb81);
        }
        146 => {
            c.w32(11, 0x1u64);
            blk = 147; continue;
        }
        147 => {
            c.w32(1, 0x1ceu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008acb8d); }
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008acb8e); };
            c.w64(3, t107);
            c.w64(0, c.r64(11));
            return c.call(0x700086ed74, 0x70008acb9d, 0x70008acb98);
        }
        148 => {
            return RegfileExit::fault(0x70008acb9d);
        }
        149 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008acb9e); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008acba4); };
            c.w64(3, t108);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008acbb3, 0x70008acbae);
        }
        150 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008acbb3); };
            c.w64(6, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008acbb7); };
            c.w64(0, t110);
            blk = 1; continue;
        }
        151 => {
            c.w32(1, 0xc6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008acbc5); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008acbc6); };
            c.w64(1, t111);
            let Some(t112) = c.ld32(c.r64(1).wrapping_add(0x21b)) else { return RegfileExit::fault(0x70008acbcd); };
            c.w32(1, t112);
            let r113 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r113);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008acbd6); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x8f8)) else { return RegfileExit::fault(0x70008acbd7); };
            c.w64(1, t114);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008acbde); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008acbdf); };
            c.w64(3, t115);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008acbf0, 0x70008acbeb);
        }
        152 => {
            return RegfileExit::fault(0x70008acbf0);
        }
        153 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008acbf1); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008acbf5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008acbf9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008acbfd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008acc01); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008acc05); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70008acc11, 0x70008acc0c);
        }
        154 => {
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008acc11); };
            c.w64(15, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008acc15); };
            c.w64(9, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008acc19); };
            c.w64(8, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008acc1d); };
            c.w64(11, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008acc21); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008acc25); };
            c.w64(12, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008acc29); };
            c.w64(7, t122);
            blk = 78; continue;
        }
        155 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008acc32); };
            c.w64(7, t123);
            let Some(t124) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008acc39); };
            c.w64(8, t124);
            c.w64(11, c.r64(8).wrapping_add(0x10));
            let r125 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r125);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008acc44); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008acc48); };
            c.w64(12, t126);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008acc4f); }
            let Some(t127) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008acc53); };
            c.cmp(t127, c.r64(11), 64);
            if c.cc_a() { blk = 158; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008acc63, 0x70008acc5e);
        }
        157 => {
            blk = 159; continue;
        }
        158 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008acc65); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 159; continue;
        }
        159 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008acc6f); };
            c.w64(1, t128);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008acc76); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008acc79); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008acc80); }
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008acc87); };
            c.w64(1, t129);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70008acc8b); }
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008acc8f); };
            c.w64(1, t130);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008acc96); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008acc97); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008acc9d); }
            return c.ret(0x0, 0x70008acc9f);
        }
        160 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008acca0); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008acca7); };
            c.w64(12, t132);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008accaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008accb3); }
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008accb7); };
            c.w64(0, t133);
            let Some(t134) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008accbe); };
            c.cmp(t134, c.r64(15), 64);
            if c.cc_a() { blk = 163; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008acccd, 0x70008accc8);
        }
        162 => {
            blk = 164; continue;
        }
        163 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008acccf); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 164; continue;
        }
        164 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008accda); };
            c.w64(1, t135);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008acce1); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008acce4); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008acceb); }
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008accf2); };
            c.w64(1, t136);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008accf6); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008accf9); };
            c.w64(8, t137);
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008acd00); };
            c.w32(7, t138);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            return RegfileExit::fault(0x70008acd0f);
        }
        166 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008acd10); };
            c.w64(7, t139);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008acd14); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r140 = c.r64(1) & c.r64(8);
            c.w64(1, r140);
            c.logic(r140, 64);
            let Some(t141) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008acd26); };
            c.logic(t141 & 0x4u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008acd30); };
            c.w64(9, t142);
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008acd3b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008acd3c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008acd42); }
            return c.ret(0x0, 0x70008acd44);
        }
        169 => {
            let Some(t143) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008acd45); };
            c.w32(8, t143);
            let Some(t144) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008acd49); };
            c.w32(12, t144);
            c.w64(0, c.r64(8));
            let r145 = c.r32(0) ^ c.r32(12);
            c.w32(0, r145);
            c.logic(r145, 32);
            let r146 = c.r32(0) & c.r32(15);
            c.w32(0, r146);
            c.logic(r146, 32);
            c.w32(15, c.r64(0).wrapping_add(c.r64(0)));
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let Some(t147) = c.ld32(c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008acd5e); };
            c.w32(0, t147);
            let r148 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r148);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 192; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t149) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008acd6e); };
            c.w32(3, t149);
            let Some(t150) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008acd71); };
            c.cmp(t150, c.r32(3), 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t151) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008acd7e); };
            c.w32(3, t151);
            let Some(t152) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008acd81); };
            c.cmp(t152, c.r32(8), 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t153) = c.ld32(c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008acd94); };
            c.w32(0, t153);
            let r154 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r154);
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008acda3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008acda4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008acdaa); }
            return c.ret(0x0, 0x70008acdac);
        }
        174 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008acdad); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008acdb1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008acdb5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008acdb9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008acdbd); }
            return c.call(0x70007febb4, 0x70008acdc6, 0x70008acdc1);
        }
        175 => {
            c.w64(1, c.r64(0));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008acdc9); };
            c.w64(9, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008acdcd); };
            c.w64(3, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008acdd1); };
            c.w64(11, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008acdd5); };
            c.w64(12, t158);
            c.w32(8, 0x66666667u64);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008acddf); };
            c.w64(7, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008acde3); };
            c.w64(0, t160);
            blk = 124; continue;
        }
        176 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008acdec); };
            c.w64(11, t161);
            let Some(t162) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008acdf3); };
            c.w64(12, t162);
            c.w64(0, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008acdfb); }
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008acdff); };
            c.w64(3, t163);
            let Some(t164) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ace06); };
            c.cmp(t164, c.r64(0), 64);
            if c.cc_a() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008ace15, 0x70008ace10);
        }
        178 => {
            blk = 180; continue;
        }
        179 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008ace17); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 180; continue;
        }
        180 => {
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008ace22); };
            c.w64(1, t165);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ace29); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008ace2c); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008ace33); }
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ace3a); };
            c.w64(7, t166);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008ace3e); }
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ace42); };
            c.w64(15, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ace46); };
            c.w64(9, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ace4a); };
            c.w64(1, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ace4e); };
            c.w64(6, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ace52); };
            c.w64(8, t171);
            blk = 141; continue;
        }
        181 => {
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ace5b); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008ace62); };
            c.w64(3, t173);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008ace69); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008ace6d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008ace71); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ace75); };
            c.w64(3, t174);
            let Some(t175) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ace7c); };
            c.cmp(t175, c.r64(2), 64);
            if c.cc_a() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008ace8b, 0x70008ace86);
        }
        183 => {
            blk = 185; continue;
        }
        184 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008ace8d); }
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ace90); };
            c.w64(0, t176);
            let r177 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r177);
            blk = 185; continue;
        }
        185 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008ace98); };
            c.w64(1, t178);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ace9f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008acea2); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008acea9); }
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008aceb0); };
            c.w64(1, t179);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008aceb4); }
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008aceb7); };
            c.w64(12, t180);
            let Some(t181) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008acebe); };
            c.w32(7, t181);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            return RegfileExit::fault(0x70008acece);
        }
        187 => {
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008acecf); };
            c.w64(11, t182);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008aced3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r183 = c.r64(1) & c.r64(12);
            c.w64(1, r183);
            c.logic(r183, 64);
            let Some(t184) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008acee5); };
            c.logic(t184 & 0x4u64, 8);
            if c.cc_ne() { blk = 194; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008aceeb); };
            c.w64(15, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008aceef); };
            c.w64(9, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008acef3); };
            c.w64(1, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008acef7); };
            c.w64(6, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008acefb); };
            c.w64(8, t189);
            blk = 141; continue;
        }
        190 => {
            return RegfileExit::fault(0x70008acf04);
        }
        191 => {
            return RegfileExit::fault(0x70008acf05);
        }
        192 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008acf06); }
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x70008acf07); };
            c.w64(3, t190);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(11));
            return c.call(0x700086ed74, 0x70008acf18, 0x70008acf13);
        }
        193 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008acf18); };
            c.w64(1, t191);
            let r192 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r192);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008acf23); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008acf24); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008acf2a); }
            return c.ret(0x0, 0x70008acf2c);
        }
        194 => {
            c.w32(0, c.r32(0));
            let r193 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r193);
            c.w64(1, 0xfffffffffffc0000u64);
            let r194 = c.r64(1) & c.r64(0);
            c.w64(1, r194);
            c.logic(r194, 64);
            let Some(t195) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008acf3c); };
            c.logic(t195 & 0x2u64, 8);
            if c.cc_e() { blk = 189; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008acf46); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008acf47); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x70008acf54, 0x70008acf4f);
        }
        196 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008acf54); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008acf56); }
            blk = 189; continue;
        }
        197 => {
            c.w32(0, c.r32(0));
            let r196 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r196);
            c.w64(1, 0xfffffffffffc0000u64);
            let r197 = c.r64(1) & c.r64(0);
            c.w64(1, r197);
            c.logic(r197, 64);
            let Some(t198) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008acf68); };
            c.logic(t198 & 0x2u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008acf76); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008acf77); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008acf84, 0x70008acf7f);
        }
        199 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008acf84); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008acf86); }
            blk = 168; continue;
        }
        200 => {
            c.w32(0, c.r32(0));
            let r199 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r199);
            c.w64(8, 0xfffffffffffc0000u64);
            let r200 = c.r64(8) & c.r64(0);
            c.w64(8, r200);
            c.logic(r200, 64);
            let Some(t201) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008acf9b); };
            c.logic(t201 & 0x2u64, 8);
            if c.cc_e() { blk = 145; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008acfaa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008acfab); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008acfb8, 0x70008acfb3);
        }
        202 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008acfb8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008acfba); }
            blk = 145; continue;
        }
        203 => {
            let r202 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r202);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008acfc4); }
            return c.call(0x70008708b4, 0x70008acfce, 0x70008acfc9);
        }
        204 => {
            let Some(t203) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008acfce); };
            c.w32(1, t203);
            let r204 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r204);
            blk = 61; continue;
        }
        205 => {
            let r205 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r205);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008acfde); }
            return c.call(0x70008708b4, 0x70008acfe8, 0x70008acfe3);
        }
        206 => {
            let Some(t206) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008acfe8); };
            c.w32(8, t206);
            let r207 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r207);
            blk = 57; continue;
        }
        207 => {
            return RegfileExit::jump(0x70008acff8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ad018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ad019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad01a); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ad01b); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ad025); };
            c.w64(2, t1);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ad02e); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad035); };
            c.w64(1, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad03c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x10, 0x70008ad043);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ad046); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ad04d); }
            return c.ret(0x0, 0x70008ad04f);
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad050); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad053); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ad05c); }
            c.mov128_rr(1, 0);
            c.arith_sd(1, 1, c.xmm_u64(0));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(2, 0x1u64);
            blk = 11; continue;
        }
        10 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ad07e); };
            c.w64(0, t6);
            blk = 3; continue;
        }
        13 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ad087); };
            c.w64(0, t7);
            blk = 3; continue;
        }
        14 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ad090); };
            c.w64(0, t8);
            blk = 3; continue;
        }
        15 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ad09b); };
            c.w64(3, t9);
            return c.call(0x700086ed74, 0x70008ad0a7, 0x70008ad0a2);
        }
        16 => {
            blk = 1; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008ad0b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad0b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad0b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ad0b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ad0b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad0ba); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ad0bb); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ad0c5); };
            c.w64(2, t1);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad0d2); };
            c.w32(1, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad0d5); };
            c.cmp(t3, c.r32(1), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ad0e2); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.mov128_rr(2, 0);
            blk = 16; continue;
        }
        7 => {
            c.mov128_rr(2, 0);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008ad11b); };
            c.w64(10, t4);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008ad122); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(3, 10);
            c.mov128_rr(4, 2);
            c.arith_sd(0, 4, c.xmm_u64(3));
            c.arith_sd(1, 4, c.xmm_u64(3));
            c.ucomisd(c.xmm_u64(4), c.xmm_u64(2));
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 4, c.xmm_u64(2));
            blk = 9; continue;
        }
        9 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008ad158); };
            c.w64(10, t5);
            if !c.xorpd_rm(4, c.r64(10)) { return RegfileExit::fault(0x70008ad15f); }
            c.mov128_rr(2, 4);
            blk = 16; continue;
        }
        10 => {
            c.mov128_rr(2, 0);
            blk = 16; continue;
        }
        11 => {
            let Some(t6) = c.ld64(0x70008ad128u64) else { return RegfileExit::fault(0x70008ad16d); };
            c.w64(10, t6);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_ae() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.mov128_rr(3, 0);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.mov128_rr(2, 3);
            blk = 16; continue;
        }
        14 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.mov128_rr(2, 3);
            blk = 16; continue;
        }
        15 => {
            c.mov128_rr(2, 0);
            blk = 16; continue;
        }
        16 => {
            c.arith_sd(1, 0, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_np() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        18 => {
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            blk = 19; continue;
        }
        19 => {
            blk = 21; continue;
        }
        20 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ad1cc); };
            c.w64(0, t7);
            blk = 23; continue;
        }
        23 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad1d3); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad1da); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return c.ret(0x10, 0x70008ad1e1);
        }
        25 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ad1e4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ad1eb); }
            return c.ret(0x0, 0x70008ad1ed);
        }
        26 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ad1ee); };
            c.w64(0, t9);
            blk = 23; continue;
        }
        27 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ad1f9); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x70008ad205, 0x70008ad200);
        }
        28 => {
            blk = 1; continue;
        }
        29 => {
            return RegfileExit::jump(0x70008ad20c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad214); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ad218); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ad219); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad21a); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ad21b); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ad221); };
            c.w64(2, t1);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ad22a); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad231); };
            c.w64(1, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad238); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x10, 0x70008ad23f);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ad242); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ad249); }
            return c.ret(0x0, 0x70008ad24b);
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad24c); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad24f); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ad258); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ad265); };
            c.w64(0, t6);
            blk = 3; continue;
        }
        10 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ad26e); };
            c.w64(0, t7);
            blk = 3; continue;
        }
        11 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ad277); };
            c.w64(0, t8);
            blk = 3; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ad280); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ad28e, 0x70008ad289);
        }
        13 => {
            blk = 1; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008ad294);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad2b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad2b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ad2b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ad2b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad2ba); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ad2bb); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ad2c5); };
            c.w64(2, t1);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad2d2); };
            c.w32(1, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad2d5); };
            c.cmp(t3, c.r32(1), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ad2e2); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.mov128_rr(2, 0);
            blk = 16; continue;
        }
        7 => {
            c.mov128_rr(2, 0);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008ad31b); };
            c.w64(10, t4);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008ad322); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(3, 10);
            c.mov128_rr(4, 2);
            c.arith_sd(0, 4, c.xmm_u64(3));
            c.arith_sd(1, 4, c.xmm_u64(3));
            c.ucomisd(c.xmm_u64(4), c.xmm_u64(2));
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 4, c.xmm_u64(2));
            blk = 9; continue;
        }
        9 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008ad358); };
            c.w64(10, t5);
            if !c.xorpd_rm(4, c.r64(10)) { return RegfileExit::fault(0x70008ad35f); }
            c.mov128_rr(2, 4);
            blk = 16; continue;
        }
        10 => {
            c.mov128_rr(2, 0);
            blk = 16; continue;
        }
        11 => {
            let Some(t6) = c.ld64(0x70008ad328u64) else { return RegfileExit::fault(0x70008ad36d); };
            c.w64(10, t6);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_ae() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.mov128_rr(3, 0);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.mov128_rr(2, 3);
            blk = 16; continue;
        }
        14 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.mov128_rr(2, 3);
            blk = 16; continue;
        }
        15 => {
            c.mov128_rr(2, 0);
            blk = 16; continue;
        }
        16 => {
            c.arith_sd(1, 0, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        19 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x11c8)) else { return RegfileExit::fault(0x70008ad3bf); };
            c.w64(10, t7);
            if !c.and128_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008ad3c6); }
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(0, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            c.w8(0, (c.cc_ae() as u64));
            c.w32(0, c.r8(0));
            blk = 21; continue;
        }
        20 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ad3ec); };
            c.w64(0, t8);
            blk = 23; continue;
        }
        23 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad3f3); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad3fa); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return c.ret(0x10, 0x70008ad401);
        }
        25 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ad404); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ad40b); }
            return c.ret(0x0, 0x70008ad40d);
        }
        26 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ad40e); };
            c.w64(0, t10);
            blk = 23; continue;
        }
        27 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ad419); };
            c.w64(3, t11);
            return c.call(0x700086ed74, 0x70008ad425, 0x70008ad420);
        }
        28 => {
            blk = 1; continue;
        }
        29 => {
            return RegfileExit::jump(0x70008ad42c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad434); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ad438); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ad439); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad43a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ad43f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ad445); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ad44b); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad455); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad45b); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad463); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad46c); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad473); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x70008ad47a);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ad47d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ad484); }
            return c.ret(0x0, 0x70008ad486);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ad487); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ad48b); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ad499, 0x70008ad494);
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ad499); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ad4a4); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ad4a5); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_add(0x21f)) else { return RegfileExit::fault(0x70008ad4ac); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ad4b5); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x8f8)) else { return RegfileExit::fault(0x70008ad4b6); };
            c.w64(2, t15);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ad4bd); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008ad4be); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008ad4cf, 0x70008ad4ca);
        }
        12 => {
            return RegfileExit::fault(0x70008ad4cf);
        }
        13 => {
            return RegfileExit::jump(0x70008ad4d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad4f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ad4f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ad4f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad4fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ad4ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ad503); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ad50d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad515); };
            c.w64(1, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad51c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.ret(0x10, 0x70008ad523);
        }
        4 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ad526); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ad52d); }
            return c.ret(0x0, 0x70008ad52f);
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad530); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad536); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ad53f); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 2; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_ne() { blk = 2; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        9 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad554); };
            c.cmp(t7, 0x3fu64, 16);
            if c.cc_be() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.call(0x700089c794, 0x70008ad560, 0x70008ad55b);
        }
        11 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ad560); };
            c.w64(6, t8);
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ad564); };
            c.w32(2, t9);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(1, c.r64(2));
            let r10 = c.shr(c.r32(1), 0x1au64, 32).unwrap_or(c.r32(1));
            c.w32(1, r10);
            c.cmp(c.r32(1), 0x7u64, 32);
            if c.cc_b() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad577); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x22e0)) else { return RegfileExit::fault(0x70008ad578); };
            c.w64(3, t11);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008ad589, 0x70008ad584);
        }
        15 => {
            blk = 2; continue;
        }
        16 => {
            let r12 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r12);
            let r13 = c.r32(2) & 0xffffffu64;
            c.w32(2, r13);
            c.logic(r13, 32);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            blk = 2; continue;
        }
        17 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ad59c); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ad5aa, 0x70008ad5a5);
        }
        18 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ad5aa); };
            c.w64(6, t15);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x70008ad5b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad5d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ad5d4); };
            c.cmp(t0, c.r32(3), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(1, 0x14u64);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x0, 0x70008ad5f4);
        }
        5 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad5f5); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad5fb); };
            c.cmp(t3, c.r32(1), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ad604); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 23; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(1, c.r32(1));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x11c8)) else { return RegfileExit::fault(0x70008ad62a); };
            c.w64(10, t4);
            if !c.and128_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008ad631); }
            c.w64(10, 0x41e0000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_be() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_b() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(7, c.r64(1));
            let r5 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r5);
            if c.cc_o() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008ad668);
        }
        13 => {
            let Some(t6) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad669); };
            c.cmp(t6, 0x3fu64, 16);
            if c.cc_a() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ad670); };
            c.w32(1, t7);
            c.logic(c.r8(1) & 0x2u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(7, c.r64(1));
            let r8 = c.shr(c.r32(7), 0x1au64, 32).unwrap_or(c.r32(7));
            c.w32(7, r8);
            c.cmp(c.r32(7), 0x7u64, 32);
            if c.cc_ae() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let r9 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r9);
            let r10 = c.r32(1) & 0xffffffu64;
            c.w32(1, r10);
            c.logic(r10, 32);
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            return c.ret(0x0, 0x70008ad68f);
        }
        17 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008ad690); }
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ad691); };
            if !c.push64(t11) { return RegfileExit::fault(0x70008ad691); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ad695); }
            c.w32(0, 0x2u64);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x22e8)) else { return RegfileExit::fault(0x70008ad69f); };
            c.w64(3, t12);
            return RegfileExit::jump(0x700086ed74);
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad6ab); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ad6af); }
            let r13 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ad6b5); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ad6bc); };
            c.w64(7, t15);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x70008ad6c3); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ad6c8); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ad6cf); };
            c.cmp(t17, c.r64(8), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ad6de, 0x70008ad6d9);
        }
        20 => {
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ad6e0); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 22; continue;
        }
        22 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad6ea); };
            c.w64(1, t18);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ad6f1); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ad6f4); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008ad6f9); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad701); }
            return c.ret(0x0, 0x70008ad702);
        }
        23 => {
            let r19 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r19);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008ad707); }
            return c.call(0x70008708b4, 0x70008ad711, 0x70008ad70c);
        }
        24 => {
            let Some(t20) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008ad711); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r21);
            blk = 7; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008ad720);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad734); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ad738); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ad739); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ad73a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ad73f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ad745); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ad749); };
            c.w64(3, t3);
            return c.call(0x70008ad5d4, 0x70008ad752, 0x70008ad74d);
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad752); };
            c.w64(1, t4);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad759); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.ret(0x18, 0x70008ad760);
        }
        4 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ad763); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ad76a); }
            return c.ret(0x0, 0x70008ad76c);
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ad76d); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ad771); };
            c.w64(3, t5);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ad77f, 0x70008ad77a);
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ad77f); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008ad788);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ad794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ad794); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ad798); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ad79e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ad7a2); }
            blk = 2; continue;
        }
        1 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008ad7a8); }
            c.w64(3, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ad7af); };
            c.w64(6, t1);
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad7b3); };
            c.w32(8, t2);
            let Some(t3) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad7b7); };
            c.logic(t3 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(9, c.r32(3));
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad7d5); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad7dc); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad7e9); };
            c.w32(8, t7);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 60; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0x3fu64, 32);
            if c.cc_be() { blk = 59; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            let r8 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r8);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2374, 0x70008ad824, 0x70008ad81f);
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008ad82e, 0x70008ad829);
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad82e); };
            c.w64(1, t9);
            blk = 1; continue;
        }
        13 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad837); };
            c.w64(0, t10);
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad83b); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad841); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad853); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad859); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t17) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad866); };
            c.w32(1, t17);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 66; continue; }
            blk = 18; continue;
        }
        18 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            let r18 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r18);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2374, 0x70008ad89b, 0x70008ad896);
        }
        21 => {
            blk = 23; continue;
        }
        22 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008ad8a5, 0x70008ad8a0);
        }
        23 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad8a5); };
            c.w64(1, t19);
            blk = 1; continue;
        }
        24 => {
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad8ae); };
            c.w32(1, t20);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 67; continue; }
            blk = 26; continue;
        }
        26 => {
            let r21 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r21);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ad8dc); }
            return c.call(0x70008a2374, 0x70008ad8e5, 0x70008ad8e0);
        }
        29 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ad8e5); };
            c.w64(6, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad8e9); };
            c.w64(3, t23);
            blk = 30; continue;
        }
        30 => {
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(3));
            blk = 1; continue;
        }
        31 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(1, c.r64(3));
            let Some(t24) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad900); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            let Some(t26) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad906); };
            c.cmp(t26, 0x3fu64, 16);
            if c.cc_be() { blk = 70; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0x1u64);
            c.w64(1, c.r64(3));
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t27) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad922); };
            c.w32(7, t27);
            let r28 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r28);
            let Some(t29) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad928); };
            c.w32(7, t29);
            c.w32(8, c.r64(7).wrapping_sub(0xba));
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(7, c.r16(7));
            let r30 = c.sub(c.r32(7), 0xbau64, 32);
            c.w32(7, r30);
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a2374, 0x70008ad95a, 0x70008ad955);
        }
        40 => {
            c.w64(1, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad95d); };
            c.w64(0, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ad961); };
            c.w64(6, t32);
            blk = 41; continue;
        }
        41 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 1; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008ad973); }
            return c.call(0x70008a10d4, 0x70008ad97c, 0x70008ad977);
        }
        43 => {
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ad97f); };
            c.w64(0, t33);
            blk = 1; continue;
        }
        44 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(1, c.r64(3));
            let Some(t34) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ad994); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ad99a); };
            c.cmp(t36, c.r32(7), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ad9a7); };
            c.w32(7, t37);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(7, c.r16(7));
            c.cmp(c.r32(7), 0x3fu64, 32);
            if c.cc_be() { blk = 70; continue; }
            blk = 48; continue;
        }
        48 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            let r38 = c.sub(c.r32(7), 0xbau64, 32);
            c.w32(7, r38);
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a2374, 0x70008ad9dc, 0x70008ad9d7);
        }
        51 => {
            blk = 53; continue;
        }
        52 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a10d4, 0x70008ad9e6, 0x70008ad9e1);
        }
        53 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad9e6); };
            c.w64(1, t39);
            blk = 1; continue;
        }
        54 => {
            let r40 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r40);
            if c.cc_o() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 56; continue;
        }
        56 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ad9fa); }
            return c.ret(0x0, 0x70008ad9fb);
        }
        57 => {
            let r41 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ad9fe); };
            c.w64(1, t42);
            let r43 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r43);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 63; continue;
        }
        58 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ada16); };
            c.w64(1, t44);
            let r45 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r45);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ada1c); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 63; continue;
        }
        59 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ada2b); };
            c.w64(0, t46);
            c.w64(1, c.r64(3));
            blk = 70; continue;
        }
        60 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ada37); };
            c.w64(0, t47);
            blk = 72; continue;
        }
        61 => {
            let r48 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r48);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ada42); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 63; continue;
        }
        62 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ada51); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ada56); }
            blk = 63; continue;
        }
        63 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ada5b); };
            c.w64(1, t49);
            let Some(t50) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ada62); };
            c.w64(7, t50);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ada69); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ada70); };
            c.cmp(t52, c.r64(8), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ada75); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ada80); };
            c.w64(7, t53);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ada87); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ada8a); }
            c.w64(0, c.r64(1));
            blk = 56; continue;
        }
        66 => {
            c.w64(1, c.r64(3));
            blk = 70; continue;
        }
        67 => {
            let Some(t54) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ada9c); };
            c.w64(5, t54);
            let r55 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r55);
            return RegfileExit::jump(0x700089f114);
        }
        68 => {
            blk = 70; continue;
        }
        69 => {
            c.w64(1, c.r64(3));
            blk = 71; continue;
        }
        70 => {
            let Some(t56) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008adab0); };
            c.w64(5, t56);
            c.w64(3, c.r64(1));
            let r57 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r57);
            return RegfileExit::jump(0x700089eb34);
        }
        71 => {
            c.w64(3, c.r64(1));
            blk = 72; continue;
        }
        72 => {
            let Some(t58) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008adac3); };
            c.w64(5, t58);
            let r59 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r59);
            return RegfileExit::jump(0x700089b7d4);
        }
        73 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008adad5); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 1) { return RegfileExit::fault(0x70008adada); }
            return c.call(0x70007febf4, 0x70008adae4, 0x70008adadf);
        }
        74 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008adae7); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008adaec); }
            blk = 65; continue;
        }
        75 => {
            return RegfileExit::jump(0x70008adaf4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008adb14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008adb14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008adb18); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008adb1e); }
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008adb2a); };
            c.w64(6, t1);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(7, c.r32(3));
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adb3f); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adb45); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008adb52); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 33; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 19; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adb62); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adb68); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adb7a); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adb80); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008adb8d); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 33; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 19; continue;
        }
        12 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008adb9a); };
            c.cmp(t13, 0x41u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008adba1); }
            return c.call(0x70008a10d4, 0x70008adbaa, 0x70008adba5);
        }
        14 => {
            c.w64(1, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008adbad); };
            c.w64(0, t14);
            blk = 1; continue;
        }
        15 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, c.r64(3));
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adbc2); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adbc9); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008adbd6); };
            c.cmp(t18, 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(3, c.r64(8));
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008adbe4); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008adbf0, 0x70008adbeb);
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008adbf0); };
            c.w64(1, t19);
            blk = 1; continue;
        }
        21 => {
            let r20 = c.sub(c.r32(1), c.r32(7), 32);
            c.w32(1, r20);
            if c.cc_o() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008adc03); }
            return c.ret(0x0, 0x70008adc04);
        }
        24 => {
            let r21 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r21);
            let r22 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r22);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 28; continue;
        }
        25 => {
            let r23 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r23);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008adc1d); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 28; continue;
        }
        26 => {
            let r24 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r24);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008adc2e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 28; continue;
        }
        27 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008adc3d); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008adc42); }
            blk = 28; continue;
        }
        28 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008adc47); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008adc4e); };
            c.w64(7, t26);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008adc55); };
            c.w64(9, t27);
            let Some(t28) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008adc5c); };
            c.cmp(t28, c.r64(8), 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008adc61); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 30; continue;
        }
        30 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adc6c); };
            c.w64(7, t29);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008adc73); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008adc76); }
            c.w64(0, c.r64(1));
            blk = 23; continue;
        }
        31 => {
            c.w64(8, c.r64(3));
            blk = 32; continue;
        }
        32 => {
            c.w64(3, c.r64(8));
            blk = 33; continue;
        }
        33 => {
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008adc86); };
            c.w64(5, t30);
            let r31 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(0x700089c174);
        }
        34 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x70008adc98); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x70008adc9d); }
            return c.call(0x70007febf4, 0x70008adca7, 0x70008adca2);
        }
        35 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008adcaa); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008adcaf); }
            blk = 30; continue;
        }
        36 => {
            return RegfileExit::jump(0x70008adcb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008adcd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008adcd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008adcd8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008adcde); }
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008adcea); };
            c.w64(6, t1);
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adcfb); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008add01); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008add0e); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 19; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008add1e); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008add24); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008add36); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008add3c); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008add49); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 19; continue;
        }
        12 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008add56); };
            c.cmp(t13, 0x41u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008add5d); }
            return c.call(0x70008a10d4, 0x70008add66, 0x70008add61);
        }
        14 => {
            c.w64(1, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008add69); };
            c.w64(0, t14);
            blk = 1; continue;
        }
        15 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, c.r64(3));
            let Some(t15) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008add7e); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008add84); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008add91); };
            c.cmp(t18, 0x41u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(3, c.r64(1));
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008add9f); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008addab, 0x70008adda6);
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008addab); };
            c.w64(1, t19);
            blk = 1; continue;
        }
        21 => {
            c.w64(1, c.r64(3));
            let r20 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r20);
            c.w64(7, c.r64(0));
            let r21 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r21);
            c.w64(8, c.r64(7));
            let r22 = c.imul2(c.r32(8), c.r32(1), 32);
            c.w32(8, r22);
            if c.cc_o() { blk = 43; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(1, c.r64(8));
            let r23 = c.add(c.r32(1), c.r32(8), 32);
            c.w32(1, r23);
            if c.cc_o() { blk = 38; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008adde3); }
            return c.ret(0x0, 0x70008adde4);
        }
        26 => {
            let r24 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r24);
            let r25 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r25);
            let r26 = c.r32(3) | c.r32(0);
            c.w32(3, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        28 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008addf4); };
            c.w64(0, t27);
            blk = 25; continue;
        }
        29 => {
            let r28 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r28);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008addff); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 32; continue;
        }
        30 => {
            let r29 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r29);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ade10); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 32; continue;
        }
        31 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ade1f); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ade24); }
            blk = 32; continue;
        }
        32 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ade29); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ade30); };
            c.w64(7, t31);
            c.w64(9, c.r64(7).wrapping_add(0xc));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ade37); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ade3e); };
            c.cmp(t33, c.r64(9), 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008ade47); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 34; continue;
        }
        34 => {
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ade52); };
            c.w64(7, t34);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ade59); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ade5c); }
            c.w64(0, c.r64(1));
            blk = 25; continue;
        }
        35 => {
            c.w64(1, c.r64(3));
            blk = 36; continue;
        }
        36 => {
            c.w64(3, c.r64(1));
            blk = 37; continue;
        }
        37 => {
            let Some(t35) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ade6f); };
            c.w64(5, t35);
            let r36 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r36);
            let Some(t37) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ade77); };
            if !c.push64(t37) { return RegfileExit::fault(0x70008ade77); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ade7b); }
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ade85); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008ade8a); };
            c.w64(3, t38);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x4u64) { return RegfileExit::fault(0x70008ade91); }
            return RegfileExit::jump(0x700086ed74);
        }
        38 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ade9f); };
            c.w64(9, t39);
            let Some(t40) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008adea6); };
            c.w64(1, t40);
            c.w64(7, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008adead); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008adeb1); };
            c.w64(11, t41);
            let Some(t42) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008adeb8); };
            c.cmp(t42, c.r64(7), 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008adec7, 0x70008adec2);
        }
        40 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008adec7); };
            c.w64(8, t43);
            blk = 42; continue;
        }
        41 => {
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x70008adecd); }
            let r44 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r44);
            c.w64(0, c.r64(1));
            blk = 42; continue;
        }
        42 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aded7); };
            c.w64(12, t45);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008adede); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008adeeb); }
            blk = 25; continue;
        }
        43 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008adef5); };
            c.w64(8, t46);
            let Some(t47) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008adefc); };
            c.w64(9, t47);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008adf03); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008adf07); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008adf0b); };
            c.w64(11, t48);
            let Some(t49) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008adf12); };
            c.cmp(t49, c.r64(12), 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008adf21, 0x70008adf1c);
        }
        45 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008adf21); };
            c.w64(1, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008adf25); };
            c.w64(7, t51);
            blk = 47; continue;
        }
        46 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008adf2b); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 47; continue;
        }
        47 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adf49); };
            c.w64(1, t52);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008adf50); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008adf53); }
            blk = 25; continue;
        }
        48 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008adf5d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 1) { return RegfileExit::fault(0x70008adf62); }
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008adf71, 0x70008adf6c);
        }
        49 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008adf74); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008adf79); }
            blk = 34; continue;
        }
        50 => {
            return RegfileExit::jump(0x70008adf84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008adf94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008adf94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008adf98); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008adf9e); }
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008adfaa); };
            c.w64(6, t1);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(7, c.r32(3));
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adfbf); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adfc5); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008adfd2); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 19; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adfe2); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008adfe8); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008adffa); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae000); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae00d); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 19; continue;
        }
        12 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae01a); };
            c.cmp(t13, 0x41u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ae021); }
            return c.call(0x70008a10d4, 0x70008ae02a, 0x70008ae025);
        }
        14 => {
            c.w64(1, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ae02d); };
            c.w64(0, t14);
            blk = 1; continue;
        }
        15 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, c.r64(3));
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae042); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae049); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae056); };
            c.cmp(t18, 0x41u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(3, c.r64(8));
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ae064); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008ae070, 0x70008ae06b);
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae070); };
            c.w64(1, t19);
            blk = 1; continue;
        }
        21 => {
            c.w64(8, c.r64(3));
            let r20 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r20);
            c.w64(9, c.r64(0));
            let r21 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r21);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(8), 0xffffffffffffffffu64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0xc0000000u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(9));
            c.cdq();
            c.idiv(c.r32(8), 32);
            c.w64(1, c.r64(0));
            let r22 = c.imul2(c.r32(1), c.r32(8), 32);
            c.w32(1, r22);
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let r23 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r23);
            blk = 28; continue;
        }
        28 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ae0bb); }
            return c.ret(0x0, 0x70008ae0bc);
        }
        29 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 33; continue;
        }
        30 => {
            let r24 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r24);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae0d3); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 33; continue;
        }
        31 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae0e4); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 33; continue;
        }
        32 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae0f3); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae0f8); }
            blk = 33; continue;
        }
        33 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ae0fd); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ae104); };
            c.w64(7, t27);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ae10b); };
            c.w64(9, t28);
            let Some(t29) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ae112); };
            c.cmp(t29, c.r64(8), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ae117); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 35; continue;
        }
        35 => {
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae125); };
            c.w64(7, t30);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ae12c); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008ae12f); }
            c.w64(0, c.r64(1));
            blk = 28; continue;
        }
        36 => {
            c.w64(8, c.r64(3));
            blk = 37; continue;
        }
        37 => {
            c.w64(3, c.r64(8));
            blk = 38; continue;
        }
        38 => {
            let Some(t31) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ae142); };
            c.w64(5, t31);
            let r32 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r32);
            let Some(t33) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ae14a); };
            if !c.push64(t33) { return RegfileExit::fault(0x70008ae14a); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ae14e); }
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ae158); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008ae15d); };
            c.w64(3, t34);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x6u64) { return RegfileExit::fault(0x70008ae164); }
            return RegfileExit::jump(0x700086ed74);
        }
        39 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x70008ae177); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x70008ae17c); }
            return c.call(0x70007febf4, 0x70008ae186, 0x70008ae181);
        }
        40 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ae189); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008ae18e); }
            blk = 35; continue;
        }
        41 => {
            return RegfileExit::jump(0x70008ae198);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ae1b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ae1b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ae1b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ae1be); }
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ae1ca); };
            c.w64(6, t1);
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae1db); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae1e1); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae1ee); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 35; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 19; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae1fe); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae204); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae216); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae21c); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae229); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 35; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 19; continue;
        }
        12 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae236); };
            c.cmp(t13, 0x41u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ae23d); }
            return c.call(0x70008a10d4, 0x70008ae246, 0x70008ae241);
        }
        14 => {
            c.w64(1, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ae249); };
            c.w64(0, t14);
            blk = 1; continue;
        }
        15 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, c.r64(3));
            let Some(t15) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae25e); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae264); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae271); };
            c.cmp(t18, 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(3, c.r64(1));
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ae27f); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008ae28b, 0x70008ae286);
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae28b); };
            c.w64(1, t19);
            blk = 1; continue;
        }
        21 => {
            let r20 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r20);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 22; continue;
        }
        22 => {
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            blk = 24; continue;
        }
        24 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ae2b3); }
            return c.ret(0x0, 0x70008ae2b4);
        }
        25 => {
            let r22 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r22);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae2b7); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 28; continue;
        }
        26 => {
            let r23 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r23);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae2c8); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 28; continue;
        }
        27 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae2d7); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae2dc); }
            blk = 28; continue;
        }
        28 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ae2e1); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ae2e8); };
            c.w64(7, t25);
            c.w64(9, c.r64(7).wrapping_add(0xc));
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ae2ef); };
            c.w64(8, t26);
            let Some(t27) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ae2f6); };
            c.cmp(t27, c.r64(9), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008ae2ff); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 30; continue;
        }
        30 => {
            let r28 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r28);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008ae30a); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008ae30f); }
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x70008ae312); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008ae317); }
            blk = 31; continue;
        }
        31 => {
            c.fprem();
            c.fnstsw_ax();
            let r29 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            let r30 = c.r32(0) & 0xffu64;
            c.w32(0, r30);
            c.logic(r30, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ae326); }
            if !c.popfq() { return RegfileExit::fault(0x70008ae327); }
            if c.cc_p() { blk = 31; continue; }
            blk = 32; continue;
        }
        32 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x70008ae32c); }
            if !c.movsd_rm(1, c.r64(4)) { return RegfileExit::fault(0x70008ae32f); }
            let r31 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae338); };
            c.w64(7, t32);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ae33f); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008ae342); }
            c.w64(0, c.r64(1));
            blk = 24; continue;
        }
        33 => {
            c.w64(1, c.r64(3));
            blk = 34; continue;
        }
        34 => {
            c.w64(3, c.r64(1));
            blk = 35; continue;
        }
        35 => {
            let Some(t33) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ae355); };
            c.w64(5, t33);
            let r34 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ae35d); };
            if !c.push64(t35) { return RegfileExit::fault(0x70008ae35d); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ae361); }
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ae36b); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008ae370); };
            c.w64(3, t36);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x8u64) { return RegfileExit::fault(0x70008ae377); }
            return RegfileExit::jump(0x700086ed74);
        }
        36 => {
            c.cdq();
            c.idiv(c.r32(3), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008ae388); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(2));
            let r37 = c.add(c.r32(1), c.r32(2), 32);
            c.w32(1, r37);
            if c.cc_o() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 24; continue;
        }
        39 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ae3a0); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ae3a7); };
            c.w64(7, t39);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ae3ae); };
            c.w64(9, t40);
            let Some(t41) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ae3b5); };
            c.cmp(t41, c.r64(8), 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x70008ae3c7, 0x70008ae3c2);
        }
        41 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae3c7); };
            c.w64(2, t42);
            blk = 43; continue;
        }
        42 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ae3cd); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 43; continue;
        }
        43 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae3d7); };
            c.w64(8, t43);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008ae3de); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ae3ea); }
            blk = 24; continue;
        }
        44 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008ae3f4); };
            c.w64(0, t44);
            blk = 24; continue;
        }
        45 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008ae400); };
            c.w64(0, t45);
            blk = 24; continue;
        }
        46 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008ae40c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 1) { return RegfileExit::fault(0x70008ae411); }
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ae420, 0x70008ae41b);
        }
        47 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ae423); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008ae428); }
            blk = 30; continue;
        }
        48 => {
            return RegfileExit::jump(0x70008ae434);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ae454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ae454); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ae458); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ae45e); }
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ae46a); };
            c.w64(6, t1);
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae472); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae478); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae48a); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae490); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae499); };
            c.cmp(t8, 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(1, c.r64(3));
            blk = 14; continue;
        }
        8 => {
            let Some(t9) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae4a9); };
            c.cmp(t9, 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ae4b0); }
            return c.call(0x70008a10d4, 0x70008ae4b9, 0x70008ae4b4);
        }
        10 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ae4bc); };
            c.w64(0, t10);
            blk = 1; continue;
        }
        11 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(3));
            let Some(t11) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae4ce); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae4d4); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t14) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae4e1); };
            c.cmp(t14, 0x41u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ae4ec); }
            c.w64(0, c.r64(1));
            return c.call(0x70008a10d4, 0x70008ae4f8, 0x70008ae4f3);
        }
        15 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae4f8); };
            c.w64(1, t15);
            blk = 1; continue;
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ae501); }
            blk = 19; continue;
        }
        17 => {
            return c.call(0x70008a1014, 0x70008ae50c, 0x70008ae507);
        }
        18 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ae50c); };
            c.w64(6, t16);
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae514); };
            c.w32(1, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae517); };
            c.cmp(t18, c.r32(1), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae520); }
            blk = 23; continue;
        }
        22 => {
            let r19 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r19);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 23; continue;
        }
        23 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 2) { return RegfileExit::fault(0x70008ae531); }
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ae536); };
            c.w64(0, t20);
            blk = 26; continue;
        }
        24 => {
            return c.call(0x70008a1014, 0x70008ae541, 0x70008ae53c);
        }
        25 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ae541); };
            c.w64(6, t21);
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae549); };
            c.w32(1, t22);
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae54c); };
            c.cmp(t23, c.r32(1), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae555); }
            blk = 30; continue;
        }
        29 => {
            let r24 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r24);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ae566); }
            c.mov128_rr(1, 2);
            c.w64(10, c.r64(4));
            let r25 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            let r26 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r26);
            c.logic(r26, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ae579); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70008ae57e); };
            c.w64(0, t27);
            c.w64(10, 0x70008ae58cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ae58c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ae590); }
            return c.call(c.r64(0), 0x70008ae596, 0x70008ae594);
        }
        31 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ae596); }
            let Some(t28) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ae59e); };
            c.w64(4, t28);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 41; continue; }
            blk = 32; continue;
        }
        32 => {
            if c.cc_ne() { blk = 41; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(7, c.r64(1));
            let r29 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r29);
            if c.cc_o() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ae5d7); }
            return c.ret(0x0, 0x70008ae5d8);
        }
        38 => {
            c.w64(1, c.r64(3));
            blk = 40; continue;
        }
        39 => {
            c.w64(1, c.r64(3));
            blk = 40; continue;
        }
        40 => {
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ae5e1); };
            c.w64(5, t30);
            let r31 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r31);
            let Some(t32) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ae5e9); };
            if !c.push64(t32) { return RegfileExit::fault(0x70008ae5e9); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008ae5ed); };
            c.w64(3, t33);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ae5f4); }
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0xau64) { return RegfileExit::fault(0x70008ae5fe); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(1)) { return RegfileExit::fault(0x70008ae607); }
            return RegfileExit::jump(0x700086ed74);
        }
        41 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ae611); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ae618); };
            c.w64(7, t35);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008ae61f); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ae624); };
            c.w64(9, t36);
            let Some(t37) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ae62b); };
            c.cmp(t37, c.r64(8), 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ae63a, 0x70008ae635);
        }
        43 => {
            blk = 45; continue;
        }
        44 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ae63c); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 45; continue;
        }
        45 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae646); };
            c.w64(1, t38);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ae64d); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ae650); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ae655); }
            blk = 37; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008ae660);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ae674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ae674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ae678); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008ae67e); }
            blk = 3; continue;
        }
        1 => {
            return c.call(0x70008a10d4, 0x70008ae689, 0x70008ae684);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae689); };
            c.w64(6, t1);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae691); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae697); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae6a0); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 1; continue;
        }
        7 => {
            let r6 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r6);
            c.w32(1, 0);
            c.logic(0, 32);
            let r7 = c.sub(c.r32(1), c.r32(0), 32);
            c.w32(1, r7);
            if c.cc_o() { blk = 22; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(7, c.r64(1));
            let r8 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r8);
            if c.cc_o() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ae6cb); }
            return c.ret(0x0, 0x70008ae6cc);
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008ae6cd); };
            c.w64(0, t9);
            blk = 11; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ae6d6); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ae6db); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ae6e2); };
            c.w64(7, t11);
            c.w64(9, c.r64(7).wrapping_add(0xc));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ae6e9); };
            c.w64(8, t12);
            let Some(t13) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ae6f0); };
            c.cmp(t13, c.r64(9), 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008ae6f9); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008ae700); };
            c.w64(10, t14);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008ae707); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae70b); };
            c.w64(7, t15);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ae712); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ae715); }
            c.w64(0, c.r64(1));
            blk = 11; continue;
        }
        16 => {
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ae71f); };
            c.w64(5, t16);
            let r17 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r17);
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ae727); };
            if !c.push64(t18) { return RegfileExit::fault(0x70008ae727); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008ae72b); };
            c.w64(3, t19);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ae732); }
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x1au64) { return RegfileExit::fault(0x70008ae73c); }
            return RegfileExit::jump(0x700086ed74);
        }
        17 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ae74a); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ae751); };
            c.w64(8, t21);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008ae758); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ae75c); };
            c.w64(11, t22);
            let Some(t23) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ae763); };
            c.cmp(t23, c.r64(9), 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ae772, 0x70008ae76d);
        }
        19 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae772); };
            c.w64(1, t24);
            blk = 21; continue;
        }
        20 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008ae778); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 21; continue;
        }
        21 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae782); };
            c.w64(8, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008ae789); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ae795); }
            blk = 11; continue;
        }
        22 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ae79f); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ae7a6); };
            c.w64(7, t27);
            c.w64(9, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ae7ad); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ae7b1); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ae7b8); };
            c.cmp(t29, c.r64(9), 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ae7c7, 0x70008ae7c2);
        }
        24 => {
            c.w64(1, c.r64(0));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae7ca); };
            c.w64(0, t30);
            blk = 26; continue;
        }
        25 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008ae7d0); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008ae7df); };
            c.w64(10, t31);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008ae7e6); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae7ea); };
            c.w64(7, t32);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ae7f1); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ae7f4); }
            c.w64(0, c.r64(1));
            blk = 11; continue;
        }
        27 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008ae801); }
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ae810, 0x70008ae80b);
        }
        28 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ae813); }
            blk = 15; continue;
        }
        29 => {
            return RegfileExit::jump(0x70008ae820);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ae834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ae834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ae838); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008ae83e); }
            blk = 3; continue;
        }
        1 => {
            return c.call(0x70008a10d4, 0x70008ae849, 0x70008ae844);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae849); };
            c.w64(6, t1);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae851); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae857); };
            c.cmp(t4, c.r32(2), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae860); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 1; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ae869); };
            c.w64(5, t6);
            c.w64(3, 0xfffffffffffffffeu64);
            let r7 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r7);
            return RegfileExit::jump(0x70008af374);
        }
        8 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ae87d); }
            c.w32(2, 0x18u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ae883); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008ae884); };
            c.w64(3, t8);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ae895, 0x70008ae890);
        }
        9 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ae898); }
            return c.ret(0x0, 0x70008ae899);
        }
        10 => {
            return RegfileExit::jump(0x70008ae89c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ae8b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ae8b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ae8b8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008ae8be); }
            blk = 3; continue;
        }
        1 => {
            return c.call(0x70008a10d4, 0x70008ae8c9, 0x70008ae8c4);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae8c9); };
            c.w64(6, t1);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae8d1); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae8d7); };
            c.cmp(t4, c.r32(2), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae8e0); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 1; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ae8e9); };
            c.w64(5, t6);
            c.w32(3, 0x2u64);
            let r7 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r7);
            return RegfileExit::jump(0x70008adb14);
        }
        8 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ae8fb); }
            c.w32(2, 0x1eu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ae901); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008ae902); };
            c.w64(3, t8);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ae913, 0x70008ae90e);
        }
        9 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ae916); }
            return c.ret(0x0, 0x70008ae917);
        }
        10 => {
            return RegfileExit::jump(0x70008ae91c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ae934(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ae934); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ae938); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008ae93e); }
            blk = 3; continue;
        }
        1 => {
            return c.call(0x70008a10d4, 0x70008ae949, 0x70008ae944);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ae949); };
            c.w64(6, t1);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae951); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae957); };
            c.cmp(t4, c.r32(2), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae960); };
            c.cmp(t5, 0x41u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 1; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ae969); };
            c.w64(5, t6);
            c.w32(3, 0x2u64);
            let r7 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r7);
            return RegfileExit::jump(0x70008ad794);
        }
        8 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ae97b); }
            c.w32(2, 0x1cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ae981); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x70008ae982); };
            c.w64(3, t8);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ae993, 0x70008ae98e);
        }
        9 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ae996); }
            return c.ret(0x0, 0x70008ae997);
        }
        10 => {
            return RegfileExit::jump(0x70008ae99c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ae9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ae9b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ae9b8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008ae9be); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008ae9c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008ae9c6); }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(0));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ae9d3); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ae9da); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ae9e7); };
            c.w32(8, t4);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a10d4, 0x70008aea02, 0x70008ae9fd);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008aea02); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aea06); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aea0c); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r9);
            blk = 20; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aea26); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aea2d); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aea36); };
            c.w32(8, t13);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aea49); }
            return c.call(0x70008a10d4, 0x70008aea52, 0x70008aea4d);
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aea52); };
            c.w64(7, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aea56); };
            c.w64(6, t15);
            blk = 17; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aea5c); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        16 => {
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        17 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(7));
            blk = 31; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aea72); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 44; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(7, c.r32(7));
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aea88); }
            blk = 22; continue;
        }
        21 => {
            c.w64(3, c.r64(0));
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aea96); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aea9d); };
            c.cmp(t20, c.r32(8), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aeaa6); };
            c.w32(8, t21);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008aeac1, 0x70008aeabc);
        }
        27 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aeac1); };
            c.w64(6, t22);
            blk = 21; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aeac7); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            c.w64(0, c.r64(3));
            blk = 21; continue;
        }
        29 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            blk = 35; continue;
        }
        30 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aead6); };
            c.w64(0, t26);
            blk = 31; continue;
        }
        31 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aeada); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008aeadb); }
            c.w32(7, 0x12u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aeae1); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008aeae2); };
            c.w64(3, t27);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008aeaf3, 0x70008aeaee);
        }
        32 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aeaf6); }
            return c.ret(0x0, 0x70008aeaf7);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aeaf8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(3, c.r32(3));
            blk = 35; continue;
        }
        35 => {
            c.w64(1, c.r64(3));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aeb0d); };
            c.w64(7, t28);
            let r29 = c.shl(c.r32(7), c.r8(1), 32).unwrap_or(c.r32(7));
            c.w32(7, r29);
            c.w64(8, c.r64(7));
            let r30 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r30);
            if c.cc_o() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 32; continue;
        }
        37 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aeb20); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008aeb27); };
            c.w64(9, t32);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aeb2e); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aeb32); };
            c.w64(12, t33);
            let Some(t34) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008aeb39); };
            c.cmp(t34, c.r64(11), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aeb49, 0x70008aeb44);
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aeb49); };
            c.w64(7, t35);
            blk = 41; continue;
        }
        40 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008aeb4f); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 41; continue;
        }
        41 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aeb59); };
            c.w64(8, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008aeb60); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aeb6c); }
            blk = 32; continue;
        }
        42 => {
            let r37 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008aeb77); }
            return c.call(0x70008708b4, 0x70008aeb81, 0x70008aeb7c);
        }
        43 => {
            let Some(t38) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008aeb81); };
            c.w32(3, t38);
            let r39 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r39);
            blk = 34; continue;
        }
        44 => {
            let r40 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r40);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008aeb91); }
            return c.call(0x70008708b4, 0x70008aeb9b, 0x70008aeb96);
        }
        45 => {
            let Some(t41) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008aeb9b); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r42);
            blk = 19; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008aeba8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aebb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aebb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008aebb8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008aebbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008aebc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008aebc6); }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(0));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aebd3); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aebda); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aebe7); };
            c.w32(8, t4);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a10d4, 0x70008aec02, 0x70008aebfd);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008aec02); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aec06); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aec0c); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r9);
            blk = 20; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aec26); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aec2d); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aec36); };
            c.w32(8, t13);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aec49); }
            return c.call(0x70008a10d4, 0x70008aec52, 0x70008aec4d);
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aec52); };
            c.w64(7, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aec56); };
            c.w64(6, t15);
            blk = 17; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aec5c); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        16 => {
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        17 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(7));
            blk = 31; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aec72); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 44; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(7, c.r32(7));
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aec88); }
            blk = 22; continue;
        }
        21 => {
            c.w64(3, c.r64(0));
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aec96); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aec9d); };
            c.cmp(t20, c.r32(8), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aeca6); };
            c.w32(8, t21);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008aecc1, 0x70008aecbc);
        }
        27 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aecc1); };
            c.w64(6, t22);
            blk = 21; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aecc7); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            c.w64(0, c.r64(3));
            blk = 21; continue;
        }
        29 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            blk = 35; continue;
        }
        30 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aecd6); };
            c.w64(0, t26);
            blk = 31; continue;
        }
        31 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aecda); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008aecdb); }
            c.w32(7, 0x14u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aece1); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008aece2); };
            c.w64(3, t27);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008aecf3, 0x70008aecee);
        }
        32 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aecf6); }
            return c.ret(0x0, 0x70008aecf7);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aecf8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(3, c.r32(3));
            blk = 35; continue;
        }
        35 => {
            c.w64(1, c.r64(3));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aed0d); };
            c.w64(7, t28);
            let r29 = c.sar(c.r32(7), c.r8(1), 32).unwrap_or(c.r32(7));
            c.w32(7, r29);
            c.w64(8, c.r64(7));
            let r30 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r30);
            if c.cc_o() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 32; continue;
        }
        37 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aed20); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008aed27); };
            c.w64(9, t32);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aed2e); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aed32); };
            c.w64(12, t33);
            let Some(t34) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008aed39); };
            c.cmp(t34, c.r64(11), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aed49, 0x70008aed44);
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aed49); };
            c.w64(7, t35);
            blk = 41; continue;
        }
        40 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008aed4f); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 41; continue;
        }
        41 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aed59); };
            c.w64(8, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008aed60); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aed6c); }
            blk = 32; continue;
        }
        42 => {
            let r37 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008aed77); }
            return c.call(0x70008708b4, 0x70008aed81, 0x70008aed7c);
        }
        43 => {
            let Some(t38) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008aed81); };
            c.w32(3, t38);
            let r39 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r39);
            blk = 34; continue;
        }
        44 => {
            let r40 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r40);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008aed91); }
            return c.call(0x70008708b4, 0x70008aed9b, 0x70008aed96);
        }
        45 => {
            let Some(t41) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008aed9b); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r42);
            blk = 19; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008aeda8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aedb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aedb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008aedb8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008aedbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008aedc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008aedc6); }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(0));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aedd3); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aedda); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aede7); };
            c.w32(8, t4);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a10d4, 0x70008aee02, 0x70008aedfd);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008aee02); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aee06); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aee0c); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r9);
            blk = 20; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aee26); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aee2d); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aee36); };
            c.w32(8, t13);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aee49); }
            return c.call(0x70008a10d4, 0x70008aee52, 0x70008aee4d);
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aee52); };
            c.w64(7, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aee56); };
            c.w64(6, t15);
            blk = 17; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aee5c); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        16 => {
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        17 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(7));
            blk = 31; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aee72); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 44; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(7, c.r32(7));
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aee88); }
            blk = 22; continue;
        }
        21 => {
            c.w64(3, c.r64(0));
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aee96); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aee9d); };
            c.cmp(t20, c.r32(8), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aeea6); };
            c.w32(8, t21);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008aeec1, 0x70008aeebc);
        }
        27 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008aeec1); };
            c.w64(6, t22);
            blk = 21; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008aeec7); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            c.w64(0, c.r64(3));
            blk = 21; continue;
        }
        29 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            blk = 35; continue;
        }
        30 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008aeed6); };
            c.w64(0, t26);
            blk = 31; continue;
        }
        31 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008aeeda); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008aeedb); }
            c.w32(7, 0x16u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008aeee1); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008aeee2); };
            c.w64(3, t27);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008aeef3, 0x70008aeeee);
        }
        32 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008aeef6); }
            return c.ret(0x0, 0x70008aeef7);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aeef8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(3, c.r32(3));
            blk = 35; continue;
        }
        35 => {
            c.w64(1, c.r64(3));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aef0d); };
            c.w64(7, t28);
            let r29 = c.shr(c.r32(7), c.r8(1), 32).unwrap_or(c.r32(7));
            c.w32(7, r29);
            c.cmp(c.r32(7), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            blk = 32; continue;
        }
        37 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008aef20); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008aef27); };
            c.w64(9, t31);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008aef2e); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008aef32); };
            c.w64(12, t32);
            let Some(t33) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008aef39); };
            c.cmp(t33, c.r64(11), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008aef49, 0x70008aef44);
        }
        39 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008aef49); };
            c.w64(7, t34);
            blk = 41; continue;
        }
        40 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008aef4f); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 41; continue;
        }
        41 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aef59); };
            c.w64(8, t35);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008aef60); }
            c.w32(10, c.r32(7));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008aef70); }
            blk = 32; continue;
        }
        42 => {
            let r36 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r36);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008aef7e); }
            return c.call(0x70008708b4, 0x70008aef88, 0x70008aef83);
        }
        43 => {
            let Some(t37) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008aef88); };
            c.w32(3, t37);
            let r38 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r38);
            blk = 34; continue;
        }
        44 => {
            let r39 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r39);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008aef98); }
            return c.call(0x70008708b4, 0x70008aefa2, 0x70008aef9d);
        }
        45 => {
            let Some(t40) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008aefa2); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r41);
            blk = 19; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008aefb0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008aefb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aefb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008aefb8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008aefbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008aefc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008aefc6); }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(0));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aefd2); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aefd8); };
            c.cmp(t3, c.r32(7), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aefe5); };
            c.w32(7, t4);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a10d4, 0x70008aeffd, 0x70008aeff8);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008aeffd); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af001); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af007); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            c.w64(0, c.r64(1));
            blk = 1; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r9);
            blk = 20; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af01e); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af024); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af02d); };
            c.w32(7, t13);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008af03d); }
            return c.call(0x70008a10d4, 0x70008af046, 0x70008af041);
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af046); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af04a); };
            c.w64(6, t15);
            blk = 17; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af050); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        16 => {
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        17 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            blk = 31; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af066); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 44; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(1, c.r32(1));
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008af07c); }
            blk = 22; continue;
        }
        21 => {
            c.w64(3, c.r64(0));
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af08a); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af090); };
            c.cmp(t20, c.r32(7), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af099); };
            c.w32(7, t21);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008af0b1, 0x70008af0ac);
        }
        27 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af0b1); };
            c.w64(6, t22);
            blk = 21; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af0b7); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            c.w64(0, c.r64(3));
            blk = 21; continue;
        }
        29 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            blk = 35; continue;
        }
        30 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af0c6); };
            c.w64(0, t26);
            blk = 31; continue;
        }
        31 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af0ca); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af0cb); }
            c.w32(2, 0xcu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008af0d1); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008af0d2); };
            c.w64(3, t27);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008af0e3, 0x70008af0de);
        }
        32 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008af0e6); }
            return c.ret(0x0, 0x70008af0e7);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af0e8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(3, c.r32(3));
            blk = 35; continue;
        }
        35 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af0fa); };
            let r29 = c.r32(3) & t28;
            c.w32(3, r29);
            c.logic(r29, 32);
            c.w64(1, c.r64(3));
            let r30 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r30);
            if c.cc_o() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 32; continue;
        }
        37 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008af109); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008af110); };
            c.w64(7, t32);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008af117); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008af11b); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008af122); };
            c.cmp(t34, c.r64(8), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008af131, 0x70008af12c);
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af131); };
            c.w64(3, t35);
            blk = 41; continue;
        }
        40 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008af137); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 41; continue;
        }
        41 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af141); };
            c.w64(1, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008af148); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008af153); }
            blk = 32; continue;
        }
        42 => {
            let r37 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008af15e); }
            return c.call(0x70008708b4, 0x70008af168, 0x70008af163);
        }
        43 => {
            let Some(t38) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008af168); };
            c.w32(3, t38);
            let r39 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r39);
            blk = 34; continue;
        }
        44 => {
            let r40 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r40);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008af175); }
            return c.call(0x70008708b4, 0x70008af17f, 0x70008af17a);
        }
        45 => {
            let Some(t41) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008af17f); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r42);
            blk = 19; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008af18c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008af194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008af194); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008af198); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008af19e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008af1a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008af1a6); }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(0));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af1b2); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af1b8); };
            c.cmp(t3, c.r32(7), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af1c5); };
            c.w32(7, t4);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a10d4, 0x70008af1dd, 0x70008af1d8);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008af1dd); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af1e1); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af1e7); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            c.w64(0, c.r64(1));
            blk = 1; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r9);
            blk = 20; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af1fe); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af204); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af20d); };
            c.w32(7, t13);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008af21d); }
            return c.call(0x70008a10d4, 0x70008af226, 0x70008af221);
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af226); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af22a); };
            c.w64(6, t15);
            blk = 17; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af230); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        16 => {
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        17 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            blk = 31; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af246); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 44; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(1, c.r32(1));
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008af25c); }
            blk = 22; continue;
        }
        21 => {
            c.w64(3, c.r64(0));
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af26a); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af270); };
            c.cmp(t20, c.r32(7), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af279); };
            c.w32(7, t21);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008af291, 0x70008af28c);
        }
        27 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af291); };
            c.w64(6, t22);
            blk = 21; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af297); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            c.w64(0, c.r64(3));
            blk = 21; continue;
        }
        29 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            blk = 35; continue;
        }
        30 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af2a6); };
            c.w64(0, t26);
            blk = 31; continue;
        }
        31 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af2aa); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af2ab); }
            c.w32(1, 0xeu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af2b1); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008af2b2); };
            c.w64(3, t27);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008af2c3, 0x70008af2be);
        }
        32 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008af2c6); }
            return c.ret(0x0, 0x70008af2c7);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af2c8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(3, c.r32(3));
            blk = 35; continue;
        }
        35 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af2da); };
            let r29 = c.r32(3) | t28;
            c.w32(3, r29);
            c.logic(r29, 32);
            c.w64(1, c.r64(3));
            let r30 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r30);
            if c.cc_o() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 32; continue;
        }
        37 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008af2e9); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008af2f0); };
            c.w64(7, t32);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008af2f7); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008af2fb); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008af302); };
            c.cmp(t34, c.r64(8), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008af311, 0x70008af30c);
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af311); };
            c.w64(3, t35);
            blk = 41; continue;
        }
        40 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008af317); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 41; continue;
        }
        41 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af321); };
            c.w64(1, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008af328); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008af333); }
            blk = 32; continue;
        }
        42 => {
            let r37 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008af33e); }
            return c.call(0x70008708b4, 0x70008af348, 0x70008af343);
        }
        43 => {
            let Some(t38) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008af348); };
            c.w32(3, t38);
            let r39 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r39);
            blk = 34; continue;
        }
        44 => {
            let r40 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r40);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008af355); }
            return c.call(0x70008708b4, 0x70008af35f, 0x70008af35a);
        }
        45 => {
            let Some(t41) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008af35f); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r42);
            blk = 19; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008af36c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008af374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008af374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008af378); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008af37e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008af382); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008af386); }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(0));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af392); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af398); };
            c.cmp(t3, c.r32(7), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af3a5); };
            c.w32(7, t4);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a10d4, 0x70008af3bd, 0x70008af3b8);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008af3bd); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af3c1); };
            c.w64(6, t6);
            blk = 1; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af3c7); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            c.w64(0, c.r64(1));
            blk = 1; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r9);
            blk = 20; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af3de); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af3e4); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af3ed); };
            c.w32(7, t13);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008af3fd); }
            return c.call(0x70008a10d4, 0x70008af406, 0x70008af401);
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af406); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af40a); };
            c.w64(6, t15);
            blk = 17; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af410); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        16 => {
            c.w64(0, c.r64(3));
            blk = 17; continue;
        }
        17 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            blk = 31; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af426); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 44; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(1, c.r32(1));
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008af43c); }
            blk = 22; continue;
        }
        21 => {
            c.w64(3, c.r64(0));
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af44a); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af450); };
            c.cmp(t20, c.r32(7), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af459); };
            c.w32(7, t21);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008af471, 0x70008af46c);
        }
        27 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af471); };
            c.w64(6, t22);
            blk = 21; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008af477); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            c.w64(0, c.r64(3));
            blk = 21; continue;
        }
        29 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            blk = 35; continue;
        }
        30 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af486); };
            c.w64(0, t26);
            blk = 31; continue;
        }
        31 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af48a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af48b); }
            c.w32(1, 0x10u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af491); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008af492); };
            c.w64(3, t27);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008af4a3, 0x70008af49e);
        }
        32 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008af4a6); }
            return c.ret(0x0, 0x70008af4a7);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af4a8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(3, c.r32(3));
            blk = 35; continue;
        }
        35 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af4ba); };
            let r29 = c.r32(3) ^ t28;
            c.w32(3, r29);
            c.logic(r29, 32);
            c.w64(1, c.r64(3));
            let r30 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r30);
            if c.cc_o() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 32; continue;
        }
        37 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008af4c9); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008af4d0); };
            c.w64(7, t32);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008af4d7); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008af4db); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008af4e2); };
            c.cmp(t34, c.r64(8), 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008af4f1, 0x70008af4ec);
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af4f1); };
            c.w64(3, t35);
            blk = 41; continue;
        }
        40 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008af4f7); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 41; continue;
        }
        41 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af501); };
            c.w64(1, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008af508); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008af513); }
            blk = 32; continue;
        }
        42 => {
            let r37 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008af51e); }
            return c.call(0x70008708b4, 0x70008af528, 0x70008af523);
        }
        43 => {
            let Some(t38) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008af528); };
            c.w32(3, t38);
            let r39 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r39);
            blk = 34; continue;
        }
        44 => {
            let r40 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r40);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008af535); }
            return c.call(0x70008708b4, 0x70008af53f, 0x70008af53a);
        }
        45 => {
            let Some(t41) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008af53f); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r42);
            blk = 19; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008af54c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008af554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af559); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af55d); };
            c.w64(6, t1);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008af564); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af572); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af578); };
            c.cmp(t4, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af58c); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 41; continue;
        }
        7 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 44; continue;
        }
        9 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af5b5); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af5bb); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 47; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af5c9); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af5ce); }
            blk = 15; continue;
        }
        13 => {
            let r10 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r10);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af5d7); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af5e8); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008af601); };
            c.w64(0, t12);
            return c.ret(0x0, 0x70008af608);
        }
        19 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008af609); };
            c.w64(0, t13);
            return c.ret(0x0, 0x70008af610);
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008af611); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008af615); }
            let r14 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r14);
            let Some(t15) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af61b); };
            c.w32(7, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008af61f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af623); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008af627); }
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(7), 0xbau64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x70008af64a, 0x70008af645);
        }
        24 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af64a); };
            c.w64(6, t16);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008af64e); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af652); };
            c.w64(0, t17);
            return c.call(0x70008a10d4, 0x70008af65b, 0x70008af656);
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af65b); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af65f); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        26 => {
            return c.call(0x70008a2474, 0x70008af66d, 0x70008af668);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af66d); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af671); };
            c.w64(0, t19);
            blk = 1; continue;
        }
        28 => {
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af67a); };
            c.w32(1, t20);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 52; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(1), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            return c.call(0x70008a10d4, 0x70008af69e, 0x70008af699);
        }
        32 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af69e); };
            c.w64(6, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af6a2); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af6a6); };
            c.w64(0, t22);
            return c.call(0x70008a11d4, 0x70008af6af, 0x70008af6aa);
        }
        33 => {
            blk = 1; continue;
        }
        34 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x70008af6bc, 0x70008af6b7);
        }
        35 => {
            blk = 1; continue;
        }
        36 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af6c1); };
            c.cmp(t23, c.r32(1), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t24) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af6ce); };
            c.w32(1, t24);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 54; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008af6ee, 0x70008af6e9);
        }
        40 => {
            blk = 1; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008af6f3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008af6f7); }
            let r25 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af6fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008af701); }
            let Some(t26) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af705); };
            c.cmp(t26, 0x41u64, 16);
            if c.cc_e() { blk = 60; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008af718, 0x70008af713);
        }
        43 => {
            blk = 1; continue;
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008af71d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008af721); }
            let r27 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r27);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008af727); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008af72b); }
            let Some(t28) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af72f); };
            c.cmp(t28, 0x41u64, 16);
            if c.cc_e() { blk = 62; continue; }
            blk = 45; continue;
        }
        45 => {
            return c.call(0x70008a10d4, 0x70008af73f, 0x70008af73a);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af73f); }
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af743); };
            c.w64(0, t29);
            blk = 1; continue;
        }
        47 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008af74c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008af750); }
            let r30 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r30);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af756); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008af75a); }
            let Some(t31) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af75e); };
            c.cmp(t31, 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008af771, 0x70008af76c);
        }
        49 => {
            blk = 1; continue;
        }
        50 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af77b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af77c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af77d); }
            c.w32(0, 0x3u64);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008af783); };
            c.w64(3, t32);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008af794, 0x70008af78f);
        }
        51 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008af797); }
            return c.ret(0x0, 0x70008af798);
        }
        52 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb9f4, 0x70008af7a4, 0x70008af79f);
        }
        53 => {
            blk = 51; continue;
        }
        54 => {
            c.w32(1, 0x24u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af7ab); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af7ac); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af7ad); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008af7ae); };
            c.w64(3, t33);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008af7c1, 0x70008af7bc);
        }
        55 => {
            blk = 51; continue;
        }
        56 => {
            c.w32(1, 0x24u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af7c8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af7c9); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af7ca); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x70008af7cb); };
            c.w64(3, t34);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008af7de, 0x70008af7d9);
        }
        57 => {
            blk = 51; continue;
        }
        58 => {
            c.w32(1, 0x24u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af7e5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af7e6); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af7e7); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008af7ea); };
            c.w64(3, t35);
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008af7fe, 0x70008af7f9);
        }
        59 => {
            blk = 51; continue;
        }
        60 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af805); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af806); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af807); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008af808); };
            c.w64(3, t36);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008af81b, 0x70008af816);
        }
        61 => {
            blk = 51; continue;
        }
        62 => {
            c.w32(1, 0x24u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af825); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af826); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af827); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008af828); };
            c.w64(3, t37);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008af83b, 0x70008af836);
        }
        63 => {
            blk = 51; continue;
        }
        64 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008af845); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008af846); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008af847); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008af848); };
            c.w64(3, t38);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008af85b, 0x70008af856);
        }
        65 => {
            blk = 51; continue;
        }
        66 => {
            return RegfileExit::jump(0x70008af864);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008af874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af879); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af87d); };
            c.w64(6, t1);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008af884); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af892); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af898); };
            c.cmp(t4, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af8ac); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 41; continue;
        }
        7 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 44; continue;
        }
        9 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008af8d5); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af8db); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 47; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af8e9); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af8ee); }
            blk = 15; continue;
        }
        13 => {
            let r10 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r10);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af8f7); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008af908); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008af921); };
            c.w64(0, t12);
            return c.ret(0x0, 0x70008af928);
        }
        19 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008af929); };
            c.w64(0, t13);
            return c.ret(0x0, 0x70008af930);
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008af931); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008af935); }
            let r14 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r14);
            let Some(t15) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af93b); };
            c.w32(7, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008af93f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af943); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008af947); }
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(7), 0xbau64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x70008af96a, 0x70008af965);
        }
        24 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af96a); };
            c.w64(6, t16);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008af96e); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008af972); };
            c.w64(0, t17);
            return c.call(0x70008a10d4, 0x70008af97b, 0x70008af976);
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af97b); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af97f); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        26 => {
            return c.call(0x70008a2474, 0x70008af98d, 0x70008af988);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af98d); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af991); };
            c.w64(0, t19);
            blk = 1; continue;
        }
        28 => {
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af99a); };
            c.w32(1, t20);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 52; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(1), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            return c.call(0x70008a10d4, 0x70008af9be, 0x70008af9b9);
        }
        32 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008af9be); };
            c.w64(6, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008af9c2); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008af9c6); };
            c.w64(0, t22);
            return c.call(0x70008a11d4, 0x70008af9cf, 0x70008af9ca);
        }
        33 => {
            blk = 1; continue;
        }
        34 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x70008af9dc, 0x70008af9d7);
        }
        35 => {
            blk = 1; continue;
        }
        36 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008af9e1); };
            c.cmp(t23, c.r32(1), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t24) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008af9ee); };
            c.w32(1, t24);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 54; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008afa0e, 0x70008afa09);
        }
        40 => {
            blk = 1; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008afa13); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008afa17); }
            let r25 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afa1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008afa21); }
            let Some(t26) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afa25); };
            c.cmp(t26, 0x41u64, 16);
            if c.cc_e() { blk = 60; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008afa38, 0x70008afa33);
        }
        43 => {
            blk = 1; continue;
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008afa3d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008afa41); }
            let r27 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r27);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008afa47); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008afa4b); }
            let Some(t28) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afa4f); };
            c.cmp(t28, 0x41u64, 16);
            if c.cc_e() { blk = 62; continue; }
            blk = 45; continue;
        }
        45 => {
            return c.call(0x70008a10d4, 0x70008afa5f, 0x70008afa5a);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afa5f); }
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008afa63); };
            c.w64(0, t29);
            blk = 1; continue;
        }
        47 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008afa6c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008afa70); }
            let r30 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r30);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afa76); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008afa7a); }
            let Some(t31) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afa7e); };
            c.cmp(t31, 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008afa91, 0x70008afa8c);
        }
        49 => {
            blk = 1; continue;
        }
        50 => {
            c.w32(1, 0x2au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afa9b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afa9c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afa9d); }
            c.w32(0, 0x3u64);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008afaa3); };
            c.w64(3, t32);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008afab4, 0x70008afaaf);
        }
        51 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008afab7); }
            return c.ret(0x0, 0x70008afab8);
        }
        52 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fbb34, 0x70008afac4, 0x70008afabf);
        }
        53 => {
            blk = 51; continue;
        }
        54 => {
            c.w32(1, 0x26u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afacb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afacc); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afacd); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008aface); };
            c.w64(3, t33);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afae1, 0x70008afadc);
        }
        55 => {
            blk = 51; continue;
        }
        56 => {
            c.w32(1, 0x26u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afae8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afae9); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afaea); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x70008afaeb); };
            c.w64(3, t34);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afafe, 0x70008afaf9);
        }
        57 => {
            blk = 51; continue;
        }
        58 => {
            c.w32(1, 0x26u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afb05); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afb06); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afb07); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afb0a); };
            c.w64(3, t35);
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008afb1e, 0x70008afb19);
        }
        59 => {
            blk = 51; continue;
        }
        60 => {
            c.w32(1, 0x2au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afb25); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afb26); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afb27); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afb28); };
            c.w64(3, t36);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afb3b, 0x70008afb36);
        }
        61 => {
            blk = 51; continue;
        }
        62 => {
            c.w32(1, 0x26u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afb45); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afb46); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afb47); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afb48); };
            c.w64(3, t37);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afb5b, 0x70008afb56);
        }
        63 => {
            blk = 51; continue;
        }
        64 => {
            c.w32(1, 0x2au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afb65); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afb66); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afb67); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afb68); };
            c.w64(3, t38);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afb7b, 0x70008afb76);
        }
        65 => {
            blk = 51; continue;
        }
        66 => {
            return RegfileExit::jump(0x70008afb84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008afb94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008afb99); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008afb9d); };
            c.w64(6, t1);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008afba4); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008afbb2); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008afbb8); };
            c.cmp(t4, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008afbcc); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 41; continue;
        }
        7 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 44; continue;
        }
        9 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008afbf5); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008afbfb); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 47; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008afc09); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008afc0e); }
            blk = 15; continue;
        }
        13 => {
            let r10 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r10);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008afc17); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008afc28); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008afc41); };
            c.w64(0, t12);
            return c.ret(0x0, 0x70008afc48);
        }
        19 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008afc49); };
            c.w64(0, t13);
            return c.ret(0x0, 0x70008afc50);
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008afc51); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008afc55); }
            let r14 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r14);
            let Some(t15) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afc5b); };
            c.w32(7, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008afc5f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afc63); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008afc67); }
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(7), 0xbau64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x70008afc8a, 0x70008afc85);
        }
        24 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008afc8a); };
            c.w64(6, t16);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008afc8e); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008afc92); };
            c.w64(0, t17);
            return c.call(0x70008a10d4, 0x70008afc9b, 0x70008afc96);
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afc9b); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008afc9f); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        26 => {
            return c.call(0x70008a2474, 0x70008afcad, 0x70008afca8);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afcad); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008afcb1); };
            c.w64(0, t19);
            blk = 1; continue;
        }
        28 => {
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afcba); };
            c.w32(1, t20);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 52; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(1), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            return c.call(0x70008a10d4, 0x70008afcde, 0x70008afcd9);
        }
        32 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008afcde); };
            c.w64(6, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afce2); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008afce6); };
            c.w64(0, t22);
            return c.call(0x70008a11d4, 0x70008afcef, 0x70008afcea);
        }
        33 => {
            blk = 1; continue;
        }
        34 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x70008afcfc, 0x70008afcf7);
        }
        35 => {
            blk = 1; continue;
        }
        36 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008afd01); };
            c.cmp(t23, c.r32(1), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t24) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afd0e); };
            c.w32(1, t24);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 54; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008afd2e, 0x70008afd29);
        }
        40 => {
            blk = 1; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008afd33); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008afd37); }
            let r25 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afd3d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008afd41); }
            let Some(t26) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afd45); };
            c.cmp(t26, 0x41u64, 16);
            if c.cc_e() { blk = 60; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008afd58, 0x70008afd53);
        }
        43 => {
            blk = 1; continue;
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008afd5d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008afd61); }
            let r27 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r27);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008afd67); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008afd6b); }
            let Some(t28) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afd6f); };
            c.cmp(t28, 0x41u64, 16);
            if c.cc_e() { blk = 62; continue; }
            blk = 45; continue;
        }
        45 => {
            return c.call(0x70008a10d4, 0x70008afd7f, 0x70008afd7a);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afd7f); }
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008afd83); };
            c.w64(0, t29);
            blk = 1; continue;
        }
        47 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008afd8c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008afd90); }
            let r30 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r30);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008afd96); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008afd9a); }
            let Some(t31) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008afd9e); };
            c.cmp(t31, 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008afdb1, 0x70008afdac);
        }
        49 => {
            blk = 1; continue;
        }
        50 => {
            c.w32(1, 0x24u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afdbb); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afdbc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afdbd); }
            c.w32(0, 0x3u64);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008afdc3); };
            c.w64(3, t32);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008afdd4, 0x70008afdcf);
        }
        51 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008afdd7); }
            return c.ret(0x0, 0x70008afdd8);
        }
        52 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb774, 0x70008afde4, 0x70008afddf);
        }
        53 => {
            blk = 51; continue;
        }
        54 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afdeb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afdec); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afded); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008afdee); };
            c.w64(3, t33);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afe01, 0x70008afdfc);
        }
        55 => {
            blk = 51; continue;
        }
        56 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afe08); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afe09); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afe0a); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x70008afe0b); };
            c.w64(3, t34);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afe1e, 0x70008afe19);
        }
        57 => {
            blk = 51; continue;
        }
        58 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afe25); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afe26); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afe27); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afe2a); };
            c.w64(3, t35);
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008afe3e, 0x70008afe39);
        }
        59 => {
            blk = 51; continue;
        }
        60 => {
            c.w32(1, 0x24u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afe45); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afe46); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afe47); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afe48); };
            c.w64(3, t36);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afe5b, 0x70008afe56);
        }
        61 => {
            blk = 51; continue;
        }
        62 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afe65); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afe66); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afe67); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afe68); };
            c.w64(3, t37);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afe7b, 0x70008afe76);
        }
        63 => {
            blk = 51; continue;
        }
        64 => {
            c.w32(1, 0x24u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008afe85); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008afe86); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008afe87); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008afe88); };
            c.w64(3, t38);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008afe9b, 0x70008afe96);
        }
        65 => {
            blk = 51; continue;
        }
        66 => {
            return RegfileExit::jump(0x70008afea4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008afeb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            blk = 2; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008afeb9); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008afebd); };
            c.w64(6, t1);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008afec4); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008afed2); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008afed8); };
            c.cmp(t4, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008afeec); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 41; continue;
        }
        7 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 44; continue;
        }
        9 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008aff15); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008aff1b); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 47; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aff29); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aff2e); }
            blk = 15; continue;
        }
        13 => {
            let r10 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r10);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aff37); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008aff48); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008aff61); };
            c.w64(0, t12);
            return c.ret(0x0, 0x70008aff68);
        }
        19 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008aff69); };
            c.w64(0, t13);
            return c.ret(0x0, 0x70008aff70);
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008aff71); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008aff75); }
            let r14 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r14);
            let Some(t15) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008aff7b); };
            c.w32(7, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008aff7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008aff83); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008aff87); }
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(7), 0xbau64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a11d4, 0x70008affaa, 0x70008affa5);
        }
        24 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008affaa); };
            c.w64(6, t16);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008affae); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008affb2); };
            c.w64(0, t17);
            return c.call(0x70008a10d4, 0x70008affbb, 0x70008affb6);
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008affbb); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008affbf); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        26 => {
            return c.call(0x70008a2474, 0x70008affcd, 0x70008affc8);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008affcd); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008affd1); };
            c.w64(0, t19);
            blk = 1; continue;
        }
        28 => {
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008affda); };
            c.w32(1, t20);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 52; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(1), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            return c.call(0x70008a10d4, 0x70008afffe, 0x70008afff9);
        }
        32 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008afffe); };
            c.w64(6, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008b0002); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b0006); };
            c.w64(0, t22);
            return c.call(0x70008a11d4, 0x70008b000f, 0x70008b000a);
        }
        33 => {
            blk = 1; continue;
        }
        34 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2474, 0x70008b001c, 0x70008b0017);
        }
        35 => {
            blk = 1; continue;
        }
        36 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b0021); };
            c.cmp(t23, c.r32(1), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t24) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b002e); };
            c.w32(1, t24);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 54; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008b004e, 0x70008b0049);
        }
        40 => {
            blk = 1; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b0053); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b0057); }
            let r25 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008b005d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b0061); }
            let Some(t26) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0065); };
            c.cmp(t26, 0x41u64, 16);
            if c.cc_e() { blk = 60; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008b0078, 0x70008b0073);
        }
        43 => {
            blk = 1; continue;
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b007d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b0081); }
            let r27 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r27);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008b0087); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b008b); }
            let Some(t28) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b008f); };
            c.cmp(t28, 0x41u64, 16);
            if c.cc_e() { blk = 62; continue; }
            blk = 45; continue;
        }
        45 => {
            return c.call(0x70008a10d4, 0x70008b009f, 0x70008b009a);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008b009f); }
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b00a3); };
            c.w64(0, t29);
            blk = 1; continue;
        }
        47 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b00ac); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b00b0); }
            let r30 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r30);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008b00b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b00ba); }
            let Some(t31) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b00be); };
            c.cmp(t31, 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x70008b00d1, 0x70008b00cc);
        }
        49 => {
            blk = 1; continue;
        }
        50 => {
            c.w32(1, 0x26u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b00db); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b00dc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b00dd); }
            c.w32(0, 0x3u64);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008b00e3); };
            c.w64(3, t32);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008b00f4, 0x70008b00ef);
        }
        51 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b00f7); }
            return c.ret(0x0, 0x70008b00f8);
        }
        52 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb8b4, 0x70008b0104, 0x70008b00ff);
        }
        53 => {
            blk = 51; continue;
        }
        54 => {
            c.w32(1, 0x2au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b010b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b010c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b010d); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008b010e); };
            c.w64(3, t33);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b0121, 0x70008b011c);
        }
        55 => {
            blk = 51; continue;
        }
        56 => {
            c.w32(1, 0x2au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b0128); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b0129); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b012a); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x70008b012b); };
            c.w64(3, t34);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b013e, 0x70008b0139);
        }
        57 => {
            blk = 51; continue;
        }
        58 => {
            c.w32(1, 0x2au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b0145); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b0146); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b0147); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008b014a); };
            c.w64(3, t35);
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008b015e, 0x70008b0159);
        }
        59 => {
            blk = 51; continue;
        }
        60 => {
            c.w32(1, 0x26u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b0165); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b0166); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b0167); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008b0168); };
            c.w64(3, t36);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b017b, 0x70008b0176);
        }
        61 => {
            blk = 51; continue;
        }
        62 => {
            c.w32(1, 0x2au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b0185); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b0186); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b0187); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008b0188); };
            c.w64(3, t37);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b019b, 0x70008b0196);
        }
        63 => {
            blk = 51; continue;
        }
        64 => {
            c.w32(1, 0x26u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b01a5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b01a6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b01a7); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008b01a8); };
            c.w64(3, t38);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b01bb, 0x70008b01b6);
        }
        65 => {
            blk = 51; continue;
        }
        66 => {
            return RegfileExit::jump(0x70008b01c4);
        }
            _ => unreachable!(),
        }
    }
}
