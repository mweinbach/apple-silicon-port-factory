use crate::exe::{self, rd, wr};

/// Original: `0x1420a0420` (46 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_bink_plane_name(plane: u32) -> usize {
    match plane {
        0 => exe::ptr::<u8>(0x1_433d_d608) as usize,
        1 => exe::ptr::<u8>(0x1_433d_d5e8) as usize,
        2 => exe::ptr::<u8>(0x1_433d_d5f8) as usize,
        _ => exe::ptr::<u8>(0x1_433d_d5d0) as usize,
    }
}

pub unsafe fn diff_d3d_bink_plane_name(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(u32) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(u32) -> usize = core::mem::transmute(new);
    for p in [0u32, 1, 2, 3, 4, u32::MAX] {
        let r_o = f_orig(p);
        let r_n = f_new(p);
        crate::diff_eq!(r_o, r_n, "bink_plane_name");
    }
    Ok(6)
}

/// Original: `0x1420a0460` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fx_release_handle_slot(this: usize) {
    let ptr = rd::<usize>(this);
    if ptr != 0 {
        let inner = rd::<usize>(ptr + 0x10);
        exe::call_va!(0x1_4209_d6f0, (inner, 0u32) -> ());
        wr::<usize>(this, 0);
    }
}

pub unsafe fn diff_d3d_fx_release_handle_slot(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let mut slot_o = 0usize;
    let mut slot_n = 0usize;
    f_orig((&mut slot_o as *mut usize) as usize);
    f_new((&mut slot_n as *mut usize) as usize);
    crate::diff_eq!(slot_o, slot_n, "null_slot");
    Ok(1)
}

/// Original: `0x1420a05d0` (107 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fx_init_resource_entry(this: usize, out: usize) -> usize {
    let arg5 = rd::<u64>(this + 0x10);
    let r8 = rd::<usize>(this + 0x8);
    let rcx = rd::<usize>(out + 0x10);
    exe::call_va!(0x1_4207_5bc0, (rcx, 0u32, r8, 0u32, arg5) -> ());
    let mut tmp = [0u8; 16];
    let rax = exe::call_va!(0x1_4207_34e0, (tmp.as_mut_ptr() as usize, 1u32) -> usize);
    wr::<[u8; 16]>(out, rd::<[u8; 16]>(rax));
    wr::<u64>(out + 0x18, rd::<u64>(this + 0x10));
    wr::<u64>(out + 0x20, rd::<u64>(this + 0x18));
    let map_ptr = rd::<usize>(this).wrapping_add(0x18);
    exe::call_va!(0x1_411e_96c0, (map_ptr, out) -> usize)
}

pub unsafe fn diff_d3d_fx_init_resource_entry(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_75bc0(rcx: usize, rdx: u32, r8: usize, r9: u32, arg5: u64) {
        wr::<u32>(rcx, rdx.wrapping_add(0x1111));
        wr::<usize>(rcx + 0x8, r8);
        wr::<u32>(rcx + 0x10, r9.wrapping_add(0x2222));
        wr::<u64>(rcx + 0x18, arg5);
    }
    unsafe extern "win64" fn fake_734e0(rcx: usize, rdx: u32) -> usize {
        for i in 0..16usize {
            wr::<u8>(rcx + i, (i as u8).wrapping_add(rdx as u8).wrapping_mul(7));
        }
        rcx
    }
    unsafe extern "win64" fn fake_96c0(map_ptr: usize, out: usize) -> usize {
        wr::<u64>(map_ptr, rd::<u64>(out).wrapping_add(rd::<u64>(out + 0x18)));
        wr::<u64>(map_ptr + 0x8, rd::<u64>(out + 0x20));
        0x7788_99aausize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4207_5bc0, fake_75bc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_34e0, fake_734e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_96c0, fake_96c0 as *const ())?;

    let f_orig: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(new);

    let mut map_o = [0u8; 48];
    let mut map_n = [0u8; 48];
    let mut inner_o = [0u8; 32];
    let mut inner_n = [0u8; 32];
    let mut this_o = [0u8; 32];
    let mut this_n = [0u8; 32];
    let mut out_o = [0u8; 40];
    let mut out_n = [0u8; 40];

    wr::<usize>(this_o.as_mut_ptr() as usize, map_o.as_mut_ptr() as usize);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x8, 0x1234_5678);
    wr::<u64>(this_o.as_mut_ptr() as usize + 0x10, 0x9abc_def0_1122_3344);
    wr::<u64>(this_o.as_mut_ptr() as usize + 0x18, 0x5566_7788_99aa_bbcc);
    wr::<usize>(out_o.as_mut_ptr() as usize + 0x10, inner_o.as_mut_ptr() as usize);

    wr::<usize>(this_n.as_mut_ptr() as usize, map_n.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x8, 0x1234_5678);
    wr::<u64>(this_n.as_mut_ptr() as usize + 0x10, 0x9abc_def0_1122_3344);
    wr::<u64>(this_n.as_mut_ptr() as usize + 0x18, 0x5566_7788_99aa_bbcc);
    wr::<usize>(out_n.as_mut_ptr() as usize + 0x10, inner_n.as_mut_ptr() as usize);

    let r_o = f_orig(this_o.as_mut_ptr() as usize, out_o.as_mut_ptr() as usize);
    let r_n = f_new(this_n.as_mut_ptr() as usize, out_n.as_mut_ptr() as usize);

    crate::diff_eq!(r_o, r_n, "init_resource_entry ret");
    crate::diff_eq!(&out_o[0..0x10], &out_n[0..0x10], "init_resource_entry out[0..16]");
    crate::diff_eq!(&out_o[0x18..0x28], &out_n[0x18..0x28], "init_resource_entry out[24..40]");
    crate::diff_eq!(inner_o, inner_n, "init_resource_entry inner");
    crate::diff_eq!(map_o, map_n, "init_resource_entry map");
    Ok(1)
}

/// Original: `0x1420a0a40` (211 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fx_clear_dual_resource_arrays(this: usize) {
    let mut cur = rd::<usize>(this + 0x8);
    let count0 = rd::<u32>(this + 0x10) as usize;
    let end0 = cur.wrapping_add(count0.wrapping_mul(0x28));
    while cur != end0 {
        let obj = rd::<usize>(cur + 0x10);
        if obj != 0 {
            exe::call_va!(0x1_4209_d670, (obj) -> ());
            exe::call_va!(0x1_411e_7fc0, (obj.wrapping_add(0x58)) -> ());
            exe::call_va!(0x1_4244_2de0, (obj) -> ());
        }
        wr::<usize>(cur + 0x10, 0);
        cur = cur.wrapping_add(0x28);
    }

    let mut cur1 = rd::<usize>(this + 0x18);
    let count1 = rd::<u32>(this + 0x20) as usize;
    let end1 = cur1.wrapping_add(count1.wrapping_mul(0x28));
    while cur1 != end1 {
        let obj = rd::<usize>(cur1 + 0x10);
        if obj != 0 {
            exe::call_va!(0x1_4209_d670, (obj) -> ());
            exe::call_va!(0x1_411e_7fc0, (obj.wrapping_add(0x58)) -> ());
            exe::call_va!(0x1_4244_2de0, (obj) -> ());
        }
        wr::<usize>(cur1 + 0x10, 0);
        cur1 = cur1.wrapping_add(0x28);
    }

    exe::call_va!(0x1_401d_e830, (this.wrapping_add(0x18)) -> ());
    exe::call_va!(0x1_401d_e830, (this.wrapping_add(0x8)) -> ());
}

pub unsafe fn diff_d3d_fx_clear_dual_resource_arrays(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_d670(obj: usize) {
        wr::<u32>(obj, rd::<u32>(obj).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_7fc0(p58: usize) {
        wr::<u32>(p58, rd::<u32>(p58).wrapping_add(0x20));
    }
    unsafe extern "win64" fn fake_2de0(obj: usize) {
        wr::<u32>(obj + 0x4, rd::<u32>(obj + 0x4).wrapping_add(0x30));
    }
    unsafe extern "win64" fn fake_e830(vec_hdr: usize) {
        wr::<usize>(vec_hdr, 0);
        wr::<u64>(vec_hdr + 0x8, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_d670, fake_d670 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411e_7fc0, fake_7fc0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4244_2de0, fake_2de0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_e830, fake_e830 as *const ())?;

    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);

    let mut obj0_o = [0u8; 0x60];
    let mut obj0_n = [0u8; 0x60];
    let mut obj1_o = [0u8; 0x60];
    let mut obj1_n = [0u8; 0x60];
    let mut arr0_o = [0u8; 0x50];
    let mut arr0_n = [0u8; 0x50];
    let mut arr1_o = [0u8; 0x28];
    let mut arr1_n = [0u8; 0x28];
    let mut this_o = [0u8; 0x30];
    let mut this_n = [0u8; 0x30];

    wr::<usize>(arr0_o.as_mut_ptr() as usize + 0x10, obj0_o.as_mut_ptr() as usize);
    wr::<usize>(arr0_o.as_mut_ptr() as usize + 0x28 + 0x10, 0);
    wr::<usize>(arr1_o.as_mut_ptr() as usize + 0x10, obj1_o.as_mut_ptr() as usize);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x8, arr0_o.as_mut_ptr() as usize);
    wr::<u32>(this_o.as_mut_ptr() as usize + 0x10, 2);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x18, arr1_o.as_mut_ptr() as usize);
    wr::<u32>(this_o.as_mut_ptr() as usize + 0x20, 1);

    wr::<usize>(arr0_n.as_mut_ptr() as usize + 0x10, obj0_n.as_mut_ptr() as usize);
    wr::<usize>(arr0_n.as_mut_ptr() as usize + 0x28 + 0x10, 0);
    wr::<usize>(arr1_n.as_mut_ptr() as usize + 0x10, obj1_n.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x8, arr0_n.as_mut_ptr() as usize);
    wr::<u32>(this_n.as_mut_ptr() as usize + 0x10, 2);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x18, arr1_n.as_mut_ptr() as usize);
    wr::<u32>(this_n.as_mut_ptr() as usize + 0x20, 1);

    f_orig(this_o.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize);

    crate::diff_eq!(this_o, this_n, "clear_dual_resource_arrays this");
    crate::diff_eq!(arr0_o, arr0_n, "clear_dual_resource_arrays arr0");
    crate::diff_eq!(arr1_o, arr1_n, "clear_dual_resource_arrays arr1");
    crate::diff_eq!(obj0_o, obj0_n, "clear_dual_resource_arrays obj0");
    crate::diff_eq!(obj1_o, obj1_n, "clear_dual_resource_arrays obj1");
    Ok(1)
}

/// Original: `0x1420a1150` (109 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_mesh_apply_tag_to_u16_span_a(
    span: usize,
    tag: u16,
    table: usize,
) {
    let count = rd::<usize>(span);
    let mut cur = rd::<usize>(span + 0x8);
    let end = cur.wrapping_add(count.wrapping_mul(2));
    if cur != end {
        let base = rd::<usize>(table + 0x8);
        let mut tag_slot: u16 = tag;
        let tag_ptr = (&mut tag_slot as *mut u16) as usize;
        while cur != end {
            let idx = rd::<u16>(cur) as usize;
            let entry = base.wrapping_add(idx << 4);
            exe::call_va!(0x1_411e_9ea0, (entry, tag_ptr) -> ());
            cur = cur.wrapping_add(2);
        }
    }
}

pub unsafe fn diff_d3d_mesh_apply_tag_to_u16_span_a(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_9ea0(entry: usize, tag_ptr: usize) {
        let tag = rd::<u16>(tag_ptr);
        wr::<u16>(entry, rd::<u16>(entry).wrapping_add(tag));
    }
    let _m = crate::difftest::Mock::new(0x1_411e_9ea0, fake_9ea0 as *const ())?;
    let f_orig: unsafe extern "win64" fn(usize, u16, usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, u16, usize) = core::mem::transmute(new);

    let indices: [u16; 3] = [2, 0, 3];
    let mut entries_o = [0u8; 64];
    let mut entries_n = [0u8; 64];
    let table_o = [0usize, entries_o.as_mut_ptr() as usize];
    let table_n = [0usize, entries_n.as_mut_ptr() as usize];

    let empty_span = [0usize, indices.as_ptr() as usize];
    f_orig(empty_span.as_ptr() as usize, 0x1234, table_o.as_ptr() as usize);
    f_new(empty_span.as_ptr() as usize, 0x1234, table_n.as_ptr() as usize);
    crate::diff_eq!(entries_o, entries_n, "span_a empty");

    let span = [3usize, indices.as_ptr() as usize];
    f_orig(span.as_ptr() as usize, 0x1234, table_o.as_ptr() as usize);
    f_new(span.as_ptr() as usize, 0x1234, table_n.as_ptr() as usize);
    crate::diff_eq!(entries_o, entries_n, "span_a 3 items");
    Ok(2)
}

/// Original: `0x1420a11d0` (109 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_mesh_apply_tag_to_u16_span_b(
    span: usize,
    tag: u16,
    table: usize,
) {
    let count = rd::<usize>(span);
    let mut cur = rd::<usize>(span + 0x8);
    let end = cur.wrapping_add(count.wrapping_mul(2));
    if cur != end {
        let base = rd::<usize>(table + 0x8);
        let mut tag_slot: u16 = tag;
        let tag_ptr = (&mut tag_slot as *mut u16) as usize;
        while cur != end {
            let idx = rd::<u16>(cur) as usize;
            let entry = base.wrapping_add(idx << 4);
            exe::call_va!(0x1_411e_9ea0, (entry, tag_ptr) -> ());
            cur = cur.wrapping_add(2);
        }
    }
}

pub unsafe fn diff_d3d_mesh_apply_tag_to_u16_span_b(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_9ea0(entry: usize, tag_ptr: usize) {
        let tag = rd::<u16>(tag_ptr);
        wr::<u16>(entry + 2, rd::<u16>(entry + 2).wrapping_add(tag));
    }
    let _m = crate::difftest::Mock::new(0x1_411e_9ea0, fake_9ea0 as *const ())?;
    let f_orig: unsafe extern "win64" fn(usize, u16, usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, u16, usize) = core::mem::transmute(new);

    let indices: [u16; 3] = [1, 3, 0];
    let mut entries_o = [0u8; 64];
    let mut entries_n = [0u8; 64];
    let table_o = [0usize, entries_o.as_mut_ptr() as usize];
    let table_n = [0usize, entries_n.as_mut_ptr() as usize];

    let empty_span = [0usize, indices.as_ptr() as usize];
    f_orig(empty_span.as_ptr() as usize, 0x5678, table_o.as_ptr() as usize);
    f_new(empty_span.as_ptr() as usize, 0x5678, table_n.as_ptr() as usize);
    crate::diff_eq!(entries_o, entries_n, "span_b empty");

    let span = [3usize, indices.as_ptr() as usize];
    f_orig(span.as_ptr() as usize, 0x5678, table_o.as_ptr() as usize);
    f_new(span.as_ptr() as usize, 0x5678, table_n.as_ptr() as usize);
    crate::diff_eq!(entries_o, entries_n, "span_b 3 items");
    Ok(2)
}

/// Original: `0x1420a1450` (121 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_any_entry_matches_pair(
    a: usize,
    b: usize,
    list: usize,
) -> u8 {
    let count = rd::<usize>(list);
    let mut cur = rd::<usize>(list + 0x8);
    let end = cur.wrapping_add(count.wrapping_mul(0x48));
    if cur == end {
        return 0;
    }
    #[repr(C, align(16))]
    struct Aligned16([u8; 16]);
    let mut local_b = Aligned16(rd::<[u8; 16]>(b));
    let mut local_a = Aligned16(rd::<[u8; 16]>(a));
    let ptr_a = local_a.0.as_mut_ptr() as usize;
    let ptr_b = local_b.0.as_mut_ptr() as usize;
    while cur != end {
        let hit = exe::call_va!(0x1_420a_14d0, (ptr_a, ptr_b, cur) -> u8);
        if hit != 0 {
            return 1;
        }
        cur = cur.wrapping_add(0x48);
    }
    0
}

pub unsafe fn diff_d3d_mesh_any_entry_matches_pair(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize, usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize, usize) -> u8 = core::mem::transmute(new);
    let a = [0u8; 16];
    let b = [0u8; 16];
    let empty_list = [0usize, 0x1000usize];
    let r_o = f_orig(
        a.as_ptr() as usize,
        b.as_ptr() as usize,
        empty_list.as_ptr() as usize,
    );
    let r_n = f_new(
        a.as_ptr() as usize,
        b.as_ptr() as usize,
        empty_list.as_ptr() as usize,
    );
    crate::diff_eq!(r_o, r_n, "empty_list");
    Ok(1)
}

/// Original: `0x1420a1610` (153 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_any_span_pair_matches_1710(
    span_a: usize,
    span_b: usize,
) -> u8 {
    let count_a = rd::<usize>(span_a);
    let mut cur_a = rd::<usize>(span_a + 0x8);
    let end_a = cur_a.wrapping_add(count_a.wrapping_mul(0x48));
    if cur_a == end_a {
        return 0;
    }
    let count_b = rd::<usize>(span_b);
    let start_b = rd::<usize>(span_b + 0x8);
    let end_b = start_b.wrapping_add(count_b.wrapping_mul(0x48));
    while cur_a != end_a {
        let mut cur_b = start_b;
        while cur_b != end_b {
            let hit = exe::call_va!(0x1_420a_1710, (cur_a, cur_b) -> u8);
            if hit != 0 {
                return 1;
            }
            cur_b = cur_b.wrapping_add(0x48);
        }
        cur_a = cur_a.wrapping_add(0x48);
    }
    0
}

pub unsafe fn diff_d3d_mesh_any_span_pair_matches_1710(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);
    let empty_a = [0usize, 0x2000usize];
    let empty_b = [0usize, 0x3000usize];
    let r_o = f_orig(empty_a.as_ptr() as usize, empty_b.as_ptr() as usize);
    let r_n = f_new(empty_a.as_ptr() as usize, empty_b.as_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "empty_spans");
    Ok(1)
}

/// Original: `0x1420a16b0` (87 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_any_entry_matches_1710(item: usize, span: usize) -> u8 {
    let count = rd::<usize>(span);
    let mut cur = rd::<usize>(span + 0x8);
    let end = cur.wrapping_add(count.wrapping_mul(0x48));
    while cur != end {
        let hit = exe::call_va!(0x1_420a_1710, (item, cur) -> u8);
        if hit != 0 {
            return 1;
        }
        cur = cur.wrapping_add(0x48);
    }
    0
}

pub unsafe fn diff_d3d_mesh_any_entry_matches_1710(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);
    let empty = [0usize, 0x2000usize];
    let r_o = f_orig(0, empty.as_ptr() as usize);
    let r_n = f_new(0, empty.as_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "empty_span");
    Ok(1)
}

/// Original: `0x1420a2280` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_insert_named_binding(this: usize, key_ptr: usize) {
    let tag = rd::<u32>(this + 0x10) as u16;
    let slot = exe::call_va!(0x1_411e_9f50, (this.wrapping_add(0x8), key_ptr) -> usize);
    let cstr = rd::<usize>(key_ptr);
    wr::<u16>(slot, tag);
    let mut len = 0usize;
    while rd::<u8>(cstr.wrapping_add(len)) != 0 {
        len = len.wrapping_add(1);
    }
    wr::<usize>(slot + 0x8, cstr);
    wr::<usize>(slot + 0x10, len);
}

pub unsafe fn diff_d3d_mesh_insert_named_binding(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_9f50(map_ptr: usize, _key_ptr: usize) -> usize {
        rd::<usize>(map_ptr)
    }
    let _m = crate::difftest::Mock::new(0x1_411e_9f50, fake_9f50 as *const ())?;
    let f_orig: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(new);

    for (s, tag) in [(b"albedo_tex\0" as &[u8], 0x4321u32), (b"\0", 0x9999u32)] {
        let mut slot_o = [0u8; 24];
        let mut slot_n = [0u8; 24];
        let mut this_o = [0u8; 24];
        let mut this_n = [0u8; 24];
        wr::<usize>(this_o.as_mut_ptr() as usize + 0x8, slot_o.as_mut_ptr() as usize);
        wr::<u32>(this_o.as_mut_ptr() as usize + 0x10, tag);
        wr::<usize>(this_n.as_mut_ptr() as usize + 0x8, slot_n.as_mut_ptr() as usize);
        wr::<u32>(this_n.as_mut_ptr() as usize + 0x10, tag);

        let key = [s.as_ptr() as usize];
        f_orig(this_o.as_mut_ptr() as usize, key.as_ptr() as usize);
        f_new(this_n.as_mut_ptr() as usize, key.as_ptr() as usize);
        crate::diff_eq!(slot_o, slot_n, "insert_named_binding");
    }
    Ok(2)
}

/// Original: `0x1420a2320` (125 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_iter_advance_stride12(iter: usize) {
    let container0 = rd::<usize>(iter);
    let mut cur = rd::<usize>(iter + 0x8);
    let count0 = rd::<usize>(container0);
    let base0 = rd::<usize>(container0 + 0x8);
    let end0 = base0.wrapping_add(count0.wrapping_mul(12));
    if cur == end0 {
        return;
    }
    loop {
        wr::<usize>(iter + 0x10, cur);
        wr::<u8>(iter + 0x18, 1);
        let hi = rd::<i32>(cur + 0x4);
        if hi > 1 {
            break;
        }
        cur = rd::<usize>(iter + 0x8).wrapping_add(0xc);
        wr::<usize>(iter + 0x8, cur);
        let container = rd::<usize>(iter);
        let count = rd::<usize>(container);
        let base = rd::<usize>(container + 0x8);
        let end = base.wrapping_add(count.wrapping_mul(12));
        if cur == end {
            break;
        }
    }
}

pub unsafe fn diff_d3d_mesh_iter_advance_stride12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Entry12 {
        a: i32,
        b: i32,
        c: i32,
    }
    let entries = [
        Entry12 { a: 10, b: 1, c: 0 },
        Entry12 { a: 20, b: 2, c: 0 },
        Entry12 { a: 30, b: 0, c: 0 },
    ];
    let container = [3usize, entries.as_ptr() as usize];
    let mut iter_o = [container.as_ptr() as usize, entries.as_ptr() as usize, 0usize, 0usize];
    let mut iter_n = iter_o;

    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "iter_advance_stride12");
    Ok(1)
}

/// Original: `0x1420a2470` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_entry_hi_gt_one(_this: usize, entry: usize) -> u8 {
    (rd::<i32>(entry + 0x4) > 1) as u8
}

pub unsafe fn diff_d3d_mesh_entry_hi_gt_one(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);
    for v in [-5i32, 0, 1, 2, 100] {
        let pair = [0i32, v];
        let ptr = pair.as_ptr() as usize;
        let r_o = f_orig(0, ptr);
        let r_n = f_new(0, ptr);
        crate::diff_eq!(r_o, r_n, "entry_hi_gt_one");
    }
    Ok(5)
}

/// Original: `0x1420a2ae0` (28 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_iter_at_end(iter: usize) -> u8 {
    let container = rd::<usize>(iter);
    let count = rd::<usize>(container);
    let base = rd::<usize>(container + 0x8);
    let end = base.wrapping_add(count.wrapping_mul(2));
    (rd::<usize>(iter + 0x8) == end) as u8
}

pub unsafe fn diff_d3d_mesh_u16_iter_at_end(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(new);
    let container = [4usize, 0x1000usize];
    for cur in [0x1000usize, 0x1004usize, 0x1008usize] {
        let iter = [container.as_ptr() as usize, cur];
        let r_o = f_orig(iter.as_ptr() as usize);
        let r_n = f_new(iter.as_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "u16_iter_at_end");
    }
    Ok(3)
}

/// Original: `0x1420a2b20` (57 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_iter_eq_sentinel(iter: usize, sentinel: usize) -> u8 {
    if rd::<u8>(iter + 0x20) == 0 && rd::<u8>(sentinel + 0x20) == 1 {
        let container = rd::<usize>(iter);
        let count = rd::<usize>(container);
        let base = rd::<usize>(container + 0x8);
        let end = base.wrapping_add(count.wrapping_mul(2));
        return (rd::<usize>(iter + 0x8) == end) as u8;
    }
    exe::call_va!(0x1_4020_0070, () -> ());
    unreachable!()
}

pub unsafe fn diff_d3d_mesh_u16_iter_eq_sentinel(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);
    let container = [3usize, 0x2000usize];
    let mut sentinel = [0u8; 40];
    sentinel[0x20] = 1;
    for cur in [0x2000usize, 0x2006usize] {
        let mut iter = [0u8; 40];
        let c_ptr = container.as_ptr() as usize;
        iter[0..8].copy_from_slice(&c_ptr.to_le_bytes());
        iter[8..16].copy_from_slice(&cur.to_le_bytes());
        iter[0x20] = 0;
        let r_o = f_orig(iter.as_ptr() as usize, sentinel.as_ptr() as usize);
        let r_n = f_new(iter.as_ptr() as usize, sentinel.as_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "u16_iter_eq_sentinel");
    }
    Ok(2)
}

/// Original: `0x1420a2ba0` (37 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_mesh_u16_iter_next(iter: usize) -> usize {
    if rd::<u8>(iter + 0x20) == 0 {
        let next = rd::<usize>(iter + 0x8).wrapping_add(2);
        wr::<usize>(iter + 0x8, next);
        exe::call_va!(0x1_420a_3250, (iter) -> ());
        return iter;
    }
    exe::call_va!(0x1_4020_0070, () -> ());
    unreachable!()
}

pub unsafe fn diff_d3d_mesh_u16_iter_next(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(new);

    let indices: [u16; 3] = [0, 0, 1];
    let mut entries = [0u8; 128];
    let mut items = [0u8; 96];
    wr::<u16>(entries.as_mut_ptr() as usize + 0x22, 0);
    wr::<u16>(entries.as_mut_ptr() as usize + 0x40 + 0x22, 1);
    wr::<u8>(items.as_mut_ptr() as usize + 0x20, 0);
    wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x20, 1);

    let rcx0 = [entries.as_ptr() as usize];
    let rcx1 = [0usize, 0usize, items.as_ptr() as usize];
    let rdx = [rcx0.as_ptr() as usize, 0usize, rcx1.as_ptr() as usize];
    let container = [3usize, indices.as_ptr() as usize, rdx.as_ptr() as usize];

    let mut iter_o = [0u8; 40];
    wr::<usize>(iter_o.as_mut_ptr() as usize, container.as_ptr() as usize);
    wr::<usize>(iter_o.as_mut_ptr() as usize + 0x8, indices.as_ptr() as usize);
    wr::<u8>(iter_o.as_mut_ptr() as usize + 0x20, 0);
    let mut iter_n = iter_o;

    let r_o = f_orig(iter_o.as_mut_ptr() as usize);
    let r_n = f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o == iter_o.as_ptr() as usize, r_n == iter_n.as_ptr() as usize, "u16_iter_next ret");
    crate::diff_eq!(iter_o, iter_n, "u16_iter_next skip inactive");

    let r_o2 = f_orig(iter_o.as_mut_ptr() as usize);
    let r_n2 = f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o2 == iter_o.as_ptr() as usize, r_n2 == iter_n.as_ptr() as usize, "u16_iter_next ret end");
    crate::diff_eq!(iter_o, iter_n, "u16_iter_next at end");
    Ok(2)
}

/// Original: `0x1420a2bd0` (51 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_iter_ne_sentinel(iter: usize, sentinel: usize) -> u8 {
    if rd::<u8>(iter + 0x20) == 0 && rd::<u8>(sentinel + 0x20) == 1 {
        let container = rd::<usize>(iter);
        let count = rd::<usize>(container);
        let base = rd::<usize>(container + 0x8);
        let end = base.wrapping_add(count.wrapping_mul(2));
        return (rd::<usize>(iter + 0x8) != end) as u8;
    }
    exe::call_va!(0x1_4020_0070, () -> ());
    unreachable!()
}

pub unsafe fn diff_d3d_mesh_u16_iter_ne_sentinel(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);
    let container = [3usize, 0x2000usize];
    let mut sentinel = [0u8; 40];
    sentinel[0x20] = 1;
    for cur in [0x2000usize, 0x2006usize] {
        let mut iter = [0u8; 40];
        let c_ptr = container.as_ptr() as usize;
        iter[0..8].copy_from_slice(&c_ptr.to_le_bytes());
        iter[8..16].copy_from_slice(&cur.to_le_bytes());
        iter[0x20] = 0;
        let r_o = f_orig(iter.as_ptr() as usize, sentinel.as_ptr() as usize);
        let r_n = f_new(iter.as_ptr() as usize, sentinel.as_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "u16_iter_ne_sentinel");
    }
    Ok(2)
}

/// Original: `0x1420a2c30` (25 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_iter_deref(iter: usize) -> usize {
    if rd::<u8>(iter + 0x20) == 0 {
        return rd::<usize>(iter + 0x10);
    }
    exe::call_va!(0x1_4020_0070, () -> ());
    unreachable!()
}

pub unsafe fn diff_d3d_mesh_u16_iter_deref(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(new);
    let mut iter = [0u8; 40];
    let expected = 0xdead_beef_cafe_bab0usize;
    iter[0x10..0x18].copy_from_slice(&expected.to_le_bytes());
    iter[0x20] = 0;
    let r_o = f_orig(iter.as_ptr() as usize);
    let r_n = f_new(iter.as_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "u16_iter_deref");
    Ok(1)
}

/// Original: `0x1420a2cc0` (51 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_opt_iter_copy(dst: usize, src: usize) {
    let tag = rd::<i8>(src + 0x20) as i64;
    if tag.wrapping_add(1) == 0 {
        return;
    }
    if tag == 0 {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(dst + 0x8, rd::<u64>(src + 0x8));
        wr::<[u8; 16]>(dst + 0x10, rd::<[u8; 16]>(src + 0x10));
        wr::<u8>(dst + 0x20, 0);
    } else {
        wr::<u8>(dst + 0x20, 1);
    }
}

pub unsafe fn diff_d3d_mesh_opt_iter_copy(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(new);
    for tag in [0u8, 1u8, 0xffu8] {
        let mut src = [0u8; 40];
        for (i, b) in src[..32].iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(10);
        }
        src[0x20] = tag;
        let mut dst_o = [0x5au8; 40];
        let mut dst_n = [0x5au8; 40];
        f_orig(dst_o.as_mut_ptr() as usize, src.as_ptr() as usize);
        f_new(dst_n.as_mut_ptr() as usize, src.as_ptr() as usize);
        crate::diff_eq!(dst_o, dst_n, "opt_iter_copy");
    }
    Ok(3)
}

/// Original: `0x1420a2d70` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_opt_iter_from_value(dst: usize, src: usize) -> usize {
    wr::<u64>(dst, rd::<u64>(src));
    wr::<u64>(dst + 0x8, rd::<u64>(src + 0x8));
    wr::<[u8; 16]>(dst + 0x10, rd::<[u8; 16]>(src + 0x10));
    wr::<u8>(dst + 0x20, 0);
    dst
}

pub unsafe fn diff_d3d_mesh_opt_iter_from_value(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(new);
    let mut src = [0u8; 32];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(3);
    }
    let mut dst_o = [0xffu8; 40];
    let mut dst_n = [0xffu8; 40];
    let r_o = f_orig(dst_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    let r_n = f_new(dst_n.as_mut_ptr() as usize, src.as_ptr() as usize);
    crate::diff_eq!(r_o == dst_o.as_ptr() as usize, r_n == dst_n.as_ptr() as usize, "ret");
    crate::diff_eq!(dst_o, dst_n, "opt_iter_from_value");
    Ok(1)
}

/// Original: `0x1420a2da0` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_opt_iter_set_sentinel(dst: usize) -> usize {
    wr::<u8>(dst + 0x20, 1);
    dst
}

pub unsafe fn diff_d3d_mesh_opt_iter_set_sentinel(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(new);
    let mut dst_o = [0u8; 40];
    let mut dst_n = [0u8; 40];
    f_orig(dst_o.as_mut_ptr() as usize);
    f_new(dst_n.as_mut_ptr() as usize);
    crate::diff_eq!(dst_o, dst_n, "opt_iter_set_sentinel");
    Ok(1)
}

/// Original: `0x1420a30a0` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_list_iter_next(iter: usize) -> usize {
    let node = rd::<usize>(iter + 0x8);
    let next = rd::<usize>(node);
    wr::<usize>(iter + 0x8, next);
    iter
}

pub unsafe fn diff_d3d_mesh_list_iter_next(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(new);
    let node = [0x1234_5678usize];
    let mut iter_o = [0usize, node.as_ptr() as usize];
    let mut iter_n = [0usize, node.as_ptr() as usize];
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "list_iter_next");
    Ok(1)
}

/// Original: `0x1420a30c0` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_list_iter_ne(iter: usize, end: usize) -> u8 {
    (rd::<usize>(iter + 0x8) != end) as u8
}

pub unsafe fn diff_d3d_mesh_list_iter_ne(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);
    let iter = [0usize, 0x4000usize];
    for end in [0x4000usize, 0x5000usize] {
        let r_o = f_orig(iter.as_ptr() as usize, end);
        let r_n = f_new(iter.as_ptr() as usize, end);
        crate::diff_eq!(r_o, r_n, "list_iter_ne");
    }
    Ok(2)
}

/// Original: `0x1420a3100` (11 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_list_iter_get_hi16(iter: usize) -> u32 {
    let node = rd::<usize>(iter + 0x8);
    rd::<u32>(node + 0x10) >> 16
}

pub unsafe fn diff_d3d_mesh_list_iter_get_hi16(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> u32 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> u32 = core::mem::transmute(new);
    let node = [0u32, 0, 0, 0, 0xabcd_1234u32];
    let iter = [0usize, node.as_ptr() as usize];
    let r_o = f_orig(iter.as_ptr() as usize);
    let r_n = f_new(iter.as_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "list_iter_get_hi16");
    Ok(1)
}

/// Original: `0x1420a31e0` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_slot_iter_skip_flagged(iter: usize) {
    let c0 = rd::<usize>(iter);
    let mut idx = rd::<usize>(iter + 0x10);
    if idx == rd::<usize>(c0 + 0x8) {
        return;
    }
    loop {
        wr::<u8>(iter + 0x1a, 1);
        let idx16 = idx as u16;
        wr::<u16>(iter + 0x18, idx16);
        let c = rd::<usize>(iter);
        let sub = rd::<usize>(c + 0x18);
        let arr = rd::<usize>(sub + 0x10);
        if rd::<u8>(arr.wrapping_add((idx16 as usize).wrapping_mul(4).wrapping_add(2))) == 0 {
            break;
        }
        idx = rd::<usize>(iter + 0x10).wrapping_add(1);
        wr::<usize>(iter + 0x10, idx);
        if idx == rd::<usize>(c + 0x8) {
            break;
        }
    }
}

pub unsafe fn diff_d3d_mesh_slot_iter_skip_flagged(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let flags = [[0u8, 0, 1, 0], [0u8, 0, 0, 0]];
    let sub = [0usize, 0usize, flags.as_ptr() as usize];
    let container = [0usize, 2usize, 0usize, sub.as_ptr() as usize];
    let mut iter_o = [0u8; 32];
    iter_o[0..8].copy_from_slice(&(container.as_ptr() as usize).to_le_bytes());
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "slot_iter_skip_flagged");
    Ok(1)
}

/// Original: `0x1420a3250` (117 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_iter_skip_inactive(iter: usize) {
    let c0 = rd::<usize>(iter);
    let mut cur = rd::<usize>(iter + 0x8);
    let cnt0 = rd::<usize>(c0);
    let base0 = rd::<usize>(c0 + 0x8);
    if cur == base0.wrapping_add(cnt0.wrapping_mul(2)) {
        return;
    }
    loop {
        wr::<usize>(iter + 0x10, cur);
        wr::<u8>(iter + 0x18, 1);
        let r10 = rd::<usize>(iter);
        let idx = rd::<u16>(cur) as usize;
        let rdx = rd::<usize>(r10 + 0x10);
        let rcx0 = rd::<usize>(rdx);
        let entry = rd::<usize>(rcx0).wrapping_add(idx << 6);
        let sub_idx = rd::<u16>(entry + 0x22) as usize;
        let rcx1 = rd::<usize>(rdx + 0x10);
        let item = rd::<usize>(rcx1 + 0x10).wrapping_add(sub_idx.wrapping_mul(0x30));
        if rd::<u8>(item + 0x20) != 0 {
            break;
        }
        cur = rd::<usize>(iter + 0x8).wrapping_add(2);
        wr::<usize>(iter + 0x8, cur);
        let cnt = rd::<usize>(r10);
        let base = rd::<usize>(r10 + 0x8);
        if cur == base.wrapping_add(cnt.wrapping_mul(2)) {
            break;
        }
    }
}

pub unsafe fn diff_d3d_mesh_u16_iter_skip_inactive(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let container = [0usize, 0x1000usize, 0usize];
    let mut iter_o = [container.as_ptr() as usize, 0x1000usize, 0usize, 0usize];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "u16_iter_skip_inactive_empty");
    Ok(1)
}

/// Original: `0x1420a32f0` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_subiter_at_end(iter: usize) -> u8 {
    let container = rd::<usize>(iter);
    (rd::<u16>(iter + 0x8) == rd::<u16>(container + 0x2)) as u8
}

pub unsafe fn diff_d3d_mesh_u16_subiter_at_end(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(new);
    let container = [0u16, 5u16];
    for idx in [0u16, 5u16] {
        let mut iter = [0u8; 16];
        iter[0..8].copy_from_slice(&(container.as_ptr() as usize).to_le_bytes());
        iter[8..10].copy_from_slice(&idx.to_le_bytes());
        let r_o = f_orig(iter.as_ptr() as usize);
        let r_n = f_new(iter.as_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "u16_subiter_at_end");
    }
    Ok(2)
}

/// Original: `0x1420a3310` (165 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_subiter_advance(iter: usize) {
    let c0 = rd::<usize>(iter);
    let mut idx = rd::<u16>(iter + 0x8);
    if idx == rd::<u16>(c0 + 0x2) {
        return;
    }
    loop {
        wr::<u16>(iter + 0xa, idx);
        wr::<u8>(iter + 0xc, 1);
        let r10 = rd::<usize>(iter);
        let r9 = idx as usize;
        let r8 = rd::<usize>(r10 + 0x8);
        let r11 = rd::<u16>(r10 + 0x10);
        let rdx0 = rd::<usize>(r8);
        let rax = rd::<usize>(rdx0).wrapping_add((r11 as usize) << 6);
        let rdx1 = rd::<usize>(rdx0 + 0x10);
        let rbx = rdx1.wrapping_add(r9.wrapping_mul(8));
        let rdx2 = rd::<usize>(r8 + 0x18);
        let r8_arr = rd::<usize>(rdx2);
        if rd::<u32>(rbx + 0x4) == 2 {
            let dx = rd::<u16>(rbx + 0x2);
            if dx == r11 {
                break;
            }
            if dx == 0xffff && rd::<u16>(r8_arr.wrapping_add(r9.wrapping_mul(2))) == rd::<u16>(rax + 0x22) {
                break;
            }
        }
        idx = rd::<u16>(iter + 0x8).wrapping_add(1);
        wr::<u16>(iter + 0x8, idx);
        if idx == rd::<u16>(r10 + 0x2) {
            break;
        }
    }
}

pub unsafe fn diff_d3d_mesh_u16_subiter_advance(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let container = [0u16, 3u16];
    let mut iter_o = [0u8; 16];
    iter_o[0..8].copy_from_slice(&(container.as_ptr() as usize).to_le_bytes());
    iter_o[8..10].copy_from_slice(&3u16.to_le_bytes());
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "u16_subiter_advance_at_end");
    Ok(1)
}

/// Original: `0x1420a33d0` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_stride64_iter_at_end(iter: usize) -> u8 {
    let c = rd::<usize>(iter);
    let inner = rd::<usize>(c);
    let count = rd::<u32>(inner + 0x8) as usize;
    let end = rd::<usize>(inner).wrapping_add(count << 6);
    (rd::<usize>(iter + 0x8) == end) as u8
}

pub unsafe fn diff_d3d_mesh_stride64_iter_at_end(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(new);
    let inner = [0x1000usize, 2usize];
    let c = [inner.as_ptr() as usize];
    for cur in [0x1000usize, 0x1080usize] {
        let iter = [c.as_ptr() as usize, cur];
        let r_o = f_orig(iter.as_ptr() as usize);
        let r_n = f_new(iter.as_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "stride64_iter_at_end");
    }
    Ok(2)
}

/// Original: `0x1420a33f0` (108 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_stride64_iter_advance(iter: usize) {
    let c0 = rd::<usize>(iter);
    let inner0 = rd::<usize>(c0);
    let count0 = rd::<u32>(inner0 + 0x8) as usize;
    let end0 = rd::<usize>(inner0).wrapping_add(count0 << 6);
    let mut cur = rd::<usize>(iter + 0x8);
    if cur == end0 {
        return;
    }
    loop {
        wr::<usize>(iter + 0x10, cur);
        wr::<u8>(iter + 0x18, 1);
        let sub_idx = rd::<u16>(cur + 0x22) as usize;
        let r9 = rd::<usize>(iter);
        let rax = rd::<usize>(r9 + 0x8);
        let rcx = rd::<usize>(rax + 0x10);
        let item = rcx.wrapping_add(sub_idx.wrapping_mul(0x30));
        if rd::<u8>(item + 0x20) != 0 && rd::<u8>(item + 0x21) == 0 {
            break;
        }
        cur = rd::<usize>(iter + 0x8).wrapping_add(0x40);
        wr::<usize>(iter + 0x8, cur);
        let inner = rd::<usize>(r9);
        let count = rd::<u32>(inner + 0x8) as usize;
        let end = rd::<usize>(inner).wrapping_add(count << 6);
        if cur == end {
            break;
        }
    }
}

pub unsafe fn diff_d3d_mesh_stride64_iter_advance(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let inner = [0x1000usize, 0usize];
    let c = [inner.as_ptr() as usize, 0usize];
    let mut iter_o = [c.as_ptr() as usize, 0x1000usize, 0usize, 0usize];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "stride64_iter_advance_empty");
    Ok(1)
}

/// Original: `0x1420a34a0` (93 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_stride8_iter_advance(iter: usize) {
    let c0 = rd::<usize>(iter);
    let inner0 = rd::<usize>(c0);
    let count0 = rd::<u32>(inner0 + 0x8) as usize;
    let end0 = rd::<usize>(inner0).wrapping_add(count0.wrapping_mul(8));
    let mut cur = rd::<usize>(iter + 0x8);
    if cur == end0 {
        return;
    }
    loop {
        let idx = rd::<u64>(iter + 0x10);
        wr::<u64>(iter + 0x18, idx);
        wr::<usize>(iter + 0x20, cur);
        wr::<u8>(iter + 0x28, 1);
        if rd::<u16>(cur + 0x2) == 0xffff {
            break;
        }
        wr::<u64>(iter + 0x10, rd::<u64>(iter + 0x10).wrapping_add(1));
        cur = rd::<usize>(iter + 0x8).wrapping_add(8);
        wr::<usize>(iter + 0x8, cur);
        let c = rd::<usize>(iter);
        let inner = rd::<usize>(c);
        let count = rd::<u32>(inner + 0x8) as usize;
        let end = rd::<usize>(inner).wrapping_add(count.wrapping_mul(8));
        if cur == end {
            break;
        }
    }
}

pub unsafe fn diff_d3d_mesh_stride8_iter_advance(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let entries: [[u16; 4]; 2] = [[1, 2, 0, 0], [3, 0xffff, 0, 0]];
    let inner = [entries.as_ptr() as usize, 2usize];
    let c = [inner.as_ptr() as usize];
    let mut iter_o = [c.as_ptr() as usize, entries.as_ptr() as usize, 0usize, 0usize, 0usize, 0usize];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "stride8_iter_advance");
    Ok(1)
}

/// Original: `0x1420a3560` (85 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_slot_iter_init(
    iter: usize,
    container: usize,
    state_ptr: usize,
) -> usize {
    wr::<[u8; 16]>(iter + 0x8, rd::<[u8; 16]>(state_ptr));
    wr::<usize>(iter, container);
    wr::<u8>(iter + 0x1a, 0);
    let mut idx = rd::<usize>(iter + 0x10);
    if idx != rd::<usize>(container + 0x8) {
        loop {
            wr::<u8>(iter + 0x1a, 1);
            let idx16 = idx as u16;
            wr::<u16>(iter + 0x18, idx16);
            let r9 = rd::<usize>(iter);
            let rax = rd::<usize>(r9 + 0x18);
            let rdx = rd::<usize>(rax + 0x10);
            if rd::<u8>(rdx.wrapping_add((idx16 as usize).wrapping_mul(4).wrapping_add(2))) == 0 {
                break;
            }
            idx = rd::<usize>(iter + 0x10).wrapping_add(1);
            wr::<usize>(iter + 0x10, idx);
            if idx == rd::<usize>(r9 + 0x8) {
                break;
            }
        }
    }
    iter
}

pub unsafe fn diff_d3d_mesh_slot_iter_init(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(new);
    let flags = [[0u8, 0, 1, 0], [0u8, 0, 0, 0]];
    let sub = [0usize, 0usize, flags.as_ptr() as usize];
    let container = [0usize, 2usize, 0usize, sub.as_ptr() as usize];
    let state = [0x99usize, 0usize];
    let mut iter_o = [0u8; 32];
    let mut iter_n = [0u8; 32];
    f_orig(
        iter_o.as_mut_ptr() as usize,
        container.as_ptr() as usize,
        state.as_ptr() as usize,
    );
    f_new(
        iter_n.as_mut_ptr() as usize,
        container.as_ptr() as usize,
        state.as_ptr() as usize,
    );
    crate::diff_eq!(iter_o, iter_n, "slot_iter_init");
    Ok(1)
}

/// Original: `0x1420a35e0` (79 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_slot_iter_next(iter: usize) {
    let mut idx = rd::<usize>(iter + 0x10).wrapping_add(1);
    wr::<usize>(iter + 0x10, idx);
    let c0 = rd::<usize>(iter);
    if idx == rd::<usize>(c0 + 0x8) {
        return;
    }
    loop {
        wr::<u8>(iter + 0x1a, 1);
        let idx16 = idx as u16;
        wr::<u16>(iter + 0x18, idx16);
        let r9 = rd::<usize>(iter);
        let rax = rd::<usize>(r9 + 0x18);
        let rdx = rd::<usize>(rax + 0x10);
        if rd::<u8>(rdx.wrapping_add((idx16 as usize).wrapping_mul(4).wrapping_add(2))) == 0 {
            break;
        }
        idx = rd::<usize>(iter + 0x10).wrapping_add(1);
        wr::<usize>(iter + 0x10, idx);
        if idx == rd::<usize>(r9 + 0x8) {
            break;
        }
    }
}

pub unsafe fn diff_d3d_mesh_slot_iter_next(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let flags = [[0u8, 0, 0, 0], [0u8, 0, 1, 0], [0u8, 0, 0, 0]];
    let sub = [0usize, 0usize, flags.as_ptr() as usize];
    let container = [0usize, 3usize, 0usize, sub.as_ptr() as usize];
    let mut iter_o = [0u8; 32];
    iter_o[0..8].copy_from_slice(&(container.as_ptr() as usize).to_le_bytes());
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "slot_iter_next");
    Ok(1)
}

/// Original: `0x1420a36d0` (31 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_u16_iter_init(
    iter: usize,
    container: usize,
    ptr: usize,
) -> usize {
    wr::<usize>(iter, container);
    wr::<usize>(iter + 0x8, ptr);
    wr::<u8>(iter + 0x18, 0);
    exe::call_va!(0x1_420a_3250, (iter) -> ());
    iter
}

pub unsafe fn diff_d3d_mesh_u16_iter_init(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(new);

    let indices: [u16; 2] = [0, 1];
    let mut entries = [0u8; 128];
    let mut items = [0u8; 96];
    wr::<u16>(entries.as_mut_ptr() as usize + 0x22, 0);
    wr::<u16>(entries.as_mut_ptr() as usize + 0x40 + 0x22, 1);
    wr::<u8>(items.as_mut_ptr() as usize + 0x20, 0);
    wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x20, 1);

    let rcx0 = [entries.as_ptr() as usize];
    let rcx1 = [0usize, 0usize, items.as_ptr() as usize];
    let rdx = [rcx0.as_ptr() as usize, 0usize, rcx1.as_ptr() as usize];
    let container = [2usize, indices.as_ptr() as usize, rdx.as_ptr() as usize];

    let mut iter_o = [0u8; 32];
    let mut iter_n = [0u8; 32];
    let r_o = f_orig(
        iter_o.as_mut_ptr() as usize,
        container.as_ptr() as usize,
        indices.as_ptr() as usize,
    );
    let r_n = f_new(
        iter_n.as_mut_ptr() as usize,
        container.as_ptr() as usize,
        indices.as_ptr() as usize,
    );
    crate::diff_eq!(r_o == iter_o.as_ptr() as usize, r_n == iter_n.as_ptr() as usize, "u16_iter_init ret");
    crate::diff_eq!(iter_o, iter_n, "u16_iter_init");
    Ok(1)
}

/// Original: `0x1420a3720` (22 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mesh_tagged_pair_init(
    dst: usize,
    tag_ptr: usize,
    pair_ptr: usize,
) -> usize {
    wr::<u32>(dst, rd::<u32>(tag_ptr));
    wr::<[u8; 16]>(dst + 0x8, rd::<[u8; 16]>(pair_ptr));
    wr::<u8>(dst + 0x18, 1);
    dst
}

pub unsafe fn diff_d3d_mesh_tagged_pair_init(orig: usize, new: usize) -> Result<u32, String> {
    let f_orig: unsafe extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(orig);
    let f_new: unsafe extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(new);
    let tag = 0x1357_9bdfu32;
    let pair = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut dst_o = [0u8; 32];
    let mut dst_n = [0u8; 32];
    f_orig(
        dst_o.as_mut_ptr() as usize,
        (&tag as *const u32) as usize,
        pair.as_ptr() as usize,
    );
    f_new(
        dst_n.as_mut_ptr() as usize,
        (&tag as *const u32) as usize,
        pair.as_ptr() as usize,
    );
    crate::diff_eq!(dst_o, dst_n, "tagged_pair_init");
    Ok(1)
}

// REGISTRY: Replacement::verified_throws(0x1_420a_05d0, "d3d::d3d_fx_init_resource_entry_1420a05d0", rend::d3d_fx_mesh_a::d3d_fx_init_resource_entry as *const (), Throws::Discard, rend::d3d_fx_mesh_a::diff_d3d_fx_init_resource_entry),
// REGISTRY: Replacement::verified(0x1_420a_0a40, "d3d::d3d_fx_clear_dual_resource_arrays_1420a0a40", rend::d3d_fx_mesh_a::d3d_fx_clear_dual_resource_arrays as *const (), rend::d3d_fx_mesh_a::diff_d3d_fx_clear_dual_resource_arrays),
// REGISTRY: Replacement::verified_throws(0x1_420a_1150, "d3d::d3d_mesh_apply_tag_to_u16_span_a_1420a1150", rend::d3d_fx_mesh_a::d3d_mesh_apply_tag_to_u16_span_a as *const (), Throws::Discard, rend::d3d_fx_mesh_a::diff_d3d_mesh_apply_tag_to_u16_span_a),
// REGISTRY: Replacement::verified_throws(0x1_420a_11d0, "d3d::d3d_mesh_apply_tag_to_u16_span_b_1420a11d0", rend::d3d_fx_mesh_a::d3d_mesh_apply_tag_to_u16_span_b as *const (), Throws::Discard, rend::d3d_fx_mesh_a::diff_d3d_mesh_apply_tag_to_u16_span_b),
// REGISTRY: Replacement::verified(0x1_420a_2280, "d3d::d3d_mesh_insert_named_binding_1420a2280", rend::d3d_fx_mesh_a::d3d_mesh_insert_named_binding as *const (), rend::d3d_fx_mesh_a::diff_d3d_mesh_insert_named_binding),
// REGISTRY: Replacement::verified_throws(0x1_420a_2ba0, "d3d::d3d_mesh_u16_iter_next_1420a2ba0", rend::d3d_fx_mesh_a::d3d_mesh_u16_iter_next as *const (), Throws::Discard, rend::d3d_fx_mesh_a::diff_d3d_mesh_u16_iter_next),
// REGISTRY: Replacement::verified(0x1_420a_36d0, "d3d::d3d_mesh_u16_iter_init_1420a36d0", rend::d3d_fx_mesh_a::d3d_mesh_u16_iter_init as *const (), rend::d3d_fx_mesh_a::diff_d3d_mesh_u16_iter_init),
