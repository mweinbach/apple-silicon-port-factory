use crate::exe::{self, rd, wr};
use std::sync::atomic::{AtomicU64, Ordering};

/// Original: `0x14208ac60` (29 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_cmp_slot_24_b(
    ctx_ptr: usize,
    idx_a: usize,
    idx_b: usize,
) -> u8 {
    let ctx = rd::<usize>(ctx_ptr);
    let table = rd::<usize>(ctx.wrapping_add(0x8));
    let val_a = rd::<u64>(table.wrapping_add(idx_a.wrapping_mul(24)).wrapping_add(0x8));
    let val_b = rd::<u64>(table.wrapping_add(idx_b.wrapping_mul(24)).wrapping_add(0x8));
    u8::from(val_b < val_a)
}

pub unsafe fn diff_d3d_cmp_slot_24_b(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut table = [0u64; 12];
    table[1] = 100;
    table[4] = 50;
    table[7] = 200;

    let mut ctx = [0usize; 2];
    ctx[1] = table.as_ptr() as usize;
    let inner_ptr: usize = ctx.as_ptr() as usize;
    let ctx_ptr: usize = &inner_ptr as *const usize as usize;

    for (a, b) in [(0usize, 1usize), (1, 0), (0, 2), (2, 2)] {
        let r_orig = orig_fn(ctx_ptr, a, b);
        let r_new = new_fn(ctx_ptr, a, b);
        crate::diff_eq!(r_orig, r_new, "d3d_cmp_slot_24_b result");
    }
    Ok(4)
}

/// Original: `0x14208acb0` (21 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_iter_pair(src_ptr: usize, out_pair: usize) -> usize {
    let inner = rd::<usize>(src_ptr);
    let first = rd::<usize>(inner);
    wr::<usize>(out_pair, first);
    wr::<usize>(out_pair.wrapping_add(0x8), 0);
    out_pair
}

pub unsafe fn diff_d3d_init_iter_pair(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let inner: usize = 0xdead_beef_cafe_1234;
    let inner_ptr: usize = &inner as *const usize as usize;
    let src_ptr: usize = &inner_ptr as *const usize as usize;

    let mut out_o = [0x1111_2222_3333_4444usize; 2];
    let mut out_n = [0x1111_2222_3333_4444usize; 2];

    let r_o = orig_fn(src_ptr, out_o.as_mut_ptr() as usize);
    let r_n = new_fn(src_ptr, out_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, out_o.as_mut_ptr() as usize, "d3d_init_iter_pair ret orig");
    crate::diff_eq!(r_n, out_n.as_mut_ptr() as usize, "d3d_init_iter_pair ret new");
    crate::diff_eq!(out_o, out_n, "d3d_init_iter_pair out");
    Ok(1)
}

/// Original: `0x14208b0f0` (58 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_locked_get_slot_10(this: usize, idx: u16) -> usize {
    exe::call_iat!(0x1_4264_55a8, (this) -> ());
    let table = rd::<usize>(this.wrapping_add(0x8));
    let val = rd::<usize>(
        table
            .wrapping_add((idx as usize).wrapping_mul(24))
            .wrapping_add(0x10),
    );
    exe::call_iat!(0x1_4264_5598, (this) -> ());
    val
}

pub unsafe fn diff_d3d_locked_get_slot_10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> usize;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut table = [0usize; 9];
    table[2] = 0x1111_2222_3333_4444;
    table[5] = 0x5555_6666_7777_8888;
    let mut obj = [0usize; 2];
    obj[1] = table.as_ptr() as usize;

    let r_o = orig_fn(obj.as_mut_ptr() as usize, 1);
    let r_n = new_fn(obj.as_mut_ptr() as usize, 1);
    crate::diff_eq!(r_o, r_n, "d3d_locked_get_slot_10 idx 1");
    Ok(1)
}

/// Original: `0x14208b130` (76 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_locked_set_slot_state3(this: usize, idx: u16, val: usize) {
    exe::call_iat!(0x1_4264_55a8, (this) -> ());
    let table = rd::<usize>(this.wrapping_add(0x8));
    let entry = table.wrapping_add((idx as usize).wrapping_mul(24));
    wr::<u32>(entry, 3);
    wr::<usize>(entry.wrapping_add(0x8), val);
    exe::call_iat!(0x1_4264_5598, (this) -> ());
}

pub unsafe fn diff_d3d_locked_set_slot_state3(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, usize);
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut table_o = [0u64; 9];
    let mut table_n = [0u64; 9];
    let mut obj_o = [0usize; 2];
    let mut obj_n = [0usize; 2];
    obj_o[1] = table_o.as_mut_ptr() as usize;
    obj_n[1] = table_n.as_mut_ptr() as usize;

    orig_fn(obj_o.as_mut_ptr() as usize, 2, 0xabcd_ef01_2345_6789);
    new_fn(obj_n.as_mut_ptr() as usize, 2, 0xabcd_ef01_2345_6789);
    crate::diff_eq!(table_o, table_n, "d3d_locked_set_slot_state3 table");
    Ok(1)
}

/// Original: `0x14208b3d0` (138 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_release_slot_table(this: usize) {
    let mut cur = rd::<usize>(this.wrapping_add(0x8));
    let count = rd::<u32>(this.wrapping_add(0x10)) as usize;
    let end = cur.wrapping_add(count.wrapping_mul(24));
    while cur != end {
        let state = rd::<u32>(cur);
        if state == 1 || state == 2 || state == 3 {
            let com_ptr = rd::<usize>(cur.wrapping_add(0x10));
            if com_ptr != 0 {
                let vtable = rd::<usize>(com_ptr);
                let release_fn = rd::<usize>(vtable.wrapping_add(0x10));
                exe::call_va!(release_fn, (com_ptr) -> u32);
            }
            wr::<usize>(cur.wrapping_add(0x10), 0);
            let atomic_cnt = &*(this.wrapping_add(0x18) as *const AtomicU64);
            atomic_cnt.fetch_sub(1, Ordering::SeqCst);
        }
        cur = cur.wrapping_add(0x18);
    }
    exe::call_va!(0x1_411e_0270, (this.wrapping_add(0x8)) -> ());
}

/// Original: `0x14208b460` (17 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_queue_wait_fence(this: usize, queue: usize, val: u64) -> u32 {
    let vtable = rd::<usize>(queue);
    let fence = rd::<usize>(this.wrapping_add(0x8));
    let wait_fn = rd::<usize>(vtable.wrapping_add(0x78));
    exe::call_va!(wait_fn, (queue, fence, val) -> u32)
}

/// Original: `0x14208b480` (21 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_queue_signal_fence(this: usize, queue: usize, val: u64) -> u32 {
    wr::<u64>(this.wrapping_add(0x10), val);
    let vtable = rd::<usize>(queue);
    let fence = rd::<usize>(this.wrapping_add(0x8));
    let signal_fn = rd::<usize>(vtable.wrapping_add(0x70));
    exe::call_va!(signal_fn, (queue, fence, val) -> u32)
}

/// Original: `0x14208b4b0` (44 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fence_get_completed_value(this: usize) -> u64 {
    let fence = rd::<usize>(this.wrapping_add(0x8));
    let vtable = rd::<usize>(fence);
    let get_completed_fn = rd::<usize>(vtable.wrapping_add(0x40));
    let val = exe::call_va!(get_completed_fn, (fence) -> u64);
    if val == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
    }
    val
}

/// Original: `0x14208b500` (11 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_post_inc_u64(this: usize) -> u64 {
    let cur = rd::<u64>(this);
    wr::<u64>(this, cur.wrapping_add(1));
    cur
}

pub unsafe fn diff_d3d_post_inc_u64(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut v_o: u64 = 41;
    let mut v_n: u64 = 41;
    let r_o = orig_fn(&mut v_o as *mut u64 as usize);
    let r_n = new_fn(&mut v_n as *mut u64 as usize);
    crate::diff_eq!(r_o, r_n, "d3d_post_inc_u64 ret");
    crate::diff_eq!(v_o, v_n, "d3d_post_inc_u64 val");
    Ok(1)
}

/// Original: `0x14208b520` (102 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_create_fence_wrapper(this: usize) -> usize {
    wr::<u64>(this, 1);
    let fence_out = this.wrapping_add(0x8);
    wr::<usize>(fence_out, 0);
    wr::<u64>(this.wrapping_add(0x10), 0);
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let vtable = rd::<usize>(dev);
    wr::<usize>(fence_out, 0);
    let riid = exe::ptr::<u8>(0x1_433d_cab8) as usize;
    let create_fence_fn = rd::<usize>(vtable.wrapping_add(0x120));
    let hr = exe::call_va!(create_fence_fn, (dev, 0u64, 0u32, riid, fence_out) -> u32);
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
    }
    this
}

/// Original: `0x14208b800` (38 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_eq_desc_20(a: usize, b: usize) -> u8 {
    if rd::<u64>(a) != rd::<u64>(b) {
        return 0;
    }
    if rd::<u32>(a.wrapping_add(0x8)) != rd::<u32>(b.wrapping_add(0x8)) {
        return 0;
    }
    if rd::<u32>(a.wrapping_add(0xc)) != rd::<u32>(b.wrapping_add(0xc)) {
        return 0;
    }
    if rd::<u32>(a.wrapping_add(0x10)) != rd::<u32>(b.wrapping_add(0x10)) {
        return 0;
    }
    1
}

pub unsafe fn diff_d3d_eq_desc_20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let a = [1u32, 2, 3, 4, 5];
    let mut b = [1u32, 2, 3, 4, 5];
    crate::diff_eq!(
        orig_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        new_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        "d3d_eq_desc_20 equal"
    );
    b[4] = 9;
    crate::diff_eq!(
        orig_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        new_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        "d3d_eq_desc_20 diff tail"
    );
    Ok(2)
}

/// Original: `0x14208b850` (82 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_eq_desc_14(a: usize, b: usize) -> u8 {
    if rd::<u8>(a.wrapping_add(0xc)) != rd::<u8>(b.wrapping_add(0xc)) {
        return 0;
    }
    if rd::<u8>(a.wrapping_add(0xd)) != rd::<u8>(b.wrapping_add(0xd)) {
        return 0;
    }
    if rd::<u16>(a) != rd::<u16>(b) {
        return 0;
    }
    if rd::<u16>(a.wrapping_add(0x2)) != rd::<u16>(b.wrapping_add(0x2)) {
        return 0;
    }
    if rd::<u16>(a.wrapping_add(0x4)) != rd::<u16>(b.wrapping_add(0x4)) {
        return 0;
    }
    if rd::<u16>(a.wrapping_add(0x6)) != rd::<u16>(b.wrapping_add(0x6)) {
        return 0;
    }
    if rd::<u16>(a.wrapping_add(0x8)) != rd::<u16>(b.wrapping_add(0x8)) {
        return 0;
    }
    if rd::<u16>(a.wrapping_add(0xa)) != rd::<u16>(b.wrapping_add(0xa)) {
        return 0;
    }
    1
}

pub unsafe fn diff_d3d_eq_desc_14(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let a = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let mut b = a;
    crate::diff_eq!(
        orig_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        new_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        "d3d_eq_desc_14 equal"
    );
    b[13] = 99;
    crate::diff_eq!(
        orig_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        new_fn(a.as_ptr() as usize, b.as_ptr() as usize),
        "d3d_eq_desc_14 diff"
    );
    Ok(2)
}

/// Original: `0x14208b8b0` (110 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_pack_grid_desc(out: usize, src: usize) -> usize {
    wr::<u64>(out.wrapping_add(0x8), 0);
    let flag = rd::<u8>(src.wrapping_add(0xd));
    let w_a = rd::<u16>(src.wrapping_add(0xa)) as u32;
    let w_8 = rd::<u16>(src.wrapping_add(0x8)) as u32;
    let w_6 = rd::<u16>(src.wrapping_add(0x6)) as u32;
    if flag != 0 {
        wr::<u32>(out.wrapping_add(0x4), 0);
        let prod1 = (w_a.wrapping_mul(w_6)) as u16 as u32;
        let prod2 = (prod1.wrapping_mul(w_8)) as u16 as u32;
        wr::<u32>(out, prod2);
    } else {
        wr::<u32>(out.wrapping_add(0x4), 1);
        let prod1 = w_a.wrapping_mul(w_6);
        wr::<u32>(out.wrapping_add(0x8), w_6);
        wr::<u16>(out.wrapping_add(0xc), w_8 as u16);
        wr::<u16>(out.wrapping_add(0xe), w_a as u16);
        let prod2 = ((prod1 as u16 as u32).wrapping_mul(w_8)) as u16 as u32;
        wr::<u32>(out, prod2);
    }
    out
}

pub unsafe fn diff_d3d_pack_grid_desc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    for flag in [0u8, 1u8] {
        let mut src = [0u8; 16];
        src[6] = 4;
        src[8] = 8;
        src[10] = 3;
        src[13] = flag;
        let mut out_o = [0u64; 2];
        let mut out_n = [0u64; 2];
        orig_fn(out_o.as_mut_ptr() as usize, src.as_ptr() as usize);
        new_fn(out_n.as_mut_ptr() as usize, src.as_ptr() as usize);
        crate::diff_eq!(out_o, out_n, "d3d_pack_grid_desc out");
    }
    Ok(2)
}

/// Original: `0x14208b930` (30 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unpack_dims_u16_to_u32(out: usize, src: usize) -> usize {
    wr::<u32>(out.wrapping_add(0xc), rd::<u8>(src.wrapping_add(0xc)) as u32);
    wr::<u32>(out, rd::<u16>(src) as u32);
    wr::<u32>(out.wrapping_add(0x4), rd::<u16>(src.wrapping_add(0x2)) as u32);
    wr::<u32>(out.wrapping_add(0x8), rd::<u16>(src.wrapping_add(0x4)) as u32);
    out
}

pub unsafe fn diff_d3d_unpack_dims_u16_to_u32(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let src: [u8; 14] = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0, 0, 0, 0, 0, 0, 0xef, 0];
    let mut out_o = [0u32; 4];
    let mut out_n = [0u32; 4];
    orig_fn(out_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    new_fn(out_n.as_mut_ptr() as usize, src.as_ptr() as usize);
    crate::diff_eq!(out_o, out_n, "d3d_unpack_dims_u16_to_u32 out");
    Ok(1)
}

/// Original: `0x14208b960` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_signal_latency_fence(_this: usize, queue: usize) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_8078) as usize) != 0 {
        let state = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6228) as usize);
        let next_val = rd::<u64>(state.wrapping_add(0x8)).wrapping_add(1);
        wr::<u64>(state.wrapping_add(0x8), next_val);
        let vtable = rd::<usize>(queue);
        let fence = rd::<usize>(state);
        let signal_fn = rd::<usize>(vtable.wrapping_add(0x70));
        exe::call_va!(signal_fn, (queue, fence, next_val) -> u32);
    }
}

/// Original: `0x14208b990` (124 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_wait_latency_fence(_this: usize) {
    let enabled = rd::<u8>(exe::ptr::<u8>(0x1_4385_8078) as usize);
    let state = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6228) as usize);
    if enabled == 0 || rd::<u64>(state.wrapping_add(0x8)) < 0x64 {
        return;
    }
    let fence = rd::<usize>(state);
    let vtable = rd::<usize>(fence);
    let get_completed_fn = rd::<usize>(vtable.wrapping_add(0x40));
    let completed = exe::call_va!(get_completed_fn, (fence) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
    }
    let cur = rd::<u64>(state.wrapping_add(0x8));
    if completed > cur.wrapping_sub(0x64) {
        return;
    }
    let fence2 = rd::<usize>(state);
    let event = rd::<usize>(state.wrapping_add(0x10));
    let vtable2 = rd::<usize>(fence2);
    let set_event_fn = rd::<usize>(vtable2.wrapping_add(0x48));
    exe::call_va!(set_event_fn, (fence2, cur.wrapping_sub(0x63), event) -> u32);
    let event2 = rd::<usize>(state.wrapping_add(0x10));
    exe::call_iat!(0x1_4264_5748, (event2, 0xffff_ffffu32) -> u32);
}

/// Original: `0x14208ba20` (59 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_destroy_latency_fence(this: usize) {
    let event = rd::<usize>(this.wrapping_add(0x10));
    if event != 0 {
        exe::call_iat!(0x1_4264_55b0, (event) -> i32);
        wr::<usize>(this.wrapping_add(0x10), 0);
    }
    let fence = rd::<usize>(this);
    if fence != 0 {
        let vtable = rd::<usize>(fence);
        let release_fn = rd::<usize>(vtable.wrapping_add(0x10));
        exe::call_va!(release_fn, (fence) -> u32);
    }
    wr::<usize>(this, 0);
}

/// Original: `0x14208ba70` (114 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_latency_fence(this: usize) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this.wrapping_add(0x8), 0);
    wr::<u64>(this.wrapping_add(0x10), 0);
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let vtable = rd::<usize>(dev);
    wr::<u64>(this, 0);
    let riid = exe::ptr::<u8>(0x1_433d_cab8) as usize;
    let create_fence_fn = rd::<usize>(vtable.wrapping_add(0x120));
    let hr = exe::call_va!(create_fence_fn, (dev, 0u64, 0u32, riid, this) -> u32);
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
    }
    let event = exe::call_iat!(0x1_4264_5740, (0usize, 0i32, 0i32, 0usize) -> usize);
    wr::<usize>(this.wrapping_add(0x10), event);
    this
}

/// Original: `0x14208bb00` (172 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_enqueue_task_simple(this: usize, task: usize) {
    let queue_base = rd::<usize>(this.wrapping_add(0x18)).wrapping_add(0x30);
    let mut queue_ptr: usize = queue_base;
    let lock = queue_base.wrapping_add(0x8);
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let queue_ptr_addr = &mut queue_ptr as *mut usize as usize;
    exe::call_va!(0x1_4208_86c0, (queue_ptr_addr, task) -> ());
    exe::call_iat!(0x1_4264_5598, (lock) -> ());

    let notify = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    let notify_lock = notify.wrapping_add(0x250);
    exe::call_iat!(0x1_4264_55a8, (notify_lock) -> ());
    let cnt_addr = notify.wrapping_add(0x258);
    wr::<u32>(cnt_addr, rd::<u32>(cnt_addr).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (notify_lock) -> ());
    exe::call_iat!(0x1_4264_5590, (notify.wrapping_add(0x248)) -> ());

    let callable = rd::<usize>(task.wrapping_add(0x38));
    if callable != 0 {
        let vtable = rd::<usize>(callable);
        let destroy_fn = rd::<usize>(vtable.wrapping_add(0x20));
        let not_inline = u8::from(callable != task);
        exe::call_va!(destroy_fn, (callable, not_inline) -> ());
        wr::<usize>(task.wrapping_add(0x38), 0);
    }
}

/// Original: `0x14208bbc0` (188 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_enqueue_task_with_dep(
    this: usize,
    dep_a: usize,
    dep_b: usize,
    task: usize,
) {
    let queue_base = rd::<usize>(this.wrapping_add(0x18)).wrapping_add(0x30);
    let mut queue_ptr: usize = queue_base;
    let lock = queue_base.wrapping_add(0x8);
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let queue_ptr_addr = &mut queue_ptr as *mut usize as usize;
    exe::call_va!(0x1_4208_7760, (lock, queue_ptr_addr, dep_a, dep_b) -> ());
    exe::call_va!(0x1_4208_86c0, (queue_ptr_addr, task) -> ());
    exe::call_iat!(0x1_4264_5598, (lock) -> ());

    let notify = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    let notify_lock = notify.wrapping_add(0x250);
    exe::call_iat!(0x1_4264_55a8, (notify_lock) -> ());
    let cnt_addr = notify.wrapping_add(0x258);
    wr::<u32>(cnt_addr, rd::<u32>(cnt_addr).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (notify_lock) -> ());
    exe::call_iat!(0x1_4264_5590, (notify.wrapping_add(0x248)) -> ());

    let callable = rd::<usize>(task.wrapping_add(0x38));
    if callable != 0 {
        let vtable = rd::<usize>(callable);
        let destroy_fn = rd::<usize>(vtable.wrapping_add(0x20));
        let not_inline = u8::from(callable != task);
        exe::call_va!(destroy_fn, (callable, not_inline) -> ());
        wr::<usize>(task.wrapping_add(0x38), 0);
    }
}

/// Original: `0x14208bdd0` (200 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_enqueue_task_with_xmm_dep(
    this: usize,
    dep_xmm_ptr: usize,
    dep_b: usize,
    task: usize,
) {
    let queue_base = rd::<usize>(this.wrapping_add(0x18)).wrapping_add(0x30);
    let mut queue_ptr: usize = queue_base;
    let lock = queue_base.wrapping_add(0x8);
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let mut dep_copy = [0u64; 2];
    dep_copy[0] = rd::<u64>(dep_xmm_ptr);
    dep_copy[1] = rd::<u64>(dep_xmm_ptr.wrapping_add(0x8));
    let queue_ptr_addr = &mut queue_ptr as *mut usize as usize;
    let dep_copy_addr = dep_copy.as_mut_ptr() as usize;
    exe::call_va!(0x1_4208_7920, (lock, queue_ptr_addr, dep_copy_addr, dep_b) -> ());
    exe::call_va!(0x1_4208_86c0, (queue_ptr_addr, task) -> ());
    exe::call_iat!(0x1_4264_5598, (lock) -> ());

    let notify = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    let notify_lock = notify.wrapping_add(0x250);
    exe::call_iat!(0x1_4264_55a8, (notify_lock) -> ());
    let cnt_addr = notify.wrapping_add(0x258);
    wr::<u32>(cnt_addr, rd::<u32>(cnt_addr).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (notify_lock) -> ());
    exe::call_iat!(0x1_4264_5590, (notify.wrapping_add(0x248)) -> ());

    let callable = rd::<usize>(task.wrapping_add(0x38));
    if callable != 0 {
        let vtable = rd::<usize>(callable);
        let destroy_fn = rd::<usize>(vtable.wrapping_add(0x20));
        let not_inline = u8::from(callable != task);
        exe::call_va!(destroy_fn, (callable, not_inline) -> ());
        wr::<usize>(task.wrapping_add(0x38), 0);
    }
}

/// Original: `0x14208c670` (16 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_atomic_sub_210(this: usize, delta: u64) {
    let inner = rd::<usize>(this.wrapping_add(0x18));
    let atomic_val = &*(inner.wrapping_add(0x210) as *const AtomicU64);
    atomic_val.fetch_add(delta.wrapping_neg(), Ordering::SeqCst);
}

pub unsafe fn diff_d3d_atomic_sub_210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut inner_o = [0u64; 70];
    let mut inner_n = [0u64; 70];
    inner_o[0x210 / 8] = 1000;
    inner_n[0x210 / 8] = 1000;
    let mut outer_o = [0usize; 4];
    let mut outer_n = [0usize; 4];
    outer_o[3] = inner_o.as_mut_ptr() as usize;
    outer_n[3] = inner_n.as_mut_ptr() as usize;

    orig_fn(outer_o.as_mut_ptr() as usize, 250);
    new_fn(outer_n.as_mut_ptr() as usize, 250);
    crate::diff_eq!(inner_o[0x210 / 8], inner_n[0x210 / 8], "d3d_atomic_sub_210");
    Ok(1)
}

/// Original: `0x14208c690` (13 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_atomic_add_210(this: usize, delta: u64) {
    let inner = rd::<usize>(this.wrapping_add(0x18));
    let atomic_val = &*(inner.wrapping_add(0x210) as *const AtomicU64);
    atomic_val.fetch_add(delta, Ordering::SeqCst);
}

pub unsafe fn diff_d3d_atomic_add_210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut inner_o = [0u64; 70];
    let mut inner_n = [0u64; 70];
    inner_o[0x210 / 8] = 1000;
    inner_n[0x210 / 8] = 1000;
    let mut outer_o = [0usize; 4];
    let mut outer_n = [0usize; 4];
    outer_o[3] = inner_o.as_mut_ptr() as usize;
    outer_n[3] = inner_n.as_mut_ptr() as usize;

    orig_fn(outer_o.as_mut_ptr() as usize, 250);
    new_fn(outer_n.as_mut_ptr() as usize, 250);
    crate::diff_eq!(inner_o[0x210 / 8], inner_n[0x210 / 8], "d3d_atomic_add_210");
    Ok(1)
}

/// Original: `0x14208d180` (104 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_acquire_shared_com_3bd8(_this: usize, out_ptr: usize) -> usize {
    let global_obj = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let lock = global_obj.wrapping_add(0x3bd0);
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let com_obj = rd::<usize>(global_obj.wrapping_add(0x3bd8));
    if com_obj != 0 {
        let vtable = rd::<usize>(com_obj);
        let addref_fn = rd::<usize>(vtable.wrapping_add(0x8));
        exe::call_va!(addref_fn, (com_obj) -> u32);
    }
    wr::<usize>(out_ptr, com_obj);
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
    out_ptr
}

/// Original: `0x14208dda0` (81 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_store_global_config_58(src: usize) {
    let slot = exe::ptr::<usize>(0x1_43a3_62b8) as usize;
    let mut dst = rd::<usize>(slot);
    if dst == 0 {
        dst = exe::call_va!(0x1_411e_1a10, () -> usize);
        wr::<usize>(slot, dst);
    }
    let mut offset = 0usize;
    while offset < 0x58 {
        let val = rd::<u64>(src.wrapping_add(offset));
        wr::<u64>(dst.wrapping_add(0x8).wrapping_add(offset), val);
        offset += 8;
    }
}

/// Original: `0x14208e150` (44 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fill_tag_ptrs_468(this: usize, out_ptrs: usize) {
    let mut i: u32 = 0;
    if rd::<u32>(this) == 0 {
        return;
    }
    loop {
        let ptr_val = this
            .wrapping_add(0x468)
            .wrapping_add((i as usize) << 6);
        wr::<usize>(out_ptrs.wrapping_add((i as usize).wrapping_mul(8)), ptr_val);
        i = i.wrapping_add(1);
        if i >= rd::<u32>(this) {
            break;
        }
    }
}

pub unsafe fn diff_d3d_fill_tag_ptrs_468(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut hdr = [0u32; 4];
    hdr[0] = 5;
    let base = hdr.as_mut_ptr() as usize;
    let mut out_o = [0usize; 6];
    let mut out_n = [0usize; 6];
    orig_fn(base, out_o.as_mut_ptr() as usize);
    new_fn(base, out_n.as_mut_ptr() as usize);
    crate::diff_eq!(out_o, out_n, "d3d_fill_tag_ptrs_468 out");
    Ok(1)
}

/// Original: `0x14208e2d0` (101 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sl_pcl_set_marker_present() {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) == 0 {
        return;
    }
    let fn_slot = exe::ptr::<usize>(0x1_43c8_c608) as usize;
    let mut fn_ptr = rd::<usize>(fn_slot);
    let frame_token = rd::<u64>(exe::ptr::<u64>(0x1_4385_8398) as usize);
    if fn_ptr == 0 {
        let name = exe::ptr::<u8>(0x1_433d_d168) as usize;
        let status = exe::call_iat!(0x1_4264_7308, (4u32, name, fn_slot) -> u32);
        if status != 0 {
            return;
        }
        fn_ptr = rd::<usize>(fn_slot);
    }
    exe::call_va!(fn_ptr, (3u32, frame_token) -> ());
}

/// Original: `0x14208e340` (236 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sl_dequeue_frame_and_mark_render_submit() {
    let lock = exe::ptr::<u8>(0x1_4385_83b8) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let count_addr = exe::ptr::<u32>(0x1_4385_83b0) as usize;
    let count = rd::<u32>(count_addr);
    if count != 0 {
        let head_addr = exe::ptr::<u32>(0x1_4385_83a8) as usize;
        let cap_addr = exe::ptr::<u32>(0x1_4385_83b4) as usize;
        let head = rd::<u32>(head_addr);
        let cap = rd::<u32>(cap_addr);
        let idx = ((head as u64) % (cap as u64)) as usize;
        let ring = rd::<usize>(exe::ptr::<usize>(0x1_4385_83a0) as usize);
        let next_head = head.wrapping_add(1);
        let token = rd::<u64>(ring.wrapping_add(idx.wrapping_mul(8)));
        wr::<u64>(exe::ptr::<u64>(0x1_4385_8398) as usize, token);
        wr::<u32>(head_addr, next_head);
        if next_head == cap {
            wr::<u32>(head_addr, 0);
        }
        wr::<u32>(count_addr, count.wrapping_sub(1));
    }
    exe::call_iat!(0x1_4264_5598, (lock) -> ());

    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) != 0 {
        let fn_slot = exe::ptr::<usize>(0x1_43c8_c608) as usize;
        let mut fn_ptr = rd::<usize>(fn_slot);
        let frame_token = rd::<u64>(exe::ptr::<u64>(0x1_4385_8398) as usize);
        let mut can_call = true;
        if fn_ptr == 0 {
            let name = exe::ptr::<u8>(0x1_433d_d168) as usize;
            let status = exe::call_iat!(0x1_4264_7308, (4u32, name, fn_slot) -> u32);
            if status != 0 {
                can_call = false;
            } else {
                fn_ptr = rd::<usize>(fn_slot);
            }
        }
        if can_call {
            exe::call_va!(fn_ptr, (2u32, frame_token) -> ());
        }
    }

    let flag_cf = exe::ptr::<u8>(0x1_4385_83cf) as usize;
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ce) as usize) != 0 {
        let cond = rd::<u8>(exe::ptr::<u8>(0x1_4385_82c0) as usize);
        wr::<u8>(flag_cf, 1);
        if cond == 0 {
            wr::<u8>(flag_cf, 0);
        }
    } else {
        wr::<u8>(flag_cf, 0);
    }
}

/// Original: `0x14208e440` (45 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sl_advance_frame_token() {
    let token_slot = exe::ptr::<usize>(0x1_4385_8388) as usize;
    let token_obj = rd::<usize>(token_slot);
    let vtable = rd::<usize>(token_obj);
    let get_idx_fn = rd::<usize>(vtable);
    let next_idx = exe::call_va!(get_idx_fn, (token_obj) -> u32).wrapping_add(1);
    let mut next_idx_stack: u32 = next_idx;
    let next_idx_ptr = &mut next_idx_stack as *mut u32 as usize;
    exe::call_iat!(0x1_4264_7310, (token_slot, next_idx_ptr) -> u32);
}

/// Original: `0x14208e480` (136 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sl_enqueue_frame_token() {
    let lock = exe::ptr::<u8>(0x1_4385_83b8) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let count_addr = exe::ptr::<u32>(0x1_4385_83b0) as usize;
    let cap_addr = exe::ptr::<u32>(0x1_4385_83b4) as usize;
    let ring_addr = exe::ptr::<usize>(0x1_4385_83a0) as usize;
    let tail_addr = exe::ptr::<u32>(0x1_4385_83ac) as usize;
    let count = rd::<u32>(count_addr);
    if count == rd::<u32>(cap_addr) {
        exe::call_va!(0x1_411d_aad0, (ring_addr, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(tail_addr);
    let ring = rd::<usize>(ring_addr);
    let cur_token = rd::<u64>(exe::ptr::<u64>(0x1_4385_8390) as usize);
    wr::<u64>(ring.wrapping_add((tail as usize).wrapping_mul(8)), cur_token);
    let next_tail = rd::<u32>(tail_addr).wrapping_add(1);
    wr::<u32>(tail_addr, next_tail);
    if next_tail == rd::<u32>(cap_addr) {
        wr::<u32>(tail_addr, 0);
    }
    wr::<u32>(count_addr, rd::<u32>(count_addr).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

/// Original: `0x14208e7b0` (231 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sl_tag_hudless_and_depth_mvec(res_ptr: usize, flag: u8) {
    if rd::<u32>(exe::ptr::<u32>(0x1_4385_83f8) as usize) == 0 {
        return;
    }
    let hudless_slot = exe::ptr::<u8>(0x1_4385_8408) as usize;
    let hudless_name = exe::ptr::<u8>(0x1_433d_d258) as usize;
    exe::call_va!(0x1_4208_f220, (hudless_slot, res_ptr, hudless_name) -> ());

    let h = rd::<u32>(res_ptr.wrapping_add(0x4));
    let w = rd::<u32>(res_ptr);
    let conv_slot = exe::ptr::<u8>(0x1_4385_8410) as usize;
    let conv_name = exe::ptr::<u8>(0x1_433d_d0d8) as usize;
    exe::call_va!(0x1_4208_f090, (conv_slot, conv_name, w, h, flag as usize) -> ());

    let mut batch = [0u64; 230];
    let batch_addr = batch.as_mut_ptr() as usize;
    exe::call_va!(0x1_4208_e540, (batch_addr) -> usize);

    let depth_res = rd::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize);
    exe::call_va!(0x1_4208_e190, (batch_addr, depth_res, 0u32) -> ());

    let mvec_res = rd::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize);
    exe::call_va!(0x1_4208_e190, (batch_addr, mvec_res, 1u32) -> ());

    let mut color_holder = 0usize;
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_8290) as usize) != 0 {
        color_holder = rd::<usize>(conv_slot);
    }
    if color_holder == 0 {
        color_holder = rd::<usize>(hudless_slot);
    }
    let color_res = rd::<usize>(color_holder);
    exe::call_va!(0x1_4208_e190, (batch_addr, color_res, 2u32) -> ());

    let count = rd::<u32>(batch_addr);
    let viewport = exe::ptr::<u8>(0x1_4385_8360) as usize;
    exe::call_iat!(
        0x1_4264_7300,
        (viewport, batch_addr.wrapping_add(0x158), count, 0usize) -> u32
    );
}

/// Original: `0x14208e8a0` (222 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_sl_resource_tag_batch(this: usize) -> usize {
    wr::<u32>(this, 0);
    let mut i = 0usize;
    while i < 10 {
        let base = this.wrapping_add(0x8).wrapping_add(i.wrapping_mul(0x70));
        wr::<u64>(base, 0);
        wr::<u32>(base.wrapping_add(0x8), 0x3a9d_70cf);
        wr::<u32>(base.wrapping_add(0xc), 0x4b72_2418);
        wr::<u32>(base.wrapping_add(0x10), 0xf813_9183);
        wr::<u32>(base.wrapping_add(0x14), 0x6172_1c72);
        wr::<u64>(base.wrapping_add(0x18), 1);
        wr::<u8>(base.wrapping_add(0x20), 0);
        wr::<u64>(base.wrapping_add(0x28), 0);
        wr::<u64>(base.wrapping_add(0x30), 0);
        wr::<u64>(base.wrapping_add(0x38), 0);
        wr::<u32>(base.wrapping_add(0x40), 0xffff_ffff);
        wr::<u64>(base.wrapping_add(0x44), 0);
        wr::<u64>(base.wrapping_add(0x4c), 0);
        wr::<u32>(base.wrapping_add(0x54), 0);
        wr::<u64>(base.wrapping_add(0x58), 0);
        wr::<u64>(base.wrapping_add(0x64), 0);
        i += 1;
    }
    let mut j = 0usize;
    while j < 10 {
        let base = this.wrapping_add(0x468).wrapping_add(j.wrapping_mul(0x40));
        wr::<u64>(base, 0);
        wr::<u32>(base.wrapping_add(0x8), 0x4c6a_5aad);
        wr::<u32>(base.wrapping_add(0xc), 0x496c_b445);
        wr::<u32>(base.wrapping_add(0x10), 0xf31a_ff87);
        wr::<u32>(base.wrapping_add(0x14), 0x53e6_5b84);
        wr::<u64>(base.wrapping_add(0x18), 1);
        wr::<u64>(base.wrapping_add(0x20), 0);
        wr::<u64>(base.wrapping_add(0x28), 0);
        wr::<u64>(base.wrapping_add(0x30), 0);
        wr::<u64>(base.wrapping_add(0x38), 0);
        j += 1;
    }
    this
}

pub unsafe fn diff_d3d_init_sl_resource_tag_batch(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut buf_o = [0x5au8; 0x6e8];
    let mut buf_n = [0x5au8; 0x6e8];
    let r_o = orig_fn(buf_o.as_mut_ptr() as usize);
    let r_n = new_fn(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(buf_o, buf_n, "d3d_init_sl_resource_tag_batch buf");
    Ok(1)
}

/// Original: `0x14208ec60` (71 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sl_record_camera_params(
    p0: u64,
    p1: u64,
    p2: u64,
    p3: u64,
    p4: u64,
    p5: u64,
) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83cd) as usize) == 0 {
        return;
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83cf) as usize) == 0 {
        return;
    }
    wr::<u64>(exe::ptr::<u64>(0x1_4385_8460) as usize, p4);
    wr::<u64>(exe::ptr::<u64>(0x1_4385_8468) as usize, p5);
    wr::<u64>(exe::ptr::<u64>(0x1_4385_8440) as usize, p0);
    wr::<u64>(exe::ptr::<u64>(0x1_4385_8448) as usize, p1);
    wr::<u64>(exe::ptr::<u64>(0x1_4385_8450) as usize, p2);
    wr::<u64>(exe::ptr::<u64>(0x1_4385_8458) as usize, p3);
}

/// Original: `0x14208ecb0` (128 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sl_update_depth_and_mvec_resources(mvec_res: usize, depth_res: usize) {
    let mode = rd::<u32>(exe::ptr::<u32>(0x1_4385_83f8) as usize);
    if mode == 0 {
        if rd::<u8>(exe::ptr::<u8>(0x1_4385_83cd) as usize) != 0 {
            wr::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize, depth_res);
            wr::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize, mvec_res);
        }
        return;
    }
    let depth_slot = exe::ptr::<usize>(0x1_4385_8418) as usize;
    let depth_name = exe::ptr::<u8>(0x1_433d_d220) as usize;
    exe::call_va!(0x1_4208_f220, (depth_slot, depth_res, depth_name) -> ());

    let mvec_slot = exe::ptr::<usize>(0x1_4385_8420) as usize;
    let mvec_name = exe::ptr::<u8>(0x1_433d_d238) as usize;
    exe::call_va!(0x1_4208_f220, (mvec_slot, mvec_res, mvec_name) -> ());

    let depth_holder = rd::<usize>(depth_slot);
    let depth_inner = rd::<usize>(depth_holder);
    let mvec_holder = rd::<usize>(mvec_slot);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize, depth_inner);
    let mvec_inner = rd::<usize>(mvec_holder);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize, mvec_inner);
}

/// Original: `0x14208ed40` (172 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_sl_single_resource_tag(this: usize) -> usize {
    wr::<u32>(this, 0);
    wr::<u64>(this.wrapping_add(0x8), 0);
    wr::<u32>(this.wrapping_add(0x10), 0x3a9d_70cf);
    wr::<u32>(this.wrapping_add(0x14), 0x4b72_2418);
    wr::<u32>(this.wrapping_add(0x18), 0xf813_9183);
    wr::<u32>(this.wrapping_add(0x1c), 0x6172_1c72);
    wr::<u64>(this.wrapping_add(0x20), 1);
    wr::<u8>(this.wrapping_add(0x28), 0);
    wr::<u64>(this.wrapping_add(0x30), 0);
    wr::<u64>(this.wrapping_add(0x38), 0);
    wr::<u64>(this.wrapping_add(0x40), 0);
    wr::<u32>(this.wrapping_add(0x48), 0xffff_ffff);
    wr::<u64>(this.wrapping_add(0x4c), 0);
    wr::<u64>(this.wrapping_add(0x54), 0);
    wr::<u32>(this.wrapping_add(0x5c), 0);
    wr::<u64>(this.wrapping_add(0x60), 0);
    wr::<u64>(this.wrapping_add(0x6c), 0);
    wr::<u64>(this.wrapping_add(0x78), 0);
    wr::<u32>(this.wrapping_add(0x80), 0x4c6a_5aad);
    wr::<u32>(this.wrapping_add(0x84), 0x496c_b445);
    wr::<u32>(this.wrapping_add(0x88), 0xf31a_ff87);
    wr::<u32>(this.wrapping_add(0x8c), 0x53e6_5b84);
    wr::<u64>(this.wrapping_add(0x90), 1);
    wr::<u64>(this.wrapping_add(0x98), 0);
    wr::<u64>(this.wrapping_add(0xa0), 0);
    wr::<u64>(this.wrapping_add(0xa8), 0);
    wr::<u64>(this.wrapping_add(0xb0), 0);
    this
}

pub unsafe fn diff_d3d_init_sl_single_resource_tag(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut buf_o = [0x5au8; 0xb8];
    let mut buf_n = [0x5au8; 0xb8];
    let r_o = orig_fn(buf_o.as_mut_ptr() as usize);
    let r_n = new_fn(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(buf_o, buf_n, "d3d_init_sl_single_resource_tag buf");
    Ok(1)
}

/// Original: `0x14208f3b0` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_accum_delta_stat(this: usize, start: u64, end: u64) {
    if end >= start {
        let delta = (end as u32).wrapping_sub(start as u32);
        wr::<u32>(this, rd::<u32>(this).wrapping_add(delta));
        wr::<u32>(
            this.wrapping_add(0x4),
            rd::<u32>(this.wrapping_add(0x4)).wrapping_add(1),
        );
    }
}

pub unsafe fn diff_d3d_accum_delta_stat(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64);
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut s_o = [10u32, 2];
    let mut s_n = [10u32, 2];
    orig_fn(s_o.as_mut_ptr() as usize, 100, 150);
    new_fn(s_n.as_mut_ptr() as usize, 100, 150);
    crate::diff_eq!(s_o, s_n, "d3d_accum_delta_stat >= ");

    orig_fn(s_o.as_mut_ptr() as usize, 200, 150);
    new_fn(s_n.as_mut_ptr() as usize, 200, 150);
    crate::diff_eq!(s_o, s_n, "d3d_accum_delta_stat < ");
    Ok(2)
}

/// Original: `0x142090840` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_sl_consts_struct(this: usize) -> usize {
    wr::<u32>(this, 0);
    wr::<u64>(this.wrapping_add(0x8), 0);
    wr::<u32>(this.wrapping_add(0x10), 0xef1d_0957);
    wr::<u32>(this.wrapping_add(0x14), 0x4df7_fd58);
    wr::<u32>(this.wrapping_add(0x18), 0x698b_04b5);
    wr::<u32>(this.wrapping_add(0x1c), 0x766b_aad8);
    wr::<u64>(this.wrapping_add(0x20), 1);
    wr::<u64>(this.wrapping_add(0x28), 0);
    wr::<u64>(this.wrapping_add(0x30), 0);
    wr::<u64>(this.wrapping_add(0x38), 0);
    wr::<u32>(this.wrapping_add(0x40), 0);
    wr::<u8>(this.wrapping_add(0x48), 0);
    this
}

pub unsafe fn diff_d3d_init_sl_consts_struct(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = std::mem::transmute(orig);
    let new_fn: F = std::mem::transmute(new);

    let mut buf_o = [0x5au8; 0x50];
    let mut buf_n = [0x5au8; 0x50];
    let r_o = orig_fn(buf_o.as_mut_ptr() as usize);
    let r_n = new_fn(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(buf_o, buf_n, "d3d_init_sl_consts_struct buf");
    Ok(1)
}

unsafe extern "win64" fn fake_com_addref_or_release(this: usize) -> u32 {
    let next = rd::<u32>(this + 0x08).wrapping_add(1);
    wr::<u32>(this + 0x08, next);
    next
}

unsafe extern "win64" fn fake_free_slot_vec_e0270(vec_ptr: usize) {
    wr::<usize>(vec_ptr, 0);
    wr::<u64>(vec_ptr + 0x08, 0);
}

pub unsafe fn diff_d3d_release_slot_table(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Mock;
    let _m = Mock::new(0x1_411e_0270, fake_free_slot_vec_e0270 as *const ())?;

    let mut vtbl = [0usize; 4];
    vtbl[2] = fake_com_addref_or_release as *const () as usize;

    let mut com0_o = [vtbl.as_ptr() as usize, 0usize];
    let mut com1_o = [vtbl.as_ptr() as usize, 0usize];
    let mut com0_n = com0_o;
    let mut com1_n = com1_o;

    let mut slots_o = [
        0u64,
        11,
        com0_o.as_mut_ptr() as usize as u64,
        2,
        22,
        com1_o.as_mut_ptr() as usize as u64,
        3,
        33,
        0,
    ];
    let mut slots_n = [
        0u64,
        11,
        com0_n.as_mut_ptr() as usize as u64,
        2,
        22,
        com1_n.as_mut_ptr() as usize as u64,
        3,
        33,
        0,
    ];

    let mut this_o = [0u64, slots_o.as_mut_ptr() as usize as u64, 3, 10];
    let mut this_n = [0u64, slots_n.as_mut_ptr() as usize as u64, 3, 10];

    exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> ());

    crate::diff_eq!(com0_o[1], com0_n[1], "release_slot_table com0");
    crate::diff_eq!(com1_o[1], com1_n[1], "release_slot_table com1");
    crate::diff_eq!(slots_o[5], slots_n[5], "release_slot_table slot1 cleared");
    crate::diff_eq!(this_o[1..], this_n[1..], "release_slot_table this");
    Ok(1)
}

unsafe extern "win64" fn fake_queue_wait_or_signal(
    queue: usize,
    fence: usize,
    val: u64,
) -> u32 {
    wr::<usize>(queue + 0x08, fence);
    wr::<u64>(queue + 0x10, val);
    0x1234_5678
}

pub unsafe fn diff_d3d_queue_wait_fence(orig: usize, new: usize) -> Result<u32, String> {
    let mut vtbl = [0usize; 16];
    vtbl[15] = fake_queue_wait_or_signal as *const () as usize;

    let this_s = [0u64, 0xfeee_1111u64, 0u64];
    let mut q_o = [vtbl.as_ptr() as usize as u64, 0, 0];
    let mut q_n = q_o;

    let ro = exe::call_va!(orig, (this_s.as_ptr() as usize, q_o.as_mut_ptr() as usize, 99u64) -> u32);
    let rn = exe::call_va!(new, (this_s.as_ptr() as usize, q_n.as_mut_ptr() as usize, 99u64) -> u32);

    crate::diff_eq!(ro, rn, "queue_wait_fence ret");
    crate::diff_eq!(&q_o[1..], &q_n[1..], "queue_wait_fence args");
    Ok(1)
}

pub unsafe fn diff_d3d_queue_signal_fence(orig: usize, new: usize) -> Result<u32, String> {
    let mut vtbl = [0usize; 16];
    vtbl[14] = fake_queue_wait_or_signal as *const () as usize;

    let mut this_o = [0u64, 0xfeee_2222u64, 0u64];
    let mut this_n = this_o;
    let mut q_o = [vtbl.as_ptr() as usize as u64, 0, 0];
    let mut q_n = q_o;

    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize, q_o.as_mut_ptr() as usize, 77u64) -> u32);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize, q_n.as_mut_ptr() as usize, 77u64) -> u32);

    crate::diff_eq!(ro, rn, "queue_signal_fence ret");
    crate::diff_eq!(this_o, this_n, "queue_signal_fence this");
    crate::diff_eq!(&q_o[1..], &q_n[1..], "queue_signal_fence args");
    Ok(1)
}

unsafe extern "win64" fn fake_fence_get_completed_40(fence: usize) -> u64 {
    rd::<u64>(fence + 0x08)
}

unsafe extern "win64" fn fake_device_removed_d410(_rcx: usize, _hr: u32) {}

pub unsafe fn diff_d3d_fence_get_completed_value(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Mock;
    let _m = Mock::new(0x1_4205_d410, fake_device_removed_d410 as *const ())?;

    let mut vtbl = [0usize; 10];
    vtbl[8] = fake_fence_get_completed_40 as *const () as usize;

    for val in [42u64, u64::MAX] {
        let fence = [vtbl.as_ptr() as usize as u64, val];
        let this_s = [0u64, fence.as_ptr() as usize as u64];
        let ro = exe::call_va!(orig, (this_s.as_ptr() as usize) -> u64);
        let rn = exe::call_va!(new, (this_s.as_ptr() as usize) -> u64);
        crate::diff_eq!(ro, rn, "fence_get_completed_value");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_dev_create_fence_120(
    dev: usize,
    init_val: u64,
    flags: u32,
    riid: usize,
    out_fence: usize,
) -> u32 {
    wr::<u64>(dev + 0x08, init_val);
    wr::<u32>(dev + 0x10, flags);
    wr::<usize>(dev + 0x18, riid);
    wr::<usize>(out_fence, 0xfeee_3333);
    0
}

pub unsafe fn diff_d3d_create_fence_wrapper(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Snapshot;
    let dev_slot = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let snap = Snapshot::take(dev_slot as *mut u8, 8);

    let mut vtbl = [0usize; 37];
    vtbl[36] = fake_dev_create_fence_120 as *const () as usize;

    let mut dev_o = [vtbl.as_ptr() as usize as u64, 0, 0, 0];
    let mut dev_n = dev_o;
    let mut this_o = [0x99u64; 3];
    let mut this_n = [0x99u64; 3];

    wr::<usize>(dev_slot, dev_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> usize);

    wr::<usize>(dev_slot, dev_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> usize);

    snap.restore();

    crate::diff_eq!(
        ro - this_o.as_ptr() as usize,
        rn - this_n.as_ptr() as usize,
        "create_fence_wrapper ret"
    );
    crate::diff_eq!(this_o, this_n, "create_fence_wrapper this");
    crate::diff_eq!(&dev_o[1..], &dev_n[1..], "create_fence_wrapper dev");
    Ok(1)
}

pub unsafe fn diff_d3d_signal_latency_fence(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Snapshot;
    let en_va = exe::ptr::<u8>(0x1_4385_8078) as usize;
    let st_va = exe::ptr::<usize>(0x1_43a3_6228) as usize;
    let snap_en = Snapshot::take(en_va as *mut u8, 1);
    let snap_st = Snapshot::take(st_va as *mut u8, 8);

    let mut vtbl = [0usize; 16];
    vtbl[14] = fake_queue_wait_or_signal as *const () as usize;

    for en in [0u8, 1u8] {
        let mut st_o = [0xfeee_4444u64, 10u64];
        let mut st_n = st_o;
        let mut q_o = [vtbl.as_ptr() as usize as u64, 0, 0];
        let mut q_n = q_o;

        wr::<u8>(en_va, en);
        wr::<usize>(st_va, st_o.as_mut_ptr() as usize);
        exe::call_va!(orig, (0usize, q_o.as_mut_ptr() as usize) -> ());

        wr::<u8>(en_va, en);
        wr::<usize>(st_va, st_n.as_mut_ptr() as usize);
        exe::call_va!(new, (0usize, q_n.as_mut_ptr() as usize) -> ());

        if st_o != st_n || q_o[1..] != q_n[1..] {
            snap_st.restore();
            snap_en.restore();
            return Err(format!("signal_latency_fence mismatch en={en}"));
        }
    }

    snap_st.restore();
    snap_en.restore();
    Ok(2)
}

unsafe extern "win64" fn fake_fence_set_event_48(
    fence: usize,
    target_val: u64,
    event: usize,
) -> u32 {
    wr::<u64>(fence + 0x10, target_val);
    wr::<usize>(fence + 0x18, event);
    0
}

unsafe extern "win64" fn fake_wait_single_object_5748(event: usize, timeout: u32) -> u32 {
    wr::<u32>(event, timeout);
    0
}

pub unsafe fn diff_d3d_wait_latency_fence(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{IatMock, Snapshot};
    let _im = IatMock::new(0x1_4264_5748, fake_wait_single_object_5748 as *const ())?;

    let en_va = exe::ptr::<u8>(0x1_4385_8078) as usize;
    let st_va = exe::ptr::<usize>(0x1_43a3_6228) as usize;
    let snap_en = Snapshot::take(en_va as *mut u8, 1);
    let snap_st = Snapshot::take(st_va as *mut u8, 8);

    let mut vtbl = [0usize; 10];
    vtbl[8] = fake_fence_get_completed_40 as *const () as usize;
    vtbl[9] = fake_fence_set_event_48 as *const () as usize;

    for (en, cur, comp) in [(0u8, 200u64, 50u64), (1, 200, 150), (1, 200, 50)] {
        let mut ev_o = 0u32;
        let mut ev_n = 0u32;
        let mut fence_o = [vtbl.as_ptr() as usize as u64, comp, 0, 0];
        let mut fence_n = fence_o;
        let st_o = [
            fence_o.as_mut_ptr() as usize as u64,
            cur,
            (&mut ev_o as *mut u32) as usize as u64,
        ];
        let st_n = [
            fence_n.as_mut_ptr() as usize as u64,
            cur,
            (&mut ev_n as *mut u32) as usize as u64,
        ];

        wr::<u8>(en_va, en);
        wr::<usize>(st_va, st_o.as_ptr() as usize);
        exe::call_va!(orig, (0usize) -> ());

        wr::<u8>(en_va, en);
        wr::<usize>(st_va, st_n.as_ptr() as usize);
        exe::call_va!(new, (0usize) -> ());

        if fence_o[1..3] != fence_n[1..3] || ev_o != ev_n {
            snap_st.restore();
            snap_en.restore();
            return Err(format!("wait_latency_fence mismatch en={en} comp={comp}"));
        }
    }

    snap_st.restore();
    snap_en.restore();
    Ok(3)
}

unsafe extern "win64" fn fake_close_handle_55b0(h: usize) -> i32 {
    wr::<u64>(h, rd::<u64>(h).wrapping_add(1));
    1
}

pub unsafe fn diff_d3d_destroy_latency_fence(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::IatMock;
    let _im = IatMock::new(0x1_4264_55b0, fake_close_handle_55b0 as *const ())?;

    let mut vtbl = [0usize; 4];
    vtbl[2] = fake_com_addref_or_release as *const () as usize;

    let mut fence_o = [vtbl.as_ptr() as usize, 0usize];
    let mut fence_n = fence_o;
    let mut ev_o = 0u64;
    let mut ev_n = 0u64;

    let mut this_o = [
        fence_o.as_mut_ptr() as usize as u64,
        42,
        (&mut ev_o as *mut u64) as usize as u64,
    ];
    let mut this_n = [
        fence_n.as_mut_ptr() as usize as u64,
        42,
        (&mut ev_n as *mut u64) as usize as u64,
    ];

    exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> ());

    crate::diff_eq!(this_o, this_n, "destroy_latency_fence this");
    crate::diff_eq!(fence_o[1], fence_n[1], "destroy_latency_fence release");
    crate::diff_eq!(ev_o, ev_n, "destroy_latency_fence close_handle");
    Ok(1)
}

unsafe extern "win64" fn fake_create_event_5740(
    _attr: usize,
    _manual: i32,
    _init: i32,
    _name: usize,
) -> usize {
    0xeeee_5555
}

pub unsafe fn diff_d3d_init_latency_fence(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{IatMock, Snapshot};
    let _im = IatMock::new(0x1_4264_5740, fake_create_event_5740 as *const ())?;
    let dev_slot = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let snap = Snapshot::take(dev_slot as *mut u8, 8);

    let mut vtbl = [0usize; 37];
    vtbl[36] = fake_dev_create_fence_120 as *const () as usize;

    let mut dev_o = [vtbl.as_ptr() as usize as u64, 0, 0, 0];
    let mut dev_n = dev_o;
    let mut this_o = [0x99u64; 3];
    let mut this_n = [0x99u64; 3];

    wr::<usize>(dev_slot, dev_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> usize);

    wr::<usize>(dev_slot, dev_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> usize);

    snap.restore();

    crate::diff_eq!(
        ro - this_o.as_ptr() as usize,
        rn - this_n.as_ptr() as usize,
        "init_latency_fence ret"
    );
    crate::diff_eq!(this_o, this_n, "init_latency_fence this");
    crate::diff_eq!(&dev_o[1..], &dev_n[1..], "init_latency_fence dev");
    Ok(1)
}

unsafe extern "win64" fn fake_enqueue_cb_886c0(queue_ptr_addr: usize, task: usize) {
    let q = rd::<usize>(queue_ptr_addr);
    wr::<u64>(q + 0x28, rd::<u64>(task + 0x08));
}

unsafe extern "win64" fn fake_enqueue_dep_87760(
    _unused_rcx: usize,
    queue_ptr_addr: usize,
    dep_a: usize,
    dep_b: usize,
) {
    let q = rd::<usize>(queue_ptr_addr);
    wr::<usize>(q + 0x10, dep_a);
    wr::<usize>(q + 0x18, dep_b);
}

unsafe extern "win64" fn fake_enqueue_xmm_dep_87920(
    _unused_rcx: usize,
    queue_ptr_addr: usize,
    dep_copy_addr: usize,
    dep_b: usize,
) {
    let q = rd::<usize>(queue_ptr_addr);
    wr::<u64>(q + 0x10, rd::<u64>(dep_copy_addr));
    wr::<u64>(q + 0x18, rd::<u64>(dep_copy_addr + 0x08));
    wr::<usize>(q + 0x20, dep_b);
}

unsafe extern "win64" fn fake_callable_destroy_20(callable: usize, not_inline: u8) {
    wr::<u8>(callable + 0x10, not_inline.wrapping_add(0x40));
}

pub unsafe fn diff_d3d_enqueue_task_simple(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m = Mock::new(0x1_4208_86c0, fake_enqueue_cb_886c0 as *const ())?;
    let notify_slot = exe::ptr::<usize>(0x1_43a0_8298) as usize;
    let snap = Snapshot::take(notify_slot as *mut u8, 8);

    let mut vtbl = [0usize; 5];
    vtbl[4] = fake_callable_destroy_20 as *const () as usize;

    let mut inner_o = [0u8; 0x80];
    let mut inner_n = [0u8; 0x80];
    let mut notify_o = [0u8; 0x260];
    let mut notify_n = [0u8; 0x260];
    let this_o = [0usize, 0, 0, inner_o.as_mut_ptr() as usize];
    let this_n = [0usize, 0, 0, inner_n.as_mut_ptr() as usize];

    let mut task_o = [0u64; 8];
    task_o[0] = vtbl.as_ptr() as usize as u64;
    task_o[1] = 0x5566_7788;
    task_o[7] = task_o.as_mut_ptr() as usize as u64;
    let mut task_n = [0u64; 8];
    task_n[0] = vtbl.as_ptr() as usize as u64;
    task_n[1] = 0x5566_7788;
    task_n[7] = task_n.as_mut_ptr() as usize as u64;

    wr::<usize>(notify_slot, notify_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (this_o.as_ptr() as usize, task_o.as_mut_ptr() as usize) -> ());

    wr::<usize>(notify_slot, notify_n.as_mut_ptr() as usize);
    exe::call_va!(new, (this_n.as_ptr() as usize, task_n.as_mut_ptr() as usize) -> ());

    snap.restore();

    crate::diff_eq!(inner_o, inner_n, "enqueue_task_simple inner");
    crate::diff_eq!(
        rd::<u32>(notify_o.as_ptr() as usize + 0x258),
        rd::<u32>(notify_n.as_ptr() as usize + 0x258),
        "enqueue_task_simple notify_cnt"
    );
    crate::diff_eq!(&task_o[1..], &task_n[1..], "enqueue_task_simple task");
    Ok(1)
}

pub unsafe fn diff_d3d_enqueue_task_with_dep(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m1 = Mock::new(0x1_4208_7760, fake_enqueue_dep_87760 as *const ())?;
    let _m2 = Mock::new(0x1_4208_86c0, fake_enqueue_cb_886c0 as *const ())?;
    let notify_slot = exe::ptr::<usize>(0x1_43a0_8298) as usize;
    let snap = Snapshot::take(notify_slot as *mut u8, 8);

    let mut vtbl = [0usize; 5];
    vtbl[4] = fake_callable_destroy_20 as *const () as usize;

    let mut inner_o = [0u8; 0x80];
    let mut inner_n = [0u8; 0x80];
    let mut notify_o = [0u8; 0x260];
    let mut notify_n = [0u8; 0x260];
    let this_o = [0usize, 0, 0, inner_o.as_mut_ptr() as usize];
    let this_n = [0usize, 0, 0, inner_n.as_mut_ptr() as usize];

    let mut task_o = [0u64; 8];
    task_o[0] = vtbl.as_ptr() as usize as u64;
    task_o[1] = 0x1122_3344;
    task_o[7] = task_o.as_mut_ptr() as usize as u64;
    let mut task_n = [0u64; 8];
    task_n[0] = vtbl.as_ptr() as usize as u64;
    task_n[1] = 0x1122_3344;
    task_n[7] = task_n.as_mut_ptr() as usize as u64;

    wr::<usize>(notify_slot, notify_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (this_o.as_ptr() as usize, 0xaaaausize, 0xbbbbusize, task_o.as_mut_ptr() as usize) -> ());

    wr::<usize>(notify_slot, notify_n.as_mut_ptr() as usize);
    exe::call_va!(new, (this_n.as_ptr() as usize, 0xaaaausize, 0xbbbbusize, task_n.as_mut_ptr() as usize) -> ());

    snap.restore();

    crate::diff_eq!(inner_o, inner_n, "enqueue_task_with_dep inner");
    crate::diff_eq!(
        rd::<u32>(notify_o.as_ptr() as usize + 0x258),
        rd::<u32>(notify_n.as_ptr() as usize + 0x258),
        "enqueue_task_with_dep notify_cnt"
    );
    crate::diff_eq!(&task_o[1..], &task_n[1..], "enqueue_task_with_dep task");
    Ok(1)
}

pub unsafe fn diff_d3d_enqueue_task_with_xmm_dep(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m1 = Mock::new(0x1_4208_7920, fake_enqueue_xmm_dep_87920 as *const ())?;
    let _m2 = Mock::new(0x1_4208_86c0, fake_enqueue_cb_886c0 as *const ())?;
    let notify_slot = exe::ptr::<usize>(0x1_43a0_8298) as usize;
    let snap = Snapshot::take(notify_slot as *mut u8, 8);

    let mut vtbl = [0usize; 5];
    vtbl[4] = fake_callable_destroy_20 as *const () as usize;

    let mut inner_o = [0u8; 0x80];
    let mut inner_n = [0u8; 0x80];
    let mut notify_o = [0u8; 0x260];
    let mut notify_n = [0u8; 0x260];
    let this_o = [0usize, 0, 0, inner_o.as_mut_ptr() as usize];
    let this_n = [0usize, 0, 0, inner_n.as_mut_ptr() as usize];
    let dep_xmm = [0x1111_2222_3333_4444u64, 0x5555_6666_7777_8888u64];

    let mut task_o = [0u64; 8];
    task_o[0] = vtbl.as_ptr() as usize as u64;
    task_o[1] = 0x99aa_bbcc;
    task_o[7] = task_o.as_mut_ptr() as usize as u64;
    let mut task_n = [0u64; 8];
    task_n[0] = vtbl.as_ptr() as usize as u64;
    task_n[1] = 0x99aa_bbcc;
    task_n[7] = task_n.as_mut_ptr() as usize as u64;

    wr::<usize>(notify_slot, notify_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (this_o.as_ptr() as usize, dep_xmm.as_ptr() as usize, 0xccccusize, task_o.as_mut_ptr() as usize) -> ());

    wr::<usize>(notify_slot, notify_n.as_mut_ptr() as usize);
    exe::call_va!(new, (this_n.as_ptr() as usize, dep_xmm.as_ptr() as usize, 0xccccusize, task_n.as_mut_ptr() as usize) -> ());

    snap.restore();

    crate::diff_eq!(inner_o, inner_n, "enqueue_task_with_xmm_dep inner");
    crate::diff_eq!(
        rd::<u32>(notify_o.as_ptr() as usize + 0x258),
        rd::<u32>(notify_n.as_ptr() as usize + 0x258),
        "enqueue_task_with_xmm_dep notify_cnt"
    );
    crate::diff_eq!(&task_o[1..], &task_n[1..], "enqueue_task_with_xmm_dep task");
    Ok(1)
}

pub unsafe fn diff_d3d_acquire_shared_com_3bd8(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Snapshot;
    let g_slot = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let snap = Snapshot::take(g_slot as *mut u8, 8);

    let mut vtbl = [0usize; 4];
    vtbl[1] = fake_com_addref_or_release as *const () as usize;
    let mut com_o = [vtbl.as_ptr() as usize, 0usize];
    let mut com_n = com_o;

    let mut g_o = vec![0u8; 0x3be0];
    let mut g_n = vec![0u8; 0x3be0];
    wr::<usize>(g_o.as_mut_ptr() as usize + 0x3bd8, com_o.as_mut_ptr() as usize);
    wr::<usize>(g_n.as_mut_ptr() as usize + 0x3bd8, com_n.as_mut_ptr() as usize);

    let mut out_o = 0usize;
    let mut out_n = 0usize;

    wr::<usize>(g_slot, g_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(orig, (0usize, (&mut out_o as *mut usize) as usize) -> usize);

    wr::<usize>(g_slot, g_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(new, (0usize, (&mut out_n as *mut usize) as usize) -> usize);

    snap.restore();

    crate::diff_eq!(ro, (&mut out_o as *mut usize) as usize, "acquire_3bd8 ret_o");
    crate::diff_eq!(rn, (&mut out_n as *mut usize) as usize, "acquire_3bd8 ret_n");
    crate::diff_eq!(out_o != 0, out_n != 0, "acquire_3bd8 nonnull");
    crate::diff_eq!(com_o[1], com_n[1], "acquire_3bd8 addref");
    Ok(1)
}

static NEXT_ALLOC_1A10: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

unsafe extern "win64" fn fake_alloc_config_1a10() -> usize {
    NEXT_ALLOC_1A10.load(Ordering::SeqCst)
}

pub unsafe fn diff_d3d_store_global_config_58(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m = Mock::new(0x1_411e_1a10, fake_alloc_config_1a10 as *const ())?;
    let slot = exe::ptr::<usize>(0x1_43a3_62b8) as usize;
    let snap = Snapshot::take(slot as *mut u8, 8);

    let mut src = [0u64; 11];
    for (i, v) in src.iter_mut().enumerate() {
        *v = 0x1000_0000_0000_0000u64.wrapping_add((i as u64) * 0x1111_2222);
    }

    let mut dst_o = [0u64; 12];
    let mut dst_n = [0u64; 12];

    wr::<usize>(slot, 0);
    NEXT_ALLOC_1A10.store(dst_o.as_mut_ptr() as usize, Ordering::SeqCst);
    exe::call_va!(orig, (src.as_ptr() as usize) -> ());

    wr::<usize>(slot, 0);
    NEXT_ALLOC_1A10.store(dst_n.as_mut_ptr() as usize, Ordering::SeqCst);
    exe::call_va!(new, (src.as_ptr() as usize) -> ());

    snap.restore();

    crate::diff_eq!(dst_o, dst_n, "store_global_config_58");
    Ok(1)
}

static PCL_MARKER_REC: AtomicU64 = AtomicU64::new(0);

unsafe extern "win64" fn fake_pcl_set_marker(marker: u32, token: u64) {
    PCL_MARKER_REC.store(((marker as u64) << 48) ^ token, Ordering::SeqCst);
}

unsafe extern "win64" fn fake_sl_get_feature_fn_7308(
    feat: u32,
    _name: usize,
    out_slot: usize,
) -> u32 {
    if feat == 4 {
        wr::<usize>(out_slot, fake_pcl_set_marker as *const () as usize);
        0
    } else {
        1
    }
}

pub unsafe fn diff_d3d_sl_pcl_set_marker_present(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{IatMock, Snapshot};
    let _im = IatMock::new(0x1_4264_7308, fake_sl_get_feature_fn_7308 as *const ())?;

    let ca_va = exe::ptr::<u8>(0x1_4385_83ca) as usize;
    let tok_va = exe::ptr::<u64>(0x1_4385_8398) as usize;
    let fn_va = exe::ptr::<usize>(0x1_43c8_c608) as usize;
    let snap_ca = Snapshot::take(ca_va as *mut u8, 1);
    let snap_tok = Snapshot::take(tok_va as *mut u8, 8);
    let snap_fn = Snapshot::take(fn_va as *mut u8, 8);

    wr::<u8>(ca_va, 1);
    wr::<u64>(tok_va, 0x1234_5678);
    wr::<usize>(fn_va, 0);
    PCL_MARKER_REC.store(0, Ordering::SeqCst);
    exe::call_va!(orig, () -> ());
    let rec_o = PCL_MARKER_REC.load(Ordering::SeqCst);
    let fn_o = rd::<usize>(fn_va);

    wr::<u8>(ca_va, 1);
    wr::<u64>(tok_va, 0x1234_5678);
    wr::<usize>(fn_va, 0);
    PCL_MARKER_REC.store(0, Ordering::SeqCst);
    exe::call_va!(new, () -> ());
    let rec_n = PCL_MARKER_REC.load(Ordering::SeqCst);
    let fn_n = rd::<usize>(fn_va);

    snap_fn.restore();
    snap_tok.restore();
    snap_ca.restore();

    crate::diff_eq!(rec_o, rec_n, "sl_pcl_set_marker_present rec");
    crate::diff_eq!(fn_o, fn_n, "sl_pcl_set_marker_present fn_slot");
    Ok(1)
}

pub unsafe fn diff_d3d_sl_dequeue_frame_and_mark_render_submit(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use crate::difftest::Snapshot;
    let c0_va = exe::ptr::<u8>(0x1_4385_82c0) as usize;
    let block_va = exe::ptr::<u8>(0x1_4385_8398) as usize;
    let fn_va = exe::ptr::<usize>(0x1_43c8_c608) as usize;
    let snap_c0 = Snapshot::take(c0_va as *mut u8, 1);
    let snap_block = Snapshot::take(block_va as *mut u8, 0x38);
    let snap_fn = Snapshot::take(fn_va as *mut u8, 8);

    let ring = [100u64, 200, 300, 400];

    std::ptr::write_bytes(block_va as *mut u8, 0, 0x38);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_83a0) as usize, ring.as_ptr() as usize);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83a8) as usize, 3);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b0) as usize, 2);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b4) as usize, 4);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize, 1);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_83ce) as usize, 1);
    wr::<u8>(c0_va, 1);
    wr::<usize>(fn_va, fake_pcl_set_marker as *const () as usize);
    PCL_MARKER_REC.store(0, Ordering::SeqCst);
    exe::call_va!(orig, () -> ());
    let block_o = snap_block.current();
    let rec_o = PCL_MARKER_REC.load(Ordering::SeqCst);

    std::ptr::write_bytes(block_va as *mut u8, 0, 0x38);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_83a0) as usize, ring.as_ptr() as usize);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83a8) as usize, 3);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b0) as usize, 2);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b4) as usize, 4);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize, 1);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_83ce) as usize, 1);
    wr::<u8>(c0_va, 1);
    wr::<usize>(fn_va, fake_pcl_set_marker as *const () as usize);
    PCL_MARKER_REC.store(0, Ordering::SeqCst);
    exe::call_va!(new, () -> ());
    let block_n = snap_block.current();
    let rec_n = PCL_MARKER_REC.load(Ordering::SeqCst);

    snap_fn.restore();
    snap_block.restore();
    snap_c0.restore();

    crate::diff_eq!(block_o, block_n, "sl_dequeue_frame block");
    crate::diff_eq!(rec_o, rec_n, "sl_dequeue_frame marker");
    Ok(1)
}

unsafe extern "win64" fn fake_token_get_idx_0(obj: usize) -> u32 {
    rd::<u32>(obj + 0x08)
}

unsafe extern "win64" fn fake_sl_get_new_frame_token_7310(
    token_slot: usize,
    idx_ptr: usize,
) -> u32 {
    let obj = rd::<usize>(token_slot);
    wr::<u32>(obj + 0x08, rd::<u32>(idx_ptr));
    0
}

pub unsafe fn diff_d3d_sl_advance_frame_token(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{IatMock, Snapshot};
    let _im = IatMock::new(0x1_4264_7310, fake_sl_get_new_frame_token_7310 as *const ())?;
    let slot = exe::ptr::<usize>(0x1_4385_8388) as usize;
    let snap = Snapshot::take(slot as *mut u8, 8);

    let vtbl = [fake_token_get_idx_0 as *const () as usize];
    let mut obj_o = [vtbl.as_ptr() as usize as u64, 41u64];
    let mut obj_n = obj_o;

    wr::<usize>(slot, obj_o.as_mut_ptr() as usize);
    exe::call_va!(orig, () -> ());

    wr::<usize>(slot, obj_n.as_mut_ptr() as usize);
    exe::call_va!(new, () -> ());

    snap.restore();

    crate::diff_eq!(obj_o[1], obj_n[1], "sl_advance_frame_token");
    Ok(1)
}

pub unsafe fn diff_d3d_sl_enqueue_frame_token(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Snapshot;
    let block_va = exe::ptr::<u8>(0x1_4385_8390) as usize;
    let snap = Snapshot::take(block_va as *mut u8, 0x30);

    let mut ring_o = [0u64; 2];
    let mut ring_n = [0u64; 2];

    std::ptr::write_bytes(block_va as *mut u8, 0, 0x30);
    wr::<u64>(block_va, 0x7788_99aa);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_83a0) as usize, ring_o.as_mut_ptr() as usize);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83ac) as usize, 1);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b0) as usize, 1);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b4) as usize, 2);
    exe::call_va!(orig, () -> ());
    let state_o = snap.current();

    std::ptr::write_bytes(block_va as *mut u8, 0, 0x30);
    wr::<u64>(block_va, 0x7788_99aa);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_83a0) as usize, ring_n.as_mut_ptr() as usize);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83ac) as usize, 1);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b0) as usize, 1);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83b4) as usize, 2);
    exe::call_va!(new, () -> ());
    let state_n = snap.current();

    snap.restore();

    crate::diff_eq!(ring_o, ring_n, "sl_enqueue_frame_token ring");
    crate::diff_eq!(&state_o[0x18..], &state_n[0x18..], "sl_enqueue_frame_token state");
    Ok(1)
}

unsafe extern "win64" fn fake_tag_res_f220(slot: usize, res: usize, _name: usize) {
    let holder = rd::<usize>(slot);
    if holder != 0 {
        wr::<usize>(holder + 0x08, res);
    }
}

unsafe extern "win64" fn fake_conv_res_f090(
    slot: usize,
    _name: usize,
    w: u32,
    h: u32,
    flag: u8,
) {
    let holder = rd::<usize>(slot);
    if holder != 0 {
        wr::<u32>(holder + 0x08, w);
        wr::<u32>(holder + 0x0c, h);
        wr::<u8>(holder + 0x10, flag);
    }
}

unsafe extern "win64" fn fake_sl_set_tag_7300(
    viewport: usize,
    tags: usize,
    count: u32,
    _cmd: usize,
) -> u32 {
    let mut acc = count as u64;
    for i in 0..(count as usize) {
        let tag = tags + i * 0x40;
        let res_s = rd::<usize>(tag + 0x20);
        let native_ptr = rd::<u64>(res_s + 0x28);
        let tag_ty = rd::<u32>(tag + 0x28) as u64;
        acc = acc.wrapping_mul(131).wrapping_add(native_ptr ^ tag_ty);
    }
    wr::<u64>(viewport, acc);
    0
}

pub unsafe fn diff_d3d_sl_tag_hudless_and_depth_mvec(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use crate::difftest::{IatMock, Mock, Snapshot};
    let _m1 = Mock::new(0x1_4208_f220, fake_tag_res_f220 as *const ())?;
    let _m2 = Mock::new(0x1_4208_f090, fake_conv_res_f090 as *const ())?;
    let _im = IatMock::new(0x1_4264_7300, fake_sl_set_tag_7300 as *const ())?;

    let c290_va = exe::ptr::<u8>(0x1_4385_8290) as usize;
    let blk_va = exe::ptr::<u8>(0x1_4385_8360) as usize;
    let snap_290 = Snapshot::take(c290_va as *mut u8, 1);
    let snap_blk = Snapshot::take(blk_va as *mut u8, 0xe0);

    let mut depth_res = [0u8; 0x80];
    let mut mvec_res = [0u8; 0x80];
    let mut hud_res = [0u8; 0x80];
    wr::<u64>(depth_res.as_mut_ptr() as usize + 0x70, 0x1111);
    wr::<u64>(mvec_res.as_mut_ptr() as usize + 0x70, 0x2222);
    wr::<u64>(hud_res.as_mut_ptr() as usize + 0x70, 0x3333);
    wr::<u32>(hud_res.as_mut_ptr() as usize, 1920);
    wr::<u32>(hud_res.as_mut_ptr() as usize + 0x04, 1080);

    let mut hud_holder_o = [hud_res.as_mut_ptr() as usize, 0usize, 0usize];
    let mut conv_holder_o = [hud_res.as_mut_ptr() as usize, 0usize, 0usize];
    let mut hud_holder_n = hud_holder_o;
    let mut conv_holder_n = conv_holder_o;

    wr::<u8>(c290_va, 1);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_83f8) as usize, 1);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_8408) as usize, hud_holder_o.as_mut_ptr() as usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_8410) as usize, conv_holder_o.as_mut_ptr() as usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize, depth_res.as_mut_ptr() as usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize, mvec_res.as_mut_ptr() as usize);
    wr::<u64>(blk_va, 0);
    exe::call_va!(orig, (hud_res.as_mut_ptr() as usize, 1u8) -> ());
    let vp_o = rd::<u64>(blk_va);

    wr::<usize>(exe::ptr::<usize>(0x1_4385_8408) as usize, hud_holder_n.as_mut_ptr() as usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4385_8410) as usize, conv_holder_n.as_mut_ptr() as usize);
    wr::<u64>(blk_va, 0);
    exe::call_va!(new, (hud_res.as_mut_ptr() as usize, 1u8) -> ());
    let vp_n = rd::<u64>(blk_va);

    snap_blk.restore();
    snap_290.restore();

    crate::diff_eq!(vp_o, vp_n, "sl_tag_hudless vp_hash");
    crate::diff_eq!(hud_holder_o[1..], hud_holder_n[1..], "sl_tag_hudless hud_holder");
    crate::diff_eq!(conv_holder_o[1..], conv_holder_n[1..], "sl_tag_hudless conv_holder");
    Ok(1)
}

pub unsafe fn diff_d3d_sl_record_camera_params(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::Snapshot;
    let flags_va = exe::ptr::<u8>(0x1_4385_83cd) as usize;
    let cam_va = exe::ptr::<u8>(0x1_4385_8440) as usize;
    let snap_flags = Snapshot::take(flags_va as *mut u8, 3);
    let snap_cam = Snapshot::take(cam_va as *mut u8, 48);

    for (f_cd, f_cf) in [(0u8, 1u8), (1, 0), (1, 1)] {
        std::ptr::write_bytes(cam_va as *mut u8, 0, 48);
        wr::<u8>(flags_va, f_cd);
        wr::<u8>(flags_va + 2, f_cf);
        exe::call_va!(orig, (11u64, 22u64, 33u64, 44u64, 55u64, 66u64) -> ());
        let cam_o = snap_cam.current();

        std::ptr::write_bytes(cam_va as *mut u8, 0, 48);
        wr::<u8>(flags_va, f_cd);
        wr::<u8>(flags_va + 2, f_cf);
        exe::call_va!(new, (11u64, 22u64, 33u64, 44u64, 55u64, 66u64) -> ());
        let cam_n = snap_cam.current();

        if cam_o != cam_n {
            snap_cam.restore();
            snap_flags.restore();
            return Err(format!("sl_record_camera_params mismatch cd={f_cd} cf={f_cf}"));
        }
    }

    snap_cam.restore();
    snap_flags.restore();
    Ok(3)
}

unsafe extern "win64" fn fake_update_res_f220(slot: usize, res: usize, _name: usize) {
    let holder = rd::<usize>(slot);
    wr::<usize>(holder, res.wrapping_add(0x100));
}

pub unsafe fn diff_d3d_sl_update_depth_and_mvec_resources(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m = Mock::new(0x1_4208_f220, fake_update_res_f220 as *const ())?;

    let cd_va = exe::ptr::<u8>(0x1_4385_83cd) as usize;
    let blk_va = exe::ptr::<u8>(0x1_4385_83f8) as usize;
    let snap_cd = Snapshot::take(cd_va as *mut u8, 1);
    let snap_blk = Snapshot::take(blk_va as *mut u8, 0x48);

    for (mode, cd) in [(0u32, 0u8), (0, 1), (1, 0)] {
        let mut dh_o = 0usize;
        let mut mh_o = 0usize;
        let mut dh_n = 0usize;
        let mut mh_n = 0usize;

        wr::<u8>(cd_va, cd);
        wr::<u32>(blk_va, mode);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8418) as usize, (&mut dh_o as *mut usize) as usize);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8420) as usize, (&mut mh_o as *mut usize) as usize);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize, 0);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize, 0);
        exe::call_va!(orig, (0x2000usize, 0x1000usize) -> ());
        let d_o = rd::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize);
        let m_o = rd::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize);

        wr::<u8>(cd_va, cd);
        wr::<u32>(blk_va, mode);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8418) as usize, (&mut dh_n as *mut usize) as usize);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8420) as usize, (&mut mh_n as *mut usize) as usize);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize, 0);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize, 0);
        exe::call_va!(new, (0x2000usize, 0x1000usize) -> ());
        let d_n = rd::<usize>(exe::ptr::<usize>(0x1_4385_8430) as usize);
        let m_n = rd::<usize>(exe::ptr::<usize>(0x1_4385_8438) as usize);

        if d_o != d_n || m_o != m_n {
            snap_blk.restore();
            snap_cd.restore();
            return Err(format!("sl_update_depth_and_mvec mismatch mode={mode} cd={cd}"));
        }
    }

    snap_blk.restore();
    snap_cd.restore();
    Ok(3)
}

// REGISTRY: Replacement::verified(0x14208b3d0, "d3d::d3d_release_slot_table_14208b3d0", crate::ns::rend::d3d_device_init::d3d_release_slot_table as *const (), crate::ns::rend::d3d_device_init::diff_d3d_release_slot_table)
// REGISTRY: Replacement::verified(0x14208b460, "d3d::d3d_queue_wait_fence_14208b460", crate::ns::rend::d3d_device_init::d3d_queue_wait_fence as *const (), crate::ns::rend::d3d_device_init::diff_d3d_queue_wait_fence)
// REGISTRY: Replacement::verified(0x14208b480, "d3d::d3d_queue_signal_fence_14208b480", crate::ns::rend::d3d_device_init::d3d_queue_signal_fence as *const (), crate::ns::rend::d3d_device_init::diff_d3d_queue_signal_fence)
// REGISTRY: Replacement::verified_throws(0x14208b4b0, "d3d::d3d_fence_get_completed_value_14208b4b0", crate::ns::rend::d3d_device_init::d3d_fence_get_completed_value as *const (), Throws::Discard, crate::ns::rend::d3d_device_init::diff_d3d_fence_get_completed_value)
// REGISTRY: Replacement::verified_throws(0x14208b520, "d3d::d3d_create_fence_wrapper_14208b520", crate::ns::rend::d3d_device_init::d3d_create_fence_wrapper as *const (), Throws::Discard, crate::ns::rend::d3d_device_init::diff_d3d_create_fence_wrapper)
// REGISTRY: Replacement::verified(0x14208b960, "d3d::d3d_signal_latency_fence_14208b960", crate::ns::rend::d3d_device_init::d3d_signal_latency_fence as *const (), crate::ns::rend::d3d_device_init::diff_d3d_signal_latency_fence)
// REGISTRY: Replacement::verified_throws(0x14208b990, "d3d::d3d_wait_latency_fence_14208b990", crate::ns::rend::d3d_device_init::d3d_wait_latency_fence as *const (), Throws::Discard, crate::ns::rend::d3d_device_init::diff_d3d_wait_latency_fence)
// REGISTRY: Replacement::verified(0x14208ba20, "d3d::d3d_destroy_latency_fence_14208ba20", crate::ns::rend::d3d_device_init::d3d_destroy_latency_fence as *const (), crate::ns::rend::d3d_device_init::diff_d3d_destroy_latency_fence)
// REGISTRY: Replacement::verified_throws(0x14208ba70, "d3d::d3d_init_latency_fence_14208ba70", crate::ns::rend::d3d_device_init::d3d_init_latency_fence as *const (), Throws::Discard, crate::ns::rend::d3d_device_init::diff_d3d_init_latency_fence)
// REGISTRY: Replacement::verified(0x14208bb00, "d3d::d3d_enqueue_task_simple_14208bb00", crate::ns::rend::d3d_device_init::d3d_enqueue_task_simple as *const (), crate::ns::rend::d3d_device_init::diff_d3d_enqueue_task_simple)
// REGISTRY: Replacement::verified(0x14208bbc0, "d3d::d3d_enqueue_task_with_dep_14208bbc0", crate::ns::rend::d3d_device_init::d3d_enqueue_task_with_dep as *const (), crate::ns::rend::d3d_device_init::diff_d3d_enqueue_task_with_dep)
// REGISTRY: Replacement::verified(0x14208bdd0, "d3d::d3d_enqueue_task_with_xmm_dep_14208bdd0", crate::ns::rend::d3d_device_init::d3d_enqueue_task_with_xmm_dep as *const (), crate::ns::rend::d3d_device_init::diff_d3d_enqueue_task_with_xmm_dep)
// REGISTRY: Replacement::verified(0x14208d180, "d3d::d3d_acquire_shared_com_3bd8_14208d180", crate::ns::rend::d3d_device_init::d3d_acquire_shared_com_3bd8 as *const (), crate::ns::rend::d3d_device_init::diff_d3d_acquire_shared_com_3bd8)
// REGISTRY: Replacement::verified(0x14208dda0, "d3d::d3d_store_global_config_58_14208dda0", crate::ns::rend::d3d_device_init::d3d_store_global_config_58 as *const (), crate::ns::rend::d3d_device_init::diff_d3d_store_global_config_58)
// REGISTRY: Replacement::verified(0x14208e2d0, "d3d::d3d_sl_pcl_set_marker_present_14208e2d0", crate::ns::rend::d3d_device_init::d3d_sl_pcl_set_marker_present as *const (), crate::ns::rend::d3d_device_init::diff_d3d_sl_pcl_set_marker_present)
// REGISTRY: Replacement::verified(0x14208e340, "d3d::d3d_sl_dequeue_frame_and_mark_render_submit_14208e340", crate::ns::rend::d3d_device_init::d3d_sl_dequeue_frame_and_mark_render_submit as *const (), crate::ns::rend::d3d_device_init::diff_d3d_sl_dequeue_frame_and_mark_render_submit)
// REGISTRY: Replacement::verified(0x14208e440, "d3d::d3d_sl_advance_frame_token_14208e440", crate::ns::rend::d3d_device_init::d3d_sl_advance_frame_token as *const (), crate::ns::rend::d3d_device_init::diff_d3d_sl_advance_frame_token)
// REGISTRY: Replacement::verified(0x14208e480, "d3d::d3d_sl_enqueue_frame_token_14208e480", crate::ns::rend::d3d_device_init::d3d_sl_enqueue_frame_token as *const (), crate::ns::rend::d3d_device_init::diff_d3d_sl_enqueue_frame_token)
// REGISTRY: Replacement::verified_throws(0x14208e7b0, "d3d::d3d_sl_tag_hudless_and_depth_mvec_14208e7b0", crate::ns::rend::d3d_device_init::d3d_sl_tag_hudless_and_depth_mvec as *const (), Throws::Discard, crate::ns::rend::d3d_device_init::diff_d3d_sl_tag_hudless_and_depth_mvec)
// REGISTRY: Replacement::verified(0x14208ec60, "d3d::d3d_sl_record_camera_params_14208ec60", crate::ns::rend::d3d_device_init::d3d_sl_record_camera_params as *const (), crate::ns::rend::d3d_device_init::diff_d3d_sl_record_camera_params)
// REGISTRY: Replacement::verified_throws(0x14208ecb0, "d3d::d3d_sl_update_depth_and_mvec_resources_14208ecb0", crate::ns::rend::d3d_device_init::d3d_sl_update_depth_and_mvec_resources as *const (), Throws::Discard, crate::ns::rend::d3d_device_init::diff_d3d_sl_update_depth_and_mvec_resources)
