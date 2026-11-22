#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700082f094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082f094); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082f098); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f099); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f09a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082f09f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f0a5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f0b2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f0b8); };
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082f0cb); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x1eu64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082f0d9); };
            c.w64(0, t8);
            c.w64(10, 0x700082f0e7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082f0e7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082f0eb); }
            return c.call(c.r64(0), 0x700082f0f1, 0x700082f0ef);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082f0f1); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082f0f9); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f0fe); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f105); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082f10c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f10f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f116); }
            return c.ret(0x0, 0x700082f118);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f119); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f11d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f12b, 0x700082f126);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f12b); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f139); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f13a); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082f14b, 0x700082f146);
        }
        10 => {
            return RegfileExit::fault(0x700082f14b);
        }
        11 => {
            return RegfileExit::jump(0x700082f150);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082f154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082f154); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082f158); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f159); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f15a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082f15f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f165); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f172); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f178); };
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082f18b); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x24u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082f199); };
            c.w64(0, t8);
            c.w64(10, 0x700082f1a7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082f1a7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082f1ab); }
            return c.call(c.r64(0), 0x700082f1b1, 0x700082f1af);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082f1b1); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082f1b9); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f1be); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f1c5); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082f1cc);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f1cf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f1d6); }
            return c.ret(0x0, 0x700082f1d8);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f1d9); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f1dd); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f1eb, 0x700082f1e6);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f1eb); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f1f9); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f1fa); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082f20b, 0x700082f206);
        }
        10 => {
            return RegfileExit::fault(0x700082f20b);
        }
        11 => {
            return RegfileExit::jump(0x700082f210);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082f214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082f214); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082f218); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f219); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f21a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082f21f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f225); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f232); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f238); };
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082f24b); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x20u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082f259); };
            c.w64(0, t8);
            c.w64(10, 0x700082f267u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082f267); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082f26b); }
            return c.call(c.r64(0), 0x700082f271, 0x700082f26f);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082f271); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082f279); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f27e); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f285); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082f28c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f28f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f296); }
            return c.ret(0x0, 0x700082f298);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f299); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f29d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f2ab, 0x700082f2a6);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f2ab); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f2b9); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f2ba); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082f2cb, 0x700082f2c6);
        }
        10 => {
            return RegfileExit::fault(0x700082f2cb);
        }
        11 => {
            return RegfileExit::jump(0x700082f2d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082f2d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082f2d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082f2d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f2d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f2da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082f2df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f2e5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f2f2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f2f8); };
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082f30b); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x18u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082f319); };
            c.w64(0, t8);
            c.w64(10, 0x700082f327u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082f327); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082f32b); }
            return c.call(c.r64(0), 0x700082f331, 0x700082f32f);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082f331); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082f339); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f33e); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f345); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082f34c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f34f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f356); }
            return c.ret(0x0, 0x700082f358);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f359); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f35d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f36b, 0x700082f366);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f36b); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f379); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f37a); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082f38b, 0x700082f386);
        }
        10 => {
            return RegfileExit::fault(0x700082f38b);
        }
        11 => {
            return RegfileExit::jump(0x700082f390);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082f394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082f394); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082f398); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f399); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f39a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082f39f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f3a5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f3b2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f3b8); };
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082f3cb); }
            c.w64(1, c.r64(13));
            c.w32(8, 0x22u64);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x12d0)) else { return RegfileExit::fault(0x700082f3d9); };
            c.w64(0, t8);
            c.w64(10, 0x700082f3e7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082f3e7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700082f3eb); }
            return c.call(c.r64(0), 0x700082f3f1, 0x700082f3ef);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082f3f1); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700082f3f9); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f3fe); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f405); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x700082f40c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f40f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f416); }
            return c.ret(0x0, 0x700082f418);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f419); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f41d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f42b, 0x700082f426);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f42b); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        9 => {
            c.w32(7, 0xc4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f439); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f43a); };
            c.w64(3, t13);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082f44b, 0x700082f446);
        }
        10 => {
            return RegfileExit::fault(0x700082f44b);
        }
        11 => {
            return RegfileExit::jump(0x700082f450);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082f454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082f454); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082f458); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f459); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f45a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082f45f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f465); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f46e); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f474); };
            c.cmp(t5, 0x839u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082f47c); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f482); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f489); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x8, 0x700082f490);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f493); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f49a); }
            return c.ret(0x0, 0x700082f49c);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f49d); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f4a1); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f4af, 0x700082f4aa);
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f4af); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        8 => {
            c.w32(2, 0xc4u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082f4ba); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f4bb); };
            c.w64(3, t11);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700082f4cc, 0x700082f4c7);
        }
        9 => {
            return RegfileExit::fault(0x700082f4cc);
        }
        10 => {
            return RegfileExit::jump(0x700082f4d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082f4d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082f4d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082f4d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082f4d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f4da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082f4df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082f4e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f4ed); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f4f9); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f4ff); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082f50b); };
            c.w64(2, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x700082f50f); };
            c.cmp(t7, c.r32(2), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x628)) else { return RegfileExit::fault(0x700082f51c); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x700082f529); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082f53f); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082f545); };
            c.cmp(t12, 0x40u64, 16);
            if c.cc_ae() { blk = 25; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x700082f550); };
            c.w64(0, t13);
            c.w64(1, c.r64(0));
            return c.call(0x70007fb534, 0x700082f55f, 0x700082f55a);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082f55f); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x628)) else { return RegfileExit::fault(0x700082f568); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082f56f); };
            c.w64(2, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f573); };
            c.w64(6, t17);
            return c.call(0x70007fb534, 0x700082f57c, 0x700082f577);
        }
        11 => {
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082f57c); };
            c.cmp(t18, c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x700082f585); };
            c.w64(0, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082f58c); };
            c.w64(2, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f590); };
            c.w64(6, t21);
            return c.call(0x70007fb534, 0x700082f599, 0x700082f594);
        }
        13 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082f599); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 24; continue;
        }
        15 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f5a4); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f5a8); };
            c.w64(6, t24);
            blk = 20; continue;
        }
        16 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082f5ae); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f5b2); };
            c.w64(6, t26);
            return c.call(0x70008a27d4, 0x700082f5bb, 0x700082f5b6);
        }
        17 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082f5bb); };
            c.w64(1, t27);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082f5c2); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return c.ret(0x10, 0x700082f5c9);
        }
        19 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082f5cc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082f5d3); }
            return c.ret(0x0, 0x700082f5d5);
        }
        20 => {
            return c.call(0x70008a2674, 0x700082f5db, 0x700082f5d6);
        }
        21 => {
            blk = 17; continue;
        }
        22 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700082f5dd); };
            c.w64(3, t28);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082f5eb, 0x700082f5e6);
        }
        23 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f5eb); };
            c.w64(6, t29);
            blk = 1; continue;
        }
        24 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082f5f4); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082f5f8); };
            c.w64(2, t31);
            blk = 25; continue;
        }
        25 => {
            c.w32(1, 0x1a0u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082f601); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082f602); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f603); };
            c.w64(3, t32);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700082f614, 0x700082f60f);
        }
        26 => {
            return RegfileExit::fault(0x700082f614);
        }
        27 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082f61a); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700082f61b); };
            c.w64(1, t33);
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082f622); };
            c.w32(1, t34);
            let r35 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r35);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082f628); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082f629); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700082f62a); };
            c.w64(3, t36);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700082f63b, 0x700082f636);
        }
        28 => {
            return RegfileExit::fault(0x700082f63b);
        }
        29 => {
            return RegfileExit::jump(0x700082f640);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082f654(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18b8)) else { return RegfileExit::fault(0x700082f654); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f674(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18c0)) else { return RegfileExit::fault(0x700082f674); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f694(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18c8)) else { return RegfileExit::fault(0x700082f694); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f6b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18d0)) else { return RegfileExit::fault(0x700082f6b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f6d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18d8)) else { return RegfileExit::fault(0x700082f6d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f6f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18e0)) else { return RegfileExit::fault(0x700082f6f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f714(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18e8)) else { return RegfileExit::fault(0x700082f714); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f734(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18f0)) else { return RegfileExit::fault(0x700082f734); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f754(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x18f8)) else { return RegfileExit::fault(0x700082f754); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f774(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1900)) else { return RegfileExit::fault(0x700082f774); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f794(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1908)) else { return RegfileExit::fault(0x700082f794); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f7b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1910)) else { return RegfileExit::fault(0x700082f7b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f7d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1918)) else { return RegfileExit::fault(0x700082f7d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f7f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1920)) else { return RegfileExit::fault(0x700082f7f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f814(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1928)) else { return RegfileExit::fault(0x700082f814); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f834(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1930)) else { return RegfileExit::fault(0x700082f834); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f854(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1938)) else { return RegfileExit::fault(0x700082f854); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f874(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1940)) else { return RegfileExit::fault(0x700082f874); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f894(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1948)) else { return RegfileExit::fault(0x700082f894); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f8b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1950)) else { return RegfileExit::fault(0x700082f8b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f8d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1958)) else { return RegfileExit::fault(0x700082f8d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f8f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1960)) else { return RegfileExit::fault(0x700082f8f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f914(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1968)) else { return RegfileExit::fault(0x700082f914); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f934(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1970)) else { return RegfileExit::fault(0x700082f934); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f954(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1978)) else { return RegfileExit::fault(0x700082f954); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f974(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1980)) else { return RegfileExit::fault(0x700082f974); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f994(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1988)) else { return RegfileExit::fault(0x700082f994); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f9b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1990)) else { return RegfileExit::fault(0x700082f9b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f9d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1998)) else { return RegfileExit::fault(0x700082f9d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082f9f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19a0)) else { return RegfileExit::fault(0x700082f9f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082fa14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082fa14); };
            c.w64(2, t0);
            c.w64(3, c.r64(2));
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082fa1e); };
            c.w64(7, t1);
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082fa28); };
            c.w64(2, t2);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082fa33); };
            c.w64(3, t3);
            blk = 3; continue;
        }
        3 => {
            if !c.pop64_reg(1) { return RegfileExit::fault(0x700082fa38); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(0).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700082fa3e); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082fa3f); }
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700082fa40); };
            c.cmp(c.r32(3), t4, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082fa49); };
            c.cmp(c.r32(3), t5, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::jump(0x70007f6194);
        }
        6 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return RegfileExit::jump(0x70007f2274);
        }
        7 => {
            return RegfileExit::fault(0x700082fa5e);
        }
        8 => {
            return RegfileExit::fault(0x700082fa5f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082fa74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19a8)) else { return RegfileExit::fault(0x700082fa74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082fa94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082fa94); };
            c.w64(7, t0);
            if !c.pop64_reg(3) { return RegfileExit::fault(0x700082fa99); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082fa9a); }
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082faa1); };
            if !c.push64(t1) { return RegfileExit::fault(0x700082faa1); }
            let r2 = c.inc(c.r64(0), 64);
            c.w64(0, r2);
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082faab); }
            let r3 = c.dec(c.r64(0), 64);
            c.w64(0, r3);
            return RegfileExit::jump(0x70007f2274);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082fad4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19b0)) else { return RegfileExit::fault(0x700082fad4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082faf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082faf4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082fafa); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082fb00); };
            c.w32(6, t4);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x68u64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700082fb0f);
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x700082fb10); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082fb19); };
            c.w32(6, t7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082fb1d); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700082fb24); };
            c.w64(8, t9);
            c.w64(2, c.r64(6).wrapping_mul(4));
            c.w64(6, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082fb33); };
            c.w64(9, t10);
            let Some(t11) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082fb3a); };
            c.cmp(t11, c.r64(6), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x700082fb3f); }
            c.w64(6, c.r64(8).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082fb46); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082fb49); };
            c.w64(1, t12);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700082fb50); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700082fb53); }
            c.w64(1, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082fb60); };
            c.w64(7, t13);
            blk = 6; continue;
        }
        6 => {
            let r14 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r14);
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(7)) { return RegfileExit::fault(0x700082fb6b); }
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700082fb74); }
            if !c.st32(c.r64(6).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x700082fb77); }
            c.w64(0, c.r64(6));
            return c.ret(0x0, 0x700082fb7d);
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fb7e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fb82); }
            let r15 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700082fb88); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700082fb8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700082fb90); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700082fb94); }
            return c.call(0x70007febf4, 0x700082fb9d, 0x700082fb98);
        }
        9 => {
            c.w64(6, c.r64(0));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082fba0); };
            c.w64(3, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082fba4); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082fba8); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082fbac); };
            c.w64(1, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082fbb3); }
            blk = 4; continue;
        }
        10 => {
            return RegfileExit::jump(0x700082fbb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082fbd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082fbd4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082fbda); };
            c.logic(t2 & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fbe0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fbe4); }
            let r3 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r3);
            blk = 32; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082fbef); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082fbf5); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082fbfb); };
            c.cmp(t8, 0xacu64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082fc07); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082fc0d); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082fc14); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082fc1b); };
            c.cmp(t15, 0x5eu64, 16);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082fc23); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            blk = 5; continue;
        }
        5 => {
            let Some(t18) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082fc2a); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082fc31); };
            c.cmp(t20, 0x69u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082fc39); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            blk = 7; continue;
        }
        7 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082fc40); };
            c.w64(8, (t23 as u32 as i32 as i64 as u64));
            let Some(t24) = c.ld16(c.r64(7).wrapping_add(0x15)) else { return RegfileExit::fault(0x700082fc44); };
            c.w32(7, t24);
            if let Some(sr) = c.sar(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let r25 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r25);
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 42; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082fc5c); };
            c.w64(8, t26);
            let Some(t27) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082fc63); };
            c.w64(9, t27);
            c.w64(2, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082fc72); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082fc79); };
            c.cmp(t29, c.r64(11), 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082fc90); }
            let r30 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r30);
            blk = 11; continue;
        }
        11 => {
            c.w32(11, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700082fc9b); };
            c.w64(15, t31);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700082fca2); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700082fca6); }
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082fcb8); };
            c.w64(11, t32);
            blk = 13; continue;
        }
        13 => {
            let r33 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r33);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x700082fcc3); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082fccd); };
            c.w32(7, t34);
            c.w64(2, c.r64(7).wrapping_mul(4));
            let Some(t35) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082fcd9); };
            c.w64(7, t35);
            c.w64(11, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t36) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082fce0); };
            c.cmp(t36, c.r64(11), 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082fcea); }
            let r37 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r37);
            blk = 16; continue;
        }
        16 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082fcf5); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700082fcf8); };
            c.w64(11, t38);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700082fcff); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700082fd03); }
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082fd07); };
            c.w32(11, t39);
            c.w64(12, c.r64(11));
            let r40 = c.r32(12) & 0xe0000000u64;
            c.w32(12, r40);
            c.logic(r40, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 28; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082fd1d); };
            c.w64(11, t41);
            blk = 19; continue;
        }
        19 => {
            let r42 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r42);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(11)) { return RegfileExit::fault(0x700082fd28); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 19; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 28; continue;
        }
        21 => {
            let r43 = c.sub(c.r32(11), 0x20000000u64, 32);
            c.w32(11, r43);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700082fd3b); }
            let Some(t44) = c.ld8(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x700082fd3f); };
            c.w32(11, t44);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x700082fd51); };
            c.w64(15, t45);
            blk = 23; continue;
        }
        23 => {
            let r46 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r46);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(15)) { return RegfileExit::fault(0x700082fd5c); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 23; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082fd6b); };
            c.w64(8, t47);
            blk = 26; continue;
        }
        26 => {
            let r48 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r48);
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(8)) { return RegfileExit::fault(0x700082fd76); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            let r49 = c.add(c.r32(12), 0xe0000000u64, 32);
            c.w32(12, r49);
            if c.cc_e() { blk = 40; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700082fd8d); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x700082fd90); }
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x700082fd93); }
            if !c.st32(c.r64(7).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x700082fd96); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x1b), c.r32(6)) { return RegfileExit::fault(0x700082fd9c); }
            c.w64(8, 0xfffffffffffffffcu64);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700082fda6); }
            let Some(t50) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082fdaa); };
            c.cmp(t50, 0x82cu64, 16);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st32(c.r64(7).wrapping_add(0x2b), c.r32(6)) { return RegfileExit::fault(0x700082fdb2); }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700082fdb8);
        }
        31 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fdb9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fdbd); }
            let r51 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r51);
            blk = 32; continue;
        }
        32 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082fdc3); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700082fdc4); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1ff0)) else { return RegfileExit::fault(0x700082fdc5); };
            c.w64(3, t52);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700082fdd6, 0x700082fdd1);
        }
        33 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082fdd9); }
            return c.ret(0x0, 0x700082fdda);
        }
        34 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fddb); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fddf); }
            let r53 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r53);
            blk = 36; continue;
        }
        35 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fde7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fdeb); }
            let r54 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r54);
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700082fdf1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700082fdf5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082fdf9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700082fdfd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700082fe01); }
            return c.call(0x70007febb4, 0x700082fe0a, 0x700082fe05);
        }
        37 => {
            c.w64(9, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082fe0d); };
            c.w64(0, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082fe11); };
            c.w64(3, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082fe15); };
            c.w64(6, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082fe19); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082fe1d); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082fe24); };
            c.w64(12, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082fe2b); };
            c.w64(7, t61);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082fe32); }
            blk = 11; continue;
        }
        38 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fe38); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fe3c); }
            let r62 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r62);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700082fe42); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700082fe46); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082fe4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700082fe4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700082fe52); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700082fe56); }
            return c.call(0x70007febf4, 0x700082fe5f, 0x700082fe5a);
        }
        39 => {
            c.w64(7, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082fe62); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082fe66); };
            c.w64(3, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082fe6a); };
            c.w64(6, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082fe6e); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082fe72); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082fe76); };
            c.w64(2, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082fe7d); }
            blk = 16; continue;
        }
        40 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fe83); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fe87); }
            let r69 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r69);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700082fe8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700082fe8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700082fe92); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700082fe96); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700082fe9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700082fe9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700082fea2); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x700082fead); };
            c.w64(3, t70);
            c.w64(8, c.r64(6));
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700082fec2, 0x700082febd);
        }
        41 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082fec2); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082fec6); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082feca); };
            c.w64(6, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082fece); };
            c.w64(7, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082fed2); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082fed6); };
            c.w64(9, t76);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082fedd); }
            blk = 28; continue;
        }
        42 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082fee3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082fee7); }
            let r77 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r77);
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700082feed); };
            c.w64(3, t78);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082fefd, 0x700082fef8);
        }
        43 => {
            return RegfileExit::fault(0x700082fefd);
        }
        44 => {
            return RegfileExit::jump(0x700082ff00);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700082ff14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19b8)) else { return RegfileExit::fault(0x700082ff14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700082ff34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082ff34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700082ff38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082ff39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082ff3a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700082ff3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700082ff43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082ff57); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700082ff5f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700082ff63); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082ff6d); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082ff73); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082ff84); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082ff8b); };
            c.cmp(t7, 0x82au64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082ff98); };
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
            if !c.st32(c.r64(7).wrapping_add(0x1b), c.r32(9)) { return RegfileExit::fault(0x700082ffb3); }
            c.w64(0, c.r64(1));
            c.w64(2, c.r64(7));
            return c.call(0x70007fa8f4, 0x700082ffc2, 0x700082ffbd);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082ffc2); };
            c.w64(1, t9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700082ffc6); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082ffc9); };
            c.w64(8, t11);
            let r12 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700082ffd1); }
            c.cmp(c.r32(7), 0xfffffffffffffffcu64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082ffdd); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700082ffde); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700082ffe4); }
            return c.ret(0x0, 0x700082ffe6);
        }
        11 => {
            c.w64(7, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(1).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700082ffee); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082fff1); };
            c.w64(3, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082fff8); };
            c.w64(6, t14);
            return c.call(0x700082faf4, 0x7000830001, 0x700082fffc);
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000830001); };
            c.w64(1, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830008); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000830009); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700083000f); }
            return c.ret(0x0, 0x7000830011);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000830012); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000830016); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000830027, 0x7000830022);
        }
        14 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000830027); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083002b); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        15 => {
            c.w64(1, 0xfffffffffffffffeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083003b); };
            c.w64(7, t19);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700083003f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000830042); }
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000830048); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083004f); };
            c.w64(6, t21);
            return c.call(0x700086ed74, 0x7000830058, 0x7000830053);
        }
        16 => {
            return RegfileExit::fault(0x7000830058);
        }
        17 => {
            c.w32(1, 0x70u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083005e); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700083005f); };
            c.w64(3, t22);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000830070, 0x700083006b);
        }
        18 => {
            return RegfileExit::fault(0x7000830070);
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000830071); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000830078); };
            c.w64(3, t24);
            return c.call(0x700082faf4, 0x7000830084, 0x700083007f);
        }
        20 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000830084); };
            c.w64(8, t25);
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083008f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000830090); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000830096); }
            return c.ret(0x0, 0x7000830098);
        }
        21 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083009e); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700083009f); };
            c.w64(1, t26);
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008300a6); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008300ac); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008300ad); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008300ae); };
            c.w64(3, t29);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008300bf, 0x70008300ba);
        }
        22 => {
            return RegfileExit::fault(0x70008300bf);
        }
        23 => {
            return RegfileExit::jump(0x70008300c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008300d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008300d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008300d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008300d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008300da); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008300df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008300e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008300f7); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008300ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000830103); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083010d); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000830113); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830124); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083012b); };
            c.cmp(t7, 0x82au64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000830138); };
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
            c.w32(9, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x1b), c.r32(9)) { return RegfileExit::fault(0x7000830156); }
            c.w64(0, c.r64(1));
            c.w64(2, c.r64(7));
            return c.call(0x70007fa8f4, 0x7000830165, 0x7000830160);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000830165); };
            c.w64(1, t9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000830169); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083016c); };
            c.w64(8, t11);
            let r12 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000830174); }
            c.cmp(c.r32(7), 0xfffffffffffffffcu64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830180); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000830181); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000830187); }
            return c.ret(0x0, 0x7000830189);
        }
        11 => {
            c.w64(7, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(1).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000830191); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000830194); };
            c.w64(3, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083019b); };
            c.w64(6, t14);
            return c.call(0x700082faf4, 0x70008301a4, 0x700083019f);
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008301a4); };
            c.w64(1, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008301ab); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008301ac); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008301b2); }
            return c.ret(0x0, 0x70008301b4);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008301b5); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008301b9); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008301ca, 0x70008301c5);
        }
        14 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008301ca); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008301ce); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        15 => {
            c.w64(1, 0xfffffffffffffffeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008301de); };
            c.w64(7, t19);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70008301e2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008301e5); }
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x70008301eb); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008301f2); };
            c.w64(6, t21);
            return c.call(0x700086ed74, 0x70008301fb, 0x70008301f6);
        }
        16 => {
            return RegfileExit::fault(0x70008301fb);
        }
        17 => {
            c.w32(1, 0x70u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000830201); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000830202); };
            c.w64(3, t22);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000830213, 0x700083020e);
        }
        18 => {
            return RegfileExit::fault(0x7000830213);
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000830214); };
            c.w64(3, t23);
            c.w64(0, c.r64(1));
            return c.call(0x700082faf4, 0x7000830223, 0x700083021e);
        }
        20 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000830223); };
            c.w64(8, t24);
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083022e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700083022f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000830235); }
            return c.ret(0x0, 0x7000830237);
        }
        21 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083023d); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700083023e); };
            c.w64(1, t25);
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000830245); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083024b); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083024c); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700083024d); };
            c.w64(3, t28);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700083025e, 0x7000830259);
        }
        22 => {
            return RegfileExit::fault(0x700083025e);
        }
        23 => {
            return RegfileExit::jump(0x7000830260);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000830274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000830274); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000830278); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000830279); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083027a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700083027f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000830283); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000830297); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700083029f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008302a3); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008302ad); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008302b3); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008302c4); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008302cb); };
            c.cmp(t7, 0x82au64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008302d8); };
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
            c.w32(9, 0x4u64);
            if !c.st32(c.r64(7).wrapping_add(0x1b), c.r32(9)) { return RegfileExit::fault(0x70008302f6); }
            c.w64(0, c.r64(1));
            c.w64(2, c.r64(7));
            return c.call(0x70007fa8f4, 0x7000830305, 0x7000830300);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000830305); };
            c.w64(1, t9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000830309); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083030c); };
            c.w64(8, t11);
            let r12 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000830314); }
            c.cmp(c.r32(7), 0xfffffffffffffffcu64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830320); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000830321); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000830327); }
            return c.ret(0x0, 0x7000830329);
        }
        11 => {
            c.w64(7, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(1).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000830331); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000830334); };
            c.w64(3, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083033b); };
            c.w64(6, t14);
            return c.call(0x700082faf4, 0x7000830344, 0x700083033f);
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000830344); };
            c.w64(1, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083034b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700083034c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000830352); }
            return c.ret(0x0, 0x7000830354);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000830355); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000830359); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700083036a, 0x7000830365);
        }
        14 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083036a); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083036e); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        15 => {
            c.w64(1, 0xfffffffffffffffeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083037e); };
            c.w64(7, t19);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x7000830382); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000830385); }
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x700083038b); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000830392); };
            c.w64(6, t21);
            return c.call(0x700086ed74, 0x700083039b, 0x7000830396);
        }
        16 => {
            return RegfileExit::fault(0x700083039b);
        }
        17 => {
            c.w32(1, 0x70u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008303a1); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008303a2); };
            c.w64(3, t22);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008303b3, 0x70008303ae);
        }
        18 => {
            return RegfileExit::fault(0x70008303b3);
        }
        19 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008303b4); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x21b8)) else { return RegfileExit::fault(0x70008303b5); };
            c.w64(3, t23);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008303c6, 0x70008303c1);
        }
        20 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008303c6); };
            c.w64(8, t24);
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008303d1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008303d2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008303d8); }
            return c.ret(0x0, 0x70008303da);
        }
        21 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008303e0); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008303e1); };
            c.w64(1, t25);
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008303e8); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008303ee); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008303ef); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008303f0); };
            c.w64(3, t28);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000830401, 0x70008303fc);
        }
        22 => {
            return RegfileExit::fault(0x7000830401);
        }
        23 => {
            return RegfileExit::jump(0x7000830404);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000830414(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19c0)) else { return RegfileExit::fault(0x7000830414); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000830434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(6, c.r64(5));
            let Some(t0) = c.ld64(c.r64(6).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000830437); };
            c.w64(7, t0);
            let r1 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r1);
            let r2 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r2);
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x700083043f); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r3 = c.r64(8) & c.r64(0);
            c.w64(8, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000830452); };
            c.logic(t4 & 0x4u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st32(c.r64(0).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x700083045d); }
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x7000830460); }
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000830463); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000830469); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083046f); };
            c.w32(7, t9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000830472); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x15)) else { return RegfileExit::fault(0x700083047b); };
            c.w32(1, t12);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ae() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t13) = c.ld64(c.r64(6).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x7000830498); };
            c.w64(9, t13);
            c.w64(11, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(9)) { return RegfileExit::fault(0x70008304a4); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r14 = c.r64(12) & c.r64(3);
            c.w64(12, r14);
            c.logic(r14, 64);
            let Some(t15) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008304b8); };
            c.logic(t15 & 0x4u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let r16 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r16);
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ne() { blk = 5; continue; }
            blk = 8; continue;
        }
        8 => {
            let r17 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r17);
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ae() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(1).wrapping_sub(0x6));
            blk = 12; continue;
        }
        11 => {
            c.w64(1, c.r64(8));
            blk = 12; continue;
        }
        12 => {
            c.w64(8, c.r64(1).wrapping_add(0x1));
            c.w64(9, c.r64(7));
            let r18 = c.sub(c.r64(9), c.r64(1), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld64(c.r64(6).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70008304ed); };
            c.w64(9, t19);
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x70008304f9); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r20 = c.r64(11) & c.r64(3);
            c.w64(11, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083050d); };
            c.logic(t21 & 0x4u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ne() { blk = 11; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000830519); };
            c.w64(0, t22);
            return c.ret(0x0, 0x7000830520);
        }
        16 => {
            return RegfileExit::fault(0x7000830521);
        }
        17 => {
            return RegfileExit::fault(0x7000830522);
        }
        18 => {
            c.w32(9, c.r32(9));
            let r23 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r23);
            c.w64(11, 0xfffffffffffc0000u64);
            let r24 = c.r64(11) & c.r64(9);
            c.w64(11, r24);
            c.logic(r24, 64);
            let Some(t25) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000830533); };
            c.logic(t25 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083053e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083053f); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700083054c, 0x7000830547);
        }
        20 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083054c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083054e); }
            blk = 14; continue;
        }
        21 => {
            c.w32(9, c.r32(9));
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            c.w64(12, 0xfffffffffffc0000u64);
            let r27 = c.r64(12) & c.r64(9);
            c.w64(12, r27);
            c.logic(r27, 64);
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000830561); };
            c.logic(t28 & 0x2u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000830571); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000830572); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x700083057f, 0x700083057a);
        }
        23 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083057f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000830581); }
            blk = 7; continue;
        }
        24 => {
            c.w32(7, c.r32(7));
            let r29 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r29);
            c.w64(8, 0xfffffffffffc0000u64);
            let r30 = c.r64(8) & c.r64(7);
            c.w64(8, r30);
            c.logic(r30, 64);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000830596); };
            c.logic(t31 & 0x2u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, c.r64(0).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008305a5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008305a6); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70008305b3, 0x70008305ae);
        }
        26 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008305b3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008305b5); }
            blk = 2; continue;
        }
        27 => {
            return RegfileExit::jump(0x70008305bc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008305d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008305d4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008305da); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008305e0); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008305e6); };
            c.w32(6, t6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x15)) else { return RegfileExit::fault(0x70008305e9); };
            c.w32(2, t7);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r8 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r8);
            c.cmp(c.r64(3), c.r64(6), 64);
            if c.cc_ae() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(2).wrapping_sub(0x6));
            c.w64(7, c.r64(5));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x198)) else { return RegfileExit::fault(0x7000830607); };
            c.w64(8, t9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083060e); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            c.w64(11, c.r64(6));
            let r12 = c.sub(c.r64(11), c.r64(2), 64);
            c.w64(11, r12);
            if !c.st64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)), c.r64(9)) { return RegfileExit::fault(0x700083061c); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000830620); }
            let r13 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r13);
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ne() { blk = 3; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083062e); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            return c.ret(0x0, 0x7000830634);
        }
        5 => {
            return RegfileExit::fault(0x7000830635);
        }
        6 => {
            return RegfileExit::jump(0x7000830638);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000830654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000830654); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000830658); }
            let Some(t0) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700083065a); };
            c.w64(0, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083065e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000830662); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000830666); };
            c.w64(2, t3);
            return c.call(0x70008a8b74, 0x700083066f, 0x700083066a);
        }
        1 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830672); }
            return c.ret(0x20, 0x7000830673);
        }
        2 => {
            return RegfileExit::jump(0x7000830678);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000830694(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19c8)) else { return RegfileExit::fault(0x7000830694); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008306b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19d0)) else { return RegfileExit::fault(0x70008306b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008306d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19d8)) else { return RegfileExit::fault(0x70008306d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008306f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19e0)) else { return RegfileExit::fault(0x70008306f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000830714(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19e8)) else { return RegfileExit::fault(0x7000830714); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000830734(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19f0)) else { return RegfileExit::fault(0x7000830734); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000830754(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x19f8)) else { return RegfileExit::fault(0x7000830754); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000830774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000830774); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000830778); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000830779); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083077a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700083077f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000830783); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000830789); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x7000830794, 0x700083078f);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000830794); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083079c); };
            c.w32(2, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083079f); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008307a8); }
            c.mov128_rr(1, 0);
            c.arith_sd(1, 1, c.xmm_u64(0));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008307bc); };
            c.w64(0, t6);
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008307c3); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008307ca); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x10, 0x70008307d1);
        }
        11 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008307d4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008307db); }
            return c.ret(0x0, 0x70008307dd);
        }
        12 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008307de); };
            c.w64(0, t8);
            blk = 9; continue;
        }
        13 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008307e7); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008307f5, 0x70008307f0);
        }
        14 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008307f5); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008307fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000830814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000830814); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000830818); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000830819); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083081a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700083081f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000830823); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000830829); };
            c.w64(0, t2);
            blk = 4; continue;
        }
        2 => {
            return c.call(0x70008a1014, 0x7000830834, 0x700083082f);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000830834); };
            c.w64(6, t3);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083083c); };
            c.w32(2, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083083f); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000830848); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000830855); };
            c.w64(0, t6);
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083085c); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830863); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x10, 0x700083086a);
        }
        11 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700083086d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000830874); }
            return c.ret(0x0, 0x7000830876);
        }
        12 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000830877); };
            c.w64(0, t8);
            blk = 9; continue;
        }
        13 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000830880); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700083088e, 0x7000830889);
        }
        14 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083088e); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        15 => {
            return RegfileExit::jump(0x7000830898);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008308b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a00)) else { return RegfileExit::fault(0x70008308b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008308d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a08)) else { return RegfileExit::fault(0x70008308d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008308f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830905); };
            c.w32(9, t0);
            let r1 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1);
            blk = 3; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083090e); };
            c.w64(9, t2);
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000830915); };
            c.logic(t3 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000830923); };
            c.cmp(t4, c.r32(3), 32);
            if c.cc_e() { blk = 347; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(12, c.r32(9));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000830937); };
            c.w32(15, t5);
            let r6 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r6);
            let r7 = c.r32(12) | 0x2u64;
            c.w32(12, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(12, c.r32(15));
            let r8 = c.r32(12) & 0x3u64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830959); };
            c.w32(12, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x700083095d); };
            c.cmp(t10, c.r32(12), 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083096a); };
            c.w32(8, t11);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r12 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r12);
            blk = 9; continue;
        }
        9 => {
            c.w64(11, c.r64(8).wrapping_mul(4));
            c.w32(12, c.r32(9));
            let Some(t13) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000830983); };
            c.w32(7, t13);
            let r14 = c.r32(12) | 0x2u64;
            c.w32(12, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let r15 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r15);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ge() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 73; continue;
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008309a0); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            blk = 14; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008309aa); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(9, c.r64(2));
            c.w64(11, c.r64(8));
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11));
            let r20 = c.r64(12) & 0xfu64;
            c.w64(12, r20);
            c.logic(r20, 64);
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(12), 0x5u64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(12), 0xdu64, 64);
            if c.cc_ne() { blk = 181; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008309e5); };
            c.w64(0, t21);
            return c.ret(0x0, 0x70008309ec);
        }
        20 => {
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x70008309f0);
        }
        21 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let r22 = c.r64(8) & 0x7ffu64;
            c.w64(8, r22);
            c.logic(r22, 64);
            c.logic(c.r8(11) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 30; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830a1d); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000830a2a); };
            c.w64(9, t25);
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830a31); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x7000830a42);
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830a4d); };
            c.w32(9, t28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000830a51); };
            c.cmp(t29, c.r32(9), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000830a5e); }
            blk = 69; continue;
        }
        30 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830a6f); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            return c.ret(0x0, 0x7000830a77);
        }
        32 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830a78); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830a8a); };
            c.w32(9, t34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000830a8e); };
            c.cmp(t35, c.r32(9), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000830a9b); }
            blk = 69; continue;
        }
        35 => {
            c.w64(8, c.r64(11));
            let r36 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r36);
            let r37 = c.r32(8) & 0xfu64;
            c.w32(8, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 44; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 345; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 328; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 328; continue;
        }
        44 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 328; continue;
        }
        49 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 328; continue;
        }
        52 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r38 = c.r64(11) & 0xfffffu64;
            c.w64(11, r38);
            c.logic(r38, 64);
            let Some(t39) = c.ld8(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830b36); };
            c.w32(8, (t39 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x7000830b40);
        }
        53 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r40 = c.r64(11) & 0xfffffu64;
            c.w64(11, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld16(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830b4c); };
            c.w32(8, (t41 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x7000830b56);
        }
        54 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r42 = c.r64(11) & 0xfffffu64;
            c.w64(11, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830b62); };
            c.w32(8, t43);
            c.w64(9, c.r64(8));
            let r44 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r44);
            if c.cc_o() { blk = 329; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x7000830b76);
        }
        56 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r45 = c.r64(11) & 0xfffffu64;
            c.w64(11, r45);
            c.logic(r45, 64);
            let Some(t46) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830b82); };
            c.w32(8, t46);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 334; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x7000830b98);
        }
        58 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r47 = c.r64(11) & 0xfffffu64;
            c.w64(11, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld64(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830ba4); };
            c.w64(8, t48);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 64; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000830baf); };
            c.w64(9, t49);
            let Some(t50) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000830bb6); };
            c.w64(11, t50);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000830bbd); };
            c.w64(15, t51);
            let Some(t52) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000830bc4); };
            c.cmp(t52, c.r64(12), 64);
            if c.cc_be() { blk = 339; continue; }
            blk = 60; continue;
        }
        60 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000830bcd); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 61; continue;
        }
        61 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000830bd4); };
            c.w64(11, t53);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000830bdb); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000830be5); }
            let r54 = c.neg(c.r64(8), 64);
            c.w64(8, r54);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000830bf0); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x7000830bf7);
        }
        63 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000830bf8); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000830c00); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x7000830c07);
        }
        64 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000830c08); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000830c0f); };
            c.w64(9, t56);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000830c16); };
            c.w64(11, t57);
            let Some(t58) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000830c1d); };
            c.cmp(t58, c.r64(12), 64);
            if c.cc_be() { blk = 341; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000830c26); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 66; continue;
        }
        66 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000830c2d); };
            c.w64(9, t59);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000830c34); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x7000830c38); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x7000830c43);
        }
        67 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r60 = c.r64(11) & 0xfffffu64;
            c.w64(11, r60);
            c.logic(r60, 64);
            if !c.movss_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000830c4f); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 69; continue;
        }
        68 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r61 = c.r64(11) & 0xfffffu64;
            c.w64(11, r61);
            c.logic(r61, 64);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000830c67); }
            blk = 69; continue;
        }
        69 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000830c6e); };
            c.w64(8, t62);
            let Some(t63) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000830c75); };
            c.w64(9, t63);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000830c7c); };
            c.w64(12, t64);
            let Some(t65) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000830c83); };
            c.cmp(t65, c.r64(11), 64);
            if c.cc_be() { blk = 343; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000830c8d); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 71; continue;
        }
        71 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000830c94); };
            c.w64(9, t66);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000830c9b); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000830c9f); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x7000830ca8);
        }
        72 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r67 = c.r64(11) & 0xfffffu64;
            c.w64(11, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830cb4); };
            c.w32(0, t68);
            let r69 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r69);
            return c.ret(0x0, 0x7000830cbc);
        }
        73 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000830cbd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000830cbe); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000830cbf); }
            let Some(t70) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000830cc0); };
            if !c.push64(t70) { return RegfileExit::fault(0x7000830cc0); }
            c.w32(0, 0x4u64);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2050)) else { return RegfileExit::fault(0x7000830cc9); };
            c.w64(3, t71);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x7000830cd0); }
            return RegfileExit::jump(0x700086ed74);
        }
        74 => {
            blk = 73; continue;
        }
        75 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000830cdc); };
            c.cmp(t72, c.r32(15), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xc00)) else { return RegfileExit::fault(0x7000830ce5); };
            c.cmp(t73, c.r32(15), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t74) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000830cee); };
            c.cmp(t74, 0x422u64, 16);
            if c.cc_ne() { blk = 73; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000830cf7); };
            c.logic(t75 & 0x20u64, 8);
            if c.cc_ne() { blk = 73; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t76) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000830cfe); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0xfb0)) else { return RegfileExit::fault(0x7000830d06); };
            c.w64(9, t78);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000830d10); };
            c.cmp(t79, c.r32(11), 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830d16); };
            c.w32(9, t80);
            let r81 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r81);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 81; continue;
        }
        81 => {
            let r82 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000830d27); };
            c.w32(8, t83);
            let r84 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r84);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 82; continue;
        }
        82 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000830d34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000830d38); }
            let r85 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r85);
            let r86 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r86);
            c.logic(r86, 64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000830d42); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            c.w64(6, c.r64(8));
            return c.call(0x70007f6e94, 0x7000830d50, 0x7000830d4b);
        }
        83 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830d53); }
            return c.ret(0x0, 0x7000830d54);
        }
        84 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t87) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830d5f); };
            c.w32(8, t87);
            let r88 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r88);
            c.w64(9, c.r64(8));
            if let Some(sr) = c.shr(c.r64(9), 0xdu64, 64) { c.w64(9, sr); }
            let Some(t89) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830d6d); };
            c.w32(11, t89);
            let r90 = c.r64(9) ^ c.r64(8);
            c.w64(9, r90);
            c.logic(r90, 64);
            let r91 = c.add(c.r32(9), c.r32(11), 32);
            c.w32(9, r91);
            let r92 = c.r32(9) & 0x1ffcu64;
            c.w32(9, r92);
            c.logic(r92, 32);
            let r93 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(11, r93);
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2f18)) else { return RegfileExit::fault(0x7000830d82); };
            c.w64(12, t94);
            let Some(t95) = c.ld32(c.r64(12).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000830d89); };
            c.cmp(t95, c.r32(1), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000830d8f); };
            c.cmp(t96, c.r32(8), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000830d96); };
            c.w32(8, t97);
            let r98 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r98);
            blk = 14; continue;
        }
        88 => {
            c.w32(11, c.r32(1));
            c.w32(9, c.r32(9));
            let r99 = c.sub(c.r32(9), c.r32(11), 32);
            c.w32(9, r99);
            let r100 = c.sub(c.r32(9), 0x4e93591bu64, 32);
            c.w32(9, r100);
            let r101 = c.r32(9) & 0x7fcu64;
            c.w32(9, r101);
            c.logic(r101, 32);
            let r102 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r102);
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2f30)) else { return RegfileExit::fault(0x7000830dbe); };
            c.w64(11, t103);
            let Some(t104) = c.ld32(c.r64(11).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000830dc5); };
            c.cmp(t104, c.r32(1), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t105) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000830dcf); };
            c.cmp(t105, c.r32(8), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t106) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000830dda); };
            c.w32(8, t106);
            let r107 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r107);
            blk = 14; continue;
        }
        91 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000830de7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000830deb); }
            let r108 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r108);
            let Some(t109) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830df1); };
            c.w32(9, t109);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000830df5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000830df9); }
            let Some(t110) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000830dfd); };
            c.cmp(t110, c.r32(9), 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000830e0a); };
            c.w32(9, t111);
            let r112 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r112);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t113) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830e19); };
            c.cmp(t113, c.r32(11), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830e22); }
            blk = 73; continue;
        }
        95 => {
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830e28); };
            c.w32(9, t114);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000830e32); }
            c.logic(c.r8(12) & 0x30u64, 8);
            if c.cc_e() { blk = 169; continue; }
            blk = 96; continue;
        }
        96 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 158; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t115) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830e4a); };
            c.w32(12, t115);
            let r116 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r116);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000830e57); };
            c.w64(12, t117);
            blk = 99; continue;
        }
        99 => {
            let Some(t118) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000830e5e); };
            c.w32(15, t118);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r119 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r119);
            let Some(t120) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830e6d); };
            c.w32(7, t120);
            let r121 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r121);
            let r122 = c.r64(7) & c.r64(15);
            c.w64(7, r122);
            c.logic(r122, 64);
            blk = 100; continue;
        }
        100 => {
            c.w64(0, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(0).wrapping_mul(4));
            let Some(t123) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000830e83); };
            c.w32(9, t123);
            let Some(t124) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000830e88); };
            c.cmp(t124, c.r32(9), 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let r125 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r125);
            c.w64(9, c.r64(11).wrapping_add(c.r64(7)));
            c.w64(0, c.r64(15));
            let r126 = c.r64(0) & c.r64(9);
            c.w64(0, r126);
            c.logic(r126, 64);
            c.w64(7, c.r64(0));
            blk = 100; continue;
        }
        103 => {
            let Some(t127) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830ead); };
            c.w32(8, t127);
            let Some(t128) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000830eb2); };
            c.w32(9, t128);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            return RegfileExit::fault(0x7000830ec6);
        }
        105 => {
            let Some(t129) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000830ec7); };
            c.w32(0, t129);
            let r130 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r130);
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 150; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t131) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830ed9); };
            c.w32(8, t131);
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000830edd); };
            c.cmp(t132, c.r32(8), 32);
            if c.cc_ne() { blk = 131; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t133) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000830eea); };
            c.w32(8, t133);
            let r134 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r134);
            let Some(t135) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830ef1); };
            c.w32(9, t135);
            let r136 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r136);
            let Some(t137) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000830ef8); };
            c.logic(t137 & 0x2u64, 8);
            if c.cc_ne() { blk = 129; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000830f03); };
            c.cmp(t138, c.r32(9), 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000830f0c); };
            c.w64(0, t139);
            blk = 83; continue;
        }
        110 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000830f18); };
            c.w64(9, t140);
            let Some(t141) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x7000830f1f); };
            c.cmp(t141, 0x20u64, 8);
            if c.cc_e() { blk = 127; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000830f29); };
            c.w32(9, t142);
            let Some(t143) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000830f2d); };
            c.cmp(t143, c.r32(9), 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000830f36); };
            c.w64(0, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000830f3a); };
            c.w64(3, t145);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830f41); }
            blk = 73; continue;
        }
        113 => {
            let Some(t146) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830f47); };
            c.w32(9, t146);
            let r147 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r147);
            let Some(t148) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830f4e); };
            c.w32(11, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000830f52); };
            c.cmp(t149, c.r32(11), 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t150) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000830f5b); };
            c.w32(9, t150);
            let r151 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r151);
            let Some(t152) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830f62); };
            c.w32(11, t152);
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000830f66); };
            c.cmp(t153, c.r32(11), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 115; continue;
        }
        115 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 126; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t154) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830f79); };
            c.w32(11, t154);
            let r155 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r155);
            let Some(t156) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000830f80); };
            c.w32(12, t156);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 122; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t157) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000830fa0); };
            c.cmp(t157, c.r32(11), 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w64(9, c.r64(2));
            blk = 122; continue;
        }
        120 => {
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000830fae); };
            c.w64(0, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000830fb2); };
            c.w64(3, t159);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000830fb9); }
            blk = 73; continue;
        }
        121 => {
            let Some(t160) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000830fbf); };
            c.w32(9, t160);
            let r161 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r161);
            blk = 122; continue;
        }
        122 => {
            let Some(t162) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000830fc6); };
            c.w32(9, t162);
            let r163 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r163);
            let Some(t164) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000830fcd); };
            c.w32(9, t164);
            let r165 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r165);
            let Some(t166) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000830fd4); };
            c.w32(11, t166);
            let r167 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r167);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t168) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000830fe6); };
            c.w32(6, t168);
            let r169 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r169);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000830fed); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007f6fb4, 0x7000830ff8, 0x7000830ff3);
        }
        124 => {
            blk = 83; continue;
        }
        125 => {
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000830ffd); };
            c.w64(0, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831001); };
            c.w64(3, t171);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831008); }
            blk = 73; continue;
        }
        126 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083100e); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831012); };
            c.w64(3, t173);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831019); }
            blk = 73; continue;
        }
        127 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083101f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000831020); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000831021); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000831022); };
            c.w64(3, t174);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000831033, 0x700083102e);
        }
        128 => {
            blk = 83; continue;
        }
        129 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000831038); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(8));
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x7000831046, 0x7000831041);
        }
        130 => {
            blk = 83; continue;
        }
        131 => {
            let Some(t175) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083104b); };
            c.w32(8, t175);
            let r176 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r176);
            let Some(t177) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000831052); };
            c.w32(9, t177);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 154; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 142; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083107c); };
            c.w64(0, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831080); };
            c.w64(3, t179);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831087); }
            blk = 73; continue;
        }
        135 => {
            let Some(t180) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083108d); };
            c.w32(8, t180);
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000831091); };
            c.cmp(t181, c.r32(8), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083109a); };
            c.w64(0, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083109e); };
            c.w64(3, t183);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008310a5); }
            blk = 73; continue;
        }
        137 => {
            let Some(t184) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008310ab); };
            c.w32(8, t184);
            let r185 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r185);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008310b8); };
            c.w64(0, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008310bc); };
            c.w64(3, t187);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008310c3); }
            blk = 73; continue;
        }
        139 => {
            let Some(t188) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008310c9); };
            c.w32(9, t188);
            let r189 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r189);
            let Some(t190) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008310d0); };
            c.cmp(t190, 0x40u64, 16);
            if c.cc_b() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008310d8); };
            c.w64(0, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008310dc); };
            c.w64(3, t192);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008310e3); }
            blk = 73; continue;
        }
        141 => {
            let Some(t193) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008310e9); };
            c.w32(8, t193);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 83; continue;
        }
        142 => {
            let Some(t194) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008310f6); };
            c.w32(9, t194);
            let Some(t195) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008310fa); };
            c.cmp(t195, c.r32(9), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831103); };
            c.w64(0, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831107); };
            c.w64(3, t197);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083110e); }
            blk = 73; continue;
        }
        144 => {
            let Some(t198) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000831114); };
            c.w32(8, t198);
            c.w64(9, c.r64(8));
            let r199 = c.r32(9) & 0xc0u64;
            c.w32(9, r199);
            c.logic(r199, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t200) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083112c); };
            c.w32(9, t200);
            let r201 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r201);
            let Some(t202) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000831133); };
            c.w32(9, t202);
            let r203 = c.r32(9) & 0x1fu64;
            c.w32(9, r203);
            c.logic(r203, 32);
            let r204 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r204);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831145); };
            c.w64(0, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831149); };
            c.w64(3, t206);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831150); }
            blk = 73; continue;
        }
        147 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 153; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t207) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083115c); };
            c.w32(0, t207);
            let r208 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r208);
            let Some(t209) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000831162); };
            c.cmp(t209, c.r32(0), 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t210) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083116b); };
            c.w32(8, t210);
            let Some(t211) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083116f); };
            c.cmp(t211, c.r32(8), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 83; continue;
        }
        151 => {
            let Some(t212) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083117d); };
            c.w32(0, t212);
            let r213 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r213);
            blk = 83; continue;
        }
        152 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831188); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083118c); };
            c.w64(3, t215);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831193); }
            blk = 73; continue;
        }
        153 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831199); };
            c.w64(0, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083119d); };
            c.w64(3, t217);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008311a4); }
            blk = 73; continue;
        }
        154 => {
            let Some(t218) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008311aa); };
            c.w32(8, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008311ae); };
            c.cmp(t219, c.r32(8), 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008311b7); };
            c.w64(0, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008311bb); };
            c.w64(3, t221);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008311c2); }
            blk = 73; continue;
        }
        156 => {
            let Some(t222) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008311c8); };
            c.w32(0, t222);
            let r223 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r223);
            blk = 83; continue;
        }
        157 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008311d3); };
            c.w64(0, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008311d7); };
            c.w64(3, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008311db); };
            c.w64(9, t226);
            blk = 169; continue;
        }
        158 => {
            let Some(t227) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008311e4); };
            c.w32(11, t227);
            let r228 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r228);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 159; continue;
        }
        159 => {
            let r229 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r229);
            c.logic(r229, 64);
            let Some(t230) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008311f9); };
            c.w32(12, t230);
            let r231 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r231);
            let Some(t232) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000831200); };
            c.w32(12, t232);
            let r233 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r233);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t234) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083120d); };
            c.w32(15, t234);
            let r235 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r235);
            let Some(t236) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000831214); };
            c.cmp(t236, 0xbeu64, 16);
            if c.cc_e() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831220); }
            blk = 73; continue;
        }
        162 => {
            let Some(t237) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831226); };
            c.w32(15, t237);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            return RegfileExit::fault(0x7000831239);
        }
        164 => {
            let Some(t238) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083123a); };
            c.w32(15, t238);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t239) = c.ld32(c.r64(11).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x7000831242); };
            c.w32(11, t239);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            return RegfileExit::fault(0x7000831255);
        }
        166 => {
            let Some(t240) = c.ld32(c.r64(12).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x7000831256); };
            c.cmp(t240, c.r32(11), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831263); }
            blk = 73; continue;
        }
        168 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083126c); }
            blk = 73; continue;
        }
        169 => {
            let Some(t241) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000831272); };
            c.w32(11, t241);
            let r242 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r242);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 176; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t243) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700083127f); };
            c.cmp(t243, c.r32(11), 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let r244 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r244);
            c.logic(r244, 64);
            blk = 175; continue;
        }
        173 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831297); }
            blk = 73; continue;
        }
        174 => {
            c.w64(11, c.r64(2));
            blk = 175; continue;
        }
        175 => {
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008312ac); }
            blk = 16; continue;
        }
        176 => {
            c.w64(0, c.r64(11));
            blk = 83; continue;
        }
        177 => {
            let Some(t245) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008312ba); };
            c.w64(5, t245);
            let r246 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r246);
            let Some(t247) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008312c2); };
            c.logic(t247 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let r248 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r248);
            blk = 180; continue;
        }
        179 => {
            let Some(t249) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008312d2); };
            c.w32(8, t249);
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008312d6); };
            c.w64(8, t250);
            blk = 180; continue;
        }
        180 => {
            return RegfileExit::jump(c.r64(8));
        }
        181 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008312e1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008312e5); }
            let r251 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r251);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008312eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008312ef); }
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_e() { blk = 268; continue; }
            blk = 182; continue;
        }
        182 => {
            c.cmp(c.r64(12), 0x6u64, 64);
            if c.cc_e() { blk = 266; continue; }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 263; continue; }
            blk = 184; continue;
        }
        184 => {
            c.cmp(c.r64(12), 0x8u64, 64);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 185; continue;
        }
        185 => {
            c.cmp(c.r64(12), 0x9u64, 64);
            if c.cc_e() { blk = 252; continue; }
            blk = 186; continue;
        }
        186 => {
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_e() { blk = 197; continue; }
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 265; continue; }
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r64(12), 0xcu64, 64);
            if c.cc_e() { blk = 195; continue; }
            blk = 189; continue;
        }
        189 => {
            c.cmp(c.r64(12), 0xeu64, 64);
            if c.cc_e() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t252) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000831355); };
            c.w64(5, t252);
            let r253 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r253);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083135d); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700083135e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000831360); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000831361); }
            let Some(t254) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000831362); };
            if !c.push64(t254) { return RegfileExit::fault(0x7000831362); }
            c.w32(0, 0x5u64);
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700083136b); };
            c.w64(3, t255);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x7000831372); }
            return RegfileExit::jump(0x700086ed74);
        }
        191 => {
            let Some(t256) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083137c); };
            c.w32(8, t256);
            let r257 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r257);
            let Some(t258) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831383); };
            c.w32(8, t258);
            let r259 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r259);
            let Some(t260) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083138a); };
            c.w32(9, t260);
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r261 = c.r64(11) & 0x1ffffffu64;
            c.w64(11, r261);
            c.logic(r261, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            return RegfileExit::fault(0x70008313a4);
        }
        193 => {
            let Some(t262) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008313a5); };
            c.w32(8, t262);
            let r263 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r263);
            let Some(t264) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008313ad); };
            c.w32(0, t264);
            let r265 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r265);
            let Some(t266) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008313b4); };
            c.cmp(t266, c.r32(0), 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t267) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008313c1); };
            c.w64(5, t267);
            let r268 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r268);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008313c9); }
            let Some(t269) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008313ca); };
            if !c.push64(t269) { return RegfileExit::fault(0x70008313ca); }
            let Some(t270) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008313ce); };
            c.w64(3, t270);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x70008313da); }
            return RegfileExit::jump(0x700086ed74);
        }
        195 => {
            let Some(t271) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008313e8); };
            c.w64(5, t271);
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            c.w64(0, c.r64(9));
            let r272 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r272);
            return RegfileExit::jump(0x70008bc174);
        }
        196 => {
            blk = 265; continue;
        }
        197 => {
            let Some(t273) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000831408); };
            c.w32(8, t273);
            let r274 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r274);
            let Some(t275) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083140f); };
            c.w32(11, t275);
            let Some(t276) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000831413); };
            c.cmp(t276, c.r32(8), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 198; continue;
        }
        198 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 250; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t277) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083142a); };
            c.w32(11, t277);
            let r278 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r278);
            let Some(t279) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000831431); };
            c.cmp(t279, c.r32(11), 32);
            if c.cc_ne() { blk = 224; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t280) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083143e); };
            c.w32(11, t280);
            let r281 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r281);
            let Some(t282) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831445); };
            c.w32(12, t282);
            let r283 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r283);
            let Some(t284) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083144c); };
            c.logic(t284 & 0x2u64, 8);
            if c.cc_ne() { blk = 222; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t285) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000831458); };
            c.cmp(t285, c.r32(12), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000831461); };
            c.w64(0, t286);
            blk = 83; continue;
        }
        203 => {
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083146d); };
            c.w64(12, t287);
            let Some(t288) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x7000831474); };
            c.cmp(t288, 0x20u64, 8);
            if c.cc_e() { blk = 220; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t289) = c.ld32(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x700083147f); };
            c.w32(12, t289);
            let Some(t290) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000831483); };
            c.cmp(t290, c.r32(12), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083148f); }
            blk = 73; continue;
        }
        206 => {
            let Some(t291) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831495); };
            c.w32(12, t291);
            let r292 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r292);
            let Some(t293) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083149c); };
            c.w32(15, t293);
            let Some(t294) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008314a1); };
            c.cmp(t294, c.r32(15), 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t295) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008314aa); };
            c.w32(12, t295);
            let r296 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r296);
            let Some(t297) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008314b2); };
            c.w32(15, t297);
            let Some(t298) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008314b7); };
            c.cmp(t298, c.r32(15), 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 208; continue;
        }
        208 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t299) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008314ca); };
            c.w32(15, t299);
            let r300 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r300);
            let Some(t301) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008314d2); };
            c.w32(7, t301);
            c.w32(0, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 214; continue; }
            blk = 210; continue;
        }
        210 => {
            c.w32(12, c.r16(7));
            c.cmp(c.r32(12), 0x82au64, 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t302) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008314ef); };
            c.cmp(t302, c.r32(15), 32);
            if c.cc_ne() { blk = 215; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008314f8); };
            c.w64(0, t303);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008314ff); }
            blk = 73; continue;
        }
        213 => {
            c.w64(9, c.r64(8));
            blk = 215; continue;
        }
        214 => {
            c.w64(9, c.r64(12));
            blk = 215; continue;
        }
        215 => {
            let Some(t304) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083150d); };
            c.w32(8, t304);
            let r305 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r305);
            let Some(t306) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831514); };
            c.w32(8, t306);
            let r307 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r307);
            let Some(t308) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083151b); };
            c.w32(9, t308);
            let r309 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r309);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t310) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083152d); };
            c.w32(6, t310);
            let r311 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r311);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000831534); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(11));
            return c.call(0x70007f6fb4, 0x700083153f, 0x700083153a);
        }
        217 => {
            blk = 83; continue;
        }
        218 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831544); };
            c.w64(0, t312);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083154b); }
            blk = 73; continue;
        }
        219 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831554); }
            blk = 73; continue;
        }
        220 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700083155a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083155c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083155d); }
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083155e); };
            c.w64(3, t313);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700083156f, 0x700083156a);
        }
        221 => {
            blk = 83; continue;
        }
        222 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000831574); }
            c.w64(7, c.r64(11));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x700083157f, 0x700083157a);
        }
        223 => {
            blk = 83; continue;
        }
        224 => {
            let Some(t314) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831584); };
            c.w32(12, t314);
            let r315 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r315);
            let Some(t316) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083158b); };
            c.w32(15, t316);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 247; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w32(7, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 235; continue; }
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_e() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008315b4); }
            blk = 73; continue;
        }
        228 => {
            let Some(t317) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008315ba); };
            c.w32(9, t317);
            let Some(t318) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008315be); };
            c.cmp(t318, c.r32(9), 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008315ca); }
            blk = 73; continue;
        }
        230 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008315d9); }
            blk = 73; continue;
        }
        232 => {
            let Some(t319) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008315df); };
            c.cmp(t319, 0x40u64, 16);
            if c.cc_b() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008315ea); }
            blk = 73; continue;
        }
        234 => {
            let Some(t320) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008315f0); };
            c.w32(8, t320);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 83; continue;
        }
        235 => {
            let Some(t321) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008315fd); };
            c.w32(11, t321);
            let Some(t322) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000831601); };
            c.cmp(t322, c.r32(11), 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083160d); }
            blk = 73; continue;
        }
        237 => {
            let Some(t323) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000831613); };
            c.w32(11, t323);
            c.w64(12, c.r64(11));
            let r324 = c.r32(12) & 0xc0u64;
            c.w32(12, r324);
            c.logic(r324, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t325) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083162c); };
            c.w32(8, t325);
            let r326 = c.r32(8) & 0x1fu64;
            c.w32(8, r326);
            c.logic(r326, 32);
            let r327 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r327);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_be() { blk = 240; continue; }
            blk = 239; continue;
        }
        239 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831641); }
            blk = 73; continue;
        }
        240 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 246; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t328) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083164d); };
            c.w32(8, t328);
            let r329 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r329);
            let Some(t330) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000831654); };
            c.cmp(t330, c.r32(8), 32);
            if c.cc_e() { blk = 245; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t331) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083165d); };
            c.w32(9, t331);
            let Some(t332) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000831661); };
            c.cmp(t332, c.r32(9), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w64(0, c.r64(8));
            blk = 83; continue;
        }
        244 => {
            let Some(t333) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000831672); };
            c.w32(0, t333);
            let r334 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r334);
            blk = 83; continue;
        }
        245 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831681); }
            blk = 73; continue;
        }
        246 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083168a); }
            blk = 73; continue;
        }
        247 => {
            let Some(t335) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831690); };
            c.w32(9, t335);
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000831694); };
            c.cmp(t336, c.r32(9), 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008316a0); }
            blk = 73; continue;
        }
        249 => {
            c.w64(0, c.r64(8));
            blk = 83; continue;
        }
        250 => {
            c.w64(0, c.r64(8));
            blk = 83; continue;
        }
        251 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008316b9); }
            blk = 73; continue;
        }
        252 => {
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008316bf); };
            c.w64(12, t337);
            let Some(t338) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x70008316c6); };
            c.cmp(t338, 0x20u64, 8);
            if c.cc_e() { blk = 265; continue; }
            blk = 253; continue;
        }
        253 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t339) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008316d7); };
            c.w32(8, t339);
            let r340 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r340);
            blk = 256; continue;
        }
        255 => {
            let Some(t341) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008316e0); };
            c.w32(8, t341);
            let r342 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r342);
            blk = 256; continue;
        }
        256 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 258; continue; }
            blk = 257; continue;
        }
        257 => {
            return RegfileExit::fault(0x70008316ed);
        }
        258 => {
            let r343 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r343);
            c.logic(r343, 64);
            let Some(t344) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008316f2); };
            c.w32(11, t344);
            let r345 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r345);
            let Some(t346) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008316f9); };
            c.w32(3, t346);
            let r347 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r347);
            let Some(t348) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831700); };
            c.w64(9, t348);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t349) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083170a); };
            c.w32(11, t349);
            let r350 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r350);
            let Some(t351) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000831711); };
            c.w32(11, t351);
            let r352 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r352);
            blk = 261; continue;
        }
        260 => {
            c.w64(11, c.r64(2));
            blk = 261; continue;
        }
        261 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083171d); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            c.w64(7, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x700083172e, 0x7000831729);
        }
        262 => {
            blk = 83; continue;
        }
        263 => {
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000831733); };
            c.w64(8, t353);
            let Some(t354) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x700083173a); };
            c.cmp(t354, 0x20u64, 8);
            if c.cc_e() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r355 = c.r64(11) & 0x3ffu64;
            c.w64(11, r355);
            c.logic(r355, 64);
            let r356 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r356);
            let Some(t357) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083174f); };
            c.w32(11, t357);
            let r358 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r358);
            let Some(t359) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000831756); };
            c.w32(11, t359);
            let r360 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r360);
            let Some(t361) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083175d); };
            c.w32(3, t361);
            let r362 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r362);
            let Some(t363) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000831765); };
            c.w64(5, t363);
            c.w64(1, c.r64(9));
            let r364 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r364);
            return RegfileExit::jump(0x70007fea14);
        }
        265 => {
            let Some(t365) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000831775); };
            c.w64(5, t365);
            let r366 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r366);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083177d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083177e); }
            let Some(t367) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083177f); };
            if !c.push64(t367) { return RegfileExit::fault(0x700083177f); }
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000831783); };
            c.w64(3, t368);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700083178f); }
            return RegfileExit::jump(0x700086ed74);
        }
        266 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r369 = c.r64(11) & 0x3ffu64;
            c.w64(11, r369);
            c.logic(r369, 64);
            let r370 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r370);
            let Some(t371) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008317a8); };
            c.w32(9, t371);
            let r372 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r372);
            let Some(t373) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008317af); };
            c.w32(9, t373);
            let r374 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r374);
            let Some(t375) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008317b6); };
            c.w32(8, t375);
            let r376 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r376);
            let Some(t377) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008317be); };
            c.w32(7, t377);
            let r378 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r378);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008317c5); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x70008317cd, 0x70008317c8);
        }
        267 => {
            blk = 83; continue;
        }
        268 => {
            let Some(t379) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008317d2); };
            c.w32(8, t379);
            let r380 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r380);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 270; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t381) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008317df); };
            c.w64(8, t381);
            blk = 270; continue;
        }
        270 => {
            let Some(t382) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008317e6); };
            c.w32(11, t382);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r383 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r383);
            let Some(t384) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008317f4); };
            c.w32(12, t384);
            let r385 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r385);
            let r386 = c.r64(12) & c.r64(11);
            c.w64(12, r386);
            c.logic(r386, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 271; continue;
        }
        271 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(0, c.r64(7).wrapping_mul(4));
            let Some(t387) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083180f); };
            c.w32(3, t387);
            let Some(t388) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000831814); };
            c.cmp(t388, c.r32(3), 32);
            if c.cc_e() { blk = 327; continue; }
            blk = 272; continue;
        }
        272 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            let r389 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r389);
            let r390 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r390);
            c.w64(0, c.r64(11));
            let r391 = c.r64(0) & c.r64(12);
            c.w64(0, r391);
            c.logic(r391, 64);
            c.w64(12, c.r64(0));
            blk = 271; continue;
        }
        274 => {
            let Some(t392) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831837); };
            c.w32(11, t392);
            let Some(t393) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083183b); };
            c.w32(12, t393);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 276; continue; }
            blk = 275; continue;
        }
        275 => {
            return RegfileExit::fault(0x700083184f);
        }
        276 => {
            let Some(t394) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000831850); };
            c.w32(0, t394);
            let r395 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r395);
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 150; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t396) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831862); };
            c.w32(8, t396);
            let Some(t397) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000831866); };
            c.cmp(t397, c.r32(8), 32);
            if c.cc_ne() { blk = 302; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t398) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831873); };
            c.w32(8, t398);
            let r399 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r399);
            let Some(t400) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083187a); };
            c.w32(11, t400);
            let r401 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r401);
            let Some(t402) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000831881); };
            c.logic(t402 & 0x2u64, 8);
            if c.cc_ne() { blk = 300; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t403) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083188c); };
            c.cmp(t403, c.r32(11), 32);
            if c.cc_e() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t404) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000831895); };
            c.w64(0, t404);
            blk = 83; continue;
        }
        281 => {
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008318a1); };
            c.w64(11, t405);
            let Some(t406) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008318a8); };
            c.cmp(t406, 0x20u64, 8);
            if c.cc_e() { blk = 298; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t407) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008318b2); };
            c.w32(11, t407);
            let Some(t408) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008318b6); };
            c.cmp(t408, c.r32(11), 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008318bf); };
            c.w64(0, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008318c3); };
            c.w64(3, t410);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008318ca); }
            blk = 73; continue;
        }
        284 => {
            let Some(t411) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008318d0); };
            c.w32(11, t411);
            let r412 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r412);
            let Some(t413) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008318d7); };
            c.w32(12, t413);
            let Some(t414) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008318db); };
            c.cmp(t414, c.r32(12), 32);
            if c.cc_ne() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t415) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008318e4); };
            c.w32(11, t415);
            let r416 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r416);
            let Some(t417) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008318eb); };
            c.w32(12, t417);
            let Some(t418) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008318ef); };
            c.cmp(t418, c.r32(12), 32);
            if c.cc_e() { blk = 285; continue; }
            blk = 286; continue;
        }
        286 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 297; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t419) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831902); };
            c.w32(12, t419);
            let r420 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r420);
            let Some(t421) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000831909); };
            c.w32(15, t421);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 292; continue; }
            blk = 288; continue;
        }
        288 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 291; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t422) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000831928); };
            c.cmp(t422, c.r32(12), 32);
            if c.cc_ne() { blk = 293; continue; }
            blk = 290; continue;
        }
        290 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831931); };
            c.w64(0, t423);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831935); };
            c.w64(3, t424);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083193c); }
            blk = 73; continue;
        }
        291 => {
            let Some(t425) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000831942); };
            c.w32(11, t425);
            let r426 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r426);
            c.w64(9, c.r64(11));
            blk = 293; continue;
        }
        292 => {
            c.w64(9, c.r64(11));
            blk = 293; continue;
        }
        293 => {
            let Some(t427) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000831951); };
            c.w32(9, t427);
            let r428 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r428);
            let Some(t429) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831958); };
            c.w32(9, t429);
            let r430 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r430);
            let Some(t431) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083195f); };
            c.w32(11, t431);
            let r432 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r432);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 296; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t433) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000831971); };
            c.w32(6, t433);
            let r434 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r434);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000831978); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007f6fb4, 0x7000831983, 0x700083197e);
        }
        295 => {
            blk = 83; continue;
        }
        296 => {
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831988); };
            c.w64(0, t435);
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083198c); };
            c.w64(3, t436);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831993); }
            blk = 73; continue;
        }
        297 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831999); };
            c.w64(0, t437);
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083199d); };
            c.w64(3, t438);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008319a4); }
            blk = 73; continue;
        }
        298 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008319aa); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008319ac); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008319ad); }
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008319ae); };
            c.w64(3, t439);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008319bf, 0x70008319ba);
        }
        299 => {
            blk = 83; continue;
        }
        300 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008319c4); }
            c.w64(7, c.r64(8));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x70008319cf, 0x70008319ca);
        }
        301 => {
            blk = 83; continue;
        }
        302 => {
            let Some(t440) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008319d4); };
            c.w32(8, t440);
            let r441 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r441);
            let Some(t442) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008319db); };
            c.w32(11, t442);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 324; continue; }
            blk = 303; continue;
        }
        303 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 313; continue; }
            blk = 304; continue;
        }
        304 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_e() { blk = 306; continue; }
            blk = 305; continue;
        }
        305 => {
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831a05); };
            c.w64(0, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831a09); };
            c.w64(3, t444);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831a10); }
            blk = 73; continue;
        }
        306 => {
            let Some(t445) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831a16); };
            c.w32(8, t445);
            let Some(t446) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000831a1a); };
            c.cmp(t446, c.r32(8), 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831a23); };
            c.w64(0, t447);
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831a27); };
            c.w64(3, t448);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831a2e); }
            blk = 73; continue;
        }
        308 => {
            let Some(t449) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000831a34); };
            c.w32(8, t449);
            let r450 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r450);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 310; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831a41); };
            c.w64(0, t451);
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831a45); };
            c.w64(3, t452);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831a4c); }
            blk = 73; continue;
        }
        310 => {
            let Some(t453) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831a52); };
            c.w32(9, t453);
            let r454 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r454);
            let Some(t455) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000831a59); };
            c.cmp(t455, 0x40u64, 16);
            if c.cc_b() { blk = 312; continue; }
            blk = 311; continue;
        }
        311 => {
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831a61); };
            c.w64(0, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831a65); };
            c.w64(3, t457);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831a6c); }
            blk = 73; continue;
        }
        312 => {
            let Some(t458) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000831a72); };
            c.w32(8, t458);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 83; continue;
        }
        313 => {
            let Some(t459) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831a7f); };
            c.w32(11, t459);
            let Some(t460) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000831a83); };
            c.cmp(t460, c.r32(11), 32);
            if c.cc_e() { blk = 315; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831a8c); };
            c.w64(0, t461);
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831a90); };
            c.w64(3, t462);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831a97); }
            blk = 73; continue;
        }
        315 => {
            let Some(t463) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000831a9d); };
            c.w32(8, t463);
            c.w64(11, c.r64(8));
            let r464 = c.r32(11) & 0xc0u64;
            c.w32(11, r464);
            c.logic(r464, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 318; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t465) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000831ab5); };
            c.w32(11, t465);
            let r466 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r466);
            let Some(t467) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000831abc); };
            c.w32(11, t467);
            let r468 = c.r32(11) & 0x1fu64;
            c.w32(11, r468);
            c.logic(r468, 32);
            let r469 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r469);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 318; continue; }
            blk = 317; continue;
        }
        317 => {
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831ace); };
            c.w64(0, t470);
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831ad2); };
            c.w64(3, t471);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831ad9); }
            blk = 73; continue;
        }
        318 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 323; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t472) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000831ae5); };
            c.w32(0, t472);
            let r473 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r473);
            let Some(t474) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000831aec); };
            c.cmp(t474, c.r32(0), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t475) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831af5); };
            c.w32(8, t475);
            let Some(t476) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000831af9); };
            c.cmp(t476, c.r32(8), 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t477) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000831b06); };
            c.w32(0, t477);
            let r478 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r478);
            blk = 83; continue;
        }
        322 => {
            let Some(t479) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831b11); };
            c.w64(0, t479);
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831b15); };
            c.w64(3, t480);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831b1c); }
            blk = 73; continue;
        }
        323 => {
            let Some(t481) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831b22); };
            c.w64(0, t481);
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831b26); };
            c.w64(3, t482);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831b2d); }
            blk = 73; continue;
        }
        324 => {
            let Some(t483) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831b33); };
            c.w32(8, t483);
            let Some(t484) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000831b37); };
            c.cmp(t484, c.r32(8), 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t485) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831b40); };
            c.w64(0, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831b44); };
            c.w64(3, t486);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831b4b); }
            blk = 73; continue;
        }
        326 => {
            let Some(t487) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000831b51); };
            c.w32(0, t487);
            let r488 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r488);
            blk = 83; continue;
        }
        327 => {
            let Some(t489) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831b5d); };
            c.w64(0, t489);
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000831b61); };
            c.w64(3, t490);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831b68); }
            blk = 73; continue;
        }
        328 => {
            return RegfileExit::fault(0x7000831b6e);
        }
        329 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000831b6f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000831b73); }
            let r491 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r491);
            let Some(t492) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000831b79); };
            c.w64(9, t492);
            let Some(t493) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000831b80); };
            c.w64(11, t493);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000831b87); }
            let Some(t494) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000831b8b); };
            c.w64(15, t494);
            let Some(t495) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000831b92); };
            c.cmp(t495, c.r64(12), 64);
            if c.cc_a() { blk = 332; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000831ba1, 0x7000831b9c);
        }
        331 => {
            let Some(t496) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831ba1); };
            c.w64(8, t496);
            blk = 333; continue;
        }
        332 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000831ba7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 333; continue;
        }
        333 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t497) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000831bba); };
            c.w64(8, t497);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000831bc1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000831bc5); }
            blk = 83; continue;
        }
        334 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000831bcf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000831bd3); }
            let r498 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r498);
            let Some(t499) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000831bd9); };
            c.w64(9, t499);
            let Some(t500) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000831be0); };
            c.w64(11, t500);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000831be7); }
            let Some(t501) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000831beb); };
            c.w64(15, t501);
            let Some(t502) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000831bf2); };
            c.cmp(t502, c.r64(12), 64);
            if c.cc_a() { blk = 337; continue; }
            blk = 335; continue;
        }
        335 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000831c01, 0x7000831bfc);
        }
        336 => {
            let Some(t503) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831c01); };
            c.w64(8, t503);
            blk = 338; continue;
        }
        337 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000831c07); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 338; continue;
        }
        338 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t504) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000831c1d); };
            c.w64(8, t504);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000831c24); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000831c28); }
            blk = 83; continue;
        }
        339 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000831c32); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000831c36); }
            let r505 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r505);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000831c41); }
            return c.call(0x70007febb4, 0x7000831c4a, 0x7000831c45);
        }
        340 => {
            c.w64(9, c.r64(0));
            let Some(t506) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000831c4d); };
            c.w64(8, t506);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831c54); }
            blk = 61; continue;
        }
        341 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000831c5a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000831c5e); }
            let r507 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r507);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000831c6e, 0x7000831c69);
        }
        342 => {
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831c74); }
            blk = 66; continue;
        }
        343 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000831c7a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000831c7e); }
            let r508 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r508);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000831c89); }
            return c.call(0x70007febf4, 0x7000831c93, 0x7000831c8e);
        }
        344 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000831c96); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000831c9e); }
            blk = 71; continue;
        }
        345 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000831ca4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000831ca8); }
            let r509 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r509);
            let Some(t510) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000831cae); };
            c.w64(8, t510);
            let Some(t511) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000831cb5); };
            c.w32(8, t511);
            let r512 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r512);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000831cbc); }
            let Some(t513) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000831cbe); };
            c.w64(3, t513);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000831cd1, 0x7000831ccc);
        }
        346 => {
            return RegfileExit::fault(0x7000831cd1);
        }
        347 => {
            c.w32(0, 0xau64);
            return RegfileExit::jump(0x7000804ff4);
        }
        348 => {
            blk = 73; continue;
        }
        349 => {
            return RegfileExit::jump(0x7000831ce4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000831cf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831cfd); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            c.w64(9, c.r64(8));
            if let Some(sr) = c.shr(c.r64(9), 0xdu64, 64) { c.w64(9, sr); }
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831d0b); };
            c.w32(11, t2);
            let r3 = c.r64(9) ^ c.r64(8);
            c.w64(9, r3);
            c.logic(r3, 64);
            let r4 = c.add(c.r32(9), c.r32(11), 32);
            c.w32(9, r4);
            let r5 = c.r32(9) & 0x1ffcu64;
            c.w32(9, r5);
            c.logic(r5, 32);
            let r6 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(12, r6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2f18)) else { return RegfileExit::fault(0x7000831d20); };
            c.w64(15, t7);
            let Some(t8) = c.ld32(c.r64(15).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000831d27); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000831d2d); };
            c.cmp(t9, c.r32(8), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000831d34); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            blk = 7; continue;
        }
        4 => {
            c.w32(12, c.r32(1));
            c.w32(9, c.r32(9));
            let r12 = c.sub(c.r32(9), c.r32(12), 32);
            c.w32(9, r12);
            let r13 = c.sub(c.r32(9), 0x4e93591bu64, 32);
            c.w32(9, r13);
            let r14 = c.r32(9) & 0x7fcu64;
            c.w32(9, r14);
            c.logic(r14, 32);
            let r15 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2f30)) else { return RegfileExit::fault(0x7000831d59); };
            c.w64(12, t16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000831d60); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000831d6a); };
            c.cmp(t18, c.r32(8), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000831d75); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(12, c.r64(2));
            c.w64(15, c.r64(9));
            blk = 9; continue;
        }
        9 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(7, c.r64(15));
            let r21 = c.r64(7) & 0xfu64;
            c.w64(7, r21);
            c.logic(r21, 64);
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(7), 0x5u64, 64);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(7), 0xdu64, 64);
            if c.cc_ne() { blk = 157; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000831db0); };
            c.w64(0, t22);
            return c.ret(0x0, 0x7000831db7);
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.ret(0x0, 0x7000831dbb);
        }
        14 => {
            c.logic(c.r8(15) & 0x40u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(15));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(15) & 0x100u64, 16);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let r23 = c.r64(8) & 0x7ffu64;
            c.w64(8, r23);
            c.logic(r23, 64);
            c.logic(c.r8(15) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000831de8); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000831df6); };
            c.w64(11, t26);
            blk = 18; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831dfd); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x7000831e0e);
        }
        20 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831e19); };
            c.w32(9, t29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000831e1d); };
            c.cmp(t30, c.r32(9), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000831e2a); }
            blk = 62; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831e3b); };
            c.w32(0, t31);
            let r32 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r32);
            return c.ret(0x0, 0x7000831e43);
        }
        25 => {
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831e44); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831e56); };
            c.w32(9, t35);
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000831e5a); };
            c.cmp(t36, c.r32(9), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000831e67); }
            blk = 62; continue;
        }
        28 => {
            c.w64(8, c.r64(15));
            let r37 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r37);
            let r38 = c.r32(8) & 0xfu64;
            c.w32(8, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 321; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 304; continue;
        }
        34 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 304; continue;
        }
        37 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 304; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 304; continue;
        }
        45 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r39 = c.r64(15) & 0xfffffu64;
            c.w64(15, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831f02); };
            c.w32(8, (t40 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x7000831f0c);
        }
        46 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r41 = c.r64(15) & 0xfffffu64;
            c.w64(15, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld16(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831f18); };
            c.w32(8, (t42 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x7000831f22);
        }
        47 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r43 = c.r64(15) & 0xfffffu64;
            c.w64(15, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831f2e); };
            c.w32(8, t44);
            c.w64(9, c.r64(8));
            let r45 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r45);
            if c.cc_o() { blk = 305; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x7000831f42);
        }
        49 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r46 = c.r64(15) & 0xfffffu64;
            c.w64(15, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831f4e); };
            c.w32(8, t47);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 310; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x7000831f64);
        }
        51 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r48 = c.r64(15) & 0xfffffu64;
            c.w64(15, r48);
            c.logic(r48, 64);
            let Some(t49) = c.ld64(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000831f70); };
            c.w64(8, t49);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 57; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000831f7b); };
            c.w64(9, t50);
            let Some(t51) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000831f82); };
            c.w64(11, t51);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000831f89); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000831f90); };
            c.cmp(t53, c.r64(12), 64);
            if c.cc_be() { blk = 315; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000831f99); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 54; continue;
        }
        54 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000831fa0); };
            c.w64(11, t54);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000831fa7); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000831fb1); }
            let r55 = c.neg(c.r64(8), 64);
            c.w64(8, r55);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000831fbc); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x7000831fc3);
        }
        56 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000831fc4); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000831fcc); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x7000831fd3);
        }
        57 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000831fd4); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000831fdb); };
            c.w64(9, t57);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000831fe2); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000831fe9); };
            c.cmp(t59, c.r64(12), 64);
            if c.cc_be() { blk = 317; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000831ff2); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 59; continue;
        }
        59 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000831ff9); };
            c.w64(9, t60);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000832000); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x7000832004); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x700083200f);
        }
        60 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r61 = c.r64(15) & 0xfffffu64;
            c.w64(15, r61);
            c.logic(r61, 64);
            if !c.movss_rm(0, c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700083201b); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 62; continue;
        }
        61 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r62 = c.r64(15) & 0xfffffu64;
            c.w64(15, r62);
            c.logic(r62, 64);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000832033); }
            blk = 62; continue;
        }
        62 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083203a); };
            c.w64(8, t63);
            let Some(t64) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000832041); };
            c.w64(9, t64);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000832048); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700083204f); };
            c.cmp(t66, c.r64(11), 64);
            if c.cc_be() { blk = 319; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000832059); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000832060); };
            c.w64(9, t67);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000832067); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083206b); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x7000832074);
        }
        65 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r68 = c.r64(15) & 0xfffffu64;
            c.w64(15, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832080); };
            c.w32(0, t69);
            let r70 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r70);
            return c.ret(0x0, 0x7000832088);
        }
        66 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000832089); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083208a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083208b); }
            let Some(t71) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083208c); };
            if !c.push64(t71) { return RegfileExit::fault(0x700083208c); }
            c.w32(0, 0x4u64);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2050)) else { return RegfileExit::fault(0x7000832095); };
            c.w64(3, t72);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x700083209c); }
            return RegfileExit::jump(0x700086ed74);
        }
        67 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008320a6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008320aa); }
            let r73 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r73);
            let Some(t74) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008320b0); };
            c.w32(12, t74);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008320b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008320b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008320bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008320c0); }
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70008320c4); };
            c.cmp(t75, c.r32(12), 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t76) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008320d1); };
            c.w32(12, t76);
            let r77 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r77);
            c.w32(15, 0);
            c.logic(0, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t78) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008320e0); };
            c.cmp(t78, c.r32(15), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008320ea); }
            blk = 66; continue;
        }
        71 => {
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008320ed); };
            c.w32(12, t79);
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008320f7); }
            c.logic(c.r8(7) & 0x30u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 72; continue;
        }
        72 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 135; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(7, c.r64(11));
            let r80 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r80);
            let Some(t81) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832115); };
            c.w32(0, t81);
            let r82 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r82);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083211f); };
            c.w64(0, t83);
            blk = 75; continue;
        }
        75 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832126); };
            c.w32(3, t84);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r85 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r85);
            let r86 = c.r64(7) & c.r64(3);
            c.w64(7, r86);
            c.logic(r86, 64);
            blk = 76; continue;
        }
        76 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(12, c.r64(11).wrapping_mul(4));
            let Some(t87) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832143); };
            c.w32(6, t87);
            let Some(t88) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000832148); };
            c.cmp(t88, c.r32(6), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let r89 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r89);
            c.w64(11, c.r64(15).wrapping_add(c.r64(7)));
            c.w64(12, c.r64(3));
            let r90 = c.r64(12) & c.r64(11);
            c.w64(12, r90);
            c.logic(r90, 64);
            c.w64(7, c.r64(12));
            blk = 76; continue;
        }
        79 => {
            let Some(t91) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083216c); };
            c.w32(9, t91);
            let Some(t92) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832170); };
            c.w32(15, t92);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r93 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r93);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            return RegfileExit::fault(0x7000832184);
        }
        81 => {
            let Some(t94) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832185); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r95);
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 127; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t96) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832197); };
            c.w32(9, t96);
            let Some(t97) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083219b); };
            c.cmp(t97, c.r32(9), 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t98) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008321a8); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008321af); };
            c.w32(11, t100);
            let r101 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r101);
            let Some(t102) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008321b6); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_ne() { blk = 106; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008321c1); };
            c.cmp(t103, c.r32(11), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008321ca); };
            c.w64(0, t104);
            blk = 86; continue;
        }
        86 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008321d4); }
            return c.ret(0x0, 0x70008321d5);
        }
        87 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008321d6); };
            c.w64(11, t105);
            let Some(t106) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008321dd); };
            c.cmp(t106, 0x20u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t107) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008321e7); };
            c.w32(11, t107);
            let Some(t108) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008321eb); };
            c.cmp(t108, c.r32(11), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008321f4); };
            c.w64(0, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008321f8); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008321fc); };
            c.w64(6, t111);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832203); }
            blk = 66; continue;
        }
        90 => {
            let Some(t112) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832209); };
            c.w32(11, t112);
            let Some(t113) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083220d); };
            c.cmp(t113, c.r32(11), 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000832216); };
            c.w32(8, t114);
            let r115 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r115);
            let Some(t116) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083221d); };
            c.w32(11, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000832221); };
            c.cmp(t117, c.r32(11), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 92; continue;
        }
        92 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832234); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083223b); };
            c.w32(12, t120);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 99; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w32(8, c.r16(12));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083225b); };
            c.cmp(t121, c.r32(11), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(8, c.r64(2));
            blk = 99; continue;
        }
        97 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832269); };
            c.w64(0, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083226d); };
            c.w64(3, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000832271); };
            c.w64(6, t124);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832278); }
            blk = 66; continue;
        }
        98 => {
            let Some(t125) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083227e); };
            c.w32(8, t125);
            let r126 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r126);
            blk = 99; continue;
        }
        99 => {
            let Some(t127) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832285); };
            c.w32(8, t127);
            let r128 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r128);
            let Some(t129) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083228c); };
            c.w32(8, t129);
            let r130 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r130);
            let Some(t131) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832293); };
            c.w32(11, t131);
            let r132 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r132);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 102; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t133) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008322a5); };
            c.w32(6, t133);
            let r134 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r134);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008322ac); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x70008322b7, 0x70008322b2);
        }
        101 => {
            blk = 86; continue;
        }
        102 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008322bc); };
            c.w64(0, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008322c0); };
            c.w64(3, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008322c4); };
            c.w64(6, t137);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008322cb); }
            blk = 66; continue;
        }
        103 => {
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008322d1); };
            c.w64(0, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008322d5); };
            c.w64(3, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008322d9); };
            c.w64(6, t140);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008322e0); }
            blk = 66; continue;
        }
        104 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008322e6); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008322e7); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008322e8); }
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008322e9); };
            c.w64(3, t141);
            c.w32(0, 0x3u64);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008322f5); };
            c.w64(6, t142);
            return c.call(0x700086ed74, 0x70008322fe, 0x70008322f9);
        }
        105 => {
            blk = 86; continue;
        }
        106 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832303); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(9));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000832309); };
            c.w64(6, t143);
            return c.call(0x70007f2274, 0x7000832312, 0x700083230d);
        }
        107 => {
            blk = 86; continue;
        }
        108 => {
            let Some(t144) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832317); };
            c.w32(9, t144);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 131; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 119; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832341); };
            c.w64(0, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832345); };
            c.w64(3, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000832349); };
            c.w64(6, t147);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832350); }
            blk = 66; continue;
        }
        112 => {
            let Some(t148) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832356); };
            c.w32(8, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083235a); };
            c.cmp(t149, c.r32(8), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832363); };
            c.w64(0, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832367); };
            c.w64(3, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083236b); };
            c.w64(6, t152);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832372); }
            blk = 66; continue;
        }
        114 => {
            let Some(t153) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832378); };
            c.w32(8, t153);
            let r154 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r154);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832385); };
            c.w64(0, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832389); };
            c.w64(3, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083238d); };
            c.w64(6, t157);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832394); }
            blk = 66; continue;
        }
        116 => {
            let Some(t158) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083239a); };
            c.w32(9, t158);
            let r159 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r159);
            let Some(t160) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008323a1); };
            c.cmp(t160, 0x40u64, 16);
            if c.cc_b() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008323a9); };
            c.w64(0, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008323ad); };
            c.w64(3, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008323b1); };
            c.w64(6, t163);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008323b8); }
            blk = 66; continue;
        }
        118 => {
            let Some(t164) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008323be); };
            c.w32(8, t164);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 86; continue;
        }
        119 => {
            let Some(t165) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008323cb); };
            c.w32(9, t165);
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008323cf); };
            c.cmp(t166, c.r32(9), 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008323d8); };
            c.w64(0, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008323dc); };
            c.w64(3, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008323e0); };
            c.w64(6, t169);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008323e7); }
            blk = 66; continue;
        }
        121 => {
            let Some(t170) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008323ed); };
            c.w32(8, t170);
            c.w64(9, c.r64(8));
            let r171 = c.r32(9) & 0xc0u64;
            c.w32(9, r171);
            c.logic(r171, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t172) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832405); };
            c.w32(9, t172);
            let r173 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r173);
            let Some(t174) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083240c); };
            c.w32(9, t174);
            let r175 = c.r32(9) & 0x1fu64;
            c.w32(9, r175);
            c.logic(r175, 32);
            let r176 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r176);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083241e); };
            c.w64(0, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832422); };
            c.w64(3, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000832426); };
            c.w64(6, t179);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083242d); }
            blk = 66; continue;
        }
        124 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 130; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t180) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000832439); };
            c.w32(0, t180);
            let r181 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r181);
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083243f); };
            c.cmp(t182, c.r32(0), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t183) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832448); };
            c.w32(8, t183);
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083244c); };
            c.cmp(t184, c.r32(8), 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            blk = 86; continue;
        }
        128 => {
            let Some(t185) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083245a); };
            c.w32(0, t185);
            let r186 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r186);
            blk = 86; continue;
        }
        129 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832465); };
            c.w64(0, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832469); };
            c.w64(3, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083246d); };
            c.w64(6, t189);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832474); }
            blk = 66; continue;
        }
        130 => {
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083247a); };
            c.w64(0, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083247e); };
            c.w64(3, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000832482); };
            c.w64(6, t192);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832489); }
            blk = 66; continue;
        }
        131 => {
            let Some(t193) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083248f); };
            c.w32(8, t193);
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000832493); };
            c.cmp(t194, c.r32(8), 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083249c); };
            c.w64(0, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008324a0); };
            c.w64(3, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008324a4); };
            c.w64(6, t197);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008324ab); }
            blk = 66; continue;
        }
        133 => {
            let Some(t198) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008324b1); };
            c.w32(0, t198);
            let r199 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r199);
            blk = 86; continue;
        }
        134 => {
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008324bc); };
            c.w64(3, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008324c0); };
            c.w64(6, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008324c4); };
            c.w64(11, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008324c8); };
            c.w64(12, t203);
            blk = 146; continue;
        }
        135 => {
            let Some(t204) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008324d1); };
            c.w32(15, t204);
            let r205 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r205);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 136; continue;
        }
        136 => {
            let r206 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r206);
            c.logic(r206, 64);
            let Some(t207) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008324e2); };
            c.w32(7, t207);
            let r208 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r208);
            let Some(t209) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008324e8); };
            c.w32(7, t209);
            let r210 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r210);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t211) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008324f3); };
            c.cmp(t211, 0xbeu64, 16);
            if c.cc_e() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008324ff); }
            blk = 66; continue;
        }
        139 => {
            let Some(t212) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832505); };
            c.w32(0, t212);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xa9u64, 64);
            if c.cc_a() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            return RegfileExit::fault(0x7000832517);
        }
        141 => {
            let Some(t213) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832518); };
            c.w32(0, t213);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t214) = c.ld32(c.r64(15).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083251e); };
            c.w32(15, t214);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xa9u64, 64);
            if c.cc_a() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            return RegfileExit::fault(0x7000832530);
        }
        143 => {
            let Some(t215) = c.ld32(c.r64(7).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x7000832531); };
            c.cmp(t215, c.r32(15), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083253a); };
            c.w64(0, t216);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832541); }
            blk = 66; continue;
        }
        145 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083254a); }
            blk = 66; continue;
        }
        146 => {
            let Some(t217) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832550); };
            c.w32(0, t217);
            let r218 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r218);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 127; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700083255f); };
            c.cmp(t219, c.r32(0), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let r220 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r220);
            c.logic(r220, 64);
            c.w64(15, c.r64(0));
            blk = 152; continue;
        }
        150 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832576); };
            c.w64(0, t221);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083257d); }
            blk = 66; continue;
        }
        151 => {
            c.w64(15, c.r64(2));
            blk = 152; continue;
        }
        152 => {
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(15));
            c.w64(15, c.r64(10));
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083258f); };
            c.w64(0, t222);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832596); }
            blk = 9; continue;
        }
        153 => {
            let Some(t223) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083259c); };
            c.w64(5, t223);
            let r224 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r224);
            let Some(t225) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008325a4); };
            c.logic(t225 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let r226 = c.add(c.r64(9), 0x3fu64, 64);
            c.w64(9, r226);
            blk = 156; continue;
        }
        155 => {
            let Some(t227) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008325b4); };
            c.w32(9, t227);
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008325b8); };
            c.w64(9, t228);
            blk = 156; continue;
        }
        156 => {
            return RegfileExit::jump(c.r64(9));
        }
        157 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008325c3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008325c7); }
            let r229 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r229);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008325cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008325d1); }
            c.cmp(c.r64(7), 0x2u64, 64);
            if c.cc_e() { blk = 244; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r64(7), 0x6u64, 64);
            if c.cc_e() { blk = 242; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 239; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r64(7), 0x8u64, 64);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r64(7), 0x9u64, 64);
            if c.cc_e() { blk = 228; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r64(7), 0x3u64, 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 241; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r64(7), 0xcu64, 64);
            if c.cc_e() { blk = 171; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r64(7), 0xeu64, 64);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t230) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000832637); };
            c.w64(5, t230);
            let r231 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r231);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083263f); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000832640); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000832642); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000832643); }
            let Some(t232) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000832644); };
            if !c.push64(t232) { return RegfileExit::fault(0x7000832644); }
            c.w32(0, 0x5u64);
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700083264d); };
            c.w64(3, t233);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x7000832654); }
            return RegfileExit::jump(0x700086ed74);
        }
        167 => {
            let Some(t234) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083265e); };
            c.w32(8, t234);
            let r235 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r235);
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832666); };
            c.w32(8, t236);
            let r237 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r237);
            let Some(t238) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083266d); };
            c.w32(9, t238);
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r239 = c.r64(15) & 0x1ffffffu64;
            c.w64(15, r239);
            c.logic(r239, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            return RegfileExit::fault(0x7000832687);
        }
        169 => {
            let Some(t240) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832688); };
            c.w32(8, t240);
            let r241 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r241);
            let Some(t242) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832690); };
            c.w32(0, t242);
            let r243 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r243);
            let Some(t244) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000832697); };
            c.cmp(t244, c.r32(0), 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t245) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008326a4); };
            c.w64(5, t245);
            let r246 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r246);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008326ac); }
            let Some(t247) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008326ad); };
            if !c.push64(t247) { return RegfileExit::fault(0x70008326ad); }
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008326b1); };
            c.w64(3, t248);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x70008326bd); }
            return RegfileExit::jump(0x700086ed74);
        }
        171 => {
            let Some(t249) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008326cb); };
            c.w64(5, t249);
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            c.w64(0, c.r64(12));
            let r250 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r250);
            return RegfileExit::jump(0x70008bc174);
        }
        172 => {
            blk = 241; continue;
        }
        173 => {
            let Some(t251) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008326eb); };
            c.w32(8, t251);
            let r252 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r252);
            let Some(t253) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008326f3); };
            c.w32(9, t253);
            let Some(t254) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008326f8); };
            c.cmp(t254, c.r32(8), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 174; continue;
        }
        174 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 226; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t255) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083270f); };
            c.w32(9, t255);
            let r256 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r256);
            let Some(t257) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000832716); };
            c.cmp(t257, c.r32(9), 32);
            if c.cc_ne() { blk = 200; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t258) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832723); };
            c.w32(9, t258);
            let r259 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r259);
            let Some(t260) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083272a); };
            c.w32(11, t260);
            let r261 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r261);
            let Some(t262) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000832731); };
            c.logic(t262 & 0x2u64, 8);
            if c.cc_ne() { blk = 198; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t263) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083273c); };
            c.cmp(t263, c.r32(11), 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000832745); };
            c.w64(0, t264);
            blk = 86; continue;
        }
        179 => {
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000832751); };
            c.w64(11, t265);
            let Some(t266) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000832758); };
            c.cmp(t266, 0x20u64, 8);
            if c.cc_e() { blk = 196; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t267) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000832762); };
            c.w32(11, t267);
            let Some(t268) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000832766); };
            c.cmp(t268, c.r32(11), 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832772); }
            blk = 66; continue;
        }
        182 => {
            let Some(t269) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832778); };
            c.w32(11, t269);
            let r270 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r270);
            let Some(t271) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832780); };
            c.w32(15, t271);
            let Some(t272) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000832784); };
            c.cmp(t272, c.r32(15), 32);
            if c.cc_ne() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t273) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083278d); };
            c.w32(11, t273);
            let r274 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r274);
            let Some(t275) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832794); };
            c.w32(15, t275);
            let Some(t276) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000832798); };
            c.cmp(t276, c.r32(15), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 184; continue;
        }
        184 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 195; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t277) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008327ab); };
            c.w32(15, t277);
            let r278 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r278);
            let Some(t279) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008327b2); };
            c.w32(7, t279);
            c.w32(0, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 190; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w32(11, c.r16(7));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t280) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008327cf); };
            c.cmp(t280, c.r32(15), 32);
            if c.cc_ne() { blk = 191; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008327d8); };
            c.w64(0, t281);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008327df); }
            blk = 66; continue;
        }
        189 => {
            c.w64(12, c.r64(8));
            blk = 191; continue;
        }
        190 => {
            c.w64(12, c.r64(11));
            blk = 191; continue;
        }
        191 => {
            let Some(t282) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008327ed); };
            c.w32(8, t282);
            let r283 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r283);
            let Some(t284) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008327f5); };
            c.w32(8, t284);
            let r285 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r285);
            let Some(t286) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008327fc); };
            c.w32(11, t286);
            let r287 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r287);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 194; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t288) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083280e); };
            c.w32(6, t288);
            let r289 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r289);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832815); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x7000832820, 0x700083281b);
        }
        193 => {
            blk = 86; continue;
        }
        194 => {
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832825); };
            c.w64(0, t290);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083282c); }
            blk = 66; continue;
        }
        195 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832835); }
            blk = 66; continue;
        }
        196 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700083283b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083283d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083283e); }
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083283f); };
            c.w64(3, t291);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000832850, 0x700083284b);
        }
        197 => {
            blk = 86; continue;
        }
        198 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832855); }
            c.w64(7, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000832860, 0x700083285b);
        }
        199 => {
            blk = 86; continue;
        }
        200 => {
            let Some(t292) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832865); };
            c.w32(11, t292);
            let r293 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r293);
            let Some(t294) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083286d); };
            c.w32(15, t294);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 223; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w32(7, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 211; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_e() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832895); }
            blk = 66; continue;
        }
        204 => {
            let Some(t295) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083289b); };
            c.w32(11, t295);
            let Some(t296) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083289f); };
            c.cmp(t296, c.r32(11), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008328ab); }
            blk = 66; continue;
        }
        206 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008328ba); }
            blk = 66; continue;
        }
        208 => {
            let Some(t297) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008328c0); };
            c.cmp(t297, 0x40u64, 16);
            if c.cc_b() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008328cb); }
            blk = 66; continue;
        }
        210 => {
            let Some(t298) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008328d1); };
            c.w32(8, t298);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 86; continue;
        }
        211 => {
            let Some(t299) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008328de); };
            c.w32(9, t299);
            let Some(t300) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008328e2); };
            c.cmp(t300, c.r32(9), 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008328ee); }
            blk = 66; continue;
        }
        213 => {
            let Some(t301) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008328f4); };
            c.w32(9, t301);
            c.w64(11, c.r64(9));
            let r302 = c.r32(11) & 0xc0u64;
            c.w32(11, r302);
            c.logic(r302, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t303) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083290c); };
            c.w32(8, t303);
            let r304 = c.r32(8) & 0x1fu64;
            c.w32(8, r304);
            c.logic(r304, 32);
            let r305 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r305);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_be() { blk = 216; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832921); }
            blk = 66; continue;
        }
        216 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 222; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t306) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083292d); };
            c.w32(8, t306);
            let r307 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r307);
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000832935); };
            c.cmp(t308, c.r32(8), 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t309) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083293e); };
            c.w32(9, t309);
            let Some(t310) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000832942); };
            c.cmp(t310, c.r32(9), 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(0, c.r64(8));
            blk = 86; continue;
        }
        220 => {
            let Some(t311) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832953); };
            c.w32(0, t311);
            let r312 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r312);
            blk = 86; continue;
        }
        221 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832962); }
            blk = 66; continue;
        }
        222 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083296b); }
            blk = 66; continue;
        }
        223 => {
            let Some(t313) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832971); };
            c.w32(9, t313);
            let Some(t314) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000832975); };
            c.cmp(t314, c.r32(9), 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832981); }
            blk = 66; continue;
        }
        225 => {
            c.w64(0, c.r64(8));
            blk = 86; continue;
        }
        226 => {
            c.w64(0, c.r64(8));
            blk = 86; continue;
        }
        227 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083299a); }
            blk = 66; continue;
        }
        228 => {
            let Some(t315) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008329a0); };
            c.w64(0, t315);
            let Some(t316) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x70008329a7); };
            c.cmp(t316, 0x20u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 229; continue;
        }
        229 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 231; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t317) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008329b6); };
            c.w32(9, t317);
            let r318 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r318);
            blk = 232; continue;
        }
        231 => {
            let Some(t319) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008329bf); };
            c.w32(9, t319);
            let r320 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r320);
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            return RegfileExit::fault(0x70008329cc);
        }
        234 => {
            let r321 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r321);
            c.logic(r321, 64);
            let Some(t322) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008329d1); };
            c.w32(15, t322);
            let r323 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r323);
            let Some(t324) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008329d9); };
            c.w32(3, t324);
            let r325 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r325);
            let Some(t326) = c.ld64(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008329e1); };
            c.w64(12, t326);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t327) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008329eb); };
            c.w32(8, t327);
            let r328 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r328);
            blk = 237; continue;
        }
        236 => {
            c.w64(8, c.r64(2));
            blk = 237; continue;
        }
        237 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008329f7); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(12));
            c.w64(7, c.r64(8));
            c.w64(6, c.r64(9));
            return c.call(0x70007fe8d4, 0x7000832a08, 0x7000832a03);
        }
        238 => {
            blk = 86; continue;
        }
        239 => {
            let Some(t329) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000832a0d); };
            c.w64(8, t329);
            let Some(t330) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000832a14); };
            c.cmp(t330, 0x20u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r331 = c.r64(15) & 0x3ffu64;
            c.w64(15, r331);
            c.logic(r331, 64);
            let r332 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(8, r332);
            let Some(t333) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832a29); };
            c.w32(9, t333);
            let r334 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r334);
            let Some(t335) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000832a31); };
            c.w32(9, t335);
            let r336 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r336);
            let Some(t337) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000832a38); };
            c.w32(3, t337);
            let r338 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r338);
            let Some(t339) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000832a40); };
            c.w64(5, t339);
            c.w64(1, c.r64(12));
            let r340 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r340);
            return RegfileExit::jump(0x70007fea14);
        }
        241 => {
            let Some(t341) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000832a50); };
            c.w64(5, t341);
            let r342 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r342);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000832a58); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832a59); }
            let Some(t343) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000832a5a); };
            if !c.push64(t343) { return RegfileExit::fault(0x7000832a5a); }
            let Some(t344) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000832a5e); };
            c.w64(3, t344);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x7000832a6a); }
            return RegfileExit::jump(0x700086ed74);
        }
        242 => {
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r345 = c.r64(15) & 0x3ffu64;
            c.w64(15, r345);
            c.logic(r345, 64);
            let r346 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(8, r346);
            let Some(t347) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832a83); };
            c.w32(9, t347);
            let r348 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r348);
            let Some(t349) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000832a8b); };
            c.w32(9, t349);
            let r350 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r350);
            let Some(t351) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000832a92); };
            c.w32(8, t351);
            let r352 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r352);
            let Some(t353) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832a9a); };
            c.w32(7, t353);
            let r354 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r354);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832aa1); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000832aa9, 0x7000832aa4);
        }
        243 => {
            blk = 86; continue;
        }
        244 => {
            let Some(t355) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832aae); };
            c.w32(8, t355);
            let r356 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r356);
            let r357 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r357);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t358) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000832ac0); };
            c.w64(8, t358);
            blk = 246; continue;
        }
        246 => {
            let Some(t359) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832ac7); };
            c.w32(9, t359);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r360 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r360);
            let r361 = c.r64(11) & c.r64(9);
            c.w64(11, r361);
            c.logic(r361, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 247; continue;
        }
        247 => {
            c.w64(7, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(0, c.r64(7).wrapping_mul(4));
            let Some(t362) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832ae8); };
            c.w32(3, t362);
            let Some(t363) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000832aed); };
            c.cmp(t363, c.r32(3), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 248; continue;
        }
        248 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 249; continue;
        }
        249 => {
            let r364 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r364);
            let r365 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r365);
            c.w64(0, c.r64(9));
            let r366 = c.r64(0) & c.r64(11);
            c.w64(0, r366);
            c.logic(r366, 64);
            c.w64(11, c.r64(0));
            blk = 247; continue;
        }
        250 => {
            let Some(t367) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832b10); };
            c.w32(9, t367);
            let Some(t368) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832b14); };
            c.w32(11, t368);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            return RegfileExit::fault(0x7000832b28);
        }
        252 => {
            let Some(t369) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832b29); };
            c.w32(0, t369);
            let r370 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r370);
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 127; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t371) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832b3b); };
            c.w32(8, t371);
            let Some(t372) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000832b3f); };
            c.cmp(t372, c.r32(8), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t373) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832b4c); };
            c.w32(8, t373);
            let r374 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r374);
            let Some(t375) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832b53); };
            c.w32(9, t375);
            let r376 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r376);
            let Some(t377) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000832b5a); };
            c.logic(t377 & 0x2u64, 8);
            if c.cc_ne() { blk = 276; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t378) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000832b65); };
            c.cmp(t378, c.r32(9), 32);
            if c.cc_e() { blk = 257; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t379) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000832b6e); };
            c.w64(0, t379);
            blk = 86; continue;
        }
        257 => {
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000832b7a); };
            c.w64(9, t380);
            let Some(t381) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x7000832b81); };
            c.cmp(t381, 0x20u64, 8);
            if c.cc_e() { blk = 274; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t382) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000832b8b); };
            c.w32(9, t382);
            let Some(t383) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000832b8f); };
            c.cmp(t383, c.r32(9), 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832b98); };
            c.w64(0, t384);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832b9c); };
            c.w64(3, t385);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832ba3); }
            blk = 66; continue;
        }
        260 => {
            let Some(t386) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832ba9); };
            c.w32(9, t386);
            let r387 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r387);
            let Some(t388) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832bb1); };
            c.w32(11, t388);
            let Some(t389) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000832bb5); };
            c.cmp(t389, c.r32(11), 32);
            if c.cc_ne() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t390) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000832bbe); };
            c.w32(9, t390);
            let r391 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r391);
            let Some(t392) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832bc5); };
            c.w32(11, t392);
            let Some(t393) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000832bc9); };
            c.cmp(t393, c.r32(11), 32);
            if c.cc_e() { blk = 261; continue; }
            blk = 262; continue;
        }
        262 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 273; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t394) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832bdc); };
            c.w32(11, t394);
            let r395 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r395);
            let Some(t396) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832be3); };
            c.w32(15, t396);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 268; continue; }
            blk = 264; continue;
        }
        264 => {
            c.w32(9, c.r16(15));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 267; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t397) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000832c01); };
            c.cmp(t397, c.r32(11), 32);
            if c.cc_ne() { blk = 269; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832c0a); };
            c.w64(0, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832c0e); };
            c.w64(3, t399);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832c15); }
            blk = 66; continue;
        }
        267 => {
            let Some(t400) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832c1b); };
            c.w32(0, t400);
            let r401 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r401);
            c.w64(12, c.r64(0));
            blk = 269; continue;
        }
        268 => {
            c.w64(12, c.r64(9));
            blk = 269; continue;
        }
        269 => {
            let Some(t402) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832c2b); };
            c.w32(9, t402);
            let r403 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r403);
            let Some(t404) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832c33); };
            c.w32(9, t404);
            let r405 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r405);
            let Some(t406) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832c3a); };
            c.w32(11, t406);
            let r407 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r407);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 272; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t408) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000832c4c); };
            c.w32(6, t408);
            let r409 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r409);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832c53); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007f6fb4, 0x7000832c5e, 0x7000832c59);
        }
        271 => {
            blk = 86; continue;
        }
        272 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832c63); };
            c.w64(0, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832c67); };
            c.w64(3, t411);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832c6e); }
            blk = 66; continue;
        }
        273 => {
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832c74); };
            c.w64(0, t412);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832c78); };
            c.w64(3, t413);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832c7f); }
            blk = 66; continue;
        }
        274 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000832c85); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000832c87); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832c88); }
            let Some(t414) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000832c89); };
            c.w64(3, t414);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000832c9a, 0x7000832c95);
        }
        275 => {
            blk = 86; continue;
        }
        276 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000832c9f); }
            c.w64(7, c.r64(8));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000832caa, 0x7000832ca5);
        }
        277 => {
            blk = 86; continue;
        }
        278 => {
            let Some(t415) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832caf); };
            c.w32(8, t415);
            let r416 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r416);
            let Some(t417) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832cb7); };
            c.w32(9, t417);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 300; continue; }
            blk = 279; continue;
        }
        279 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 289; continue; }
            blk = 280; continue;
        }
        280 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 282; continue; }
            blk = 281; continue;
        }
        281 => {
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832ce1); };
            c.w64(0, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832ce5); };
            c.w64(3, t419);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832cec); }
            blk = 66; continue;
        }
        282 => {
            let Some(t420) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832cf2); };
            c.w32(8, t420);
            let Some(t421) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000832cf6); };
            c.cmp(t421, c.r32(8), 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832cff); };
            c.w64(0, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832d03); };
            c.w64(3, t423);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832d0a); }
            blk = 66; continue;
        }
        284 => {
            let Some(t424) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832d10); };
            c.w32(8, t424);
            let r425 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r425);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832d1e); };
            c.w64(0, t426);
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832d22); };
            c.w64(3, t427);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832d29); }
            blk = 66; continue;
        }
        286 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832d2f); };
            c.w32(9, t428);
            let r429 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r429);
            let Some(t430) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832d36); };
            c.cmp(t430, 0x40u64, 16);
            if c.cc_b() { blk = 288; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832d3e); };
            c.w64(0, t431);
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832d42); };
            c.w64(3, t432);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832d49); }
            blk = 66; continue;
        }
        288 => {
            let Some(t433) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832d4f); };
            c.w32(8, t433);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 86; continue;
        }
        289 => {
            let Some(t434) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832d5c); };
            c.w32(9, t434);
            let Some(t435) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000832d60); };
            c.cmp(t435, c.r32(9), 32);
            if c.cc_e() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832d69); };
            c.w64(0, t436);
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832d6d); };
            c.w64(3, t437);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832d74); }
            blk = 66; continue;
        }
        291 => {
            let Some(t438) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000832d7a); };
            c.w32(8, t438);
            c.w64(9, c.r64(8));
            let r439 = c.r32(9) & 0xc0u64;
            c.w32(9, r439);
            c.logic(r439, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 294; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t440) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832d92); };
            c.w32(9, t440);
            let r441 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r441);
            let Some(t442) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000832d9a); };
            c.w32(9, t442);
            let r443 = c.r32(9) & 0x1fu64;
            c.w32(9, r443);
            c.logic(r443, 32);
            let r444 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r444);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832dac); };
            c.w64(0, t445);
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832db0); };
            c.w64(3, t446);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832db7); }
            blk = 66; continue;
        }
        294 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 299; continue; }
            blk = 295; continue;
        }
        295 => {
            let Some(t447) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000832dc3); };
            c.w32(0, t447);
            let r448 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r448);
            let Some(t449) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000832dcb); };
            c.cmp(t449, c.r32(0), 32);
            if c.cc_e() { blk = 298; continue; }
            blk = 296; continue;
        }
        296 => {
            let Some(t450) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832dd4); };
            c.w32(8, t450);
            let Some(t451) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000832dd8); };
            c.cmp(t451, c.r32(8), 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 297; continue;
        }
        297 => {
            let Some(t452) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000832de5); };
            c.w32(0, t452);
            let r453 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r453);
            blk = 86; continue;
        }
        298 => {
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832df0); };
            c.w64(0, t454);
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832df4); };
            c.w64(3, t455);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832dfb); }
            blk = 66; continue;
        }
        299 => {
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832e01); };
            c.w64(0, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832e05); };
            c.w64(3, t457);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832e0c); }
            blk = 66; continue;
        }
        300 => {
            let Some(t458) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000832e12); };
            c.w32(8, t458);
            let Some(t459) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000832e16); };
            c.cmp(t459, c.r32(8), 32);
            if c.cc_e() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832e1f); };
            c.w64(0, t460);
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832e23); };
            c.w64(3, t461);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832e2a); }
            blk = 66; continue;
        }
        302 => {
            let Some(t462) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000832e30); };
            c.w32(0, t462);
            let r463 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r463);
            blk = 86; continue;
        }
        303 => {
            let Some(t464) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832e3d); };
            c.w64(0, t464);
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000832e41); };
            c.w64(3, t465);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832e48); }
            blk = 66; continue;
        }
        304 => {
            return RegfileExit::fault(0x7000832e4e);
        }
        305 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000832e4f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000832e53); }
            let r466 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r466);
            let Some(t467) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000832e59); };
            c.w64(9, t467);
            let Some(t468) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000832e60); };
            c.w64(11, t468);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000832e67); }
            let Some(t469) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000832e6b); };
            c.w64(15, t469);
            let Some(t470) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000832e72); };
            c.cmp(t470, c.r64(12), 64);
            if c.cc_a() { blk = 308; continue; }
            blk = 306; continue;
        }
        306 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000832e81, 0x7000832e7c);
        }
        307 => {
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832e81); };
            c.w64(8, t471);
            blk = 309; continue;
        }
        308 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000832e87); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 309; continue;
        }
        309 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t472) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000832e9a); };
            c.w64(8, t472);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000832ea1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000832ea5); }
            blk = 86; continue;
        }
        310 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000832eaf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000832eb3); }
            let r473 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r473);
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000832eb9); };
            c.w64(9, t474);
            let Some(t475) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000832ec0); };
            c.w64(11, t475);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000832ec7); }
            let Some(t476) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000832ecb); };
            c.w64(15, t476);
            let Some(t477) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000832ed2); };
            c.cmp(t477, c.r64(12), 64);
            if c.cc_a() { blk = 313; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000832ee1, 0x7000832edc);
        }
        312 => {
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832ee1); };
            c.w64(8, t478);
            blk = 314; continue;
        }
        313 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000832ee7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 314; continue;
        }
        314 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t479) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000832efd); };
            c.w64(8, t479);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000832f04); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000832f08); }
            blk = 86; continue;
        }
        315 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000832f12); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000832f16); }
            let r480 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r480);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000832f21); }
            return c.call(0x70007febb4, 0x7000832f2a, 0x7000832f25);
        }
        316 => {
            c.w64(9, c.r64(0));
            let Some(t481) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000832f2d); };
            c.w64(8, t481);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832f34); }
            blk = 54; continue;
        }
        317 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000832f3a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000832f3e); }
            let r482 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r482);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000832f4e, 0x7000832f49);
        }
        318 => {
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832f54); }
            blk = 59; continue;
        }
        319 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000832f5a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000832f5e); }
            let r483 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r483);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000832f69); }
            return c.call(0x70007febf4, 0x7000832f73, 0x7000832f6e);
        }
        320 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000832f76); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000832f7e); }
            blk = 64; continue;
        }
        321 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000832f84); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000832f88); }
            let r484 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r484);
            let Some(t485) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000832f8e); };
            c.w64(8, t485);
            let Some(t486) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000832f95); };
            c.w32(8, t486);
            let r487 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r487);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000832f9c); }
            let Some(t488) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000832f9e); };
            c.w64(3, t488);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000832fb1, 0x7000832fac);
        }
        322 => {
            return RegfileExit::fault(0x7000832fb1);
        }
        323 => {
            blk = 66; continue;
        }
        324 => {
            return RegfileExit::jump(0x7000832fb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000832fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(0).wrapping_add(c.r64(0)));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t0) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000832fe3); };
            c.w32(11, t0);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000832fee); };
            c.w32(12, t1);
            let r2 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000832ff7); };
            c.w64(12, t3);
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000832ffe); };
            c.cmp(t4, c.r32(11), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700083300b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700083300f); }
            let r5 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xc00)) else { return RegfileExit::fault(0x7000833015); };
            c.cmp(t6, c.r32(11), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833022); };
            c.cmp(t7, 0x422u64, 16);
            if c.cc_ne() { blk = 80; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000833030); };
            c.logic(t8 & 0x20u64, 8);
            if c.cc_ne() { blk = 84; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083303c); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xfb0)) else { return RegfileExit::fault(0x7000833044); };
            c.w64(9, t11);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083304e); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833058); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 9; continue;
        }
        9 => {
            let r15 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r15);
            c.logic(r15, 64);
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083306d); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 10; continue;
        }
        10 => {
            let r18 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r18);
            c.logic(r18, 64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833082); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            c.w64(6, c.r64(8));
            return c.call(0x70007f6e94, 0x7000833090, 0x700083308b);
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833093); }
            return c.ret(0x0, 0x7000833094);
        }
        12 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083309f); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            c.w64(9, c.r64(8));
            if let Some(sr) = c.shr(c.r64(9), 0xdu64, 64) { c.w64(9, sr); }
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008330ad); };
            c.w32(11, t21);
            let r22 = c.r64(9) ^ c.r64(8);
            c.w64(9, r22);
            c.logic(r22, 64);
            let r23 = c.add(c.r32(9), c.r32(11), 32);
            c.w32(9, r23);
            let r24 = c.r32(9) & 0x1ffcu64;
            c.w32(9, r24);
            c.logic(r24, 32);
            let r25 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(12, r25);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2f18)) else { return RegfileExit::fault(0x70008330c2); };
            c.w64(15, t26);
            let Some(t27) = c.ld32(c.r64(15).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008330c9); };
            c.cmp(t27, c.r32(1), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t28) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008330cf); };
            c.cmp(t28, c.r32(8), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t29) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008330d6); };
            c.w32(9, t29);
            let r30 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r30);
            blk = 19; continue;
        }
        16 => {
            c.w32(12, c.r32(1));
            c.w32(9, c.r32(9));
            let r31 = c.sub(c.r32(9), c.r32(12), 32);
            c.w32(9, r31);
            let r32 = c.sub(c.r32(9), 0x4e93591bu64, 32);
            c.w32(9, r32);
            let r33 = c.r32(9) & 0x7fcu64;
            c.w32(9, r33);
            c.logic(r33, 32);
            let r34 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2f30)) else { return RegfileExit::fault(0x70008330fb); };
            c.w64(12, t35);
            let Some(t36) = c.ld32(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000833102); };
            c.cmp(t36, c.r32(1), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t37) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083310c); };
            c.cmp(t37, c.r32(8), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000833117); };
            c.w32(9, t38);
            let r39 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r39);
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(12, c.r64(2));
            c.w64(15, c.r64(9));
            blk = 21; continue;
        }
        21 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(7, c.r64(15));
            let r40 = c.r64(7) & 0xfu64;
            c.w64(7, r40);
            c.logic(r40, 64);
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(7), 0x5u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r64(7), 0xdu64, 64);
            if c.cc_ne() { blk = 174; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000833152); };
            c.w64(0, t41);
            return c.ret(0x0, 0x7000833159);
        }
        25 => {
            c.w64(0, c.r64(12));
            return c.ret(0x0, 0x700083315d);
        }
        26 => {
            c.logic(c.r8(15) & 0x40u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(8, c.r64(15));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(15) & 0x100u64, 16);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let r42 = c.r64(8) & 0x7ffu64;
            c.w64(8, r42);
            c.logic(r42, 64);
            c.logic(c.r8(15) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083318a); };
            c.w32(11, t43);
            let r44 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r44);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000833198); };
            c.w64(11, t45);
            blk = 30; continue;
        }
        30 => {
            let Some(t46) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083319f); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70008331b0);
        }
        32 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t48) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008331bb); };
            c.w32(9, t48);
            let Some(t49) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008331bf); };
            c.cmp(t49, c.r32(9), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008331cc); }
            blk = 74; continue;
        }
        35 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008331dd); };
            c.w32(0, t50);
            let r51 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r51);
            return c.ret(0x0, 0x70008331e5);
        }
        37 => {
            let Some(t52) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008331e6); };
            c.w32(8, t52);
            let r53 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r53);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t54) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008331f8); };
            c.w32(9, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008331fc); };
            c.cmp(t55, c.r32(9), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000833209); }
            blk = 74; continue;
        }
        40 => {
            c.w64(8, c.r64(15));
            let r56 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r56);
            let r57 = c.r32(8) & 0xfu64;
            c.w32(8, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 49; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 46; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 338; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 321; continue;
        }
        46 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 321; continue;
        }
        49 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 321; continue;
        }
        54 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 321; continue;
        }
        57 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r58 = c.r64(15) & 0xfffffu64;
            c.w64(15, r58);
            c.logic(r58, 64);
            let Some(t59) = c.ld8(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008332a4); };
            c.w32(8, (t59 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x70008332ae);
        }
        58 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r60 = c.r64(15) & 0xfffffu64;
            c.w64(15, r60);
            c.logic(r60, 64);
            let Some(t61) = c.ld16(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008332ba); };
            c.w32(8, (t61 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x70008332c4);
        }
        59 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r62 = c.r64(15) & 0xfffffu64;
            c.w64(15, r62);
            c.logic(r62, 64);
            let Some(t63) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008332d0); };
            c.w32(8, t63);
            c.w64(9, c.r64(8));
            let r64 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r64);
            if c.cc_o() { blk = 322; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008332e4);
        }
        61 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r65 = c.r64(15) & 0xfffffu64;
            c.w64(15, r65);
            c.logic(r65, 64);
            let Some(t66) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008332f0); };
            c.w32(8, t66);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 327; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x7000833306);
        }
        63 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r67 = c.r64(15) & 0xfffffu64;
            c.w64(15, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld64(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833312); };
            c.w64(8, t68);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083331d); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000833324); };
            c.w64(11, t70);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083332b); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000833332); };
            c.cmp(t72, c.r64(12), 64);
            if c.cc_be() { blk = 332; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083333b); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 66; continue;
        }
        66 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000833342); };
            c.w64(11, t73);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000833349); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000833353); }
            let r74 = c.neg(c.r64(8), 64);
            c.w64(8, r74);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700083335e); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x7000833365);
        }
        68 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000833366); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700083336e); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x7000833375);
        }
        69 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000833376); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700083337d); };
            c.w64(9, t76);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000833384); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700083338b); };
            c.cmp(t78, c.r64(12), 64);
            if c.cc_be() { blk = 334; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000833394); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 71; continue;
        }
        71 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700083339b); };
            c.w64(9, t79);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008333a2); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008333a6); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70008333b1);
        }
        72 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r80 = c.r64(15) & 0xfffffu64;
            c.w64(15, r80);
            c.logic(r80, 64);
            if !c.movss_rm(0, c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008333bd); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 74; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r81 = c.r64(15) & 0xfffffu64;
            c.w64(15, r81);
            c.logic(r81, 64);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008333d5); }
            blk = 74; continue;
        }
        74 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008333dc); };
            c.w64(8, t82);
            let Some(t83) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008333e3); };
            c.w64(9, t83);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008333ea); };
            c.w64(12, t84);
            let Some(t85) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008333f1); };
            c.cmp(t85, c.r64(11), 64);
            if c.cc_be() { blk = 336; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008333fb); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000833402); };
            c.w64(9, t86);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000833409); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083340d); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x7000833416);
        }
        77 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r87 = c.r64(15) & 0xfffffu64;
            c.w64(15, r87);
            c.logic(r87, 64);
            let Some(t88) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833422); };
            c.w32(0, t88);
            let r89 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r89);
            return c.ret(0x0, 0x700083342a);
        }
        78 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083342b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083342c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083342d); }
            let Some(t90) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083342e); };
            if !c.push64(t90) { return RegfileExit::fault(0x700083342e); }
            c.w32(0, 0x4u64);
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2050)) else { return RegfileExit::fault(0x7000833437); };
            c.w64(3, t91);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x700083343e); }
            return RegfileExit::jump(0x700086ed74);
        }
        79 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083344b); }
            blk = 78; continue;
        }
        80 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833451); }
            blk = 78; continue;
        }
        81 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833457); }
            blk = 78; continue;
        }
        82 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083345d); }
            blk = 78; continue;
        }
        83 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833463); }
            blk = 78; continue;
        }
        84 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833469); }
            blk = 78; continue;
        }
        85 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700083346c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000833470); }
            let r92 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r92);
            let Some(t93) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833476); };
            c.w32(12, t93);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700083347a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700083347e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000833482); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000833486); }
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700083348a); };
            c.cmp(t94, c.r32(12), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833497); };
            c.w32(12, t95);
            let r96 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r96);
            c.w32(15, 0);
            c.logic(0, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008334a6); };
            c.cmp(t97, c.r32(15), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008334b0); }
            blk = 78; continue;
        }
        89 => {
            let Some(t98) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008334b6); };
            c.w32(12, t98);
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008334c0); }
            c.logic(c.r8(7) & 0x30u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 90; continue;
        }
        90 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 152; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(7, c.r64(11));
            let r99 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r99);
            let Some(t100) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008334de); };
            c.w32(0, t100);
            let r101 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r101);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008334e8); };
            c.w64(0, t102);
            blk = 93; continue;
        }
        93 => {
            let Some(t103) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008334ef); };
            c.w32(3, t103);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r104 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r104);
            let r105 = c.r64(7) & c.r64(3);
            c.w64(7, r105);
            c.logic(r105, 64);
            blk = 94; continue;
        }
        94 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(12, c.r64(11).wrapping_mul(4));
            let Some(t106) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083350c); };
            c.w32(6, t106);
            let Some(t107) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000833511); };
            c.cmp(t107, c.r32(6), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            let r108 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r108);
            c.w64(11, c.r64(15).wrapping_add(c.r64(7)));
            c.w64(12, c.r64(3));
            let r109 = c.r64(12) & c.r64(11);
            c.w64(12, r109);
            c.logic(r109, 64);
            c.w64(7, c.r64(12));
            blk = 94; continue;
        }
        97 => {
            let Some(t110) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833535); };
            c.w32(9, t110);
            let Some(t111) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833539); };
            c.w32(15, t111);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r112 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r112);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            return RegfileExit::fault(0x700083354d);
        }
        99 => {
            let Some(t113) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083354e); };
            c.w32(0, t113);
            let r114 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r114);
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t115) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833560); };
            c.w32(9, t115);
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000833564); };
            c.cmp(t116, c.r32(9), 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t117) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833571); };
            c.w32(9, t117);
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            let Some(t119) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833578); };
            c.w32(11, t119);
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            let Some(t121) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083357f); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_ne() { blk = 123; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t122) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083358a); };
            c.cmp(t122, c.r32(11), 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000833593); };
            c.w64(0, t123);
            blk = 11; continue;
        }
        104 => {
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083359f); };
            c.w64(11, t124);
            let Some(t125) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008335a6); };
            c.cmp(t125, 0x20u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t126) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008335b0); };
            c.w32(11, t126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008335b4); };
            c.cmp(t127, c.r32(11), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008335bd); };
            c.w64(0, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008335c1); };
            c.w64(3, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008335c5); };
            c.w64(6, t130);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008335cc); }
            blk = 78; continue;
        }
        107 => {
            let Some(t131) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008335d2); };
            c.w32(11, t131);
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008335d6); };
            c.cmp(t132, c.r32(11), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t133) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008335df); };
            c.w32(8, t133);
            let r134 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r134);
            let Some(t135) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008335e6); };
            c.w32(11, t135);
            let Some(t136) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008335ea); };
            c.cmp(t136, c.r32(11), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 109; continue;
        }
        109 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 120; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t137) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008335fd); };
            c.w32(11, t137);
            let r138 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r138);
            let Some(t139) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833604); };
            c.w32(12, t139);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 116; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w32(8, c.r16(12));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t140) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000833624); };
            c.cmp(t140, c.r32(11), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(8, c.r64(2));
            blk = 116; continue;
        }
        114 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833632); };
            c.w64(0, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833636); };
            c.w64(3, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083363a); };
            c.w64(6, t143);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833641); }
            blk = 78; continue;
        }
        115 => {
            let Some(t144) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833647); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            blk = 116; continue;
        }
        116 => {
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083364e); };
            c.w32(8, t146);
            let r147 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r147);
            let Some(t148) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833655); };
            c.w32(8, t148);
            let r149 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r149);
            let Some(t150) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083365c); };
            c.w32(11, t150);
            let r151 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r151);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 119; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083366e); };
            c.w32(6, t152);
            let r153 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r153);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833675); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x7000833680, 0x700083367b);
        }
        118 => {
            blk = 11; continue;
        }
        119 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833685); };
            c.w64(0, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833689); };
            c.w64(3, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083368d); };
            c.w64(6, t156);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833694); }
            blk = 78; continue;
        }
        120 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083369a); };
            c.w64(0, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083369e); };
            c.w64(3, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008336a2); };
            c.w64(6, t159);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008336a9); }
            blk = 78; continue;
        }
        121 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008336af); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008336b0); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008336b1); }
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008336b2); };
            c.w64(3, t160);
            c.w32(0, 0x3u64);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008336be); };
            c.w64(6, t161);
            return c.call(0x700086ed74, 0x70008336c7, 0x70008336c2);
        }
        122 => {
            blk = 11; continue;
        }
        123 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008336cc); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(9));
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008336d2); };
            c.w64(6, t162);
            return c.call(0x70007f2274, 0x70008336db, 0x70008336d6);
        }
        124 => {
            blk = 11; continue;
        }
        125 => {
            let Some(t163) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008336e0); };
            c.w32(9, t163);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 148; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 136; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083370a); };
            c.w64(0, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083370e); };
            c.w64(3, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000833712); };
            c.w64(6, t166);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833719); }
            blk = 78; continue;
        }
        129 => {
            let Some(t167) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083371f); };
            c.w32(8, t167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000833723); };
            c.cmp(t168, c.r32(8), 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083372c); };
            c.w64(0, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833730); };
            c.w64(3, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000833734); };
            c.w64(6, t171);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083373b); }
            blk = 78; continue;
        }
        131 => {
            let Some(t172) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833741); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083374e); };
            c.w64(0, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833752); };
            c.w64(3, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000833756); };
            c.w64(6, t176);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083375d); }
            blk = 78; continue;
        }
        133 => {
            let Some(t177) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833763); };
            c.w32(9, t177);
            let r178 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r178);
            let Some(t179) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083376a); };
            c.cmp(t179, 0x40u64, 16);
            if c.cc_b() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833772); };
            c.w64(0, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833776); };
            c.w64(3, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083377a); };
            c.w64(6, t182);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833781); }
            blk = 78; continue;
        }
        135 => {
            let Some(t183) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833787); };
            c.w32(8, t183);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 11; continue;
        }
        136 => {
            let Some(t184) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833794); };
            c.w32(9, t184);
            let Some(t185) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000833798); };
            c.cmp(t185, c.r32(9), 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008337a1); };
            c.w64(0, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008337a5); };
            c.w64(3, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008337a9); };
            c.w64(6, t188);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008337b0); }
            blk = 78; continue;
        }
        138 => {
            let Some(t189) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008337b6); };
            c.w32(8, t189);
            c.w64(9, c.r64(8));
            let r190 = c.r32(9) & 0xc0u64;
            c.w32(9, r190);
            c.logic(r190, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t191) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008337ce); };
            c.w32(9, t191);
            let r192 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r192);
            let Some(t193) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008337d5); };
            c.w32(9, t193);
            let r194 = c.r32(9) & 0x1fu64;
            c.w32(9, r194);
            c.logic(r194, 32);
            let r195 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r195);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008337e7); };
            c.w64(0, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008337eb); };
            c.w64(3, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008337ef); };
            c.w64(6, t198);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008337f6); }
            blk = 78; continue;
        }
        141 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 147; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t199) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000833802); };
            c.w32(0, t199);
            let r200 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r200);
            let Some(t201) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000833808); };
            c.cmp(t201, c.r32(0), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t202) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833811); };
            c.w32(8, t202);
            let Some(t203) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000833815); };
            c.cmp(t203, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 11; continue;
        }
        145 => {
            let Some(t204) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833823); };
            c.w32(0, t204);
            let r205 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r205);
            blk = 11; continue;
        }
        146 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083382e); };
            c.w64(0, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833832); };
            c.w64(3, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000833836); };
            c.w64(6, t208);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083383d); }
            blk = 78; continue;
        }
        147 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833843); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833847); };
            c.w64(3, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083384b); };
            c.w64(6, t211);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833852); }
            blk = 78; continue;
        }
        148 => {
            let Some(t212) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833858); };
            c.w32(8, t212);
            let Some(t213) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083385c); };
            c.cmp(t213, c.r32(8), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833865); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833869); };
            c.w64(3, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083386d); };
            c.w64(6, t216);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833874); }
            blk = 78; continue;
        }
        150 => {
            let Some(t217) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083387a); };
            c.w32(0, t217);
            let r218 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r218);
            blk = 11; continue;
        }
        151 => {
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833885); };
            c.w64(3, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000833889); };
            c.w64(6, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083388d); };
            c.w64(11, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000833891); };
            c.w64(12, t222);
            blk = 163; continue;
        }
        152 => {
            let Some(t223) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083389a); };
            c.w32(15, t223);
            let r224 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r224);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 153; continue;
        }
        153 => {
            let r225 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r225);
            c.logic(r225, 64);
            let Some(t226) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008338ab); };
            c.w32(7, t226);
            let r227 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r227);
            let Some(t228) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008338b1); };
            c.w32(7, t228);
            let r229 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r229);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t230) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008338bc); };
            c.cmp(t230, 0xbeu64, 16);
            if c.cc_e() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008338c8); }
            blk = 78; continue;
        }
        156 => {
            let Some(t231) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008338ce); };
            c.w32(0, t231);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xa9u64, 64);
            if c.cc_a() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            return RegfileExit::fault(0x70008338e0);
        }
        158 => {
            let Some(t232) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008338e1); };
            c.w32(0, t232);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t233) = c.ld32(c.r64(15).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70008338e7); };
            c.w32(15, t233);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xa9u64, 64);
            if c.cc_a() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            return RegfileExit::fault(0x70008338f9);
        }
        160 => {
            let Some(t234) = c.ld32(c.r64(7).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70008338fa); };
            c.cmp(t234, c.r32(15), 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833903); };
            c.w64(0, t235);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083390a); }
            blk = 78; continue;
        }
        162 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833913); }
            blk = 78; continue;
        }
        163 => {
            let Some(t236) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833919); };
            c.w32(0, t236);
            let r237 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r237);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t238) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000833928); };
            c.cmp(t238, c.r32(0), 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let r239 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r239);
            c.logic(r239, 64);
            c.w64(15, c.r64(0));
            blk = 169; continue;
        }
        167 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083393f); };
            c.w64(0, t240);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833946); }
            blk = 78; continue;
        }
        168 => {
            c.w64(15, c.r64(2));
            blk = 169; continue;
        }
        169 => {
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(15));
            c.w64(15, c.r64(10));
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833958); };
            c.w64(0, t241);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083395f); }
            blk = 21; continue;
        }
        170 => {
            let Some(t242) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000833965); };
            c.w64(5, t242);
            let r243 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r243);
            let Some(t244) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083396d); };
            c.logic(t244 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            let r245 = c.add(c.r64(9), 0x3fu64, 64);
            c.w64(9, r245);
            blk = 173; continue;
        }
        172 => {
            let Some(t246) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083397d); };
            c.w32(9, t246);
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000833981); };
            c.w64(9, t247);
            blk = 173; continue;
        }
        173 => {
            return RegfileExit::jump(c.r64(9));
        }
        174 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700083398c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000833990); }
            let r248 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r248);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000833996); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700083399a); }
            c.cmp(c.r64(7), 0x2u64, 64);
            if c.cc_e() { blk = 261; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r64(7), 0x6u64, 64);
            if c.cc_e() { blk = 259; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 256; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r64(7), 0x8u64, 64);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 245; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r64(7), 0x9u64, 64);
            if c.cc_e() { blk = 245; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r64(7), 0x3u64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 180; continue;
        }
        180 => {
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 258; continue; }
            blk = 181; continue;
        }
        181 => {
            c.cmp(c.r64(7), 0xcu64, 64);
            if c.cc_e() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            c.cmp(c.r64(7), 0xeu64, 64);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t249) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000833a00); };
            c.w64(5, t249);
            let r250 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r250);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833a08); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000833a09); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000833a0b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000833a0c); }
            let Some(t251) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000833a0d); };
            if !c.push64(t251) { return RegfileExit::fault(0x7000833a0d); }
            c.w32(0, 0x5u64);
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000833a16); };
            c.w64(3, t252);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x7000833a1d); }
            return RegfileExit::jump(0x700086ed74);
        }
        184 => {
            let Some(t253) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833a27); };
            c.w32(8, t253);
            let r254 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r254);
            let Some(t255) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833a2f); };
            c.w32(8, t255);
            let r256 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r256);
            let Some(t257) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833a36); };
            c.w32(9, t257);
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r258 = c.r64(15) & 0x1ffffffu64;
            c.w64(15, r258);
            c.logic(r258, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            return RegfileExit::fault(0x7000833a50);
        }
        186 => {
            let Some(t259) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833a51); };
            c.w32(8, t259);
            let r260 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r260);
            let Some(t261) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833a59); };
            c.w32(0, t261);
            let r262 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r262);
            let Some(t263) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000833a60); };
            c.cmp(t263, c.r32(0), 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t264) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000833a6d); };
            c.w64(5, t264);
            let r265 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r265);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000833a75); }
            let Some(t266) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000833a76); };
            if !c.push64(t266) { return RegfileExit::fault(0x7000833a76); }
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000833a7a); };
            c.w64(3, t267);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x7000833a86); }
            return RegfileExit::jump(0x700086ed74);
        }
        188 => {
            let Some(t268) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000833a94); };
            c.w64(5, t268);
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            c.w64(0, c.r64(12));
            let r269 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r269);
            return RegfileExit::jump(0x70008bc174);
        }
        189 => {
            blk = 258; continue;
        }
        190 => {
            let Some(t270) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833ab4); };
            c.w32(8, t270);
            let r271 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r271);
            let Some(t272) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833abc); };
            c.w32(9, t272);
            let Some(t273) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000833ac1); };
            c.cmp(t273, c.r32(8), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 191; continue;
        }
        191 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t274) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833ad8); };
            c.w32(9, t274);
            let r275 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r275);
            let Some(t276) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000833adf); };
            c.cmp(t276, c.r32(9), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t277) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833aec); };
            c.w32(9, t277);
            let r278 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r278);
            let Some(t279) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833af3); };
            c.w32(11, t279);
            let r280 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r280);
            let Some(t281) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000833afa); };
            c.logic(t281 & 0x2u64, 8);
            if c.cc_ne() { blk = 215; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t282) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000833b05); };
            c.cmp(t282, c.r32(11), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000833b0e); };
            c.w64(0, t283);
            blk = 11; continue;
        }
        196 => {
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000833b1a); };
            c.w64(11, t284);
            let Some(t285) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000833b21); };
            c.cmp(t285, 0x20u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t286) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000833b2b); };
            c.w32(11, t286);
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000833b2f); };
            c.cmp(t287, c.r32(11), 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833b3b); }
            blk = 78; continue;
        }
        199 => {
            let Some(t288) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833b41); };
            c.w32(11, t288);
            let r289 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r289);
            let Some(t290) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833b49); };
            c.w32(15, t290);
            let Some(t291) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000833b4d); };
            c.cmp(t291, c.r32(15), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t292) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000833b56); };
            c.w32(11, t292);
            let r293 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r293);
            let Some(t294) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833b5d); };
            c.w32(15, t294);
            let Some(t295) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000833b61); };
            c.cmp(t295, c.r32(15), 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 201; continue;
        }
        201 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t296) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833b74); };
            c.w32(15, t296);
            let r297 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r297);
            let Some(t298) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833b7b); };
            c.w32(7, t298);
            c.w32(0, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 207; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w32(11, c.r16(7));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t299) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000833b98); };
            c.cmp(t299, c.r32(15), 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833ba1); };
            c.w64(0, t300);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833ba8); }
            blk = 78; continue;
        }
        206 => {
            c.w64(12, c.r64(8));
            blk = 208; continue;
        }
        207 => {
            c.w64(12, c.r64(11));
            blk = 208; continue;
        }
        208 => {
            let Some(t301) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833bb6); };
            c.w32(8, t301);
            let r302 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r302);
            let Some(t303) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833bbe); };
            c.w32(8, t303);
            let r304 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r304);
            let Some(t305) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833bc5); };
            c.w32(11, t305);
            let r306 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r306);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 211; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t307) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000833bd7); };
            c.w32(6, t307);
            let r308 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r308);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833bde); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x7000833be9, 0x7000833be4);
        }
        210 => {
            blk = 11; continue;
        }
        211 => {
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833bee); };
            c.w64(0, t309);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833bf5); }
            blk = 78; continue;
        }
        212 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833bfe); }
            blk = 78; continue;
        }
        213 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000833c04); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000833c06); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833c07); }
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000833c08); };
            c.w64(3, t310);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000833c19, 0x7000833c14);
        }
        214 => {
            blk = 11; continue;
        }
        215 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833c1e); }
            c.w64(7, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000833c29, 0x7000833c24);
        }
        216 => {
            blk = 11; continue;
        }
        217 => {
            let Some(t311) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833c2e); };
            c.w32(11, t311);
            let r312 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r312);
            let Some(t313) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833c36); };
            c.w32(15, t313);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 240; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w32(7, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 228; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_e() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833c5e); }
            blk = 78; continue;
        }
        221 => {
            let Some(t314) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833c64); };
            c.w32(11, t314);
            let Some(t315) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000833c68); };
            c.cmp(t315, c.r32(11), 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833c74); }
            blk = 78; continue;
        }
        223 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833c83); }
            blk = 78; continue;
        }
        225 => {
            let Some(t316) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833c89); };
            c.cmp(t316, 0x40u64, 16);
            if c.cc_b() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833c94); }
            blk = 78; continue;
        }
        227 => {
            let Some(t317) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833c9a); };
            c.w32(8, t317);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 11; continue;
        }
        228 => {
            let Some(t318) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833ca7); };
            c.w32(9, t318);
            let Some(t319) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000833cab); };
            c.cmp(t319, c.r32(9), 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833cb7); }
            blk = 78; continue;
        }
        230 => {
            let Some(t320) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000833cbd); };
            c.w32(9, t320);
            c.w64(11, c.r64(9));
            let r321 = c.r32(11) & 0xc0u64;
            c.w32(11, r321);
            c.logic(r321, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t322) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000833cd5); };
            c.w32(8, t322);
            let r323 = c.r32(8) & 0x1fu64;
            c.w32(8, r323);
            c.logic(r323, 32);
            let r324 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r324);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_be() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833cea); }
            blk = 78; continue;
        }
        233 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t325) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000833cf6); };
            c.w32(8, t325);
            let r326 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r326);
            let Some(t327) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000833cfe); };
            c.cmp(t327, c.r32(8), 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t328) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833d07); };
            c.w32(9, t328);
            let Some(t329) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000833d0b); };
            c.cmp(t329, c.r32(9), 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w64(0, c.r64(8));
            blk = 11; continue;
        }
        237 => {
            let Some(t330) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833d1c); };
            c.w32(0, t330);
            let r331 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r331);
            blk = 11; continue;
        }
        238 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833d2b); }
            blk = 78; continue;
        }
        239 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833d34); }
            blk = 78; continue;
        }
        240 => {
            let Some(t332) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833d3a); };
            c.w32(9, t332);
            let Some(t333) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000833d3e); };
            c.cmp(t333, c.r32(9), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833d4a); }
            blk = 78; continue;
        }
        242 => {
            c.w64(0, c.r64(8));
            blk = 11; continue;
        }
        243 => {
            c.w64(0, c.r64(8));
            blk = 11; continue;
        }
        244 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833d63); }
            blk = 78; continue;
        }
        245 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000833d69); };
            c.w64(0, t334);
            let Some(t335) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x7000833d70); };
            c.cmp(t335, 0x20u64, 8);
            if c.cc_e() { blk = 258; continue; }
            blk = 246; continue;
        }
        246 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t336) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833d7f); };
            c.w32(9, t336);
            let r337 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r337);
            blk = 249; continue;
        }
        248 => {
            let Some(t338) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000833d88); };
            c.w32(9, t338);
            let r339 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r339);
            blk = 249; continue;
        }
        249 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            return RegfileExit::fault(0x7000833d95);
        }
        251 => {
            let r340 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r340);
            c.logic(r340, 64);
            let Some(t341) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833d9a); };
            c.w32(15, t341);
            let r342 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r342);
            let Some(t343) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833da2); };
            c.w32(3, t343);
            let r344 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r344);
            let Some(t345) = c.ld64(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833daa); };
            c.w64(12, t345);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t346) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833db4); };
            c.w32(8, t346);
            let r347 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r347);
            blk = 254; continue;
        }
        253 => {
            c.w64(8, c.r64(2));
            blk = 254; continue;
        }
        254 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833dc0); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(12));
            c.w64(7, c.r64(8));
            c.w64(6, c.r64(9));
            return c.call(0x70007fe8d4, 0x7000833dd1, 0x7000833dcc);
        }
        255 => {
            blk = 11; continue;
        }
        256 => {
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000833dd6); };
            c.w64(8, t348);
            let Some(t349) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000833ddd); };
            c.cmp(t349, 0x20u64, 8);
            if c.cc_e() { blk = 258; continue; }
            blk = 257; continue;
        }
        257 => {
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r350 = c.r64(15) & 0x3ffu64;
            c.w64(15, r350);
            c.logic(r350, 64);
            let r351 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(8, r351);
            let Some(t352) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833df2); };
            c.w32(9, t352);
            let r353 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r353);
            let Some(t354) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000833dfa); };
            c.w32(9, t354);
            let r355 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r355);
            let Some(t356) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000833e01); };
            c.w32(3, t356);
            let r357 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r357);
            let Some(t358) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000833e09); };
            c.w64(5, t358);
            c.w64(1, c.r64(12));
            let r359 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r359);
            return RegfileExit::jump(0x70007fea14);
        }
        258 => {
            let Some(t360) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000833e19); };
            c.w64(5, t360);
            let r361 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r361);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000833e21); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833e22); }
            let Some(t362) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000833e23); };
            if !c.push64(t362) { return RegfileExit::fault(0x7000833e23); }
            let Some(t363) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000833e27); };
            c.w64(3, t363);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x7000833e33); }
            return RegfileExit::jump(0x700086ed74);
        }
        259 => {
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r364 = c.r64(15) & 0x3ffu64;
            c.w64(15, r364);
            c.logic(r364, 64);
            let r365 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(8, r365);
            let Some(t366) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833e4c); };
            c.w32(9, t366);
            let r367 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r367);
            let Some(t368) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000833e54); };
            c.w32(9, t368);
            let r369 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r369);
            let Some(t370) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000833e5b); };
            c.w32(8, t370);
            let r371 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r371);
            let Some(t372) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833e63); };
            c.w32(7, t372);
            let r373 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r373);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000833e6a); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000833e72, 0x7000833e6d);
        }
        260 => {
            blk = 11; continue;
        }
        261 => {
            let Some(t374) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833e77); };
            c.w32(8, t374);
            let r375 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r375);
            let r376 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r376);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t377) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000833e89); };
            c.w64(8, t377);
            blk = 263; continue;
        }
        263 => {
            let Some(t378) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833e90); };
            c.w32(9, t378);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r379 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r379);
            let r380 = c.r64(11) & c.r64(9);
            c.w64(11, r380);
            c.logic(r380, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 264; continue;
        }
        264 => {
            c.w64(7, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(0, c.r64(7).wrapping_mul(4));
            let Some(t381) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833eb1); };
            c.w32(3, t381);
            let Some(t382) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000833eb6); };
            c.cmp(t382, c.r32(3), 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 265; continue;
        }
        265 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 267; continue; }
            blk = 266; continue;
        }
        266 => {
            let r383 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r383);
            let r384 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r384);
            c.w64(0, c.r64(9));
            let r385 = c.r64(0) & c.r64(11);
            c.w64(0, r385);
            c.logic(r385, 64);
            c.w64(11, c.r64(0));
            blk = 264; continue;
        }
        267 => {
            let Some(t386) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833ed9); };
            c.w32(9, t386);
            let Some(t387) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833edd); };
            c.w32(11, t387);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            return RegfileExit::fault(0x7000833ef1);
        }
        269 => {
            let Some(t388) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833ef2); };
            c.w32(0, t388);
            let r389 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r389);
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t390) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833f04); };
            c.w32(8, t390);
            let Some(t391) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000833f08); };
            c.cmp(t391, c.r32(8), 32);
            if c.cc_ne() { blk = 295; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t392) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000833f15); };
            c.w32(8, t392);
            let r393 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r393);
            let Some(t394) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833f1c); };
            c.w32(9, t394);
            let r395 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r395);
            let Some(t396) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000833f23); };
            c.logic(t396 & 0x2u64, 8);
            if c.cc_ne() { blk = 293; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t397) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000833f2e); };
            c.cmp(t397, c.r32(9), 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t398) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000833f37); };
            c.w64(0, t398);
            blk = 11; continue;
        }
        274 => {
            let Some(t399) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000833f43); };
            c.w64(9, t399);
            let Some(t400) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x7000833f4a); };
            c.cmp(t400, 0x20u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t401) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000833f54); };
            c.w32(9, t401);
            let Some(t402) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000833f58); };
            c.cmp(t402, c.r32(9), 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833f61); };
            c.w64(0, t403);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833f65); };
            c.w64(3, t404);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833f6c); }
            blk = 78; continue;
        }
        277 => {
            let Some(t405) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833f72); };
            c.w32(9, t405);
            let r406 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r406);
            let Some(t407) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833f7a); };
            c.w32(11, t407);
            let Some(t408) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000833f7e); };
            c.cmp(t408, c.r32(11), 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t409) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000833f87); };
            c.w32(9, t409);
            let r410 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r410);
            let Some(t411) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833f8e); };
            c.w32(11, t411);
            let Some(t412) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000833f92); };
            c.cmp(t412, c.r32(11), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 279; continue;
        }
        279 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 290; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t413) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833fa5); };
            c.w32(11, t413);
            let r414 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r414);
            let Some(t415) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000833fac); };
            c.w32(15, t415);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 285; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w32(9, c.r16(15));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t416) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000833fca); };
            c.cmp(t416, c.r32(11), 32);
            if c.cc_ne() { blk = 286; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000833fd3); };
            c.w64(0, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000833fd7); };
            c.w64(3, t418);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000833fde); }
            blk = 78; continue;
        }
        284 => {
            let Some(t419) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000833fe4); };
            c.w32(0, t419);
            let r420 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r420);
            c.w64(12, c.r64(0));
            blk = 286; continue;
        }
        285 => {
            c.w64(12, c.r64(9));
            blk = 286; continue;
        }
        286 => {
            let Some(t421) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000833ff4); };
            c.w32(9, t421);
            let r422 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r422);
            let Some(t423) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000833ffc); };
            c.w32(9, t423);
            let r424 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r424);
            let Some(t425) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834003); };
            c.w32(11, t425);
            let r426 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r426);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 289; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t427) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000834015); };
            c.w32(6, t427);
            let r428 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r428);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083401c); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007f6fb4, 0x7000834027, 0x7000834022);
        }
        288 => {
            blk = 11; continue;
        }
        289 => {
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083402c); };
            c.w64(0, t429);
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000834030); };
            c.w64(3, t430);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000834037); }
            blk = 78; continue;
        }
        290 => {
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083403d); };
            c.w64(0, t431);
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000834041); };
            c.w64(3, t432);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000834048); }
            blk = 78; continue;
        }
        291 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700083404e); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000834050); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000834051); }
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000834052); };
            c.w64(3, t433);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000834063, 0x700083405e);
        }
        292 => {
            blk = 11; continue;
        }
        293 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000834068); }
            c.w64(7, c.r64(8));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000834073, 0x700083406e);
        }
        294 => {
            blk = 11; continue;
        }
        295 => {
            let Some(t434) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834078); };
            c.w32(8, t434);
            let r435 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r435);
            let Some(t436) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834080); };
            c.w32(9, t436);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 317; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 306; continue; }
            blk = 297; continue;
        }
        297 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008340aa); };
            c.w64(0, t437);
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008340ae); };
            c.w64(3, t438);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008340b5); }
            blk = 78; continue;
        }
        299 => {
            let Some(t439) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008340bb); };
            c.w32(8, t439);
            let Some(t440) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008340bf); };
            c.cmp(t440, c.r32(8), 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008340c8); };
            c.w64(0, t441);
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008340cc); };
            c.w64(3, t442);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008340d3); }
            blk = 78; continue;
        }
        301 => {
            let Some(t443) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008340d9); };
            c.w32(8, t443);
            let r444 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r444);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008340e7); };
            c.w64(0, t445);
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008340eb); };
            c.w64(3, t446);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008340f2); }
            blk = 78; continue;
        }
        303 => {
            let Some(t447) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008340f8); };
            c.w32(9, t447);
            let r448 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r448);
            let Some(t449) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008340ff); };
            c.cmp(t449, 0x40u64, 16);
            if c.cc_b() { blk = 305; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000834107); };
            c.w64(0, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083410b); };
            c.w64(3, t451);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000834112); }
            blk = 78; continue;
        }
        305 => {
            let Some(t452) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834118); };
            c.w32(8, t452);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 11; continue;
        }
        306 => {
            let Some(t453) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834125); };
            c.w32(9, t453);
            let Some(t454) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000834129); };
            c.cmp(t454, c.r32(9), 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000834132); };
            c.w64(0, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000834136); };
            c.w64(3, t456);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083413d); }
            blk = 78; continue;
        }
        308 => {
            let Some(t457) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000834143); };
            c.w32(8, t457);
            c.w64(9, c.r64(8));
            let r458 = c.r32(9) & 0xc0u64;
            c.w32(9, r458);
            c.logic(r458, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 311; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t459) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083415b); };
            c.w32(9, t459);
            let r460 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r460);
            let Some(t461) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000834163); };
            c.w32(9, t461);
            let r462 = c.r32(9) & 0x1fu64;
            c.w32(9, r462);
            c.logic(r462, 32);
            let r463 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r463);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 311; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t464) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000834175); };
            c.w64(0, t464);
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000834179); };
            c.w64(3, t465);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000834180); }
            blk = 78; continue;
        }
        311 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 316; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t466) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083418c); };
            c.w32(0, t466);
            let r467 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r467);
            let Some(t468) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000834194); };
            c.cmp(t468, c.r32(0), 32);
            if c.cc_e() { blk = 315; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t469) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083419d); };
            c.w32(8, t469);
            let Some(t470) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008341a1); };
            c.cmp(t470, c.r32(8), 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t471) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008341ae); };
            c.w32(0, t471);
            let r472 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r472);
            blk = 11; continue;
        }
        315 => {
            let Some(t473) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008341b9); };
            c.w64(0, t473);
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008341bd); };
            c.w64(3, t474);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008341c4); }
            blk = 78; continue;
        }
        316 => {
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008341ca); };
            c.w64(0, t475);
            let Some(t476) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008341ce); };
            c.w64(3, t476);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008341d5); }
            blk = 78; continue;
        }
        317 => {
            let Some(t477) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008341db); };
            c.w32(8, t477);
            let Some(t478) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008341df); };
            c.cmp(t478, c.r32(8), 32);
            if c.cc_e() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t479) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008341e8); };
            c.w64(0, t479);
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008341ec); };
            c.w64(3, t480);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008341f3); }
            blk = 78; continue;
        }
        319 => {
            let Some(t481) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008341f9); };
            c.w32(0, t481);
            let r482 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r482);
            blk = 11; continue;
        }
        320 => {
            let Some(t483) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000834206); };
            c.w64(0, t483);
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083420a); };
            c.w64(3, t484);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000834211); }
            blk = 78; continue;
        }
        321 => {
            return RegfileExit::fault(0x7000834217);
        }
        322 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000834218); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700083421c); }
            let r485 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r485);
            let Some(t486) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000834222); };
            c.w64(9, t486);
            let Some(t487) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000834229); };
            c.w64(11, t487);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000834230); }
            let Some(t488) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000834234); };
            c.w64(15, t488);
            let Some(t489) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083423b); };
            c.cmp(t489, c.r64(12), 64);
            if c.cc_a() { blk = 325; continue; }
            blk = 323; continue;
        }
        323 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700083424a, 0x7000834245);
        }
        324 => {
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083424a); };
            c.w64(8, t490);
            blk = 326; continue;
        }
        325 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000834250); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 326; continue;
        }
        326 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t491) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000834263); };
            c.w64(8, t491);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083426a); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083426e); }
            blk = 11; continue;
        }
        327 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000834278); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700083427c); }
            let r492 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r492);
            let Some(t493) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000834282); };
            c.w64(9, t493);
            let Some(t494) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000834289); };
            c.w64(11, t494);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000834290); }
            let Some(t495) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000834294); };
            c.w64(15, t495);
            let Some(t496) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083429b); };
            c.cmp(t496, c.r64(12), 64);
            if c.cc_a() { blk = 330; continue; }
            blk = 328; continue;
        }
        328 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008342aa, 0x70008342a5);
        }
        329 => {
            let Some(t497) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008342aa); };
            c.w64(8, t497);
            blk = 331; continue;
        }
        330 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008342b0); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 331; continue;
        }
        331 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t498) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008342c6); };
            c.w64(8, t498);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008342cd); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008342d1); }
            blk = 11; continue;
        }
        332 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008342db); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008342df); }
            let r499 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r499);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008342ea); }
            return c.call(0x70007febb4, 0x70008342f3, 0x70008342ee);
        }
        333 => {
            c.w64(9, c.r64(0));
            let Some(t500) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008342f6); };
            c.w64(8, t500);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008342fd); }
            blk = 66; continue;
        }
        334 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000834303); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000834307); }
            let r501 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r501);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000834317, 0x7000834312);
        }
        335 => {
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083431d); }
            blk = 71; continue;
        }
        336 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000834323); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000834327); }
            let r502 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r502);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000834332); }
            return c.call(0x70007febf4, 0x700083433c, 0x7000834337);
        }
        337 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700083433f); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000834347); }
            blk = 76; continue;
        }
        338 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700083434d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000834351); }
            let r503 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r503);
            let Some(t504) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000834357); };
            c.w64(8, t504);
            let Some(t505) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083435e); };
            c.w32(8, t505);
            let r506 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r506);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000834365); }
            let Some(t507) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000834367); };
            c.w64(3, t507);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700083437a, 0x7000834375);
        }
        339 => {
            return RegfileExit::fault(0x700083437a);
        }
        340 => {
            blk = 78; continue;
        }
        341 => {
            return RegfileExit::jump(0x7000834384);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000834394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000834397); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083439b); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008343a1); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008343a7); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008343ab); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008343b4); };
            c.w64(7, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(7));
            return RegfileExit::jump(0x70008308f4);
        }
        3 => {
            return RegfileExit::jump(0x70008343c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008343d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008343d7); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008343db); };
            c.w64(7, t1);
            c.w64(6, c.r64(7));
            c.w64(3, c.r64(8));
            return RegfileExit::jump(0x70008308f4);
        }
        1 => {
            return RegfileExit::jump(0x70008343ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008343f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008343f7); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008343fb); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834401); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834407); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700083440b); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000834414); };
            c.w64(7, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(7));
            return RegfileExit::jump(0x7000831cf4);
        }
        3 => {
            return RegfileExit::jump(0x7000834424);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000834434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000834434); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000834438); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700083443e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000834442); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000834446); }
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083444a); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834469); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000834472); };
            c.w64(9, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834479); };
            c.logic(t5 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(12, c.r32(9));
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083448e); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let r8 = c.r32(12) | 0x2u64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(12, c.r32(15));
            let r9 = c.r32(12) & 0x3u64;
            c.w32(12, r9);
            c.logic(r9, 32);
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008344b0); };
            c.w32(12, t10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x70008344b4); };
            c.cmp(t11, c.r32(12), 32);
            if c.cc_ne() { blk = 268; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008344c1); };
            c.w32(8, t12);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r13 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r13);
            blk = 9; continue;
        }
        9 => {
            c.w64(11, c.r64(8).wrapping_mul(4));
            c.w32(12, c.r32(9));
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008344da); };
            c.w32(2, t14);
            let r15 = c.r32(12) | 0x2u64;
            c.w32(12, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let r16 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r16);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ge() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 74; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008344f7); };
            c.w32(8, t17);
            let r18 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r18);
            blk = 14; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000834501); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 284; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(9, c.r64(7));
            c.w64(11, c.r64(8));
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11));
            let r21 = c.r64(12) & 0xfu64;
            c.w64(12, r21);
            c.logic(r21, 64);
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(12), 0x5u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(12), 0xdu64, 64);
            if c.cc_ne() { blk = 369; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083453c); };
            c.w64(0, t22);
            blk = 20; continue;
        }
        20 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000834546); }
            return c.ret(0x0, 0x7000834547);
        }
        21 => {
            c.w64(0, c.r64(9));
            blk = 20; continue;
        }
        22 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_ne() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let r23 = c.r64(8) & 0x7ffu64;
            c.w64(8, r23);
            c.logic(r23, 64);
            c.logic(c.r8(11) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834579); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000834586); };
            c.w64(9, t26);
            blk = 26; continue;
        }
        26 => {
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083458d); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        28 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008345aa); };
            c.w32(9, t29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008345ae); };
            c.cmp(t30, c.r32(9), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008345bb); }
            blk = 70; continue;
        }
        31 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008345cc); };
            c.w32(0, t31);
            let r32 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r32);
            blk = 20; continue;
        }
        33 => {
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008345d9); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008345eb); };
            c.w32(9, t35);
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008345ef); };
            c.cmp(t36, c.r32(9), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008345fc); }
            blk = 70; continue;
        }
        36 => {
            c.w64(8, c.r64(11));
            let r37 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r37);
            let r38 = c.r32(8) & 0xfu64;
            c.w32(8, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 522; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 505; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 505; continue;
        }
        45 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 50; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 505; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 505; continue;
        }
        53 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r39 = c.r64(11) & 0xfffffu64;
            c.w64(11, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083469b); };
            c.w32(8, (t40 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        54 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r41 = c.r64(11) & 0xfffffu64;
            c.w64(11, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld16(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008346b5); };
            c.w32(8, (t42 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        55 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r43 = c.r64(11) & 0xfffffu64;
            c.w64(11, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008346cf); };
            c.w32(8, t44);
            c.w64(9, c.r64(8));
            let r45 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r45);
            if c.cc_o() { blk = 506; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        57 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r46 = c.r64(11) & 0xfffffu64;
            c.w64(11, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008346f3); };
            c.w32(8, t47);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 511; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        59 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r48 = c.r64(11) & 0xfffffu64;
            c.w64(11, r48);
            c.logic(r48, 64);
            let Some(t49) = c.ld64(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834719); };
            c.w64(8, t49);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000834724); };
            c.w64(9, t50);
            let Some(t51) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083472b); };
            c.w64(11, t51);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000834732); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000834739); };
            c.cmp(t53, c.r64(12), 64);
            if c.cc_be() { blk = 516; continue; }
            blk = 61; continue;
        }
        61 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000834742); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 62; continue;
        }
        62 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000834749); };
            c.w64(11, t54);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000834750); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700083475a); }
            let r55 = c.neg(c.r64(8), 64);
            c.w64(8, r55);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000834765); }
            c.w64(0, c.r64(9));
            blk = 20; continue;
        }
        64 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000834771); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000834779); }
            c.w64(0, c.r64(9));
            blk = 20; continue;
        }
        65 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000834785); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700083478c); };
            c.w64(9, t57);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000834793); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700083479a); };
            c.cmp(t59, c.r64(12), 64);
            if c.cc_be() { blk = 518; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008347a3); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 67; continue;
        }
        67 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008347aa); };
            c.w64(9, t60);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008347b1); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008347b5); }
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        68 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r61 = c.r64(11) & 0xfffffu64;
            c.w64(11, r61);
            c.logic(r61, 64);
            if !c.movss_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008347d0); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 70; continue;
        }
        69 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r62 = c.r64(11) & 0xfffffu64;
            c.w64(11, r62);
            c.logic(r62, 64);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008347e8); }
            blk = 70; continue;
        }
        70 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008347ef); };
            c.w64(8, t63);
            let Some(t64) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008347f6); };
            c.w64(9, t64);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008347fd); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000834804); };
            c.cmp(t66, c.r64(11), 64);
            if c.cc_be() { blk = 520; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700083480e); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 72; continue;
        }
        72 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000834815); };
            c.w64(9, t67);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700083481c); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000834820); }
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r68 = c.r64(11) & 0xfffffu64;
            c.w64(11, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834839); };
            c.w32(0, t69);
            let r70 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r70);
            blk = 20; continue;
        }
        74 => {
            let Some(t71) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000834846); };
            c.w64(5, t71);
            let Some(t72) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x700083484a); };
            c.w64(10, t72);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700083484f); }
            let Some(t73) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000834854); };
            c.w64(10, t73);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x7000834859); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(0)) { return RegfileExit::fault(0x700083485e); }
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(7)) { return RegfileExit::fault(0x7000834868); }
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(1)) { return RegfileExit::fault(0x700083486d); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(3)) { return RegfileExit::fault(0x7000834872); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2060)) else { return RegfileExit::fault(0x7000834877); };
            c.w64(3, t74);
            let r75 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r75);
            return RegfileExit::jump(0x700086ed74);
        }
        75 => {
            c.w64(0, c.r64(7));
            let Some(t76) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083488a); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834891); };
            c.w32(9, t78);
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t79) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008348a2); };
            c.w32(11, t79);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008348a7); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 262; continue; }
            blk = 77; continue;
        }
        77 => {
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008348c4); };
            c.w32(12, t80);
            let r81 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r81);
            c.logic(c.r32(9) & 0xffc00u64, 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 79; continue;
        }
        79 => {
            let r82 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r82);
            let r83 = c.r32(9) & 0x3ffu64;
            c.w32(9, r83);
            c.logic(r83, 32);
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_be() { blk = 92; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t84) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008348ed); };
            c.w32(15, (t84 as u16 as i16 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008348f3); };
            c.w32(3, t85);
            let r86 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r86);
            let r87 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r87);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008348fd); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        81 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000834905); }
            blk = 82; continue;
        }
        82 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000834909); };
            c.w64(11, t88);
            let r89 = c.sub(c.r32(11), c.r32(7), 32);
            c.w32(11, r89);
            let r90 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r90);
            let r91 = c.add(c.r32(11), c.r32(7), 32);
            c.w32(11, r91);
            c.w32(2, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083491d); };
            c.w32(2, t92);
            let r93 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r93);
            let r94 = c.shr(c.r32(2), 0x9u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r94);
            let r95 = c.r32(2) & 0x3ffu64;
            c.w32(2, r95);
            c.logic(r95, 32);
            c.w32(2, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834933); };
            c.w32(2, t96);
            let r97 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r97);
            let Some(t98) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083493b); };
            c.w32(2, t98);
            let r99 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r99);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_be() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w32(7, c.r64(11).wrapping_add(0x1));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000834949); };
            c.w64(11, t100);
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 85; continue;
        }
        85 => {
            blk = 87; continue;
        }
        86 => {
            let r101 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r101);
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_l() { blk = 156; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w32(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000834967); };
            c.w32(11, t102);
            let r103 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r103);
            let r104 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r104);
            let r105 = c.r32(11) & 0x3ffu64;
            c.w32(11, r105);
            c.logic(r105, 32);
            c.w32(2, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834981); };
            c.w32(6, t106);
            let r107 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r107);
            let Some(t108) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834989); };
            c.w32(0, t108);
            let r109 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r109);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_ne() { blk = 156; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_be() { blk = 156; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(9, c.r64(2));
            blk = 96; continue;
        }
        92 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r110 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r110);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 156; continue; }
            blk = 93; continue;
        }
        93 => {
            let r111 = c.sub(c.r64(9), 0x3u64, 64);
            c.w64(9, r111);
            let Some(t112) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008349be); };
            c.cmp(t112, c.r32(1), 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ne() { blk = 93; continue; }
            blk = 95; continue;
        }
        95 => {
            blk = 156; continue;
        }
        96 => {
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t113) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008349d4); };
            c.w32(11, t113);
            c.w32(15, c.r32(11));
            c.w64(0, c.r64(11));
            let r114 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r114);
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t115) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008349e7); };
            c.w32(8, t115);
            let r116 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008349ef); };
            c.w64(12, t117);
            blk = 109; continue;
        }
        98 => {
            c.w64(9, c.r64(0));
            if let Some(sr) = c.shr(c.r64(9), 0x13u64, 64) { c.w64(9, sr); }
            let Some(t118) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008349ff); };
            c.w32(11, t118);
            let r119 = c.r64(9) & 0x3ffu64;
            c.w64(9, r119);
            c.logic(r119, 64);
            let r120 = c.r32(15) & 0x380u64;
            c.w32(15, r120);
            c.logic(r120, 32);
            let r121 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r121);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t122) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834a24); };
            c.w32(8, t122);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 103; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000834a2e); };
            c.w64(12, t123);
            let Some(t124) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834a32); };
            c.w32(15, t124);
            let r125 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r125);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000834a40); };
            c.w64(15, t126);
            blk = 101; continue;
        }
        101 => {
            let r127 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r127);
            let Some(t128) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834a4a); };
            c.w32(8, t128);
            let r129 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r129);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 102; continue;
        }
        102 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000834a58); }
            blk = 105; continue;
        }
        103 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000834a66); };
            c.w64(12, t130);
            let Some(t131) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834a6a); };
            c.w32(8, t131);
            let r132 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r132);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000834a72); }
            blk = 105; continue;
        }
        105 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000834a78); };
            c.w64(8, t133);
            let Some(t134) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000834a7f); };
            c.w64(9, t134);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000834a86); };
            c.w64(15, t135);
            let Some(t136) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000834a8d); };
            c.cmp(t136, c.r64(11), 64);
            if c.cc_be() { blk = 525; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000834a96); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 107; continue;
        }
        107 => {
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000834a9d); };
            c.w64(9, t137);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000834aa4); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000834aa8); }
            blk = 109; continue;
        }
        108 => {
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000834ab0); };
            c.w64(12, t138);
            let Some(t139) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834ab4); };
            c.w32(8, t139);
            let r140 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r140);
            blk = 109; continue;
        }
        109 => {
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(12));
            blk = 118; continue;
        }
        110 => {
            let Some(t141) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834ac7); };
            c.w32(9, t141);
            let r142 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r142);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000834ad4); };
            c.w64(9, t143);
            blk = 112; continue;
        }
        112 => {
            let Some(t144) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834adb); };
            c.w32(12, t144);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r145 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r145);
            let Some(t146) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834ae9); };
            c.w32(15, t146);
            let r147 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r147);
            let r148 = c.r64(15) & c.r64(12);
            c.w64(15, r148);
            c.logic(r148, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 114; continue;
        }
        113 => {
            let r149 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r149);
            let r150 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r150);
            c.w64(2, c.r64(12));
            let r151 = c.r64(2) & c.r64(15);
            c.w64(2, r151);
            c.logic(r151, 64);
            c.w64(15, c.r64(2));
            blk = 114; continue;
        }
        114 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(7).wrapping_mul(4));
            let Some(t152) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834b15); };
            c.w32(6, t152);
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000834b1a); };
            c.cmp(t153, c.r32(6), 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_ne() { blk = 113; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t154) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834b2b); };
            c.w32(8, t154);
            let Some(t155) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834b2f); };
            c.w32(11, t155);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(12, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(8), 64);
            if c.cc_ae() { blk = 527; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t156) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834b47); };
            c.w32(8, t156);
            let r157 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r157);
            let r158 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r158);
            c.w64(9, c.r64(11));
            blk = 118; continue;
        }
        118 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t159) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834b5f); };
            c.w32(9, t159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000834b63); };
            c.cmp(t160, c.r32(9), 32);
            if c.cc_ne() { blk = 138; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834b70); };
            c.w32(2, t161);
            let r162 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r162);
            let Some(t163) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834b77); };
            c.w32(8, t163);
            let r164 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r164);
            let Some(t165) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000834b7e); };
            c.logic(t165 & 0x2u64, 8);
            if c.cc_ne() { blk = 136; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000834b89); };
            c.cmp(t166, c.r32(8), 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000834b92); };
            c.w64(0, t167);
            blk = 20; continue;
        }
        123 => {
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000834b9e); };
            c.w64(8, t168);
            let Some(t169) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000834ba5); };
            c.cmp(t169, 0x20u64, 8);
            if c.cc_e() { blk = 529; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t170) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000834baf); };
            c.w32(8, t170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000834bb3); };
            c.cmp(t171, c.r32(8), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t172) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834bc0); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            let Some(t174) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834bc7); };
            c.w32(9, t174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000834bcb); };
            c.cmp(t175, c.r32(9), 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t176) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000834bd4); };
            c.w32(8, t176);
            let r177 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r177);
            let Some(t178) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834bdb); };
            c.w32(9, t178);
            let Some(t179) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000834bdf); };
            c.cmp(t179, c.r32(9), 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 127; continue;
        }
        127 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t180) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834bf2); };
            c.w32(9, t180);
            let r181 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r181);
            let Some(t182) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834bf9); };
            c.w32(11, t182);
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 133; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w32(8, c.r16(11));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t183) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000834c18); };
            c.cmp(t183, c.r32(9), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w64(8, c.r64(0));
            blk = 133; continue;
        }
        132 => {
            let Some(t184) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834c2a); };
            c.w32(8, t184);
            let r185 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r185);
            blk = 133; continue;
        }
        133 => {
            let Some(t186) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834c31); };
            c.w32(8, t186);
            let r187 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r187);
            let Some(t188) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834c38); };
            c.w32(8, t188);
            let r189 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r189);
            let Some(t190) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834c3f); };
            c.w32(9, t190);
            let r191 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r191);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 266; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t192) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000834c55); };
            c.w32(6, t192);
            let r193 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000834c5c); };
            if !c.push64(t194) { return RegfileExit::fault(0x7000834c5c); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000834c66, 0x7000834c61);
        }
        135 => {
            blk = 20; continue;
        }
        136 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000834c6b); };
            if !c.push64(t195) { return RegfileExit::fault(0x7000834c6b); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000834c73); };
            c.w64(6, t196);
            return c.call(0x70007f2274, 0x7000834c7c, 0x7000834c77);
        }
        137 => {
            blk = 20; continue;
        }
        138 => {
            let Some(t197) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834c81); };
            c.w32(9, t197);
            let r198 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r198);
            let Some(t199) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834c88); };
            c.w32(11, t199);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 153; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 145; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 266; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t200) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834cb2); };
            c.w32(8, t200);
            let Some(t201) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000834cb6); };
            c.cmp(t201, c.r32(8), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t202) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834cc3); };
            c.w32(8, t202);
            let r203 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r203);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t204) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834cd4); };
            c.w32(9, t204);
            let r205 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r205);
            let Some(t206) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834cdb); };
            c.cmp(t206, 0x40u64, 16);
            if c.cc_ae() { blk = 266; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t207) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834ce7); };
            c.w32(8, t207);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        145 => {
            let Some(t208) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834cf4); };
            c.w32(8, t208);
            let Some(t209) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000834cf8); };
            c.cmp(t209, c.r32(8), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t210) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000834d05); };
            c.w32(8, t210);
            c.w64(9, c.r64(8));
            let r211 = c.r32(9) & 0xc0u64;
            c.w32(9, r211);
            c.logic(r211, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t212) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834d1d); };
            c.w32(9, t212);
            let r213 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r213);
            let Some(t214) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000834d24); };
            c.w32(9, t214);
            let r215 = c.r32(9) & 0x1fu64;
            c.w32(9, r215);
            c.logic(r215, 32);
            let r216 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r216);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_a() { blk = 266; continue; }
            blk = 148; continue;
        }
        148 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 266; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t217) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000834d44); };
            c.w32(8, t217);
            let r218 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000834d4b); };
            c.cmp(t219, c.r32(8), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t220) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834d58); };
            c.w32(9, t220);
            let Some(t221) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000834d5c); };
            c.cmp(t221, c.r32(9), 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        152 => {
            let Some(t222) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834d6d); };
            c.w32(0, t222);
            let r223 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r223);
            blk = 20; continue;
        }
        153 => {
            let Some(t224) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834d79); };
            c.w32(8, t224);
            let Some(t225) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000834d7d); };
            c.cmp(t225, c.r32(8), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t226) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834d8a); };
            c.w32(0, t226);
            let r227 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r227);
            blk = 20; continue;
        }
        155 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        156 => {
            let Some(t228) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834d9d); };
            c.w32(9, t228);
            let Some(t229) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000834da1); };
            c.cmp(t229, c.r32(9), 32);
            if c.cc_ne() { blk = 160; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t230) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834daa); };
            c.w32(9, t230);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 160; continue; }
            blk = 158; continue;
        }
        158 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 531; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 261; continue;
        }
        160 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000834dc3); };
            c.w64(11, t231);
            blk = 162; continue;
        }
        161 => {
            c.w64(8, c.r64(9));
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(11), 0x82eu64, 32);
            if c.cc_e() { blk = 540; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t232) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834dd9); };
            c.w32(8, t232);
            let r233 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r233);
            let Some(t234) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000834de0); };
            c.cmp(t234, c.r32(8), 32);
            if c.cc_e() { blk = 261; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t235) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000834ded); };
            c.w32(9, t235);
            let r236 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r236);
            let Some(t237) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834df4); };
            c.w32(11, t237);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000834df9); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 193; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t238) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000834e09); };
            c.w32(12, t238);
            c.logic(c.r32(12) & 0x200000u64, 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t239) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000834e1a); };
            c.w32(15, t239);
            let r240 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r240);
            c.logic(c.r32(12) & 0xffc00u64, 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 167; continue;
        }
        167 => {
            let r241 = c.shr(c.r32(12), 0xau64, 32).unwrap_or(c.r32(12));
            c.w32(12, r241);
            let r242 = c.r32(12) & 0x3ffu64;
            c.w32(12, r242);
            c.logic(r242, 32);
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_be() { blk = 182; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t243) = c.ld16(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000834e3f); };
            c.w32(0, (t243 as u16 as i16 as i64 as u64));
            let Some(t244) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834e44); };
            c.w32(3, t244);
            let r245 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r245);
            let r246 = c.sub(c.r32(0), 0x1u64, 32);
            c.w32(0, r246);
            c.w64(6, c.r64(0));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 170; continue;
        }
        169 => {
            c.w64(6, c.r64(7));
            blk = 170; continue;
        }
        170 => {
            c.w64(7, c.r64(6));
            let r247 = c.sub(c.r32(7), c.r32(2), 32);
            c.w32(7, r247);
            let r248 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r248);
            let r249 = c.add(c.r32(7), c.r32(2), 32);
            c.w32(7, r249);
            c.w32(9, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t250) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000834e67); };
            c.w32(9, t250);
            let r251 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r251);
            let r252 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r252);
            let r253 = c.r32(9) & 0x3ffu64;
            c.w32(9, r253);
            c.logic(r253, 32);
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t254) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834e81); };
            c.w32(9, t254);
            let r255 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r255);
            let Some(t256) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834e89); };
            c.w32(9, t256);
            let r257 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r257);
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_be() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w32(2, c.r64(7).wrapping_add(0x1));
            c.w64(7, c.r64(6));
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 173; continue;
        }
        173 => {
            blk = 175; continue;
        }
        174 => {
            let r258 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r258);
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_l() { blk = 181; continue; }
            blk = 176; continue;
        }
        176 => {
            c.w32(9, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t259) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000834eb0); };
            c.w32(9, t259);
            let r260 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r260);
            let r261 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r261);
            let r262 = c.r32(9) & 0x3ffu64;
            c.w32(9, r262);
            c.logic(r262, 32);
            c.w32(6, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t263) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834eca); };
            c.w32(7, t263);
            let r264 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r264);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000834ed2); }
            let Some(t265) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834ed6); };
            c.w32(6, t265);
            let r266 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r266);
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_ne() { blk = 180; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_ne() { blk = 174; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_a() { blk = 205; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000834eed); };
            c.w64(9, t267);
            blk = 161; continue;
        }
        180 => {
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000834ef6); };
            c.w64(9, t268);
            blk = 161; continue;
        }
        181 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000834eff); };
            c.w64(9, t269);
            blk = 161; continue;
        }
        182 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r270 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(6, r270);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 161; continue; }
            blk = 183; continue;
        }
        183 => {
            let r271 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r271);
            let Some(t272) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834f1d); };
            c.cmp(t272, c.r32(1), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 184; continue;
        }
        184 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 183; continue; }
            blk = 185; continue;
        }
        185 => {
            blk = 161; continue;
        }
        186 => {
            let Some(t273) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834f33); };
            c.w32(12, t273);
            let r274 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r274);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000834f40); };
            c.w64(12, t275);
            blk = 188; continue;
        }
        188 => {
            let Some(t276) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834f47); };
            c.w32(15, t276);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r277 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r277);
            let Some(t278) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834f56); };
            c.w32(0, t278);
            let r279 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r279);
            let r280 = c.r64(0) & c.r64(15);
            c.w64(0, r280);
            c.logic(r280, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 190; continue;
        }
        189 => {
            let r281 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r281);
            let r282 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r282);
            c.w64(2, c.r64(15));
            let r283 = c.r64(2) & c.r64(0);
            c.w64(2, r283);
            c.logic(r283, 64);
            c.w64(0, c.r64(2));
            blk = 190; continue;
        }
        190 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(2).wrapping_mul(4));
            let Some(t284) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834f80); };
            c.w32(7, t284);
            let Some(t285) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000834f85); };
            c.cmp(t285, c.r32(7), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 192; continue;
        }
        192 => {
            blk = 189; continue;
        }
        193 => {
            c.cmp(c.r16(11), 0xbdu64, 16);
            if c.cc_ne() { blk = 260; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t286) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000834fa8); };
            c.logic(t286 & 0x24u64, 8);
            if c.cc_ne() { blk = 260; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t287) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834fb3); };
            c.w32(12, t287);
            let r288 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r288);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t289) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000834fc0); };
            c.w64(12, t289);
            blk = 197; continue;
        }
        197 => {
            let Some(t290) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000834fc7); };
            c.w32(15, t290);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r291 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r291);
            let Some(t292) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000834fd6); };
            c.w32(0, t292);
            let r293 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r293);
            let r294 = c.r64(0) & c.r64(15);
            c.w64(0, r294);
            c.logic(r294, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 199; continue;
        }
        198 => {
            let r295 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r295);
            let r296 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r296);
            c.w64(2, c.r64(15));
            let r297 = c.r64(2) & c.r64(0);
            c.w64(2, r297);
            c.logic(r297, 64);
            c.w64(0, c.r64(2));
            blk = 199; continue;
        }
        199 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t298) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000834ff7); };
            c.w32(6, t298);
            let r299 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r299);
            let Some(t300) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000834fff); };
            c.cmp(t300, c.r32(6), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t301) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083500c); };
            c.cmp(t301, c.r32(6), 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t302) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835015); };
            c.cmp(t302, c.r32(1), 32);
            if c.cc_ne() { blk = 198; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t303) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083501a); };
            c.w32(12, t303);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_ae() { blk = 536; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t304) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083502e); };
            c.w32(12, t304);
            let r305 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r305);
            let Some(t306) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000835035); };
            c.cmp(t306, c.r32(12), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 204; continue;
        }
        204 => {
            blk = 221; continue;
        }
        205 => {
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000835047); };
            c.w64(6, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083504b); };
            c.w64(9, t308);
            blk = 206; continue;
        }
        206 => {
            c.w64(11, c.r64(6).wrapping_mul(4));
            let Some(t309) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000835057); };
            c.w32(12, t309);
            c.w32(0, c.r32(12));
            c.w64(3, c.r64(12));
            let r310 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r310);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t311) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083506a); };
            c.w32(12, t311);
            let r312 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r312);
            c.w64(9, c.r64(12));
            blk = 222; continue;
        }
        208 => {
            c.w64(11, c.r64(3));
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t313) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000835081); };
            c.w32(12, t313);
            let r314 = c.r64(11) & 0x3ffu64;
            c.w64(11, r314);
            c.logic(r314, 64);
            let r315 = c.r32(0) & 0x380u64;
            c.w32(0, r315);
            c.logic(r315, 32);
            let r316 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r316);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let Some(t317) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008350a2); };
            c.w32(9, t317);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 213; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t318) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008350ac); };
            c.w32(15, t318);
            let r319 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r319);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008350b9); };
            c.w64(15, t320);
            blk = 211; continue;
        }
        211 => {
            let r321 = c.sub(c.r64(11), c.r64(9), 64);
            c.w64(11, r321);
            let Some(t322) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008350c3); };
            c.w32(9, t322);
            let r323 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r323);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 222; continue; }
            blk = 212; continue;
        }
        212 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008350d5); }
            blk = 215; continue;
        }
        213 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 218; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t324) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008350e3); };
            c.w32(9, t324);
            let r325 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r325);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008350eb); }
            blk = 215; continue;
        }
        215 => {
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008350f1); };
            c.w64(9, t326);
            let Some(t327) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008350f8); };
            c.w64(11, t327);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008350ff); };
            c.w64(15, t328);
            let Some(t329) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000835106); };
            c.cmp(t329, c.r64(12), 64);
            if c.cc_be() { blk = 532; continue; }
            blk = 216; continue;
        }
        216 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083510f); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 217; continue;
        }
        217 => {
            let Some(t330) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000835116); };
            c.w64(11, t330);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700083511d); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000835121); }
            blk = 222; continue;
        }
        218 => {
            let Some(t331) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835129); };
            c.w32(12, t331);
            let r332 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r332);
            c.w64(9, c.r64(12));
            blk = 222; continue;
        }
        219 => {
            let Some(t333) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835136); };
            c.w32(9, t333);
            let Some(t334) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083513b); };
            c.w32(11, t334);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t335) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835153); };
            c.w32(12, t335);
            let r336 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r336);
            let r337 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r337);
            c.w64(9, c.r64(12));
            c.w64(3, c.r64(11));
            blk = 222; continue;
        }
        221 => {
            let Some(t338) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835166); };
            c.w32(9, t338);
            let r339 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r339);
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(12));
            blk = 222; continue;
        }
        222 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t340) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083517c); };
            c.w32(11, t340);
            let Some(t341) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000835180); };
            c.cmp(t341, c.r32(11), 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t342) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083518d); };
            c.w32(2, t342);
            let r343 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r343);
            let Some(t344) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835194); };
            c.w32(9, t344);
            let r345 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r345);
            let Some(t346) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083519b); };
            c.logic(t346 & 0x2u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t347) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008351a6); };
            c.cmp(t347, c.r32(9), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008351af); };
            c.w64(0, t348);
            blk = 20; continue;
        }
        227 => {
            let Some(t349) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008351bb); };
            c.w64(9, t349);
            let Some(t350) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x70008351c2); };
            c.cmp(t350, 0x20u64, 8);
            if c.cc_e() { blk = 538; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t351) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008351cc); };
            c.w32(9, t351);
            let Some(t352) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008351d0); };
            c.cmp(t352, c.r32(9), 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t353) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008351dd); };
            c.w32(9, t353);
            let r354 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r354);
            let Some(t355) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008351e4); };
            c.w32(11, t355);
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008351e8); };
            c.cmp(t356, c.r32(11), 32);
            if c.cc_ne() { blk = 231; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t357) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008351f1); };
            c.w32(9, t357);
            let r358 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r358);
            let Some(t359) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008351f8); };
            c.w32(11, t359);
            let Some(t360) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008351fc); };
            c.cmp(t360, c.r32(11), 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 231; continue;
        }
        231 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 260; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t361) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083520f); };
            c.w32(11, t361);
            let r362 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r362);
            let Some(t363) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835216); };
            c.w32(12, t363);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 237; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000835236); };
            c.cmp(t364, c.r32(11), 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 235; continue;
        }
        235 => {
            blk = 238; continue;
        }
        236 => {
            let Some(t365) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835245); };
            c.w32(8, t365);
            let r366 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r366);
            blk = 238; continue;
        }
        237 => {
            c.w64(8, c.r64(9));
            blk = 238; continue;
        }
        238 => {
            let Some(t367) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835251); };
            c.w32(8, t367);
            let r368 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r368);
            let Some(t369) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835258); };
            c.w32(8, t369);
            let r370 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r370);
            let Some(t371) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083525f); };
            c.w32(9, t371);
            let r372 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r372);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 260; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t373) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000835275); };
            c.w32(6, t373);
            let r374 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083527c); };
            if !c.push64(t375) { return RegfileExit::fault(0x700083527c); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000835286, 0x7000835281);
        }
        240 => {
            blk = 20; continue;
        }
        241 => {
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083528b); };
            if !c.push64(t376) { return RegfileExit::fault(0x700083528b); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000835293); };
            c.w64(6, t377);
            return c.call(0x70007f2274, 0x700083529c, 0x7000835297);
        }
        242 => {
            blk = 20; continue;
        }
        243 => {
            let Some(t378) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008352a1); };
            c.w32(11, t378);
            let r379 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r379);
            let Some(t380) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008352a8); };
            c.w32(12, t380);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 257; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 250; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 260; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t381) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008352d3); };
            c.w32(9, t381);
            let Some(t382) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008352d7); };
            c.cmp(t382, c.r32(9), 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t383) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008352e4); };
            c.w32(8, t383);
            let r384 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r384);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 260; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t385) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008352f5); };
            c.w32(9, t385);
            let r386 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r386);
            let Some(t387) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008352fc); };
            c.cmp(t387, 0x40u64, 16);
            if c.cc_ae() { blk = 260; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t388) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835308); };
            c.w32(8, t388);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        250 => {
            let Some(t389) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835315); };
            c.w32(9, t389);
            let Some(t390) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000835319); };
            c.cmp(t390, c.r32(9), 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t391) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000835326); };
            c.w32(9, t391);
            c.w64(11, c.r64(9));
            let r392 = c.r32(11) & 0xc0u64;
            c.w32(11, r392);
            c.logic(r392, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t393) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083533e); };
            c.w32(11, t393);
            let r394 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r394);
            let Some(t395) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000835345); };
            c.w32(11, t395);
            let r396 = c.r32(11) & 0x1fu64;
            c.w32(11, r396);
            c.logic(r396, 32);
            let r397 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r397);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 260; continue; }
            blk = 253; continue;
        }
        253 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 260; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t398) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083535d); };
            c.w32(0, t398);
            let r399 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r399);
            let Some(t400) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000835364); };
            c.cmp(t400, c.r32(0), 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t401) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083536d); };
            c.w32(8, t401);
            let Some(t402) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000835371); };
            c.cmp(t402, c.r32(8), 32);
            if c.cc_ne() { blk = 342; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t403) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083537e); };
            c.w32(0, t403);
            let r404 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r404);
            blk = 20; continue;
        }
        257 => {
            let Some(t405) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835389); };
            c.w32(9, t405);
            let Some(t406) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083538d); };
            c.cmp(t406, c.r32(9), 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t407) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835396); };
            c.w32(0, t407);
            let r408 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r408);
            blk = 20; continue;
        }
        259 => {
            c.w64(0, c.r64(9));
            blk = 20; continue;
        }
        260 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008353aa); };
            c.w64(0, t409);
            blk = 266; continue;
        }
        261 => {
            let Some(t410) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008353b0); };
            c.w64(0, t410);
            blk = 20; continue;
        }
        262 => {
            c.cmp(c.r16(11), 0xbcu64, 16);
            if c.cc_ne() { blk = 266; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t411) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008353c4); };
            c.w32(8, t411);
            let Some(t412) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008353c8); };
            c.cmp(t412, c.r32(8), 32);
            if c.cc_ne() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t413) = c.ld8(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008353d1); };
            c.logic(t413 & 0x1u64, 8);
            if c.cc_ne() { blk = 266; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t414) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008353d7); };
            c.w64(5, t414);
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            let r415 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r415);
            return RegfileExit::jump(0x70008bc174);
        }
        266 => {
            let Some(t416) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008353ef); };
            c.w64(5, t416);
            let Some(t417) = c.ld64(c.r64(13).wrapping_add(0x2068)) else { return RegfileExit::fault(0x70008353f3); };
            c.w64(3, t417);
            let Some(t418) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008353fa); };
            c.w64(10, t418);
            let Some(t419) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x70008353ff); };
            if !c.push64(t419) { return RegfileExit::fault(0x70008353ff); }
            let sp420 = c.r64(4);
            let Some(pv421) = c.ld64(sp420) else { return RegfileExit::fault(0x7000835403); };
            c.set_rsp(sp420.wrapping_add(8));
            let pea422 = c.r64(4).wrapping_add(0x30);
            if !c.st64(pea422, pv421) { c.set_rsp(sp420); return RegfileExit::fault(0x7000835403); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x7000835407); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(0)) { return RegfileExit::fault(0x700083540c); }
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(1)) { return RegfileExit::fault(0x7000835411); }
            c.w32(0, 0x3u64);
            let Some(t423) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083541b); };
            c.w64(6, t423);
            let r424 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r424);
            return RegfileExit::jump(0x700086ed74);
        }
        267 => {
            blk = 74; continue;
        }
        268 => {
            let Some(t425) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700083542e); };
            c.cmp(t425, c.r32(15), 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t426) = c.ld32(c.r64(13).wrapping_add(0xc00)) else { return RegfileExit::fault(0x7000835437); };
            c.cmp(t426, c.r32(15), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t427) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835444); };
            c.cmp(t427, 0x422u64, 16);
            if c.cc_ne() { blk = 74; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t428) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000835451); };
            c.logic(t428 & 0x20u64, 8);
            if c.cc_ne() { blk = 74; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t429) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083545c); };
            c.w32(8, t429);
            let r430 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r430);
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0xfb0)) else { return RegfileExit::fault(0x7000835464); };
            c.w64(9, t431);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t432) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083546e); };
            c.cmp(t432, c.r32(11), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t433) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835478); };
            c.w32(9, t433);
            let r434 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r434);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 274; continue;
        }
        274 => {
            let r435 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r435);
            c.logic(r435, 64);
            let Some(t436) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083548d); };
            c.w32(8, t436);
            let r437 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r437);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 275; continue;
        }
        275 => {
            let r438 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r438);
            c.logic(r438, 64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008354a2); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            c.w64(6, c.r64(8));
            return c.call(0x70007f6e94, 0x70008354b0, 0x70008354ab);
        }
        276 => {
            blk = 20; continue;
        }
        277 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t439) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008354bf); };
            c.w32(8, t439);
            let r440 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r440);
            c.w64(9, c.r64(8));
            if let Some(sr) = c.shr(c.r64(9), 0xdu64, 64) { c.w64(9, sr); }
            let Some(t441) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008354cd); };
            c.w32(11, t441);
            let r442 = c.r64(9) ^ c.r64(8);
            c.w64(9, r442);
            c.logic(r442, 64);
            let r443 = c.add(c.r32(9), c.r32(11), 32);
            c.w32(9, r443);
            let r444 = c.r32(9) & 0x1ffcu64;
            c.w32(9, r444);
            c.logic(r444, 32);
            let r445 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(11, r445);
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0x2f18)) else { return RegfileExit::fault(0x70008354e2); };
            c.w64(12, t446);
            let Some(t447) = c.ld32(c.r64(12).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008354e9); };
            c.cmp(t447, c.r32(1), 32);
            if c.cc_ne() { blk = 281; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t448) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008354ef); };
            c.cmp(t448, c.r32(8), 32);
            if c.cc_ne() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t449) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008354f6); };
            c.w32(8, t449);
            let r450 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r450);
            blk = 14; continue;
        }
        281 => {
            c.w32(11, c.r32(1));
            c.w32(9, c.r32(9));
            let r451 = c.sub(c.r32(9), c.r32(11), 32);
            c.w32(9, r451);
            let r452 = c.sub(c.r32(9), 0x4e93591bu64, 32);
            c.w32(9, r452);
            let r453 = c.r32(9) & 0x7fcu64;
            c.w32(9, r453);
            c.logic(r453, 32);
            let r454 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r454);
            let Some(t455) = c.ld64(c.r64(13).wrapping_add(0x2f30)) else { return RegfileExit::fault(0x700083551e); };
            c.w64(11, t455);
            let Some(t456) = c.ld32(c.r64(11).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000835525); };
            c.cmp(t456, c.r32(1), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t457) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083552f); };
            c.cmp(t457, c.r32(8), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t458) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x700083553a); };
            c.w32(8, t458);
            let r459 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r459);
            blk = 14; continue;
        }
        284 => {
            let Some(t460) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835547); };
            c.w32(9, t460);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700083554b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700083554f); }
            let Some(t461) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000835553); };
            c.cmp(t461, c.r32(9), 32);
            if c.cc_e() { blk = 365; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t462) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835560); };
            c.w32(9, t462);
            let r463 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r463);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t464) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083556f); };
            c.cmp(t464, c.r32(11), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t465) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835579); };
            c.w32(9, t465);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000835583); }
            c.logic(c.r8(12) & 0x30u64, 8);
            if c.cc_e() { blk = 358; continue; }
            blk = 288; continue;
        }
        288 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 350; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t466) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083559b); };
            c.w32(12, t466);
            let r467 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r467);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            let Some(t468) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008355a8); };
            c.w64(12, t468);
            blk = 291; continue;
        }
        291 => {
            let Some(t469) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008355af); };
            c.w32(15, t469);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r470 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r470);
            let Some(t471) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008355be); };
            c.w32(2, t471);
            let r472 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r472);
            let r473 = c.r64(2) & c.r64(15);
            c.w64(2, r473);
            c.logic(r473, 64);
            blk = 292; continue;
        }
        292 => {
            c.w64(0, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(0).wrapping_mul(4));
            let Some(t474) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008355d4); };
            c.w32(9, t474);
            let Some(t475) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008355d9); };
            c.cmp(t475, c.r32(9), 32);
            if c.cc_e() { blk = 349; continue; }
            blk = 293; continue;
        }
        293 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_e() { blk = 295; continue; }
            blk = 294; continue;
        }
        294 => {
            let r476 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r476);
            c.w64(9, c.r64(11).wrapping_add(c.r64(2)));
            c.w64(0, c.r64(15));
            let r477 = c.r64(0) & c.r64(9);
            c.w64(0, r477);
            c.logic(r477, 64);
            c.w64(2, c.r64(0));
            blk = 292; continue;
        }
        295 => {
            let Some(t478) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008355fe); };
            c.w32(8, t478);
            let Some(t479) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835603); };
            c.w32(9, t479);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 297; continue; }
            blk = 296; continue;
        }
        296 => {
            return RegfileExit::fault(0x7000835617);
        }
        297 => {
            let Some(t480) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835618); };
            c.w32(0, t480);
            let r481 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r481);
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 342; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t482) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083562a); };
            c.w32(8, t482);
            let Some(t483) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083562e); };
            c.cmp(t483, c.r32(8), 32);
            if c.cc_ne() { blk = 323; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t484) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083563b); };
            c.w32(2, t484);
            let r485 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r485);
            let Some(t486) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835641); };
            c.w32(8, t486);
            let r487 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r487);
            let Some(t488) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000835648); };
            c.logic(t488 & 0x2u64, 8);
            if c.cc_ne() { blk = 321; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t489) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000835653); };
            c.cmp(t489, c.r32(8), 32);
            if c.cc_e() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            let Some(t490) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083565c); };
            c.w64(0, t490);
            blk = 20; continue;
        }
        302 => {
            let Some(t491) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000835668); };
            c.w64(8, t491);
            let Some(t492) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x700083566f); };
            c.cmp(t492, 0x20u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 303; continue;
        }
        303 => {
            let Some(t493) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000835679); };
            c.w32(8, t493);
            let Some(t494) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083567d); };
            c.cmp(t494, c.r32(8), 32);
            if c.cc_e() { blk = 305; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t495) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000835686); };
            c.w64(0, t495);
            let Some(t496) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083568a); };
            c.w64(3, t496);
            blk = 74; continue;
        }
        305 => {
            let Some(t497) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835693); };
            c.w32(8, t497);
            let r498 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r498);
            let Some(t499) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083569a); };
            c.w32(9, t499);
            let Some(t500) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083569e); };
            c.cmp(t500, c.r32(9), 32);
            if c.cc_ne() { blk = 307; continue; }
            blk = 306; continue;
        }
        306 => {
            let Some(t501) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008356a7); };
            c.w32(8, t501);
            let r502 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r502);
            let Some(t503) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008356ae); };
            c.w32(9, t503);
            let Some(t504) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008356b2); };
            c.cmp(t504, c.r32(9), 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 307; continue;
        }
        307 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 318; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t505) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008356c5); };
            c.w32(9, t505);
            let r506 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r506);
            let Some(t507) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008356cc); };
            c.w32(11, t507);
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 314; continue; }
            blk = 309; continue;
        }
        309 => {
            c.w32(8, c.r16(11));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t508) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008356eb); };
            c.cmp(t508, c.r32(9), 32);
            if c.cc_e() { blk = 312; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(8, c.r64(7));
            blk = 314; continue;
        }
        312 => {
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008356f9); };
            c.w64(0, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008356fd); };
            c.w64(3, t510);
            blk = 74; continue;
        }
        313 => {
            let Some(t511) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835706); };
            c.w32(8, t511);
            let r512 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r512);
            blk = 314; continue;
        }
        314 => {
            let Some(t513) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083570d); };
            c.w32(8, t513);
            let r514 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r514);
            let Some(t515) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835714); };
            c.w32(8, t515);
            let r516 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r516);
            let Some(t517) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083571b); };
            c.w32(9, t517);
            let r518 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r518);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 317; continue; }
            blk = 315; continue;
        }
        315 => {
            let Some(t519) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083572d); };
            c.w32(6, t519);
            let r520 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000835734); };
            if !c.push64(t521) { return RegfileExit::fault(0x7000835734); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700083573e, 0x7000835739);
        }
        316 => {
            blk = 20; continue;
        }
        317 => {
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000835743); };
            c.w64(0, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000835747); };
            c.w64(3, t523);
            blk = 74; continue;
        }
        318 => {
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000835750); };
            c.w64(0, t524);
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000835754); };
            c.w64(3, t525);
            blk = 74; continue;
        }
        319 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083575d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083575e); }
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083575f); };
            if !c.push64(t526) { return RegfileExit::fault(0x700083575f); }
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000835762); };
            c.w64(3, t527);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000835773, 0x700083576e);
        }
        320 => {
            blk = 20; continue;
        }
        321 => {
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000835778); };
            if !c.push64(t528) { return RegfileExit::fault(0x7000835778); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x7000835788, 0x7000835783);
        }
        322 => {
            blk = 20; continue;
        }
        323 => {
            let Some(t529) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083578d); };
            c.w32(8, t529);
            let r530 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r530);
            let Some(t531) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835794); };
            c.w32(9, t531);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 346; continue; }
            blk = 324; continue;
        }
        324 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 334; continue; }
            blk = 325; continue;
        }
        325 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 327; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008357be); };
            c.w64(0, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008357c2); };
            c.w64(3, t533);
            blk = 74; continue;
        }
        327 => {
            let Some(t534) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008357cb); };
            c.w32(8, t534);
            let Some(t535) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008357cf); };
            c.cmp(t535, c.r32(8), 32);
            if c.cc_e() { blk = 329; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008357d8); };
            c.w64(0, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008357dc); };
            c.w64(3, t537);
            blk = 74; continue;
        }
        329 => {
            let Some(t538) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008357e5); };
            c.w32(8, t538);
            let r539 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r539);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 331; continue; }
            blk = 330; continue;
        }
        330 => {
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008357f2); };
            c.w64(0, t540);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008357f6); };
            c.w64(3, t541);
            blk = 74; continue;
        }
        331 => {
            let Some(t542) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008357ff); };
            c.w32(9, t542);
            let r543 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r543);
            let Some(t544) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835806); };
            c.cmp(t544, 0x40u64, 16);
            if c.cc_b() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083580e); };
            c.w64(0, t545);
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000835812); };
            c.w64(3, t546);
            blk = 74; continue;
        }
        333 => {
            let Some(t547) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083581b); };
            c.w32(8, t547);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        334 => {
            let Some(t548) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835828); };
            c.w32(9, t548);
            let Some(t549) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700083582c); };
            c.cmp(t549, c.r32(9), 32);
            if c.cc_e() { blk = 336; continue; }
            blk = 335; continue;
        }
        335 => {
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000835835); };
            c.w64(0, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000835839); };
            c.w64(3, t551);
            blk = 74; continue;
        }
        336 => {
            let Some(t552) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000835842); };
            c.w32(8, t552);
            c.w64(9, c.r64(8));
            let r553 = c.r32(9) & 0xc0u64;
            c.w32(9, r553);
            c.logic(r553, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 339; continue; }
            blk = 337; continue;
        }
        337 => {
            let Some(t554) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083585a); };
            c.w32(9, t554);
            let r555 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r555);
            let Some(t556) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000835861); };
            c.w32(9, t556);
            let r557 = c.r32(9) & 0x1fu64;
            c.w32(9, r557);
            c.logic(r557, 32);
            let r558 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r558);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 339; continue; }
            blk = 338; continue;
        }
        338 => {
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000835873); };
            c.w64(0, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000835877); };
            c.w64(3, t560);
            blk = 74; continue;
        }
        339 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 345; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t561) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000835886); };
            c.w32(0, t561);
            let r562 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r562);
            let Some(t563) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083588c); };
            c.cmp(t563, c.r32(0), 32);
            if c.cc_e() { blk = 344; continue; }
            blk = 341; continue;
        }
        341 => {
            let Some(t564) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835895); };
            c.w32(8, t564);
            let Some(t565) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000835899); };
            c.cmp(t565, c.r32(8), 32);
            if c.cc_e() { blk = 343; continue; }
            blk = 342; continue;
        }
        342 => {
            blk = 20; continue;
        }
        343 => {
            let Some(t566) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008358a7); };
            c.w32(0, t566);
            let r567 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r567);
            blk = 20; continue;
        }
        344 => {
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008358b2); };
            c.w64(0, t568);
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008358b6); };
            c.w64(3, t569);
            blk = 74; continue;
        }
        345 => {
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008358bf); };
            c.w64(0, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008358c3); };
            c.w64(3, t571);
            blk = 74; continue;
        }
        346 => {
            let Some(t572) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008358cc); };
            c.w32(8, t572);
            let Some(t573) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008358d0); };
            c.cmp(t573, c.r32(8), 32);
            if c.cc_e() { blk = 348; continue; }
            blk = 347; continue;
        }
        347 => {
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008358d9); };
            c.w64(0, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008358dd); };
            c.w64(3, t575);
            blk = 74; continue;
        }
        348 => {
            let Some(t576) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008358e6); };
            c.w32(0, t576);
            let r577 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r577);
            blk = 20; continue;
        }
        349 => {
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008358f1); };
            c.w64(0, t578);
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008358f5); };
            c.w64(3, t579);
            let Some(t580) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008358f9); };
            c.w64(9, t580);
            blk = 358; continue;
        }
        350 => {
            let Some(t581) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835902); };
            c.w32(11, t581);
            let r582 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r582);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 351; continue;
        }
        351 => {
            let r583 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r583);
            c.logic(r583, 64);
            let Some(t584) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835917); };
            c.w32(12, t584);
            let r585 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r585);
            let Some(t586) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083591e); };
            c.w32(12, t586);
            let r587 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r587);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 358; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t588) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083592b); };
            c.w32(15, t588);
            let r589 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r589);
            let Some(t590) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835932); };
            c.cmp(t590, 0xbeu64, 16);
            if c.cc_ne() { blk = 74; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t591) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083593f); };
            c.w32(15, t591);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 355; continue; }
            blk = 354; continue;
        }
        354 => {
            return RegfileExit::fault(0x7000835952);
        }
        355 => {
            let Some(t592) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835953); };
            c.w32(15, t592);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t593) = c.ld32(c.r64(11).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083595b); };
            c.w32(11, t593);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 357; continue; }
            blk = 356; continue;
        }
        356 => {
            return RegfileExit::fault(0x700083596e);
        }
        357 => {
            let Some(t594) = c.ld32(c.r64(12).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083596f); };
            c.cmp(t594, c.r32(11), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 358; continue;
        }
        358 => {
            let Some(t595) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083597d); };
            c.w32(11, t595);
            let r596 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r596);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 364; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t597) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700083598a); };
            c.cmp(t597, c.r32(11), 32);
            if c.cc_e() { blk = 362; continue; }
            blk = 360; continue;
        }
        360 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 361; continue;
        }
        361 => {
            let r598 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r598);
            c.logic(r598, 64);
            blk = 363; continue;
        }
        362 => {
            c.w64(11, c.r64(7));
            blk = 363; continue;
        }
        363 => {
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 16; continue;
        }
        364 => {
            c.w64(0, c.r64(11));
            blk = 20; continue;
        }
        365 => {
            let Some(t599) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008359bc); };
            c.w64(5, t599);
            c.w64(2, c.r64(7));
            let r600 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r600);
            let Some(t601) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008359c7); };
            c.logic(t601 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 367; continue; }
            blk = 366; continue;
        }
        366 => {
            let r602 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r602);
            blk = 368; continue;
        }
        367 => {
            let Some(t603) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008359d7); };
            c.w32(8, t603);
            let Some(t604) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008359db); };
            c.w64(8, t604);
            blk = 368; continue;
        }
        368 => {
            return RegfileExit::jump(c.r64(8));
        }
        369 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_e() { blk = 443; continue; }
            blk = 370; continue;
        }
        370 => {
            c.cmp(c.r64(12), 0x6u64, 64);
            if c.cc_e() { blk = 441; continue; }
            blk = 371; continue;
        }
        371 => {
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 438; continue; }
            blk = 372; continue;
        }
        372 => {
            c.cmp(c.r64(12), 0x8u64, 64);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 428; continue; }
            blk = 373; continue;
        }
        373 => {
            c.cmp(c.r64(12), 0x9u64, 64);
            if c.cc_e() { blk = 428; continue; }
            blk = 374; continue;
        }
        374 => {
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_e() { blk = 385; continue; }
            blk = 375; continue;
        }
        375 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 440; continue; }
            blk = 376; continue;
        }
        376 => {
            c.cmp(c.r64(12), 0xcu64, 64);
            if c.cc_e() { blk = 383; continue; }
            blk = 377; continue;
        }
        377 => {
            c.cmp(c.r64(12), 0xeu64, 64);
            if c.cc_e() { blk = 379; continue; }
            blk = 378; continue;
        }
        378 => {
            let Some(t605) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000835a48); };
            c.w64(5, t605);
            let Some(t606) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x7000835a4c); };
            c.w64(10, t606);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000835a51); }
            let Some(t607) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000835a56); };
            c.w64(10, t607);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x7000835a5b); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(0)) { return RegfileExit::fault(0x7000835a60); }
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(1)) { return RegfileExit::fault(0x7000835a6a); }
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(9)) { return RegfileExit::fault(0x7000835a6f); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(3)) { return RegfileExit::fault(0x7000835a74); }
            let Some(t608) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000835a79); };
            c.w64(3, t608);
            let r609 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r609);
            return RegfileExit::jump(0x700086ed74);
        }
        379 => {
            let Some(t610) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835a89); };
            c.w32(8, t610);
            let r611 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r611);
            let Some(t612) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835a90); };
            c.w32(8, t612);
            let r613 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r613);
            let Some(t614) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835a97); };
            c.w32(9, t614);
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r615 = c.r64(11) & 0x1ffffffu64;
            c.w64(11, r615);
            c.logic(r615, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 381; continue; }
            blk = 380; continue;
        }
        380 => {
            return RegfileExit::fault(0x7000835ab1);
        }
        381 => {
            let Some(t616) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835ab2); };
            c.w32(8, t616);
            let r617 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r617);
            let Some(t618) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835aba); };
            c.w32(0, t618);
            let r619 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r619);
            let Some(t620) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000835ac1); };
            c.cmp(t620, c.r32(0), 32);
            if c.cc_ne() { blk = 342; continue; }
            blk = 382; continue;
        }
        382 => {
            let Some(t621) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000835ace); };
            c.w64(5, t621);
            let r622 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r622);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000835ad6); }
            let Some(t623) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000835ad7); };
            if !c.push64(t623) { return RegfileExit::fault(0x7000835ad7); }
            let Some(t624) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000835adb); };
            c.w64(3, t624);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x7000835ae7); }
            return RegfileExit::jump(0x700086ed74);
        }
        383 => {
            let Some(t625) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000835af5); };
            c.w64(5, t625);
            c.w32(2, 0x2u64);
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(1));
            let Some(t626) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000835b04); };
            c.w64(1, t626);
            let r627 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r627);
            return RegfileExit::jump(0x70008bc174);
        }
        384 => {
            blk = 440; continue;
        }
        385 => {
            let Some(t628) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835b17); };
            c.w32(8, t628);
            let r629 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r629);
            let Some(t630) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835b1e); };
            c.w32(11, t630);
            let Some(t631) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000835b22); };
            c.cmp(t631, c.r32(8), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 386; continue;
        }
        386 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 427; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t632) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835b39); };
            c.w32(11, t632);
            let r633 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r633);
            let Some(t634) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000835b40); };
            c.cmp(t634, c.r32(11), 32);
            if c.cc_ne() { blk = 410; continue; }
            blk = 388; continue;
        }
        388 => {
            let Some(t635) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835b4d); };
            c.w32(2, t635);
            let r636 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r636);
            let Some(t637) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835b54); };
            c.w32(11, t637);
            let r638 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r638);
            let Some(t639) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000835b5b); };
            c.logic(t639 & 0x2u64, 8);
            if c.cc_ne() { blk = 408; continue; }
            blk = 389; continue;
        }
        389 => {
            let Some(t640) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000835b66); };
            c.cmp(t640, c.r32(11), 32);
            if c.cc_e() { blk = 391; continue; }
            blk = 390; continue;
        }
        390 => {
            let Some(t641) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000835b6f); };
            c.w64(0, t641);
            blk = 20; continue;
        }
        391 => {
            let Some(t642) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000835b7b); };
            c.w64(11, t642);
            let Some(t643) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000835b82); };
            c.cmp(t643, 0x20u64, 8);
            if c.cc_e() { blk = 406; continue; }
            blk = 392; continue;
        }
        392 => {
            let Some(t644) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000835b8c); };
            c.w32(11, t644);
            let Some(t645) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000835b90); };
            c.cmp(t645, c.r32(11), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t646) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835b9d); };
            c.w32(11, t646);
            let r647 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r647);
            let Some(t648) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835ba4); };
            c.w32(12, t648);
            let Some(t649) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000835ba8); };
            c.cmp(t649, c.r32(12), 32);
            if c.cc_ne() { blk = 395; continue; }
            blk = 394; continue;
        }
        394 => {
            let Some(t650) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000835bb1); };
            c.w32(11, t650);
            let r651 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r651);
            let Some(t652) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835bb8); };
            c.w32(12, t652);
            let Some(t653) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000835bbc); };
            c.cmp(t653, c.r32(12), 32);
            if c.cc_e() { blk = 394; continue; }
            blk = 395; continue;
        }
        395 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 396; continue;
        }
        396 => {
            let Some(t654) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835bcf); };
            c.w32(12, t654);
            let r655 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r655);
            let Some(t656) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835bd6); };
            c.w32(15, t656);
            c.w32(6, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 401; continue; }
            blk = 397; continue;
        }
        397 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 400; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t657) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000835bf5); };
            c.cmp(t657, c.r32(12), 32);
            if c.cc_ne() { blk = 402; continue; }
            blk = 399; continue;
        }
        399 => {
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000835bfe); };
            c.w64(6, t658);
            blk = 74; continue;
        }
        400 => {
            c.w64(9, c.r64(8));
            blk = 402; continue;
        }
        401 => {
            c.w64(9, c.r64(11));
            blk = 402; continue;
        }
        402 => {
            let Some(t659) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835c0f); };
            c.w32(8, t659);
            let r660 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r660);
            let Some(t661) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835c16); };
            c.w32(8, t661);
            let r662 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r662);
            let Some(t663) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835c1d); };
            c.w32(9, t663);
            let r664 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r664);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 405; continue; }
            blk = 403; continue;
        }
        403 => {
            let Some(t665) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000835c2f); };
            c.w32(6, t665);
            let r666 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r666);
            let Some(t667) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000835c36); };
            if !c.push64(t667) { return RegfileExit::fault(0x7000835c36); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000835c40, 0x7000835c3b);
        }
        404 => {
            blk = 20; continue;
        }
        405 => {
            let Some(t668) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000835c45); };
            c.w64(6, t668);
            blk = 74; continue;
        }
        406 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000835c4e); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000835c50); }
            let Some(t669) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000835c51); };
            if !c.push64(t669) { return RegfileExit::fault(0x7000835c51); }
            let Some(t670) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000835c54); };
            c.w64(3, t670);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000835c65, 0x7000835c60);
        }
        407 => {
            blk = 20; continue;
        }
        408 => {
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000835c6a); };
            if !c.push64(t671) { return RegfileExit::fault(0x7000835c6a); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000835c77, 0x7000835c72);
        }
        409 => {
            blk = 20; continue;
        }
        410 => {
            let Some(t672) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835c7c); };
            c.w32(12, t672);
            let r673 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r673);
            let Some(t674) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835c83); };
            c.w32(15, t674);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 425; continue; }
            blk = 411; continue;
        }
        411 => {
            c.w32(2, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 417; continue; }
            blk = 412; continue;
        }
        412 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_ne() { blk = 74; continue; }
            blk = 413; continue;
        }
        413 => {
            let Some(t675) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835cad); };
            c.w32(9, t675);
            let Some(t676) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000835cb1); };
            c.cmp(t676, c.r32(9), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 414; continue;
        }
        414 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t677) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835cc8); };
            c.cmp(t677, 0x40u64, 16);
            if c.cc_ae() { blk = 74; continue; }
            blk = 416; continue;
        }
        416 => {
            let Some(t678) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835cd4); };
            c.w32(8, t678);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        417 => {
            let Some(t679) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835ce1); };
            c.w32(11, t679);
            let Some(t680) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000835ce5); };
            c.cmp(t680, c.r32(11), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 418; continue;
        }
        418 => {
            let Some(t681) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000835cf2); };
            c.w32(11, t681);
            c.w64(12, c.r64(11));
            let r682 = c.r32(12) & 0xc0u64;
            c.w32(12, r682);
            c.logic(r682, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 420; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t683) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000835d0b); };
            c.w32(8, t683);
            let r684 = c.r32(8) & 0x1fu64;
            c.w32(8, r684);
            c.logic(r684, 32);
            let r685 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r685);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_a() { blk = 74; continue; }
            blk = 420; continue;
        }
        420 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 74; continue; }
            blk = 421; continue;
        }
        421 => {
            let Some(t686) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000835d2b); };
            c.w32(8, t686);
            let r687 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r687);
            let Some(t688) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000835d32); };
            c.cmp(t688, c.r32(8), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t689) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835d3f); };
            c.w32(9, t689);
            let Some(t690) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000835d43); };
            c.cmp(t690, c.r32(9), 32);
            if c.cc_e() { blk = 424; continue; }
            blk = 423; continue;
        }
        423 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        424 => {
            let Some(t691) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835d54); };
            c.w32(0, t691);
            let r692 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r692);
            blk = 20; continue;
        }
        425 => {
            let Some(t693) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835d60); };
            c.w32(9, t693);
            let Some(t694) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000835d64); };
            c.cmp(t694, c.r32(9), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 426; continue;
        }
        426 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        427 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        428 => {
            let Some(t695) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000835d81); };
            c.w64(12, t695);
            let Some(t696) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x7000835d88); };
            c.cmp(t696, 0x20u64, 8);
            if c.cc_e() { blk = 440; continue; }
            blk = 429; continue;
        }
        429 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 431; continue; }
            blk = 430; continue;
        }
        430 => {
            let Some(t697) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835d99); };
            c.w32(8, t697);
            let r698 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r698);
            blk = 432; continue;
        }
        431 => {
            let Some(t699) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000835da2); };
            c.w32(8, t699);
            let r700 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r700);
            blk = 432; continue;
        }
        432 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 434; continue; }
            blk = 433; continue;
        }
        433 => {
            return RegfileExit::fault(0x7000835daf);
        }
        434 => {
            let r701 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r701);
            c.logic(r701, 64);
            let Some(t702) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835db4); };
            c.w32(11, t702);
            let r703 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r703);
            let Some(t704) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835dbb); };
            c.w32(3, t704);
            let r705 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r705);
            let Some(t706) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835dc2); };
            c.w64(2, t706);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 436; continue; }
            blk = 435; continue;
        }
        435 => {
            let Some(t707) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835dcc); };
            c.w32(9, t707);
            let r708 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r708);
            let Some(t709) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835dd3); };
            c.w32(7, t709);
            let r710 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r710);
            blk = 436; continue;
        }
        436 => {
            let Some(t711) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000835dda); };
            if !c.push64(t711) { return RegfileExit::fault(0x7000835dda); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x7000835de7, 0x7000835de2);
        }
        437 => {
            blk = 20; continue;
        }
        438 => {
            let Some(t712) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000835dec); };
            c.w64(8, t712);
            let Some(t713) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000835df3); };
            c.cmp(t713, 0x20u64, 8);
            if c.cc_e() { blk = 440; continue; }
            blk = 439; continue;
        }
        439 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r714 = c.r64(11) & 0x3ffu64;
            c.w64(11, r714);
            c.logic(r714, 64);
            let r715 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r715);
            let Some(t716) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835e08); };
            c.w32(11, t716);
            let r717 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r717);
            let Some(t718) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000835e0f); };
            c.w32(11, t718);
            let r719 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r719);
            let Some(t720) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000835e16); };
            c.w32(3, t720);
            let r721 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r721);
            let Some(t722) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000835e1e); };
            c.w64(5, t722);
            let Some(t723) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000835e22); };
            c.w64(2, t723);
            c.w64(1, c.r64(9));
            let r724 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r724);
            return RegfileExit::jump(0x70007fea14);
        }
        440 => {
            let Some(t725) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000835e33); };
            c.w64(5, t725);
            let Some(t726) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000835e37); };
            c.w64(3, t726);
            c.w32(0, 0x3u64);
            let Some(t727) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000835e43); };
            c.w64(10, t727);
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(10)) { return RegfileExit::fault(0x7000835e48); }
            let Some(t728) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x7000835e4d); };
            c.w64(10, t728);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x7000835e52); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(7)) { return RegfileExit::fault(0x7000835e57); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(1)) { return RegfileExit::fault(0x7000835e5c); }
            let r729 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r729);
            return RegfileExit::jump(0x700086ed74);
        }
        441 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r730 = c.r64(11) & 0x3ffu64;
            c.w64(11, r730);
            c.logic(r730, 64);
            let r731 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r731);
            let Some(t732) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835e79); };
            c.w32(9, t732);
            let r733 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r733);
            let Some(t734) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000835e80); };
            c.w32(9, t734);
            let r735 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r735);
            let Some(t736) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000835e87); };
            c.w32(8, t736);
            let r737 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r737);
            let Some(t738) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835e8f); };
            c.w32(7, t738);
            let r739 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r739);
            let Some(t740) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000835e96); };
            if !c.push64(t740) { return RegfileExit::fault(0x7000835e96); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000835ea0, 0x7000835e9b);
        }
        442 => {
            blk = 20; continue;
        }
        443 => {
            let Some(t741) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835ea5); };
            c.w32(8, t741);
            let r742 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r742);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 445; continue; }
            blk = 444; continue;
        }
        444 => {
            let Some(t743) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000835eb2); };
            c.w64(8, t743);
            blk = 445; continue;
        }
        445 => {
            let Some(t744) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835eb9); };
            c.w32(11, t744);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r745 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r745);
            let Some(t746) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835ec7); };
            c.w32(12, t746);
            let r747 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r747);
            let r748 = c.r64(12) & c.r64(11);
            c.w64(12, r748);
            c.logic(r748, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 446; continue;
        }
        446 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(2).wrapping_mul(4));
            let Some(t749) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835ee2); };
            c.w32(7, t749);
            let Some(t750) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000835ee7); };
            c.cmp(t750, c.r32(7), 32);
            if c.cc_e() { blk = 504; continue; }
            blk = 447; continue;
        }
        447 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 449; continue; }
            blk = 448; continue;
        }
        448 => {
            let r751 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r751);
            let r752 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r752);
            c.w64(2, c.r64(11));
            let r753 = c.r64(2) & c.r64(12);
            c.w64(2, r753);
            c.logic(r753, 64);
            c.w64(12, c.r64(2));
            blk = 446; continue;
        }
        449 => {
            let Some(t754) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835f0a); };
            c.w32(11, t754);
            let Some(t755) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000835f0e); };
            c.w32(12, t755);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 451; continue; }
            blk = 450; continue;
        }
        450 => {
            return RegfileExit::fault(0x7000835f22);
        }
        451 => {
            let Some(t756) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000835f23); };
            c.w32(8, t756);
            let r757 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r757);
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 503; continue; }
            blk = 452; continue;
        }
        452 => {
            let Some(t758) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835f35); };
            c.w32(11, t758);
            let Some(t759) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000835f39); };
            c.cmp(t759, c.r32(11), 32);
            if c.cc_ne() { blk = 477; continue; }
            blk = 453; continue;
        }
        453 => {
            let Some(t760) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000835f46); };
            c.w32(2, t760);
            let r761 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r761);
            let Some(t762) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835f4d); };
            c.w32(8, t762);
            let r763 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r763);
            let Some(t764) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000835f54); };
            c.logic(t764 & 0x2u64, 8);
            if c.cc_ne() { blk = 475; continue; }
            blk = 454; continue;
        }
        454 => {
            let Some(t765) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000835f5f); };
            c.cmp(t765, c.r32(8), 32);
            if c.cc_e() { blk = 456; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t766) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000835f68); };
            c.w64(0, t766);
            blk = 20; continue;
        }
        456 => {
            let Some(t767) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000835f74); };
            c.w64(8, t767);
            let Some(t768) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000835f7b); };
            c.cmp(t768, 0x20u64, 8);
            if c.cc_e() { blk = 473; continue; }
            blk = 457; continue;
        }
        457 => {
            let Some(t769) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000835f85); };
            c.w32(8, t769);
            let Some(t770) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000835f89); };
            c.cmp(t770, c.r32(8), 32);
            if c.cc_e() { blk = 459; continue; }
            blk = 458; continue;
        }
        458 => {
            let Some(t771) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000835f92); };
            c.w64(7, t771);
            let Some(t772) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000835f96); };
            c.w64(6, t772);
            blk = 74; continue;
        }
        459 => {
            let Some(t773) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835f9f); };
            c.w32(8, t773);
            let r774 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r774);
            let Some(t775) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835fa6); };
            c.w32(11, t775);
            let Some(t776) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000835faa); };
            c.cmp(t776, c.r32(11), 32);
            if c.cc_ne() { blk = 461; continue; }
            blk = 460; continue;
        }
        460 => {
            let Some(t777) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000835fb3); };
            c.w32(8, t777);
            let r778 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r778);
            let Some(t779) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835fba); };
            c.w32(11, t779);
            let Some(t780) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000835fbe); };
            c.cmp(t780, c.r32(11), 32);
            if c.cc_e() { blk = 460; continue; }
            blk = 461; continue;
        }
        461 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 472; continue; }
            blk = 462; continue;
        }
        462 => {
            let Some(t781) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000835fd1); };
            c.w32(11, t781);
            let r782 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r782);
            let Some(t783) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000835fd8); };
            c.w32(12, t783);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 467; continue; }
            blk = 463; continue;
        }
        463 => {
            c.w32(8, c.r16(12));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 466; continue; }
            blk = 464; continue;
        }
        464 => {
            let Some(t784) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000835ff8); };
            c.cmp(t784, c.r32(11), 32);
            if c.cc_ne() { blk = 468; continue; }
            blk = 465; continue;
        }
        465 => {
            let Some(t785) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836001); };
            c.w64(7, t785);
            let Some(t786) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836005); };
            c.w64(6, t786);
            blk = 74; continue;
        }
        466 => {
            let Some(t787) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083600e); };
            c.w32(11, t787);
            let r788 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r788);
            c.w64(9, c.r64(11));
            blk = 468; continue;
        }
        467 => {
            c.w64(9, c.r64(8));
            blk = 468; continue;
        }
        468 => {
            let Some(t789) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083601d); };
            c.w32(8, t789);
            let r790 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r790);
            let Some(t791) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836024); };
            c.w32(8, t791);
            let r792 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r792);
            let Some(t793) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083602b); };
            c.w32(9, t793);
            let r794 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r794);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 471; continue; }
            blk = 469; continue;
        }
        469 => {
            let Some(t795) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083603d); };
            c.w32(6, t795);
            let r796 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r796);
            let Some(t797) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000836044); };
            if !c.push64(t797) { return RegfileExit::fault(0x7000836044); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700083604e, 0x7000836049);
        }
        470 => {
            blk = 20; continue;
        }
        471 => {
            let Some(t798) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836053); };
            c.w64(7, t798);
            let Some(t799) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836057); };
            c.w64(6, t799);
            blk = 74; continue;
        }
        472 => {
            let Some(t800) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836060); };
            c.w64(7, t800);
            let Some(t801) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836064); };
            c.w64(6, t801);
            blk = 74; continue;
        }
        473 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700083606d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083606f); }
            let Some(t802) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000836070); };
            if !c.push64(t802) { return RegfileExit::fault(0x7000836070); }
            let Some(t803) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000836073); };
            c.w64(3, t803);
            c.w32(0, 0x3u64);
            let Some(t804) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083607f); };
            c.w64(6, t804);
            return c.call(0x700086ed74, 0x7000836088, 0x7000836083);
        }
        474 => {
            blk = 20; continue;
        }
        475 => {
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083608d); };
            if !c.push64(t805) { return RegfileExit::fault(0x700083608d); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t806) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836095); };
            c.w64(6, t806);
            return c.call(0x70007f2274, 0x700083609e, 0x7000836099);
        }
        476 => {
            blk = 20; continue;
        }
        477 => {
            let Some(t807) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008360a3); };
            c.w32(11, t807);
            let r808 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r808);
            let Some(t809) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008360aa); };
            c.w32(12, t809);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 500; continue; }
            blk = 478; continue;
        }
        478 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 488; continue; }
            blk = 479; continue;
        }
        479 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_e() { blk = 481; continue; }
            blk = 480; continue;
        }
        480 => {
            let Some(t810) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008360d5); };
            c.w64(7, t810);
            let Some(t811) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008360d9); };
            c.w64(6, t811);
            blk = 74; continue;
        }
        481 => {
            let Some(t812) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008360e2); };
            c.w32(8, t812);
            let Some(t813) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008360e6); };
            c.cmp(t813, c.r32(8), 32);
            if c.cc_e() { blk = 483; continue; }
            blk = 482; continue;
        }
        482 => {
            let Some(t814) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008360ef); };
            c.w64(7, t814);
            let Some(t815) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008360f3); };
            c.w64(6, t815);
            blk = 74; continue;
        }
        483 => {
            let Some(t816) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008360fc); };
            c.w32(8, t816);
            let r817 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r817);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 485; continue; }
            blk = 484; continue;
        }
        484 => {
            let Some(t818) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836109); };
            c.w64(7, t818);
            let Some(t819) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083610d); };
            c.w64(6, t819);
            blk = 74; continue;
        }
        485 => {
            let Some(t820) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836116); };
            c.w32(9, t820);
            let r821 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r821);
            let Some(t822) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083611d); };
            c.cmp(t822, 0x40u64, 16);
            if c.cc_b() { blk = 487; continue; }
            blk = 486; continue;
        }
        486 => {
            let Some(t823) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836125); };
            c.w64(7, t823);
            let Some(t824) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836129); };
            c.w64(6, t824);
            blk = 74; continue;
        }
        487 => {
            let Some(t825) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836132); };
            c.w32(8, t825);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 20; continue;
        }
        488 => {
            let Some(t826) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083613f); };
            c.w32(8, t826);
            let Some(t827) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000836143); };
            c.cmp(t827, c.r32(8), 32);
            if c.cc_e() { blk = 490; continue; }
            blk = 489; continue;
        }
        489 => {
            let Some(t828) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083614c); };
            c.w64(7, t828);
            let Some(t829) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836150); };
            c.w64(6, t829);
            blk = 74; continue;
        }
        490 => {
            let Some(t830) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000836159); };
            c.w32(8, t830);
            c.w64(11, c.r64(8));
            let r831 = c.r32(11) & 0xc0u64;
            c.w32(11, r831);
            c.logic(r831, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 493; continue; }
            blk = 491; continue;
        }
        491 => {
            let Some(t832) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836171); };
            c.w32(11, t832);
            let r833 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r833);
            let Some(t834) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000836178); };
            c.w32(11, t834);
            let r835 = c.r32(11) & 0x1fu64;
            c.w32(11, r835);
            c.logic(r835, 32);
            let r836 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r836);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 493; continue; }
            blk = 492; continue;
        }
        492 => {
            let Some(t837) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083618a); };
            c.w64(7, t837);
            let Some(t838) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083618e); };
            c.w64(6, t838);
            blk = 74; continue;
        }
        493 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 499; continue; }
            blk = 494; continue;
        }
        494 => {
            let Some(t839) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083619d); };
            c.w32(8, t839);
            let r840 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r840);
            let Some(t841) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008361a4); };
            c.cmp(t841, c.r32(8), 32);
            if c.cc_e() { blk = 498; continue; }
            blk = 495; continue;
        }
        495 => {
            let Some(t842) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008361ad); };
            c.w32(9, t842);
            let Some(t843) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008361b1); };
            c.cmp(t843, c.r32(9), 32);
            if c.cc_e() { blk = 497; continue; }
            blk = 496; continue;
        }
        496 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        497 => {
            let Some(t844) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008361c2); };
            c.w32(0, t844);
            let r845 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r845);
            blk = 20; continue;
        }
        498 => {
            let Some(t846) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008361ce); };
            c.w64(7, t846);
            let Some(t847) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008361d2); };
            c.w64(6, t847);
            blk = 74; continue;
        }
        499 => {
            let Some(t848) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008361db); };
            c.w64(7, t848);
            let Some(t849) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008361df); };
            c.w64(6, t849);
            blk = 74; continue;
        }
        500 => {
            let Some(t850) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008361e8); };
            c.w32(8, t850);
            let Some(t851) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008361ec); };
            c.cmp(t851, c.r32(8), 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 501; continue;
        }
        501 => {
            let Some(t852) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008361f5); };
            c.w64(7, t852);
            let Some(t853) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008361f9); };
            c.w64(6, t853);
            blk = 74; continue;
        }
        502 => {
            let Some(t854) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836202); };
            c.w32(0, t854);
            let r855 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r855);
            blk = 20; continue;
        }
        503 => {
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        504 => {
            let Some(t856) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836216); };
            c.w64(7, t856);
            let Some(t857) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083621a); };
            c.w64(6, t857);
            blk = 74; continue;
        }
        505 => {
            return RegfileExit::fault(0x7000836223);
        }
        506 => {
            let Some(t858) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000836224); };
            c.w64(9, t858);
            let Some(t859) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083622b); };
            c.w64(11, t859);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000836232); }
            let Some(t860) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000836236); };
            c.w64(15, t860);
            let Some(t861) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083623d); };
            c.cmp(t861, c.r64(12), 64);
            if c.cc_a() { blk = 509; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700083624c, 0x7000836247);
        }
        508 => {
            let Some(t862) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083624c); };
            c.w64(8, t862);
            blk = 510; continue;
        }
        509 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000836252); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 510; continue;
        }
        510 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t863) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000836265); };
            c.w64(8, t863);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083626c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000836270); }
            blk = 20; continue;
        }
        511 => {
            let Some(t864) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083627a); };
            c.w64(9, t864);
            let Some(t865) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000836281); };
            c.w64(11, t865);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000836288); }
            let Some(t866) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083628c); };
            c.w64(15, t866);
            let Some(t867) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000836293); };
            c.cmp(t867, c.r64(12), 64);
            if c.cc_a() { blk = 514; continue; }
            blk = 512; continue;
        }
        512 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008362a2, 0x700083629d);
        }
        513 => {
            let Some(t868) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008362a2); };
            c.w64(8, t868);
            blk = 515; continue;
        }
        514 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008362a8); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 515; continue;
        }
        515 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t869) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008362be); };
            c.w64(8, t869);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008362c5); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008362c9); }
            blk = 20; continue;
        }
        516 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008362d8); }
            return c.call(0x70007febb4, 0x70008362e1, 0x70008362dc);
        }
        517 => {
            c.w64(9, c.r64(0));
            let Some(t870) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008362e4); };
            c.w64(8, t870);
            blk = 62; continue;
        }
        518 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008362f7, 0x70008362f2);
        }
        519 => {
            c.w64(8, c.r64(0));
            blk = 67; continue;
        }
        520 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000836304); }
            return c.call(0x70007febf4, 0x700083630e, 0x7000836309);
        }
        521 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000836311); }
            blk = 72; continue;
        }
        522 => {
            let Some(t871) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700083631b); };
            c.w64(8, t871);
            let Some(t872) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836322); };
            c.w32(8, t872);
            let r873 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r873);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000836329); }
            let Some(t874) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700083632b); };
            c.w64(3, t874);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700083633e, 0x7000836339);
        }
        523 => {
            return RegfileExit::fault(0x700083633e);
        }
        524 => {
            blk = 74; continue;
        }
        525 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000836349); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700083634d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000836351); }
            return c.call(0x70007febf4, 0x700083635b, 0x7000836356);
        }
        526 => {
            c.w64(8, c.r64(0));
            let Some(t875) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083635e); };
            c.w64(12, t875);
            let Some(t876) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000836362); };
            c.w64(1, t876);
            let Some(t877) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000836366); };
            c.w64(0, t877);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700083636a); }
            blk = 107; continue;
        }
        527 => {
            return RegfileExit::fault(0x7000836374);
        }
        528 => {
            blk = 266; continue;
        }
        529 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083637a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083637b); }
            let Some(t878) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083637c); };
            if !c.push64(t878) { return RegfileExit::fault(0x700083637c); }
            let Some(t879) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083637f); };
            c.w64(3, t879);
            c.w32(0, 0x3u64);
            let Some(t880) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083638b); };
            c.w64(6, t880);
            return c.call(0x700086ed74, 0x7000836394, 0x700083638f);
        }
        530 => {
            blk = 20; continue;
        }
        531 => {
            let Some(t881) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836399); };
            c.w64(0, t881);
            blk = 266; continue;
        }
        532 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008363a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008363ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008363af); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008363b3); }
            return c.call(0x70007febf4, 0x70008363bd, 0x70008363b8);
        }
        533 => {
            c.w64(9, c.r64(0));
            let Some(t882) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008363c0); };
            c.w64(1, t882);
            let Some(t883) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008363c4); };
            c.w64(8, t883);
            let Some(t884) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008363c8); };
            c.w64(3, t884);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008363cc); }
            blk = 217; continue;
        }
        534 => {
            return RegfileExit::fault(0x70008363d6);
        }
        535 => {
            blk = 260; continue;
        }
        536 => {
            return RegfileExit::fault(0x70008363dc);
        }
        537 => {
            blk = 260; continue;
        }
        538 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008363e2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008363e4); }
            let Some(t885) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008363e5); };
            if !c.push64(t885) { return RegfileExit::fault(0x70008363e5); }
            let Some(t886) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008363e8); };
            c.w64(3, t886);
            c.w32(0, 0x3u64);
            let Some(t887) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008363f4); };
            c.w64(6, t887);
            return c.call(0x700086ed74, 0x70008363fd, 0x70008363f8);
        }
        539 => {
            blk = 20; continue;
        }
        540 => {
            let Some(t888) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836402); };
            c.w64(0, t888);
            blk = 266; continue;
        }
        541 => {
            return RegfileExit::jump(0x700083640c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000836414(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836417); };
            c.w64(9, t0);
            let Some(t1) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700083641b); };
            c.w64(8, t1);
            c.w64(6, c.r64(8));
            c.w64(3, c.r64(9));
            return RegfileExit::jump(0x7000834434);
        }
        1 => {
            return RegfileExit::jump(0x700083642c);
        }
            _ => unreachable!(),
        }
    }
}
