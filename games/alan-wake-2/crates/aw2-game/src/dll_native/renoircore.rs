//! `renoircore.windowsdesktop.dll` (Coherent Labs Renoir renderer) hot leaves.

use core::sync::atomic::{AtomicUsize, Ordering};

use super::DllPatch;
use crate::exe::{self, rd, wr};

/// Mapped base of the DLL (set by [`patches`] before any replacement is registered).
static BASE: AtomicUsize = AtomicUsize::new(0);

/// `_tls_index` of the DLL (u32).
const RVA_TLS_INDEX: usize = 0x1e_77c0;
/// On-demand thread-local initializer (`__dyn_tls_on_demand_init`-style thunk).
const RVA_TLS_INIT: usize = 0x18_3fc4;
/// Grow the linear allocator's page array to `n` entries: `(pages_field, n)`.
const RVA_PAGES_GROW: usize = 0x6f50;
/// Global page-source object pointer (`vtbl[0](this, size, align, _)` allocates a page).
const RVA_PAGE_SOURCE: usize = 0x1e_7828;
/// `vector<unique_ptr<T>>` grow-by-one: returns the address of the new end slot.
const RVA_PTR_VEC_GROW: usize = 0xe_03f0;

pub(crate) fn patches(base: usize) -> Vec<DllPatch> {
    BASE.store(base, Ordering::Relaxed);
    vec![
        DllPatch { rva: 0x7030, len: 0x10f, fnv: 0xcbdc_06c7_3df4_56f9, target: linear_alloc as *const () as usize },
        DllPatch { rva: 0xd_e180, len: 0x183, fnv: 0x3c84_8e27_ecd6_232f, target: create_pool as *const () as usize },
        DllPatch { rva: 0x11_4ce0, len: 0x233, fnv: 0x74aa_e760_cf4d_729f, target: pool_init as *const () as usize },
    ]
}

/// Padding that aligns `p` up to `align` (`mask = align - 1`).
#[inline(always)]
fn align_pad(p: u64, align: u64, mask: u64) -> u64 {
    let m = p & mask;
    if m == 0 {
        0
    } else {
        align.wrapping_sub(m)
    }
}

/// Carve `size` bytes at `cur + pad` from `page` (`{base, used, size}`) when `free`
/// bytes remain; updates `page.used`.
#[inline(always)]
unsafe fn bump(page: usize, cur: u64, pad: u64, free: u64, size: u64) -> Option<u64> {
    if free < pad || free - pad < size {
        return None;
    }
    let p = cur.wrapping_add(pad);
    if p == 0 {
        return None;
    }
    wr::<u64>(page + 8, rd::<u64>(page + 0x10).wrapping_sub(free - pad).wrapping_add(size));
    Some(p)
}

/// [`linear_alloc`]'s bump from the current page only (never runs guest code): the block,
/// or 0 with nothing modified when the page is full (the caller then makes the real call).
/// Used by the translator's inline call path (`emu::inline_variant`).
pub unsafe extern "win64" fn linear_alloc_fast(arena: u64, size: u64, align_raw: u64) -> u64 {
    let a = arena as usize;
    let align = align_raw as u32 as u64;
    let mask = align.wrapping_sub(1);
    let idx = rd::<u64>(a);
    let page = rd::<usize>(a + 0x10).wrapping_add((idx as usize).wrapping_mul(24));
    let cur = rd::<u64>(page).wrapping_add(rd::<u64>(page + 8));
    let free = rd::<u64>(page + 0x10).wrapping_sub(rd::<u64>(page + 8));
    bump(page, cur, align_pad(cur, align, mask), free, size).unwrap_or(0)
}

/// Linear (bump) allocator.
///
/// Original: `renoircore+0x7030` (0x10f bytes). Arena `{cur: u64, page_size: u64,
/// pages: *Page, cap: u32}`, `Page = {base, used, size}` (24 bytes). Allocates from the
/// current page; otherwise moves to the next page (growing the page array via
/// `+0x6f50`), gets `max(align + size, page_size)` bytes from the global page source and
/// retries once (0 on failure).
pub unsafe extern "win64" fn linear_alloc(arena: u64, size: u64, align_raw: u64) -> u64 {
    let a = arena as usize;
    let align = align_raw as u32 as u64;
    let mask = align.wrapping_sub(1);
    let idx = rd::<u64>(a);
    let page = rd::<usize>(a + 0x10).wrapping_add((idx as usize).wrapping_mul(24));
    let cur = rd::<u64>(page).wrapping_add(rd::<u64>(page + 8));
    let free = rd::<u64>(page + 0x10).wrapping_sub(rd::<u64>(page + 8));
    if let Some(p) = bump(page, cur, align_pad(cur, align, mask), free, size) {
        return p;
    }

    let base = BASE.load(Ordering::Relaxed);
    let next = idx.wrapping_add(1);
    wr::<u64>(a, next);
    let cap = rd::<u32>(a + 0x18) as u64;
    if next >= cap {
        exe::call_va!(base + RVA_PAGES_GROW, (a + 0x10, cap * 2));
    }
    let idx2 = rd::<u64>(a);
    let page_size = rd::<u64>(a + 8);
    let mut want = align.wrapping_add(size);
    if want <= page_size {
        want = page_size;
    }
    let page = rd::<usize>(a + 0x10).wrapping_add((idx2 as usize).wrapping_mul(24));
    wr::<u64>(page + 0x10, want);
    let src = rd::<usize>(base + RVA_PAGE_SOURCE);
    let alloc_fn = rd::<usize>(rd::<usize>(src));
    // r9 is a leftover (old index * 3) in the original; passed through unchanged.
    let mem = exe::call_va!(alloc_fn, (src, want, align_raw, idx.wrapping_mul(3)) -> u64);
    wr::<u64>(page, mem);
    wr::<u64>(page + 8, 0);
    let total = rd::<u64>(page + 0x10);
    bump(page, mem, align_pad(mem, align, mask), total, size).unwrap_or(0)
}

/// Address of the next free slot of a `vector<unique_ptr<T>>` `{buf, count: u32, cap: u32}`
/// (`renoircore+0xe03f0`; only the growth case runs the original).
#[inline(always)]
unsafe fn ptr_vec_end_slot(base: usize, vec: usize) -> usize {
    let count = rd::<u32>(vec + 8);
    if rd::<u32>(vec + 0xc).wrapping_sub(count) >= 1 {
        return rd::<usize>(vec).wrapping_add(count as usize * 8);
    }
    exe::call_va!(base + RVA_PTR_VEC_GROW, (vec) -> usize)
}

/// Body of [`create_pool`] with the allocator and vector-slot callees injected.
unsafe fn build_pool(
    this: usize,
    idx: u64,
    kind: u32,
    mut alloc: impl FnMut(u64, u64) -> usize,
    end_slot: impl FnOnce(usize) -> usize,
) -> u64 {
    let obj = alloc(0x60, 8);
    core::ptr::write_bytes(obj as *mut u8, 0, 0x60);
    wr::<u64>(obj, idx);
    wr::<u32>(obj + 0x58, kind);
    let row = idx.wrapping_mul(0x54) as usize;
    let n = rd::<u32>(rd::<usize>(rd::<usize>(this)).wrapping_add(row));
    let ents = alloc(((n as u64) << 4) + 8, 8);
    wr::<u32>(ents, n);
    let pool = ents + 8;
    core::ptr::write_bytes(pool as *mut u8, 0, n as usize * 16);
    wr::<usize>(obj + 8, pool);
    let desc = rd::<usize>(rd::<usize>(this)).wrapping_add(row);
    pool_init((obj + 0x10) as u64, rd::<u32>(desc) as u64, rd::<u32>(desc + 4) as u64, pool as u64, n as u64);
    // push_back(unique_ptr): the original's self-alias branch (source element inside the
    // vector buffer) cannot trigger, the source is a stack local.
    let vec = rd::<usize>(this + 8);
    let slot = end_slot(vec);
    wr::<usize>(slot, obj);
    wr::<u32>(vec + 8, rd::<u32>(vec + 8).wrapping_add(1));
    slot as u64
}

/// Create a block pool for descriptor `idx` and append it to the owner's pool list.
///
/// Original: `renoircore+0xde180` (0x183 bytes; ~2 calls per frame on the render
/// thread, the top emulated hot spot because of its per-entry zeroing loop).
/// `this = {&&descs, &vector<unique_ptr<Pool>>}`, `descs[idx]` is 0x54 bytes
/// (`+0` entry count, `+4` second `pool_init` argument). Allocates the 0x60-byte pool
/// object and `8 + 16 * n` bytes of entries from the thread's linear allocator (TLS
/// slot `+0x10`), zeroes them and initializes the free list via [`pool_init`].
pub unsafe extern "win64" fn create_pool(this: u64, idx: u64, kind: u64) -> u64 {
    let base = BASE.load(Ordering::Relaxed);
    let tls_index = rd::<u32>(base + RVA_TLS_INDEX) as usize;
    let tls = rd::<usize>(exe::tls_pointer() + tls_index * 8);
    let alloc = |size: u64, align: u64| -> usize {
        if rd::<u8>(tls + 8) == 0 {
            exe::call_va!(base + RVA_TLS_INIT, ());
        }
        linear_alloc(rd::<u64>(tls + 0x10), size, align) as usize
    };
    build_pool(this as usize, idx, kind as u32, alloc, |vec| ptr_vec_end_slot(base, vec))
}

#[inline(always)]
unsafe fn w32(p: usize, v: u32) {
    core::ptr::write_unaligned(p as *mut u32, v);
}

#[inline(always)]
unsafe fn w64(p: usize, v: u64) {
    core::ptr::write_unaligned(p as *mut u64, v);
}

/// Block-pool header + free-list initialization.
///
/// Original: `renoircore+0x114ce0` (0x233 bytes, leaf; called from the pool
/// constructor at `+0xde180` right after it zeroes the 16-byte entries).
/// Links `pool[i].next (+8) = &pool[i + 1]` for `i < n - 1`, terminates the last
/// entry, fills the header at `obj` and returns `obj + 0x38`.
pub unsafe extern "win64" fn pool_init(obj: u64, a: u64, b: u64, pool: u64, n: u64) -> u64 {
    let (obj, pool) = (obj as usize, pool as usize);
    let (a, b, n) = (a as u32, b as u32, n as u32 as i32);
    let last = n.wrapping_sub(1).max(0) as usize;
    let mut p = pool + 8;
    let mut next = pool + 16;
    for _ in 0..last {
        w64(p, next as u64);
        p += 16;
        next += 16;
    }
    w64(pool + last * 16 + 8, 0);

    let dividend = n.wrapping_sub(1).wrapping_add(a as i32);
    let per = if n != 0 { dividend.wrapping_div(n) } else { 0 };
    w32(obj, a);
    w32(obj + 0x4, b);
    w32(obj + 0x14, n as u32);
    w64(obj + 0xc, 1);
    w64(obj + 0x20, pool as u64);
    w64(obj + 0x18, (obj + 0x28) as u64);
    w32(obj + 0x8, per as u32);
    w32(obj + 0x38, a);
    w32(obj + 0x28, 0);
    w32(obj + 0x2c, 0);
    w64(obj + 0x30, (obj + 0x38) as u64);
    w32(obj + 0x3c, 0x4000_0000);
    w64(obj + 0x40, 0);
    (obj + 0x38) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_init_links_and_header() {
        let mut obj = [0u8; 0x48];
        let mut pool = [0xffu8; 16 * 5];
        let (o, p) = (obj.as_mut_ptr() as usize, pool.as_mut_ptr() as usize);
        let r = unsafe { pool_init(o as u64, 7, 9, p as u64, 5) };
        assert_eq!(r, (o + 0x38) as u64);
        let rd = |a: usize| unsafe { core::ptr::read_unaligned(a as *const u64) };
        for i in 0..4 {
            assert_eq!(rd(p + i * 16 + 8), (p + (i + 1) * 16) as u64);
        }
        assert_eq!(rd(p + 4 * 16 + 8), 0);
        let rd32 = |a: usize| unsafe { core::ptr::read_unaligned(a as *const u32) };
        assert_eq!(rd32(o), 7);
        assert_eq!(rd32(o + 4), 9);
        assert_eq!(rd32(o + 8), (5 - 1 + 7) / 5);
        assert_eq!(rd(o + 0xc), 1);
        assert_eq!(rd32(o + 0x14), 5);
        assert_eq!(rd(o + 0x18), (o + 0x28) as u64);
        assert_eq!(rd(o + 0x20), p as u64);
        assert_eq!(rd(o + 0x28), 0);
        assert_eq!(rd(o + 0x30), (o + 0x38) as u64);
        assert_eq!(rd32(o + 0x38), 7);
        assert_eq!(rd32(o + 0x3c), 0x4000_0000);
        assert_eq!(rd(o + 0x40), 0);
    }

    fn r64(a: usize) -> u64 {
        unsafe { core::ptr::read_unaligned(a as *const u64) }
    }
    fn r32(a: usize) -> u32 {
        unsafe { core::ptr::read_unaligned(a as *const u32) }
    }
    fn w64t(a: usize, v: u64) {
        unsafe { core::ptr::write_unaligned(a as *mut u64, v) }
    }

    #[test]
    fn linear_alloc_fast_path_aligns_and_bumps() {
        let mut mem = vec![0u8; 0x200];
        let m = (mem.as_mut_ptr() as usize + 15) & !15;
        let mut pages = [0u64; 6];
        let pg = pages.as_mut_ptr() as usize;
        // page 1 is current: base m, used 3, size 0x100
        w64t(pg + 24, m as u64);
        w64t(pg + 24 + 8, 3);
        w64t(pg + 24 + 0x10, 0x100);
        let mut arena = [0u64; 4];
        let a = arena.as_mut_ptr() as usize;
        w64t(a, 1);
        w64t(a + 8, 0x1000);
        w64t(a + 0x10, pg as u64);
        w64t(a + 0x18, 2);
        let p = unsafe { linear_alloc(a as u64, 0x20, 8) };
        assert_eq!(p, (m + 8) as u64);
        assert_eq!(r64(pg + 24 + 8), 8 + 0x20);
        let q = unsafe { linear_alloc(a as u64, 0x10, 16) };
        assert_eq!(q, (m + 0x30) as u64);
        assert_eq!(r64(pg + 24 + 8), 0x40);
        // exact fit of the remaining space stays on the fast path
        let r = unsafe { linear_alloc(a as u64, 0xc0, 1) };
        assert_eq!(r, (m + 0x40) as u64);
        assert_eq!(r64(pg + 24 + 8), 0x100);
        assert_eq!(r64(a), 1);
    }

    #[test]
    fn build_pool_fills_object_entries_and_list() {
        // descriptor table: 3 rows of 0x54 bytes, row 2 = {n: 4, b: 11}
        let mut descs = vec![0u8; 0x54 * 3];
        let d = descs.as_mut_ptr() as usize;
        unsafe {
            core::ptr::write_unaligned((d + 0x54 * 2) as *mut u32, 4);
            core::ptr::write_unaligned((d + 0x54 * 2 + 4) as *mut u32, 11);
        }
        let descs_ptr = [d as u64];
        let mut vbuf = [0u64; 4];
        let mut vec = [0u64; 2];
        let v = vec.as_mut_ptr() as usize;
        w64t(v, vbuf.as_mut_ptr() as u64);
        w64t(v + 8, 1 | (4u64 << 32)); // count 1, cap 4
        let this = [descs_ptr.as_ptr() as u64, v as u64];

        let mut obj = vec![0xaau8; 0x60];
        let mut ents = vec![0xaau8; 8 + 16 * 4];
        let (o, e) = (obj.as_mut_ptr() as usize, ents.as_mut_ptr() as usize);
        let mut sizes = Vec::new();
        let r = unsafe {
            build_pool(
                this.as_ptr() as usize,
                2,
                0x77,
                |size, align| {
                    sizes.push((size, align));
                    if sizes.len() == 1 {
                        o
                    } else {
                        e
                    }
                },
                |vec| r64(vec) as usize + r32(vec + 8) as usize * 8,
            )
        };
        assert_eq!(sizes, vec![(0x60, 8), (8 + 16 * 4, 8)]);
        assert_eq!(r64(o), 2);
        assert_eq!(r64(o + 8), (e + 8) as u64);
        assert_eq!(r32(o + 0x58), 0x77);
        assert_eq!(r32(o + 0x5c), 0);
        assert_eq!(r32(e), 4);
        // pool_init header at obj + 0x10 and free list through the entries
        assert_eq!(r32(o + 0x10), 4);
        assert_eq!(r32(o + 0x14), 11);
        assert_eq!(r64(o + 0x30), (e + 8) as u64);
        for i in 0..3 {
            assert_eq!(r64(e + 8 + i * 16), 0);
            assert_eq!(r64(e + 8 + i * 16 + 8), (e + 8 + (i + 1) * 16) as u64);
        }
        assert_eq!(r64(e + 8 + 3 * 16 + 8), 0);
        let slot = vbuf.as_ptr() as usize + 8;
        assert_eq!(r, slot as u64);
        assert_eq!(vbuf[1], o as u64);
        assert_eq!(r32(v + 8), 2);
    }
}
