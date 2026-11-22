//! CityHash64 (Google CityHash v1.1), as inlined into Remedy's resource-path hash
//! (0x142423da0). Third-party algorithm. It is kept as a separate native helper
//! because the original inlines all four length classes (`HashLen0to16`,
//! `17to32`, `33to64`, and the 64-byte block loop) and the constants match
//! upstream exactly: k0 = 0xc3a5c85c97cb3127, k1 = 0xb492b66fbe98f273,
//! k2 = 0x9ae16a3b2f90404f, kMul = 0x9ddfea08eb382d69.
//!
//! Reads go through [`rd`] (unaligned loads; the input can be any game buffer).

use crate::exe::rd;

const K0: u64 = 0xc3a5_c85c_97cb_3127;
const K1: u64 = 0xb492_b66f_be98_f273;
const K2: u64 = 0x9ae1_6a3b_2f90_404f;
const KMUL: u64 = 0x9ddf_ea08_eb38_2d69;

#[inline(always)]
unsafe fn f64_(p: usize) -> u64 {
    rd::<u64>(p)
}
#[inline(always)]
unsafe fn f32_(p: usize) -> u64 {
    rd::<u32>(p) as u64
}
#[inline(always)]
fn shift_mix(v: u64) -> u64 {
    v ^ (v >> 47)
}
#[inline(always)]
fn hash_len16_mul(u: u64, v: u64, mul: u64) -> u64 {
    let mut a = (u ^ v).wrapping_mul(mul);
    a ^= a >> 47;
    let mut b = (v ^ a).wrapping_mul(mul);
    b ^= b >> 47;
    b.wrapping_mul(mul)
}
#[inline(always)]
fn hash_len16(u: u64, v: u64) -> u64 {
    hash_len16_mul(u, v, KMUL)
}

unsafe fn hash_len0to16(s: usize, len: u64) -> u64 {
    if len >= 8 {
        let mul = K2.wrapping_add(len.wrapping_mul(2));
        let a = f64_(s).wrapping_add(K2);
        let b = f64_(s + len as usize - 8);
        let c = b.rotate_right(37).wrapping_mul(mul).wrapping_add(a);
        let d = a.rotate_right(25).wrapping_add(b).wrapping_mul(mul);
        return hash_len16_mul(c, d, mul);
    }
    if len >= 4 {
        let mul = K2.wrapping_add(len.wrapping_mul(2));
        let a = f32_(s);
        return hash_len16_mul(len.wrapping_add(a << 3), f32_(s + len as usize - 4), mul);
    }
    if len > 0 {
        let a = rd::<u8>(s) as u32;
        let b = rd::<u8>(s + (len >> 1) as usize) as u32;
        let c = rd::<u8>(s + len as usize - 1) as u32;
        let y = a.wrapping_add(b << 8);
        let z = (len as u32).wrapping_add(c << 2);
        return shift_mix((y as u64).wrapping_mul(K2) ^ (z as u64).wrapping_mul(K0)).wrapping_mul(K2);
    }
    K2
}

unsafe fn hash_len17to32(s: usize, len: u64) -> u64 {
    let l = len as usize;
    let mul = K2.wrapping_add(len.wrapping_mul(2));
    let a = f64_(s).wrapping_mul(K1);
    let b = f64_(s + 8);
    let c = f64_(s + l - 8).wrapping_mul(mul);
    let d = f64_(s + l - 16).wrapping_mul(K2);
    hash_len16_mul(
        a.wrapping_add(b).rotate_right(43).wrapping_add(c.rotate_right(30)).wrapping_add(d),
        a.wrapping_add(b.wrapping_add(K2).rotate_right(18)).wrapping_add(c),
        mul,
    )
}

unsafe fn hash_len33to64(s: usize, len: u64) -> u64 {
    let l = len as usize;
    let mul = K2.wrapping_add(len.wrapping_mul(2));
    let mut a = f64_(s).wrapping_mul(K2);
    let mut b = f64_(s + 8);
    let c = f64_(s + l - 24);
    let d = f64_(s + l - 32);
    let e = f64_(s + 16).wrapping_mul(K2);
    let f = f64_(s + 24).wrapping_mul(9);
    let g = f64_(s + l - 8);
    let h = f64_(s + l - 16).wrapping_mul(mul);
    let u = a.wrapping_add(g).rotate_right(43).wrapping_add(b.rotate_right(30).wrapping_add(c).wrapping_mul(9));
    let v = (a.wrapping_add(g) ^ d).wrapping_add(f).wrapping_add(1);
    let w = u.wrapping_add(v).wrapping_mul(mul).swap_bytes().wrapping_add(h);
    let x = e.wrapping_add(f).rotate_right(42).wrapping_add(c);
    let y = v.wrapping_add(w).wrapping_mul(mul).swap_bytes().wrapping_add(g).wrapping_mul(mul);
    let z = e.wrapping_add(f).wrapping_add(c);
    a = x.wrapping_add(z).wrapping_mul(mul).wrapping_add(y).swap_bytes().wrapping_add(b);
    b = shift_mix(z.wrapping_add(a).wrapping_mul(mul).wrapping_add(d).wrapping_add(h)).wrapping_mul(mul);
    b.wrapping_add(x)
}

#[inline(always)]
fn weak_hash_len32_with_seeds_raw(w: u64, x: u64, y: u64, z: u64, mut a: u64, mut b: u64) -> (u64, u64) {
    a = a.wrapping_add(w);
    b = b.wrapping_add(a).wrapping_add(z).rotate_right(21);
    let c = a;
    a = a.wrapping_add(x);
    a = a.wrapping_add(y);
    b = b.wrapping_add(a.rotate_right(44));
    (a.wrapping_add(z), b.wrapping_add(c))
}

#[inline(always)]
unsafe fn weak_hash_len32_with_seeds(s: usize, a: u64, b: u64) -> (u64, u64) {
    weak_hash_len32_with_seeds_raw(f64_(s), f64_(s + 8), f64_(s + 16), f64_(s + 24), a, b)
}

/// `CityHash64(s, len)` (v1.1).
pub unsafe fn city_hash64(s: usize, len: u64) -> u64 {
    if len <= 32 {
        return if len <= 16 { hash_len0to16(s, len) } else { hash_len17to32(s, len) };
    }
    if len <= 64 {
        return hash_len33to64(s, len);
    }
    let l = len as usize;
    let mut x = f64_(s + l - 40);
    let mut y = f64_(s + l - 16).wrapping_add(f64_(s + l - 56));
    let mut z = hash_len16(f64_(s + l - 48).wrapping_add(len), f64_(s + l - 24));
    let mut v = weak_hash_len32_with_seeds(s + l - 64, len, z);
    let mut w = weak_hash_len32_with_seeds(s + l - 32, y.wrapping_add(K1), x);
    x = x.wrapping_mul(K1).wrapping_add(f64_(s));
    let mut rem = (len - 1) & !63u64;
    let mut p = s;
    loop {
        x = x.wrapping_add(y).wrapping_add(v.0).wrapping_add(f64_(p + 8)).rotate_right(37).wrapping_mul(K1);
        y = y.wrapping_add(v.1).wrapping_add(f64_(p + 48)).rotate_right(42).wrapping_mul(K1);
        x ^= w.1;
        y = y.wrapping_add(v.0).wrapping_add(f64_(p + 40));
        z = z.wrapping_add(w.0).rotate_right(33).wrapping_mul(K1);
        v = weak_hash_len32_with_seeds(p, v.1.wrapping_mul(K1), x.wrapping_add(w.0));
        w = weak_hash_len32_with_seeds(p + 32, z.wrapping_add(w.1), y.wrapping_add(f64_(p + 16)));
        std::mem::swap(&mut z, &mut x);
        p += 64;
        rem -= 64;
        if rem == 0 {
            break;
        }
    }
    hash_len16(
        hash_len16(v.0, w.0).wrapping_add(shift_mix(y).wrapping_mul(K1)).wrapping_add(z),
        hash_len16(v.1, w.1).wrapping_add(x),
    )
}
