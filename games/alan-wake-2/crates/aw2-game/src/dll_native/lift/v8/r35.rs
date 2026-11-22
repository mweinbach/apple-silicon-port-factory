#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000918614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000918614); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000918618); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000918622); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000918628); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091862c); };
            c.w64(2, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000918630); };
            c.w64(8, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918636); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091863c); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918640); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918646); };
            c.w32(11, (t10 as u16 as i16 as i64 as u64));
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091864c); };
            c.w32(0, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918652); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918658); };
            c.w32(12, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700091865c); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000918660); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000918669); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000918670); };
            c.w64(12, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918674); };
            c.w32(15, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000918678); };
            c.cmp(t18, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(9).wrapping_mul(4));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918695); };
            c.w32(3, t19);
            let r20 = c.add(c.r32(3), 0x8u64, 32);
            c.w32(3, r20);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x70009186a0); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009186a5); };
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
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009186bb); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70009186c8); };
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
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009186de); };
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
            let Some(t29) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918710); };
            c.w32(0, t29);
            let r30 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r30);
            let Some(t31) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918718); };
            c.w32(0, t31);
            let r32 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r32);
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091871e); };
            c.w32(6, t33);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700091872d);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700091872e); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(0, c.r32(3));
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091873d); };
            c.w64(3, t36);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091874c); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000918771); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918776); }
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
            let Some(t42) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091878b); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            blk = 20; continue;
        }
        20 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918791); };
            c.w32(3, t44);
            let r45 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r45);
            let Some(t46) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918797); };
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
            let Some(t48) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009187b4); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009187ba); };
            c.w32(3, t50);
            let r51 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009187c2); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009187c8); };
            c.w32(3, t54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009187cb); };
            c.cmp(t55, c.r32(3), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(15, c.r64(8));
            let r56 = c.r64(15) | 0x3u64;
            c.w64(15, r56);
            c.logic(r56, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x70009187db); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r57 = c.r64(0) & c.r64(2);
            c.w64(0, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009187ef); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009187f9); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091880f); };
            c.w32(0, t59);
            let r60 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r60);
            let Some(t61) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918816); };
            c.w32(0, t61);
            let r62 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r62);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918828); };
            c.w32(0, t63);
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918834); };
            c.w32(3, t64);
            let r65 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r65);
            let Some(t66) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091883a); };
            c.w32(3, t66);
            let r67 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r67);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091884d); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(3)) { return RegfileExit::fault(0x700091885f); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r71 = c.r64(15) & c.r64(2);
            c.w64(15, r71);
            c.logic(r71, 64);
            let Some(t72) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918872); };
            c.logic(t72 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091887d); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000918884); };
            c.w64(0, t73);
            blk = 55; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091888d); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x70009188ad); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009188b2); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009188b9); };
            c.w32(3, t78);
            let r79 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r79);
            let Some(t80) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009188c1); };
            c.w32(3, t80);
            let r81 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r81);
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009188c7); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x70009188d6);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(3).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009188d7); };
            c.cmp(t83, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(3, c.r64(8));
            blk = 45; continue;
        }
        41 => {
            let Some(t84) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009188e2); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x7000918902); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918907); }
            blk = 44; continue;
        }
        44 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091890e); };
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
            let Some(t89) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000918923); };
            c.w32(6, t89);
            let r90 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r90);
            blk = 48; continue;
        }
        48 => {
            let Some(t91) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918929); };
            c.w32(7, t91);
            let r92 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r92);
            let Some(t93) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091892f); };
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
            let Some(t95) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000918948); };
            c.w32(6, t95);
            let r96 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r96);
            let Some(t97) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091894e); };
            c.w32(7, t97);
            let r98 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r98);
            let Some(t99) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918956); };
            c.w32(6, t99);
            let r100 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r100);
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091895c); };
            c.w32(7, t101);
            let Some(t102) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091895f); };
            c.cmp(t102, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r103 = c.r64(3) | 0x3u64;
            c.w64(3, r103);
            c.logic(r103, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(3)) { return RegfileExit::fault(0x7000918968); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r104 = c.r64(15) & c.r64(2);
            c.w64(15, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091897b); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918982); }
            blk = 55; continue;
        }
        54 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091898b); };
            c.w64(9, t106);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x7000918992); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918997); }
            blk = 55; continue;
        }
        55 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t107) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009189a5); };
            c.w64(5, t107);
            let Some(t108) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70009189a9); };
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
            let Some(t112) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009189cb); };
            c.logic(t112 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009189d6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009189d7); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70009189e4, 0x70009189df);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009189e4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009189e6); }
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
            let Some(t115) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009189f8); };
            c.logic(t115 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000918a07); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000918a08); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x7000918a15, 0x7000918a10);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918a15); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918a17); }
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
            let Some(t118) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918a2d); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(3, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000918a3b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000918a3c); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(3));
            return c.call(0x70007f1594, 0x7000918a49, 0x7000918a44);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918a49); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918a4b); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000918a54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000918a74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            c.w64(2, c.r64(5));
            let r0 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r0);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000918a7d); }
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000918a81); };
            c.w64(3, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000918a85); };
            c.w64(1, (t2 as u16 as i16 as i64 as u64));
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918a8b); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x7000918a91); };
            c.w64(1, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918a95); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918a9b); };
            c.w32(8, t8);
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918aa1); };
            c.w32(9, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000918aa5); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000918aae); };
            c.w64(3, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000918ab5); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918ab9); };
            c.w32(9, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000918abd); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t15) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918ada); };
            c.w32(11, t15);
            let r16 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r16);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000918ae6); }
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000918aeb); };
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
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000918b03); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000918b10); };
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
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918b27); };
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
            let Some(t25) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918b5c); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918b63); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918b6b); };
            c.w32(0, t29);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000918b7c);
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000918b7d); };
            c.cmp(t30, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(11, c.r32(11));
            let r31 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r31);
            c.logic(r31, 32);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000918b8f); };
            c.w64(12, t32);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t33) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918b9f); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000918bcb); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918bd0); }
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
            let Some(t38) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000918be6); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            blk = 20; continue;
        }
        20 => {
            let Some(t40) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918bed); };
            c.w32(12, t40);
            let r41 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r41);
            let Some(t42) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918bf4); };
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
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000918c17); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918c1e); };
            c.w32(12, t46);
            let r47 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918c25); };
            c.w32(11, t48);
            let r49 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r49);
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918c2c); };
            c.w32(12, t50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918c31); };
            c.cmp(t51, c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(1));
            let r52 = c.r64(9) | 0x3u64;
            c.w64(9, r52);
            c.logic(r52, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x7000918c42); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r53 = c.r64(11) & c.r64(3);
            c.w64(11, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918c56); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918c61); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t55) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918c76); };
            c.w32(11, t55);
            let r56 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r56);
            let Some(t57) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918c7d); };
            c.w32(11, t57);
            let r58 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r58);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918c93); };
            c.w32(11, t59);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t60) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918ca0); };
            c.w32(12, t60);
            let r61 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r61);
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918ca7); };
            c.w32(12, t62);
            let r63 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r63);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t64) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918cbe); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x7000918cd2); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r67 = c.r64(9) & c.r64(3);
            c.w64(9, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918ce6); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918cf1); }
            blk = 55; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918cfd); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000918d22); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918d27); }
            blk = 37; continue;
        }
        37 => {
            let Some(t73) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918d2e); };
            c.w32(11, t73);
            let r74 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r74);
            let Some(t75) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918d35); };
            c.w32(11, t75);
            let r76 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r76);
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918d3c); };
            c.w32(12, t77);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x7000918d4c);
        }
        39 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000918d4d); };
            c.cmp(t78, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(11, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t79) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918d58); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x7000918d7d); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918d82); }
            blk = 44; continue;
        }
        44 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000918d89); };
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
            let Some(t84) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000918d9f); };
            c.w32(12, t84);
            let r85 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r85);
            blk = 48; continue;
        }
        48 => {
            let Some(t86) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918da7); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918daf); };
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
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000918dcd); };
            c.w32(12, t90);
            let r91 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r91);
            let Some(t92) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918dd5); };
            c.w32(15, t92);
            let r93 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r93);
            let Some(t94) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918ddc); };
            c.w32(12, t94);
            let r95 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r95);
            let Some(t96) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918de4); };
            c.w32(15, t96);
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918de8); };
            c.cmp(t97, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r98 = c.r64(11) | 0x3u64;
            c.w64(11, r98);
            c.logic(r98, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000918df3); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r99 = c.r64(9) & c.r64(3);
            c.w64(9, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918e07); };
            c.logic(t100 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918e0e); }
            blk = 55; continue;
        }
        54 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000918e17); };
            c.w64(8, t101);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000918e1e); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918e23); }
            blk = 55; continue;
        }
        55 => {
            let Some(t102) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000918e2a); };
            if !c.push64(t102) { return RegfileExit::fault(0x7000918e2a); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(2));
            c.w64(7, c.r64(1));
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000918e35); };
            c.w64(10, t103);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x7000918e3c); }
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
            let Some(t106) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918e56); };
            c.logic(t106 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000918e61); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000918e62); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000918e6f, 0x7000918e6a);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918e6f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918e71); }
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
            let Some(t109) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918e84); };
            c.logic(t109 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000918e93); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000918e94); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x7000918ea1, 0x7000918e9c);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918ea1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918ea3); }
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
            let Some(t112) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918eb9); };
            c.logic(t112 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000918ec8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000918ec9); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000918ed6, 0x7000918ed1);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918ed6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918ed8); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000918ee0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000918ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000918ef4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000918ef8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000918f02); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000918f08); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000918f0c); };
            c.w64(3, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000918f10); };
            c.w64(1, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918f16); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x7000918f1c); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918f20); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000918f26); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918f2c); };
            c.w32(11, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000918f30); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000918f34); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000918f3d); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000918f44); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918f48); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000918f4c); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918f69); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000918f74); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000918f79); };
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
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000918f8e); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000918f9b); };
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
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918fb1); };
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
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918fe4); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            let Some(t29) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918feb); };
            c.w32(2, t29);
            let r30 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r30);
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918ff1); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000919000);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000919001); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            let r33 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r33);
            c.logic(r33, 32);
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091900f); };
            c.w64(0, t34);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091901e); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x7000919043); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919048); }
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
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091905d); };
            c.w32(2, t40);
            let r41 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r41);
            blk = 20; continue;
        }
        20 => {
            let Some(t42) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919063); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            let Some(t44) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919069); };
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
            let Some(t46) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000919084); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091908a); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919091); };
            c.w32(2, t50);
            let r51 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919097); };
            c.w32(0, t52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091909a); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(2, c.r64(1));
            let r54 = c.r64(2) | 0x3u64;
            c.w64(2, r54);
            c.logic(r54, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x70009190aa); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r55 = c.r64(15) & c.r64(3);
            c.w64(15, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009190bd); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009190c8); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009190dd); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009190e3); };
            c.w32(2, t59);
            let r60 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r60);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009190f6); };
            c.w32(2, t61);
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919101); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r63);
            let Some(t64) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919107); };
            c.w32(0, t64);
            let r65 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r65);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919119); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700091912b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r69 = c.r64(2) & c.r64(3);
            c.w64(2, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091913d); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919147); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091914e); };
            c.w64(2, t71);
            blk = 55; continue;
        }
        34 => {
            let Some(t72) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000919157); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000919176); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091917b); }
            blk = 37; continue;
        }
        37 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919182); };
            c.w32(0, t76);
            let r77 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r77);
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919189); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091918f); };
            c.w32(6, t80);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700091919e);
        }
        39 => {
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700091919f); };
            c.cmp(t81, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009191a9); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009191c8); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009191cd); }
            blk = 44; continue;
        }
        44 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009191d4); };
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
            let Some(t87) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009191e8); };
            c.w32(6, t87);
            let r88 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r88);
            blk = 48; continue;
        }
        48 => {
            let Some(t89) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009191ee); };
            c.w32(7, t89);
            let r90 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r90);
            let Some(t91) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009191f4); };
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
            let Some(t93) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091920d); };
            c.w32(6, t93);
            let r94 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r94);
            let Some(t95) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919213); };
            c.w32(7, t95);
            let r96 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r96);
            let Some(t97) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091921a); };
            c.w32(6, t97);
            let r98 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r98);
            let Some(t99) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919220); };
            c.w32(7, t99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919223); };
            c.cmp(t100, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r101 = c.r64(0) | 0x3u64;
            c.w64(0, r101);
            c.logic(r101, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700091922c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r102 = c.r64(15) & c.r64(3);
            c.w64(15, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091923e); };
            c.logic(t103 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919245); }
            blk = 55; continue;
        }
        54 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091924e); };
            c.w64(2, t104);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000919255); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091925a); }
            blk = 55; continue;
        }
        55 => {
            let Some(t105) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919261); };
            c.w64(3, (t105 as u16 as i16 as i64 as u64));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000919267); };
            c.w64(2, t106);
            let Some(t107) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091926b); };
            c.w64(3, t107);
            let Some(t108) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091926f); };
            c.w64(5, t108);
            let r109 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r109);
            let Some(t110) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000919277); };
            if !c.push64(t110) { return RegfileExit::fault(0x7000919277); }
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x7000919286); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091928b); };
            c.w64(10, t111);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x7000919292); }
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
            let Some(t114) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009192ab); };
            c.logic(t114 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009192b6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009192b7); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70009192c4, 0x70009192bf);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009192c4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009192c6); }
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
            let Some(t117) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009192db); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009192e9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009192ea); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x70009192f7, 0x70009192f2);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009192f7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009192f9); }
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
            let Some(t120) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091930e); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091931d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091931e); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x700091932b, 0x7000919326);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091932b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091932d); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000919334);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000919354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000919354); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000919358); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000919362); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000919368); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091936c); };
            c.w64(3, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000919370); };
            c.w64(1, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919376); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700091937c); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919380); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919386); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091938c); };
            c.w32(11, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000919390); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000919394); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091939d); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009193a4); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009193a8); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009193ac); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009193c9); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009193d4); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009193d9); };
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
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009193ee); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70009193fb); };
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
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000919411); };
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
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919444); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            let Some(t29) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091944b); };
            c.w32(2, t29);
            let r30 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r30);
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919451); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000919460);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000919461); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            let r33 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r33);
            c.logic(r33, 32);
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091946f); };
            c.w64(0, t34);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091947e); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x70009194a3); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009194a8); }
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
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009194bd); };
            c.w32(2, t40);
            let r41 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r41);
            blk = 20; continue;
        }
        20 => {
            let Some(t42) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009194c3); };
            c.w32(0, t42);
            let r43 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r43);
            let Some(t44) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009194c9); };
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
            let Some(t46) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009194e4); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009194ea); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009194f1); };
            c.w32(2, t50);
            let r51 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009194f7); };
            c.w32(0, t52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009194fa); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(2, c.r64(1));
            let r54 = c.r64(2) | 0x3u64;
            c.w64(2, r54);
            c.logic(r54, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700091950a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r55 = c.r64(15) & c.r64(3);
            c.w64(15, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091951d); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919528); }
            blk = 55; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091953d); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919543); };
            c.w32(2, t59);
            let r60 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r60);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919556); };
            c.w32(2, t61);
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919561); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r63);
            let Some(t64) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919567); };
            c.w32(0, t64);
            let r65 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r65);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919579); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700091958b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r69 = c.r64(2) & c.r64(3);
            c.w64(2, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091959d); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009195a7); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009195ae); };
            c.w64(2, t71);
            blk = 55; continue;
        }
        34 => {
            let Some(t72) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009195b7); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x70009195d6); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009195db); }
            blk = 37; continue;
        }
        37 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009195e2); };
            c.w32(0, t76);
            let r77 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r77);
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009195e9); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009195ef); };
            c.w32(6, t80);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x70009195fe);
        }
        39 => {
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009195ff); };
            c.cmp(t81, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 45; continue;
        }
        41 => {
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000919609); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000919628); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091962d); }
            blk = 44; continue;
        }
        44 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000919634); };
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
            let Some(t87) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000919648); };
            c.w32(6, t87);
            let r88 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r88);
            blk = 48; continue;
        }
        48 => {
            let Some(t89) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091964e); };
            c.w32(7, t89);
            let r90 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r90);
            let Some(t91) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919654); };
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
            let Some(t93) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091966d); };
            c.w32(6, t93);
            let r94 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r94);
            let Some(t95) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919673); };
            c.w32(7, t95);
            let r96 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r96);
            let Some(t97) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091967a); };
            c.w32(6, t97);
            let r98 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r98);
            let Some(t99) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919680); };
            c.w32(7, t99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919683); };
            c.cmp(t100, c.r32(7), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r101 = c.r64(0) | 0x3u64;
            c.w64(0, r101);
            c.logic(r101, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700091968c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r102 = c.r64(15) & c.r64(3);
            c.w64(15, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091969e); };
            c.logic(t103 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009196a5); }
            blk = 55; continue;
        }
        54 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009196ae); };
            c.w64(2, t104);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70009196b5); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009196ba); }
            blk = 55; continue;
        }
        55 => {
            let Some(t105) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009196c1); };
            c.w64(3, (t105 as u16 as i16 as i64 as u64));
            let Some(t106) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009196c7); };
            c.w64(2, (t106 as u16 as i16 as i64 as u64));
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009196cd); };
            c.w64(8, t107);
            let Some(t108) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009196d1); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009196d5); };
            c.w64(2, t109);
            let Some(t110) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009196d9); };
            c.w64(5, t110);
            let r111 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r111);
            let Some(t112) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70009196e1); };
            if !c.push64(t112) { return RegfileExit::fault(0x70009196e1); }
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x70009196f0); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70009196f5); }
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009196fa); };
            c.w64(10, t113);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x7000919701); }
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
            let Some(t116) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091971a); };
            c.logic(t116 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000919725); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000919726); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000919733, 0x700091972e);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000919733); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000919735); }
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
            let Some(t119) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091974a); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000919758); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000919759); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x7000919766, 0x7000919761);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000919766); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000919768); }
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
            let Some(t122) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091977d); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091978c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091978d); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f1594, 0x700091979a, 0x7000919795);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091979a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091979c); }
            blk = 25; continue;
        }
        65 => {
            return RegfileExit::jump(0x70009197a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009197b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009197b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009197b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009197c2); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009197c8); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009197cc); };
            c.w64(2, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009197d0); };
            c.w64(8, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009197d6); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009197dc); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009197e0); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009197e6); };
            c.w32(11, (t10 as u16 as i16 as i64 as u64));
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009197ec); };
            c.w32(15, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009197f2); };
            c.w32(0, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009197f8); };
            c.w32(3, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70009197fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009197ff); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000919803); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091980c); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919813); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919817); };
            c.w32(6, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091981a); };
            c.cmp(t18, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000919837); };
            c.w32(7, t19);
            let r20 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x7000919841); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000919845); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, c.r32(7));
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000919853); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000919860); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700091986d); };
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
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000919883); };
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
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009198b8); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009198bf); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009198c6); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009198d6);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009198d7); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t35) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009198e1); };
            c.w64(9, (t35 as u16 as i16 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t36) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70009198ea); };
            c.w64(9, t36);
            let r37 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009198fb); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x7000919927); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091992c); }
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
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000919942); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            blk = 20; continue;
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919949); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919950); };
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
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000919972); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919979); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919980); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919987); };
            c.w32(11, t55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091998b); };
            c.cmp(t56, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r57 = c.r64(9) | 0x3u64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091999c); };
            c.w64(15, t58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x70009199a0); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(2);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009199b4); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009199bf); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009199c6); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        26 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009199cf); };
            c.w64(11, t62);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009199e2); };
            c.w32(9, t63);
            let r64 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r64);
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009199e9); };
            c.w32(9, t65);
            let r66 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r66);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009199ff); };
            c.w32(9, t67);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919a08); };
            c.w32(12, t68);
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919a0f); };
            c.w32(12, t70);
            let r71 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r71);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919a26); };
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
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000919a3a); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x7000919a3e); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r76 = c.r64(9) & c.r64(2);
            c.w64(9, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000919a52); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919a5d); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000919a64); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        35 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000919a6d); };
            c.w64(11, t79);
            blk = 56; continue;
        }
        36 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000919a76); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x7000919a99); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919a9e); }
            blk = 39; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919aa5); };
            c.w32(15, t84);
            let r85 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919aac); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919ab3); };
            c.w32(7, t88);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x7000919ac3);
        }
        41 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000919ac4); };
            c.cmp(t89, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t90) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919acf); };
            c.w64(9, (t90 as u16 as i16 as i64 as u64));
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000919ad5); };
            c.w32(12, t91);
            let Some(t92) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x7000919ada); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x7000919afe); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919b03); }
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
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000919b19); };
            c.w32(12, t96);
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            blk = 49; continue;
        }
        49 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919b21); };
            c.w32(15, t98);
            let r99 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r99);
            let Some(t100) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919b29); };
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
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000919b47); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919b4f); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919b56); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919b5e); };
            c.w32(15, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919b62); };
            c.cmp(t109, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r110 = c.r64(9) | 0x3u64;
            c.w64(9, r110);
            c.logic(r110, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x7000919b6d); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r111 = c.r64(12) & c.r64(2);
            c.w64(12, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000919b81); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919b89); }
            blk = 56; continue;
        }
        55 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000919b92); };
            c.w64(9, t113);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x7000919b99); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000919b9e); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000919ba5); };
            c.w64(11, t114);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000919bb0); };
            c.w64(1, t115);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t116) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000919bb7); };
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
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000919bdd); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000919be9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000919bea); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x7000919bf7, 0x7000919bf2);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000919bf7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000919bf9); }
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
            let Some(t123) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000919c0c); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000919c1b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000919c1c); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000919c29, 0x7000919c24);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000919c29); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000919c2b); }
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
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000919c41); };
            c.logic(t126 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000919c50); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000919c51); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000919c5e, 0x7000919c59);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000919c5e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000919c60); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x7000919c68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000919c74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000919c74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000919c78); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000919c82); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000919c88); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000919c8c); };
            c.w32(2, t3);
            let r4 = c.shl(c.r32(2), 0x5u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000919c95); };
            c.w32(0, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919c9b); };
            c.w64(8, t6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919c9f); };
            c.w64(11, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2e30)) else { return RegfileExit::fault(0x7000919ca5); };
            c.w64(3, t8);
            let Some(t9) = c.ld64(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000919cac); };
            c.w64(3, t9);
            c.w64(15, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000919cb5); }
            c.w64(6, c.r64(8));
            return c.call(0x700086ef74, 0x7000919cc1, 0x7000919cbc);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000919cc1); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000919cc5); };
            c.w64(7, t11);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000919ccc); };
            c.w64(2, t12);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000919cd7); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000919cdc); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000919ce3); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000919ce7); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000919cf8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000919d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000919d14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000919d18); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000919d22); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000919d28); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000919d2c); };
            c.w32(2, t3);
            let r4 = c.shl(c.r32(2), 0x5u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000919d35); };
            c.w32(8, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919d3b); };
            c.w64(11, t6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919d3f); };
            c.w64(3, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2e30)) else { return RegfileExit::fault(0x7000919d45); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(2).wrapping_add(c.r64(1)).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000919d4c); };
            c.w64(2, t9);
            c.w64(15, c.r64(7).wrapping_add(c.r64(3).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000919d55); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000919d59); }
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x700086f594, 0x7000919d6b, 0x7000919d66);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000919d6b); };
            c.w64(7, t10);
            let Some(t11) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000919d6f); };
            c.w64(8, t11);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000919d76); };
            c.w64(11, t12);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(3, c.r64(8).wrapping_add(0x1));
            let Some(t13) = c.ld16(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919d81); };
            c.w64(3, (t13 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x7000919d87); }
            c.w64(0, c.r64(3).wrapping_sub(0x1));
            if !c.st64(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x7000919d8f); }
            c.w32(2, 0x9u64);
            c.w64(9, c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1));
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000919d9d); };
            c.w32(2, t14);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000919da2); };
            c.w64(15, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000919da9); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000919dad); };
            c.w64(5, t17);
            c.w64(12, c.r64(11));
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000919db4); };
            c.w64(0, t18);
            let r19 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r19);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000919dc0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000919dd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            c.w64(1, c.r64(5));
            let r0 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r0);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000919ddd); }
            let Some(t1) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919de1); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000919de5); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000919dec); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000919df3); };
            c.w32(11, t6);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000919dfa); };
            c.w32(15, t7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000919e00); };
            c.w32(0, (t8 as u16 as i16 as i64 as u64));
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000919e06); };
            c.w32(9, t9);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000919e14);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000919e15); };
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
            return RegfileExit::jump(0x7000919e30);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000919e34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000919e34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000919e38); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000919e42); };
            c.w64(8, t1);
            let r2 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r2);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000919e48); }
            let Some(t3) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919e4c); };
            c.w64(7, t3);
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000919e50); };
            c.w32(11, t4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000919e56); };
            c.w32(0, (t5 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000919e5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000919e60); }
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
            let Some(t6) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919f30); };
            c.w64(7, t6);
            c.w32(11, 0x58u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000919f3a); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000919f3c); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000919f50, 0x7000919f4b);
        }
        25 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000919f50); };
            c.w64(0, t8);
            c.w64(7, c.r64(0));
            blk = 79; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t9) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919f66); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000919f6a); };
            c.w64(11, t10);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c774, 0x7000919f7b, 0x7000919f76);
        }
        27 => {
            blk = 79; continue;
        }
        28 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t11) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919f87); };
            c.w64(0, t11);
            let Some(t12) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000919f8b); };
            c.w64(11, t12);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082d2d4, 0x7000919f9c, 0x7000919f97);
        }
        29 => {
            blk = 79; continue;
        }
        30 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t13) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919fa8); };
            c.w64(0, t13);
            let Some(t14) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000919fac); };
            c.w64(11, t14);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c2b4, 0x7000919fbd, 0x7000919fb8);
        }
        31 => {
            blk = 79; continue;
        }
        32 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t15) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000919fc9); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919fcd); };
            c.w64(3, t16);
            let Some(t17) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000919fd1); };
            c.w64(11, t17);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c614, 0x7000919fe2, 0x7000919fdd);
        }
        33 => {
            blk = 79; continue;
        }
        34 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t18) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000919fee); };
            c.w64(0, t18);
            let Some(t19) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000919ff2); };
            c.w64(3, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000919ff6); };
            c.w64(11, t20);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082c6b4, 0x700091a007, 0x700091a002);
        }
        35 => {
            blk = 79; continue;
        }
        36 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t21) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a013); };
            c.w64(0, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a017); };
            c.w64(11, t22);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700086bf14, 0x700091a028, 0x700091a023);
        }
        37 => {
            blk = 79; continue;
        }
        38 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t23) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a034); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a038); };
            c.w64(11, t24);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700086ca54, 0x700091a049, 0x700091a044);
        }
        39 => {
            blk = 79; continue;
        }
        40 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t25) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a055); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a059); };
            c.w64(11, t26);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000869974, 0x700091a06a, 0x700091a065);
        }
        41 => {
            blk = 79; continue;
        }
        42 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t27) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a076); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a07a); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a07e); };
            c.w64(11, t29);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000869834, 0x700091a08f, 0x700091a08a);
        }
        43 => {
            blk = 79; continue;
        }
        44 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t30) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a09b); };
            c.w64(0, t30);
            let Some(t31) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a09f); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a0a3); };
            c.w64(11, t32);
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x70008699f4, 0x700091a0b4, 0x700091a0af);
        }
        45 => {
            blk = 79; continue;
        }
        46 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t33) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a0c0); };
            c.w64(0, t33);
            let Some(t34) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a0c4); };
            c.w64(11, t34);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082fbd4, 0x700091a0d5, 0x700091a0d0);
        }
        47 => {
            blk = 79; continue;
        }
        48 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t35) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a0dd); };
            c.w64(7, t35);
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700091a0e1); };
            c.w32(7, t36);
            let r37 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r37);
            c.w64(0, c.r64(7));
            blk = 79; continue;
        }
        49 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a0f2); };
            c.w64(7, t38);
            c.w64(11, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700091a0fd); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a101); };
            c.w64(0, t39);
            c.w64(7, c.r64(0));
            blk = 79; continue;
        }
        50 => {
            let Some(t40) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a110); };
            c.w32(11, t40);
            let r41 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r41);
            let Some(t42) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a117); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a11e); };
            c.w32(0, t44);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x7au64, 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700091a12e);
        }
        52 => {
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(0x1ef)) else { return RegfileExit::fault(0x700091a12f); };
            c.w32(11, t45);
            c.w64(0, c.r64(7));
            blk = 54; continue;
        }
        53 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091a13b); };
            c.w32(0, t46);
            let r47 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r47);
            blk = 54; continue;
        }
        54 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a141); };
            c.cmp(t48, c.r32(11), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a147); };
            c.w32(3, t49);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_a() { blk = 53; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x700091a156);
        }
        57 => {
            let Some(t50) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a157); };
            c.w32(11, t50);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x700091a167);
        }
        59 => {
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091a168); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700091a16f); };
            c.w32(11, t53);
            let r54 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700091a176); };
            c.cmp(t55, c.r32(11), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, c.r64(11));
            blk = 79; continue;
        }
        61 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x22a0)) else { return RegfileExit::fault(0x700091a187); };
            c.w64(3, t56);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700091a198, 0x700091a193);
        }
        62 => {
            blk = 79; continue;
        }
        63 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t57) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a1a4); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a1a8); };
            c.w64(11, t58);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000821334, 0x700091a1b9, 0x700091a1b4);
        }
        64 => {
            blk = 79; continue;
        }
        65 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)));
            let Some(t59) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a1c5); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a1c9); };
            c.w64(11, t60);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x700082faf4, 0x700091a1da, 0x700091a1d5);
        }
        66 => {
            blk = 79; continue;
        }
        67 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t61) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a1e2); };
            c.w64(11, t61);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091a1e6); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a1f4); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r63);
            let Some(t64) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a1fb); };
            c.cmp(t64, 0xbau64, 16);
            if c.cc_b() { blk = 83; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x700091a207); };
            c.w64(3, t65);
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(7));
            return c.call(0x7000870914, 0x700091a219, 0x700091a214);
        }
        70 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091a219); };
            c.w64(6, t66);
            let Some(t67) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a21d); };
            c.w32(7, t67);
            let r68 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r68);
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a223); };
            c.w32(7, t69);
            let r70 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r70);
            let Some(t71) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a229); };
            c.w32(8, t71);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2du64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::fault(0x700091a239);
        }
        72 => {
            let Some(t72) = c.ld32(c.r64(7).wrapping_add(0xbb)) else { return RegfileExit::fault(0x700091a23a); };
            c.w32(7, t72);
            let r73 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r73);
            let Some(t74) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a243); };
            c.w32(8, t74);
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091a248); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091a24f); };
            c.w64(3, t76);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091a25e); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700091a265); };
            c.cmp(t78, c.r64(8), 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x700091a26e); }
            c.w64(8, c.r64(3).wrapping_add(0x1));
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700091a275); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091a279); };
            c.w64(7, t79);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700091a280); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700091a284); }
            c.w64(7, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a292); };
            c.w64(11, t80);
            blk = 76; continue;
        }
        76 => {
            let r81 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r81);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x700091a29d); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 76; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091a2a7); };
            c.w64(7, t82);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700091a2ab); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700091a2af); }
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(8));
            blk = 79; continue;
        }
        79 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a2b6); };
            c.w64(7, t83);
            let Some(t84) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091a2ba); };
            c.w64(8, t84);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t85) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091a2c1); };
            c.w64(7, t85);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_add(0x7));
            let Some(t86) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091a2cc); };
            c.w32(8, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700091a2d1); };
            c.w64(15, t87);
            let Some(t88) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a2d8); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091a2dc); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700091a2eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x700091a2ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700091a2f3); }
            return c.call(0x70007febf4, 0x700091a2fc, 0x700091a2f7);
        }
        82 => {
            c.w64(8, c.r64(0));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091a2ff); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091a303); };
            c.w64(2, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091a307); };
            c.w64(7, t93);
            blk = 74; continue;
        }
        83 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x700091a310); };
            c.w64(3, t94);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700091a321, 0x700091a31c);
        }
        84 => {
            c.w64(8, c.r64(0));
            blk = 78; continue;
        }
        85 => {
            return RegfileExit::jump(0x700091a328);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091a334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091a334); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091a338); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091a342); };
            c.w64(11, t1);
            let r2 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r2);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091a349); }
            let Some(t3) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a34d); };
            c.w64(8, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091a351); };
            c.w64(3, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a357); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld64(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a35e); };
            c.w64(3, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a362); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a369); };
            c.w32(2, (t10 as u16 as i16 as i64 as u64));
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091a36f); };
            c.w32(1, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a375); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a37b); };
            c.w32(7, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091a37f); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x700091a383); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700091a387); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091a38b); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a394); };
            c.w64(8, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a39b); };
            c.w64(7, t16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a39f); };
            c.w32(9, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091a3a3); };
            c.cmp(t18, c.r32(9), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, c.r64(6).wrapping_mul(4));
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091a3b8); };
            c.w32(12, t19);
            let r20 = c.add(c.r32(12), 0x8u64, 32);
            c.w32(12, r20);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x700091a3c4); }
            c.w64(6, c.r64(6).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091a3d1); };
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
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091a3e9); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700091a3f6); };
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
            let Some(t27) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a412); };
            c.w32(15, t27);
            let r28 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r28);
            let Some(t29) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a41a); };
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
            let Some(t30) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a42a); };
            c.w32(6, t30);
            let r31 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r31);
            let Some(t32) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a430); };
            c.w32(6, t32);
            let r33 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r33);
            let Some(t34) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a436); };
            c.w32(15, t34);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700091a446);
        }
        12 => {
            let Some(t35) = c.ld32(c.r64(6).wrapping_add(0xb3)) else { return RegfileExit::fault(0x700091a447); };
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
            let Some(t36) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a466); };
            c.w32(12, t36);
            let r37 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a46d); };
            c.w32(12, t38);
            let r39 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r39);
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a475); };
            c.w32(15, t40);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x700091a486);
        }
        18 => {
            let Some(t41) = c.ld32(c.r64(12).wrapping_add(0xb3)) else { return RegfileExit::fault(0x700091a487); };
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
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091a4a2); };
            c.w32(12, t42);
            let r43 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r43);
            blk = 22; continue;
        }
        22 => {
            let Some(t44) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a4aa); };
            c.w32(15, t44);
            let r45 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r45);
            let Some(t46) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a4b2); };
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
            let Some(t48) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091a4d0); };
            c.w32(12, t48);
            let r49 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r49);
            let Some(t50) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a4d8); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            let Some(t52) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a4df); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            let Some(t54) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a4e7); };
            c.w32(15, t54);
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a4eb); };
            c.cmp(t55, c.r32(15), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(0));
            let r56 = c.r64(9) | 0x3u64;
            c.w64(9, r56);
            c.logic(r56, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(9)) { return RegfileExit::fault(0x700091a4f9); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r57 = c.r64(12) & c.r64(8);
            c.w64(12, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a50d); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091a519); }
            blk = 38; continue;
        }
        28 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091a526); };
            c.w64(6, t59);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x700091a52d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091a532); }
            blk = 38; continue;
        }
        29 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x700091a53f); };
            c.w64(9, t60);
            let Some(t61) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700091a546); };
            c.w64(12, t61);
            c.w64(15, c.r64(12).wrapping_add(0x1c));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x700091a54e); };
            c.w64(0, t62);
            let Some(t63) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700091a555); };
            c.cmp(t63, c.r64(15), 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700091a55e); }
            c.w64(0, c.r64(12).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x700091a566); };
            c.w64(9, t64);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700091a56d); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700091a574); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700091a578); }
            if !c.st32(c.r64(0).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x700091a57c); }
            if !c.st32(c.r64(0).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x700091a583); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x700091a58a); };
            c.w64(9, t65);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700091a591); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x700091a595); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700091a59c); };
            c.w64(12, t67);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x700091a59f); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r68 = c.r64(15) & c.r64(0);
            c.w64(15, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a5b3); };
            c.logic(t69 & 0x4u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x700091a5be); }
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x700091a5c1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r70 = c.r64(3) & c.r64(8);
            c.w64(3, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a5d3); };
            c.logic(t71 & 0x4u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091a5dd); }
            c.w64(12, c.r64(0));
            blk = 36; continue;
        }
        36 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(6, c.r64(7));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091a5f8); };
            c.w64(7, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091a5fc); };
            c.w64(2, t73);
            c.w64(3, c.r64(12));
            return c.call(0x70007fcd34, 0x700091a608, 0x700091a603);
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
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a620); };
            c.w64(3, t74);
            return c.call(0x70007fccd4, 0x700091a62c, 0x700091a627);
        }
        39 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a62c); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091a630); };
            c.w64(11, t76);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t77) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091a637); };
            c.w64(8, t77);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0xa));
            let Some(t78) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700091a642); };
            c.w32(11, t78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700091a647); };
            c.w64(15, t79);
            let Some(t80) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a64e); };
            c.w64(1, t80);
            let Some(t81) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091a652); };
            c.w64(5, t81);
            c.w64(12, c.r64(8));
            let r82 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r82);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w32(0, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700091a664); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700091a668); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700091a66c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700091a670); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700091a674); }
            c.w64(2, c.r64(0));
            return c.call(0x70007fec74, 0x700091a680, 0x700091a67b);
        }
        41 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091a680); };
            c.w64(6, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a684); };
            c.w64(11, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091a688); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091a68c); };
            c.w64(7, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091a690); };
            c.w64(2, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091a694); };
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
            let Some(t91) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a6ac); };
            c.logic(t91 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091a6ba); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091a6bb); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700091a6c8, 0x700091a6c3);
        }
        44 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091a6c8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091a6ca); }
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
            let Some(t94) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a6e0); };
            c.logic(t94 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(3, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091a6ef); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091a6f0); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700091a6fd, 0x700091a6f8);
        }
        47 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091a6fd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091a6ff); }
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
            let Some(t97) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a715); };
            c.logic(t97 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091a725); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091a726); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700091a733, 0x700091a72e);
        }
        50 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091a733); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091a735); }
            blk = 27; continue;
        }
        51 => {
            return RegfileExit::jump(0x700091a73c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091a754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091a754); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091a758); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091a762); };
            c.w64(11, t1);
            let r2 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r2);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091a769); }
            let Some(t3) = c.ld64(c.r64(11).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a76d); };
            c.w64(8, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091a771); };
            c.w64(3, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a777); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld64(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a77e); };
            c.w64(3, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a782); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091a789); };
            c.w32(2, (t10 as u16 as i16 as i64 as u64));
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091a78f); };
            c.w32(1, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a795); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a79b); };
            c.w32(7, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x700091a79f); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091a7a3); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(8));
            blk = 3; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a7b1); };
            c.w64(7, t15);
            blk = 3; continue;
        }
        3 => {
            let Some(t16) = c.ld64(c.r64(11).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a7b8); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a7bc); };
            c.w64(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a7c3); };
            c.cmp(t18, c.r32(7), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(6).wrapping_mul(4));
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091a7d4); };
            c.w32(15, t19);
            let r20 = c.add(c.r32(15), 0x8u64, 32);
            c.w32(15, r20);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x700091a7e0); }
            c.w64(6, c.r64(6).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091a7ed); };
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
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091a811); };
            c.w64(3, t23);
            c.w64(6, c.r64(9));
            return c.call(0x70007fcd94, 0x700091a820, 0x700091a81b);
        }
        6 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091a820); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091a824); };
            c.w64(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091a82b); };
            c.w64(8, t26);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0xa));
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700091a836); };
            c.w32(11, t27);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700091a83b); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a842); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091a846); };
            c.w64(5, t30);
            c.w64(12, c.r64(8));
            let r31 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091a853); };
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
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700091a869); };
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
            let Some(t35) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a88c); };
            c.w32(15, t35);
            let r36 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r36);
            let Some(t37) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a893); };
            c.w32(15, t37);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091a8a0); };
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
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091a8b6); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a8bd); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a8c4); };
            c.w32(15, t45);
            let r46 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r46);
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a8cb); };
            c.w32(8, t47);
            let Some(t48) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091a8cf); };
            c.cmp(t48, c.r32(8), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(0));
            let r49 = c.r64(8) | 0x3u64;
            c.w64(8, r49);
            c.logic(r49, 64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(6)), c.r32(8)) { return RegfileExit::fault(0x700091a8dc); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r50 = c.r64(12) & c.r64(7);
            c.w64(12, r50);
            c.logic(r50, 64);
            let Some(t51) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a8f0); };
            c.logic(t51 & 0x4u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091a8f8); }
            blk = 5; continue;
        }
        21 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091a904); };
            c.w64(8, t52);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700091a90b); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091a910); }
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
            let Some(t55) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091a92c); };
            c.logic(t55 & 0x2u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(15, c.r64(7).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091a938); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700091a942, 0x700091a93d);
        }
        24 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091a942); }
            blk = 20; continue;
        }
        25 => {
            return RegfileExit::jump(0x700091a948);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091a954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091a954); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091a958); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091a95e); };
            c.w64(7, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091a962); };
            c.w64(8, (t2 as u16 as i16 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091a968); };
            c.w64(11, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091a96c); };
            c.w64(8, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091a970); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700091a974); }
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
            let Some(t5) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a9ba); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091a9c1); };
            c.w32(15, t7);
            let r8 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a9c8); };
            c.w32(11, t9);
            let Some(t10) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091a9ce); };
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
            let Some(t11) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091aa0d); };
            c.logic(t11 & 0x10u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 88; continue;
        }
        12 => {
            c.w32(11, c.r32(3));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091aa20); };
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
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091aa3d); };
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
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091aa83); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 23; continue;
        }
        23 => {
            let r18 = c.r32(3) | 0x8u64;
            c.w32(3, r18);
            c.logic(r18, 32);
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091aa93); };
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
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091aad0); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091aad6); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t25) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091aae3); };
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
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091ab11); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 35; continue;
        }
        35 => {
            let r27 = c.r32(11) | 0x8u64;
            c.w32(11, r27);
            c.logic(r27, 32);
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091ab22); };
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
            let Some(t30) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091ab3d); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091ab44); };
            c.cmp(t32, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t33) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ab51); };
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
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091ab91); };
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700091abdf); }
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
            let Some(t43) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091ac09); };
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
            let Some(t46) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091ac3b); };
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700091ac9d); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091aca3); }
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
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091acf8); }
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
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091ad11); };
            c.w64(0, t59);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091ad18); }
            blk = 82; continue;
        }
        80 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        81 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091ad21); };
            c.w64(0, t60);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091ad28); }
            blk = 82; continue;
        }
        82 => {
            let Some(t61) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ad2c); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ad30); };
            c.w64(7, t62);
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ad34); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091ad3b); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            c.w64(11, c.r64(9));
            let Some(t67) = c.ld16(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091ad45); };
            c.w32(3, t67);
            let Some(t68) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091ad4b); };
            c.w32(2, t68);
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091ad4f); };
            c.cmp(t69, c.r32(2), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091ad58); };
            c.w64(8, t70);
            blk = 84; continue;
        }
        84 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091ad5f); };
            c.cmp(t71, c.r32(8), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t72) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091ad68); };
            c.w32(2, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091ad6d); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091ad7e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091ad83); }
            blk = 87; continue;
        }
        87 => {
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091ad8f); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ad94); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ad98); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ad9c); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091adbd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091adc1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091adc5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091adc9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091adcd); }
            c.w64(0, c.r64(8));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091add4); };
            c.w64(6, t82);
            return c.call(0x70008a2374, 0x700091addd, 0x700091add8);
        }
        89 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091addd); };
            c.w64(3, t83);
            c.w64(8, c.r64(0));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091ade4); };
            c.w64(0, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091ade8); };
            c.w64(9, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091adec); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091ae09); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091ae0d); }
            let Some(t88) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091ae11); };
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
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091ae32); };
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
            let Some(t91) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091ae52); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091ae65); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091ae69); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091ae6d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700091ae71); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700091ae75); }
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091ae99); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091ae9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091aea1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700091aea5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091aea9); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091aead); };
            c.w64(6, t95);
            return c.call(0x70008a2374, 0x700091aeb6, 0x700091aeb1);
        }
        101 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091aeb6); };
            c.w64(3, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091aeba); };
            c.w64(8, t97);
            c.w32(2, 0x60u64);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091aec3); };
            c.w64(9, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091aec7); };
            c.w64(7, t99);
            c.w32(1, 0x20u64);
            blk = 1; continue;
        }
        102 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 103; continue;
        }
        103 => {
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091aed8); };
            c.w32(3, t100);
            let r101 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r101);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091aede); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091aee2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700091aee6); }
            let Some(t102) = c.ld8(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091aeea); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700091af0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091af10); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091af14); };
            c.w64(6, t105);
            return c.call(0x70008a0fd4, 0x700091af1d, 0x700091af18);
        }
        107 => {
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091af1d); };
            c.w64(3, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091af21); };
            c.w64(8, t107);
            c.w32(2, 0x60u64);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091af2a); };
            c.w64(9, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091af2e); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091af4c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091af50); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091af52); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091af53); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x1d50)) else { return RegfileExit::fault(0x700091af5e); };
            c.w64(3, t112);
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091af70, 0x700091af6b);
        }
        109 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091af70); };
            c.w64(7, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091af74); };
            c.w64(9, t114);
            blk = 82; continue;
        }
        110 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r115 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r115);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091af89); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091af8d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091af8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091af90); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700091af96); };
            c.w64(3, t116);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091afaa, 0x700091afa5);
        }
        111 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091afaa); };
            c.w64(7, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091afae); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091afc7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091afcb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091afcd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091afce); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x700091afd4); };
            c.w64(3, t121);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091afe8, 0x700091afe3);
        }
        113 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091afe8); };
            c.w64(7, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091afec); };
            c.w64(9, t123);
            blk = 82; continue;
        }
        114 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r124 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r124);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700091affc); }
            c.w64(2, c.r64(8));
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091b003); };
            c.w64(6, t125);
            return c.call(0x70007fb534, 0x700091b00c, 0x700091b007);
        }
        115 => {
            let Some(t126) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091b00c); };
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
            let Some(t127) = c.ld8(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091b01e); };
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
            let Some(t128) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091b035); };
            c.w32(11, t128);
            let r129 = c.r32(8) | c.r32(7);
            c.w32(8, r129);
            c.logic(r129, 32);
            let r130 = c.r32(8) | c.r32(11);
            c.w32(8, r130);
            c.logic(r130, 32);
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091b042); }
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b046); };
            c.w64(7, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091b04a); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091b066); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b06a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091b06b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091b06d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091b071); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091b075); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x700091b07b); };
            c.w64(3, t135);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091b08f, 0x700091b08a);
        }
        123 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b08f); };
            c.w64(7, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091b093); };
            c.w64(9, t137);
            blk = 82; continue;
        }
        124 => {
            return RegfileExit::jump(0x700091b0a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091b0b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091b0b7); };
            c.w64(8, (t0 as u16 as i16 as i64 as u64));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091b0bd); };
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
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b0e8); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b0ef); };
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
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b105); };
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
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b11e); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b125); };
            c.w32(0, t8);
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_ae() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091b131); };
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
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b16d); };
            c.cmp(t10, c.r32(8), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(8, 0x210u64);
            blk = 61; continue;
        }
        15 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091b185); };
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
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b1b8); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b1bf); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b1d5); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b1d9); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b1e6); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b1ec); }
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b211); }
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
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b242); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b249); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b252); };
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
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b273); }
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
            let Some(t23) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b2b4); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b2bb); };
            c.cmp(t25, c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t26) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b2c8); };
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
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700091b300); };
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
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b33a); }
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
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091b34e); };
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
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091b365); };
            c.w64(0, t29);
            blk = 65; continue;
        }
        65 => {
            let Some(t30) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091b36c); };
            c.w64(11, t30);
            let Some(t31) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b370); };
            c.w64(7, t31);
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091b374); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091b37b); };
            c.w32(11, t34);
            let r35 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r35);
            c.w64(3, c.r64(9));
            let Some(t36) = c.ld16(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091b385); };
            c.w32(2, t36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b38a); };
            c.w32(1, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091b38e); };
            c.cmp(t38, c.r32(1), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091b397); };
            c.w64(11, t39);
            blk = 67; continue;
        }
        67 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091b39e); };
            c.cmp(t40, c.r32(11), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091b3a7); };
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
            if !c.st32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700091b3bc); }
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091b3c1); }
            blk = 70; continue;
        }
        70 => {
            c.w64(9, c.r64(3).wrapping_add(0x5));
            let Some(t43) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091b3cd); };
            c.w32(8, t43);
            let Some(t44) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091b3d2); };
            c.w64(1, t44);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        71 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091b3db); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091b3df); }
            let r45 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r45);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r46 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r46);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b3ec); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091b3f0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b3f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091b3f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091b3f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091b3fb); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700091b3ff); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700091b412, 0x700091b40d);
        }
        72 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b412); };
            c.w64(7, t48);
            c.w32(8, 0x100u64);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091b41c); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091b420); };
            c.w64(9, t50);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091b427); }
            blk = 65; continue;
        }
        73 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091b42d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091b431); }
            let r51 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b437); };
            c.w32(11, t52);
            let r53 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r53);
            let Some(t54) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b43e); };
            c.w32(11, t54);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091b443); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091b447); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091b44b); }
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091b459); }
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091b495); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b499); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007fb534, 0x700091b4a7, 0x700091b4a2);
        }
        82 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091b4a7); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b4ab); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091b4af); };
            c.w64(15, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091b4b3); };
            c.w64(9, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091b4ba); }
            blk = 65; continue;
        }
        83 => {
            return RegfileExit::jump(0x700091b4c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091b4d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091b4d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091b4d8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091b4de); };
            c.w64(7, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091b4e4); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091b4ec); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091b4f3); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091b4f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091b4fb); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091b509); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091b50d); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b511); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091b515); };
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b52f); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b536); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b54b); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b577); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b57e); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b58c); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b591); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b59a); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091b5ab); }
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
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091b5d4); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091b5dd); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091b5e7); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b5eb); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091b5ef); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091b5f6); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091b5fd); };
            c.w64(11, t30);
            let Some(t31) = c.ld16(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091b601); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091b607); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091b60b); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091b614); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091b61b); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091b624); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091b636); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091b63b); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091b647); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091b64c); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091b650); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091b654); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b661); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091b666); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091b66a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091b66e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091b672); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091b676); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b697); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b6a0); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091b6b6); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b6c2); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x700091b6ce, 0x700091b6c9);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091b6ce); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b6d2); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b6d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091b6da); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091b6de); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091b6e2); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x700091b6eb, 0x700091b6e6);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091b6eb); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091b6ef); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091b6f3); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b6fc); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091b70b, 0x700091b706);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091b70b); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091b70f); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091b713); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b71c); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b746); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091b755, 0x700091b750);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091b755); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b759); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b75d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091b761); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091b765); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091b769); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x700091b772, 0x700091b76d);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b782); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091b78e, 0x700091b789);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091b798); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b7a5); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b7c7); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091b7d3, 0x700091b7ce);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091b7dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091b7e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091b7e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091b7e9); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b7ed); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b800); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091b80c, 0x700091b807);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091b816); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091b81a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091b81e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091b822); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b826); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b838); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091b847, 0x700091b842);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091b84c); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091b850); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091b859); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091b85d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091b861); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091b865); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091b869); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b87b); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091b887, 0x700091b882);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091b897); }
            c.w32(11, 0x28u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091b8a1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b8a3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091b8a4); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091b8aa); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091b8b9, 0x700091b8b4);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b8be); };
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
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700091b8d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091b8d6); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb9f4, 0x700091b8e2, 0x700091b8dd);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091b8e2); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b8e6); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091b8f5); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091b8ff); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091b901); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b902); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091b903); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091b914, 0x700091b90f);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b919); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091b933); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091b93d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091b93f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b940); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700091b941); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x700091b945); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091b956, 0x700091b951);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091b956); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b95a); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091b966); }
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091b96f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091b970); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b971); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091b972); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091b983, 0x700091b97e);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b988); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b991); }
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091b99a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b99b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091b99c); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091b99d); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091b9ae, 0x700091b9a9);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b9b3); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b9bf); }
            c.w32(8, 0x24u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091b9c9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091b9cb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b9cc); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091b9cd); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091b9de, 0x700091b9d9);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091b9e3); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091b9ec); }
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091b9f5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091b9f6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091b9f7); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091b9f8); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091ba09, 0x700091ba04);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ba0e); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x700091ba18);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ba34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091ba34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091ba38); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ba3e); };
            c.w64(7, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ba44); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ba4c); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091ba53); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091ba57); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091ba5b); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091ba69); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091ba6d); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ba71); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091ba75); };
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091ba8f); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091ba96); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091baab); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091bad7); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091bade); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091baec); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091baf1); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091bafa); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091bb0b); }
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
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091bb34); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091bb3d); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091bb47); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bb4b); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091bb4f); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091bb56); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091bb5d); };
            c.w64(11, t30);
            let Some(t31) = c.ld16(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091bb61); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091bb67); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091bb6b); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091bb74); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091bb7b); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091bb84); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091bb96); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091bb9b); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091bba7); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091bbac); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091bbb0); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091bbb4); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091bbc1); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091bbc6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091bbca); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091bbce); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091bbd2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091bbd6); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091bbf7); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091bc00); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091bc16); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091bc22); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x700091bc2e, 0x700091bc29);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091bc2e); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bc32); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091bc36); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091bc3a); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091bc3e); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091bc42); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x700091bc4b, 0x700091bc46);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091bc4b); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091bc4f); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091bc53); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091bc5c); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091bc6b, 0x700091bc66);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091bc6b); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091bc6f); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091bc73); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091bc7c); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bca6); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091bcb5, 0x700091bcb0);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091bcb5); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bcb9); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bcbd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091bcc1); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091bcc5); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091bcc9); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x700091bcd2, 0x700091bccd);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bce2); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091bcee, 0x700091bce9);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091bcf8); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091bd05); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bd27); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091bd33, 0x700091bd2e);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091bd3d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091bd41); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091bd45); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091bd49); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091bd4d); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bd60); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091bd6c, 0x700091bd67);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091bd76); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091bd7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091bd7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091bd82); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091bd86); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bd98); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091bda7, 0x700091bda2);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091bdac); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091bdb0); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091bdb9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091bdbd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091bdc1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091bdc5); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091bdc9); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bddb); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091bde7, 0x700091bde2);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091bdf7); }
            c.w32(11, 0x24u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091be01); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091be03); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091be04); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091be0a); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091be19, 0x700091be14);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091be1e); };
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
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700091be32); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091be36); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb774, 0x700091be42, 0x700091be3d);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091be42); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091be46); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091be55); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091be5f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091be61); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091be62); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091be63); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091be74, 0x700091be6f);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091be79); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091be93); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091be9d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091be9f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091bea0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700091bea1); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x700091bea5); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091beb6, 0x700091beb1);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091beb6); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091beba); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091bec6); }
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091becf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091bed0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091bed1); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091bed2); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091bee3, 0x700091bede);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bee8); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091bef1); }
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091befa); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091befb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091befc); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091befd); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091bf0e, 0x700091bf09);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bf13); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091bf1f); }
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091bf29); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091bf2b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091bf2c); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091bf2d); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091bf3e, 0x700091bf39);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bf43); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091bf4c); }
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091bf55); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091bf56); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091bf57); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091bf58); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091bf69, 0x700091bf64);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bf6e); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x700091bf78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091bf94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091bf94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091bf98); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091bf9e); };
            c.w64(7, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091bfa4); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091bfac); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091bfb3); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091bfb7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091bfbb); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091bfc9); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091bfcd); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091bfd1); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091bfd5); };
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091bfef); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091bff6); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091c00b); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091c037); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091c03e); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c04c); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c051); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c05a); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c06b); }
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
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091c094); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091c09d); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091c0a7); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c0ab); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091c0af); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091c0b6); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091c0bd); };
            c.w64(11, t30);
            let Some(t31) = c.ld16(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091c0c1); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091c0c7); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091c0cb); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091c0d4); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091c0db); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091c0e4); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091c0f6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091c0fb); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091c107); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091c10c); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091c110); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091c114); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c121); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c126); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091c12a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091c12e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c132); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c136); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091c157); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c160); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091c176); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c182); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x700091c18e, 0x700091c189);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091c18e); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c192); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c196); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091c19a); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091c19e); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091c1a2); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x700091c1ab, 0x700091c1a6);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c1ab); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c1af); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c1b3); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c1bc); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091c1cb, 0x700091c1c6);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c1cb); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c1cf); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c1d3); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c1dc); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c206); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c215, 0x700091c210);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091c215); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c219); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c21d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c221); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c225); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091c229); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x700091c232, 0x700091c22d);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c242); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091c24e, 0x700091c249);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091c258); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c265); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c287); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c293, 0x700091c28e);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c29d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091c2a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c2a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c2a9); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c2ad); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c2c0); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c2cc, 0x700091c2c7);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c2d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091c2da); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c2de); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c2e2); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c2e6); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c2f8); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c307, 0x700091c302);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c30c); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c310); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c319); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091c31d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c321); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c325); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c329); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c33b); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c347, 0x700091c342);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c357); }
            c.w32(11, 0x2au64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091c361); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c363); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c364); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091c36a); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091c379, 0x700091c374);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c37e); };
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
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700091c392); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091c396); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fbb34, 0x700091c3a2, 0x700091c39d);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c3a2); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c3a6); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091c3b5); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091c3bf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c3c1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c3c2); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091c3c3); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c3d4, 0x700091c3cf);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c3d9); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091c3f3); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091c3fd); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c3ff); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c400); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700091c401); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x700091c405); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c416, 0x700091c411);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c416); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c41a); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091c426); }
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091c42f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c430); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c431); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091c432); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c443, 0x700091c43e);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c448); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c451); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091c45a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c45b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c45c); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091c45d); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c46e, 0x700091c469);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c473); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c47f); }
            c.w32(8, 0x26u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091c489); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c48b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c48c); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091c48d); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c49e, 0x700091c499);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c4a3); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c4ac); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091c4b5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c4b6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c4b7); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091c4b8); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c4c9, 0x700091c4c4);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c4ce); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x700091c4d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091c4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091c4f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091c4f8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091c4fe); };
            c.w64(7, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091c504); };
            c.w64(8, t2);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091c50c); };
            c.w64(7, t3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let Some(t5) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091c513); };
            c.w64(3, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091c517); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091c51b); }
            c.w64(2, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091c529); };
            c.w64(11, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091c52d); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c531); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091c535); };
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
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091c54f); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091c556); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091c56b); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091c597); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091c59e); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 58; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c5ac); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c5b1); }
            blk = 15; continue;
        }
        13 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c5ba); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r19 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091c5cb); }
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
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091c5f4); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091c5fd); };
            c.w64(0, t23);
            blk = 20; continue;
        }
        20 => {
            c.w64(7, c.r64(8));
            let Some(t24) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091c607); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c60b); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091c60f); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091c616); };
            c.w32(8, t28);
            let r29 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091c61d); };
            c.w64(11, t30);
            let Some(t31) = c.ld16(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091c621); };
            c.w32(3, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091c627); };
            c.w32(2, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091c62b); };
            c.cmp(t33, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091c634); };
            c.w64(8, t34);
            blk = 22; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091c63b); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091c644); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091c656); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091c65b); }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091c667); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091c66c); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091c670); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091c674); };
            c.w64(5, t41);
            c.w64(12, c.r64(7));
            let r42 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r42);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            let Some(t43) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c681); };
            c.w32(9, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c686); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091c68a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091c68e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c692); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c696); }
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
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091c6b7); };
            c.cmp(t44, c.r32(12), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c6c0); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091c6d6); }
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c6e2); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a11d4, 0x700091c6ee, 0x700091c6e9);
        }
        35 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091c6ee); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c6f2); };
            c.w64(8, t48);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c6f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091c6fa); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091c6fe); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091c702); };
            c.w64(6, t50);
            return c.call(0x70008a10d4, 0x700091c70b, 0x700091c706);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c70b); };
            c.w64(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c70f); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c713); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c71c); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091c72b, 0x700091c726);
        }
        38 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c72b); };
            c.w64(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c72f); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c733); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        39 => {
            let Some(t55) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c73c); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c766); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c775, 0x700091c770);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091c775); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c779); };
            c.w64(7, t57);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c77d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c781); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c785); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091c789); };
            c.w64(6, t59);
            return c.call(0x70008a11d4, 0x700091c792, 0x700091c78d);
        }
        44 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        45 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c7a2); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a2474, 0x700091c7ae, 0x700091c7a9);
        }
        46 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        47 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091c7b8); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c7c5); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c7e7); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c7f3, 0x700091c7ee);
        }
        51 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c7fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091c801); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c805); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c809); }
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c80d); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c820); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c82c, 0x700091c827);
        }
        54 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c836); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091c83a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c83e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c842); }
            let Some(t63) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c846); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c858); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c867, 0x700091c862);
        }
        57 => {
            c.w32(1, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091c86c); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091c870); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700091c879); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091c87d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091c881); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091c885); }
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091c889); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c89b); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700091c8a7, 0x700091c8a2);
        }
        60 => {
            c.w32(1, 0x3feu64);
            blk = 1; continue;
        }
        61 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091c8b7); }
            c.w32(11, 0x26u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091c8c1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c8c3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c8c4); }
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091c8ca); };
            c.w64(3, t66);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700091c8d9, 0x700091c8d4);
        }
        62 => {
            c.w32(1, 0x3feu64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c8de); };
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
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700091c8f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700091c8f6); }
            c.w64(6, c.r64(3));
            return c.call(0x70007fb8b4, 0x700091c902, 0x700091c8fd);
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c902); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c906); };
            c.w64(8, t70);
            blk = 20; continue;
        }
        65 => {
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091c915); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091c91f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c921); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c922); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700091c923); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c934, 0x700091c92f);
        }
        66 => {
            c.w32(1, 0x3feu64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c939); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091c953); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091c95d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c95f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c960); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700091c961); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x700091c965); };
            c.w64(3, t74);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c976, 0x700091c971);
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091c976); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c97a); };
            c.w64(8, t76);
            blk = 20; continue;
        }
        69 => {
            c.w64(7, c.r64(11));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091c986); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091c98f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c990); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c991); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091c992); };
            c.w64(3, t77);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c9a3, 0x700091c99e);
        }
        70 => {
            c.w32(1, 0x3feu64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c9a8); };
            c.w64(8, t78);
            blk = 20; continue;
        }
        71 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c9b1); }
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091c9ba); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c9bb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c9bc); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091c9bd); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c9ce, 0x700091c9c9);
        }
        72 => {
            c.w32(1, 0x3feu64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091c9d3); };
            c.w64(8, t80);
            blk = 20; continue;
        }
        73 => {
            c.w64(7, c.r64(8));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091c9df); }
            c.w32(8, 0x2au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091c9e9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091c9eb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091c9ec); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091c9ed); };
            c.w64(3, t81);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091c9fe, 0x700091c9f9);
        }
        74 => {
            c.w32(1, 0x3feu64);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ca03); };
            c.w64(8, t82);
            blk = 20; continue;
        }
        75 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091ca0c); }
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091ca15); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091ca16); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091ca17); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700091ca18); };
            c.w64(3, t83);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700091ca29, 0x700091ca24);
        }
        76 => {
            c.w32(1, 0x3feu64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ca2e); };
            c.w64(8, t84);
            blk = 20; continue;
        }
        77 => {
            return RegfileExit::jump(0x700091ca38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ca54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091ca54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091ca58); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ca5e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ca62); };
            c.w64(11, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ca66); };
            c.w64(3, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ca6c); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ca73); };
            c.w64(3, t6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091ca77); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091ca7e); };
            c.w32(2, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091ca84); };
            c.w32(1, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700091ca88); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700091ca8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700091ca90); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x700091ca94); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091ca98); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091ca9c); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091caa5); };
            c.w64(11, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091caac); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cab0); };
            c.w32(6, t14);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700091cabe); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091cac2); };
            c.cmp(t15, c.r32(6), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091cad7); };
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
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091caed); };
            c.cmp(t19, c.r32(12), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700091cafa); };
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
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091cb1b); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            blk = 10; continue;
        }
        10 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cb21); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091cb28); };
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
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091cb47); };
            c.w32(3, t27);
            let r28 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r28);
            let Some(t29) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cb4d); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cb54); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091cb5a); };
            c.w32(12, t33);
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091cb5f); };
            c.cmp(t34, c.r32(12), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(3, c.r64(0));
            let r35 = c.r64(3) | 0x3u64;
            c.w64(3, r35);
            c.logic(r35, 64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(6)), c.r32(3)) { return RegfileExit::fault(0x700091cb6c); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r36 = c.r64(2) & c.r64(11);
            c.w64(2, r36);
            c.logic(r36, 64);
            let Some(t37) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091cb7f); };
            c.logic(t37 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091cb89); }
            blk = 17; continue;
        }
        16 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091cb93); };
            c.w64(3, t38);
            if !c.st32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x700091cb9a); }
            if !c.st32(c.r64(11).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091cb9f); }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r39 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700091cbae); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cbbb); };
            c.w32(3, t40);
            let r41 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r41);
            let Some(t42) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091cbc1); };
            c.cmp(t42, 0xbau64, 16);
            if c.cc_b() { blk = 48; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091cbcd); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xd08)) else { return RegfileExit::fault(0x700091cbd1); };
            c.w64(3, t43);
            c.w64(6, c.r64(1));
            return c.call(0x7000870914, 0x700091cbe0, 0x700091cbdb);
        }
        20 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091cbe0); };
            c.w64(6, t44);
            let Some(t45) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cbe4); };
            c.w32(8, t45);
            let r46 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r46);
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091cbeb); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091cbf2); };
            c.w32(11, t49);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xe4u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x700091cc05);
        }
        22 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(0x397)) else { return RegfileExit::fault(0x700091cc06); };
            c.cmp(t50, c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091cc13); };
            c.cmp(t51, c.r32(0), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091cc20); };
            c.cmp(t52, c.r32(0), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091cc2d); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091cc31); };
            c.w64(11, t54);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091cc35); }
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091cc39); };
            if !c.push64(t55) { return RegfileExit::fault(0x700091cc39); }
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091cc3c); };
            if !c.push64(t56) { return RegfileExit::fault(0x700091cc3c); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x700091cc4f, 0x700091cc4a);
        }
        26 => {
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091cc4f); };
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
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700091cc5f); };
            c.cmp(t58, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cc68); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091cc6f); };
            c.logic(t61 & 0x10u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091cc76); };
            c.cmp(t62, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t63) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091cc83); };
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
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091cc93); };
            c.w64(0, t64);
            c.w64(8, c.r64(0));
            blk = 37; continue;
        }
        35 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091cc9f); };
            c.w64(0, t65);
            c.w64(8, c.r64(0));
            blk = 37; continue;
        }
        36 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091ccab); };
            c.w64(8, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091ccaf); };
            c.w64(11, t67);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091ccb3); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091ccb7); };
            if !c.push64(t68) { return RegfileExit::fault(0x700091ccb7); }
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091ccba); };
            if !c.push64(t69) { return RegfileExit::fault(0x700091ccba); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            return c.call(0x70008a8174, 0x700091ccca, 0x700091ccc5);
        }
        37 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091ccca); };
            c.w64(11, t70);
            let Some(t71) = c.ld64(c.r64(11).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ccce); };
            c.w64(8, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091ccd2); };
            c.w64(11, t72);
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t73) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700091ccda); };
            c.w32(11, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ccdf); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091cce3); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091cce7); };
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
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091ccf6); };
            c.logic(t78 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 34; continue;
        }
        41 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091cd01); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 34; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 35; continue;
        }
        43 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091cd12); };
            c.w64(2, t79);
            let Some(t80) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cd16); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            let Some(t82) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091cd1d); };
            c.logic(t82 & 0x2u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091cd24); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091cd28); };
            c.w64(11, t84);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091cd2c); }
            c.w32(8, 0xa4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091cd36); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700091cd38); };
            c.w64(3, t85);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700091cd49, 0x700091cd44);
        }
        45 => {
            return RegfileExit::fault(0x700091cd49);
        }
        46 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091cd4a); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091cd4e); };
            c.w64(11, t87);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091cd52); }
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091cd56); };
            c.w64(0, t88);
            return c.call(0x7000857594, 0x700091cd5f, 0x700091cd5a);
        }
        47 => {
            blk = 37; continue;
        }
        48 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091cd64); }
            c.w32(8, 0xaeu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091cd6e); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700091cd70); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700091cd84, 0x700091cd7f);
        }
        49 => {
            return RegfileExit::fault(0x700091cd84);
        }
        50 => {
            c.w32(3, c.r32(3));
            let r90 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r90);
            c.w64(2, 0xfffffffffffc0000u64);
            let r91 = c.r64(2) & c.r64(3);
            c.w64(2, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091cd94); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091cda2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091cda3); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700091cdb0, 0x700091cdab);
        }
        52 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091cdb0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091cdb2); }
            blk = 15; continue;
        }
        53 => {
            return RegfileExit::jump(0x700091cdbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091cdd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091cdd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091cdd8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091cdde); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091cde2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091cde6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091cdec); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091cdf3); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091cdf7); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091cdfe); };
            c.w32(3, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091ce04); };
            c.w32(2, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091ce08); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091ce0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091ce10); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091ce14); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091ce1d); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091ce28); };
            c.w64(1, t13);
            let r14 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r14);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091ce34); }
            c.w64(2, c.r64(0));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(8));
            return c.call(0x70008436f4, 0x700091ce4c, 0x700091ce47);
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ce4c); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ce50); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091ce54); };
            c.w64(8, t17);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091ce5c); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ce61); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ce65); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ce69); };
            c.w64(5, t21);
            c.w64(12, c.r64(7));
            let r22 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091ce78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ce94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091ce94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091ce98); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091cea2); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091cea6); };
            c.w64(7, t2);
            let r3 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r3);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091ceac); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700091ceb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091ceb4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700091ceb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091cebc); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a1af4, 0x700091cec8, 0x700091cec3);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091cec8); };
            c.w64(3, t4);
            let Some(t5) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091cecc); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ced0); };
            c.w64(7, t6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(c.r64(7)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ced4); };
            c.w64(8, (t7 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x700091ceda); }
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091cee2); };
            c.w32(3, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091cee7); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ceeb); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ceef); };
            c.w64(5, t11);
            c.w64(12, c.r64(2));
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700091cef6); };
            c.w64(0, t12);
            let r13 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700091cf04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091cf14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091cf14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091cf18); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091cf1e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091cf22); };
            c.w64(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091cf26); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091cf2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700091cf2e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cf36); };
            c.w32(7, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091cf39); };
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
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091cf4f); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a1014, 0x700091cf5b, 0x700091cf56);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091cf5b); };
            c.w64(3, t6);
            c.w32(2, 0xfeu64);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091cf64); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091cf68); };
            c.w64(9, t8);
            blk = 6; continue;
        }
        5 => {
            c.w32(2, 0x2u64);
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091cf73); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091cf77); };
            c.w64(3, t10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091cf7b); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091cf81); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            c.w64(8, c.r64(9));
            let Some(t15) = c.ld16(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091cf8a); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091cf90); };
            c.w32(1, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091cf93); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091cf9c); };
            c.w64(7, t18);
            blk = 8; continue;
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091cfa3); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091cfac); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091cfbe); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091cfc3); }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t22) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700091cfce); };
            c.w32(2, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091cfd3); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091cfd7); };
            c.w64(5, t24);
            c.w64(12, c.r64(3));
            let r25 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        12 => {
            return RegfileExit::jump(0x700091cfe8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091cff4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091cff4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091cff8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091cffe); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091d002); };
            c.w64(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091d006); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091d00a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700091d00e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d016); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091d01c); };
            c.cmp(t5, c.r32(7), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0xeu64);
            blk = 8; continue;
        }
        3 => {
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d02c); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let r7 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r7);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091d039); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x700091d045, 0x700091d040);
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d045); };
            c.w64(3, t8);
            c.w32(2, 0xfeu64);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d04e); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d052); };
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
            let Some(t11) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d064); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d068); };
            c.w64(3, t12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d06c); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d072); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            c.w64(8, c.r64(9));
            let Some(t17) = c.ld16(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091d07b); };
            c.w32(11, t17);
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d081); };
            c.w32(1, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091d084); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d08d); };
            c.w64(7, t20);
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d094); };
            c.cmp(t21, c.r32(7), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091d09d); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091d0af); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091d0b4); }
            blk = 13; continue;
        }
        13 => {
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t24) = c.ld8(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700091d0bf); };
            c.w32(2, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091d0c4); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091d0c8); };
            c.w64(5, t26);
            c.w64(12, c.r64(3));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            return RegfileExit::jump(0x700091d0d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091d0f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091d0f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091d0f8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091d102); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091d106); };
            c.w64(7, t2);
            let r3 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r3);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091d10c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700091d110); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091d114); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700091d118); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091d11c); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a2054, 0x700091d128, 0x700091d123);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091d128); };
            c.w64(3, t4);
            let Some(t5) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d12c); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d130); };
            c.w64(7, t6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(c.r64(7)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091d134); };
            c.w64(8, (t7 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x700091d13a); }
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091d142); };
            c.w32(3, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d147); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091d14b); };
            c.w64(1, t10);
            let Some(t11) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091d14f); };
            c.w64(5, t11);
            c.w64(12, c.r64(2));
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700091d156); };
            c.w64(0, t12);
            let r13 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700091d164);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091d174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            c.w64(8, c.r64(5));
            let r0 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r0);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091d17d); }
            let Some(t1) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d181); };
            c.w64(7, t1);
            let Some(t2) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d185); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091d18b); };
            c.w32(11, t4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d191); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d199); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d19f); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d1a7); };
            c.w32(0, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091d1aa); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d1b3); };
            c.w64(7, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091d1ba); };
            c.w32(0, t14);
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091d1c0); };
            c.w64(3, t15);
            let Some(t16) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d1c4); };
            c.w32(2, t16);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d1cd); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091d1da); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091d1ea); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700091d1f1); };
            c.w64(11, t21);
            c.w64(0, c.r64(11).wrapping_add(0x1c));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091d1f8); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700091d1ff); };
            c.cmp(t23, c.r64(0), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x700091d208); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d20f); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d216); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d21d); };
            c.w32(0, t28);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x700091d22f);
        }
        8 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x700091d230); };
            c.w32(11, t29);
            let r30 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r30);
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700091d23a); };
            c.w32(11, t31);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700091d23e); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091d242); };
            c.w64(11, t32);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700091d249); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700091d24d); }
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d251); };
            c.w32(11, t33);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700091d255); }
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d259); };
            c.w32(11, t34);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700091d25d); }
            let Some(t35) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091d261); };
            c.w32(11, t35);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x700091d265); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x700091d26c); }
            blk = 9; continue;
        }
        9 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091d270); };
            c.w64(11, t36);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t37) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d277); };
            c.w64(8, t37);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700091d282); };
            c.w32(11, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700091d287); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091d28e); };
            c.w64(1, t40);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091d29a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091d29e); }
            let r41 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r41);
            blk = 14; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091d2a6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091d2aa); }
            let r42 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r42);
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700091d2b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700091d2b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700091d2bd); }
            return c.call(0x70007febf4, 0x700091d2c6, 0x700091d2c1);
        }
        12 => {
            c.w64(7, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d2c9); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d2cd); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d2d1); };
            c.w64(1, t45);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091d2d8); }
            blk = 6; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091d2de); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091d2e2); }
            let r46 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r46);
            blk = 14; continue;
        }
        14 => {
            let r47 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r47);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091d2ea); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091d2eb); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091d2ec); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091d2ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700091d2ef); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x4u64);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2290)) else { return RegfileExit::fault(0x700091d2fb); };
            c.w64(3, t48);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700091d30a, 0x700091d305);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d30d); };
            c.w64(8, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091d314); }
            blk = 9; continue;
        }
        16 => {
            return RegfileExit::jump(0x700091d31c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091d334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091d334); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091d338); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091d33e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d342); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d346); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d34d); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d354); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091d358); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091d35c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091d360); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091d364); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d36d); };
            c.w64(11, t9);
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d374); };
            c.w32(0, t10);
            let Some(t11) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091d37a); };
            c.w64(3, t11);
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091d381); };
            c.w32(2, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091d387); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d38b); };
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
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091d3a1); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700091d3a9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d3b6); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091d3bd); };
            c.w32(0, t18);
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d3c1); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d3c8); };
            c.w32(11, t21);
            let r22 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r22);
            c.w64(6, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t23) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x700091d3d2); };
            c.w32(2, t23);
            let Some(t24) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d3d6); };
            c.w32(8, t24);
            c.w64(12, c.r64(6));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700091d3e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700091d3e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700091d3e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700091d3ec); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 30; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700091d3fa); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700091d407); };
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
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091d427); };
            c.w64(7, t28);
            let Some(t29) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700091d42e); };
            c.w64(7, t29);
            c.w64(0, c.r64(6).wrapping_add(0x8));
            c.w64(3, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091d439); };
            c.w64(2, t30);
            let Some(t31) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700091d440); };
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
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091d455); };
            c.w64(0, t32);
            if !c.st64(c.r64(0), c.r64(3)) { return RegfileExit::fault(0x700091d45c); }
            let r33 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r33);
            blk = 12; continue;
        }
        12 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700091d463); }
            c.w64(0, c.r64(7));
            let r34 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r34);
            c.logic(r34, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700091d470); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700091d474); }
            let Some(t35) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091d478); };
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
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700091d48d); };
            c.w32(2, t36);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x700091d491); }
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700091d4b2); }
            c.w64(1, c.r64(11));
            c.w64(8, c.r64(6));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700091d4bd); };
            c.w64(0, t40);
            c.w64(10, 0x700091d4cbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700091d4cb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700091d4cf); }
            return c.call(c.r64(0), 0x700091d4d5, 0x700091d4d3);
        }
        18 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700091d4d5); }
            let Some(t41) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700091d4dd); };
            c.w64(4, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d4e2); };
            c.w64(15, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d4e6); };
            c.w64(9, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091d4ea); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700091d4ee); };
            c.w64(7, t45);
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(7));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d4f5); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091d4f9); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091d4fd); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091d501); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700091d51f); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 22; continue;
        }
        22 => {
            let r50 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091d533); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700091d53a); };
            c.w64(3, t52);
            let Some(t53) = c.ld64(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091d53d); };
            c.w64(0, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x700091d541); }
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091d557); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091d55b); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091d563); }
            c.w32(0, 0x2u64);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700091d56a); };
            c.w64(3, t56);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700091d578, 0x700091d573);
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
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091d587); };
            c.w64(11, t57);
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x700091d58e); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 27; continue;
        }
        27 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091d598); };
            c.w64(7, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700091d59c); }
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(0);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091d5a9); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091d5b4); };
            c.w64(7, t61);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700091d5b8); }
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091d5bb); };
            c.w64(7, t62);
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r63 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r63);
            let r64 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r64);
            c.logic(r64, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700091d5d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x700091d5d7); }
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091d5db); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700091d5df); };
            c.w64(0, t66);
            c.w64(10, 0x700091d5edu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700091d5ed); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700091d5f1); }
            return c.call(c.r64(0), 0x700091d5f7, 0x700091d5f5);
        }
        29 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700091d5f7); }
            let Some(t67) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700091d5ff); };
            c.w64(4, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700091d604); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d608); };
            c.w64(15, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d60c); };
            c.w64(9, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d610); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091d614); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091d618); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091d61c); };
            c.w64(0, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091d620); };
            c.w64(2, t75);
            blk = 31; continue;
        }
        30 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091d626); };
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
            let Some(t79) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d640); };
            c.w32(3, t79);
            let r80 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r80);
            let Some(t81) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d646); };
            c.w32(3, t81);
            let r82 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r82);
            let Some(t83) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d64c); };
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
            return RegfileExit::fault(0x700091d661);
        }
        35 => {
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d662); };
            c.w32(3, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091d666); };
            c.w64(2, t86);
            let Some(t87) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700091d66d); };
            c.w64(6, t87);
            c.w64(8, c.r64(6).wrapping_add(0x18));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091d674); };
            c.w64(12, t88);
            let Some(t89) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700091d67b); };
            c.cmp(t89, c.r64(8), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x700091d685); }
            c.w64(2, c.r64(6).wrapping_add(0x1));
            blk = 37; continue;
        }
        37 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x6));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700091d693); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700091d696); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091d699); };
            c.w64(0, t90);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700091d6a0); }
            c.w64(0, c.r64(2).wrapping_add(0x10));
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700091d6a7); };
            c.w64(3, t91);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700091d6ae); }
            c.w64(3, c.r64(1));
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700091d6b4); }
            let Some(t92) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d6b7); };
            c.w32(0, t92);
            let r93 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r93);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x700091d6bd); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700091d6c0); }
            let Some(t94) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d6c4); };
            c.w64(7, t94);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091d6c8); };
            c.w32(11, t95);
            let Some(t96) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091d6cd); };
            c.w64(1, t96);
            let Some(t97) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091d6d1); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700091d6e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x700091d6ea); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x700091d6f6, 0x700091d6f1);
        }
        40 => {
            c.w64(7, c.r64(0));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d6f9); };
            c.w64(15, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d6fd); };
            c.w64(9, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091d701); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091d705); };
            c.w64(11, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091d709); };
            c.w64(8, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091d70d); };
            c.w64(6, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091d711); };
            c.w64(12, t105);
            blk = 12; continue;
        }
        41 => {
            c.w64(0, c.r64(9));
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            let r106 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r106);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091d724); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700091d72c); };
            c.w64(3, t107);
            c.w64(6, c.r64(2));
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700091d73e, 0x700091d739);
        }
        42 => {
            return RegfileExit::fault(0x700091d73e);
        }
        43 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r108 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r108);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091d746); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700091d74a); };
            c.w64(3, t109);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700091d75a, 0x700091d755);
        }
        44 => {
            return RegfileExit::fault(0x700091d75a);
        }
        45 => {
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(0).wrapping_sub(0x1));
            let r110 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r110);
            let r111 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r111);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091d767); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091d76b); }
            c.w32(11, 0x4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091d773); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700091d775); };
            c.w64(3, t112);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700091d788, 0x700091d783);
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091d797); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700091d79b); };
            c.w64(3, t115);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700091d7ab, 0x700091d7a6);
        }
        48 => {
            return RegfileExit::fault(0x700091d7ab);
        }
        49 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700091d7b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091d7b5); }
            return c.call(0x70007febf4, 0x700091d7be, 0x700091d7b9);
        }
        50 => {
            c.w64(2, c.r64(0));
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d7c1); };
            c.w64(15, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d7c5); };
            c.w64(9, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d7c9); };
            c.w64(7, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700091d7cd); };
            c.w64(11, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091d7d1); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091d7d5); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091d7d9); };
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
            let Some(t125) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091d7ec); };
            c.w32(6, t125);
            let Some(t126) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d7f2); };
            c.w32(8, t126);
            let r127 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r127);
            let r128 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r128);
            let Some(t129) = c.ld32(c.r64(8).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d7fc); };
            c.w32(6, t129);
            let r130 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r130);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700091d804); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091d808); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091d80a); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700091d80b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091d80c); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x4u64);
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2270)) else { return RegfileExit::fault(0x700091d815); };
            c.w64(3, t131);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700091d824, 0x700091d81f);
        }
        52 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d824); };
            c.w64(7, t132);
            let Some(t133) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091d828); };
            c.w64(7, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091d82c); };
            c.w64(8, t134);
            c.w64(9, c.r64(8).wrapping_add(0x6));
            let Some(t135) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091d834); };
            c.w32(11, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d839); };
            c.w64(15, t136);
            let Some(t137) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091d83d); };
            c.w64(1, t137);
            let Some(t138) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091d841); };
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
            let Some(t142) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091d85d); };
            c.logic(t142 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(3, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091d86c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091d86d); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f13b4, 0x700091d87a, 0x700091d875);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091d87a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091d87c); }
            blk = 28; continue;
        }
        56 => {
            return RegfileExit::jump(0x700091d884);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091d894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091d897); };
            c.w64(6, t0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d89b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d8a1); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d8a7); };
            c.w32(7, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091d8aa); };
            c.cmp(t6, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d8b3); };
            c.w64(6, t7);
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091d8ba); };
            c.w64(3, t8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091d8be); };
            c.w32(7, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091d8c4); };
            c.cmp(t10, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7).wrapping_mul(2).wrapping_add(0x1f));
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091d8df); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d8f0); };
            c.w32(6, t13);
            let r14 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r14);
            let r15 = c.r32(6) & 0x3fu64;
            c.w32(6, r15);
            c.logic(r15, 32);
            let Some(t16) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091d8f8); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let r18 = c.add(c.r64(6), 0x69u64, 64);
            c.w64(6, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d905); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091d90b); };
            c.w32(8, t21);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(6), c.r64(8), 64);
            if c.cc_b() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700091d91a);
        }
        6 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091d91b); };
            c.w32(3, t22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091d91f); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700091d926); };
            c.w64(8, t24);
            c.w64(11, c.r64(8).wrapping_add(0x18));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091d92d); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700091d934); };
            c.cmp(t26, c.r64(11), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x700091d93d); }
            c.w64(6, c.r64(8).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700091d944); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x700091d949); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091d94c); };
            c.w64(3, t27);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700091d953); }
            c.w64(8, c.r64(6).wrapping_add(0x10));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700091d95a); };
            c.w64(11, t28);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700091d961); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700091d965); }
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091d969); };
            c.w32(8, t29);
            let r30 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r30);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x700091d971); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700091d975); }
            blk = 9; continue;
        }
        9 => {
            let r31 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r31);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091d97c); };
            c.w32(3, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091d981); };
            c.w64(1, t33);
            c.w64(0, c.r64(6));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091d98a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091d98e); }
            let r34 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x700091d994); };
            c.w64(7, t35);
            let Some(t36) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700091d99b); };
            c.w64(11, t36);
            c.w64(0, c.r64(11).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x700091d9a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091d9a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700091d9aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091d9ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700091d9b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700091d9b6); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x700091d9ba); };
            c.w64(2, t37);
            let Some(t38) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700091d9c1); };
            c.cmp(t38, c.r64(0), 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(2, 0x1cu64);
            return c.call(0x70007fec74, 0x700091d9d0, 0x700091d9cb);
        }
        12 => {
            blk = 14; continue;
        }
        13 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x700091d9d2); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 14; continue;
        }
        14 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x700091d9dc); };
            c.w64(3, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700091d9e3); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700091d9e8); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700091d9eb); }
            if !c.st32(c.r64(0).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x700091d9ee); }
            if !c.st32(c.r64(0).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x700091d9f5); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x700091d9fc); };
            c.w64(3, t40);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x700091da03); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x700091da06); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700091da0d); };
            c.w64(6, t42);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x700091da10); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r43 = c.r64(7) & c.r64(0);
            c.w64(7, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091da23); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700091da2d); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091da30); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091da34); };
            c.w64(8, t46);
            if !c.st32(c.r64(6).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700091da38); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r47 = c.r64(3) & c.r64(6);
            c.w64(3, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091da4a); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(7, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091da57); };
            c.w64(12, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091da5b); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091da5f); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091da63); };
            c.w64(3, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091da6a); }
            blk = 4; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091da70); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091da74); }
            let r53 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r53);
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x700091da7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091da83); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700091da87); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700091da8b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091da8f); }
            return c.call(0x70007febf4, 0x700091da98, 0x700091da93);
        }
        20 => {
            c.w64(6, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091da9b); };
            c.w64(12, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091da9f); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091daa3); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091daa7); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091daab); };
            c.w64(3, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091dab2); }
            blk = 8; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091dab8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091dabc); }
            let r59 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r59);
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091dac2); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r61);
            let Some(t62) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091dac8); };
            c.w32(3, t62);
            let r63 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r63);
            let Some(t64) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dace); };
            c.w32(6, t64);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x700091dad7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091dadb); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700091dadf); }
            c.cmp(c.r64(6), 0x69u64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x700091dae9);
        }
        23 => {
            let Some(t65) = c.ld32(c.r64(3).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x700091daea); };
            c.w32(3, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091daf0); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700091daf7); };
            c.w64(7, t67);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091dafe); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091db02); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091db09); };
            c.cmp(t69, c.r64(8), 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700091db18, 0x700091db13);
        }
        25 => {
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x700091db1a); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(6));
            blk = 27; continue;
        }
        27 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091db24); };
            c.w64(3, t70);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700091db28); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x700091db2d); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091db30); };
            c.w64(3, t71);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700091db37); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700091db3a); }
            c.w64(6, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091db40); };
            c.w64(12, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091db44); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091db48); };
            c.w64(9, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091db4f); }
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
            let Some(t77) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091db64); };
            c.logic(t77 & 0x2u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(7, c.r64(6).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091db72); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091db73); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(6));
            return c.call(0x70007f0e54, 0x700091db80, 0x700091db7b);
        }
        30 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091db80); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091db82); }
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
            let Some(t80) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091db97); };
            c.logic(t80 & 0x2u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091dba5); }
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x700091dbae, 0x700091dba9);
        }
        33 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091dbae); }
            blk = 16; continue;
        }
        34 => {
            return RegfileExit::jump(0x700091dbb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091dbd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091dbd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091dbd8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091dbde); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091dbe2); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091dbe6); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dbed); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091dbf4); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091dbf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091dbfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091dc00); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091dc04); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091dc12); };
            c.w64(0, t9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dc19); };
            c.w32(3, t10);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091dc22); };
            c.w32(2, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091dc28); };
            c.w64(11, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091dc2f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700091dc33); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091dc37); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(2) & 0x20u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091dc4d); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700091dc54); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dc61); };
            c.w32(6, t16);
            let r17 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r17);
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091dc67); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091dc6e); };
            c.w32(3, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x700091dc72); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700091dc76); }
            c.logic(c.r32(3) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r32(3) & 0x200000u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dc8e); };
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
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091dc9d); };
            c.w64(3, t22);
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091dca4); };
            c.w64(11, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091dcab); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 98; continue;
        }
        13 => {
            let Some(t25) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dcbd); };
            c.w32(3, t25);
            let r26 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r26);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700091dcc8); };
            c.w64(3, t27);
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091dccf); };
            c.w32(0, t28);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x1000u64, 64);
            if c.cc_a() { blk = 97; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091dce9); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700091dcf0); };
            c.w64(7, t30);
            c.w64(1, c.r64(2).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(1).wrapping_add(c.r64(7)));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091dd00); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700091dd07); };
            c.cmp(t32, c.r64(8), 64);
            if c.cc_be() { blk = 76; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(12), c.r64(8)) { return RegfileExit::fault(0x700091dd10); }
            let r33 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r33);
            blk = 18; continue;
        }
        18 => {
            let r34 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x700091dd1a); };
            c.w64(8, t35);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700091dd21); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700091dd25); }
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dd28); };
            c.w32(2, t36);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 78; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700091dd3d); }
            let Some(t37) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dd40); };
            c.w32(8, t37);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 79; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700091dd54); }
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dd57); };
            c.w32(2, t38);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 80; continue; }
            blk = 21; continue;
        }
        21 => {
            let r39 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700091dd6c); }
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dd6f); };
            c.w32(0, t40);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x700091dd87); }
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dd8a); };
            c.w32(0, t41);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x4u64, 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700091dd9f); }
            c.w64(0, c.r64(7).wrapping_add(c.r64(1)).wrapping_sub(0x1));
            c.w64(2, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(2), c.r32(11)) { return RegfileExit::fault(0x700091ddb0); }
            let r42 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r42);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 24; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t43) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091ddbc); };
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
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700091ddd5); };
            c.w32(0, t45);
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700091ddd9); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(11, c.r64(7));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091dde6); };
            c.w64(9, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ddea); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091ddee); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091ddf2); };
            c.w64(8, t49);
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ddf6); };
            c.w32(0, t50);
            let r51 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091ddfc); }
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091de00); };
            c.cmp(t52, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091de0d); };
            c.w32(3, t53);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091de13); };
            c.w32(2, t54);
            c.w64(12, c.r64(3));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700091de26); };
            c.cmp(t55, c.r32(2), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700091de33); };
            c.cmp(t56, c.r32(2), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 85; continue; }
            blk = 33; continue;
        }
        33 => {
            let r57 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r57);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091de53); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700091de5a); };
            c.w64(11, t59);
            c.w64(1, c.r64(3).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(11)));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091de65); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700091de6c); };
            c.cmp(t61, c.r64(6), 64);
            if c.cc_be() { blk = 83; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x700091de75); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700091de7c); }
            c.w64(11, c.r64(7));
            let r62 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r62);
            c.logic(r62, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700091de89); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700091de8d); }
            let Some(t63) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091de91); };
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
            let Some(t64) = c.ld32(c.r64(0).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700091dea7); };
            c.w32(2, t64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x700091deaa); }
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700091decb); }
            c.w64(8, c.r64(3));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700091ded3); };
            c.w64(0, t68);
            c.w64(10, 0x700091dee1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700091dee1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700091dee5); }
            return c.call(c.r64(0), 0x700091deeb, 0x700091dee9);
        }
        41 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700091deeb); }
            let Some(t69) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700091def3); };
            c.w64(4, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091def8); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091defc); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091df00); };
            c.w64(7, t72);
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(7));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091df07); };
            c.w64(7, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091df0b); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091df0f); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091df13); };
            c.w64(11, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091df17); };
            c.w64(8, t77);
            blk = 51; continue;
        }
        43 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 91; continue; }
            blk = 44; continue;
        }
        44 => {
            if let Some(sr) = c.shl(c.r64(3), 0x2u64, 64) { c.w64(3, sr); }
            c.w64(11, c.r64(3).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700091df35); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 87; continue; }
            blk = 45; continue;
        }
        45 => {
            let r78 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091df49); };
            c.w64(3, t79);
            let Some(t80) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700091df50); };
            c.w64(1, t80);
            c.w64(6, c.r64(11).wrapping_add(c.r64(1)));
            let Some(t81) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091df57); };
            c.cmp(t81, c.r64(6), 64);
            if c.cc_be() { blk = 89; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x700091df61); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700091df68); }
            c.w64(3, 0xfffffffffffc0000u64);
            let r82 = c.r64(3) & c.r64(11);
            c.w64(3, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091df76); };
            c.logic(t83 & 0x4u64, 8);
            if c.cc_ne() { blk = 103; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700091df80); }
            c.w64(2, c.r64(0).wrapping_add(0x7));
            c.w64(1, c.r64(11).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r84 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r84);
            let r85 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r85);
            c.logic(r85, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700091df97); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091df9c); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091dfa0); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700091dfa4); };
            c.w64(0, t87);
            c.w64(10, 0x700091dfb2u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700091dfb2); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700091dfb6); }
            return c.call(c.r64(0), 0x700091dfbc, 0x700091dfba);
        }
        49 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700091dfbc); }
            let Some(t88) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700091dfc4); };
            c.w64(4, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091dfc9); };
            c.w64(0, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091dfcd); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091dfd1); };
            c.w64(9, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091dfd5); };
            c.w64(7, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091dfd9); };
            c.w64(6, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091dfdd); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091dfe1); };
            c.w64(11, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091dfe5); };
            c.w64(8, t96);
            blk = 51; continue;
        }
        50 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091dfeb); };
            c.w64(0, t97);
            blk = 51; continue;
        }
        51 => {
            c.w64(3, c.r64(8));
            let Some(t98) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091dff5); };
            c.w32(2, t98);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091dffd); };
            c.w64(8, t99);
            let Some(t100) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700091e004); };
            c.w64(12, t100);
            c.w64(15, c.r64(2).wrapping_add(0x8));
            c.w64(6, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091e00f); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700091e016); };
            c.cmp(t102, c.r64(6), 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x700091e01f); }
            c.w64(1, c.r64(12).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700091e027); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700091e02a); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700091e02e); }
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700091e035); };
            c.w64(0, t103);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700091e03c); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091e040); };
            c.w64(0, t104);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700091e044); }
            let Some(t105) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091e048); };
            c.w32(11, t105);
            let r106 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r106);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x700091e050); }
            c.cmp(c.r64(2), 0xcu64, 64);
            if c.cc_e() { blk = 74; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091e05e); };
            c.w64(3, t107);
            c.w32(0, 0xcu64);
            c.w64(11, c.r64(0));
            blk = 55; continue;
        }
        55 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e06a); };
            c.w32(6, t108);
            let r109 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r109);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t110) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e077); };
            c.w32(8, t110);
            let Some(t111) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091e07b); };
            c.cmp(t111, c.r32(8), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st32(c.r64(1).wrapping_add(c.r64(0)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700091e084); }
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
            let Some(t113) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e09e); };
            c.w32(8, t113);
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700091e0a3); }
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
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091e0c3); };
            c.w64(3, t115);
            blk = 66; continue;
        }
        66 => {
            c.w64(6, c.r64(0).wrapping_sub(0x1));
            let Some(t116) = c.ld32(c.r64(1).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e0ce); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e0dc); };
            c.w32(12, t118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091e0e0); };
            c.cmp(t119, c.r32(12), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700091e0e9); }
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091e0ef); };
            c.w64(8, t120);
            let Some(t121) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700091e0f6); };
            c.w64(12, t121);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091e0fe); };
            c.w64(7, t122);
            let Some(t123) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700091e105); };
            c.cmp(t123, c.r64(15), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x700091e10e); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700091e116); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700091e119); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(7)) { return RegfileExit::fault(0x700091e11e); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r124 = c.r64(8) & c.r64(1);
            c.w64(8, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091e131); };
            c.logic(t125 & 0x4u64, 8);
            if c.cc_ne() { blk = 100; continue; }
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
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e145); };
            c.w64(7, t127);
            blk = 74; continue;
        }
        74 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x6));
            let Some(t128) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e150); };
            c.w64(7, t128);
            let Some(t129) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091e154); };
            c.w32(11, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e159); };
            c.w64(15, t130);
            let Some(t131) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e15d); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e161); };
            c.w64(5, t132);
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(12, c.r64(7));
            let r133 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r133);
            return RegfileExit::jump(c.r64(1));
        }
        75 => {
            blk = 98; continue;
        }
        76 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700091e179); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700091e17d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700091e181); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091e185); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x700091e191, 0x700091e18c);
        }
        77 => {
            c.w64(7, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e194); };
            c.w64(15, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091e198); };
            c.w64(6, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091e19c); };
            c.w64(3, t136);
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091e1a0); };
            c.w64(11, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091e1a7); };
            c.w64(1, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091e1ab); };
            c.w64(0, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091e1af); };
            c.w64(2, t140);
            blk = 18; continue;
        }
        78 => {
            return RegfileExit::fault(0x700091e1b8);
        }
        79 => {
            return RegfileExit::fault(0x700091e1b9);
        }
        80 => {
            return RegfileExit::fault(0x700091e1ba);
        }
        81 => {
            return RegfileExit::fault(0x700091e1bb);
        }
        82 => {
            return RegfileExit::fault(0x700091e1bc);
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700091e1bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x700091e1c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700091e1c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x700091e1c9); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x700091e1d5, 0x700091e1d0);
        }
        84 => {
            c.w64(7, c.r64(0));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e1d8); };
            c.w64(15, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e1dc); };
            c.w64(9, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091e1e0); };
            c.w64(0, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091e1e4); };
            c.w64(2, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091e1e8); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700091e1ec); };
            c.w64(12, t146);
            blk = 35; continue;
        }
        85 => {
            c.w64(11, c.r64(9));
            let r147 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r147);
            let r148 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r148);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e1ff); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700091e207); };
            c.w64(3, t149);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700091e216, 0x700091e211);
        }
        86 => {
            return RegfileExit::fault(0x700091e216);
        }
        87 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r150 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r150);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e21e); }
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700091e222); };
            c.w64(3, t151);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700091e232, 0x700091e22d);
        }
        88 => {
            return RegfileExit::fault(0x700091e232);
        }
        89 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r152 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r152);
            let r153 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r153);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091e23c); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091e240); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700091e244); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700091e245); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x700091e249); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x700091e24d); }
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700091e251); };
            c.w64(3, t154);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700091e265, 0x700091e260);
        }
        90 => {
            c.w64(11, c.r64(0));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091e268); };
            c.w64(0, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091e26c); };
            c.w64(2, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700091e270); };
            c.w64(12, t157);
            blk = 47; continue;
        }
        91 => {
            c.w64(11, c.r64(9));
            let r158 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r158);
            let r159 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r159);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e283); }
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700091e287); };
            c.w64(3, t160);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700091e297, 0x700091e292);
        }
        92 => {
            return RegfileExit::fault(0x700091e297);
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091e298); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700091e29c); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x700091e2a8, 0x700091e2a3);
        }
        94 => {
            c.w64(1, c.r64(0));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e2ab); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e2af); };
            c.w64(7, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091e2b3); };
            c.w64(2, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091e2b7); };
            c.w64(0, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091e2bb); };
            c.w64(11, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091e2bf); };
            c.w64(3, t166);
            blk = 53; continue;
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700091e2c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091e2cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700091e2d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700091e2d4); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x700091e2d8); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700091e2e5, 0x700091e2e0);
        }
        96 => {
            c.w64(7, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e2e8); };
            c.w64(9, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091e2ec); };
            c.w64(1, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091e2f0); };
            c.w64(2, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091e2f4); };
            c.w64(0, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091e2f8); };
            c.w64(6, t171);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700091e2fc); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091e301); };
            c.w64(3, t172);
            c.w32(11, 0xcu64);
            blk = 70; continue;
        }
        97 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091e313); };
            c.w64(0, t173);
            blk = 98; continue;
        }
        98 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r174 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r174);
            let Some(t175) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e31e); };
            c.w32(3, t175);
            let Some(t176) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e324); };
            c.w32(1, t176);
            let r177 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r177);
            let r178 = c.r64(2) & 0x1fu64;
            c.w64(2, r178);
            c.logic(r178, 64);
            let Some(t179) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e330); };
            c.w64(6, t179);
            let Some(t180) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e334); };
            c.w32(3, t180);
            let r181 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r181);
            let r182 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r182);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e33d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091e341); }
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091e342); };
            if !c.push64(t183) { return RegfileExit::fault(0x700091e342); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700091e345); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091e346); }
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x2280)) else { return RegfileExit::fault(0x700091e347); };
            c.w64(3, t184);
            c.w32(0, 0x4u64);
            return c.call(0x700086ed74, 0x700091e358, 0x700091e353);
        }
        99 => {
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e358); };
            c.w64(7, t185);
            let Some(t186) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e35c); };
            c.w64(7, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e360); };
            c.w64(11, t187);
            c.w64(9, c.r64(11).wrapping_add(0x6));
            let Some(t188) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091e368); };
            c.w32(11, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e36d); };
            c.w64(15, t189);
            let Some(t190) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e371); };
            c.w64(1, t190);
            let Some(t191) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e375); };
            c.w64(5, t191);
            c.w64(12, c.r64(7));
            let r192 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r192);
            return RegfileExit::jump(c.r64(1));
        }
        100 => {
            c.w32(7, c.r32(7));
            let r193 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r193);
            c.w64(8, 0xfffffffffffc0000u64);
            let r194 = c.r64(8) & c.r64(7);
            c.w64(8, r194);
            c.logic(r194, 64);
            let Some(t195) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091e391); };
            c.logic(t195 & 0x2u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091e3a0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091e3a1); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700091e3ae, 0x700091e3a9);
        }
        102 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091e3ae); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091e3b0); }
            blk = 72; continue;
        }
        103 => {
            c.w32(2, c.r32(2));
            let r196 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r196);
            c.w64(3, 0xfffffffffffc0000u64);
            let r197 = c.r64(3) & c.r64(2);
            c.w64(3, r197);
            c.logic(r197, 64);
            let Some(t198) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091e3c5); };
            c.logic(t198 & 0x2u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(1, c.r64(11).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091e3d3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091e3d4); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f13b4, 0x700091e3e1, 0x700091e3dc);
        }
        105 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091e3e1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091e3e3); }
            blk = 48; continue;
        }
        106 => {
            return RegfileExit::jump(0x700091e3ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091e3f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091e3f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091e3f8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e3fe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e402); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e406); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091e40c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e413); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e417); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e41e); };
            c.w32(0, t9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x700091e424); };
            c.w32(3, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e42a); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091e42e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091e432); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091e436); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091e43a); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091e443); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t14) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e44e); };
            c.w64(1, t14);
            let r15 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r15);
            let r16 = c.r64(0) & 0x1fu64;
            c.w64(0, r16);
            c.logic(r16, 64);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r17 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091e460); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(2, c.r64(8));
            return c.call(0x70008425f4, 0x700091e478, 0x700091e473);
        }
        3 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e478); };
            c.w64(7, t18);
            let Some(t19) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e47c); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e480); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0x6));
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091e488); };
            c.w32(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e48d); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e491); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e495); };
            c.w64(5, t24);
            c.w64(12, c.r64(7));
            let r25 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091e4a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091e4b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091e4b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091e4b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e4be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e4c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e4c6); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091e4ca); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e4d1); };
            c.w32(0, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e4d7); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e4df); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e4e3); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e4ea); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e4f0); };
            c.w32(3, t13);
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091e4f7); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e4fe); };
            c.w32(11, t17);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091e502); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091e506); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091e50a); }
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091e50e); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091e517); };
            c.w64(8, t19);
            blk = 2; continue;
        }
        2 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r20 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r20);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e525); }
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(8));
            return c.call(0x70008a8734, 0x700091e534, 0x700091e52f);
        }
        3 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e534); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e538); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e53c); };
            c.w64(8, t23);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t24) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091e544); };
            c.w32(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e549); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e54d); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e551); };
            c.w64(5, t27);
            c.w64(12, c.r64(7));
            let r28 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r28);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091e560);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091e574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091e574); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091e578); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e57e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e582); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091e586); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e58c); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e592); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e59a); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e59e); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091e5a4); };
            c.w32(3, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e5aa); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e5b0); };
            c.w32(8, t13);
            let r14 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e5b8); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091e5bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700091e5c3); }
            let Some(t17) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e5c7); };
            c.cmp(t17, 0x81u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700091e5d0); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            blk = 2; continue;
        }
        2 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e5d6); };
            c.w32(11, t20);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_ae() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r21 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r21);
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e5ec); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let r24 = c.add(c.r64(9), 0x6u64, 64);
            c.w64(9, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700091e5f7); }
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e600); }
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(7));
            return c.call(0x70007f9f34, 0x700091e612, 0x700091e60d);
        }
        5 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e612); };
            c.w64(2, t25);
            let Some(t26) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e616); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e61a); };
            c.w64(9, t27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091e61e); };
            c.w32(7, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e623); };
            c.w64(15, t29);
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e627); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e62b); };
            c.w64(5, t31);
            c.w64(12, c.r64(2));
            let r32 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r32);
            return RegfileExit::jump(c.r64(1));
        }
        6 => {
            return RegfileExit::fault(0x700091e638);
        }
        7 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e63e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091e642); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091e644); }
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x26d8)) else { return RegfileExit::fault(0x700091e64d); };
            c.w64(3, t33);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700091e65c, 0x700091e657);
        }
        9 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e65c); };
            c.w64(2, t34);
            let Some(t35) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e660); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e664); };
            c.w64(9, t36);
            let Some(t37) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091e668); };
            c.w32(7, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e66d); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e671); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e675); };
            c.w64(5, t40);
            c.w64(12, c.r64(2));
            let r41 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r41);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091e682); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091e686); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091e688); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x26e0)) else { return RegfileExit::fault(0x700091e689); };
            c.w64(3, t42);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700091e69d, 0x700091e698);
        }
        11 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e69d); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e6a1); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e6a5); };
            c.w64(9, t45);
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091e6a9); };
            c.w32(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e6ae); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e6b2); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e6b6); };
            c.w64(5, t49);
            c.w64(12, c.r64(2));
            let r50 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        12 => {
            return RegfileExit::jump(0x700091e6c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091e6d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091e6d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091e6d8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e6e2); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091e6e8); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e6ec); };
            c.w32(2, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e6f2); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e6fa); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e6fe); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091e706); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700091e707); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2708)) else { return RegfileExit::fault(0x700091e70b); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700091e71f, 0x700091e71a);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e71f); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091e723); };
            c.w64(7, t11);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e72a); };
            c.w64(2, t12);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x4));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091e735); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700091e73a); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e741); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e745); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700091e754);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091e774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091e774); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091e778); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e782); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091e788); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e78c); };
            c.w64(2, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e792); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e798); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e7a0); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e7a4); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e7a8); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091e7b0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091e7b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700091e7b3); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2710)) else { return RegfileExit::fault(0x700091e7b7); };
            c.w64(3, t11);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700091e7ce, 0x700091e7c9);
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e7ce); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091e7d2); };
            c.w64(7, t13);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e7d9); };
            c.w64(2, t14);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x6));
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091e7e4); };
            c.w32(7, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700091e7e9); };
            c.w64(15, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e7f0); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091e7f4); };
            c.w64(5, t18);
            c.w64(12, c.r64(2));
            let r19 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r19);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700091e804);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091e814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e814); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e81a); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w64(7, c.r64(5));
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e825); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e82b); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e82f); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091e833); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700091e83a); };
            c.w64(11, t7);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x10));
            c.w64(0, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091e849); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700091e850); };
            c.cmp(t9, c.r64(0), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x700091e855); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e85c); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091e863); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e86a); };
            c.w32(0, t14);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x79u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x700091e87a);
        }
        4 => {
            let r15 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r15);
            let r16 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x700091e882); };
            c.w32(11, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700091e889); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700091e88d); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700091e891); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x700091e894); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091e8a2); };
            c.w64(6, t18);
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700091e8a9); }
            let r19 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r19);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            let r20 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091e8ba); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e8bf); };
            c.w64(1, t22);
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091e8c8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091e8cc); }
            let r23 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x700091e8d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091e8d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700091e8da); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091e8de); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700091e8e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091e8e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700091e8ea); }
            return c.call(0x70007febf4, 0x700091e8f3, 0x700091e8ee);
        }
        9 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091e8f6); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091e8fa); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091e8fe); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091e902); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091e906); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091e90a); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091e90e); };
            c.w64(8, t30);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091e915); }
            blk = 2; continue;
        }
        10 => {
            return RegfileExit::jump(0x700091e91c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091e934(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091e934); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e93a); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w64(7, c.r64(5));
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e945); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091e94b); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091e94f); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091e953); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700091e95a); };
            c.w64(11, t7);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x10));
            c.w64(0, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091e969); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700091e970); };
            c.cmp(t9, c.r64(0), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x700091e975); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091e97c); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091e983); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091e98a); };
            c.w32(0, t14);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x7bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x700091e99a);
        }
        4 => {
            let r15 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r15);
            let r16 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x1f3)) else { return RegfileExit::fault(0x700091e9a2); };
            c.w32(11, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700091e9a9); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700091e9ad); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700091e9b1); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x700091e9b4); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091e9c2); };
            c.w64(6, t18);
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700091e9c9); }
            let r19 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r19);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            let r20 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091e9da); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091e9df); };
            c.w64(1, t22);
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091e9e8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091e9ec); }
            let r23 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x700091e9f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091e9f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700091e9fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091e9fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700091ea02); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091ea06); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700091ea0a); }
            return c.call(0x70007febf4, 0x700091ea13, 0x700091ea0e);
        }
        9 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ea16); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091ea1a); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ea1e); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091ea22); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091ea26); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091ea2a); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091ea2e); };
            c.w64(8, t30);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091ea35); }
            blk = 2; continue;
        }
        10 => {
            return RegfileExit::jump(0x700091ea3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ea54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091ea54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091ea58); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ea62); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091ea68); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ea6c); };
            c.w64(2, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091ea72); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ea78); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ea80); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091ea84); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ea88); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091ea90); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091ea91); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700091ea93); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2718)) else { return RegfileExit::fault(0x700091ea97); };
            c.w64(3, t11);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700091eaae, 0x700091eaa9);
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091eaae); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091eab2); };
            c.w64(7, t13);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091eab9); };
            c.w64(2, t14);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x6));
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091eac4); };
            c.w32(7, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700091eac9); };
            c.w64(15, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ead0); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ead4); };
            c.w64(5, t18);
            c.w64(12, c.r64(2));
            let r19 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r19);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700091eae4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091eaf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091eaf7); };
            c.w64(7, t0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091eafb); };
            c.w32(8, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091eb01); };
            c.w32(11, (t2 as u16 as i16 as i64 as u64));
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091eb07); };
            c.w32(3, (t3 as u8 as i8 as i64 as u64));
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_g() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091eb12); };
            c.w64(7, t4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091eb16); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            c.w32(3, c.r32(8));
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091eb20); };
            c.w32(1, t7);
            let r8 = c.sub(c.r32(1), 0x5u64, 32);
            c.w32(1, r8);
            let r9 = c.sub(c.r32(1), c.r32(3), 32);
            c.w32(1, r9);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700091eb32); }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091eb36); };
            c.w64(2, t10);
            let r11 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091eb3d); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091eb42); };
            c.w64(1, t13);
            c.w64(12, c.r64(2));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091eb4b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091eb4f); }
            let r14 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r14);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r15 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r15);
            let Some(t16) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091eb5c); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091eb60); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091eb66); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            let Some(t21) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091eb6c); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700091eb72); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091eb76); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700091eb7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700091eb7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700091eb82); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700091eb86); }
            let Some(t23) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091eb8a); };
            c.cmp(t23, 0x5eu64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, c.r32(8));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091eb98); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700091eb9c); }
            c.w64(6, c.r64(7));
            return c.call(0x70007fcf54, 0x700091eba8, 0x700091eba3);
        }
        6 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091eba8); };
            c.w64(2, t24);
            let Some(t25) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ebac); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ebb0); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ebb7); };
            c.w32(11, t28);
            let r29 = c.sub(c.r32(11), 0x5u64, 32);
            c.w32(11, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091ebbf); };
            c.w64(9, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091ebc3); };
            let r32 = c.sub(c.r64(9), t31, 64);
            c.w64(9, r32);
            let Some(t33) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091ebc7); };
            let r34 = c.sub(c.r32(11), t33, 32);
            c.w32(11, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700091ebcb); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ge() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t35) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091ebd5); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091ebd9); };
            c.w64(11, t36);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091ebdd); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091ebe1); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2148)) else { return RegfileExit::fault(0x700091ebe2); };
            c.w64(3, t37);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x700091ebf6, 0x700091ebf1);
        }
        8 => {
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ebf6); };
            c.w64(2, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091ebfa); };
            c.w64(9, t39);
            blk = 10; continue;
        }
        9 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700091ec00); }
            blk = 10; continue;
        }
        10 => {
            let Some(t40) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ec04); };
            c.w64(2, t40);
            let Some(t41) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091ec08); };
            c.w32(7, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091ec0d); };
            c.w64(15, t42);
            let Some(t43) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ec11); };
            c.w64(1, t43);
            let Some(t44) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ec15); };
            c.w64(5, t44);
            c.w64(12, c.r64(2));
            let Some(t45) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700091ec1c); };
            c.w64(0, t45);
            let r46 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r46);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091ec27); }
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091ec2b); };
            c.w64(5, t47);
            c.w64(6, c.r64(7));
            let r48 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(0x70007fd494);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091ec3b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091ec3f); }
            let r49 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r49);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r50 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r50);
            let Some(t51) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091ec4c); };
            c.w64(3, t51);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091ec50); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091ec54); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700091ec55); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091ec59); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700091ec5d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700091ec61); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700091ec65); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x1u64);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2148)) else { return RegfileExit::fault(0x700091ec71); };
            c.w64(3, t52);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700091ec83, 0x700091ec7e);
        }
        13 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ec83); };
            c.w64(0, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091ec87); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091ec8b); };
            c.w64(2, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091ec8f); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091ec93); };
            c.w64(8, t57);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091ec9a); }
            blk = 3; continue;
        }
        14 => {
            return RegfileExit::jump(0x700091eca4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ecb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ecb7); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ecbb); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ecc1); };
            c.w32(2, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ecc7); };
            c.w32(6, t4);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091ecce); }
            let r5 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ecd4); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ecd9); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x700091ece0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ecf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ecf7); };
            c.w32(2, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ecfd); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ed05); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ed09); };
            c.w32(2, t4);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ed10); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ed19); };
            c.w32(6, t7);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091ed20); }
            let r8 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ed26); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ed2b); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x700091ed34);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ed54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ed54); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ed5a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ed62); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091ed66); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ed73); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ed78); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ed81); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ed88); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ed91); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091ed98); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ed9e); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091eda3); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091edac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091edb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091edb4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091edba); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091edc2); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091edc6); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091edd3); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091edd8); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ede1); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ede8); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091edf1); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091edf8); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091edfe); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ee03); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091ee0c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ee14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ee14); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ee1a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ee22); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091ee26); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ee33); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ee38); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ee41); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ee48); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ee51); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091ee58); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ee5e); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ee63); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091ee6c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ee74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ee74); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ee7a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ee82); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091ee86); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ee93); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ee98); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091eea1); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091eea8); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091eeb1); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091eeb8); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091eebe); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091eec3); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091eecc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091eed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091eed4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091eedd); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091eeea); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091eeef); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            c.w64(3, c.r64(5));
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091eef8); };
            c.w32(2, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091eefe); };
            c.w32(6, t6);
            let r7 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r7);
            let Some(t8) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ef06); };
            c.w64(3, t8);
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ef0a); };
            c.w32(2, t9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ef11); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ef1a); };
            c.w32(6, t12);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091ef21); }
            let r13 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ef27); };
            c.w32(3, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ef2c); };
            c.w64(1, t15);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091ef34);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091ef54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091ef54); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ef5a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ef62); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091ef66); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ef73); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091ef78); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ef81); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091ef88); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091ef91); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091ef98); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091ef9e); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091efa3); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091efac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091efb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091efb4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091efba); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091efc2); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091efc6); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091efd3); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091efd8); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091efe1); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091efe8); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091eff1); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091eff8); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091effe); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f003); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091f00c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f014); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f01a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f022); };
            c.w32(3, t3);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f02a); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f030); };
            c.cmp(t6, 0xbau64, 16);
            if c.cc_b() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f03b); };
            c.w64(2, t7);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f042); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f04b); };
            c.w32(6, t10);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f052); }
            let r11 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f058); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f05d); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r14 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r14);
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f067); };
            c.w32(3, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f06c); };
            c.w64(1, t16);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091f074);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f094); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f09a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f0a2); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091f0a6); };
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
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700091f0b5); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f0be); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091f0c4); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091f0ca); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f0d3); };
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
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f0e4); };
            c.w64(2, t11);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f0eb); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f0f4); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f0fb); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f101); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f106); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            let r18 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f110); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f115); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f11b); };
            c.logic(t21 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091f126); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x700091f138);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f154); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f15a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f162); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091f166); };
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
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700091f175); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f17e); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091f184); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091f18a); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f193); };
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
            let r11 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f1a5); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f1aa); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            c.w64(2, c.r64(5));
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f1b3); };
            c.w64(2, t14);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f1ba); };
            c.w32(2, t15);
            let r16 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r16);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f1c3); };
            c.w32(6, t17);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f1ca); }
            let r18 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f1d0); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f1d5); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f1db); };
            c.logic(t21 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091f1e6); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x700091f1f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f214); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091f21a); };
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
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700091f229); };
            c.cmp(t2, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f232); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091f238); };
            c.logic(t5 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091f23e); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f247); };
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
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f258); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f25c); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f262); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f269); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f26f); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f274); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            let r15 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f27e); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f283); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f289); };
            c.logic(t18 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091f294); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x700091f2a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f2b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f2b4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091f2ba); };
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
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700091f2c9); };
            c.cmp(t2, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f2d2); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091f2d8); };
            c.logic(t5 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091f2de); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f2e7); };
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
            let r8 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f2f9); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f2fe); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            c.w64(2, c.r64(5));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f307); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f30b); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f311); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f318); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f31e); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f323); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f329); };
            c.logic(t18 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091f334); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x700091f348);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f354); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091f35a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f367); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f36c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f375); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f379); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f37f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f386); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f38c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f391); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091f398);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f3b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f3b4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091f3ba); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f3c7); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f3cc); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f3d5); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f3d9); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f3df); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f3e6); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f3ec); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f3f1); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091f3f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f414(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f414); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091f41a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f427); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f42c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f435); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f439); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f43f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f446); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f44c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f451); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091f458);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f474); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091f47a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f487); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f48c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f495); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f499); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f49f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f4a6); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f4ac); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f4b1); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091f4b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f4d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f4d4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091f4da); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f4e7); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f4ec); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f4f5); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f4f9); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f4ff); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f506); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f50c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f511); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091f518);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f534); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091f53a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f547); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f54c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f555); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f559); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f55f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f566); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f56c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f571); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091f578);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091f594); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091f59d); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f5aa); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f5af); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            c.w64(3, c.r64(5));
            let Some(t5) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f5b8); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f5bc); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f5c2); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f5c8); };
            c.w32(6, t9);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f5cf); }
            let r10 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f5d5); };
            c.w32(3, t11);
            let Some(t12) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f5da); };
            c.w64(1, t12);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091f5e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f5f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f5f4); };
            c.w32(3, t0);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f5fe); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f604); };
            c.cmp(t3, 0xbau64, 16);
            if c.cc_b() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f60f); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f613); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f619); };
            c.w32(6, t7);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x3));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f620); }
            let r8 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f626); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f62b); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r11 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f635); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f63a); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091f644);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091f657); };
            c.w64(2, (t0 as u16 as i16 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f660); };
            c.w32(6, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f666); };
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
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f680); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f688); };
            c.w64(2, t7);
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f68c); };
            c.w32(3, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f693); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f69c); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x7));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091f6a3); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f6a9); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f6ae); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r15 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f6b8); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f6bd); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091f6c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f6d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f6d4); };
            c.w64(2, (t0 as u16 as i16 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f6dd); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f6e1); };
            c.w32(0, t2);
            let r3 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091f6e7); };
            c.w64(8, t4);
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091f6eb); };
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
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f70c); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091f70f); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x700091f718); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f721); };
            c.cmp(t10, 0x834u64, 16);
            if c.cc_ne() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091f72c); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091f731); };
            c.w32(3, t12);
            let r13 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091f738); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f741); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091f748); };
            c.w32(1, t17);
            c.logic(c.r16(1) & 0x3ffu64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 12; continue;
        }
        9 => {
            let r18 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f759); };
            c.w32(2, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f75e); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091f764); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091f768); }
            let r21 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091f76e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091f772); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091f776); }
            blk = 26; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091f77f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091f783); }
            let r22 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091f789); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091f78d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091f791); }
            blk = 26; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091f79a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091f79e); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091f7a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091f7a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091f7ac); }
            blk = 26; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091f7b5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091f7b9); }
            let r24 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091f7bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091f7c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091f7c7); }
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f7d8); };
            c.w32(11, t25);
            let r26 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r26);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700091f7e5); };
            c.w64(11, t27);
            blk = 16; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f7ec); };
            c.w32(3, t28);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x700091f7fc);
        }
        18 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f7ff); };
            c.cmp(t29, c.r32(3), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(2));
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f808); };
            c.w32(3, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091f80c); };
            c.cmp(t31, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x700091f815); };
            c.cmp(t32, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f81e); };
            c.cmp(t33, 0x834u64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091f828); };
            c.cmp(t34, c.r32(3), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091f82e); };
            c.w32(11, t35);
            let r36 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700091f835); };
            c.cmp(t37, c.r32(11), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t38) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f83e); };
            c.w32(0, t38);
            let r39 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r39);
            let Some(t40) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091f845); };
            c.w32(3, t40);
            c.logic(c.r16(3) & 0x3ffu64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r41);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091f856); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091f85a); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1f50)) else { return RegfileExit::fault(0x700091f85b); };
            c.w64(3, t42);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x700091f86f, 0x700091f86a);
        }
        27 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091f86f); };
            c.w64(7, t43);
            let Some(t44) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091f873); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091f877); };
            c.w64(9, t45);
            let r46 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091f87f); };
            c.w32(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f884); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f888); };
            c.w64(1, t49);
            let Some(t50) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091f88c); };
            c.w64(5, t50);
            c.w64(12, c.r64(2));
            let r51 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r51);
            return RegfileExit::jump(c.r64(1));
        }
        28 => {
            let r52 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r52);
            let Some(t53) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f89d); };
            c.w32(2, t53);
            let Some(t54) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f8a2); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091f8a6); };
            c.w64(5, t55);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700091f8aa); };
            c.w64(0, t56);
            let r57 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r57);
            return RegfileExit::jump(c.r64(1));
        }
        29 => {
            return RegfileExit::jump(0x700091f8b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091f8d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091f8d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091f8d8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091f8de); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f8e2); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091f8e6); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f8ec); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f8f2); };
            c.w32(6, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f8f8); };
            c.w32(7, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091f8fb); }
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091f8ff); };
            c.cmp(t9, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091f908); };
            c.w64(2, t10);
            blk = 2; continue;
        }
        2 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091f90f); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091f912); };
            c.cmp(t12, c.r32(2), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700091f921); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x700091f92e); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091f934); };
            c.w32(7, t16);
            let r17 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r17);
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091f93a); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f941); };
            c.w32(11, t20);
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091f945); };
            c.w32(1, t21);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f94b); };
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
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091f974); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700091f98a); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091f98f); }
            blk = 9; continue;
        }
        9 => {
            c.w32(2, c.r64(1).wrapping_add(c.r64(1)));
            blk = 10; continue;
        }
        10 => {
            let Some(t27) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091f999); };
            c.w64(6, (t27 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x700091f99f); }
            let r28 = c.sub(c.r64(6), 0x1u64, 64);
            c.w64(6, r28);
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(7)) { return RegfileExit::fault(0x700091f9a7); }
            let r29 = c.sub(c.r64(6), 0x1u64, 64);
            c.w64(6, r29);
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x700091f9af); }
            let r30 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r30);
            let Some(t31) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091f9b7); };
            c.w32(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091f9bc); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091f9c0); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091f9c4); };
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
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091f9d4); };
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
            if !c.st32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700091f9e7); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091f9eb); }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091f9f2); };
            c.w32(2, t38);
            c.w64(7, c.r64(0));
            blk = 10; continue;
        }
        15 => {
            return RegfileExit::jump(0x700091f9fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fa14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091fa14); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091fa1d); };
            c.w64(6, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fa23); };
            c.w64(3, t2);
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fa27); };
            c.w64(2, t3);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700091fa2f); };
            c.w64(3, t4);
            blk = 3; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700091fa38); };
            c.w64(3, t5);
            blk = 3; continue;
        }
        3 => {
            let r6 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r6);
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091fa43); };
            c.w32(2, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fa48); };
            c.w64(1, t8);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700091fa54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fa74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091fa74); };
            c.w64(2, (t0 as u16 as i16 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091fa7d); };
            c.w64(8, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091fa83); };
            c.w64(11, (t2 as u16 as i16 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fa89); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fa8d); };
            c.w64(8, t4);
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091fa95); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091fa99); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fa9f); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700091faa3); };
            c.w64(11, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091faa7); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091faad); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091fab3); };
            c.w32(6, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091fab6); };
            c.cmp(t14, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091fabf); };
            c.w64(3, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091fac6); };
            c.w32(6, t16);
            c.cmp(c.r32(8), c.r32(6), 32);
            if c.cc_ae() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fad1); };
            c.w32(0, t17);
            let r18 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r18);
            c.w64(8, c.r64(9).wrapping_add(0x9));
            let Some(t19) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091fadd); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091fae3); };
            c.w32(2, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fae8); };
            c.w64(1, t21);
            c.w64(9, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            return RegfileExit::fault(0x700091faf1);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091faf2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091faf6); }
            let r22 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r22);
            let Some(t23) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091fafc); };
            c.w64(11, t23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700091fb00); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700091fb04); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091fb08); }
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091fb0c); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091fb15); };
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
            if !c.st32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x700091fb28); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091fb2c); }
            blk = 9; continue;
        }
        9 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r27 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r27);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700091fb39); }
            c.w64(6, c.r64(11));
            c.w64(3, c.r64(2));
            return c.call(0x7000857fb4, 0x700091fb48, 0x700091fb43);
        }
        10 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091fb48); };
            c.w64(7, t28);
            let Some(t29) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091fb4c); };
            c.w64(2, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091fb50); };
            c.w64(8, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700091fb54); };
            c.w32(7, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091fb59); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fb5d); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091fb61); };
            c.w64(5, t34);
            c.w64(12, c.r64(2));
            c.w64(9, c.r64(8));
            let r35 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r35);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            return RegfileExit::jump(0x700091fb74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fb94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091fb94); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fb9d); };
            c.w64(3, t1);
            let r2 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r2);
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            let r3 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091fbab); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fbb0); };
            c.w64(1, t5);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x700091fbb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fbd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700091fbd4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r1 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091fbe1); };
            c.w32(2, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fbe6); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091fbec); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091fbf0); }
            let r4 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091fbf6); };
            c.w32(2, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fbfc); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091fc04); };
            c.w64(8, t8);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fc0c); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091fc13); };
            c.w64(7, t11);
            let r12 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r12);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091fc1a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091fc1e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700091fc1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700091fc23); }
            c.w32(0, 0x1u64);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2228)) else { return RegfileExit::fault(0x700091fc2c); };
            c.w64(3, t13);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700091fc3b, 0x700091fc36);
        }
        3 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091fc3b); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091fc3f); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091fc43); };
            c.w64(8, t16);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091fc47); }
            c.w32(2, 0x62u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091fc50); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x700091fc51); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700091fc65, 0x700091fc60);
        }
        4 => {
            return RegfileExit::fault(0x700091fc65);
        }
        5 => {
            return RegfileExit::jump(0x700091fc68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fc74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091fc74); };
            c.w64(2, (t0 as u16 as i16 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fc7d); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091fc81); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091fc85); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700091fc8c); };
            c.logic(t5 & 0x40u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r6 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r6);
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091fc97); };
            c.w32(2, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fc9c); };
            c.w64(1, t8);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091fca2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091fca6); }
            let r9 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r9);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t10) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091fcb0); };
            c.w64(0, t10);
            let r11 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r11);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091fcb7); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091fcbb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091fcbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700091fcbd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700091fcc1); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1da8)) else { return RegfileExit::fault(0x700091fcc5); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x700091fcd9, 0x700091fcd4);
        }
        3 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091fcd9); };
            c.w64(7, t13);
            let Some(t14) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091fcdd); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091fce1); };
            c.w64(8, t15);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091fce5); }
            c.w32(7, 0x62u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091fcee); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x700091fcef); };
            c.w64(3, t16);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700091fd03, 0x700091fcfe);
        }
        4 => {
            return RegfileExit::fault(0x700091fd03);
        }
        5 => {
            return RegfileExit::jump(0x700091fd08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fd14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091fd14); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fd1d); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091fd21); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091fd2a); };
            c.w32(6, t3);
            c.w64(7, 0xfffffffffffffffcu64);
            if !c.st32(c.r64(3).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700091fd34); }
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091fd37); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            if !c.st64(c.r64(2).wrapping_sub(0x8), c.r64(3)) { return RegfileExit::fault(0x700091fd3d); }
            c.w64(3, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091fd44); };
            c.w32(6, t6);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r7 = c.add(c.r64(3), c.r64(6), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fd50); };
            c.w32(6, t8);
            let r9 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r9);
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091fd58); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fd5c); };
            c.w32(3, t11);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091fd63); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fd6c); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x7));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700091fd73); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091fd79); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fd7e); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            let r18 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700091fd88); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fd8d); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700091fd94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fdb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091fdb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091fdb8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091fdc2); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091fdc8); }
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091fdcc); };
            c.w64(7, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091fdd2); };
            c.w64(8, t4);
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fdd6); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091fdda); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091fdde); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091fde5); };
            c.w32(15, t9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fdeb); };
            c.w32(12, t10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091fdf1); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x15)) else { return RegfileExit::fault(0x700091fdf7); };
            c.w32(8, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700091fdfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091fe00); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091fe04); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091fe08); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091fe0c); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(0));
            blk = 4; continue;
        }
        1 => {
            c.w64(7, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(2)) { return RegfileExit::fault(0x700091fe23); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r14 = c.r64(11) & c.r64(3);
            c.w64(11, r14);
            c.logic(r14, 64);
            let Some(t15) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091fe35); };
            c.logic(t15 & 0x4u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(0, c.r64(7));
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_ae() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(2, c.r64(0).wrapping_add(0x3));
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091fe4c); };
            c.w32(7, t16);
            let Some(t17) = c.ld64(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fe4f); };
            c.w64(2, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(0), c.r64(7), 64);
            if c.cc_b() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 20; continue;
        }
        7 => {
            let r18 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r18);
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(7)), c.r32(2)) { return RegfileExit::fault(0x700091fe6f); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r19 = c.r64(9) & c.r64(3);
            c.w64(9, r19);
            c.logic(r19, 64);
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091fe81); };
            c.logic(t20 & 0x4u64, 8);
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
            let r21 = c.sub(c.r64(2), c.r64(6), 64);
            c.w64(2, r21);
            let Some(t22) = c.ld64(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700091fe9b); };
            c.w64(2, t22);
            c.w64(7, c.r64(8).wrapping_add(c.r64(6)));
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091fea3); };
            c.w32(9, t23);
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
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091febb); };
            c.w64(7, t24);
            c.w32(8, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091fec3); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091fec7); };
            c.w64(11, t26);
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700091fecb); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r27 = c.r64(12) & c.r64(9);
            c.w64(12, r27);
            c.logic(r27, 64);
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091fedf); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st32(c.r64(9).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x700091fee7); }
            if !c.st32(c.r64(9).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x700091feeb); }
            let Some(t29) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091feef); };
            c.w64(2, t29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091fef3); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091fefa); };
            c.w32(9, t32);
            let r33 = c.sub(c.r32(7), 0x21u64, 32);
            c.w32(7, r33);
            let r34 = c.sub(c.r32(9), 0x9u64, 32);
            c.w32(9, r34);
            let r35 = c.sub(c.r32(9), c.r32(7), 32);
            c.w32(9, r35);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700091ff0e); }
            blk = 16; continue;
        }
        16 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091ff12); };
            c.w64(0, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091ff19); }
            return c.ret(0x0, 0x700091ff1a);
        }
        17 => {
            let Some(t37) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091ff1b); };
            c.w64(7, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091ff1f); };
            c.w64(8, t38);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091ff23); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700091ff27); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2140)) else { return RegfileExit::fault(0x700091ff28); };
            c.w64(3, t39);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700091ff3c, 0x700091ff37);
        }
        18 => {
            blk = 16; continue;
        }
        19 => {
            return RegfileExit::fault(0x700091ff3e);
        }
        20 => {
            return RegfileExit::fault(0x700091ff3f);
        }
        21 => {
            c.w32(11, c.r32(11));
            let r40 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r40);
            c.w64(12, 0xfffffffffffc0000u64);
            let r41 = c.r64(12) & c.r64(11);
            c.w64(12, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091ff50); };
            c.logic(t42 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(15, c.r64(9).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091ff5c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091ff5d); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700091ff6a, 0x700091ff65);
        }
        23 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091ff6a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091ff6c); }
            blk = 14; continue;
        }
        24 => {
            c.w32(2, c.r32(2));
            let r43 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r43);
            c.w64(9, 0xfffffffffffc0000u64);
            let r44 = c.r64(9) & c.r64(2);
            c.w64(9, r44);
            c.logic(r44, 64);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091ff81); };
            c.logic(t45 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091ff90); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091ff91); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700091ff9e, 0x700091ff99);
        }
        26 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091ff9e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091ffa0); }
            blk = 9; continue;
        }
        27 => {
            c.w32(2, c.r32(2));
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            c.w64(11, 0xfffffffffffc0000u64);
            let r47 = c.r64(11) & c.r64(2);
            c.w64(11, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091ffb5); };
            c.logic(t48 & 0x2u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091ffc4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091ffc5); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700091ffd2, 0x700091ffcd);
        }
        29 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091ffd2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091ffd4); }
            blk = 3; continue;
        }
        30 => {
            return RegfileExit::jump(0x700091ffdc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700091fff4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091fff4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091fff8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091fffe); };
            c.w64(3, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920002); };
            c.w64(2, (t2 as u16 as i16 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920008); };
            c.w64(6, t3);
            let Some(t4) = c.ld64(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700092000c); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000920010); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000920016); };
            c.w32(7, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x23)) else { return RegfileExit::fault(0x700092001c); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(6).wrapping_add(0x15)) else { return RegfileExit::fault(0x7000920023); };
            c.w32(6, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000920027); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            let r11 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x198)) else { return RegfileExit::fault(0x7000920034); };
            c.w64(1, t12);
            if !c.st32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700092003b); }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(0, c.r64(6).wrapping_add(c.r64(11)));
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920049); };
            c.w32(1, t13);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_ae() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920058); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            c.w64(15, 0xfffffffffffffffau64);
            let r16 = c.sub(c.r64(15), c.r64(11), 64);
            c.w64(15, r16);
            if !c.st64(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(8)), c.r64(1)) { return RegfileExit::fault(0x700092006a); }
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092006e); };
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
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x700092007f); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            let r20 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920089); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092008e); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920092); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920096); };
            c.w64(5, t24);
            let r25 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            return RegfileExit::fault(0x70009200a0);
        }
        8 => {
            return RegfileExit::fault(0x70009200a1);
        }
        9 => {
            return RegfileExit::jump(0x70009200a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009200b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009200b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009200b8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009200c2); };
            c.w64(8, t1);
            let r2 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r2);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70009200c8); }
            let Some(t3) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009200cc); };
            c.w64(7, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009200d0); };
            c.w64(11, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009200d6); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009200dc); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009200e0); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009200e4); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009200ea); };
            c.w32(3, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70009200ed); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009200f1); };
            c.cmp(t12, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009200fa); };
            c.w64(7, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000920101); };
            c.w32(3, t14);
            let Some(t15) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920107); };
            c.w32(2, t15);
            c.w64(1, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(2, c.r64(7));
            return c.call(0x70008a8954, 0x7000920123, 0x700092011e);
        }
        3 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920123); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000920127); };
            c.w64(8, t17);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t18) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092012e); };
            c.w64(7, t18);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_add(0x8));
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000920139); };
            c.w32(8, t19);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x700092013e); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920145); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920149); };
            c.w64(5, t22);
            c.w64(12, c.r64(7));
            let r23 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000920158);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000920174); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000920178); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092017e); };
            c.w64(2, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920182); };
            c.w32(7, t2);
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920188); };
            c.w64(8, t3);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000920190); };
            c.w64(3, t4);
            let r5 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r5);
            let r6 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r6);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000920199); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700092019d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009201a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70009201a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009201a9); }
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(7));
            return c.call(0x70008a86d4, 0x70009201bb, 0x70009201b6);
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009201bb); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009201bf); };
            c.w64(2, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009201c3); };
            c.w64(7, t9);
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009201cb); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009201d0); };
            c.w64(15, t11);
            let Some(t12) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009201d4); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009201d8); };
            c.w64(5, t13);
            c.w64(12, c.r64(2));
            let Some(t14) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70009201df); };
            c.w64(0, t14);
            let r15 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r15);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009201ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009201f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009201f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009201f8); }
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009201fa); };
            c.w32(2, t0);
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r1 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r1);
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920206); };
            c.w64(7, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092020c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000920210); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000920211); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000920224, 0x700092021f);
        }
        1 => {
            return RegfileExit::fault(0x7000920224);
        }
        2 => {
            return RegfileExit::jump(0x7000920228);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000920234); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000920238); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920242); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000920246); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092024c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000920250); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000920251); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000920255); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000920259); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700092025d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x7000920271, 0x700092026c);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920274); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920278); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092027c); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000920280); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920284); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700092028b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000920298);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009202b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009202b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009202b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009202c2); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009202c6); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009202cc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009202d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70009202d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009202d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009202d9); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70009202dd); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x70009202f1, 0x70009202ec);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009202f4); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009202f8); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009202fc); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000920300); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920304); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700092030b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000920318);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000920334); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000920338); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920342); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000920346); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092034c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000920350); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000920351); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000920355); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000920359); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700092035d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x7000920371, 0x700092036c);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920374); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920378); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092037c); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000920380); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920384); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700092038b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000920398);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009203b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009203b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009203b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009203c2); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009203c6); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009203cc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009203d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70009203d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009203d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009203d9); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70009203dd); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x70009203f1, 0x70009203ec);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009203f4); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009203f8); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009203fc); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000920400); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920404); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700092040b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000920418);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000920434); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000920438); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920442); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000920446); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092044c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000920450); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000920451); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000920455); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000920459); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700092045d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x7000920471, 0x700092046c);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920474); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920478); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092047c); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000920480); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920484); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700092048b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000920498);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009204b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009204b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009204b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009204c2); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009204c6); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009204cc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009204d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70009204d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009204d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009204d9); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70009204dd); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x70009204f1, 0x70009204ec);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009204f4); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009204f8); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009204fc); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000920500); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920504); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700092050b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000920518);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920534); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092053c); };
            c.w64(3, t1);
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920544); };
            c.w32(2, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920549); };
            c.w64(1, t4);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000920554);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920574); };
            c.w32(3, t0);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            let r1 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920580); };
            c.w32(3, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920585); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x700092058c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920594); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920599); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009205a1); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009205ac); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009205b1); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70009205b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009205d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009205d4); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009205dc); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009205e0); };
            c.w32(2, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009205e5); };
            c.w32(6, t3);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009205ef); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009205fe); };
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
            return RegfileExit::fault(0x700092060b);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092060c); };
            c.w32(6, t8);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x700092061a);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092061b); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920626); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092062b); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            return RegfileExit::jump(0x7000920634);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920654); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092065c); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000920660); };
            c.w32(2, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000920665); };
            c.w32(6, t3);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092066f); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092067e); };
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
            return RegfileExit::fault(0x700092068b);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092068c); };
            c.w32(6, t8);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x700092069a);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092069b); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009206a6); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009206ab); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            return RegfileExit::jump(0x70009206b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009206d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009206d7); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009206db); };
            c.w32(2, t1);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009206e1); };
            c.w32(6, t2);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70009206ee);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009206ef); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009206fa); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009206ff); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000920708);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000920717); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092071b); };
            c.w32(2, t1);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920721); };
            c.w32(6, t2);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700092072e);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092072f); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092073a); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092073f); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000920748);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920754); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            if !c.st64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x700092075c); }
            let r1 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920764); };
            c.w32(3, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920769); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000920770);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920774); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092077c); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000920780); };
            c.w64(6, (t2 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(3)) { return RegfileExit::fault(0x7000920785); }
            let r3 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092078d); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920792); };
            c.w64(1, t5);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x700092079c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009207b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009207b7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009207bb); };
            c.w64(6, (t1 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x70009207c0); }
            if !c.st64(c.r64(3).wrapping_sub(0x8), c.r64(0)) { return RegfileExit::fault(0x70009207c4); }
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009207cc); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009207d1); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70009207d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009207f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009207f4); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009207fc); };
            c.w64(3, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x8), c.r64(3)) { return RegfileExit::fault(0x7000920800); }
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920808); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092080d); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000920814);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920834); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092083c); };
            c.w64(3, t1);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000920844); };
            c.w64(3, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700092084d); };
            c.w64(3, t3);
            blk = 3; continue;
        }
        3 => {
            let r4 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r4);
            let Some(t5) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920858); };
            c.w32(2, t5);
            let Some(t6) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700092085d); };
            c.w64(1, t6);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000920868);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000920874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920877); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092087b); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920882); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920889); };
            c.w32(0, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092088d); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092089b); };
            c.w64(0, t7);
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009208a2); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009208a9); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009208b6); };
            c.w64(3, (t10 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(2, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t11) = c.ld32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009208c2); };
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
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009208db); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009208e2); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let r17 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r17);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009208f3); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009208f8); };
            c.w64(1, t19);
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            let Some(t20) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000920901); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920905); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092090c); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920913); };
            c.w32(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x7000920926);
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x7000920927); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920934); };
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
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000920958); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920960); };
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
            return RegfileExit::fault(0x700092097a);
        }
        13 => {
            let r34 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r34);
            let Some(t35) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092097f); };
            c.w32(0, t35);
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920984); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092098c); };
            c.w64(1, t38);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000920995); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000920999); }
            let r39 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092099f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009209a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70009209a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70009209ab); }
            blk = 253; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009209b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009209b8); }
            let r40 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009209be); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009209c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009209c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009209cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009209d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70009209d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70009209d9); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70009209dd); };
            c.cmp(t43, c.r32(7), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t44) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009209ea); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009209ee); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009209f4); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009209fa); };
            c.w32(6, t49);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000920a03); }
            c.cmp(c.r64(6), 0x3u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x7000920a0d);
        }
        18 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000920a0e); };
            c.w32(0, t50);
            let r51 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000920a14); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x7000920a1e);
        }
        20 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000920a1f); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920a2f); };
            c.w32(6, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000920a32); };
            c.cmp(t55, c.r32(6), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920a3b); };
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
            let Some(t58) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920a47); };
            c.cmp(t58, c.r32(3), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920a50); };
            c.w32(3, t59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000920a53); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000920a5f); };
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
            let r64 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r64);
            let Some(t65) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920a8c); };
            c.w32(8, t65);
            let Some(t66) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920a91); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920a95); };
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
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920aad); };
            c.w32(6, t70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920ab2); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920aba); };
            c.w32(6, t73);
            let r74 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r74);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000920ac2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000920ac6); }
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(0, c.r64(3));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000920adf); };
            c.w64(3, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000920ae3); };
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
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000920af2); };
            c.w32(7, t78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000920af5); };
            c.w64(7, t79);
            blk = 34; continue;
        }
        34 => {
            return c.call(c.r64(7), 0x7000920aff, 0x7000920afd);
        }
        35 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920aff); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920b03); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000920b07); };
            c.w64(9, t82);
            let Some(t83) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000920b0b); };
            c.w32(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920b10); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920b14); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920b18); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000920b38); }
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
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920bc4); };
            c.w32(2, t90);
            let Some(t91) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920bc9); };
            c.w32(1, t91);
            let r92 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r92);
            let Some(t93) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920bd1); };
            c.w32(2, t93);
            let r94 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r94);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000920bd8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000920bdc); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000920bdd); };
            if !c.push64(t95) { return RegfileExit::fault(0x7000920bdd); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000920be0); }
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000920be1); };
            if !c.push64(t96) { return RegfileExit::fault(0x7000920be1); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000920be4); };
            if !c.push64(t97) { return RegfileExit::fault(0x7000920be4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000920be7); }
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000920beb); };
            c.w64(3, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000920bf2); };
            c.w64(6, t99);
            c.w32(0, 0x5u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000920c03, 0x7000920bfe);
        }
        50 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920c03); };
            c.w64(8, t100);
            let Some(t101) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920c07); };
            c.w64(8, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000920c0b); };
            c.w64(9, t102);
            let Some(t103) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000920c0f); };
            c.w32(11, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920c14); };
            c.w64(15, t104);
            let Some(t105) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920c18); };
            c.w64(1, t105);
            let Some(t106) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920c1c); };
            c.w64(5, t106);
            c.w64(12, c.r64(8));
            let r107 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r107);
            return RegfileExit::jump(c.r64(1));
        }
        51 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000920c29); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920c30); };
            c.w32(11, t110);
            let r111 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r111);
            let Some(t112) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920c37); };
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
            return RegfileExit::fault(0x7000920c51);
        }
        53 => {
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920c52); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920c5a); };
            c.w32(0, t116);
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(11, c.r64(9).wrapping_add(0x9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000920c65); }
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000920c69); };
            c.cmp(t118, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t119) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000920c72); };
            c.w32(8, t119);
            let Some(t120) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920c77); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920c7b); };
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
            let Some(t124) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920c8e); };
            c.w32(3, t124);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920c93); };
            c.w32(2, t125);
            let r126 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r126);
            let Some(t127) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920c9b); };
            c.w32(3, t127);
            let r128 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r128);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000920ca2); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000920cab); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000920cac); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000920cad); };
            c.w64(3, t129);
            c.w32(0, 0x2u64);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000920cb9); };
            c.w64(6, t130);
            return c.call(0x700086ed74, 0x7000920cc2, 0x7000920cbd);
        }
        56 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920cc2); };
            c.w64(8, t131);
            let Some(t132) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920cc6); };
            c.w64(8, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000920cca); };
            c.w64(9, t133);
            let Some(t134) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000920cce); };
            c.w32(11, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920cd3); };
            c.w64(15, t135);
            let Some(t136) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920cd7); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920cdb); };
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
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t140) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920cf3); };
            c.w32(2, t140);
            let Some(t141) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920cf8); };
            c.w32(1, t141);
            let r142 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r142);
            let Some(t143) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920d00); };
            c.w32(2, t143);
            let r144 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r144);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000920d07); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000920d0b); }
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000920d0f); };
            c.w64(6, t145);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(2));
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000920d19); };
            c.w64(1, t146);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x7000920d27, 0x7000920d22);
        }
        58 => {
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920d27); };
            c.w64(8, t147);
            let Some(t148) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920d2b); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000920d2f); };
            c.w64(9, t149);
            let Some(t150) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000920d33); };
            c.w32(11, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920d38); };
            c.w64(15, t151);
            let Some(t152) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920d3c); };
            c.w64(1, t152);
            let Some(t153) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920d40); };
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
            let Some(t155) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000920d52); };
            c.w32(0, t155);
            let r156 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r156);
            let Some(t157) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920d58); };
            c.w32(2, t157);
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000920d5b); };
            c.cmp(t158, c.r32(0), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t159) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000920d68); };
            c.w32(1, t159);
            let Some(t160) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920d6d); };
            c.w32(6, t160);
            let r161 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r161);
            let Some(t162) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920d75); };
            c.w32(1, t162);
            let r163 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r163);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t164) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920d85); };
            c.w32(2, t164);
            let r165 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r165);
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000920d8b); };
            c.cmp(t166, c.r32(2), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t167) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920d98); };
            c.w32(2, t167);
            let r168 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r168);
            let Some(t169) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920d9e); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000920da4); };
            c.logic(t171 & 0x2u64, 8);
            if c.cc_ne() { blk = 82; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000920dae); };
            c.cmp(t172, c.r32(6), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(0, c.r64(11));
            blk = 99; continue;
        }
        66 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000920dbf); };
            c.w64(11, t173);
            let Some(t174) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000920dc6); };
            c.cmp(t174, 0x20u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t175) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000920dd0); };
            c.w32(11, t175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000920dd4); };
            c.cmp(t176, c.r32(11), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t177) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920de1); };
            c.w32(11, t177);
            let r178 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r178);
            let Some(t179) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920de8); };
            c.w32(1, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000920dec); };
            c.cmp(t180, c.r32(1), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t181) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000920df5); };
            c.w32(11, t181);
            let r182 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r182);
            let Some(t183) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920dfc); };
            c.w32(1, t183);
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000920e00); };
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
            let Some(t185) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920e13); };
            c.w32(1, t185);
            let r186 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r186);
            let Some(t187) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920e1a); };
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
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000920e36); };
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
            let Some(t189) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000920e4d); };
            c.w32(11, t189);
            let r190 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r190);
            let Some(t191) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920e54); };
            c.w32(11, t191);
            let r192 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r192);
            let Some(t193) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920e5b); };
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
            let Some(t196) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000920e74); };
            c.w32(6, t196);
            let r197 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r197);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000920e7b); }
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000920e7f); };
            if !c.push64(t198) { return RegfileExit::fault(0x7000920e7f); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000920e89, 0x7000920e84);
        }
        79 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920e89); };
            c.w64(8, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920e8d); };
            c.w64(15, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000920e91); };
            c.w64(9, t201);
            blk = 99; continue;
        }
        80 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r202 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r202);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000920ea1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000920ea5); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000920ea6); }
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000920ea7); };
            if !c.push64(t203) { return RegfileExit::fault(0x7000920ea7); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000920eaa); };
            c.w64(6, t204);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000920eae); };
            c.w64(3, t205);
            c.w32(0, 0x3u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x7000920ec2, 0x7000920ebd);
        }
        81 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920ec2); };
            c.w64(8, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920ec6); };
            c.w64(15, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000920eca); };
            c.w64(9, t208);
            blk = 99; continue;
        }
        82 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r209 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r209);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000920edc); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000920ee0); };
            if !c.push64(t210) { return RegfileExit::fault(0x7000920ee0); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000920ee8); };
            c.w64(6, t211);
            return c.call(0x70007f2274, 0x7000920ef1, 0x7000920eec);
        }
        83 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920ef1); };
            c.w64(8, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000920ef5); };
            c.w64(15, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000920ef9); };
            c.w64(9, t214);
            blk = 99; continue;
        }
        84 => {
            let Some(t215) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920f02); };
            c.w32(11, t215);
            let r216 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r216);
            let Some(t217) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920f09); };
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
            let Some(t218) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920f2f); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000920f33); };
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
            let Some(t220) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920f48); };
            c.cmp(t220, 0x40u64, 16);
            if c.cc_ae() { blk = 253; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t221) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000920f53); };
            c.w32(11, t221);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 99; continue;
        }
        91 => {
            let Some(t222) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920f60); };
            c.w32(2, t222);
            let Some(t223) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000920f63); };
            c.cmp(t223, c.r32(2), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t224) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000920f70); };
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
            let Some(t226) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000920f86); };
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
            let Some(t229) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000920fa2); };
            c.w32(0, t229);
            let r230 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000920fa8); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t232) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000920fb5); };
            c.w32(11, t232);
            let Some(t233) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000920fb9); };
            c.cmp(t233, c.r32(11), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000920fc2); };
            c.w32(0, t234);
            let r235 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r235);
            blk = 99; continue;
        }
        98 => {
            let Some(t236) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000920fca); };
            c.w32(11, t236);
            let Some(t237) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000920fce); };
            c.cmp(t237, c.r32(11), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 99; continue;
        }
        99 => {
            let r238 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r238);
            let Some(t239) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000920fdf); };
            c.w64(8, t239);
            let Some(t240) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000920fe3); };
            c.w32(11, t240);
            let Some(t241) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000920fe8); };
            c.w64(1, t241);
            let Some(t242) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000920fec); };
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
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000920ffe); };
            c.w64(11, t244);
            let Some(t245) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000921005); };
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
            let Some(t246) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000921015); };
            c.w32(11, t246);
            let r247 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r247);
            blk = 105; continue;
        }
        104 => {
            let Some(t248) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092101e); };
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
            return RegfileExit::fault(0x700092102b);
        }
        107 => {
            let r250 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r250);
            c.logic(r250, 64);
            let Some(t251) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921030); };
            c.w32(2, t251);
            let r252 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r252);
            let Some(t253) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000921036); };
            c.w32(3, t253);
            let r254 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r254);
            let Some(t255) = c.ld64(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092103c); };
            c.w64(2, t255);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t256) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921045); };
            c.w32(0, t256);
            let r257 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r257);
            let Some(t258) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092104b); };
            c.w32(1, t258);
            let r259 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r259);
            blk = 109; continue;
        }
        109 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r260 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r260);
            let r261 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r261);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092105b); }
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092105f); };
            if !c.push64(t262) { return RegfileExit::fault(0x700092105f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000921062); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(0));
            return c.call(0x70007fe8d4, 0x7000921076, 0x7000921071);
        }
        110 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921076); };
            c.w64(8, t263);
            let Some(t264) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092107a); };
            c.w64(8, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092107e); };
            c.w64(9, t265);
            let Some(t266) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000921082); };
            c.w32(11, t266);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921087); };
            c.w64(15, t267);
            let Some(t268) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092108b); };
            c.w64(1, t268);
            let Some(t269) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092108f); };
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
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700092109e); };
            c.w64(11, t271);
            let Some(t272) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70009210a5); };
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
            let r275 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r275);
            let r276 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r276);
            let Some(t277) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009210c5); };
            c.w32(1, t277);
            let r278 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r278);
            let Some(t279) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70009210cb); };
            c.w32(1, t279);
            let r280 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r280);
            let Some(t281) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70009210d1); };
            c.w32(2, t281);
            let r282 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r282);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009210d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009210dc); }
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(0));
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009210e9); };
            c.w64(6, t283);
            return c.call(0x70007fea14, 0x70009210f2, 0x70009210ed);
        }
        114 => {
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009210f2); };
            c.w64(8, t284);
            let Some(t285) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009210f6); };
            c.w64(8, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009210fa); };
            c.w64(9, t286);
            let Some(t287) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009210fe); };
            c.w32(11, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921103); };
            c.w64(15, t288);
            let Some(t289) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921107); };
            c.w64(1, t289);
            let Some(t290) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092110b); };
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
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t293) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000921123); };
            c.w32(3, t293);
            let Some(t294) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921128); };
            c.w32(2, t294);
            let r295 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r295);
            let Some(t296) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921130); };
            c.w32(3, t296);
            let r297 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r297);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000921137); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092113b); }
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092113c); };
            if !c.push64(t298) { return RegfileExit::fault(0x700092113c); }
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092113f); };
            if !c.push64(t299) { return RegfileExit::fault(0x700092113f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000921142); }
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x7000921146); };
            c.w64(3, t300);
            c.w32(0, 0x3u64);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000921152); };
            c.w64(6, t301);
            return c.call(0x700086ed74, 0x700092115b, 0x7000921156);
        }
        116 => {
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092115b); };
            c.w64(8, t302);
            let Some(t303) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092115f); };
            c.w64(8, t303);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000921163); };
            c.w64(9, t304);
            let Some(t305) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000921167); };
            c.w32(11, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092116c); };
            c.w64(15, t306);
            let Some(t307) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921170); };
            c.w64(1, t307);
            let Some(t308) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921174); };
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
            let r312 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r312);
            let r313 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r313);
            let Some(t314) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092119b); };
            c.w32(3, t314);
            let r315 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r315);
            let Some(t316) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70009211a1); };
            c.w32(3, t316);
            let r317 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r317);
            let Some(t318) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70009211a7); };
            c.w32(3, t318);
            let r319 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r319);
            let Some(t320) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009211ae); };
            c.w32(7, t320);
            let r321 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r321);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009211b4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009211b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009211b9); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009211bf); };
            c.w64(6, t322);
            return c.call(0x70007f2274, 0x70009211c8, 0x70009211c3);
        }
        118 => {
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009211c8); };
            c.w64(8, t323);
            let Some(t324) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009211cc); };
            c.w64(8, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009211d0); };
            c.w64(9, t325);
            let Some(t326) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009211d4); };
            c.w32(11, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009211d9); };
            c.w64(15, t327);
            let Some(t328) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009211dd); };
            c.w64(1, t328);
            let Some(t329) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009211e1); };
            c.w64(5, t329);
            c.w64(12, c.r64(8));
            let r330 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r330);
            return RegfileExit::jump(c.r64(1));
        }
        119 => {
            let Some(t331) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009211ee); };
            c.w32(2, t331);
            let r332 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r332);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70009211f9); };
            c.w64(2, t333);
            blk = 121; continue;
        }
        121 => {
            let Some(t334) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000921200); };
            c.w32(1, t334);
            let Some(t335) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000921203); };
            c.w32(6, t335);
            let Some(t336) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921208); };
            c.w32(7, t336);
            let r337 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r337);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t338) = c.ld32(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921213); };
            c.w32(6, t338);
            let r339 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r339);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r340 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r340);
            let Some(t341) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921221); };
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
            let Some(t344) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092123a); };
            c.w32(11, t344);
            let Some(t345) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092123f); };
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
            let Some(t348) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921264); };
            c.w32(11, t348);
            let Some(t349) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000921268); };
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
            return RegfileExit::fault(0x700092127b);
        }
        127 => {
            let Some(t351) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092127c); };
            c.w32(8, t351);
            let Some(t352) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000921281); };
            c.w32(11, t352);
            let r353 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r353);
            let Some(t354) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921289); };
            c.w32(8, t354);
            let r355 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r355);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t356) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921299); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700092129d); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t358) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009212aa); };
            c.w32(2, t358);
            let r359 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r359);
            let Some(t360) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009212b1); };
            c.w32(11, t360);
            let r361 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r361);
            let Some(t362) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009212b8); };
            c.logic(t362 & 0x2u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t363) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70009212c3); };
            c.cmp(t363, c.r32(11), 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009212cc); };
            c.w64(11, t364);
            blk = 178; continue;
        }
        132 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70009212d8); };
            c.w64(11, t365);
            let Some(t366) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70009212df); };
            c.cmp(t366, 0x20u64, 8);
            if c.cc_e() { blk = 149; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t367) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70009212e9); };
            c.w32(8, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009212ed); };
            c.cmp(t368, c.r32(8), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009212f6); };
            c.w64(8, t369);
            blk = 253; continue;
        }
        135 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009212ff); };
            c.w64(0, t370);
            let Some(t371) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921303); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092130a); };
            c.w32(11, t373);
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700092130e); };
            c.cmp(t374, c.r32(11), 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t375) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000921317); };
            c.w32(8, t375);
            let r376 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r376);
            let Some(t377) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092131e); };
            c.w32(11, t377);
            let Some(t378) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000921322); };
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
            let Some(t379) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921335); };
            c.w32(11, t379);
            let r380 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092133c); };
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
            let Some(t382) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000921359); };
            c.cmp(t382, c.r32(11), 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921362); };
            c.w64(8, t383);
            blk = 253; continue;
        }
        142 => {
            let Some(t384) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092136b); };
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
            let Some(t386) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000921379); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921380); };
            c.w32(8, t388);
            let r389 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r389);
            let Some(t390) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921387); };
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
            let Some(t393) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009213a0); };
            c.w32(6, t393);
            let r394 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009213a7); };
            c.w64(8, t395);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009213ab); }
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009213af); };
            if !c.push64(t396) { return RegfileExit::fault(0x70009213af); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70009213b9, 0x70009213b4);
        }
        146 => {
            c.w64(11, c.r64(0));
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009213bc); };
            c.w64(9, t397);
            blk = 178; continue;
        }
        147 => {
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009213c5); };
            c.w64(8, t398);
            blk = 253; continue;
        }
        148 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009213ce); };
            c.w64(8, t399);
            blk = 253; continue;
        }
        149 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r400 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009213e0); };
            c.w64(3, t401);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009213e4); }
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009213e8); };
            if !c.push64(t402) { return RegfileExit::fault(0x70009213e8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009213eb); }
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009213ed); };
            if !c.push64(t403) { return RegfileExit::fault(0x70009213ed); }
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009213f0); };
            c.w64(6, t404);
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70009213f4); };
            c.w64(3, t405);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000921408, 0x7000921403);
        }
        150 => {
            c.w64(11, c.r64(0));
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092140b); };
            c.w64(9, t406);
            blk = 178; continue;
        }
        151 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r407 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092141e); };
            c.w64(0, t408);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000921422); }
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000921426); };
            if !c.push64(t409) { return RegfileExit::fault(0x7000921426); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092142e); };
            c.w64(6, t410);
            return c.call(0x70007f2274, 0x7000921437, 0x7000921432);
        }
        152 => {
            c.w64(11, c.r64(0));
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092143a); };
            c.w64(9, t411);
            blk = 178; continue;
        }
        153 => {
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000921443); };
            c.w64(0, t412);
            let Some(t413) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921447); };
            c.w32(8, t413);
            let r414 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r414);
            let Some(t415) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092144e); };
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
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921470); };
            c.w64(8, t416);
            blk = 253; continue;
        }
        157 => {
            let Some(t417) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921479); };
            c.w32(8, t417);
            let Some(t418) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700092147d); };
            c.cmp(t418, c.r32(8), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921486); };
            c.w64(8, t419);
            blk = 253; continue;
        }
        159 => {
            let Some(t420) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092148f); };
            c.w32(8, t420);
            let r421 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r421);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092149c); };
            c.w64(8, t422);
            blk = 253; continue;
        }
        161 => {
            let Some(t423) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009214a5); };
            c.w32(11, t423);
            let r424 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r424);
            let Some(t425) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009214ac); };
            c.cmp(t425, 0x40u64, 16);
            if c.cc_b() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009214b4); };
            c.w64(8, t426);
            blk = 253; continue;
        }
        163 => {
            let Some(t427) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009214bd); };
            c.w32(8, t427);
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            blk = 178; continue;
        }
        164 => {
            let Some(t428) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009214ca); };
            c.w32(11, t428);
            let Some(t429) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70009214ce); };
            c.cmp(t429, c.r32(11), 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009214d7); };
            c.w64(8, t430);
            blk = 253; continue;
        }
        166 => {
            let Some(t431) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009214e0); };
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
            let Some(t433) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009214f8); };
            c.w32(11, t433);
            let r434 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r434);
            let Some(t435) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70009214ff); };
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
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921511); };
            c.w64(8, t438);
            blk = 253; continue;
        }
        169 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 174; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t439) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000921520); };
            c.w32(11, t439);
            let r440 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000921527); };
            c.cmp(t441, c.r32(11), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t442) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921530); };
            c.w32(8, t442);
            let Some(t443) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000921534); };
            c.cmp(t443, c.r32(8), 32);
            if c.cc_ne() { blk = 178; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t444) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092153d); };
            c.w32(11, t444);
            let r445 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r445);
            blk = 178; continue;
        }
        173 => {
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921546); };
            c.w64(8, t446);
            blk = 253; continue;
        }
        174 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092154f); };
            c.w64(8, t447);
            blk = 253; continue;
        }
        175 => {
            let Some(t448) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921558); };
            c.w32(8, t448);
            let Some(t449) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700092155c); };
            c.cmp(t449, c.r32(8), 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921565); };
            c.w64(8, t450);
            blk = 253; continue;
        }
        177 => {
            let Some(t451) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092156e); };
            c.w32(11, t451);
            let r452 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r452);
            blk = 178; continue;
        }
        178 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092157c); };
            c.w64(0, t453);
            let Some(t454) = c.ld64(c.r64(0).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921580); };
            c.w64(8, t454);
            let Some(t455) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000921584); };
            c.w32(0, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921589); };
            c.w64(15, t456);
            let Some(t457) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092158d); };
            c.w64(1, t457);
            let Some(t458) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921591); };
            c.w64(5, t458);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r459 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r459);
            return RegfileExit::jump(c.r64(1));
        }
        179 => {
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009215a1); };
            c.w64(8, t460);
            blk = 253; continue;
        }
        180 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r461 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r461);
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t462) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009215b5); };
            c.w64(3, t462);
            let Some(t463) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009215b9); };
            c.w32(2, t463);
            let Some(t464) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009215be); };
            c.w32(3, t464);
            let r465 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r465);
            let Some(t466) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009215c4); };
            c.w32(3, t466);
            let r467 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r467);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009215cb); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009215cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009215d0); }
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009215d4); };
            c.w64(6, t468);
            let Some(t469) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70009215d8); };
            c.w64(3, t469);
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70009215ec, 0x70009215e7);
        }
        181 => {
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009215ec); };
            c.w64(8, t470);
            let Some(t471) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009215f0); };
            c.w64(8, t471);
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009215f4); };
            c.w64(9, t472);
            let Some(t473) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009215f8); };
            c.w32(11, t473);
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009215fd); };
            c.w64(15, t474);
            let Some(t475) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921601); };
            c.w64(1, t475);
            let Some(t476) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921605); };
            c.w64(5, t476);
            c.w64(12, c.r64(8));
            let r477 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r477);
            return RegfileExit::jump(c.r64(1));
        }
        182 => {
            let r478 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r478);
            let Some(t479) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921616); };
            c.w32(8, t479);
            let Some(t480) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092161b); };
            c.w64(1, t480);
            let Some(t481) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092161f); };
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
            let Some(t484) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921655); };
            c.w32(3, t484);
            let r485 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r485);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t486) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000921660); };
            c.w64(3, t486);
            blk = 187; continue;
        }
        187 => {
            let Some(t487) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921667); };
            c.w32(11, t487);
            let r488 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r488);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            let r489 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r489);
            let Some(t490) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921678); };
            c.w32(8, t490);
            let Some(t491) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092167d); };
            c.w64(1, t491);
            let Some(t492) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921681); };
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
            let Some(t494) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921698); };
            c.w32(0, t494);
            let Some(t495) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092169c); };
            c.cmp(t495, c.r32(0), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 191; continue;
        }
        191 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70009216a9); }
            blk = 245; continue;
        }
        192 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            let r496 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r496);
            let Some(t497) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009216bd); };
            c.w32(8, t497);
            let Some(t498) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009216c2); };
            c.w32(0, t498);
            let r499 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r499);
            let Some(t500) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009216ca); };
            c.w64(1, t500);
            let Some(t501) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009216ce); };
            c.w64(5, t501);
            let r502 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r502);
            return RegfileExit::jump(c.r64(1));
        }
        194 => {
            let Some(t503) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009216d8); };
            c.w32(11, t503);
            let r504 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r504);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t505) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009216ea); };
            c.w32(0, t505);
            let Some(t506) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009216ee); };
            c.cmp(t506, c.r32(0), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 196; continue;
        }
        196 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70009216fb); }
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
            return RegfileExit::fault(0x700092177f);
        }
        211 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r509 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r509);
            let r510 = c.r64(6) & 0xfffffu64;
            c.w64(6, r510);
            c.logic(r510, 64);
            let Some(t511) = c.ld8(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092178f); };
            c.w32(8, (t511 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t512) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921799); };
            c.w32(8, t512);
            let Some(t513) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092179e); };
            c.w64(1, t513);
            let Some(t514) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009217a2); };
            c.w64(5, t514);
            let r515 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r515);
            return RegfileExit::jump(c.r64(1));
        }
        212 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r516 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r516);
            let r517 = c.r64(6) & 0xfffffu64;
            c.w64(6, r517);
            c.logic(r517, 64);
            let Some(t518) = c.ld16(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009217bb); };
            c.w32(8, (t518 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t519) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009217c5); };
            c.w32(8, t519);
            let Some(t520) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009217ca); };
            c.w64(1, t520);
            let Some(t521) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009217ce); };
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
            let Some(t524) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009217e3); };
            c.w32(8, t524);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009217e8); }
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
            let Some(t526) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009217fc); };
            c.w64(11, t526);
            let Some(t527) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000921803); };
            c.w64(0, t527);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092180a); };
            c.w64(2, t528);
            let Some(t529) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000921811); };
            c.cmp(t529, c.r64(3), 64);
            if c.cc_a() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000921820, 0x700092181b);
        }
        217 => {
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921820); };
            c.w64(8, t530);
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921824); };
            c.w64(15, t531);
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000921828); };
            c.w64(9, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092182c); };
            c.w64(12, t533);
            blk = 219; continue;
        }
        218 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000921832); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 219; continue;
        }
        219 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000921845); };
            c.w64(8, t534);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700092184c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000921850); }
            blk = 220; continue;
        }
        220 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t535) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092185c); };
            c.w32(8, t535);
            let Some(t536) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921861); };
            c.w64(1, t536);
            let Some(t537) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921865); };
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
            let Some(t540) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092187a); };
            c.w32(8, t540);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700092187f); }
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
            let Some(t542) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000921891); };
            c.w64(11, t542);
            let Some(t543) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000921898); };
            c.w64(0, t543);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t544) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092189f); };
            c.w64(2, t544);
            let Some(t545) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70009218a6); };
            c.cmp(t545, c.r64(3), 64);
            if c.cc_a() { blk = 226; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70009218b5, 0x70009218b0);
        }
        225 => {
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009218b5); };
            c.w64(8, t546);
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009218b9); };
            c.w64(15, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009218bd); };
            c.w64(9, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009218c1); };
            c.w64(12, t549);
            blk = 227; continue;
        }
        226 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70009218c7); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 227; continue;
        }
        227 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t550) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009218dd); };
            c.w64(8, t550);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70009218e4); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009218e8); }
            c.w64(8, c.r64(0));
            blk = 228; continue;
        }
        228 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t551) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009218f7); };
            c.w32(11, t551);
            let Some(t552) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009218fc); };
            c.w64(1, t552);
            let Some(t553) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921900); };
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
            let Some(t556) = c.ld64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921918); };
            c.w64(8, t556);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700092191d); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 237; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t557) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000921927); };
            c.w64(11, t557);
            let Some(t558) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092192e); };
            c.w64(0, t558);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000921935); };
            c.w64(2, t559);
            let Some(t560) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700092193c); };
            c.cmp(t560, c.r64(3), 64);
            if c.cc_a() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700092194b, 0x7000921946);
        }
        232 => {
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092194b); };
            c.w64(8, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092194f); };
            c.w64(15, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000921953); };
            c.w64(9, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000921957); };
            c.w64(12, t564);
            blk = 234; continue;
        }
        233 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700092195d); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 234; continue;
        }
        234 => {
            let Some(t565) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000921967); };
            c.w64(3, t565);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700092196e); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000921977); }
            let r566 = c.neg(c.r64(8), 64);
            c.w64(8, r566);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000921981); }
            blk = 242; continue;
        }
        236 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000921987); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700092198e); }
            blk = 242; continue;
        }
        237 => {
            let Some(t567) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000921994); };
            c.w64(8, t567);
            let Some(t568) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092199b); };
            c.w64(11, t568);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t569) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009219a2); };
            c.w64(0, t569);
            let Some(t570) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70009219a9); };
            c.cmp(t570, c.r64(3), 64);
            if c.cc_a() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70009219b8, 0x70009219b3);
        }
        239 => {
            blk = 241; continue;
        }
        240 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70009219ba); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 241; continue;
        }
        241 => {
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70009219c4); };
            c.w64(8, t571);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70009219cb); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70009219cf); }
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009219d6); };
            c.w64(12, t572);
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009219da); };
            c.w64(15, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009219de); };
            c.w64(9, t574);
            blk = 242; continue;
        }
        242 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t575) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009219e9); };
            c.w32(8, t575);
            let Some(t576) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009219ee); };
            c.w64(1, t576);
            let Some(t577) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009219f2); };
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
            if !c.movss_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000921a07); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 245; continue;
        }
        244 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r580 = c.r64(6) & 0xfffffu64;
            c.w64(6, r580);
            c.logic(r580, 64);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000921a1e); }
            blk = 245; continue;
        }
        245 => {
            let Some(t581) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000921a24); };
            c.w64(8, t581);
            let Some(t582) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000921a2b); };
            c.w64(11, t582);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x7000921a32); }
            let Some(t583) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000921a37); };
            c.w64(3, t583);
            let Some(t584) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000921a3e); };
            c.cmp(t584, c.r64(0), 64);
            if c.cc_a() { blk = 248; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000921a4d, 0x7000921a48);
        }
        247 => {
            blk = 249; continue;
        }
        248 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000921a4f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 249; continue;
        }
        249 => {
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000921a59); };
            c.w64(11, t585);
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t586) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000921a61); };
            c.w64(8, t586);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000921a68); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x7000921a6c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000921a71); }
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000921a76); };
            c.w64(12, t587);
            let Some(t588) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921a7a); };
            c.w32(8, t588);
            let Some(t589) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921a7f); };
            c.w64(15, t589);
            let Some(t590) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921a83); };
            c.w64(1, t590);
            let Some(t591) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921a87); };
            c.w64(5, t591);
            let r592 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r592);
            return RegfileExit::jump(c.r64(1));
        }
        250 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r593 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r593);
            let r594 = c.r64(6) & 0xfffffu64;
            c.w64(6, r594);
            c.logic(r594, 64);
            let Some(t595) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921aa0); };
            c.w32(8, t595);
            let Some(t596) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921aa5); };
            c.w32(0, t596);
            let r597 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r597);
            let Some(t598) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921aac); };
            c.w64(1, t598);
            let Some(t599) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921ab0); };
            c.w64(5, t599);
            let r600 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r600);
            return RegfileExit::jump(c.r64(1));
        }
        251 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r601 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r601);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000921ac1); }
            let Some(t602) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000921ac5); };
            c.w64(8, t602);
            let Some(t603) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921acc); };
            c.w32(8, t603);
            let r604 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r604);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000921ad3); }
            let Some(t605) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000921ad5); };
            c.w64(3, t605);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000921ae8, 0x7000921ae3);
        }
        252 => {
            return RegfileExit::fault(0x7000921ae8);
        }
        253 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r606 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r606);
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t607) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000921af4); };
            c.w32(3, t607);
            let Some(t608) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921af9); };
            c.w32(2, t608);
            let r609 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r609);
            let Some(t610) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000921b01); };
            c.w64(1, t610);
            let Some(t611) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921b05); };
            c.w32(3, t611);
            let r612 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r612);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000921b0c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000921b10); }
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000921b11); };
            if !c.push64(t613) { return RegfileExit::fault(0x7000921b11); }
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000921b14); };
            if !c.push64(t614) { return RegfileExit::fault(0x7000921b14); }
            c.w32(11, 0x2u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000921b1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000921b1f); }
            let Some(t615) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x7000921b23); };
            c.w64(3, t615);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000921b3a, 0x7000921b35);
        }
        254 => {
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921b3a); };
            c.w64(8, t616);
            let Some(t617) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921b3e); };
            c.w64(8, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000921b42); };
            c.w64(9, t618);
            let Some(t619) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000921b46); };
            c.w32(11, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921b4b); };
            c.w64(15, t620);
            let Some(t621) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921b4f); };
            c.w64(1, t621);
            let Some(t622) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921b53); };
            c.w64(5, t622);
            c.w64(12, c.r64(8));
            let r623 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r623);
            return RegfileExit::jump(c.r64(1));
        }
        255 => {
            return RegfileExit::fault(0x7000921b60);
        }
        256 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000921b61); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000921b65); }
            let r624 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r624);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r625 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r625);
            let Some(t626) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000921b72); };
            c.w32(0, t626);
            let Some(t627) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921b77); };
            c.w32(3, t627);
            let r628 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r628);
            let Some(t629) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000921b7f); };
            c.w64(2, t629);
            let Some(t630) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921b83); };
            c.w32(1, t630);
            let r631 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r631);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000921b8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000921b8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000921b92); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000921b96); }
            c.w32(0, 0xcu64);
            c.w64(6, c.r64(2));
            return c.call(0x7000804df4, 0x7000921ba7, 0x7000921ba2);
        }
        257 => {
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921ba7); };
            c.w64(8, t632);
            let Some(t633) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921bab); };
            c.w64(8, t633);
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000921baf); };
            c.w64(11, t634);
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t635) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000921bb7); };
            c.w32(11, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921bbc); };
            c.w64(15, t636);
            let Some(t637) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921bc0); };
            c.w64(1, t637);
            let Some(t638) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921bc4); };
            c.w64(5, t638);
            c.w64(12, c.r64(8));
            let r639 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r639);
            return RegfileExit::jump(c.r64(1));
        }
        258 => {
            return RegfileExit::jump(0x7000921bd4);
        }
            _ => unreachable!(),
        }
    }
}
