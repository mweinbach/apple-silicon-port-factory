//! `r::Pack2FileSystem` directory-tree lookups.
//!
//! The pack index keeps, per directory node, a contiguous run of child
//! directories and a run of files. Both runs are sorted by name under the
//! engine's path-character fold table at 0x1426d01e0 (case/separator folding,
//! compared as signed bytes). Lookups are a `lower_bound` binary search
//! followed by an exact (folded) equality check.
//!
//! Layout (offsets from the file-system object `fs`):
//! - `fs+0x18`: child-directory entries, 0x1c bytes each: name offset u32 @+0x14,
//!   name length u32 @+0x18.
//! - `fs+0x28`: file entries, 0x20 bytes each: name offset u32 @+0x0c, name
//!   length u32 @+0x10.
//! - `fs+0x48`: string pool base (name offsets are relative to it).
//!
//! Directory node: first child dir u32 @+0x04, child dir count u32 @+0x08,
//! first file u32 @+0x0c, file count u32 @+0x10.
//!
//! The key is a `{ const char* ptr; u64 len }` string view.

use crate::exe::rd;

/// Path-character fold table (256 bytes) in the exe's .rdata.
const FOLD_VA: u64 = 0x1_426d_01e0;

#[inline(always)]
unsafe fn fold(t: *const u8, c: u8) -> i8 {
    rd::<u8>(t as usize + c as usize) as i8
}

/// Shared body of both lookups (`stride`/`name_off`/`len_off` select the entry kind).
#[inline(always)]
unsafe fn lookup_sorted(
    first_entry: u64,
    count: u64,
    stride: u64,
    name_off: u64,
    len_off: u64,
    pool: u64,
    key: u64,
) -> u64 {
    let t = crate::exe::ptr::<u8>(FOLD_VA);
    let key_ptr = rd::<u64>(key as usize);
    let key_len = rd::<u64>(key as usize + 8);
    let name_of = |e: u64| -> (u64, u64) {
        (pool.wrapping_add(rd::<u32>((e + name_off) as usize) as u64), rd::<u32>((e + len_off) as usize) as u64)
    };

    // lower_bound: first entry that is not less than the key.
    let mut lo = first_entry;
    let mut n = count as i64;
    while n > 0 {
        let half = (n as u64) >> 1;
        let mid = lo + half * stride;
        let (ep, el) = name_of(mid);
        let m = el.min(key_len);
        let mut ord = 0i32;
        for i in 0..m {
            let a = fold(t, rd::<u8>((ep + i) as usize));
            let b = fold(t, rd::<u8>((key_ptr + i) as usize));
            if a < b {
                ord = -1;
                break;
            }
            if a > b {
                ord = 1;
                break;
            }
        }
        let less = if ord != 0 { ord < 0 } else { el < key_len };
        if less {
            n = n - 1 - half as i64;
            lo = mid + stride;
        } else {
            n = half as i64;
        }
    }

    let end = first_entry + count * stride;
    if lo == end {
        return 0;
    }
    let (ep, el) = name_of(lo);
    if el != key_len {
        return 0;
    }
    for i in 0..el {
        if fold(t, rd::<u8>((ep + i) as usize)) != fold(t, rd::<u8>((key_ptr + i) as usize)) {
            return 0;
        }
    }
    lo
}

/// Find the child directory of `node` named `key` (folded compare); returns the
/// 0x1c-byte directory entry or null.
///
/// Original: 0x142432a80 (464 bytes, leaf). Callers: 0x142432c60 (walks a path
/// component by component). Binary search exactly as in the original: entry
/// `[fs+0x18] + node.first_dir*0x1c`, count `node.dir_count`, `jl`/`jg` byte
/// compares of folded characters, then the length tie-break.
pub unsafe extern "win64" fn pack2_find_dir(fs: u64, node: u64, key: u64) -> u64 {
    let first = rd::<u64>((fs + 0x18) as usize).wrapping_add(rd::<u32>((node + 4) as usize) as u64 * 0x1c);
    let count = rd::<u32>((node + 8) as usize) as u64;
    lookup_sorted(first, count, 0x1c, 0x14, 0x18, rd::<u64>((fs + 0x48) as usize), key)
}

/// Find the file of directory `node` named `key` (folded compare); returns the
/// 0x20-byte file entry or null.
///
/// Original: 0x142432790 (405 bytes, leaf). Callers include
/// `r::Pack2FileSystem::vfunc_6` (0x142436c80) and 0x142432930. Same search as
/// [`pack2_find_dir`] over `[fs+0x28] + node.first_file*0x20`, count `node.file_count`.
pub unsafe extern "win64" fn pack2_find_file(fs: u64, node: u64, key: u64) -> u64 {
    let first = rd::<u64>((fs + 0x28) as usize).wrapping_add(rd::<u32>((node + 0xc) as usize) as u64 * 0x20);
    let count = rd::<u32>((node + 0x10) as usize) as u64;
    lookup_sorted(first, count, 0x20, 0x0c, 0x10, rd::<u64>((fs + 0x48) as usize), key)
}

/// `std::string_view` `{ "/\\", 2 }` in .rdata (separator set).
const SEPS_VIEW_VA: u64 = 0x1_4307_9458;
/// MSVC STL `_Traits_find_first_not_of` with the fold traits; third-party, called as is.
const FIND_FIRST_NOT_OF_VA: u64 = 0x1_401e_1a40;
/// `std::_Xout_of_range` (throws).
const XOUT_OF_RANGE_VA: u64 = 0x1_401d_e590;
/// [`pack2_find_dir`] (called through its address like the original).
const FIND_DIR_VA: u64 = 0x1_4243_2a80;

/// Resolve directory path `path` (a string view) from the root directory entry
/// `[fs+0x18]`; returns the directory entry or null if a component is missing.
///
/// Original: 0x142432c60 (535 bytes; callers include `r::Pack2FileSystem`
/// vfuncs 6, 7 and 28). Components are split like the path helpers: a leading
/// `/`, `\` or `C:` is skipped, `find_first_not_of(seps, pos)` (0x1401e1a40)
/// finds each start, and an inlined scan for either separator character
/// (folded compare, separator pointer read once from the view at 0x143079458)
/// finds its end. A component that is exactly `"."` (raw byte compare) is
/// skipped; every other one goes through 0x142432a80 from the current node.
pub unsafe extern "win64" fn pack2_resolve_dir(fs: u64, path: u64) -> u64 {
    let mut node = rd::<u64>((fs + 0x18) as usize);
    let view = [rd::<u64>(path as usize), rd::<u64>(path as usize + 8)];
    let (p, len) = (view[0], view[1]);
    let start = if len == 0 {
        0
    } else {
        let c0 = rd::<u8>(p as usize);
        let root = if c0 == b'/' || c0 == b'\\' {
            Some(0u64)
        } else if len >= 2 && rd::<u8>(p as usize + 1) == b':' {
            Some(1)
        } else {
            None
        };
        match root {
            Some(r) if r < len => r + 1,
            Some(_) => len,
            None => 0,
        }
    };
    let t = crate::exe::ptr::<u8>(FOLD_VA);
    let seps_va = crate::exe::ptr::<u8>(SEPS_VIEW_VA) as usize;
    let seps_of = || [rd::<u64>(seps_va), rd::<u64>(seps_va + 8)];
    let mut seps = seps_of();
    let mut pos = crate::exe::call_va!(FIND_FIRST_NOT_OF_VA, (view.as_ptr(), seps.as_ptr(), start) -> u64);
    let sep_ptr = rd::<u64>(seps_va);
    let (s0, s1) = (fold(t, rd::<u8>(sep_ptr as usize)), fold(t, rd::<u8>(sep_ptr as usize + 1)));
    let find_sep = |from: u64| -> u64 {
        if from < len {
            for i in from..len {
                let c = fold(t, rd::<u8>((p + i) as usize));
                if s0 == c || s1 == c {
                    return i;
                }
            }
        }
        u64::MAX
    };
    let mut end = find_sep(pos);
    while pos != u64::MAX {
        if len < pos {
            crate::exe::call_va!(XOUT_OF_RANGE_VA, ());
            unreachable!("_Xout_of_range returned");
        }
        let comp = p + pos;
        let clen = end.wrapping_sub(pos).min(len - pos);
        if !(clen == 1 && rd::<u8>(comp as usize) == b'.') {
            let key = [comp, clen];
            node = crate::exe::call_va!(FIND_DIR_VA, (fs, node, key.as_ptr()) -> u64);
            if node == 0 {
                return 0;
            }
        }
        seps = seps_of();
        pos = crate::exe::call_va!(FIND_FIRST_NOT_OF_VA, (view.as_ptr(), seps.as_ptr(), end) -> u64);
        end = find_sep(pos);
    }
    node
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

use crate::diff_eq;
use crate::difftest::Rng;

/// Builds a fake pack index (string pool, dir and file runs, one node) and
/// compares both lookups against the originals for random and present keys.
unsafe fn diff_lookup(orig: usize, new: usize, files: bool, seed: u64) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let t = crate::exe::ptr::<u8>(FOLD_VA);
    let (stride, name_off, len_off) = if files { (0x20usize, 0x0cusize, 0x10usize) } else { (0x1c, 0x14, 0x18) };
    let alphabet = b"aAbBcC_/\\.-09zZ\xc3\xa9";
    let mut r = Rng::new(seed);
    let mut cases = 0u32;
    for i in 0..600u64 {
        let count = match i % 4 {
            0 => r.below(4),
            1 => r.below(20),
            _ => r.below(200),
        } as usize;
        let mut names: Vec<Vec<u8>> = (0..count)
            .map(|_| {
                let len = r.below(if i % 3 == 0 { 3 } else { 12 }) as usize;
                (0..len).map(|_| alphabet[r.below(alphabet.len() as u64) as usize]).collect()
            })
            .collect();
        // Most runs sorted the way the pack builder sorts them; some unsorted (both
        // implementations must still agree step for step).
        if i % 5 != 0 {
            names.sort_by(|a, b| {
                let fa: Vec<i8> = a.iter().map(|&c| fold(t, c)).collect();
                let fb: Vec<i8> = b.iter().map(|&c| fold(t, c)).collect();
                fa.cmp(&fb)
            });
        }
        let mut pool = Vec::new();
        let first_idx = r.below(3) as usize;
        // Odd base offset: entries are 0x1c apart, so most are under-aligned anyway.
        let off = (i % 2) as usize * 4;
        let mut arr = vec![0u8; off + (first_idx + count) * stride + 8];
        for (k, nm) in names.iter().enumerate() {
            let e = off + (first_idx + k) * stride;
            arr[e + name_off..e + name_off + 4].copy_from_slice(&(pool.len() as u32).to_le_bytes());
            arr[e + len_off..e + len_off + 4].copy_from_slice(&(nm.len() as u32).to_le_bytes());
            pool.extend_from_slice(nm);
        }
        pool.push(0);
        let arr_base = arr.as_ptr() as u64 + off as u64;
        let mut fs = vec![0u8; 0x60];
        let (arr_slot, pool_slot) = (if files { 0x28 } else { 0x18 }, 0x48);
        fs[arr_slot..arr_slot + 8].copy_from_slice(&arr_base.to_le_bytes());
        fs[pool_slot..pool_slot + 8].copy_from_slice(&(pool.as_ptr() as u64).to_le_bytes());
        let mut node = vec![0u8; 0x18];
        let (fi, ci) = if files { (0xc, 0x10) } else { (4, 8) };
        node[fi..fi + 4].copy_from_slice(&(first_idx as u32).to_le_bytes());
        node[ci..ci + 4].copy_from_slice(&(count as u32).to_le_bytes());
        for _ in 0..8 {
            let key: Vec<u8> = match r.below(3) {
                0 if count > 0 => {
                    // Present name, with random case flips (fold makes them equal).
                    names[r.below(count as u64) as usize]
                        .iter()
                        .map(|&c| if c.is_ascii_alphabetic() && r.below(2) == 0 { c ^ 0x20 } else { c })
                        .collect()
                }
                _ => {
                    let len = r.below(8) as usize;
                    (0..len).map(|_| alphabet[r.below(alphabet.len() as u64) as usize]).collect()
                }
            };
            let view: [u64; 2] = [key.as_ptr() as u64, key.len() as u64];
            let vp = view.as_ptr() as u64;
            let a = o(fs.as_ptr() as u64, node.as_ptr() as u64, vp);
            let b = n(fs.as_ptr() as u64, node.as_ptr() as u64, vp);
            diff_eq!(a, b, "case {i}: count {count} key {:?}", String::from_utf8_lossy(&key));
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_pack2_find_dir(orig: usize, new: usize) -> Result<u32, String> {
    diff_lookup(orig, new, false, 0x142432a80)
}

pub unsafe fn diff_pack2_find_file(orig: usize, new: usize) -> Result<u32, String> {
    diff_lookup(orig, new, true, 0x142432790)
}

/// Random directory trees (BFS layout, children contiguous and fold-sorted) and
/// random paths built from existing names (case-flipped), `.`, `..`, unknown
/// names, separators and drive prefixes.
pub unsafe fn diff_pack2_resolve_dir(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let t = crate::exe::ptr::<u8>(FOLD_VA);
    let alphabet = b"abAB_.9";
    let mut r = Rng::new(0x142432c60);
    let mut cases = 0u32;
    for i in 0..300u64 {
        // BFS: names[k] and children ranges.
        let mut names: Vec<Vec<u8>> = vec![Vec::new()];
        let mut ranges: Vec<(u32, u32)> = vec![(0, 0)];
        let mut q = 0usize;
        while q < names.len() && names.len() < 60 {
            let depth_ok = q < 20;
            let kids = if depth_ok { r.below(5) as usize } else { 0 };
            let mut kn: Vec<Vec<u8>> = (0..kids)
                .map(|_| {
                    let l = 1 + r.below(3) as usize;
                    (0..l).map(|_| alphabet[r.below(alphabet.len() as u64) as usize]).collect()
                })
                .collect();
            kn.sort_by(|a, b| {
                let fa: Vec<i8> = a.iter().map(|&c| fold(t, c)).collect();
                let fb: Vec<i8> = b.iter().map(|&c| fold(t, c)).collect();
                fa.cmp(&fb)
            });
            ranges[q] = (names.len() as u32, kn.len() as u32);
            for k in kn {
                names.push(k);
                ranges.push((0, 0));
            }
            q += 1;
        }
        let mut pool = Vec::new();
        let mut arr = vec![0u8; names.len() * 0x1c + 8];
        for (k, nm) in names.iter().enumerate() {
            let e = k * 0x1c;
            arr[e + 4..e + 8].copy_from_slice(&ranges[k].0.to_le_bytes());
            arr[e + 8..e + 12].copy_from_slice(&ranges[k].1.to_le_bytes());
            arr[e + 0x14..e + 0x18].copy_from_slice(&(pool.len() as u32).to_le_bytes());
            arr[e + 0x18..e + 0x1c].copy_from_slice(&(nm.len() as u32).to_le_bytes());
            pool.extend_from_slice(nm);
        }
        pool.push(0);
        let mut fs = vec![0u8; 0x60];
        fs[0x18..0x20].copy_from_slice(&(arr.as_ptr() as u64).to_le_bytes());
        fs[0x48..0x50].copy_from_slice(&(pool.as_ptr() as u64).to_le_bytes());
        for _ in 0..10 {
            let mut path = Vec::new();
            match r.below(5) {
                0 => path.push(b'/'),
                1 => path.extend_from_slice(b"C:"),
                2 => path.push(b'\\'),
                _ => {}
            }
            // Walk down the real tree most of the time.
            let mut cur = 0usize;
            for _ in 0..r.below(5) {
                match r.below(6) {
                    0 => path.push(b'.'),
                    1 => path.extend_from_slice(b"zz"),
                    _ => {
                        let (f, c) = ranges[cur];
                        if c == 0 {
                            path.extend_from_slice(b"..");
                        } else {
                            cur = (f + r.below(c as u64) as u32) as usize;
                            for &ch in &names[cur] {
                                path.push(if ch.is_ascii_alphabetic() && r.below(2) == 0 { ch ^ 0x20 } else { ch });
                            }
                        }
                    }
                }
                path.extend_from_slice(if r.below(3) == 0 { b"\\" } else { b"/" });
                if r.below(4) == 0 {
                    path.push(b'/');
                }
            }
            if r.below(2) == 0 && !path.is_empty() {
                path.pop();
            }
            let view: [u64; 2] = [path.as_ptr() as u64, path.len() as u64];
            let a = o(fs.as_ptr() as u64, view.as_ptr() as u64);
            let b = n(fs.as_ptr() as u64, view.as_ptr() as u64);
            diff_eq!(a, b, "case {i}: path {:?}", String::from_utf8_lossy(&path));
            cases += 1;
        }
    }
    Ok(cases)
}
