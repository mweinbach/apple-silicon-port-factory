//! Small array-search helpers. Names and the `r::` placement are inferred
//! (fndb has no namespace for them; the exe has no symbols).

/// Index of the first `value` among `count` u16 elements at `data`, or
/// 0xffffffff if absent.
///
/// Original: 0x142489ee0 (101 bytes, leaf, 5295 call sites). AVX2: it compares
/// 16 elements at a time with `vpcmpeqw` over `count & ~15` elements, then
/// does a scalar tail. That equals a first-match scan. Return value width:
/// SIMD hits return a 32-bit index, the scalar tail returns its 64-bit
/// counter, and a miss returns `mov eax, 0xffffffff` (zero-extended). All
/// three agree for counts below 2^32. The owner is Remedy by position and
/// callers (no `.pdata` entry: it's a leaf found by gap scan).
///
/// Loads are plain (not the TSO `exe::rd` acquire loads): the original's 32-byte
/// `vmovdqu` loads aren't single-copy atomic either, and a read-only scan has no
/// ordering of its own to preserve. On aarch64 it compares 8 lanes at a time.
pub unsafe extern "win64" fn find_u16(data: *const u16, count: u64, value: u16) -> u64 {
    let simd_end = count & !15;
    let mut i = 0u64;
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::*;
        let needle = vdupq_n_u16(value);
        while i < simd_end {
            let v: uint16x8_t = core::ptr::read_unaligned(data.wrapping_add(i as usize) as *const uint16x8_t);
            // 0xffff per matching lane -> 0xff per lane after the narrowing shift.
            let m = vget_lane_u64::<0>(vreinterpret_u64_u8(vshrn_n_u16::<4>(vceqq_u16(v, needle))));
            if m != 0 {
                return ((i + (m.trailing_zeros() / 8) as u64) as u32) as u64;
            }
            i += 8;
        }
    }
    while i < simd_end {
        if core::ptr::read_unaligned(data.wrapping_add(i as usize)) == value {
            return (i as u32) as u64;
        }
        i += 1;
    }
    let mut j = simd_end;
    while j < count {
        if core::ptr::read_unaligned(data.wrapping_add(j as usize)) == value {
            return j;
        }
        j += 1;
    }
    0xffff_ffff
}

/// Sort `count` u64 keys at `keys` ascending, in place.
///
/// Original: 0x140fae550 (2253 bytes, `void(u64* keys, int count)`). It is an LSD
/// radix sort over all 64 bits in six passes (digits of 10, 10, 11, 11, 11, 11 bits:
/// bits 0-9, 10-19, 20-30, 31-41, 42-52, 53-63). It builds all six histograms in
/// one pass, ping-pongs between `keys` and a scratch buffer from
/// `sub_14241c0a0(count*8, 8)` (zeroed with `j_memset`, freed with
/// `sub_14241c190`), and ends in `keys` after the even number of passes. Scatter
/// loops are unrolled by four with a scalar tail of `count % 4`. A stable radix
/// sort over every bit of a plain integer key is the ascending order, and equal
/// keys can't be told apart, so an unstable sort writes identical memory. The
/// scratch buffer never escapes, so the allocator calls are not reproduced.
/// `count <= 0` does nothing (`count == 0` skips the allocation in the original;
/// a negative count is never passed: it would ask for a negative-size buffer).
pub unsafe extern "win64" fn radix_sort_u64(keys: *mut u64, count: i32) {
    if count <= 1 {
        return;
    }
    let n = count as usize;
    // Game memory may be under-aligned: copy out, sort, copy back (byte-wise copies).
    let mut v: Vec<u64> = Vec::with_capacity(n);
    core::ptr::copy_nonoverlapping(keys as *const u8, v.as_mut_ptr() as *mut u8, n * 8);
    v.set_len(n);
    v.sort_unstable();
    core::ptr::copy_nonoverlapping(v.as_ptr() as *const u8, keys as *mut u8, n * 8);
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

use crate::diff_eq;
use crate::difftest::Rng;

pub unsafe fn diff_find_u16(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u16, u64, u16) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x142489ee0);
    const CASES: u64 = 3000;
    for i in 0..CASES {
        let len = match i % 4 {
            0 => r.below(17),
            1 => 16 * r.below(8),
            _ => r.below(200),
        } as usize;
        // Small alphabet so matches occur at all positions (incl. duplicates).
        let alpha = 1 + r.below(if i % 3 == 0 { 4 } else { 300 }) as u16;
        let v: Vec<u16> = (0..len).map(|_| (r.below(alpha as u64) as u16).wrapping_mul(0x101)).collect();
        let value = match r.below(3) {
            0 if len > 0 => v[r.below(len as u64) as usize],
            1 => 0x8000 | r.next_u32() as u16,
            _ => (r.below(alpha as u64) as u16).wrapping_mul(0x101),
        };
        // Every other case at an odd address: the original's vpcmpeqw / movzx tolerate it.
        let off = (i % 2) as usize;
        let mut buf = vec![0u8; 2 * len + 2];
        for (k, x) in v.iter().enumerate() {
            buf[off + 2 * k..off + 2 * k + 2].copy_from_slice(&x.to_le_bytes());
        }
        let p = buf.as_ptr().wrapping_add(off) as *const u16;
        let a = o(p, len as u64, value);
        let b = n(p, len as u64, value);
        diff_eq!(a, b, "case {i}: len {len} value {value:#x} offset {off}");
    }
    Ok(CASES as u32)
}

/// Test fixture standing in for the engine allocator `sub_14241c0a0(size, align)`
/// (align <= 16; the requested size is kept in a 16-byte header for the free).
unsafe extern "win64" fn fixture_alloc(size: u64, _align: u64) -> *mut u8 {
    let total = size as usize + 16;
    let p = std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(total, 16));
    if p.is_null() {
        return p;
    }
    (p as *mut usize).write(total);
    p.add(16)
}

/// Test fixture standing in for the engine free `sub_14241c190(ptr)`.
unsafe extern "win64" fn fixture_free(p: *mut u8) {
    if p.is_null() {
        return;
    }
    let base = p.sub(16);
    let total = (base as *const usize).read();
    std::alloc::dealloc(base, std::alloc::Layout::from_size_align_unchecked(total, 16));
}

pub unsafe fn diff_radix_sort_u64(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Mock;
    type F = unsafe extern "win64" fn(*mut u64, i32);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _alloc = Mock::new(0x1_4241_c0a0, fixture_alloc as *const ())?;
    let _free = Mock::new(0x1_4241_c190, fixture_free as *const ())?;
    let mut r = Rng::new(0x140fae550);
    const CASES: u64 = 400;
    for i in 0..CASES {
        let len = match i % 5 {
            0 => r.below(9),
            1 => r.below(64),
            2 => 1000 + r.below(3000),
            _ => r.below(700),
        } as usize;
        // Mix full-range keys, tiny alphabets (duplicates) and keys that differ only in
        // one radix digit, so every pass and the count%4 tail are exercised.
        let mode = r.below(4);
        let keys: Vec<u64> = (0..len)
            .map(|_| match mode {
                0 => r.next_u64(),
                1 => r.below(5),
                2 => r.next_u64() & (0x7ffu64 << (r.below(6) * 10 + 3)),
                _ => (r.below(16) << 60) | r.below(3),
            })
            .collect();
        // Every other case at a 4-byte (under-aligned) address.
        let off = if i % 2 == 1 { 4 } else { 0 };
        let mut bo = vec![0u8; len * 8 + 16];
        let mut bn = vec![0u8; len * 8 + 16];
        for (k, x) in keys.iter().enumerate() {
            bo[off + 8 * k..off + 8 * k + 8].copy_from_slice(&x.to_le_bytes());
        }
        bn.copy_from_slice(&bo);
        o(bo.as_mut_ptr().add(off) as *mut u64, len as i32);
        n(bn.as_mut_ptr().add(off) as *mut u64, len as i32);
        if bo != bn {
            let k = bo.iter().zip(&bn).position(|(a, b)| a != b).unwrap_or(0);
            return Err(format!("mismatch: case {i}: len {len} mode {mode} offset {off}: first differing byte {k}"));
        }
    }
    Ok(CASES as u32)
}
