//! ECS query iteration helpers.

/// Archetype/group table entry (0x30 bytes); only the component mask at +0x10
/// is read here.
#[repr(C, packed)]
pub struct GroupEntry {
    pub unk_00: [u64; 2],
    pub component_mask: u64,
    pub unk_18: [u64; 3],
}
const _: () = assert!(std::mem::size_of::<GroupEntry>() == 0x30);

/// Owner of the group table: `+0x10` points at the `GroupEntry` array.
#[repr(C, packed)]
pub struct GroupTable {
    pub unk_00: [u64; 2],
    pub entries: *const GroupEntry,
}

/// Query state the iterator walks (only the fields read here).
#[repr(C, packed)]
pub struct GroupQuery {
    pub unk_00: u64,
    /// One past the last group index to visit.
    pub end: u64,
    pub unk_10: u64,
    pub table: *const GroupTable,
    /// Components every matching group must have.
    pub required: u64,
    /// Components a matching group must not have.
    pub excluded: u64,
}
const _: () = {
    use std::mem::offset_of;
    assert!(offset_of!(GroupQuery, end) == 0x08);
    assert!(offset_of!(GroupQuery, table) == 0x18);
    assert!(offset_of!(GroupQuery, required) == 0x20);
    assert!(offset_of!(GroupQuery, excluded) == 0x28);
};

/// Iterator over the groups matching a [`GroupQuery`].
#[repr(C, packed)]
pub struct GroupIterator {
    pub query: *const GroupQuery,
    pub unk_08: u64,
    pub index: u64,
    /// `(u16)index` of the current group.
    pub current: u16,
    pub has_current: u8,
}
const _: () = {
    use std::mem::offset_of;
    assert!(offset_of!(GroupIterator, index) == 0x10);
    assert!(offset_of!(GroupIterator, current) == 0x18);
    assert!(offset_of!(GroupIterator, has_current) == 0x1a);
};

/// Advance `it` to the first group at or after `it.index` whose mask has all
/// `required` and none of the `excluded` components (name inferred).
///
/// Original: 0x1410f58a0 (125 bytes, leaf). Callers 0x1410f5a90 etc. build the
/// iterator on the stack and call this first. For every visited index it sets
/// `current = (u16)index`, `has_current = 1`, looks up
/// `query.table.entries[(u16)index].component_mask`; it returns on a match,
/// else `index += 1` until `index == query.end`. Boot: ~3.6k calls.
pub unsafe extern "win64" fn group_iterator_advance(it: *mut GroupIterator) {
    let q = (*it).query;
    let mut idx = (*it).index;
    if idx == (*q).end {
        return;
    }
    loop {
        let g = idx as u16;
        (*it).current = g;
        (*it).has_current = 1;
        let q = (*it).query;
        let mask = (*(*(*q).table).entries.add(g as usize)).component_mask;
        let (req, excl) = ((*q).required, (*q).excluded);
        if mask & req == req && mask & excl == 0 {
            return;
        }
        idx = (*it).index.wrapping_add(1);
        (*it).index = idx;
        if idx == (*q).end {
            return;
        }
    }
}

pub unsafe fn diff_group_iterator_advance(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut GroupIterator);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(0xec5);
    let mut cases = 0;
    for case in 0..300 {
        let count = 1 + r.below(64) as usize;
        // Few distinct bits so matches and non-matches both occur.
        let entries: Vec<GroupEntry> = (0..count)
            .map(|_| GroupEntry { unk_00: [r.next_u64(); 2], component_mask: r.next_u64() & 0x1f, unk_18: [r.next_u64(); 3] })
            .collect();
        let table = GroupTable { unk_00: [0; 2], entries: entries.as_ptr() };
        let end = r.below(count as u64 + 1);
        let q = GroupQuery {
            unk_00: r.next_u64(),
            end,
            unk_10: r.next_u64(),
            table: &table,
            required: r.next_u64() & r.next_u64() & 0x1f,
            excluded: r.next_u64() & r.next_u64() & 0x1f,
        };
        let start = r.below(end + 1);
        // The iterator at byte offset 0 or 4 (4-aligned only, as when embedded).
        let off = 4 * (case % 2);
        let mut raw = [0u8; 48];
        raw.iter_mut().for_each(|w| *w = r.next_u32() as u8);
        raw[off..off + 8].copy_from_slice(&(&q as *const GroupQuery as u64).to_le_bytes());
        raw[off + 16..off + 24].copy_from_slice(&start.to_le_bytes());
        let (mut a, mut b) = ([0u64; 6], [0u64; 6]);
        std::ptr::copy_nonoverlapping(raw.as_ptr(), a.as_mut_ptr() as *mut u8, 48);
        std::ptr::copy_nonoverlapping(raw.as_ptr(), b.as_mut_ptr() as *mut u8, 48);
        o((a.as_mut_ptr() as *mut u8).add(off) as *mut GroupIterator);
        n((b.as_mut_ptr() as *mut u8).add(off) as *mut GroupIterator);
        crate::diff_eq!(a, b, "iterator state, case {case} start {start} end {end} offset {off}");
        cases += 1;
    }
    Ok(cases)
}

/// `r::algo::find_u16`. Called via `call_native!`: through `call_va8` it was 5.8% of all
/// lifted-call dispatches in gameplay (cvP, `AW2_CALLVA_PROF`).
const FIND_U16_VA: u64 = 0x1_4248_9ee0;
const COMP_SPAN_ITER_BEGIN_VA: u64 = 0x1_401e_7c10;

#[inline(always)]
unsafe fn lookup_comp_offset(
    ids_ptr: *const u16,
    count: u64,
    table: usize,
    g: usize,
    comp_va: u64,
) -> usize {
    let comp_id = crate::exe::rd::<u16>(crate::exe::ptr::<u16>(comp_va) as usize);
    let pos = crate::exe::call_native!(FIND_U16_VA, crate::ns::r::algo::find_u16, (ids_ptr, count, comp_id) -> u64) as i32 as isize;
    if pos != -1 {
        let offsets_ptr = crate::exe::rd::<usize>(table.wrapping_add(32 * g.wrapping_add(0xc03)));
        crate::exe::rd::<u32>(offsets_ptr.wrapping_add(4 * (pos as usize))) as usize
    } else {
        0
    }
}

#[inline(always)]
unsafe fn ecs_offset_getter_n<const N: usize>(
    out: *mut usize,
    table: usize,
    group_idx: u16,
    comp_vas: [u64; N],
) -> *mut usize {
    use crate::exe::{rd, wr};
    let g = group_idx as usize;
    let count = rd::<u32>(table.wrapping_add(32 * g + 0x1806c)) as u64;
    let ids_ptr = rd::<*const u16>(table.wrapping_add(32 * g + 0x18058));
    let mut offs = [0usize; N];
    for i in 0..N {
        offs[i] = lookup_comp_offset(ids_ptr, count, table, g, comp_vas[i]);
    }
    let o = out as usize;
    for i in 0..N {
        wr::<usize>(o.wrapping_add(8 * i), offs[i]);
    }
    out
}

/// Look up 5 component byte offsets in `table` for `group_idx` and write them to `out[0..5]`.
///
/// Originals: 73 functions of 322 bytes (e.g. `0x1402280f0`).
pub unsafe extern "win64" fn ecs_offset_getter_5<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4])
}

/// Look up 6 component byte offsets in `table` for `group_idx` and write them to `out[0..6]`.
///
/// Originals: 53 functions of 364 bytes (e.g. `0x14022f1d0`).
pub unsafe extern "win64" fn ecs_offset_getter_6<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4, C5])
}

/// Look up 7 component byte offsets in `table` for `group_idx` and write them to `out[0..7]`.
///
/// Originals: 49 functions of 422 bytes (e.g. `0x14021ec30`).
pub unsafe extern "win64" fn ecs_offset_getter_7<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4, C5, C6])
}

/// Look up 8 component byte offsets in `table` for `group_idx` and write them to `out[0..8]`.
///
/// Originals: 30 functions of 446 bytes (e.g. `0x14026b830`).
pub unsafe extern "win64" fn ecs_offset_getter_8<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4, C5, C6, C7])
}

/// Look up 9 component byte offsets in `table` for `group_idx` and write them to `out[0..9]`.
///
/// Originals: 30 functions of 497 bytes (e.g. `0x140274c20`).
pub unsafe extern "win64" fn ecs_offset_getter_9<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4, C5, C6, C7, C8])
}

/// Look up 10 component byte offsets in `table` for `group_idx` and write them to `out[0..10]`.
///
/// Originals: 27 functions of 558 bytes (e.g. `0x14023efd0`).
pub unsafe extern "win64" fn ecs_offset_getter_10<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9])
}

#[inline(always)]
unsafe fn ecs_tuple_iter_ctor_n<const N: usize>(
    this: *mut u8,
    bucket_idx: u16,
    base_ptr: usize,
    count: usize,
    ctx: usize,
    comp_vas: [u64; N],
) -> *mut u8 {
    use crate::exe::{rd, wr};
    let t = this as usize;
    wr::<usize>(t + 0x10, ctx);
    let table = ctx.wrapping_add(0x40);
    wr::<u16>(t, bucket_idx);
    wr::<usize>(t + 0x08, base_ptr);
    wr::<usize>(t + 0x18, count);
    let g = bucket_idx as usize;
    let n_ids = rd::<u32>(table.wrapping_add(32 * g + 0x1806c)) as u64;
    let ids_ptr = rd::<*const u16>(table.wrapping_add(32 * g + 0x18058));
    let mut offs = [0usize; N];
    for i in 0..N {
        offs[i] = lookup_comp_offset(ids_ptr, n_ids, table, g, comp_vas[i]);
    }
    for i in 0..N {
        let p = if offs[i] != 0 { base_ptr.wrapping_add(offs[i]) } else { 0 };
        wr::<usize>(t + 0x20 + 8 * i, p);
    }
    this
}

/// Construct a 2-component tuple iterator (`0x140220cc0` family, 117 functions, 250 bytes).
pub unsafe extern "win64" fn ecs_tuple_iter_ctor_2<const C0: u64, const C1: u64>(
    this: *mut u8,
    bucket_idx: u16,
    base_ptr: usize,
    count: usize,
    ctx: usize,
) -> *mut u8 {
    ecs_tuple_iter_ctor_n(this, bucket_idx, base_ptr, count, ctx, [C0, C1])
}

/// Construct a 3-component tuple iterator (`0x14023f200` family, 107 functions, 301 bytes).
pub unsafe extern "win64" fn ecs_tuple_iter_ctor_3<const C0: u64, const C1: u64, const C2: u64>(
    this: *mut u8,
    bucket_idx: u16,
    base_ptr: usize,
    count: usize,
    ctx: usize,
) -> *mut u8 {
    ecs_tuple_iter_ctor_n(this, bucket_idx, base_ptr, count, ctx, [C0, C1, C2])
}

#[inline(always)]
unsafe fn ecs_outer_iter_load_n<const N: usize>(this: *mut u8, comp_vas: [u64; N]) {
    use crate::exe::{rd, wr};
    let it = this as usize;
    let cur = rd::<usize>(it + 0x10);
    if cur == rd::<usize>(it + 0x18) {
        return;
    }
    let g = rd::<u16>(cur) as usize;
    let table = rd::<usize>(it);
    let n_ids = rd::<u32>(table.wrapping_add(32 * g + 0x1806c)) as u64;
    let ids_ptr = rd::<*const u16>(table.wrapping_add(32 * g + 0x18058));
    let mut offs = [0usize; N];
    for i in 0..N {
        offs[i] = lookup_comp_offset(ids_ptr, n_ids, table, g, comp_vas[i]);
    }
    let base_ptr = rd::<usize>(it + 0x20);
    for i in 0..N {
        let p = if offs[i] != 0 { base_ptr.wrapping_add(offs[i]) } else { 0 };
        wr::<usize>(it + 0x38 + 8 * i, p);
    }
}

/// Load 1 component pointer into an outer ECS iterator (`0x1401ef4c0` family, 10 functions, 143 bytes).
pub unsafe extern "win64" fn ecs_outer_iter_load_1<const C0: u64>(this: *mut u8) {
    ecs_outer_iter_load_n(this, [C0]);
}

/// Load 2 component pointers into an outer ECS iterator (`0x140218470` family, 75 functions, 242 bytes).
pub unsafe extern "win64" fn ecs_outer_iter_load_2<const C0: u64, const C1: u64>(this: *mut u8) {
    ecs_outer_iter_load_n(this, [C0, C1]);
}

/// Load 3 component pointers into an outer ECS iterator (`0x1402418d0` family, 31 functions, 307 bytes).
pub unsafe extern "win64" fn ecs_outer_iter_load_3<const C0: u64, const C1: u64, const C2: u64>(
    this: *mut u8,
) {
    ecs_outer_iter_load_n(this, [C0, C1, C2]);
}

/// Load 4 component pointers into an outer ECS iterator (`0x140218720` family, 27 functions, 353 bytes).
pub unsafe extern "win64" fn ecs_outer_iter_load_4<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
>(
    this: *mut u8,
) {
    ecs_outer_iter_load_n(this, [C0, C1, C2, C3]);
}

/// Dereference a 4-component tuple iterator (`0x140239ca0` family, 66 functions, 337 bytes).
pub unsafe extern "win64" fn ecs_tuple_deref_4<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
>(
    this: *const u8,
    out: *mut usize,
) -> *mut usize {
    use crate::exe::{rd, wr};
    let it = this as usize;
    let g = rd::<u16>(it) as usize;
    let table = rd::<usize>(it + 0x10).wrapping_add(0x40);
    let n_ids = rd::<u32>(table.wrapping_add(32 * g + 0x1806c)) as u64;
    let ids_ptr = rd::<*const u16>(table.wrapping_add(32 * g + 0x18058));
    let comp_vas = [C0, C1, C2, C3];
    let mut offs = [0usize; 4];
    for i in 0..4 {
        offs[i] = lookup_comp_offset(ids_ptr, n_ids, table, g, comp_vas[i]);
    }
    let base_ptr = rd::<usize>(it + 0x08);
    let o = out as usize;
    for i in 0..4 {
        let p = if offs[i] != 0 { base_ptr.wrapping_add(offs[i]) } else { 0 };
        wr::<usize>(o + 8 * i, p);
    }
    out
}

/// Initialize an outer ECS iterator at `out` (`rdx`) from query `this` (`rcx`) and call
/// its typed component loader `LOAD_VA(out)` (`0x1401ef430` family, 18 functions, 140 bytes).
pub unsafe extern "win64" fn ecs_outer_iter_begin<const LOAD_VA: u64>(
    this: *const u8,
    out: *mut u8,
) -> *mut u8 {
    use crate::exe::{rd, wr};
    let q = this as usize;
    let o = out as usize;
    let table = rd::<usize>(q).wrapping_add(0x40);
    let span: [u64; 2] = [rd::<u64>(q + 0x08), rd::<u64>(q + 0x10)];
    let mut inner = [0u64; 3];
    let _ = crate::exe::call_va!(
        COMP_SPAN_ITER_BEGIN_VA,
        inner.as_mut_ptr() as *mut u8,
        table as *const u8,
        span.as_ptr() as *const u8,
    );
    wr::<usize>(o, table);
    wr::<u64>(o + 0x08, inner[0]);
    wr::<u64>(o + 0x10, inner[1]);
    wr::<u64>(o + 0x18, inner[2]);
    wr::<u64>(o + 0x28, 0);
    wr::<u64>(o + 0x30, 0);
    let cur = rd::<usize>(o + 0x10);
    if cur != rd::<usize>(o + 0x18) {
        let idx = rd::<u16>(cur) as usize;
        let ptr = rd::<u64>(table.wrapping_add(0x48 + 8 * idx));
        wr::<u64>(o + 0x20, ptr);
        let cnt = if rd::<u64>(table.wrapping_add(0x48 + 8 * idx)) != 0 {
            rd::<u32>(table.wrapping_add(0x10048 + 4 * idx)) as u64
        } else {
            0
        };
        wr::<u64>(o + 0x28, cnt);
    }
    let _ = crate::exe::call_va!(LOAD_VA, out);
    out
}

unsafe fn diff_ecs_offset_getter<const N: usize>(
    orig: usize,
    new: usize,
    comp_vas: [u64; N],
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut usize, usize, usize, u16) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(orig as u64);
    let mut saved_ids = [0u16; N];
    let mut comp_ids = [0u16; N];
    for i in 0..N {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        saved_ids[i] = crate::exe::rd::<u16>(p);
        comp_ids[i] = 0x100 + (i as u16) * 7;
        crate::exe::wr::<u16>(p, comp_ids[i]);
    }
    let mut table = vec![0u8; 32 * (0xc03 + 8) + 0x100];
    let t_base = table.as_mut_ptr() as usize;
    const CASES: u32 = 150;
    let res = (|| -> Result<u32, String> {
        for case in 0..CASES {
            let g = (case as usize) % 4;
            let count = (r.below(12) as usize).min(10);
            let mut ids = [0u16; 12];
            let mut offsets = [0u32; 12];
            for k in 0..count {
                ids[k] = if r.below(2) == 0 {
                    comp_ids[r.below(N as u64) as usize]
                } else {
                    r.next_u32() as u16
                };
                offsets[k] = if r.below(4) == 0 { 0 } else { r.next_u32() };
            }
            crate::exe::wr::<u32>(t_base + 32 * g + 0x1806c, count as u32);
            crate::exe::wr::<usize>(t_base + 32 * g + 0x18058, ids.as_ptr() as usize);
            crate::exe::wr::<usize>(t_base + 32 * (g + 0xc03), offsets.as_ptr() as usize);
            let mut out_a = [0usize; N];
            let mut out_b = [0usize; N];
            let rdx = r.next_u64() as usize;
            let ra = o(out_a.as_mut_ptr(), rdx, t_base, g as u16);
            let rb = n(out_b.as_mut_ptr(), rdx, t_base, g as u16);
            crate::diff_eq!(ra - out_a.as_ptr() as usize, rb - out_b.as_ptr() as usize, "ret case {case}");
            crate::diff_eq!(out_a, out_b, "out case {case}");
        }
        Ok(CASES)
    })();
    for i in 0..N {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        crate::exe::wr::<u16>(p, saved_ids[i]);
    }
    res
}

pub unsafe fn diff_ecs_offset_getter_5<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4])
}

pub unsafe fn diff_ecs_offset_getter_6<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5])
}

pub unsafe fn diff_ecs_offset_getter_7<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5, C6])
}

pub unsafe fn diff_ecs_offset_getter_8<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5, C6, C7])
}

pub unsafe fn diff_ecs_offset_getter_9<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5, C6, C7, C8])
}

pub unsafe fn diff_ecs_offset_getter_10<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9])
}

unsafe fn diff_ecs_tuple_iter_ctor<const N: usize>(
    orig: usize,
    new: usize,
    comp_vas: [u64; N],
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, u16, usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(orig as u64);
    let mut saved_ids = [0u16; N];
    let mut comp_ids = [0u16; N];
    for i in 0..N {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        saved_ids[i] = crate::exe::rd::<u16>(p);
        comp_ids[i] = 0x200 + (i as u16) * 11;
        crate::exe::wr::<u16>(p, comp_ids[i]);
    }
    let mut ctx_buf = vec![0u8; 0x40 + 32 * (0xc03 + 8) + 0x100];
    let ctx = ctx_buf.as_mut_ptr() as usize;
    let t_base = ctx + 0x40;
    const CASES: u32 = 150;
    let res = (|| -> Result<u32, String> {
        for case in 0..CASES {
            let g = (case as usize) % 4;
            let count = (r.below(10) as usize).min(8);
            let mut ids = [0u16; 10];
            let mut offsets = [0u32; 10];
            for k in 0..count {
                ids[k] = if r.below(2) == 0 {
                    comp_ids[r.below(N as u64) as usize]
                } else {
                    r.next_u32() as u16
                };
                offsets[k] = if r.below(4) == 0 { 0 } else { r.next_u32() };
            }
            crate::exe::wr::<u32>(t_base + 32 * g + 0x1806c, count as u32);
            crate::exe::wr::<usize>(t_base + 32 * g + 0x18058, ids.as_ptr() as usize);
            crate::exe::wr::<usize>(t_base + 32 * (g + 0xc03), offsets.as_ptr() as usize);
            let base_ptr = r.next_u64() as usize;
            let elem_cnt = r.next_u64() as usize;
            let init = [r.next_u64(); 8];
            let (mut a, mut b) = (init, init);
            let ra = o(a.as_mut_ptr() as *mut u8, g as u16, base_ptr, elem_cnt, ctx);
            let rb = n(b.as_mut_ptr() as *mut u8, g as u16, base_ptr, elem_cnt, ctx);
            crate::diff_eq!(ra - a.as_ptr() as usize, rb - b.as_ptr() as usize, "ret case {case}");
            crate::diff_eq!(a, b, "state case {case}");
        }
        Ok(CASES)
    })();
    for i in 0..N {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        crate::exe::wr::<u16>(p, saved_ids[i]);
    }
    res
}

pub unsafe fn diff_ecs_tuple_iter_ctor_2<const C0: u64, const C1: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_tuple_iter_ctor(orig, new, [C0, C1])
}

pub unsafe fn diff_ecs_tuple_iter_ctor_3<const C0: u64, const C1: u64, const C2: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_tuple_iter_ctor(orig, new, [C0, C1, C2])
}

unsafe fn diff_ecs_outer_iter_load<const N: usize>(
    orig: usize,
    new: usize,
    comp_vas: [u64; N],
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(orig as u64);
    let mut saved_ids = [0u16; N];
    let mut comp_ids = [0u16; N];
    for i in 0..N {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        saved_ids[i] = crate::exe::rd::<u16>(p);
        comp_ids[i] = 0x300 + (i as u16) * 13;
        crate::exe::wr::<u16>(p, comp_ids[i]);
    }
    let mut table = vec![0u8; 32 * (0xc03 + 8) + 0x100];
    let t_base = table.as_mut_ptr() as usize;
    const CASES: u32 = 150;
    let res = (|| -> Result<u32, String> {
        for case in 0..CASES {
            let g = ((case as usize) % 4) as u16;
            let count = (r.below(10) as usize).min(8);
            let mut ids = [0u16; 10];
            let mut offsets = [0u32; 10];
            for k in 0..count {
                ids[k] = if r.below(2) == 0 {
                    comp_ids[r.below(N as u64) as usize]
                } else {
                    r.next_u32() as u16
                };
                offsets[k] = if r.below(4) == 0 { 0 } else { r.next_u32() };
            }
            let gi = g as usize;
            crate::exe::wr::<u32>(t_base + 32 * gi + 0x1806c, count as u32);
            crate::exe::wr::<usize>(t_base + 32 * gi + 0x18058, ids.as_ptr() as usize);
            crate::exe::wr::<usize>(t_base + 32 * (gi + 0xc03), offsets.as_ptr() as usize);
            let cur_ptr = &g as *const u16 as u64;
            let end_ptr = if r.below(4) == 0 { cur_ptr } else { cur_ptr + 2 };
            let mut init = [0u64; 12];
            for w in &mut init {
                *w = r.next_u64();
            }
            init[0] = t_base as u64;
            init[2] = cur_ptr;
            init[3] = end_ptr;
            let (mut a, mut b) = (init, init);
            o(a.as_mut_ptr() as *mut u8);
            n(b.as_mut_ptr() as *mut u8);
            crate::diff_eq!(a, b, "state case {case}");
        }
        Ok(CASES)
    })();
    for i in 0..N {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        crate::exe::wr::<u16>(p, saved_ids[i]);
    }
    res
}

pub unsafe fn diff_ecs_outer_iter_load_1<const C0: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_outer_iter_load(orig, new, [C0])
}

pub unsafe fn diff_ecs_outer_iter_load_2<const C0: u64, const C1: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_outer_iter_load(orig, new, [C0, C1])
}

pub unsafe fn diff_ecs_outer_iter_load_3<const C0: u64, const C1: u64, const C2: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_outer_iter_load(orig, new, [C0, C1, C2])
}

pub unsafe fn diff_ecs_outer_iter_load_4<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_outer_iter_load(orig, new, [C0, C1, C2, C3])
}

pub unsafe fn diff_ecs_tuple_deref_4<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(orig as u64);
    let comp_vas = [C0, C1, C2, C3];
    let mut saved_ids = [0u16; 4];
    let mut comp_ids = [0u16; 4];
    for i in 0..4 {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        saved_ids[i] = crate::exe::rd::<u16>(p);
        comp_ids[i] = 0x400 + (i as u16) * 17;
        crate::exe::wr::<u16>(p, comp_ids[i]);
    }
    let mut ctx_buf = vec![0u8; 0x40 + 32 * (0xc03 + 8) + 0x100];
    let ctx = ctx_buf.as_mut_ptr() as usize;
    let t_base = ctx + 0x40;
    const CASES: u32 = 150;
    let res = (|| -> Result<u32, String> {
        for case in 0..CASES {
            let g = (case as usize) % 4;
            let count = (r.below(10) as usize).min(8);
            let mut ids = [0u16; 10];
            let mut offsets = [0u32; 10];
            for k in 0..count {
                ids[k] = if r.below(2) == 0 {
                    comp_ids[r.below(4) as usize]
                } else {
                    r.next_u32() as u16
                };
                offsets[k] = if r.below(4) == 0 { 0 } else { r.next_u32() };
            }
            crate::exe::wr::<u32>(t_base + 32 * g + 0x1806c, count as u32);
            crate::exe::wr::<usize>(t_base + 32 * g + 0x18058, ids.as_ptr() as usize);
            crate::exe::wr::<usize>(t_base + 32 * (g + 0xc03), offsets.as_ptr() as usize);
            let mut it = [0u64; 4];
            crate::exe::wr::<u16>(it.as_mut_ptr() as usize, g as u16);
            it[1] = r.next_u64();
            it[2] = ctx as u64;
            let mut out_a = [0usize; 4];
            let mut out_b = [0usize; 4];
            let ra = o(it.as_ptr() as *const u8, out_a.as_mut_ptr());
            let rb = n(it.as_ptr() as *const u8, out_b.as_mut_ptr());
            crate::diff_eq!(ra - out_a.as_ptr() as usize, rb - out_b.as_ptr() as usize, "ret case {case}");
            crate::diff_eq!(out_a, out_b, "out case {case}");
        }
        Ok(CASES)
    })();
    for i in 0..4 {
        let p = crate::exe::ptr::<u16>(comp_vas[i]) as usize;
        crate::exe::wr::<u16>(p, saved_ids[i]);
    }
    res
}

static LOAD_CALL_ARG: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

unsafe extern "win64" fn fake_outer_iter_load(out: *mut u8) {
    LOAD_CALL_ARG.store(out as usize, std::sync::atomic::Ordering::Relaxed);
}

pub unsafe fn diff_ecs_outer_iter_begin<const LOAD_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _mock = crate::difftest::Mock::new(LOAD_VA, fake_outer_iter_load as *const ())?;
    let mut r = crate::difftest::Rng::new(orig as u64);
    let mut ctx_buf = vec![0u8; 0x40 + 0x10048 + 4 * 16];
    let ctx = ctx_buf.as_mut_ptr() as usize;
    let table = ctx + 0x40;
    const CASES: u32 = 150;
    for case in 0..CASES {
        for idx in 0..16usize {
            let p = if r.below(2) == 0 { 0 } else { r.next_u64() };
            let c = if r.below(2) == 0 { 0 } else { r.next_u32() };
            crate::exe::wr::<u64>(table + 0x48 + idx * 8, p);
            crate::exe::wr::<u32>(table + 0x10048 + idx * 4, c);
        }
        let count = r.below(8) as usize;
        let mut indices = vec![0u16; count.max(1)];
        for x in &mut indices {
            *x = r.below(16) as u16;
        }
        let query: [u64; 3] = [ctx as u64, count as u64, indices.as_ptr() as u64];
        let init = [r.next_u64(); 8];
        let (mut a, mut b) = (init, init);
        LOAD_CALL_ARG.store(0, std::sync::atomic::Ordering::Relaxed);
        let ra = o(query.as_ptr() as *const u8, a.as_mut_ptr() as *mut u8);
        let arg_a = LOAD_CALL_ARG.load(std::sync::atomic::Ordering::Relaxed);
        LOAD_CALL_ARG.store(0, std::sync::atomic::Ordering::Relaxed);
        let rb = n(query.as_ptr() as *const u8, b.as_mut_ptr() as *mut u8);
        let arg_b = LOAD_CALL_ARG.load(std::sync::atomic::Ordering::Relaxed);
        crate::diff_eq!(ra - a.as_ptr() as usize, rb - b.as_ptr() as usize, "ret case {case}");
        crate::diff_eq!(arg_a - a.as_ptr() as usize, arg_b - b.as_ptr() as usize, "load arg case {case}");
        crate::diff_eq!(a, b, "out case {case}");
    }
    Ok(CASES)
}

/// Look up a single component byte offset `C0` in `table` (`rdx`) for `group_idx` (`r8w`).
///
/// Originals: 1,999 functions of 88 bytes (e.g. `0x14184ff20`).
pub unsafe extern "win64" fn ecs_get_component_offset<const C0: u64>(
    _this: usize,
    table: usize,
    group_idx: u16,
) -> u64 {
    use crate::exe::rd;
    let g = group_idx as usize;
    let count = rd::<u32>(table.wrapping_add(32 * g + 0x1806c)) as u64;
    let ids_ptr = rd::<*const u16>(table.wrapping_add(32 * g + 0x18058));
    lookup_comp_offset(ids_ptr, count, table, g, C0) as u64
}

/// Check whether tag/empty component `C0` is present in `table` (`rdx`) for `group_idx` (`r8w`)
/// (`0x14186d090` family, 58 functions, 70 bytes). Returns `0xf1a9_f1a9_f1a9_f1a9` if present, else `0`.
pub unsafe extern "win64" fn ecs_has_tag_component<const C0: u64>(
    _this: usize,
    table: usize,
    group_idx: u16,
) -> u64 {
    use crate::exe::rd;
    let g = group_idx as usize;
    let count = rd::<u32>(table.wrapping_add(32 * g + 0x18068)) as u64;
    let ids_ptr = rd::<*const u16>(table.wrapping_add(32 * g + 0x18050));
    let comp_id = rd::<u16>(crate::exe::ptr::<u16>(C0) as usize);
    let pos = crate::exe::call_native!(FIND_U16_VA, crate::ns::r::algo::find_u16, (ids_ptr, count, comp_id) -> u64) as i32;
    if pos != -1 {
        0xf1a9_f1a9_f1a9_f1a9
    } else {
        0
    }
}

/// Look up 11 component byte offsets in `table` for `group_idx` and write them to `out[0..11]`.
///
/// Originals: 9 functions of 616 bytes (e.g. `0x14036d6c0`).
pub unsafe extern "win64" fn ecs_offset_getter_11<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10])
}

/// Look up 12 component byte offsets in `table` for `group_idx` and write them to `out[0..12]`.
///
/// Originals: 12 functions of 670 bytes (e.g. `0x1402447c0`).
pub unsafe extern "win64" fn ecs_offset_getter_12<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(out, table, group_idx, [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11])
}

/// Look up 13 component byte offsets in `table` for `group_idx` and write them to `out[0..13]`.
///
/// Originals: 6 functions of 722 bytes (e.g. `0x1402713e0`).
pub unsafe extern "win64" fn ecs_offset_getter_13<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
    const C12: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(
        out,
        table,
        group_idx,
        [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12],
    )
}

/// Look up 14 component byte offsets in `table` for `group_idx` and write them to `out[0..14]`.
///
/// Originals: 6 functions of 774 bytes (e.g. `0x14036f690`).
pub unsafe extern "win64" fn ecs_offset_getter_14<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
    const C12: u64,
    const C13: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(
        out,
        table,
        group_idx,
        [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13],
    )
}

/// Look up 16 component byte offsets in `table` for `group_idx` and write them to `out[0..16]`.
///
/// Originals: 4 functions of 878 bytes (e.g. `0x14022ec20`).
pub unsafe extern "win64" fn ecs_offset_getter_16<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
    const C12: u64,
    const C13: u64,
    const C14: u64,
    const C15: u64,
>(
    out: *mut usize,
    _rdx: usize,
    table: usize,
    group_idx: u16,
) -> *mut usize {
    ecs_offset_getter_n(
        out,
        table,
        group_idx,
        [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15],
    )
}

pub unsafe fn diff_ecs_get_component_offset<const C0: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(orig as u64);
    let p = crate::exe::ptr::<u16>(C0) as usize;
    let saved_id = crate::exe::rd::<u16>(p);
    let target_id = 0x55aau16;
    crate::exe::wr::<u16>(p, target_id);
    let mut table = vec![0u8; 32 * (0xc03 + 8) + 0x100];
    let t_base = table.as_mut_ptr() as usize;
    const CASES: u32 = 120;
    let res = (|| -> Result<u32, String> {
        for case in 0..CASES {
            let g = (case as usize) % 4;
            let count = (r.below(10) as usize).min(8);
            let mut ids = [0u16; 10];
            let mut offsets = [0u32; 10];
            for k in 0..count {
                ids[k] = if r.below(3) == 0 { target_id } else { r.next_u32() as u16 };
                offsets[k] = if r.below(4) == 0 { 0 } else { r.next_u32() };
            }
            crate::exe::wr::<u32>(t_base + 32 * g + 0x1806c, count as u32);
            crate::exe::wr::<usize>(t_base + 32 * g + 0x18058, ids.as_ptr() as usize);
            crate::exe::wr::<usize>(t_base + 32 * (g + 0xc03), offsets.as_ptr() as usize);
            let this = r.next_u64() as usize;
            let ra = o(this, t_base, g as u16);
            let rb = n(this, t_base, g as u16);
            crate::diff_eq!(ra, rb, "ret case {case}");
        }
        Ok(CASES)
    })();
    crate::exe::wr::<u16>(p, saved_id);
    res
}

pub unsafe fn diff_ecs_has_tag_component<const C0: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(orig as u64);
    let p = crate::exe::ptr::<u16>(C0) as usize;
    let saved_id = crate::exe::rd::<u16>(p);
    let target_id = 0x33ccu16;
    crate::exe::wr::<u16>(p, target_id);
    let mut table = vec![0u8; 0x18100];
    let t_base = table.as_mut_ptr() as usize;
    const CASES: u32 = 120;
    let res = (|| -> Result<u32, String> {
        for case in 0..CASES {
            let g = (case as usize) % 4;
            let count = (r.below(10) as usize).min(8);
            let mut ids = [0u16; 10];
            for k in 0..count {
                ids[k] = if r.below(3) == 0 { target_id } else { r.next_u32() as u16 };
            }
            crate::exe::wr::<u32>(t_base + 32 * g + 0x18068, count as u32);
            crate::exe::wr::<usize>(t_base + 32 * g + 0x18050, ids.as_ptr() as usize);
            let this = r.next_u64() as usize;
            let ra = o(this, t_base, g as u16);
            let rb = n(this, t_base, g as u16);
            crate::diff_eq!(ra, rb, "ret case {case}");
        }
        Ok(CASES)
    })();
    crate::exe::wr::<u16>(p, saved_id);
    res
}

pub unsafe fn diff_ecs_offset_getter_11<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10])
}

pub unsafe fn diff_ecs_offset_getter_12<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11])
}

pub unsafe fn diff_ecs_offset_getter_13<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
    const C12: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(orig, new, [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12])
}

pub unsafe fn diff_ecs_offset_getter_14<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
    const C12: u64,
    const C13: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(
        orig,
        new,
        [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13],
    )
}

pub unsafe fn diff_ecs_offset_getter_16<
    const C0: u64,
    const C1: u64,
    const C2: u64,
    const C3: u64,
    const C4: u64,
    const C5: u64,
    const C6: u64,
    const C7: u64,
    const C8: u64,
    const C9: u64,
    const C10: u64,
    const C11: u64,
    const C12: u64,
    const C13: u64,
    const C14: u64,
    const C15: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    diff_ecs_offset_getter(
        orig,
        new,
        [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15],
    )
}


